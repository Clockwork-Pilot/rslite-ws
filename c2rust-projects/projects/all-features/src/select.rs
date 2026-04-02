use c2rust_bitfields::BitfieldStruct;
use ::libc;
extern "C" {
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type Btree;
    pub type VtabCtx;
    pub type PreUpdate;
    pub type RenameToken;
    pub type Vdbe;
    pub type TableLock;
    pub type sqlite3_mutex;
    pub type CoveringIndexCheck;
    pub type RenameCtx;
    pub type WindowRewrite;
    pub type IdxCover;
    pub type RefSrcList;
    pub type CCurHint;
    pub type WhereInfo;
    fn sqlite3_randomness(N: ::core::ffi::c_int, P: *mut ::core::ffi::c_void);
    fn sqlite3_stricmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_strnicmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3HashInit(_: *mut Hash);
    fn sqlite3HashInsert(
        _: *mut Hash,
        pKey: *const ::core::ffi::c_char,
        pData: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3HashFind(
        _: *const Hash,
        pKey: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3HashClear(_: *mut Hash);
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
    fn sqlite3VdbeCreate(_: *mut Parse) -> *mut Vdbe;
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
    fn sqlite3VdbeExplain(
        _: *mut Parse,
        _: u8_0,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeExplainPop(_: *mut Parse);
    fn sqlite3VdbeChangeOpcode(_: *mut Vdbe, addr: ::core::ffi::c_int, _: u8_0);
    fn sqlite3VdbeChangeP2(
        _: *mut Vdbe,
        addr: ::core::ffi::c_int,
        P2: ::core::ffi::c_int,
    );
    fn sqlite3VdbeChangeP5(_: *mut Vdbe, P5: u16_0);
    fn sqlite3VdbeJumpHere(_: *mut Vdbe, addr: ::core::ffi::c_int);
    fn sqlite3VdbeJumpHereOrPopInst(_: *mut Vdbe, addr: ::core::ffi::c_int);
    fn sqlite3VdbeChangeToNoop(
        _: *mut Vdbe,
        addr: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeChangeP4(
        _: *mut Vdbe,
        addr: ::core::ffi::c_int,
        zP4: *const ::core::ffi::c_char,
        N: ::core::ffi::c_int,
    );
    fn sqlite3VdbeAppendP4(
        _: *mut Vdbe,
        pP4: *mut ::core::ffi::c_void,
        p4type: ::core::ffi::c_int,
    );
    fn sqlite3VdbeGetOp(_: *mut Vdbe, _: ::core::ffi::c_int) -> *mut VdbeOp;
    fn sqlite3VdbeMakeLabel(_: *mut Parse) -> ::core::ffi::c_int;
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
    fn sqlite3WalkExpr(_: *mut Walker, _: *mut Expr) -> ::core::ffi::c_int;
    fn sqlite3WalkExprNN(_: *mut Walker, _: *mut Expr) -> ::core::ffi::c_int;
    fn sqlite3WalkSelect(_: *mut Walker, _: *mut Select) -> ::core::ffi::c_int;
    fn sqlite3ExprWalkNoop(_: *mut Walker, _: *mut Expr) -> ::core::ffi::c_int;
    fn sqlite3SelectWalkNoop(_: *mut Walker, _: *mut Select) -> ::core::ffi::c_int;
    fn sqlite3WindowUnlinkFromSelect(_: *mut Window);
    fn sqlite3WindowListDelete(db: *mut sqlite3, p: *mut Window);
    fn sqlite3WindowCodeInit(_: *mut Parse, _: *mut Select);
    fn sqlite3WindowCodeStep(
        _: *mut Parse,
        _: *mut Select,
        _: *mut WhereInfo,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3WindowRewrite(_: *mut Parse, _: *mut Select) -> ::core::ffi::c_int;
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
    fn sqlite3DbMallocRawNN(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbStrDup(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3DbReallocOrFree(
        _: *mut sqlite3,
        _: *mut ::core::ffi::c_void,
        _: u64_0,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3DbFree(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3DbFreeNN(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3DbNNFreeNN(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3MPrintf(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3ProgressCheck(_: *mut Parse);
    fn sqlite3ErrorMsg(_: *mut Parse, _: *const ::core::ffi::c_char, ...);
    fn sqlite3GetTempReg(_: *mut Parse) -> ::core::ffi::c_int;
    fn sqlite3ReleaseTempReg(_: *mut Parse, _: ::core::ffi::c_int);
    fn sqlite3GetTempRange(_: *mut Parse, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3ReleaseTempRange(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3ClearTempRegCache(_: *mut Parse);
    fn sqlite3Expr(
        _: *mut sqlite3,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> *mut Expr;
    fn sqlite3PExpr(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: *mut Expr,
        _: *mut Expr,
    ) -> *mut Expr;
    fn sqlite3PExprAddSelect(_: *mut Parse, _: *mut Expr, _: *mut Select);
    fn sqlite3ExprAnd(_: *mut Parse, _: *mut Expr, _: *mut Expr) -> *mut Expr;
    fn sqlite3ExprFunction(
        _: *mut Parse,
        _: *mut ExprList,
        _: *const Token,
        _: ::core::ffi::c_int,
    ) -> *mut Expr;
    fn sqlite3ExprDelete(_: *mut sqlite3, _: *mut Expr);
    fn sqlite3ExprListAppend(
        _: *mut Parse,
        _: *mut ExprList,
        _: *mut Expr,
    ) -> *mut ExprList;
    fn sqlite3ExprListDelete(_: *mut sqlite3, _: *mut ExprList);
    fn sqlite3ExprListDeleteGeneric(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3ColumnSetColl(
        _: *mut sqlite3,
        _: *mut Column,
        zColl: *const ::core::ffi::c_char,
    );
    fn sqlite3PrimaryKeyIndex(_: *mut Table) -> *mut Index;
    fn sqlite3RowSetClear(_: *mut ::core::ffi::c_void);
    fn sqlite3ViewGetColumnNames(_: *mut Parse, _: *mut Table) -> ::core::ffi::c_int;
    fn sqlite3DeleteTable(_: *mut sqlite3, _: *mut Table);
    fn sqlite3DeleteTableGeneric(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3IdListAppend(_: *mut Parse, _: *mut IdList, _: *mut Token) -> *mut IdList;
    fn sqlite3IdListIndex(
        _: *mut IdList,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3SrcListEnlarge(
        _: *mut Parse,
        _: *mut SrcList,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> *mut SrcList;
    fn sqlite3SrcListAppendList(
        pParse: *mut Parse,
        p1: *mut SrcList,
        p2: *mut SrcList,
    ) -> *mut SrcList;
    fn sqlite3SubqueryDetach(_: *mut sqlite3, _: *mut SrcItem) -> *mut Select;
    fn sqlite3SrcItemAttachSubquery(
        _: *mut Parse,
        _: *mut SrcItem,
        _: *mut Select,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3SrcListAppendFromTerm(
        _: *mut Parse,
        _: *mut SrcList,
        _: *mut Token,
        _: *mut Token,
        _: *mut Token,
        _: *mut Select,
        _: *mut OnOrUsing,
    ) -> *mut SrcList;
    fn sqlite3SrcListAssignCursors(_: *mut Parse, _: *mut SrcList);
    fn sqlite3IdListDelete(_: *mut sqlite3, _: *mut IdList);
    fn sqlite3SrcListDelete(_: *mut sqlite3, _: *mut SrcList);
    fn sqlite3WhereBegin(
        _: *mut Parse,
        _: *mut SrcList,
        _: *mut Expr,
        _: *mut ExprList,
        _: *mut ExprList,
        _: *mut Select,
        _: u16_0,
        _: ::core::ffi::c_int,
    ) -> *mut WhereInfo;
    fn sqlite3WhereEnd(_: *mut WhereInfo);
    fn sqlite3WhereOutputRowCount(_: *mut WhereInfo) -> LogEst;
    fn sqlite3WhereIsDistinct(_: *mut WhereInfo) -> ::core::ffi::c_int;
    fn sqlite3WhereIsOrdered(_: *mut WhereInfo) -> ::core::ffi::c_int;
    fn sqlite3WhereOrderByLimitOptLabel(_: *mut WhereInfo) -> ::core::ffi::c_int;
    fn sqlite3WhereMinMaxOptEarlyOut(_: *mut Vdbe, _: *mut WhereInfo);
    fn sqlite3WhereIsSorted(_: *mut WhereInfo) -> ::core::ffi::c_int;
    fn sqlite3WhereContinueLabel(_: *mut WhereInfo) -> ::core::ffi::c_int;
    fn sqlite3WhereBreakLabel(_: *mut WhereInfo) -> ::core::ffi::c_int;
    fn sqlite3ExprCodeMove(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3ExprToRegister(pExpr: *mut Expr, iReg: ::core::ffi::c_int);
    fn sqlite3ExprCode(_: *mut Parse, _: *mut Expr, _: ::core::ffi::c_int);
    fn sqlite3ExprNullRegisterRange(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3ExprCodeExprList(
        _: *mut Parse,
        _: *mut ExprList,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: u8_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprIfFalse(
        _: *mut Parse,
        _: *mut Expr,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3LocateTableItem(
        _: *mut Parse,
        flags: u32_0,
        _: *mut SrcItem,
    ) -> *mut Table;
    fn sqlite3ExprListCompare(
        _: *const ExprList,
        _: *const ExprList,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprImpliesNonNullRow(
        _: *mut Expr,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3AggInfoPersistWalkerInit(_: *mut Walker, _: *mut Parse);
    fn sqlite3ExprAnalyzeAggregates(_: *mut NameContext, _: *mut Expr);
    fn sqlite3ExprAnalyzeAggList(_: *mut NameContext, _: *mut ExprList);
    fn sqlite3CodeVerifySchema(_: *mut Parse, _: ::core::ffi::c_int);
    fn sqlite3IsTrueOrFalse(_: *const ::core::ffi::c_char) -> u32_0;
    fn sqlite3ExprTruthValue(_: *const Expr) -> ::core::ffi::c_int;
    fn sqlite3ExprIsConstant(_: *mut Parse, _: *mut Expr) -> ::core::ffi::c_int;
    fn sqlite3ExprIsConstantOrGroupBy(
        _: *mut Parse,
        _: *mut Expr,
        _: *mut ExprList,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprIsSingleTableConstraint(
        _: *mut Expr,
        _: *const SrcList,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprIsInteger(
        _: *const Expr,
        _: *mut ::core::ffi::c_int,
        _: *mut Parse,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprCanBeNull(_: *const Expr) -> ::core::ffi::c_int;
    fn sqlite3RowidAlias(pTab: *mut Table) -> *const ::core::ffi::c_char;
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
    fn sqlite3AuthCheck(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3LogEst(_: u64_0) -> LogEst;
    fn sqlite3LogEstAdd(_: LogEst, _: LogEst) -> LogEst;
    fn sqlite3ExprAffinity(pExpr: *const Expr) -> ::core::ffi::c_char;
    fn sqlite3ExprDataType(pExpr: *const Expr) -> ::core::ffi::c_int;
    fn sqlite3IsBinary(_: *const CollSeq) -> ::core::ffi::c_int;
    fn sqlite3ExprCollSeq(pParse: *mut Parse, pExpr: *const Expr) -> *mut CollSeq;
    fn sqlite3ExprNNCollSeq(pParse: *mut Parse, pExpr: *const Expr) -> *mut CollSeq;
    fn sqlite3ExprAddCollateString(
        _: *const Parse,
        _: *mut Expr,
        _: *const ::core::ffi::c_char,
    ) -> *mut Expr;
    fn sqlite3ExprSkipCollateAndLikely(_: *mut Expr) -> *mut Expr;
    static sqlite3StdTypeAffinity: [::core::ffi::c_char; 0];
    static mut sqlite3StdType: [*const ::core::ffi::c_char; 0];
    static sqlite3CtypeMap: [::core::ffi::c_uchar; 0];
    fn sqlite3MatchEName(
        _: *const ExprList_item,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprColUsed(_: *mut Expr) -> Bitmask;
    fn sqlite3StrIHash(_: *const ::core::ffi::c_char) -> u8_0;
    fn sqlite3ResolveSelectNames(_: *mut Parse, _: *mut Select, _: *mut NameContext);
    fn sqlite3ResolveOrderGroupBy(
        _: *mut Parse,
        _: *mut Select,
        _: *mut ExprList,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3RenameTokenRemap(
        _: *mut Parse,
        pTo: *const ::core::ffi::c_void,
        pFrom: *const ::core::ffi::c_void,
    );
    fn sqlite3AffinityType(
        _: *const ::core::ffi::c_char,
        _: *mut Column,
    ) -> ::core::ffi::c_char;
    fn sqlite3SchemaToIndex(db: *mut sqlite3, _: *mut Schema) -> ::core::ffi::c_int;
    fn sqlite3KeyInfoOfIndex(_: *mut Parse, _: *mut Index) -> *mut KeyInfo;
    fn sqlite3OomFault(_: *mut sqlite3) -> *mut ::core::ffi::c_void;
    fn sqlite3CreateColumnExpr(
        _: *mut sqlite3,
        _: *mut SrcList,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> *mut Expr;
    fn sqlite3TableLock(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: Pgno,
        _: u8_0,
        _: *const ::core::ffi::c_char,
    );
    fn sqlite3ParserAddCleanup(
        _: *mut Parse,
        _: Option<unsafe extern "C" fn(*mut sqlite3, *mut ::core::ffi::c_void) -> ()>,
        _: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3ExprCompareCollSeq(_: *mut Parse, _: *const Expr) -> *mut CollSeq;
    fn sqlite3WithDelete(_: *mut sqlite3, _: *mut With);
    fn sqlite3WithDeleteGeneric(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3SelectExprHeight(_: *const Select) -> ::core::ffi::c_int;
    fn sqlite3ExprSetErrorOffset(_: *mut Expr, _: ::core::ffi::c_int);
    fn sqlite3ExprIsVector(pExpr: *const Expr) -> ::core::ffi::c_int;
    fn sqlite3VectorErrorMsg(_: *mut Parse, _: *mut Expr);
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
pub type sqlite3_destructor_type = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
>;
pub type size_t = usize;
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
pub struct CheckOnCtx {
    pub pSrc: *mut SrcList,
    pub iJoin: ::core::ffi::c_int,
    pub pParent: *mut CheckOnCtx

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WhereConst {
    pub pParse: *mut Parse,
    pub pOomFault: *mut u8_0,
    pub nConst: ::core::ffi::c_int,
    pub nChng: ::core::ffi::c_int,
    pub bHasAffBlob: ::core::ffi::c_int,
    pub mExcludeOn: u32_0,
    pub apExpr: *mut *mut Expr

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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SortCtx {
    pub pOrderBy: *mut ExprList,
    pub nOBSat: ::core::ffi::c_int,
    pub iECursor: ::core::ffi::c_int,
    pub regReturn: ::core::ffi::c_int,
    pub labelBkOut: ::core::ffi::c_int,
    pub addrSortIndex: ::core::ffi::c_int,
    pub labelDone: ::core::ffi::c_int,
    pub labelOBLopt: ::core::ffi::c_int,
    pub sortFlags: u8_0,
    pub pDeferredRowLoad: *mut RowLoadInfo

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RowLoadInfo {
    pub regResult: ::core::ffi::c_int,
    pub ecelFlags: u8_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DistinctCtx {
    pub isTnct: u8_0,
    pub eTnctType: u8_0,
    pub tabTnct: ::core::ffi::c_int,
    pub addrTnct: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubstContext {
    pub pParse: *mut Parse,
    pub iTable: ::core::ffi::c_int,
    pub iNewTable: ::core::ffi::c_int,
    pub isOuterJoin: ::core::ffi::c_int,
    pub nSelDepth: ::core::ffi::c_int,
    pub pEList: *mut ExprList,
    pub pCList: *mut ExprList

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_24 {
    pub i: u8_0,
    pub nChar: u8_0,
    pub code: u8_0

}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_READ: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const SQLITE_SELECT: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const SQLITE_RECURSIVE: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_COLUMN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OP_Goto: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const OP_Gosub: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const OP_InitCoroutine: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const OP_Yield: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const OP_MustBeInt: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const OP_Jump: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const OP_Once: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const OP_If: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const OP_IfNot: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const OP_NotFound: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const OP_Found: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
pub const OP_Last: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const OP_SorterSort: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const OP_Sort: ::core::ffi::c_int = 35 as ::core::ffi::c_int;
pub const OP_Rewind: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const OP_IfEmpty: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const OP_SorterNext: ::core::ffi::c_int = 38 as ::core::ffi::c_int;
pub const OP_Next: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const OP_IdxLE: ::core::ffi::c_int = 41 as ::core::ffi::c_int;
pub const OP_IsNull: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
pub const OP_Ne: ::core::ffi::c_int = 53 as ::core::ffi::c_int;
pub const OP_Eq: ::core::ffi::c_int = 54 as ::core::ffi::c_int;
pub const OP_IfPos: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const OP_IfNotZero: ::core::ffi::c_int = 61 as ::core::ffi::c_int;
pub const OP_DecrJumpZero: ::core::ffi::c_int = 62 as ::core::ffi::c_int;
pub const OP_Return: ::core::ffi::c_int = 68 as ::core::ffi::c_int;
pub const OP_Integer: ::core::ffi::c_int = 72 as ::core::ffi::c_int;
pub const OP_Null: ::core::ffi::c_int = 76 as ::core::ffi::c_int;
pub const OP_Copy: ::core::ffi::c_int = 81 as ::core::ffi::c_int;
pub const OP_SCopy: ::core::ffi::c_int = 82 as ::core::ffi::c_int;
pub const OP_ResultRow: ::core::ffi::c_int = 85 as ::core::ffi::c_int;
pub const OP_CollSeq: ::core::ffi::c_int = 86 as ::core::ffi::c_int;
pub const OP_AddImm: ::core::ffi::c_int = 87 as ::core::ffi::c_int;
pub const OP_Permutation: ::core::ffi::c_int = 90 as ::core::ffi::c_int;
pub const OP_Compare: ::core::ffi::c_int = 91 as ::core::ffi::c_int;
pub const OP_Column: ::core::ffi::c_int = 95 as ::core::ffi::c_int;
pub const OP_MakeRecord: ::core::ffi::c_int = 98 as ::core::ffi::c_int;
pub const OP_Count: ::core::ffi::c_int = 99 as ::core::ffi::c_int;
pub const OP_OpenRead: ::core::ffi::c_int = 113 as ::core::ffi::c_int;
pub const OP_OpenDup: ::core::ffi::c_int = 116 as ::core::ffi::c_int;
pub const OP_OpenEphemeral: ::core::ffi::c_int = 119 as ::core::ffi::c_int;
pub const OP_SorterOpen: ::core::ffi::c_int = 120 as ::core::ffi::c_int;
pub const OP_SequenceTest: ::core::ffi::c_int = 121 as ::core::ffi::c_int;
pub const OP_OpenPseudo: ::core::ffi::c_int = 122 as ::core::ffi::c_int;
pub const OP_Close: ::core::ffi::c_int = 123 as ::core::ffi::c_int;
pub const OP_Sequence: ::core::ffi::c_int = 127 as ::core::ffi::c_int;
pub const OP_NewRowid: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const OP_Insert: ::core::ffi::c_int = 129 as ::core::ffi::c_int;
pub const OP_Delete: ::core::ffi::c_int = 131 as ::core::ffi::c_int;
pub const OP_SorterData: ::core::ffi::c_int = 134 as ::core::ffi::c_int;
pub const OP_RowData: ::core::ffi::c_int = 135 as ::core::ffi::c_int;
pub const OP_NullRow: ::core::ffi::c_int = 137 as ::core::ffi::c_int;
pub const OP_IdxInsert: ::core::ffi::c_int = 139 as ::core::ffi::c_int;
pub const OP_SorterInsert: ::core::ffi::c_int = 140 as ::core::ffi::c_int;
pub const OP_IdxDelete: ::core::ffi::c_int = 141 as ::core::ffi::c_int;
pub const OP_ResetSorter: ::core::ffi::c_int = 147 as ::core::ffi::c_int;
pub const OP_OffsetLimit: ::core::ffi::c_int = 161 as ::core::ffi::c_int;
pub const OP_AggStep: ::core::ffi::c_int = 163 as ::core::ffi::c_int;
pub const OP_AggFinal: ::core::ffi::c_int = 166 as ::core::ffi::c_int;
pub const OP_GetSubtype: ::core::ffi::c_int = 182 as ::core::ffi::c_int;
pub const OP_SetSubtype: ::core::ffi::c_int = 183 as ::core::ffi::c_int;
pub const OP_FilterAdd: ::core::ffi::c_int = 184 as ::core::ffi::c_int;
pub const OP_Explain: ::core::ffi::c_int = 189 as ::core::ffi::c_int;
pub const SORTFLAG_UseSorter: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
unsafe extern "C" fn clearSelect(
    mut db: *mut sqlite3,
    mut p: *mut Select,
    mut bFree: ::core::ffi::c_int,
) {
    unsafe {
        while !p.is_null() {
            let mut pPrior: *mut Select = (*p).pPrior;
            sqlite3ExprListDelete(db, (*p).pEList);
            sqlite3SrcListDelete(db, (*p).pSrc);
            sqlite3ExprDelete(db, (*p).pWhere);
            sqlite3ExprListDelete(db, (*p).pGroupBy);
            sqlite3ExprDelete(db, (*p).pHaving);
            sqlite3ExprListDelete(db, (*p).pOrderBy);
            sqlite3ExprDelete(db, (*p).pLimit);
            if !(*p).pWith.is_null() {
                sqlite3WithDelete(db, (*p).pWith);
            }
            if !(*p).pWinDefn.is_null() {
                sqlite3WindowListDelete(db, (*p).pWinDefn);
            }
            while !(*p).pWin.is_null() {
                sqlite3WindowUnlinkFromSelect((*p).pWin);
            }
            if bFree != 0 {
                sqlite3DbNNFreeNN(db, p as *mut ::core::ffi::c_void);
            }
            p = pPrior;
            bFree = 1 as ::core::ffi::c_int;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SelectDestInit(
    mut pDest: *mut SelectDest,
    mut eDest: ::core::ffi::c_int,
    mut iParm: ::core::ffi::c_int,
) {
    unsafe {
        (*pDest).eDest = eDest as u8_0;
        (*pDest).iSDParm = iParm;
        (*pDest).iSDParm2 = 0 as ::core::ffi::c_int;
        (*pDest).zAffSdst = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*pDest).iSdst = 0 as ::core::ffi::c_int;
        (*pDest).nSdst = 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SelectNew(
    mut pParse: *mut Parse,
    mut pEList: *mut ExprList,
    mut pSrc: *mut SrcList,
    mut pWhere: *mut Expr,
    mut pGroupBy: *mut ExprList,
    mut pHaving: *mut Expr,
    mut pOrderBy: *mut ExprList,
    mut selFlags: u32_0,
    mut pLimit: *mut Expr,
) -> *mut Select {
    unsafe {
        let mut pNew: *mut Select = ::core::ptr::null_mut::<Select>();
        let mut pAllocated: *mut Select = ::core::ptr::null_mut::<Select>();
        let mut standin: Select = Select {
            op: 0,
            nSelectRow: 0,
            selFlags: 0,
            iLimit: 0,
            iOffset: 0,
            selId: 0,
            addrOpenEphm: [0; 2],
            pEList: ::core::ptr::null_mut::<ExprList>(),
            pSrc: ::core::ptr::null_mut::<SrcList>(),
            pWhere: ::core::ptr::null_mut::<Expr>(),
            pGroupBy: ::core::ptr::null_mut::<ExprList>(),
            pHaving: ::core::ptr::null_mut::<Expr>(),
            pOrderBy: ::core::ptr::null_mut::<ExprList>(),
            pPrior: ::core::ptr::null_mut::<Select>(),
            pNext: ::core::ptr::null_mut::<Select>(),
            pLimit: ::core::ptr::null_mut::<Expr>(),
            pWith: ::core::ptr::null_mut::<With>(),
            pWin: ::core::ptr::null_mut::<Window>(),
            pWinDefn: ::core::ptr::null_mut::<Window>()

        };
        pNew = sqlite3DbMallocRawNN(
            (*pParse).db,
            ::core::mem::size_of::<Select>() as u64_0,
        ) as *mut Select;
        pAllocated = pNew;
        if pNew.is_null() {
            pNew = &raw mut standin;
        }
        if pEList.is_null() {
            pEList = sqlite3ExprListAppend(
                pParse,
                ::core::ptr::null_mut::<ExprList>(),
                sqlite3Expr(
                    (*pParse).db,
                    TK_ASTERISK,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                ),
            );
        }
        (*pNew).pEList = pEList;
        (*pNew).op = TK_SELECT as u8_0;
        (*pNew).selFlags = selFlags;
        (*pNew).iLimit = 0 as ::core::ffi::c_int;
        (*pNew).iOffset = 0 as ::core::ffi::c_int;
        (*pParse).nSelect += 1;
        (*pNew).selId = (*pParse).nSelect as u32_0;
        (*pNew).addrOpenEphm[0 as ::core::ffi::c_int as usize] = -1
            as ::core::ffi::c_int;
        (*pNew).addrOpenEphm[1 as ::core::ffi::c_int as usize] = -1
            as ::core::ffi::c_int;
        (*pNew).nSelectRow = 0 as LogEst;
        if pSrc.is_null() {
            pSrc = sqlite3DbMallocZero((*pParse).db, SZ_SRCLIST_1 as u64_0)
                as *mut SrcList;
        }
        (*pNew).pSrc = pSrc;
        (*pNew).pWhere = pWhere;
        (*pNew).pGroupBy = pGroupBy;
        (*pNew).pHaving = pHaving;
        (*pNew).pOrderBy = pOrderBy;
        (*pNew).pPrior = ::core::ptr::null_mut::<Select>();
        (*pNew).pNext = ::core::ptr::null_mut::<Select>();
        (*pNew).pLimit = pLimit;
        (*pNew).pWith = ::core::ptr::null_mut::<With>();
        (*pNew).pWin = ::core::ptr::null_mut::<Window>();
        (*pNew).pWinDefn = ::core::ptr::null_mut::<Window>();
        if (*(*pParse).db).mallocFailed != 0 {
            clearSelect(
                (*pParse).db,
                pNew,
                (pNew != &raw mut standin) as ::core::ffi::c_int,
            );
            pAllocated = ::core::ptr::null_mut::<Select>();
        }
        return pAllocated;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SelectDelete(mut db: *mut sqlite3, mut p: *mut Select) {
    unsafe {
        if !p.is_null() {
            clearSelect(db, p, 1 as ::core::ffi::c_int);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SelectDeleteGeneric(
    mut db: *mut sqlite3,
    mut p: *mut ::core::ffi::c_void,
) {
    unsafe {
        if !p.is_null() {
            clearSelect(db, p as *mut Select, 1 as ::core::ffi::c_int);
        }
    }
}
unsafe extern "C" fn findRightmost(mut p: *mut Select) -> *mut Select {
    unsafe {
        while !(*p).pNext.is_null() {
            p = (*p).pNext;
        }
        return p;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3JoinType(
    mut pParse: *mut Parse,
    mut pA: *mut Token,
    mut pB: *mut Token,
    mut pC: *mut Token,
) -> ::core::ffi::c_int {
    unsafe {
        let mut jointype: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut apAll: [*mut Token; 3] = [::core::ptr::null_mut::<Token>(); 3];
        let mut p: *mut Token = ::core::ptr::null_mut::<Token>();
        static mut zKeyText: [::core::ffi::c_char; 34] = unsafe {
            ::core::mem::transmute::<
                [u8; 34],
                [::core::ffi::c_char; 34],
            >(*b"naturaleftouterightfullinnercross\0")
        };
        static mut aKeyword: [C2Rust_Unnamed_24; 7] = [
            C2Rust_Unnamed_24 {
                i: 0 as u8_0,
                nChar: 7 as u8_0,
                code: JT_NATURAL as u8_0

            },
            C2Rust_Unnamed_24 {
                i: 6 as u8_0,
                nChar: 4 as u8_0,
                code: (JT_LEFT | JT_OUTER) as u8_0

            },
            C2Rust_Unnamed_24 {
                i: 10 as u8_0,
                nChar: 5 as u8_0,
                code: JT_OUTER as u8_0

            },
            C2Rust_Unnamed_24 {
                i: 14 as u8_0,
                nChar: 5 as u8_0,
                code: (JT_RIGHT | JT_OUTER) as u8_0

            },
            C2Rust_Unnamed_24 {
                i: 19 as u8_0,
                nChar: 4 as u8_0,
                code: (JT_LEFT | JT_RIGHT | JT_OUTER) as u8_0

            },
            C2Rust_Unnamed_24 {
                i: 23 as u8_0,
                nChar: 5 as u8_0,
                code: JT_INNER as u8_0

            },
            C2Rust_Unnamed_24 {
                i: 28 as u8_0,
                nChar: 5 as u8_0,
                code: (JT_INNER | JT_CROSS) as u8_0

            },
        ];
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        apAll[0 as ::core::ffi::c_int as usize] = pA;
        apAll[1 as ::core::ffi::c_int as usize] = pB;
        apAll[2 as ::core::ffi::c_int as usize] = pC;
        i = 0 as ::core::ffi::c_int;
        while i < 3 as ::core::ffi::c_int && !apAll[i as usize].is_null() {
            p = apAll[i as usize];
            j = 0 as ::core::ffi::c_int;
            while j
                < (::core::mem::size_of::<[C2Rust_Unnamed_24; 7]>() as usize)
                    .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_24>() as usize)
                    as ::core::ffi::c_int
            {
                if (*p).n == aKeyword[j as usize].nChar as ::core::ffi::c_uint
                    && sqlite3_strnicmp(
                        (*p).z as *mut ::core::ffi::c_char,
                        (&raw const zKeyText as *const ::core::ffi::c_char)
                            .offset(
                                (*(&raw const aKeyword as *const C2Rust_Unnamed_24)
                                    .offset(j as isize))
                                    .i as isize,
                            ) as *const ::core::ffi::c_char,
                        (*p).n as ::core::ffi::c_int,
                    ) == 0 as ::core::ffi::c_int
                {
                    jointype |= aKeyword[j as usize].code as ::core::ffi::c_int;
                    break;
                } else {
                    j += 1;
                }
            }
            if j
                >= (::core::mem::size_of::<[C2Rust_Unnamed_24; 7]>() as usize)
                    .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_24>() as usize)
                    as ::core::ffi::c_int
            {
                jointype |= JT_ERROR;
                break;
            } else {
                i += 1;
            }
        }
        if jointype & (JT_INNER | JT_OUTER) == JT_INNER | JT_OUTER
            || jointype & JT_ERROR != 0 as ::core::ffi::c_int
            || jointype & (JT_OUTER | JT_LEFT | JT_RIGHT) == JT_OUTER
        {
            let mut zSp1: *const ::core::ffi::c_char = b" \0".as_ptr()
                as *const ::core::ffi::c_char;
            let mut zSp2: *const ::core::ffi::c_char = b" \0".as_ptr()
                as *const ::core::ffi::c_char;
            if pB.is_null() {
                zSp1 = zSp1.offset(1);
            }
            if pC.is_null() {
                zSp2 = zSp2.offset(1);
            }
            sqlite3ErrorMsg(
                pParse,
                b"unknown join type: %T%s%T%s%T\0".as_ptr()
                    as *const ::core::ffi::c_char,
                pA,
                zSp1,
                pB,
                zSp2,
                pC,
            );
            jointype = JT_INNER;
        }
        return jointype;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ColumnIndex(
    mut pTab: *mut Table,
    mut zCol: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut h: u8_0 = 0;
        let mut aCol: *const Column = ::core::ptr::null::<Column>();
        let mut nCol: ::core::ffi::c_int = 0;
        h = sqlite3StrIHash(zCol);
        aCol = (*pTab).aCol;
        nCol = (*pTab).nCol as ::core::ffi::c_int;
        i = (*pTab)
            .aHx[(h as usize).wrapping_rem(::core::mem::size_of::<[u8_0; 16]>() as usize)
            as usize] as ::core::ffi::c_int;
        if (*aCol.offset(i as isize)).hName as ::core::ffi::c_int
            == h as ::core::ffi::c_int
            && sqlite3StrICmp((*aCol.offset(i as isize)).zCnName, zCol)
                == 0 as ::core::ffi::c_int
        {
            return i;
        }
        i = 0 as ::core::ffi::c_int;
        loop {
            if (*aCol.offset(i as isize)).hName as ::core::ffi::c_int
                == h as ::core::ffi::c_int
                && sqlite3StrICmp((*aCol.offset(i as isize)).zCnName, zCol)
                    == 0 as ::core::ffi::c_int
            {
                return i;
            }
            i += 1;
            if i >= nCol {
                break;
            }
        }
        return -1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SrcItemColumnUsed(
    mut pItem: *mut SrcItem,
    mut iCol: ::core::ffi::c_int,
) {
    unsafe {
        if (*pItem).fg.isNestedFrom() != 0 {
            let mut pResults: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
            pResults = (*(*(*pItem).u4.pSubq).pSelect).pEList;
            let ref mut c2rust_fresh5 = (*(&raw mut (*pResults).a as *mut ExprList_item)
                .offset(iCol as isize))
                .fg;
            (*c2rust_fresh5).set_bUsed(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
    }
}
unsafe extern "C" fn tableAndColumnIndex(
    mut pSrc: *mut SrcList,
    mut iStart: ::core::ffi::c_int,
    mut iEnd: ::core::ffi::c_int,
    mut zCol: *const ::core::ffi::c_char,
    mut piTab: *mut ::core::ffi::c_int,
    mut piCol: *mut ::core::ffi::c_int,
    mut bIgnoreHidden: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut iCol: ::core::ffi::c_int = 0;
        i = iStart;
        while i <= iEnd {
            iCol = sqlite3ColumnIndex(
                (*(&raw mut (*pSrc).a as *mut SrcItem).offset(i as isize)).pSTab,
                zCol,
            );
            if iCol >= 0 as ::core::ffi::c_int
                && (bIgnoreHidden == 0 as ::core::ffi::c_int
                    || ((*(*(*(&raw mut (*pSrc).a as *mut SrcItem).offset(i as isize))
                        .pSTab)
                        .aCol
                        .offset(iCol as isize))
                        .colFlags as ::core::ffi::c_int & COLFLAG_HIDDEN
                        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int)
            {
                if !piTab.is_null() {
                    sqlite3SrcItemColumnUsed(
                        (&raw mut (*pSrc).a as *mut SrcItem).offset(i as isize)
                            as *mut SrcItem,
                        iCol,
                    );
                    *piTab = i;
                    *piCol = iCol;
                }
                return 1 as ::core::ffi::c_int;
            }
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SetJoinExpr(
    mut p: *mut Expr,
    mut iTable: ::core::ffi::c_int,
    mut joinFlag: u32_0,
) {
    unsafe {
        while !p.is_null() {
            (*p).flags = ((*p).flags as ::core::ffi::c_uint
                | joinFlag as ::core::ffi::c_uint) as u32_0;
            (*p).w.iJoin = iTable;
            if (*p).flags & EP_xIsSelect as u32_0 == 0 as u32_0 {
                if !(*p).x.pList.is_null() {
                    let mut i: ::core::ffi::c_int = 0;
                    i = 0 as ::core::ffi::c_int;
                    while i < (*(*p).x.pList).nExpr {
                        sqlite3SetJoinExpr(
                            (*(&raw mut (*(*p).x.pList).a as *mut ExprList_item)
                                .offset(i as isize))
                                .pExpr,
                            iTable,
                            joinFlag,
                        );
                        i += 1;
                    }
                }
            }
            sqlite3SetJoinExpr((*p).pLeft, iTable, joinFlag);
            p = (*p).pRight;
        }
    }
}
unsafe extern "C" fn unsetJoinExpr(
    mut p: *mut Expr,
    mut iTable: ::core::ffi::c_int,
    mut nullable: ::core::ffi::c_int,
) {
    unsafe {
        while !p.is_null() {
            if iTable < 0 as ::core::ffi::c_int
                || (*p).flags & 0x1 as ::core::ffi::c_int as u32_0 != 0 as u32_0
                    && (*p).w.iJoin == iTable
            {
                (*p).flags = ((*p).flags as ::core::ffi::c_uint
                    & !((0x1 as ::core::ffi::c_int | 0x2 as ::core::ffi::c_int) as u32_0)
                        as ::core::ffi::c_uint) as u32_0;
                if iTable >= 0 as ::core::ffi::c_int {
                    (*p).flags = ((*p).flags as ::core::ffi::c_uint
                        | 0x2 as ::core::ffi::c_int as u32_0 as ::core::ffi::c_uint)
                        as u32_0;
                }
            }
            if (*p).op as ::core::ffi::c_int == TK_COLUMN && (*p).iTable == iTable
                && nullable == 0
            {
                (*p).flags = ((*p).flags as ::core::ffi::c_uint
                    & !(0x200000 as ::core::ffi::c_int as u32_0) as ::core::ffi::c_uint)
                    as u32_0;
            }
            if (*p).op as ::core::ffi::c_int == TK_FUNCTION {
                if !(*p).x.pList.is_null() {
                    let mut i: ::core::ffi::c_int = 0;
                    i = 0 as ::core::ffi::c_int;
                    while i < (*(*p).x.pList).nExpr {
                        unsetJoinExpr(
                            (*(&raw mut (*(*p).x.pList).a as *mut ExprList_item)
                                .offset(i as isize))
                                .pExpr,
                            iTable,
                            nullable,
                        );
                        i += 1;
                    }
                }
            }
            unsetJoinExpr((*p).pLeft, iTable, nullable);
            p = (*p).pRight;
        }
    }
}
unsafe extern "C" fn sqlite3ProcessJoin(
    mut pParse: *mut Parse,
    mut p: *mut Select,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pSrc: *mut SrcList = ::core::ptr::null_mut::<SrcList>();
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut pLeft: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
        let mut pRight: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
        pSrc = (*p).pSrc;
        pLeft = (&raw mut (*pSrc).a as *mut SrcItem)
            .offset(0 as ::core::ffi::c_int as isize) as *mut SrcItem;
        pRight = pLeft.offset(1 as ::core::ffi::c_int as isize) as *mut SrcItem;
        i = 0 as ::core::ffi::c_int;
        while i < (*pSrc).nSrc - 1 as ::core::ffi::c_int {
            let mut pRightTab: *mut Table = (*pRight).pSTab;
            let mut joinType: u32_0 = 0;
            if !((*pLeft).pSTab.is_null() || pRightTab.is_null()) {
                joinType = (if (*pRight).fg.jointype as ::core::ffi::c_int & JT_OUTER
                    != 0 as ::core::ffi::c_int
                {
                    EP_OuterON
                } else {
                    EP_InnerON
                }) as u32_0;
                if (*pRight).fg.jointype as ::core::ffi::c_int & JT_NATURAL != 0 {
                    let mut pUsing: *mut IdList = ::core::ptr::null_mut::<IdList>();
                    if (*pRight).fg.isUsing() as ::core::ffi::c_int != 0
                        || !(*pRight).u3.pOn.is_null()
                    {
                        sqlite3ErrorMsg(
                            pParse,
                            b"a NATURAL join may not have an ON or USING clause\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            0 as ::core::ffi::c_int,
                        );
                        return 1 as ::core::ffi::c_int;
                    }
                    j = 0 as ::core::ffi::c_int;
                    while j < (*pRightTab).nCol as ::core::ffi::c_int {
                        let mut zName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                            ::core::ffi::c_char,
                        >();
                        if !((*(*pRightTab).aCol.offset(j as isize)).colFlags
                            as ::core::ffi::c_int & COLFLAG_HIDDEN
                            != 0 as ::core::ffi::c_int)
                        {
                            zName = (*(*pRightTab).aCol.offset(j as isize)).zCnName;
                            if tableAndColumnIndex(
                                pSrc,
                                0 as ::core::ffi::c_int,
                                i,
                                zName,
                                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                                1 as ::core::ffi::c_int,
                            ) != 0
                            {
                                pUsing = sqlite3IdListAppend(
                                    pParse,
                                    pUsing,
                                    ::core::ptr::null_mut::<Token>(),
                                );
                                if !pUsing.is_null() {
                                    let ref mut c2rust_fresh4 = (*(&raw mut (*pUsing).a
                                        as *mut IdList_item)
                                        .offset(((*pUsing).nId - 1 as ::core::ffi::c_int) as isize))
                                        .zName;
                                    *c2rust_fresh4 = sqlite3DbStrDup((*pParse).db, zName);
                                }
                            }
                        }
                        j += 1;
                    }
                    if !pUsing.is_null() {
                        (*pRight)
                            .fg
                            .set_isUsing(
                                1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                            );
                        (*pRight)
                            .fg
                            .set_isSynthUsing(
                                1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                            );
                        (*pRight).u3.pUsing = pUsing;
                    }
                    if (*pParse).nErr != 0 {
                        return 1 as ::core::ffi::c_int;
                    }
                }
                if (*pRight).fg.isUsing() != 0 {
                    let mut pList: *mut IdList = (*pRight).u3.pUsing;
                    let mut db: *mut sqlite3 = (*pParse).db;
                    j = 0 as ::core::ffi::c_int;
                    while j < (*pList).nId {
                        let mut zName_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                            ::core::ffi::c_char,
                        >();
                        let mut iLeft: ::core::ffi::c_int = 0;
                        let mut iLeftCol: ::core::ffi::c_int = 0;
                        let mut iRightCol: ::core::ffi::c_int = 0;
                        let mut pE1: *mut Expr = ::core::ptr::null_mut::<Expr>();
                        let mut pE2: *mut Expr = ::core::ptr::null_mut::<Expr>();
                        let mut pEq: *mut Expr = ::core::ptr::null_mut::<Expr>();
                        zName_0 = (*(&raw mut (*pList).a as *mut IdList_item)
                            .offset(j as isize))
                            .zName;
                        iRightCol = sqlite3ColumnIndex(pRightTab, zName_0);
                        if iRightCol < 0 as ::core::ffi::c_int
                            || tableAndColumnIndex(
                                pSrc,
                                0 as ::core::ffi::c_int,
                                i,
                                zName_0,
                                &raw mut iLeft,
                                &raw mut iLeftCol,
                                (*pRight).fg.isSynthUsing() as ::core::ffi::c_int,
                            ) == 0 as ::core::ffi::c_int
                        {
                            sqlite3ErrorMsg(
                                pParse,
                                b"cannot join using column %s - column not present in both tables\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                zName_0,
                            );
                            return 1 as ::core::ffi::c_int;
                        }
                        pE1 = sqlite3CreateColumnExpr(db, pSrc, iLeft, iLeftCol);
                        sqlite3SrcItemColumnUsed(
                            (&raw mut (*pSrc).a as *mut SrcItem).offset(iLeft as isize)
                                as *mut SrcItem,
                            iLeftCol,
                        );
                        if (*(&raw mut (*pSrc).a as *mut SrcItem)
                            .offset(0 as ::core::ffi::c_int as isize))
                            .fg
                            .jointype as ::core::ffi::c_int & JT_LTORJ
                            != 0 as ::core::ffi::c_int
                            && (*pParse).nErr == 0 as ::core::ffi::c_int
                        {
                            let mut pFuncArgs: *mut ExprList = ::core::ptr::null_mut::<
                                ExprList,
                            >();
                            static mut tkCoalesce: Token = Token {
                                z: b"coalesce\0".as_ptr() as *const ::core::ffi::c_char,
                                n: 8 as ::core::ffi::c_uint

                            };
                            (*pE1).flags = ((*pE1).flags as ::core::ffi::c_uint
                                | 0x200000 as ::core::ffi::c_int as u32_0
                                    as ::core::ffi::c_uint) as u32_0;
                            while tableAndColumnIndex(
                                pSrc,
                                iLeft + 1 as ::core::ffi::c_int,
                                i,
                                zName_0,
                                &raw mut iLeft,
                                &raw mut iLeftCol,
                                (*pRight).fg.isSynthUsing() as ::core::ffi::c_int,
                            ) != 0 as ::core::ffi::c_int
                            {
                                if (*(&raw mut (*pSrc).a as *mut SrcItem)
                                    .offset(iLeft as isize))
                                    .fg
                                    .isUsing() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                    || sqlite3IdListIndex(
                                        (*(&raw mut (*pSrc).a as *mut SrcItem)
                                            .offset(iLeft as isize))
                                            .u3
                                            .pUsing,
                                        zName_0,
                                    ) < 0 as ::core::ffi::c_int
                                {
                                    sqlite3ErrorMsg(
                                        pParse,
                                        b"ambiguous reference to %s in USING()\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        zName_0,
                                    );
                                    break;
                                } else {
                                    pFuncArgs = sqlite3ExprListAppend(pParse, pFuncArgs, pE1);
                                    pE1 = sqlite3CreateColumnExpr(db, pSrc, iLeft, iLeftCol);
                                    sqlite3SrcItemColumnUsed(
                                        (&raw mut (*pSrc).a as *mut SrcItem).offset(iLeft as isize)
                                            as *mut SrcItem,
                                        iLeftCol,
                                    );
                                }
                            }
                            if !pFuncArgs.is_null() {
                                pFuncArgs = sqlite3ExprListAppend(pParse, pFuncArgs, pE1);
                                pE1 = sqlite3ExprFunction(
                                    pParse,
                                    pFuncArgs,
                                    &raw const tkCoalesce,
                                    0 as ::core::ffi::c_int,
                                );
                                if !pE1.is_null() {
                                    (*pE1).affExpr = SQLITE_AFF_DEFER as ::core::ffi::c_char;
                                }
                            }
                        } else if (*(&raw mut (*pSrc).a as *mut SrcItem)
                            .offset((i + 1 as ::core::ffi::c_int) as isize))
                            .fg
                            .jointype as ::core::ffi::c_int & JT_LEFT
                            != 0 as ::core::ffi::c_int
                            && (*pParse).nErr == 0 as ::core::ffi::c_int
                        {
                            (*pE1).flags = ((*pE1).flags as ::core::ffi::c_uint
                                | 0x200000 as ::core::ffi::c_int as u32_0
                                    as ::core::ffi::c_uint) as u32_0;
                        }
                        pE2 = sqlite3CreateColumnExpr(
                            db,
                            pSrc,
                            i + 1 as ::core::ffi::c_int,
                            iRightCol,
                        );
                        sqlite3SrcItemColumnUsed(pRight, iRightCol);
                        pEq = sqlite3PExpr(pParse, TK_EQ, pE1, pE2);
                        if !pEq.is_null() {
                            (*pEq).flags = ((*pEq).flags as ::core::ffi::c_uint
                                | joinType as ::core::ffi::c_uint) as u32_0;
                            (*pEq).w.iJoin = (*pE2).iTable;
                        }
                        (*p).pWhere = sqlite3ExprAnd(pParse, (*p).pWhere, pEq);
                        j += 1;
                    }
                } else if !(*pRight).u3.pOn.is_null() {
                    sqlite3SetJoinExpr((*pRight).u3.pOn, (*pRight).iCursor, joinType);
                    (*p).pWhere = sqlite3ExprAnd(pParse, (*p).pWhere, (*pRight).u3.pOn);
                    (*pRight).u3.pOn = ::core::ptr::null_mut::<Expr>();
                    (*pRight)
                        .fg
                        .set_isOn(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    (*p).selFlags = ((*p).selFlags as ::core::ffi::c_uint
                        | SF_OnToWhere as ::core::ffi::c_uint) as u32_0;
                }
            }
            i += 1;
            pRight = pRight.offset(1);
            pLeft = pLeft.offset(1);
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn innerLoopLoadRow(
    mut pParse: *mut Parse,
    mut pSelect: *mut Select,
    mut pInfo: *mut RowLoadInfo,
) {
    unsafe {
        sqlite3ExprCodeExprList(
            pParse,
            (*pSelect).pEList,
            (*pInfo).regResult,
            0 as ::core::ffi::c_int,
            (*pInfo).ecelFlags,
        );
    }
}
unsafe extern "C" fn makeSorterRecord(
    mut pParse: *mut Parse,
    mut pSort: *mut SortCtx,
    mut pSelect: *mut Select,
    mut regBase: ::core::ffi::c_int,
    mut nBase: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nOBSat: ::core::ffi::c_int = (*pSort).nOBSat;
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        (*pParse).nMem += 1;
        let mut regOut: ::core::ffi::c_int = (*pParse).nMem;
        if !(*pSort).pDeferredRowLoad.is_null() {
            innerLoopLoadRow(
                pParse,
                pSelect,
                (*pSort).pDeferredRowLoad as *mut RowLoadInfo,
            );
        }
        sqlite3VdbeAddOp3(v, OP_MakeRecord, regBase + nOBSat, nBase - nOBSat, regOut);
        return regOut;
    }
}
unsafe extern "C" fn pushOntoSorter(
    mut pParse: *mut Parse,
    mut pSort: *mut SortCtx,
    mut pSelect: *mut Select,
    mut regData: ::core::ffi::c_int,
    mut regOrigData: ::core::ffi::c_int,
    mut nData: ::core::ffi::c_int,
    mut nPrefixReg: ::core::ffi::c_int,
) {
    unsafe {
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        let mut bSeq: ::core::ffi::c_int = ((*pSort).sortFlags as ::core::ffi::c_int
            & SORTFLAG_UseSorter == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        let mut nExpr: ::core::ffi::c_int = (*(*pSort).pOrderBy).nExpr;
        let mut nBase: ::core::ffi::c_int = nExpr + bSeq + nData;
        let mut regBase: ::core::ffi::c_int = 0;
        let mut regRecord: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nOBSat: ::core::ffi::c_int = (*pSort).nOBSat;
        let mut op: ::core::ffi::c_int = 0;
        let mut iLimit: ::core::ffi::c_int = 0;
        let mut iSkip: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if nPrefixReg != 0 {
            regBase = regData - nPrefixReg;
        } else {
            regBase = (*pParse).nMem + 1 as ::core::ffi::c_int;
            (*pParse).nMem += nBase;
        }
        iLimit = if (*pSelect).iOffset != 0 {
            (*pSelect).iOffset + 1 as ::core::ffi::c_int
        } else {
            (*pSelect).iLimit
        };
        (*pSort).labelDone = sqlite3VdbeMakeLabel(pParse);
        sqlite3ExprCodeExprList(
            pParse,
            (*pSort).pOrderBy,
            regBase,
            regOrigData,
            (SQLITE_ECEL_DUP
                | (if regOrigData != 0 {
                    SQLITE_ECEL_REF
                } else {
                    0 as ::core::ffi::c_int
                })) as u8_0,
        );
        if bSeq != 0 {
            sqlite3VdbeAddOp2(v, OP_Sequence, (*pSort).iECursor, regBase + nExpr);
        }
        if nPrefixReg == 0 as ::core::ffi::c_int && nData > 0 as ::core::ffi::c_int {
            sqlite3ExprCodeMove(pParse, regData, regBase + nExpr + bSeq, nData);
        }
        if nOBSat > 0 as ::core::ffi::c_int {
            let mut regPrevKey: ::core::ffi::c_int = 0;
            let mut addrFirst: ::core::ffi::c_int = 0;
            let mut addrJmp: ::core::ffi::c_int = 0;
            let mut pOp: *mut VdbeOp = ::core::ptr::null_mut::<VdbeOp>();
            let mut nKey: ::core::ffi::c_int = 0;
            let mut pKI: *mut KeyInfo = ::core::ptr::null_mut::<KeyInfo>();
            regRecord = makeSorterRecord(pParse, pSort, pSelect, regBase, nBase);
            regPrevKey = (*pParse).nMem + 1 as ::core::ffi::c_int;
            (*pParse).nMem += (*pSort).nOBSat;
            nKey = nExpr - (*pSort).nOBSat + bSeq;
            if bSeq != 0 {
                addrFirst = sqlite3VdbeAddOp1(v, OP_IfNot, regBase + nExpr);
            } else {
                addrFirst = sqlite3VdbeAddOp1(v, OP_SequenceTest, (*pSort).iECursor);
            }
            sqlite3VdbeAddOp3(v, OP_Compare, regPrevKey, regBase, (*pSort).nOBSat);
            pOp = sqlite3VdbeGetOp(v, (*pSort).addrSortIndex);
            if (*(*pParse).db).mallocFailed != 0 {
                return;
            }
            (*pOp).p2 = nKey + nData;
            pKI = (*pOp).p4.pKeyInfo;
            memset(
                (*pKI).aSortFlags as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (*pKI).nKeyField as size_t,
            );
            sqlite3VdbeChangeP4(
                v,
                -1 as ::core::ffi::c_int,
                pKI as *mut ::core::ffi::c_char,
                P4_KEYINFO,
            );
            (*pOp).p4.pKeyInfo = sqlite3KeyInfoFromExprList(
                pParse,
                (*pSort).pOrderBy,
                nOBSat,
                (*pKI).nAllField as ::core::ffi::c_int
                    - (*pKI).nKeyField as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
            );
            pOp = ::core::ptr::null_mut::<VdbeOp>();
            addrJmp = sqlite3VdbeCurrentAddr(v);
            sqlite3VdbeAddOp3(
                v,
                OP_Jump,
                addrJmp + 1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                addrJmp + 1 as ::core::ffi::c_int,
            );
            (*pSort).labelBkOut = sqlite3VdbeMakeLabel(pParse);
            (*pParse).nMem += 1;
            (*pSort).regReturn = (*pParse).nMem;
            sqlite3VdbeAddOp2(v, OP_Gosub, (*pSort).regReturn, (*pSort).labelBkOut);
            sqlite3VdbeAddOp1(v, OP_ResetSorter, (*pSort).iECursor);
            if iLimit != 0 {
                sqlite3VdbeAddOp2(v, OP_IfNot, iLimit, (*pSort).labelDone);
            }
            sqlite3VdbeJumpHere(v, addrFirst);
            sqlite3ExprCodeMove(pParse, regBase, regPrevKey, (*pSort).nOBSat);
            sqlite3VdbeJumpHere(v, addrJmp);
        }
        if iLimit != 0 {
            let mut iCsr: ::core::ffi::c_int = (*pSort).iECursor;
            sqlite3VdbeAddOp2(
                v,
                OP_IfNotZero,
                iLimit,
                sqlite3VdbeCurrentAddr(v) + 4 as ::core::ffi::c_int,
            );
            sqlite3VdbeAddOp2(v, OP_Last, iCsr, 0 as ::core::ffi::c_int);
            iSkip = sqlite3VdbeAddOp4Int(
                v,
                OP_IdxLE,
                iCsr,
                0 as ::core::ffi::c_int,
                regBase + nOBSat,
                nExpr - nOBSat,
            );
            sqlite3VdbeAddOp1(v, OP_Delete, iCsr);
        }
        if regRecord == 0 as ::core::ffi::c_int {
            regRecord = makeSorterRecord(pParse, pSort, pSelect, regBase, nBase);
        }
        if (*pSort).sortFlags as ::core::ffi::c_int & SORTFLAG_UseSorter != 0 {
            op = OP_SorterInsert;
        } else {
            op = OP_IdxInsert;
        }
        sqlite3VdbeAddOp4Int(
            v,
            op,
            (*pSort).iECursor,
            regRecord,
            regBase + nOBSat,
            nBase - nOBSat,
        );
        if iSkip != 0 {
            sqlite3VdbeChangeP2(
                v,
                iSkip,
                if (*pSort).labelOBLopt != 0 {
                    (*pSort).labelOBLopt
                } else {
                    sqlite3VdbeCurrentAddr(v)
                },
            );
        }
    }
}
unsafe extern "C" fn codeOffset(
    mut v: *mut Vdbe,
    mut iOffset: ::core::ffi::c_int,
    mut iContinue: ::core::ffi::c_int,
) {
    unsafe {
        if iOffset > 0 as ::core::ffi::c_int {
            sqlite3VdbeAddOp3(v, OP_IfPos, iOffset, iContinue, 1 as ::core::ffi::c_int);
        }
    }
}
unsafe extern "C" fn codeDistinct(
    mut pParse: *mut Parse,
    mut eTnctType: ::core::ffi::c_int,
    mut iTab: ::core::ffi::c_int,
    mut addrRepeat: ::core::ffi::c_int,
    mut pEList: *mut ExprList,
    mut regElem: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iRet: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nResultCol: ::core::ffi::c_int = (*pEList).nExpr;
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        match eTnctType {
            WHERE_DISTINCT_ORDERED => {
                let mut i: ::core::ffi::c_int = 0;
                let mut iJump: ::core::ffi::c_int = 0;
                let mut regPrev: ::core::ffi::c_int = 0;
                regPrev = (*pParse).nMem + 1 as ::core::ffi::c_int;
                iRet = regPrev;
                (*pParse).nMem += nResultCol;
                iJump = sqlite3VdbeCurrentAddr(v) + nResultCol;
                i = 0 as ::core::ffi::c_int;
                while i < nResultCol {
                    let mut pColl: *mut CollSeq = sqlite3ExprCollSeq(
                        pParse,
                        (*(&raw mut (*pEList).a as *mut ExprList_item)
                            .offset(i as isize))
                            .pExpr,
                    );
                    if i < nResultCol - 1 as ::core::ffi::c_int {
                        sqlite3VdbeAddOp3(v, OP_Ne, regElem + i, iJump, regPrev + i);
                    } else {
                        sqlite3VdbeAddOp3(
                            v,
                            OP_Eq,
                            regElem + i,
                            addrRepeat,
                            regPrev + i,
                        );
                    }
                    sqlite3VdbeChangeP4(
                        v,
                        -1 as ::core::ffi::c_int,
                        pColl as *const ::core::ffi::c_char,
                        P4_COLLSEQ,
                    );
                    sqlite3VdbeChangeP5(v, SQLITE_NULLEQ as u16_0);
                    i += 1;
                }
                sqlite3VdbeAddOp3(
                    v,
                    OP_Copy,
                    regElem,
                    regPrev,
                    nResultCol - 1 as ::core::ffi::c_int,
                );
            }
            WHERE_DISTINCT_UNIQUE => {}
            _ => {
                let mut r1: ::core::ffi::c_int = sqlite3GetTempReg(pParse);
                sqlite3VdbeAddOp4Int(v, OP_Found, iTab, addrRepeat, regElem, nResultCol);
                sqlite3VdbeAddOp3(v, OP_MakeRecord, regElem, nResultCol, r1);
                sqlite3VdbeAddOp4Int(v, OP_IdxInsert, iTab, r1, regElem, nResultCol);
                sqlite3VdbeChangeP5(v, OPFLAG_USESEEKRESULT as u16_0);
                sqlite3ReleaseTempReg(pParse, r1);
                iRet = iTab;
            }
        }
        return iRet;
    }
}
unsafe extern "C" fn fixDistinctOpenEph(
    mut pParse: *mut Parse,
    mut eTnctType: ::core::ffi::c_int,
    mut iVal: ::core::ffi::c_int,
    mut iOpenEphAddr: ::core::ffi::c_int,
) {
    unsafe {
        if (*pParse).nErr == 0 as ::core::ffi::c_int
            && (eTnctType == WHERE_DISTINCT_UNIQUE
                || eTnctType == WHERE_DISTINCT_ORDERED)
        {
            let mut v: *mut Vdbe = (*pParse).pVdbe;
            sqlite3VdbeChangeToNoop(v, iOpenEphAddr);
            if (*sqlite3VdbeGetOp(v, iOpenEphAddr + 1 as ::core::ffi::c_int)).opcode
                as ::core::ffi::c_int == OP_Explain
            {
                sqlite3VdbeChangeToNoop(v, iOpenEphAddr + 1 as ::core::ffi::c_int);
            }
            if eTnctType == WHERE_DISTINCT_ORDERED {
                let mut pOp: *mut VdbeOp = sqlite3VdbeGetOp(v, iOpenEphAddr);
                (*pOp).opcode = OP_Null as u8_0;
                (*pOp).p1 = 1 as ::core::ffi::c_int;
                (*pOp).p2 = iVal;
            }
        }
    }
}
unsafe extern "C" fn selectInnerLoop(
    mut pParse: *mut Parse,
    mut p: *mut Select,
    mut srcTab: ::core::ffi::c_int,
    mut pSort: *mut SortCtx,
    mut pDistinct: *mut DistinctCtx,
    mut pDest: *mut SelectDest,
    mut iContinue: ::core::ffi::c_int,
    mut iBreak: ::core::ffi::c_int,
) {
    unsafe {
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        let mut i: ::core::ffi::c_int = 0;
        let mut hasDistinct: ::core::ffi::c_int = 0;
        let mut eDest: ::core::ffi::c_int = (*pDest).eDest as ::core::ffi::c_int;
        let mut iParm: ::core::ffi::c_int = (*pDest).iSDParm;
        let mut nResultCol: ::core::ffi::c_int = 0;
        let mut nPrefixReg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut sRowLoadInfo: RowLoadInfo = RowLoadInfo {
            regResult: 0,
            ecelFlags: 0

        };
        let mut regResult: ::core::ffi::c_int = 0;
        let mut regOrig: ::core::ffi::c_int = 0;
        hasDistinct = if !pDistinct.is_null() {
            (*pDistinct).eTnctType as ::core::ffi::c_int
        } else {
            WHERE_DISTINCT_NOOP
        };
        if !pSort.is_null() && (*pSort).pOrderBy.is_null() {
            pSort = ::core::ptr::null_mut::<SortCtx>();
        }
        if pSort.is_null() && hasDistinct == 0 {
            codeOffset(v, (*p).iOffset, iContinue);
        }
        nResultCol = (*(*p).pEList).nExpr;
        if (*pDest).iSdst == 0 as ::core::ffi::c_int {
            if !pSort.is_null() {
                nPrefixReg = (*(*pSort).pOrderBy).nExpr;
                if (*pSort).sortFlags as ::core::ffi::c_int & SORTFLAG_UseSorter == 0 {
                    nPrefixReg += 1;
                }
                (*pParse).nMem += nPrefixReg;
            }
            (*pDest).iSdst = (*pParse).nMem + 1 as ::core::ffi::c_int;
            (*pParse).nMem += nResultCol;
        } else if (*pDest).iSdst + nResultCol > (*pParse).nMem {
            (*pParse).nMem += nResultCol;
        }
        (*pDest).nSdst = nResultCol;
        regResult = (*pDest).iSdst;
        regOrig = regResult;
        if srcTab >= 0 as ::core::ffi::c_int {
            i = 0 as ::core::ffi::c_int;
            while i < nResultCol {
                sqlite3VdbeAddOp3(v, OP_Column, srcTab, i, regResult + i);
                i += 1;
            }
        } else if eDest != SRT_Exists {
            let mut ecelFlags: u8_0 = 0;
            let mut pEList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
            if eDest == SRT_Mem || eDest == SRT_Output || eDest == SRT_Coroutine {
                ecelFlags = SQLITE_ECEL_DUP as u8_0;
            } else {
                ecelFlags = 0 as u8_0;
            }
            if !pSort.is_null() && hasDistinct == 0 as ::core::ffi::c_int
                && eDest != SRT_EphemTab && eDest != SRT_Table
            {
                ecelFlags = (ecelFlags as ::core::ffi::c_int
                    | (SQLITE_ECEL_OMITREF | SQLITE_ECEL_REF)) as u8_0;
                i = (*pSort).nOBSat;
                while i < (*(*pSort).pOrderBy).nExpr {
                    let mut j: ::core::ffi::c_int = 0;
                    j = (*(&raw mut (*(*pSort).pOrderBy).a as *mut ExprList_item)
                        .offset(i as isize))
                        .u
                        .x
                        .iOrderByCol as ::core::ffi::c_int;
                    if j > 0 as ::core::ffi::c_int {
                        (*(&raw mut (*(*p).pEList).a as *mut ExprList_item)
                            .offset((j - 1 as ::core::ffi::c_int) as isize))
                            .u
                            .x
                            .iOrderByCol = (i + 1 as ::core::ffi::c_int
                            - (*pSort).nOBSat) as u16_0;
                    }
                    i += 1;
                }
                pEList = (*p).pEList;
                i = 0 as ::core::ffi::c_int;
                while i < (*pEList).nExpr {
                    if (*(&raw mut (*pEList).a as *mut ExprList_item).offset(i as isize))
                        .u
                        .x
                        .iOrderByCol as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                    {
                        nResultCol -= 1;
                        regOrig = 0 as ::core::ffi::c_int;
                    }
                    i += 1;
                }
            }
            sRowLoadInfo.regResult = regResult;
            sRowLoadInfo.ecelFlags = ecelFlags;
            if (*p).iLimit != 0
                && ecelFlags as ::core::ffi::c_int & SQLITE_ECEL_OMITREF
                    != 0 as ::core::ffi::c_int && nPrefixReg > 0 as ::core::ffi::c_int
            {
                (*pSort).pDeferredRowLoad = &raw mut sRowLoadInfo as *mut RowLoadInfo;
                regOrig = 0 as ::core::ffi::c_int;
            } else {
                innerLoopLoadRow(pParse, p, &raw mut sRowLoadInfo);
            }
        }
        if hasDistinct != 0 {
            let mut eType: ::core::ffi::c_int = (*pDistinct).eTnctType
                as ::core::ffi::c_int;
            let mut iTab: ::core::ffi::c_int = (*pDistinct).tabTnct;
            iTab = codeDistinct(pParse, eType, iTab, iContinue, (*p).pEList, regResult);
            fixDistinctOpenEph(pParse, eType, iTab, (*pDistinct).addrTnct);
            if pSort.is_null() {
                codeOffset(v, (*p).iOffset, iContinue);
            }
        }
        match eDest {
            SRT_Union => {
                let mut r1: ::core::ffi::c_int = 0;
                r1 = sqlite3GetTempReg(pParse);
                sqlite3VdbeAddOp3(v, OP_MakeRecord, regResult, nResultCol, r1);
                sqlite3VdbeAddOp4Int(v, OP_IdxInsert, iParm, r1, regResult, nResultCol);
                sqlite3ReleaseTempReg(pParse, r1);
            }
            SRT_Except => {
                sqlite3VdbeAddOp3(v, OP_IdxDelete, iParm, regResult, nResultCol);
            }
            SRT_Fifo | SRT_DistFifo | SRT_Table | SRT_EphemTab => {
                let mut r1_0: ::core::ffi::c_int = sqlite3GetTempRange(
                    pParse,
                    nPrefixReg + 1 as ::core::ffi::c_int,
                );
                sqlite3VdbeAddOp3(
                    v,
                    OP_MakeRecord,
                    regResult,
                    nResultCol,
                    r1_0 + nPrefixReg,
                );
                if eDest == SRT_DistFifo {
                    let mut addr: ::core::ffi::c_int = sqlite3VdbeCurrentAddr(v)
                        + 4 as ::core::ffi::c_int;
                    sqlite3VdbeAddOp4Int(
                        v,
                        OP_Found,
                        iParm + 1 as ::core::ffi::c_int,
                        addr,
                        r1_0,
                        0 as ::core::ffi::c_int,
                    );
                    sqlite3VdbeAddOp4Int(
                        v,
                        OP_IdxInsert,
                        iParm + 1 as ::core::ffi::c_int,
                        r1_0,
                        regResult,
                        nResultCol,
                    );
                }
                if !pSort.is_null() {
                    pushOntoSorter(
                        pParse,
                        pSort,
                        p,
                        r1_0 + nPrefixReg,
                        regOrig,
                        1 as ::core::ffi::c_int,
                        nPrefixReg,
                    );
                } else {
                    let mut r2: ::core::ffi::c_int = sqlite3GetTempReg(pParse);
                    sqlite3VdbeAddOp2(v, OP_NewRowid, iParm, r2);
                    sqlite3VdbeAddOp3(v, OP_Insert, iParm, r1_0, r2);
                    sqlite3VdbeChangeP5(v, OPFLAG_APPEND as u16_0);
                    sqlite3ReleaseTempReg(pParse, r2);
                }
                sqlite3ReleaseTempRange(
                    pParse,
                    r1_0,
                    nPrefixReg + 1 as ::core::ffi::c_int,
                );
            }
            SRT_Upfrom => {
                if !pSort.is_null() {
                    pushOntoSorter(
                        pParse,
                        pSort,
                        p,
                        regResult,
                        regOrig,
                        nResultCol,
                        nPrefixReg,
                    );
                } else {
                    let mut i2: ::core::ffi::c_int = (*pDest).iSDParm2;
                    let mut r1_1: ::core::ffi::c_int = sqlite3GetTempReg(pParse);
                    sqlite3VdbeAddOp2(v, OP_IsNull, regResult, iBreak);
                    sqlite3VdbeAddOp3(
                        v,
                        OP_MakeRecord,
                        regResult + (i2 < 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
                        nResultCol
                            - (i2 < 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
                        r1_1,
                    );
                    if i2 < 0 as ::core::ffi::c_int {
                        sqlite3VdbeAddOp3(v, OP_Insert, iParm, r1_1, regResult);
                    } else {
                        sqlite3VdbeAddOp4Int(
                            v,
                            OP_IdxInsert,
                            iParm,
                            r1_1,
                            regResult,
                            i2,
                        );
                    }
                }
            }
            SRT_Set => {
                if !pSort.is_null() {
                    pushOntoSorter(
                        pParse,
                        pSort,
                        p,
                        regResult,
                        regOrig,
                        nResultCol,
                        nPrefixReg,
                    );
                    (*pDest).iSDParm2 = 0 as ::core::ffi::c_int;
                } else {
                    let mut r1_2: ::core::ffi::c_int = sqlite3GetTempReg(pParse);
                    sqlite3VdbeAddOp4(
                        v,
                        OP_MakeRecord,
                        regResult,
                        nResultCol,
                        r1_2,
                        (*pDest).zAffSdst,
                        nResultCol,
                    );
                    sqlite3VdbeAddOp4Int(
                        v,
                        OP_IdxInsert,
                        iParm,
                        r1_2,
                        regResult,
                        nResultCol,
                    );
                    if (*pDest).iSDParm2 != 0 {
                        sqlite3VdbeAddOp4Int(
                            v,
                            OP_FilterAdd,
                            (*pDest).iSDParm2,
                            0 as ::core::ffi::c_int,
                            regResult,
                            nResultCol,
                        );
                        sqlite3VdbeExplain(
                            pParse,
                            0 as u8_0,
                            b"CREATE BLOOM FILTER\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    }
                    sqlite3ReleaseTempReg(pParse, r1_2);
                }
            }
            SRT_Exists => {
                sqlite3VdbeAddOp2(v, OP_Integer, 1 as ::core::ffi::c_int, iParm);
            }
            SRT_Mem => {
                if !pSort.is_null() {
                    pushOntoSorter(
                        pParse,
                        pSort,
                        p,
                        regResult,
                        regOrig,
                        nResultCol,
                        nPrefixReg,
                    );
                    (*pDest).iSDParm = regResult;
                } else if regResult != iParm {
                    sqlite3VdbeAddOp3(
                        v,
                        OP_Copy,
                        regResult,
                        iParm,
                        nResultCol - 1 as ::core::ffi::c_int,
                    );
                }
            }
            SRT_Coroutine | SRT_Output => {
                if !pSort.is_null() {
                    pushOntoSorter(
                        pParse,
                        pSort,
                        p,
                        regResult,
                        regOrig,
                        nResultCol,
                        nPrefixReg,
                    );
                } else if eDest == SRT_Coroutine {
                    sqlite3VdbeAddOp1(v, OP_Yield, (*pDest).iSDParm);
                } else {
                    sqlite3VdbeAddOp2(v, OP_ResultRow, regResult, nResultCol);
                }
            }
            SRT_DistQueue | SRT_Queue => {
                let mut nKey: ::core::ffi::c_int = 0;
                let mut r1_3: ::core::ffi::c_int = 0;
                let mut r2_0: ::core::ffi::c_int = 0;
                let mut r3: ::core::ffi::c_int = 0;
                let mut addrTest: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut pSO: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
                pSO = (*pDest).pOrderBy;
                nKey = (*pSO).nExpr;
                r1_3 = sqlite3GetTempReg(pParse);
                r2_0 = sqlite3GetTempRange(pParse, nKey + 2 as ::core::ffi::c_int);
                r3 = r2_0 + nKey + 1 as ::core::ffi::c_int;
                if eDest == SRT_DistQueue {
                    addrTest = sqlite3VdbeAddOp4Int(
                        v,
                        OP_Found,
                        iParm + 1 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        regResult,
                        nResultCol,
                    );
                }
                sqlite3VdbeAddOp3(v, OP_MakeRecord, regResult, nResultCol, r3);
                if eDest == SRT_DistQueue {
                    sqlite3VdbeAddOp2(
                        v,
                        OP_IdxInsert,
                        iParm + 1 as ::core::ffi::c_int,
                        r3,
                    );
                    sqlite3VdbeChangeP5(v, OPFLAG_USESEEKRESULT as u16_0);
                }
                i = 0 as ::core::ffi::c_int;
                while i < nKey {
                    sqlite3VdbeAddOp2(
                        v,
                        OP_SCopy,
                        regResult
                            + (*(&raw mut (*pSO).a as *mut ExprList_item)
                                .offset(i as isize))
                                .u
                                .x
                                .iOrderByCol as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int,
                        r2_0 + i,
                    );
                    i += 1;
                }
                sqlite3VdbeAddOp2(v, OP_Sequence, iParm, r2_0 + nKey);
                sqlite3VdbeAddOp3(
                    v,
                    OP_MakeRecord,
                    r2_0,
                    nKey + 2 as ::core::ffi::c_int,
                    r1_3,
                );
                sqlite3VdbeAddOp4Int(
                    v,
                    OP_IdxInsert,
                    iParm,
                    r1_3,
                    r2_0,
                    nKey + 2 as ::core::ffi::c_int,
                );
                if addrTest != 0 {
                    sqlite3VdbeJumpHere(v, addrTest);
                }
                sqlite3ReleaseTempReg(pParse, r1_3);
                sqlite3ReleaseTempRange(pParse, r2_0, nKey + 2 as ::core::ffi::c_int);
            }
            _ => {}
        }
        if pSort.is_null() && (*p).iLimit != 0 {
            sqlite3VdbeAddOp2(v, OP_DecrJumpZero, (*p).iLimit, iBreak);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3KeyInfoAlloc(
    mut db: *mut sqlite3,
    mut N: ::core::ffi::c_int,
    mut X: ::core::ffi::c_int,
) -> *mut KeyInfo {
    unsafe {
        let mut nExtra: ::core::ffi::c_int = ((N + X) as usize)
            .wrapping_mul(
                (::core::mem::size_of::<*mut CollSeq>() as usize)
                    .wrapping_add(1 as usize),
            ) as ::core::ffi::c_int;
        let mut p: *mut KeyInfo = ::core::ptr::null_mut::<KeyInfo>();
        if N + X > 0xffff as ::core::ffi::c_int {
            return sqlite3OomFault(db) as *mut KeyInfo;
        }
        p = sqlite3DbMallocRawNN(
            db,
            (32 as usize)
                .wrapping_add(
                    (0 as usize)
                        .wrapping_mul(::core::mem::size_of::<*mut CollSeq>() as usize),
                )
                .wrapping_add(nExtra as usize) as u64_0,
        ) as *mut KeyInfo;
        if !p.is_null() {
            (*p).aSortFlags = (&raw mut (*p).aColl as *mut *mut CollSeq)
                .offset((N + X) as isize) as *mut *mut CollSeq as *mut u8_0;
            (*p).nKeyField = N as u16_0;
            (*p).nAllField = (N + X) as u16_0;
            (*p).enc = (*db).enc;
            (*p).db = db;
            (*p).nRef = 1 as u32_0;
            memset(
                &raw mut (*p).aColl as *mut *mut CollSeq as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                nExtra as size_t,
            );
        } else {
            return sqlite3OomFault(db) as *mut KeyInfo
        }
        return p;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3KeyInfoUnref(mut p: *mut KeyInfo) {
    unsafe {
        if !p.is_null() {
            (*p).nRef = (*p).nRef.wrapping_sub(1);
            if (*p).nRef == 0 as u32_0 {
                sqlite3DbNNFreeNN((*p).db, p as *mut ::core::ffi::c_void);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3KeyInfoRef(mut p: *mut KeyInfo) -> *mut KeyInfo {
    unsafe {
        if !p.is_null() {
            (*p).nRef = (*p).nRef.wrapping_add(1);
        }
        return p;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3KeyInfoFromExprList(
    mut pParse: *mut Parse,
    mut pList: *mut ExprList,
    mut iStart: ::core::ffi::c_int,
    mut nExtra: ::core::ffi::c_int,
) -> *mut KeyInfo {
    unsafe {
        let mut nExpr: ::core::ffi::c_int = 0;
        let mut pInfo: *mut KeyInfo = ::core::ptr::null_mut::<KeyInfo>();
        let mut pItem: *mut ExprList_item = ::core::ptr::null_mut::<ExprList_item>();
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut i: ::core::ffi::c_int = 0;
        nExpr = (*pList).nExpr;
        pInfo = sqlite3KeyInfoAlloc(
            db,
            nExpr - iStart,
            nExtra + 1 as ::core::ffi::c_int,
        );
        if !pInfo.is_null() {
            i = iStart;
            pItem = (&raw mut (*pList).a as *mut ExprList_item).offset(iStart as isize)
                as *mut ExprList_item;
            while i < nExpr {
                let ref mut c2rust_fresh15 = *(&raw mut (*pInfo).aColl
                    as *mut *mut CollSeq)
                    .offset((i - iStart) as isize);
                *c2rust_fresh15 = sqlite3ExprNNCollSeq(pParse, (*pItem).pExpr);
                *(*pInfo).aSortFlags.offset((i - iStart) as isize) = (*pItem)
                    .fg
                    .sortFlags;
                i += 1;
                pItem = pItem.offset(1);
            }
        }
        return pInfo;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SelectOpName(
    mut id: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        match id {
            TK_ALL => {
                z = b"UNION ALL\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            }
            TK_INTERSECT => {
                z = b"INTERSECT\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            }
            TK_EXCEPT => {
                z = b"EXCEPT\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            }
            _ => {
                z = b"UNION\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            }
        }
        return z;
    }
}
unsafe extern "C" fn explainTempTable(
    mut pParse: *mut Parse,
    mut zUsage: *const ::core::ffi::c_char,
) {
    unsafe {
        sqlite3VdbeExplain(
            pParse,
            0 as u8_0,
            b"USE TEMP B-TREE FOR %s\0".as_ptr() as *const ::core::ffi::c_char,
            zUsage,
        );
    }
}
unsafe extern "C" fn generateSortTail(
    mut pParse: *mut Parse,
    mut p: *mut Select,
    mut pSort: *mut SortCtx,
    mut nColumn: ::core::ffi::c_int,
    mut pDest: *mut SelectDest,
) {
    unsafe {
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        let mut addrBreak: ::core::ffi::c_int = (*pSort).labelDone;
        let mut addrContinue: ::core::ffi::c_int = sqlite3VdbeMakeLabel(pParse);
        let mut addr: ::core::ffi::c_int = 0;
        let mut addrOnce: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iTab: ::core::ffi::c_int = 0;
        let mut pOrderBy: *mut ExprList = (*pSort).pOrderBy;
        let mut eDest: ::core::ffi::c_int = (*pDest).eDest as ::core::ffi::c_int;
        let mut iParm: ::core::ffi::c_int = (*pDest).iSDParm;
        let mut regRow: ::core::ffi::c_int = 0;
        let mut regRowid: ::core::ffi::c_int = 0;
        let mut iCol: ::core::ffi::c_int = 0;
        let mut nKey: ::core::ffi::c_int = 0;
        let mut iSortTab: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut bSeq: ::core::ffi::c_int = 0;
        let mut nRefKey: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut aOutEx: *mut ExprList_item = &raw mut (*(*p).pEList).a
            as *mut ExprList_item;
        nKey = (*pOrderBy).nExpr - (*pSort).nOBSat;
        if (*pSort).nOBSat == 0 as ::core::ffi::c_int || nKey == 1 as ::core::ffi::c_int
        {
            sqlite3VdbeExplain(
                pParse,
                0 as u8_0,
                b"USE TEMP B-TREE FOR %sORDER BY\0".as_ptr()
                    as *const ::core::ffi::c_char,
                if (*pSort).nOBSat != 0 {
                    b"LAST TERM OF \0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        } else {
            sqlite3VdbeExplain(
                pParse,
                0 as u8_0,
                b"USE TEMP B-TREE FOR LAST %d TERMS OF ORDER BY\0".as_ptr()
                    as *const ::core::ffi::c_char,
                nKey,
            );
        }
        if (*pSort).labelBkOut != 0 {
            sqlite3VdbeAddOp2(v, OP_Gosub, (*pSort).regReturn, (*pSort).labelBkOut);
            sqlite3VdbeGoto(v, addrBreak);
            sqlite3VdbeResolveLabel(v, (*pSort).labelBkOut);
        }
        iTab = (*pSort).iECursor;
        if eDest == SRT_Output || eDest == SRT_Coroutine || eDest == SRT_Mem {
            if eDest == SRT_Mem && (*p).iOffset != 0 {
                sqlite3VdbeAddOp2(v, OP_Null, 0 as ::core::ffi::c_int, (*pDest).iSdst);
            }
            regRowid = 0 as ::core::ffi::c_int;
            regRow = (*pDest).iSdst;
        } else {
            regRowid = sqlite3GetTempReg(pParse);
            if eDest == SRT_EphemTab || eDest == SRT_Table {
                regRow = sqlite3GetTempReg(pParse);
                nColumn = 0 as ::core::ffi::c_int;
            } else {
                regRow = sqlite3GetTempRange(pParse, nColumn);
            }
        }
        if (*pSort).sortFlags as ::core::ffi::c_int & SORTFLAG_UseSorter != 0 {
            (*pParse).nMem += 1;
            let mut regSortOut: ::core::ffi::c_int = (*pParse).nMem;
            let c2rust_fresh13 = (*pParse).nTab;
            (*pParse).nTab = (*pParse).nTab + 1;
            iSortTab = c2rust_fresh13;
            if (*pSort).labelBkOut != 0 {
                addrOnce = sqlite3VdbeAddOp0(v, OP_Once);
            }
            sqlite3VdbeAddOp3(
                v,
                OP_OpenPseudo,
                iSortTab,
                regSortOut,
                nKey + 1 as ::core::ffi::c_int + nColumn + nRefKey,
            );
            if addrOnce != 0 {
                sqlite3VdbeJumpHere(v, addrOnce);
            }
            addr = 1 as ::core::ffi::c_int
                + sqlite3VdbeAddOp2(v, OP_SorterSort, iTab, addrBreak);
            sqlite3VdbeAddOp3(v, OP_SorterData, iTab, regSortOut, iSortTab);
            bSeq = 0 as ::core::ffi::c_int;
        } else {
            addr = 1 as ::core::ffi::c_int
                + sqlite3VdbeAddOp2(v, OP_Sort, iTab, addrBreak);
            codeOffset(v, (*p).iOffset, addrContinue);
            iSortTab = iTab;
            bSeq = 1 as ::core::ffi::c_int;
            if (*p).iOffset > 0 as ::core::ffi::c_int {
                sqlite3VdbeAddOp2(v, OP_AddImm, (*p).iLimit, -1 as ::core::ffi::c_int);
            }
        }
        i = 0 as ::core::ffi::c_int;
        iCol = nKey + bSeq - 1 as ::core::ffi::c_int;
        while i < nColumn {
            if (*aOutEx.offset(i as isize)).u.x.iOrderByCol as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                iCol += 1;
            }
            i += 1;
        }
        i = nColumn - 1 as ::core::ffi::c_int;
        while i >= 0 as ::core::ffi::c_int {
            let mut iRead: ::core::ffi::c_int = 0;
            if (*aOutEx.offset(i as isize)).u.x.iOrderByCol != 0 {
                iRead = (*aOutEx.offset(i as isize)).u.x.iOrderByCol
                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
            } else {
                let c2rust_fresh14 = iCol;
                iCol = iCol - 1;
                iRead = c2rust_fresh14;
            }
            sqlite3VdbeAddOp3(v, OP_Column, iSortTab, iRead, regRow + i);
            i -= 1;
        }
        match eDest {
            SRT_Table | SRT_EphemTab => {
                sqlite3VdbeAddOp3(v, OP_Column, iSortTab, nKey + bSeq, regRow);
                sqlite3VdbeAddOp2(v, OP_NewRowid, iParm, regRowid);
                sqlite3VdbeAddOp3(v, OP_Insert, iParm, regRow, regRowid);
                sqlite3VdbeChangeP5(v, OPFLAG_APPEND as u16_0);
            }
            SRT_Set => {
                sqlite3VdbeAddOp4(
                    v,
                    OP_MakeRecord,
                    regRow,
                    nColumn,
                    regRowid,
                    (*pDest).zAffSdst,
                    nColumn,
                );
                sqlite3VdbeAddOp4Int(v, OP_IdxInsert, iParm, regRowid, regRow, nColumn);
            }
            SRT_Mem => {}
            SRT_Upfrom => {
                let mut i2: ::core::ffi::c_int = (*pDest).iSDParm2;
                let mut r1: ::core::ffi::c_int = sqlite3GetTempReg(pParse);
                sqlite3VdbeAddOp3(
                    v,
                    OP_MakeRecord,
                    regRow + (i2 < 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
                    nColumn - (i2 < 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
                    r1,
                );
                if i2 < 0 as ::core::ffi::c_int {
                    sqlite3VdbeAddOp3(v, OP_Insert, iParm, r1, regRow);
                } else {
                    sqlite3VdbeAddOp4Int(v, OP_IdxInsert, iParm, r1, regRow, i2);
                }
            }
            _ => {
                if eDest == SRT_Output {
                    sqlite3VdbeAddOp2(v, OP_ResultRow, (*pDest).iSdst, nColumn);
                } else {
                    sqlite3VdbeAddOp1(v, OP_Yield, (*pDest).iSDParm);
                }
            }
        }
        if regRowid != 0 {
            if eDest == SRT_Set {
                sqlite3ReleaseTempRange(pParse, regRow, nColumn);
            } else {
                sqlite3ReleaseTempReg(pParse, regRow);
            }
            sqlite3ReleaseTempReg(pParse, regRowid);
        }
        sqlite3VdbeResolveLabel(v, addrContinue);
        if (*pSort).sortFlags as ::core::ffi::c_int & SORTFLAG_UseSorter != 0 {
            sqlite3VdbeAddOp2(v, OP_SorterNext, iTab, addr);
        } else {
            sqlite3VdbeAddOp2(v, OP_Next, iTab, addr);
        }
        if (*pSort).regReturn != 0 {
            sqlite3VdbeAddOp1(v, OP_Return, (*pSort).regReturn);
        }
        sqlite3VdbeResolveLabel(v, addrBreak);
    }
}
unsafe extern "C" fn columnTypeImpl(
    mut pNC: *mut NameContext,
    mut pExpr: *mut Expr,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut zType: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut j: ::core::ffi::c_int = 0;
        match (*pExpr).op as ::core::ffi::c_int {
            TK_COLUMN => {
                let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
                let mut pS: *mut Select = ::core::ptr::null_mut::<Select>();
                let mut iCol: ::core::ffi::c_int = (*pExpr).iColumn
                    as ::core::ffi::c_int;
                while !pNC.is_null() && pTab.is_null() {
                    let mut pTabList: *mut SrcList = (*pNC).pSrcList;
                    j = 0 as ::core::ffi::c_int;
                    while j < (*pTabList).nSrc
                        && (*(&raw mut (*pTabList).a as *mut SrcItem).offset(j as isize))
                            .iCursor != (*pExpr).iTable
                    {
                        j += 1;
                    }
                    if j < (*pTabList).nSrc {
                        pTab = (*(&raw mut (*pTabList).a as *mut SrcItem)
                            .offset(j as isize))
                            .pSTab;
                        if (*(&raw mut (*pTabList).a as *mut SrcItem).offset(j as isize))
                            .fg
                            .isSubquery() != 0
                        {
                            pS = (*(*(&raw mut (*pTabList).a as *mut SrcItem)
                                .offset(j as isize))
                                .u4
                                .pSubq)
                                .pSelect;
                        } else {
                            pS = ::core::ptr::null_mut::<Select>();
                        }
                    } else {
                        pNC = (*pNC).pNext;
                    }
                }
                if !pTab.is_null() {
                    if !pS.is_null() {
                        if iCol < (*(*pS).pEList).nExpr
                            && (ViewCanHaveRowid == 0 || iCol >= 0 as ::core::ffi::c_int)
                        {
                            let mut sNC: NameContext = NameContext {
                                pParse: ::core::ptr::null_mut::<Parse>(),
                                pSrcList: ::core::ptr::null_mut::<SrcList>(),
                                uNC: C2Rust_Unnamed_23 {
                                    pEList: ::core::ptr::null_mut::<ExprList>()

                                },
                                pNext: ::core::ptr::null_mut::<NameContext>(),
                                nRef: 0,
                                nNcErr: 0,
                                ncFlags: 0,
                                nNestedSelect: 0,
                                pWinSelect: ::core::ptr::null_mut::<Select>()

                            };
                            let mut p: *mut Expr = (*(&raw mut (*(*pS).pEList).a
                                as *mut ExprList_item)
                                .offset(iCol as isize))
                                .pExpr;
                            sNC.pSrcList = (*pS).pSrc;
                            sNC.pNext = pNC;
                            sNC.pParse = (*pNC).pParse;
                            zType = columnTypeImpl(&raw mut sNC, p);
                        }
                    } else if iCol < 0 as ::core::ffi::c_int {
                        zType = b"INTEGER\0".as_ptr() as *const ::core::ffi::c_char;
                    } else {
                        zType = sqlite3ColumnType(
                            (*pTab).aCol.offset(iCol as isize) as *mut Column,
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        );
                    }
                }
            }
            TK_SELECT => {
                let mut sNC_0: NameContext = NameContext {
                    pParse: ::core::ptr::null_mut::<Parse>(),
                    pSrcList: ::core::ptr::null_mut::<SrcList>(),
                    uNC: C2Rust_Unnamed_23 {
                        pEList: ::core::ptr::null_mut::<ExprList>()

                    },
                    pNext: ::core::ptr::null_mut::<NameContext>(),
                    nRef: 0,
                    nNcErr: 0,
                    ncFlags: 0,
                    nNestedSelect: 0,
                    pWinSelect: ::core::ptr::null_mut::<Select>()

                };
                let mut pS_0: *mut Select = ::core::ptr::null_mut::<Select>();
                let mut p_0: *mut Expr = ::core::ptr::null_mut::<Expr>();
                pS_0 = (*pExpr).x.pSelect;
                p_0 = (*(&raw mut (*(*pS_0).pEList).a as *mut ExprList_item)
                    .offset(0 as ::core::ffi::c_int as isize))
                    .pExpr;
                sNC_0.pSrcList = (*pS_0).pSrc;
                sNC_0.pNext = pNC;
                sNC_0.pParse = (*pNC).pParse;
                zType = columnTypeImpl(&raw mut sNC_0, p_0);
            }
            _ => {}
        }
        return zType;
    }
}
unsafe extern "C" fn generateColumnTypes(
    mut pParse: *mut Parse,
    mut pTabList: *mut SrcList,
    mut pEList: *mut ExprList,
) {
    unsafe {
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        let mut i: ::core::ffi::c_int = 0;
        let mut sNC: NameContext = NameContext {
            pParse: ::core::ptr::null_mut::<Parse>(),
            pSrcList: ::core::ptr::null_mut::<SrcList>(),
            uNC: C2Rust_Unnamed_23 {
                pEList: ::core::ptr::null_mut::<ExprList>()

            },
            pNext: ::core::ptr::null_mut::<NameContext>(),
            nRef: 0,
            nNcErr: 0,
            ncFlags: 0,
            nNestedSelect: 0,
            pWinSelect: ::core::ptr::null_mut::<Select>()

        };
        sNC.pSrcList = pTabList;
        sNC.pParse = pParse;
        sNC.pNext = ::core::ptr::null_mut::<NameContext>();
        i = 0 as ::core::ffi::c_int;
        while i < (*pEList).nExpr {
            let mut p: *mut Expr = (*(&raw mut (*pEList).a as *mut ExprList_item)
                .offset(i as isize))
                .pExpr;
            let mut zType: *const ::core::ffi::c_char = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            zType = columnTypeImpl(&raw mut sNC, p);
            sqlite3VdbeSetColName(
                v,
                i,
                COLNAME_DECLTYPE,
                zType,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            i += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3GenerateColumnNames(
    mut pParse: *mut Parse,
    mut pSelect: *mut Select,
) {
    unsafe {
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        let mut i: ::core::ffi::c_int = 0;
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut pTabList: *mut SrcList = ::core::ptr::null_mut::<SrcList>();
        let mut pEList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut fullName: ::core::ffi::c_int = 0;
        let mut srcName: ::core::ffi::c_int = 0;
        if (*pParse).colNamesSet() != 0 {
            return;
        }
        while !(*pSelect).pPrior.is_null() {
            pSelect = (*pSelect).pPrior;
        }
        pTabList = (*pSelect).pSrc;
        pEList = (*pSelect).pEList;
        (*pParse).set_colNamesSet(1 as bft as bft);
        fullName = ((*db).flags as ::core::ffi::c_ulonglong
            & SQLITE_FullColNames as ::core::ffi::c_ulonglong
            != 0 as ::core::ffi::c_ulonglong) as ::core::ffi::c_int;
        srcName = ((*db).flags as ::core::ffi::c_ulonglong
            & SQLITE_ShortColNames as ::core::ffi::c_ulonglong
            != 0 as ::core::ffi::c_ulonglong || fullName != 0) as ::core::ffi::c_int;
        sqlite3VdbeSetNumCols(v, (*pEList).nExpr);
        i = 0 as ::core::ffi::c_int;
        while i < (*pEList).nExpr {
            let mut p: *mut Expr = (*(&raw mut (*pEList).a as *mut ExprList_item)
                .offset(i as isize))
                .pExpr;
            if !(*(&raw mut (*pEList).a as *mut ExprList_item).offset(i as isize))
                .zEName
                .is_null()
                && (*(&raw mut (*pEList).a as *mut ExprList_item).offset(i as isize))
                    .fg
                    .eEName() as ::core::ffi::c_int == ENAME_NAME
            {
                let mut zName: *mut ::core::ffi::c_char = (*(&raw mut (*pEList).a
                    as *mut ExprList_item)
                    .offset(i as isize))
                    .zEName;
                sqlite3VdbeSetColName(
                    v,
                    i,
                    COLNAME_NAME,
                    zName,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
            } else if srcName != 0 && (*p).op as ::core::ffi::c_int == TK_COLUMN {
                let mut zCol: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                let mut iCol: ::core::ffi::c_int = (*p).iColumn as ::core::ffi::c_int;
                pTab = (*p).y.pTab;
                if iCol < 0 as ::core::ffi::c_int {
                    iCol = (*pTab).iPKey as ::core::ffi::c_int;
                }
                if iCol < 0 as ::core::ffi::c_int {
                    zCol = b"rowid\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                } else {
                    zCol = (*(*pTab).aCol.offset(iCol as isize)).zCnName;
                }
                if fullName != 0 {
                    let mut zName_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                        ::core::ffi::c_char,
                    >();
                    zName_0 = sqlite3MPrintf(
                        db,
                        b"%s.%s\0".as_ptr() as *const ::core::ffi::c_char,
                        (*pTab).zName,
                        zCol,
                    );
                    sqlite3VdbeSetColName(
                        v,
                        i,
                        COLNAME_NAME,
                        zName_0,
                        Some(
                            sqlite3RowSetClear
                                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                        ),
                    );
                } else {
                    sqlite3VdbeSetColName(
                        v,
                        i,
                        COLNAME_NAME,
                        zCol,
                        ::core::mem::transmute::<
                            ::libc::intptr_t,
                            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                    );
                }
            } else {
                let mut z: *const ::core::ffi::c_char = (*(&raw mut (*pEList).a
                    as *mut ExprList_item)
                    .offset(i as isize))
                    .zEName;
                z = if z.is_null() {
                    sqlite3MPrintf(
                        db,
                        b"column%d\0".as_ptr() as *const ::core::ffi::c_char,
                        i + 1 as ::core::ffi::c_int,
                    )
                } else {
                    sqlite3DbStrDup(db, z)
                };
                sqlite3VdbeSetColName(
                    v,
                    i,
                    COLNAME_NAME,
                    z,
                    Some(
                        sqlite3RowSetClear
                            as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                    ),
                );
            }
            i += 1;
        }
        generateColumnTypes(pParse, pTabList, pEList);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ColumnsFromExprList(
    mut pParse: *mut Parse,
    mut pEList: *mut ExprList,
    mut pnCol: *mut i16_0,
    mut paCol: *mut *mut Column,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut cnt: u32_0 = 0;
        let mut aCol: *mut Column = ::core::ptr::null_mut::<Column>();
        let mut pCol: *mut Column = ::core::ptr::null_mut::<Column>();
        let mut nCol: ::core::ffi::c_int = 0;
        let mut zName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut nName: ::core::ffi::c_int = 0;
        let mut ht: Hash = Hash {
            htsize: 0,
            count: 0,
            first: ::core::ptr::null_mut::<HashElem>(),
            ht: ::core::ptr::null_mut::<_ht>()

        };
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        sqlite3HashInit(&raw mut ht);
        if !pEList.is_null() {
            nCol = (*pEList).nExpr;
            aCol = sqlite3DbMallocZero(
                db,
                (::core::mem::size_of::<Column>() as usize).wrapping_mul(nCol as usize)
                    as u64_0,
            ) as *mut Column;
            if nCol > 32767 as ::core::ffi::c_int {
                nCol = 32767 as ::core::ffi::c_int;
            }
        } else {
            nCol = 0 as ::core::ffi::c_int;
            aCol = ::core::ptr::null_mut::<Column>();
        }
        *pnCol = nCol as i16_0;
        *paCol = aCol;
        i = 0 as ::core::ffi::c_int;
        pCol = aCol;
        while i < nCol && (*pParse).nErr == 0 {
            let mut pX: *mut ExprList_item = (&raw mut (*pEList).a as *mut ExprList_item)
                .offset(i as isize) as *mut ExprList_item;
            let mut pCollide: *mut ExprList_item = ::core::ptr::null_mut::<
                ExprList_item,
            >();
            zName = (*pX).zEName;
            if !(!zName.is_null()
                && (*pX).fg.eEName() as ::core::ffi::c_int == ENAME_NAME)
            {
                let mut pColExpr: *mut Expr = sqlite3ExprSkipCollateAndLikely(
                    (*pX).pExpr,
                );
                while !pColExpr.is_null()
                    && (*pColExpr).op as ::core::ffi::c_int == TK_DOT
                {
                    pColExpr = (*pColExpr).pRight;
                }
                if (*pColExpr).op as ::core::ffi::c_int == TK_COLUMN
                    && (*pColExpr).flags
                        & (0x1000000 as ::core::ffi::c_int
                            | 0x2000000 as ::core::ffi::c_int) as u32_0 == 0 as u32_0
                    && !(*pColExpr).y.pTab.is_null()
                {
                    let mut iCol: ::core::ffi::c_int = (*pColExpr).iColumn
                        as ::core::ffi::c_int;
                    pTab = (*pColExpr).y.pTab;
                    if iCol < 0 as ::core::ffi::c_int {
                        iCol = (*pTab).iPKey as ::core::ffi::c_int;
                    }
                    zName = (if iCol >= 0 as ::core::ffi::c_int {
                        (*(*pTab).aCol.offset(iCol as isize)).zCnName
                            as *const ::core::ffi::c_char
                    } else {
                        b"rowid\0".as_ptr() as *const ::core::ffi::c_char
                    }) as *mut ::core::ffi::c_char;
                } else if (*pColExpr).op as ::core::ffi::c_int == TK_ID {
                    zName = (*pColExpr).u.zToken;
                }
            }
            if !zName.is_null() && sqlite3IsTrueOrFalse(zName) == 0 {
                zName = sqlite3DbStrDup(db, zName);
            } else {
                zName = sqlite3MPrintf(
                    db,
                    b"column%d\0".as_ptr() as *const ::core::ffi::c_char,
                    i + 1 as ::core::ffi::c_int,
                );
            }
            cnt = 0 as u32_0;
            while !zName.is_null()
                && {
                    pCollide = sqlite3HashFind(&raw mut ht, zName) as *mut ExprList_item;
                    !pCollide.is_null()
                }
            {
                if (*pCollide).fg.bUsingTerm() != 0 {
                    (*pCol).colFlags = ((*pCol).colFlags as ::core::ffi::c_int
                        | COLFLAG_NOEXPAND) as u16_0;
                }
                nName = sqlite3Strlen30(zName);
                if nName > 0 as ::core::ffi::c_int {
                    j = nName - 1 as ::core::ffi::c_int;
                    while j > 0 as ::core::ffi::c_int
                        && *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                            .offset(
                                *zName.offset(j as isize) as ::core::ffi::c_uchar as isize,
                            ) as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0
                    {
                        j -= 1;
                    }
                    if *zName.offset(j as isize) as ::core::ffi::c_int == ':' as i32 {
                        nName = j;
                    }
                }
                cnt = cnt.wrapping_add(1);
                zName = sqlite3MPrintf(
                    db,
                    b"%.*z:%u\0".as_ptr() as *const ::core::ffi::c_char,
                    nName,
                    zName,
                    cnt,
                );
                sqlite3ProgressCheck(pParse);
                if cnt > 3 as u32_0 {
                    sqlite3_randomness(
                        ::core::mem::size_of::<u32_0>() as ::core::ffi::c_int,
                        &raw mut cnt as *mut ::core::ffi::c_void,
                    );
                }
            }
            (*pCol).zCnName = zName;
            (*pCol).hName = sqlite3StrIHash(zName);
            if (*pX).fg.bNoExpand() != 0 {
                (*pCol).colFlags = ((*pCol).colFlags as ::core::ffi::c_int
                    | COLFLAG_NOEXPAND) as u16_0;
            }
            if !zName.is_null()
                && sqlite3HashInsert(&raw mut ht, zName, pX as *mut ::core::ffi::c_void)
                    == pX as *mut ::core::ffi::c_void
            {
                sqlite3OomFault(db);
            }
            i += 1;
            pCol = pCol.offset(1);
        }
        sqlite3HashClear(&raw mut ht);
        if (*pParse).nErr != 0 {
            j = 0 as ::core::ffi::c_int;
            while j < i {
                sqlite3DbFree(
                    db,
                    (*aCol.offset(j as isize)).zCnName as *mut ::core::ffi::c_void,
                );
                j += 1;
            }
            sqlite3DbFree(db, aCol as *mut ::core::ffi::c_void);
            *paCol = ::core::ptr::null_mut::<Column>();
            *pnCol = 0 as i16_0;
            return (*pParse).rc;
        }
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SubqueryColumnTypes(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut pSelect: *mut Select,
    mut aff: ::core::ffi::c_char,
) {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pCol: *mut Column = ::core::ptr::null_mut::<Column>();
        let mut pColl: *mut CollSeq = ::core::ptr::null_mut::<CollSeq>();
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut p: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut a: *mut ExprList_item = ::core::ptr::null_mut::<ExprList_item>();
        let mut sNC: NameContext = NameContext {
            pParse: ::core::ptr::null_mut::<Parse>(),
            pSrcList: ::core::ptr::null_mut::<SrcList>(),
            uNC: C2Rust_Unnamed_23 {
                pEList: ::core::ptr::null_mut::<ExprList>()

            },
            pNext: ::core::ptr::null_mut::<NameContext>(),
            nRef: 0,
            nNcErr: 0,
            ncFlags: 0,
            nNestedSelect: 0,
            pWinSelect: ::core::ptr::null_mut::<Select>()

        };
        if (*db).mallocFailed as ::core::ffi::c_int != 0
            || (*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME
        {
            return;
        }
        while !(*pSelect).pPrior.is_null() {
            pSelect = (*pSelect).pPrior;
        }
        a = &raw mut (*(*pSelect).pEList).a as *mut ExprList_item as *mut ExprList_item;
        memset(
            &raw mut sNC as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<NameContext>() as size_t,
        );
        sNC.pSrcList = (*pSelect).pSrc;
        i = 0 as ::core::ffi::c_int;
        pCol = (*pTab).aCol;
        while i < (*pTab).nCol as ::core::ffi::c_int {
            let mut zType: *const ::core::ffi::c_char = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            let mut n: i64_0 = 0;
            let mut m: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut pS2: *mut Select = pSelect;
            (*pTab).tabFlags = ((*pTab).tabFlags as ::core::ffi::c_uint
                | ((*pCol).colFlags as ::core::ffi::c_int & COLFLAG_NOINSERT)
                    as ::core::ffi::c_uint) as u32_0;
            p = (*a.offset(i as isize)).pExpr;
            (*pCol).affinity = sqlite3ExprAffinity(p);
            while (*pCol).affinity as ::core::ffi::c_int <= SQLITE_AFF_NONE
                && !(*pS2).pNext.is_null()
            {
                m
                    |= sqlite3ExprDataType(
                        (*(&raw mut (*(*pS2).pEList).a as *mut ExprList_item)
                            .offset(i as isize))
                            .pExpr,
                    );
                pS2 = (*pS2).pNext;
                (*pCol).affinity = sqlite3ExprAffinity(
                    (*(&raw mut (*(*pS2).pEList).a as *mut ExprList_item)
                        .offset(i as isize))
                        .pExpr,
                );
            }
            if (*pCol).affinity as ::core::ffi::c_int <= SQLITE_AFF_NONE {
                (*pCol).affinity = aff;
            }
            if (*pCol).affinity as ::core::ffi::c_int >= SQLITE_AFF_TEXT
                && (!(*pS2).pNext.is_null() || pS2 != pSelect)
            {
                pS2 = (*pS2).pNext;
                while !pS2.is_null() {
                    m
                        |= sqlite3ExprDataType(
                            (*(&raw mut (*(*pS2).pEList).a as *mut ExprList_item)
                                .offset(i as isize))
                                .pExpr,
                        );
                    pS2 = (*pS2).pNext;
                }
                if (*pCol).affinity as ::core::ffi::c_int == SQLITE_AFF_TEXT
                    && m & 0x1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                {
                    (*pCol).affinity = SQLITE_AFF_BLOB as ::core::ffi::c_char;
                } else if (*pCol).affinity as ::core::ffi::c_int >= SQLITE_AFF_NUMERIC
                    && m & 0x2 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                {
                    (*pCol).affinity = SQLITE_AFF_BLOB as ::core::ffi::c_char;
                }
                if (*pCol).affinity as ::core::ffi::c_int >= SQLITE_AFF_NUMERIC
                    && (*p).op as ::core::ffi::c_int == TK_CAST
                {
                    (*pCol).affinity = SQLITE_AFF_FLEXNUM as ::core::ffi::c_char;
                }
            }
            zType = columnTypeImpl(&raw mut sNC, p);
            if zType.is_null()
                || (*pCol).affinity as ::core::ffi::c_int
                    != sqlite3AffinityType(zType, ::core::ptr::null_mut::<Column>())
                        as ::core::ffi::c_int
            {
                if (*pCol).affinity as ::core::ffi::c_int == SQLITE_AFF_NUMERIC
                    || (*pCol).affinity as ::core::ffi::c_int == SQLITE_AFF_FLEXNUM
                {
                    zType = b"NUM\0".as_ptr() as *const ::core::ffi::c_char;
                } else {
                    zType = ::core::ptr::null::<::core::ffi::c_char>();
                    j = 1 as ::core::ffi::c_int;
                    while j < SQLITE_N_STDTYPE {
                        if *(&raw const sqlite3StdTypeAffinity
                            as *const ::core::ffi::c_char)
                            .offset(j as isize) as ::core::ffi::c_int
                            == (*pCol).affinity as ::core::ffi::c_int
                        {
                            zType = *(&raw mut sqlite3StdType
                                as *mut *const ::core::ffi::c_char)
                                .offset(j as isize);
                            break;
                        } else {
                            j += 1;
                        }
                    }
                }
            }
            if !zType.is_null() {
                let k: i64_0 = sqlite3Strlen30(zType) as i64_0;
                n = sqlite3Strlen30((*pCol).zCnName) as i64_0;
                (*pCol).zCnName = sqlite3DbReallocOrFree(
                    db,
                    (*pCol).zCnName as *mut ::core::ffi::c_void,
                    (n as ::core::ffi::c_longlong + k as ::core::ffi::c_longlong
                        + 2 as ::core::ffi::c_longlong) as u64_0,
                ) as *mut ::core::ffi::c_char;
                (*pCol).colFlags = ((*pCol).colFlags as ::core::ffi::c_int
                    & !(COLFLAG_HASTYPE | COLFLAG_HASCOLL)) as u16_0;
                if !(*pCol).zCnName.is_null() {
                    memcpy(
                        (*pCol)
                            .zCnName
                            .offset(
                                (n as ::core::ffi::c_longlong
                                    + 1 as ::core::ffi::c_longlong) as isize,
                            ) as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                        zType as *const ::core::ffi::c_void,
                        (k as ::core::ffi::c_longlong + 1 as ::core::ffi::c_longlong)
                            as size_t,
                    );
                    (*pCol).colFlags = ((*pCol).colFlags as ::core::ffi::c_int
                        | COLFLAG_HASTYPE) as u16_0;
                }
            }
            pColl = sqlite3ExprCollSeq(pParse, p);
            if !pColl.is_null() {
                sqlite3ColumnSetColl(db, pCol, (*pColl).zName);
            }
            i += 1;
            pCol = pCol.offset(1);
        }
        (*pTab).szTabRow = 1 as LogEst;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ResultSetOfSelect(
    mut pParse: *mut Parse,
    mut pSelect: *mut Select,
    mut aff: ::core::ffi::c_char,
) -> *mut Table {
    unsafe {
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut savedFlags: u64_0 = 0;
        savedFlags = (*db).flags;
        (*db).flags &= !(SQLITE_FullColNames as u64_0) as ::core::ffi::c_ulonglong;
        (*db).flags |= SQLITE_ShortColNames as ::core::ffi::c_ulonglong;
        sqlite3SelectPrep(pParse, pSelect, ::core::ptr::null_mut::<NameContext>());
        (*db).flags = savedFlags;
        if (*pParse).nErr != 0 {
            return ::core::ptr::null_mut::<Table>();
        }
        while !(*pSelect).pPrior.is_null() {
            pSelect = (*pSelect).pPrior;
        }
        pTab = sqlite3DbMallocZero(db, ::core::mem::size_of::<Table>() as u64_0)
            as *mut Table;
        if pTab.is_null() {
            return ::core::ptr::null_mut::<Table>();
        }
        (*pTab).nTabRef = 1 as u32_0;
        (*pTab).zName = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*pTab).nRowLogEst = 200 as LogEst;
        sqlite3ColumnsFromExprList(
            pParse,
            (*pSelect).pEList,
            &raw mut (*pTab).nCol,
            &raw mut (*pTab).aCol,
        );
        sqlite3SubqueryColumnTypes(pParse, pTab, pSelect, aff);
        (*pTab).iPKey = -1 as ::core::ffi::c_int as i16_0;
        if (*db).mallocFailed != 0 {
            sqlite3DeleteTable(db, pTab);
            return ::core::ptr::null_mut::<Table>();
        }
        return pTab;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3GetVdbe(mut pParse: *mut Parse) -> *mut Vdbe {
    unsafe {
        if !(*pParse).pVdbe.is_null() {
            return (*pParse).pVdbe;
        }
        if (*pParse).pToplevel.is_null()
            && (*(*pParse).db).dbOptFlags & 0x8 as u32_0 == 0 as u32_0
        {
            (*pParse).set_okConstFactor(1 as bft as bft);
        }
        return sqlite3VdbeCreate(pParse);
    }
}
unsafe extern "C" fn computeLimitRegisters(
    mut pParse: *mut Parse,
    mut p: *mut Select,
    mut iBreak: ::core::ffi::c_int,
) {
    unsafe {
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        let mut iLimit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iOffset: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        let mut pLimit: *mut Expr = (*p).pLimit;
        if (*p).iLimit != 0 {
            return;
        }
        if !pLimit.is_null() {
            (*pParse).nMem += 1;
            iLimit = (*pParse).nMem;
            (*p).iLimit = iLimit;
            v = sqlite3GetVdbe(pParse);
            if sqlite3ExprIsInteger((*pLimit).pLeft, &raw mut n, pParse) != 0 {
                sqlite3VdbeAddOp2(v, OP_Integer, n, iLimit);
                if n == 0 as ::core::ffi::c_int {
                    sqlite3VdbeGoto(v, iBreak);
                } else if n >= 0 as ::core::ffi::c_int
                    && (*p).nSelectRow as ::core::ffi::c_int
                        > sqlite3LogEst(n as u64_0) as ::core::ffi::c_int
                {
                    (*p).nSelectRow = sqlite3LogEst(n as u64_0);
                    (*p).selFlags = ((*p).selFlags as ::core::ffi::c_uint
                        | SF_FixedLimit as ::core::ffi::c_uint) as u32_0;
                }
            } else {
                sqlite3ExprCode(pParse, (*pLimit).pLeft, iLimit);
                sqlite3VdbeAddOp1(v, OP_MustBeInt, iLimit);
                sqlite3VdbeAddOp2(v, OP_IfNot, iLimit, iBreak);
            }
            if !(*pLimit).pRight.is_null() {
                (*pParse).nMem += 1;
                iOffset = (*pParse).nMem;
                (*p).iOffset = iOffset;
                (*pParse).nMem += 1;
                sqlite3ExprCode(pParse, (*pLimit).pRight, iOffset);
                sqlite3VdbeAddOp1(v, OP_MustBeInt, iOffset);
                sqlite3VdbeAddOp3(
                    v,
                    OP_OffsetLimit,
                    iLimit,
                    iOffset + 1 as ::core::ffi::c_int,
                    iOffset,
                );
            }
        }
    }
}
unsafe extern "C" fn multiSelectCollSeq(
    mut pParse: *mut Parse,
    mut p: *mut Select,
    mut iCol: ::core::ffi::c_int,
) -> *mut CollSeq {
    unsafe {
        let mut pRet: *mut CollSeq = ::core::ptr::null_mut::<CollSeq>();
        if !(*p).pPrior.is_null() {
            pRet = multiSelectCollSeq(pParse, (*p).pPrior, iCol);
        } else {
            pRet = ::core::ptr::null_mut::<CollSeq>();
        }
        if pRet.is_null() && iCol < (*(*p).pEList).nExpr {
            pRet = sqlite3ExprCollSeq(
                pParse,
                (*(&raw mut (*(*p).pEList).a as *mut ExprList_item)
                    .offset(iCol as isize))
                    .pExpr,
            );
        }
        return pRet;
    }
}
unsafe extern "C" fn multiSelectOrderByKeyInfo(
    mut pParse: *mut Parse,
    mut p: *mut Select,
    mut nExtra: ::core::ffi::c_int,
) -> *mut KeyInfo {
    unsafe {
        let mut pOrderBy: *mut ExprList = (*p).pOrderBy;
        let mut nOrderBy: ::core::ffi::c_int = if !pOrderBy.is_null() {
            (*pOrderBy).nExpr
        } else {
            0 as ::core::ffi::c_int
        };
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pRet: *mut KeyInfo = sqlite3KeyInfoAlloc(
            db,
            nOrderBy + nExtra,
            1 as ::core::ffi::c_int,
        );
        if !pRet.is_null() {
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i < nOrderBy {
                let mut pItem: *mut ExprList_item = (&raw mut (*pOrderBy).a
                    as *mut ExprList_item)
                    .offset(i as isize) as *mut ExprList_item;
                let mut pTerm: *mut Expr = (*pItem).pExpr;
                let mut pColl: *mut CollSeq = ::core::ptr::null_mut::<CollSeq>();
                if (*pTerm).flags & EP_Collate as u32_0 != 0 {
                    pColl = sqlite3ExprCollSeq(pParse, pTerm);
                } else {
                    pColl = multiSelectCollSeq(
                        pParse,
                        p,
                        (*pItem).u.x.iOrderByCol as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int,
                    );
                    if pColl.is_null() {
                        pColl = (*db).pDfltColl;
                    }
                    let ref mut c2rust_fresh31 = (*(&raw mut (*pOrderBy).a
                        as *mut ExprList_item)
                        .offset(i as isize))
                        .pExpr;
                    *c2rust_fresh31 = sqlite3ExprAddCollateString(
                        pParse,
                        pTerm,
                        (*pColl).zName,
                    );
                }
                let ref mut c2rust_fresh32 = *(&raw mut (*pRet).aColl
                    as *mut *mut CollSeq)
                    .offset(i as isize);
                *c2rust_fresh32 = pColl;
                *(*pRet).aSortFlags.offset(i as isize) = (*(&raw mut (*pOrderBy).a
                    as *mut ExprList_item)
                    .offset(i as isize))
                    .fg
                    .sortFlags;
                i += 1;
            }
        }
        return pRet;
    }
}
unsafe extern "C" fn generateWithRecursiveQuery(
    mut pParse: *mut Parse,
    mut p: *mut Select,
    mut pDest: *mut SelectDest,
) {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut pSrc: *mut SrcList = (*p).pSrc;
        let mut nCol: ::core::ffi::c_int = (*(*p).pEList).nExpr;
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        let mut pSetup: *mut Select = ::core::ptr::null_mut::<Select>();
        let mut pFirstRec: *mut Select = ::core::ptr::null_mut::<Select>();
        let mut addrTop: ::core::ffi::c_int = 0;
        let mut addrCont: ::core::ffi::c_int = 0;
        let mut addrBreak: ::core::ffi::c_int = 0;
        let mut iCurrent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut regCurrent: ::core::ffi::c_int = 0;
        let mut iQueue: ::core::ffi::c_int = 0;
        let mut iDistinct: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut eDest: ::core::ffi::c_int = SRT_Fifo;
        let mut destQueue: SelectDest = SelectDest {
            eDest: 0,
            iSDParm: 0,
            iSDParm2: 0,
            iSdst: 0,
            nSdst: 0,
            zAffSdst: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            pOrderBy: ::core::ptr::null_mut::<ExprList>()

        };
        let mut i: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        let mut pOrderBy: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        let mut pLimit: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut regLimit: ::core::ffi::c_int = 0;
        let mut regOffset: ::core::ffi::c_int = 0;
        if !(*p).pWin.is_null() {
            sqlite3ErrorMsg(
                pParse,
                b"cannot use window functions in recursive queries\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        if sqlite3AuthCheck(
            pParse,
            SQLITE_RECURSIVE,
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ) != 0
        {
            return;
        }
        addrBreak = sqlite3VdbeMakeLabel(pParse);
        (*p).nSelectRow = 320 as LogEst;
        computeLimitRegisters(pParse, p, addrBreak);
        pLimit = (*p).pLimit;
        regLimit = (*p).iLimit;
        regOffset = (*p).iOffset;
        (*p).pLimit = ::core::ptr::null_mut::<Expr>();
        (*p).iOffset = 0 as ::core::ffi::c_int;
        (*p).iLimit = (*p).iOffset;
        pOrderBy = (*p).pOrderBy;
        i = 0 as ::core::ffi::c_int;
        while i < (*pSrc).nSrc {
            if (*(&raw mut (*pSrc).a as *mut SrcItem).offset(i as isize))
                .fg
                .isRecursive() != 0
            {
                iCurrent = (*(&raw mut (*pSrc).a as *mut SrcItem).offset(i as isize))
                    .iCursor;
                break;
            } else {
                i += 1;
            }
        }
        let c2rust_fresh33 = (*pParse).nTab;
        (*pParse).nTab = (*pParse).nTab + 1;
        iQueue = c2rust_fresh33;
        if (*p).op as ::core::ffi::c_int == TK_UNION {
            eDest = if !pOrderBy.is_null() { SRT_DistQueue } else { SRT_DistFifo };
            let c2rust_fresh34 = (*pParse).nTab;
            (*pParse).nTab = (*pParse).nTab + 1;
            iDistinct = c2rust_fresh34;
        } else {
            eDest = if !pOrderBy.is_null() { SRT_Queue } else { SRT_Fifo };
        }
        sqlite3SelectDestInit(&raw mut destQueue, eDest, iQueue);
        (*pParse).nMem += 1;
        regCurrent = (*pParse).nMem;
        sqlite3VdbeAddOp3(v, OP_OpenPseudo, iCurrent, regCurrent, nCol);
        if !pOrderBy.is_null() {
            let mut pKeyInfo: *mut KeyInfo = multiSelectOrderByKeyInfo(
                pParse,
                p,
                1 as ::core::ffi::c_int,
            );
            sqlite3VdbeAddOp4(
                v,
                OP_OpenEphemeral,
                iQueue,
                (*pOrderBy).nExpr + 2 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                pKeyInfo as *mut ::core::ffi::c_char,
                P4_KEYINFO,
            );
            destQueue.pOrderBy = pOrderBy;
        } else {
            sqlite3VdbeAddOp2(v, OP_OpenEphemeral, iQueue, nCol);
        }
        if iDistinct != 0 {
            (*p).addrOpenEphm[0 as ::core::ffi::c_int as usize] = sqlite3VdbeAddOp2(
                v,
                OP_OpenEphemeral,
                iDistinct,
                0 as ::core::ffi::c_int,
            );
            (*p).selFlags = ((*p).selFlags as ::core::ffi::c_uint
                | SF_UsesEphemeral as ::core::ffi::c_uint) as u32_0;
        }
        (*p).pOrderBy = ::core::ptr::null_mut::<ExprList>();
        pFirstRec = p;
        loop {
            if pFirstRec.is_null() {
                c2rust_current_block = 790185930182612747;
                break;
            }
            if (*pFirstRec).selFlags & SF_Aggregate as u32_0 != 0 {
                sqlite3ErrorMsg(
                    pParse,
                    b"recursive aggregate queries not supported\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                c2rust_current_block = 14719535460218439035;
                break;
            } else {
                (*pFirstRec).op = TK_ALL as u8_0;
                if (*(*pFirstRec).pPrior).selFlags & SF_Recursive as u32_0 == 0 as u32_0
                {
                    c2rust_current_block = 790185930182612747;
                    break;
                }
                pFirstRec = (*pFirstRec).pPrior;
            }
        }
        match c2rust_current_block {
            790185930182612747 => {
                pSetup = (*pFirstRec).pPrior;
                (*pSetup).pNext = ::core::ptr::null_mut::<Select>();
                sqlite3VdbeExplain(
                    pParse,
                    1 as u8_0,
                    b"SETUP\0".as_ptr() as *const ::core::ffi::c_char,
                );
                rc = sqlite3Select(pParse, pSetup, &raw mut destQueue);
                (*pSetup).pNext = p;
                if !(rc != 0) {
                    addrTop = sqlite3VdbeAddOp2(v, OP_Rewind, iQueue, addrBreak);
                    sqlite3VdbeAddOp1(v, OP_NullRow, iCurrent);
                    if !pOrderBy.is_null() {
                        sqlite3VdbeAddOp3(
                            v,
                            OP_Column,
                            iQueue,
                            (*pOrderBy).nExpr + 1 as ::core::ffi::c_int,
                            regCurrent,
                        );
                    } else {
                        sqlite3VdbeAddOp2(v, OP_RowData, iQueue, regCurrent);
                    }
                    sqlite3VdbeAddOp1(v, OP_Delete, iQueue);
                    addrCont = sqlite3VdbeMakeLabel(pParse);
                    codeOffset(v, regOffset, addrCont);
                    selectInnerLoop(
                        pParse,
                        p,
                        iCurrent,
                        ::core::ptr::null_mut::<SortCtx>(),
                        ::core::ptr::null_mut::<DistinctCtx>(),
                        pDest,
                        addrCont,
                        addrBreak,
                    );
                    if regLimit != 0 {
                        sqlite3VdbeAddOp2(v, OP_DecrJumpZero, regLimit, addrBreak);
                    }
                    sqlite3VdbeResolveLabel(v, addrCont);
                    (*pFirstRec).pPrior = ::core::ptr::null_mut::<Select>();
                    sqlite3VdbeExplain(
                        pParse,
                        1 as u8_0,
                        b"RECURSIVE STEP\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    sqlite3Select(pParse, p, &raw mut destQueue);
                    (*pFirstRec).pPrior = pSetup;
                    sqlite3VdbeGoto(v, addrTop);
                    sqlite3VdbeResolveLabel(v, addrBreak);
                }
            }
            _ => {}
        }
        sqlite3ExprListDelete((*pParse).db, (*p).pOrderBy);
        (*p).pOrderBy = pOrderBy;
        (*p).pLimit = pLimit;
    }
}
unsafe extern "C" fn multiSelectValues(
    mut pParse: *mut Parse,
    mut p: *mut Select,
    mut pDest: *mut SelectDest,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nRow: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut rc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut bShowAll: ::core::ffi::c_int = (*p).pLimit.is_null()
            as ::core::ffi::c_int;
        loop {
            if !(*p).pWin.is_null() {
                return -1 as ::core::ffi::c_int;
            }
            if (*p).pPrior.is_null() {
                break;
            }
            p = (*p).pPrior;
            nRow += bShowAll;
        }
        sqlite3VdbeExplain(
            pParse,
            0 as u8_0,
            b"SCAN %d CONSTANT ROW%s\0".as_ptr() as *const ::core::ffi::c_char,
            nRow,
            if nRow == 1 as ::core::ffi::c_int {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"S\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        while !p.is_null() {
            selectInnerLoop(
                pParse,
                p,
                -1 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<SortCtx>(),
                ::core::ptr::null_mut::<DistinctCtx>(),
                pDest,
                1 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            if bShowAll == 0 {
                break;
            }
            (*p).nSelectRow = nRow as LogEst;
            p = (*p).pNext;
        }
        return rc;
    }
}
unsafe extern "C" fn hasAnchor(mut p: *mut Select) -> ::core::ffi::c_int {
    unsafe {
        while !p.is_null() && (*p).selFlags & SF_Recursive as u32_0 != 0 as u32_0 {
            p = (*p).pPrior;
        }
        return !p.is_null() as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn multiSelect(
    mut pParse: *mut Parse,
    mut p: *mut Select,
    mut pDest: *mut SelectDest,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pPrior: *mut Select = ::core::ptr::null_mut::<Select>();
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        let mut dest: SelectDest = SelectDest {
            eDest: 0,
            iSDParm: 0,
            iSDParm2: 0,
            iSdst: 0,
            nSdst: 0,
            zAffSdst: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            pOrderBy: ::core::ptr::null_mut::<ExprList>()

        };
        let mut pDelete: *mut Select = ::core::ptr::null_mut::<Select>();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        db = (*pParse).db;
        pPrior = (*p).pPrior;
        dest = *pDest;
        v = sqlite3GetVdbe(pParse);
        if dest.eDest as ::core::ffi::c_int == SRT_EphemTab {
            sqlite3VdbeAddOp2(v, OP_OpenEphemeral, dest.iSDParm, (*(*p).pEList).nExpr);
            dest.eDest = SRT_Table as u8_0;
        }
        if (*p).selFlags & SF_MultiValue as u32_0 != 0 {
            rc = multiSelectValues(pParse, p, &raw mut dest);
            if rc >= 0 as ::core::ffi::c_int {
                c2rust_current_block = 10511199618917093937;
            } else {
                rc = SQLITE_OK;
                c2rust_current_block = 7149356873433890176;
            }
        } else {
            c2rust_current_block = 7149356873433890176;
        }
        match c2rust_current_block {
            7149356873433890176 => {
                if (*p).selFlags & SF_Recursive as u32_0 != 0 as u32_0
                    && hasAnchor(p) != 0
                {
                    generateWithRecursiveQuery(pParse, p, &raw mut dest);
                    c2rust_current_block = 4235089732467486934;
                } else if !(*p).pOrderBy.is_null() {
                    return multiSelectOrderBy(pParse, p, pDest)
                } else {
                    if (*pPrior).pPrior.is_null() {
                        sqlite3VdbeExplain(
                            pParse,
                            1 as u8_0,
                            b"COMPOUND QUERY\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        sqlite3VdbeExplain(
                            pParse,
                            1 as u8_0,
                            b"LEFT-MOST SUBQUERY\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    }
                    match (*p).op as ::core::ffi::c_int {
                        TK_ALL => {
                            let mut addr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            let mut nLimit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            (*pPrior).iLimit = (*p).iLimit;
                            (*pPrior).iOffset = (*p).iOffset;
                            (*pPrior).pLimit = (*p).pLimit;
                            rc = sqlite3Select(pParse, pPrior, &raw mut dest);
                            (*pPrior).pLimit = ::core::ptr::null_mut::<Expr>();
                            if rc != 0 {
                                c2rust_current_block = 10511199618917093937;
                            } else {
                                (*p).pPrior = ::core::ptr::null_mut::<Select>();
                                (*p).iLimit = (*pPrior).iLimit;
                                (*p).iOffset = (*pPrior).iOffset;
                                if (*p).iLimit != 0 {
                                    addr = sqlite3VdbeAddOp1(v, OP_IfNot, (*p).iLimit);
                                    if (*p).iOffset != 0 {
                                        sqlite3VdbeAddOp3(
                                            v,
                                            OP_OffsetLimit,
                                            (*p).iLimit,
                                            (*p).iOffset + 1 as ::core::ffi::c_int,
                                            (*p).iOffset,
                                        );
                                    }
                                }
                                sqlite3VdbeExplain(
                                    pParse,
                                    1 as u8_0,
                                    b"UNION ALL\0".as_ptr() as *const ::core::ffi::c_char,
                                );
                                rc = sqlite3Select(pParse, p, &raw mut dest);
                                pDelete = (*p).pPrior;
                                (*p).pPrior = pPrior;
                                (*p).nSelectRow = sqlite3LogEstAdd(
                                    (*p).nSelectRow,
                                    (*pPrior).nSelectRow,
                                );
                                if !(*p).pLimit.is_null()
                                    && sqlite3ExprIsInteger(
                                        (*(*p).pLimit).pLeft,
                                        &raw mut nLimit,
                                        pParse,
                                    ) != 0 && nLimit > 0 as ::core::ffi::c_int
                                    && (*p).nSelectRow as ::core::ffi::c_int
                                        > sqlite3LogEst(nLimit as u64_0) as ::core::ffi::c_int
                                {
                                    (*p).nSelectRow = sqlite3LogEst(nLimit as u64_0);
                                }
                                if addr != 0 {
                                    sqlite3VdbeJumpHere(v, addr);
                                }
                                c2rust_current_block = 14579489411542934868;
                            }
                        }
                        TK_EXCEPT | TK_UNION => {
                            let mut unionTab: ::core::ffi::c_int = 0;
                            let mut op: u8_0 = 0 as u8_0;
                            let mut priorOp: ::core::ffi::c_int = 0;
                            let mut pLimit: *mut Expr = ::core::ptr::null_mut::<Expr>();
                            let mut addr_0: ::core::ffi::c_int = 0;
                            let mut emptyBypass: ::core::ffi::c_int = 0
                                as ::core::ffi::c_int;
                            let mut uniondest: SelectDest = SelectDest {
                                eDest: 0,
                                iSDParm: 0,
                                iSDParm2: 0,
                                iSdst: 0,
                                nSdst: 0,
                                zAffSdst: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                pOrderBy: ::core::ptr::null_mut::<ExprList>()

                            };
                            priorOp = SRT_Union;
                            if dest.eDest as ::core::ffi::c_int == priorOp {
                                unionTab = dest.iSDParm;
                            } else {
                                let c2rust_fresh24 = (*pParse).nTab;
                                (*pParse).nTab = (*pParse).nTab + 1;
                                unionTab = c2rust_fresh24;
                                addr_0 = sqlite3VdbeAddOp2(
                                    v,
                                    OP_OpenEphemeral,
                                    unionTab,
                                    0 as ::core::ffi::c_int,
                                );
                                (*p).addrOpenEphm[0 as ::core::ffi::c_int as usize] = addr_0;
                                let ref mut c2rust_fresh25 = (*findRightmost(p)).selFlags;
                                *c2rust_fresh25 = (*c2rust_fresh25 as ::core::ffi::c_uint
                                    | SF_UsesEphemeral as ::core::ffi::c_uint) as u32_0;
                            }
                            sqlite3SelectDestInit(&raw mut uniondest, priorOp, unionTab);
                            rc = sqlite3Select(pParse, pPrior, &raw mut uniondest);
                            if rc != 0 {
                                c2rust_current_block = 10511199618917093937;
                            } else {
                                if (*p).op as ::core::ffi::c_int == TK_EXCEPT {
                                    op = SRT_Except as u8_0;
                                    emptyBypass = sqlite3VdbeAddOp1(v, OP_IfEmpty, unionTab);
                                } else {
                                    op = SRT_Union as u8_0;
                                }
                                (*p).pPrior = ::core::ptr::null_mut::<Select>();
                                pLimit = (*p).pLimit;
                                (*p).pLimit = ::core::ptr::null_mut::<Expr>();
                                uniondest.eDest = op;
                                sqlite3VdbeExplain(
                                    pParse,
                                    1 as u8_0,
                                    b"%s USING TEMP B-TREE\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    sqlite3SelectOpName((*p).op as ::core::ffi::c_int),
                                );
                                rc = sqlite3Select(pParse, p, &raw mut uniondest);
                                pDelete = (*p).pPrior;
                                (*p).pPrior = pPrior;
                                (*p).pOrderBy = ::core::ptr::null_mut::<ExprList>();
                                if (*p).op as ::core::ffi::c_int == TK_UNION {
                                    (*p).nSelectRow = sqlite3LogEstAdd(
                                        (*p).nSelectRow,
                                        (*pPrior).nSelectRow,
                                    );
                                }
                                if emptyBypass != 0 {
                                    sqlite3VdbeJumpHere(v, emptyBypass);
                                }
                                sqlite3ExprDelete(db, (*p).pLimit);
                                (*p).pLimit = pLimit;
                                (*p).iLimit = 0 as ::core::ffi::c_int;
                                (*p).iOffset = 0 as ::core::ffi::c_int;
                                if dest.eDest as ::core::ffi::c_int != priorOp
                                    && (*db).mallocFailed as ::core::ffi::c_int
                                        == 0 as ::core::ffi::c_int
                                {
                                    let mut iCont: ::core::ffi::c_int = 0;
                                    let mut iBreak: ::core::ffi::c_int = 0;
                                    let mut iStart: ::core::ffi::c_int = 0;
                                    iBreak = sqlite3VdbeMakeLabel(pParse);
                                    iCont = sqlite3VdbeMakeLabel(pParse);
                                    computeLimitRegisters(pParse, p, iBreak);
                                    sqlite3VdbeAddOp2(v, OP_Rewind, unionTab, iBreak);
                                    iStart = sqlite3VdbeCurrentAddr(v);
                                    selectInnerLoop(
                                        pParse,
                                        p,
                                        unionTab,
                                        ::core::ptr::null_mut::<SortCtx>(),
                                        ::core::ptr::null_mut::<DistinctCtx>(),
                                        &raw mut dest,
                                        iCont,
                                        iBreak,
                                    );
                                    sqlite3VdbeResolveLabel(v, iCont);
                                    sqlite3VdbeAddOp2(v, OP_Next, unionTab, iStart);
                                    sqlite3VdbeResolveLabel(v, iBreak);
                                    sqlite3VdbeAddOp2(
                                        v,
                                        OP_Close,
                                        unionTab,
                                        0 as ::core::ffi::c_int,
                                    );
                                }
                                c2rust_current_block = 14579489411542934868;
                            }
                        }
                        _ => {
                            let mut tab1: ::core::ffi::c_int = 0;
                            let mut tab2: ::core::ffi::c_int = 0;
                            let mut iCont_0: ::core::ffi::c_int = 0;
                            let mut iBreak_0: ::core::ffi::c_int = 0;
                            let mut iStart_0: ::core::ffi::c_int = 0;
                            let mut pLimit_0: *mut Expr = ::core::ptr::null_mut::<
                                Expr,
                            >();
                            let mut addr_1: ::core::ffi::c_int = 0;
                            let mut iLimit: ::core::ffi::c_int = 0;
                            let mut iOffset: ::core::ffi::c_int = 0;
                            let mut intersectdest: SelectDest = SelectDest {
                                eDest: 0,
                                iSDParm: 0,
                                iSDParm2: 0,
                                iSdst: 0,
                                nSdst: 0,
                                zAffSdst: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                pOrderBy: ::core::ptr::null_mut::<ExprList>()

                            };
                            let mut r1: ::core::ffi::c_int = 0;
                            let mut emptyBypass_0: ::core::ffi::c_int = 0;
                            let c2rust_fresh26 = (*pParse).nTab;
                            (*pParse).nTab = (*pParse).nTab + 1;
                            tab1 = c2rust_fresh26;
                            let c2rust_fresh27 = (*pParse).nTab;
                            (*pParse).nTab = (*pParse).nTab + 1;
                            tab2 = c2rust_fresh27;
                            addr_1 = sqlite3VdbeAddOp2(
                                v,
                                OP_OpenEphemeral,
                                tab1,
                                0 as ::core::ffi::c_int,
                            );
                            (*p).addrOpenEphm[0 as ::core::ffi::c_int as usize] = addr_1;
                            let ref mut c2rust_fresh28 = (*findRightmost(p)).selFlags;
                            *c2rust_fresh28 = (*c2rust_fresh28 as ::core::ffi::c_uint
                                | SF_UsesEphemeral as ::core::ffi::c_uint) as u32_0;
                            sqlite3SelectDestInit(
                                &raw mut intersectdest,
                                SRT_Union,
                                tab1,
                            );
                            rc = sqlite3Select(pParse, pPrior, &raw mut intersectdest);
                            if rc != 0 {
                                c2rust_current_block = 10511199618917093937;
                            } else {
                                iBreak_0 = sqlite3VdbeMakeLabel(pParse);
                                computeLimitRegisters(pParse, p, iBreak_0);
                                emptyBypass_0 = sqlite3VdbeAddOp1(v, OP_IfEmpty, tab1);
                                addr_1 = sqlite3VdbeAddOp2(
                                    v,
                                    OP_OpenEphemeral,
                                    tab2,
                                    0 as ::core::ffi::c_int,
                                );
                                (*p).addrOpenEphm[1 as ::core::ffi::c_int as usize] = addr_1;
                                pLimit_0 = (*p).pLimit;
                                iLimit = (*p).iLimit;
                                iOffset = (*p).iOffset;
                                (*p).pPrior = ::core::ptr::null_mut::<Select>();
                                (*p).pLimit = ::core::ptr::null_mut::<Expr>();
                                (*p).iLimit = 0 as ::core::ffi::c_int;
                                (*p).iOffset = 0 as ::core::ffi::c_int;
                                intersectdest.iSDParm = tab2;
                                sqlite3VdbeExplain(
                                    pParse,
                                    1 as u8_0,
                                    b"%s USING TEMP B-TREE\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    sqlite3SelectOpName((*p).op as ::core::ffi::c_int),
                                );
                                rc = sqlite3Select(pParse, p, &raw mut intersectdest);
                                pDelete = (*p).pPrior;
                                (*p).pPrior = pPrior;
                                if (*p).nSelectRow as ::core::ffi::c_int
                                    > (*pPrior).nSelectRow as ::core::ffi::c_int
                                {
                                    (*p).nSelectRow = (*pPrior).nSelectRow;
                                }
                                sqlite3ExprDelete(db, (*p).pLimit);
                                (*p).pLimit = pLimit_0;
                                (*p).iLimit = iLimit;
                                (*p).iOffset = iOffset;
                                if rc != 0 {
                                    c2rust_current_block = 14579489411542934868;
                                } else {
                                    sqlite3VdbeAddOp1(v, OP_Rewind, tab1);
                                    r1 = sqlite3GetTempReg(pParse);
                                    iStart_0 = sqlite3VdbeAddOp2(v, OP_RowData, tab1, r1);
                                    iCont_0 = sqlite3VdbeMakeLabel(pParse);
                                    sqlite3VdbeAddOp4Int(
                                        v,
                                        OP_NotFound,
                                        tab2,
                                        iCont_0,
                                        r1,
                                        0 as ::core::ffi::c_int,
                                    );
                                    sqlite3ReleaseTempReg(pParse, r1);
                                    selectInnerLoop(
                                        pParse,
                                        p,
                                        tab1,
                                        ::core::ptr::null_mut::<SortCtx>(),
                                        ::core::ptr::null_mut::<DistinctCtx>(),
                                        &raw mut dest,
                                        iCont_0,
                                        iBreak_0,
                                    );
                                    sqlite3VdbeResolveLabel(v, iCont_0);
                                    sqlite3VdbeAddOp2(v, OP_Next, tab1, iStart_0);
                                    sqlite3VdbeResolveLabel(v, iBreak_0);
                                    sqlite3VdbeAddOp2(
                                        v,
                                        OP_Close,
                                        tab2,
                                        0 as ::core::ffi::c_int,
                                    );
                                    sqlite3VdbeJumpHere(v, emptyBypass_0);
                                    sqlite3VdbeAddOp2(
                                        v,
                                        OP_Close,
                                        tab1,
                                        0 as ::core::ffi::c_int,
                                    );
                                    c2rust_current_block = 14579489411542934868;
                                }
                            }
                        }
                    }
                    match c2rust_current_block {
                        10511199618917093937 => {}
                        _ => {
                            if (*p).pNext.is_null() {
                                sqlite3VdbeExplainPop(pParse);
                            }
                            c2rust_current_block = 4235089732467486934;
                        }
                    }
                }
                match c2rust_current_block {
                    10511199618917093937 => {}
                    _ => {
                        if !((*pParse).nErr != 0) {
                            if (*p).selFlags & SF_UsesEphemeral as u32_0 != 0 {
                                let mut i: ::core::ffi::c_int = 0;
                                let mut pKeyInfo: *mut KeyInfo = ::core::ptr::null_mut::<
                                    KeyInfo,
                                >();
                                let mut pLoop: *mut Select = ::core::ptr::null_mut::<
                                    Select,
                                >();
                                let mut apColl: *mut *mut CollSeq = ::core::ptr::null_mut::<
                                    *mut CollSeq,
                                >();
                                let mut nCol: ::core::ffi::c_int = 0;
                                nCol = (*(*p).pEList).nExpr;
                                pKeyInfo = sqlite3KeyInfoAlloc(
                                    db,
                                    nCol,
                                    1 as ::core::ffi::c_int,
                                );
                                if pKeyInfo.is_null() {
                                    rc = SQLITE_NOMEM_BKPT;
                                } else {
                                    i = 0 as ::core::ffi::c_int;
                                    apColl = &raw mut (*pKeyInfo).aColl as *mut *mut CollSeq;
                                    while i < nCol {
                                        *apColl = multiSelectCollSeq(pParse, p, i);
                                        if (*apColl).is_null() {
                                            *apColl = (*db).pDfltColl;
                                        }
                                        i += 1;
                                        apColl = apColl.offset(1);
                                    }
                                    pLoop = p;
                                    while !pLoop.is_null() {
                                        i = 0 as ::core::ffi::c_int;
                                        while i < 2 as ::core::ffi::c_int {
                                            let mut addr_2: ::core::ffi::c_int = (*pLoop)
                                                .addrOpenEphm[i as usize];
                                            if addr_2 < 0 as ::core::ffi::c_int {
                                                break;
                                            }
                                            sqlite3VdbeChangeP2(v, addr_2, nCol);
                                            sqlite3VdbeChangeP4(
                                                v,
                                                addr_2,
                                                sqlite3KeyInfoRef(pKeyInfo) as *mut ::core::ffi::c_char,
                                                P4_KEYINFO,
                                            );
                                            (*pLoop).addrOpenEphm[i as usize] = -1
                                                as ::core::ffi::c_int;
                                            i += 1;
                                        }
                                        pLoop = (*pLoop).pPrior;
                                    }
                                    sqlite3KeyInfoUnref(pKeyInfo);
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        (*pDest).iSdst = dest.iSdst;
        (*pDest).nSdst = dest.nSdst;
        (*pDest).iSDParm2 = dest.iSDParm2;
        if !pDelete.is_null() {
            sqlite3ParserAddCleanup(
                pParse,
                Some(
                    sqlite3SelectDeleteGeneric
                        as unsafe extern "C" fn(
                            *mut sqlite3,
                            *mut ::core::ffi::c_void,
                        ) -> (),
                ),
                pDelete as *mut ::core::ffi::c_void,
            );
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SelectWrongNumTermsError(
    mut pParse: *mut Parse,
    mut p: *mut Select,
) {
    unsafe {
        if (*p).selFlags & SF_Values as u32_0 != 0 {
            sqlite3ErrorMsg(
                pParse,
                b"all VALUES must have the same number of terms\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else {
            sqlite3ErrorMsg(
                pParse,
                b"SELECTs to the left and right of %s do not have the same number of result columns\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                sqlite3SelectOpName((*p).op as ::core::ffi::c_int),
            );
        };
    }
}
unsafe extern "C" fn generateOutputSubroutine(
    mut pParse: *mut Parse,
    mut p: *mut Select,
    mut pIn: *mut SelectDest,
    mut pDest: *mut SelectDest,
    mut regReturn: ::core::ffi::c_int,
    mut regPrev: ::core::ffi::c_int,
    mut pKeyInfo: *mut KeyInfo,
    mut iBreak: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        let mut iContinue: ::core::ffi::c_int = 0;
        let mut addr: ::core::ffi::c_int = 0;
        addr = sqlite3VdbeCurrentAddr(v);
        iContinue = sqlite3VdbeMakeLabel(pParse);
        if regPrev != 0 {
            let mut addr1: ::core::ffi::c_int = 0;
            let mut addr2: ::core::ffi::c_int = 0;
            addr1 = sqlite3VdbeAddOp1(v, OP_IfNot, regPrev);
            addr2 = sqlite3VdbeAddOp4(
                v,
                OP_Compare,
                (*pIn).iSdst,
                regPrev + 1 as ::core::ffi::c_int,
                (*pIn).nSdst,
                sqlite3KeyInfoRef(pKeyInfo) as *mut ::core::ffi::c_char,
                P4_KEYINFO,
            );
            sqlite3VdbeAddOp3(
                v,
                OP_Jump,
                addr2 + 2 as ::core::ffi::c_int,
                iContinue,
                addr2 + 2 as ::core::ffi::c_int,
            );
            sqlite3VdbeJumpHere(v, addr1);
            sqlite3VdbeAddOp3(
                v,
                OP_Copy,
                (*pIn).iSdst,
                regPrev + 1 as ::core::ffi::c_int,
                (*pIn).nSdst - 1 as ::core::ffi::c_int,
            );
            sqlite3VdbeAddOp2(v, OP_Integer, 1 as ::core::ffi::c_int, regPrev);
        }
        if (*(*pParse).db).mallocFailed != 0 {
            return 0 as ::core::ffi::c_int;
        }
        codeOffset(v, (*p).iOffset, iContinue);
        match (*pDest).eDest as ::core::ffi::c_int {
            SRT_EphemTab => {
                let mut r1: ::core::ffi::c_int = sqlite3GetTempReg(pParse);
                let mut r2: ::core::ffi::c_int = sqlite3GetTempReg(pParse);
                sqlite3VdbeAddOp3(v, OP_MakeRecord, (*pIn).iSdst, (*pIn).nSdst, r1);
                sqlite3VdbeAddOp2(v, OP_NewRowid, (*pDest).iSDParm, r2);
                sqlite3VdbeAddOp3(v, OP_Insert, (*pDest).iSDParm, r1, r2);
                sqlite3VdbeChangeP5(v, OPFLAG_APPEND as u16_0);
                sqlite3ReleaseTempReg(pParse, r2);
                sqlite3ReleaseTempReg(pParse, r1);
            }
            SRT_Set => {
                let mut r1_0: ::core::ffi::c_int = 0;
                r1_0 = sqlite3GetTempReg(pParse);
                sqlite3VdbeAddOp4(
                    v,
                    OP_MakeRecord,
                    (*pIn).iSdst,
                    (*pIn).nSdst,
                    r1_0,
                    (*pDest).zAffSdst,
                    (*pIn).nSdst,
                );
                sqlite3VdbeAddOp4Int(
                    v,
                    OP_IdxInsert,
                    (*pDest).iSDParm,
                    r1_0,
                    (*pIn).iSdst,
                    (*pIn).nSdst,
                );
                if (*pDest).iSDParm2 > 0 as ::core::ffi::c_int {
                    sqlite3VdbeAddOp4Int(
                        v,
                        OP_FilterAdd,
                        (*pDest).iSDParm2,
                        0 as ::core::ffi::c_int,
                        (*pIn).iSdst,
                        (*pIn).nSdst,
                    );
                    sqlite3VdbeExplain(
                        pParse,
                        0 as u8_0,
                        b"CREATE BLOOM FILTER\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                }
                sqlite3ReleaseTempReg(pParse, r1_0);
            }
            SRT_Mem => {
                sqlite3ExprCodeMove(
                    pParse,
                    (*pIn).iSdst,
                    (*pDest).iSDParm,
                    (*pIn).nSdst,
                );
            }
            SRT_Coroutine => {
                if (*pDest).iSdst == 0 as ::core::ffi::c_int {
                    (*pDest).iSdst = sqlite3GetTempRange(pParse, (*pIn).nSdst);
                    (*pDest).nSdst = (*pIn).nSdst;
                }
                sqlite3ExprCodeMove(pParse, (*pIn).iSdst, (*pDest).iSdst, (*pIn).nSdst);
                sqlite3VdbeAddOp1(v, OP_Yield, (*pDest).iSDParm);
            }
            _ => {
                sqlite3VdbeAddOp2(v, OP_ResultRow, (*pIn).iSdst, (*pIn).nSdst);
            }
        }
        if (*p).iLimit != 0 {
            sqlite3VdbeAddOp2(v, OP_DecrJumpZero, (*p).iLimit, iBreak);
        }
        sqlite3VdbeResolveLabel(v, iContinue);
        sqlite3VdbeAddOp1(v, OP_Return, regReturn);
        return addr;
    }
}
unsafe extern "C" fn multiSelectOrderBy(
    mut pParse: *mut Parse,
    mut p: *mut Select,
    mut pDest: *mut SelectDest,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut pPrior: *mut Select = ::core::ptr::null_mut::<Select>();
        let mut pSplit: *mut Select = ::core::ptr::null_mut::<Select>();
        let mut nSelect: ::core::ffi::c_int = 0;
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        let mut destA: SelectDest = SelectDest {
            eDest: 0,
            iSDParm: 0,
            iSDParm2: 0,
            iSdst: 0,
            nSdst: 0,
            zAffSdst: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            pOrderBy: ::core::ptr::null_mut::<ExprList>()

        };
        let mut destB: SelectDest = SelectDest {
            eDest: 0,
            iSDParm: 0,
            iSDParm2: 0,
            iSdst: 0,
            nSdst: 0,
            zAffSdst: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            pOrderBy: ::core::ptr::null_mut::<ExprList>()

        };
        let mut regAddrA: ::core::ffi::c_int = 0;
        let mut regAddrB: ::core::ffi::c_int = 0;
        let mut addrSelectA: ::core::ffi::c_int = 0;
        let mut addrSelectB: ::core::ffi::c_int = 0;
        let mut regOutA: ::core::ffi::c_int = 0;
        let mut regOutB: ::core::ffi::c_int = 0;
        let mut addrOutA: ::core::ffi::c_int = 0;
        let mut addrOutB: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut addrEofA: ::core::ffi::c_int = 0;
        let mut addrEofA_noB: ::core::ffi::c_int = 0;
        let mut addrEofB: ::core::ffi::c_int = 0;
        let mut addrAltB: ::core::ffi::c_int = 0;
        let mut addrAeqB: ::core::ffi::c_int = 0;
        let mut addrAgtB: ::core::ffi::c_int = 0;
        let mut regLimitA: ::core::ffi::c_int = 0;
        let mut regLimitB: ::core::ffi::c_int = 0;
        let mut regPrev: ::core::ffi::c_int = 0;
        let mut savedLimit: ::core::ffi::c_int = 0;
        let mut savedOffset: ::core::ffi::c_int = 0;
        let mut labelCmpr: ::core::ffi::c_int = 0;
        let mut labelEnd: ::core::ffi::c_int = 0;
        let mut addr1: ::core::ffi::c_int = 0;
        let mut op: ::core::ffi::c_int = 0;
        let mut pKeyDup: *mut KeyInfo = ::core::ptr::null_mut::<KeyInfo>();
        let mut pKeyMerge: *mut KeyInfo = ::core::ptr::null_mut::<KeyInfo>();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut pOrderBy: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        let mut nOrderBy: ::core::ffi::c_int = 0;
        let mut aPermute: *mut u32_0 = ::core::ptr::null_mut::<u32_0>();
        db = (*pParse).db;
        v = (*pParse).pVdbe;
        labelEnd = sqlite3VdbeMakeLabel(pParse);
        labelCmpr = sqlite3VdbeMakeLabel(pParse);
        op = (*p).op as ::core::ffi::c_int;
        pOrderBy = (*p).pOrderBy;
        nOrderBy = (*pOrderBy).nExpr;
        if op != TK_ALL {
            i = 1 as ::core::ffi::c_int;
            while (*db).mallocFailed as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && i <= (*(*p).pEList).nExpr
            {
                let mut pItem: *mut ExprList_item = ::core::ptr::null_mut::<
                    ExprList_item,
                >();
                j = 0 as ::core::ffi::c_int;
                pItem = &raw mut (*pOrderBy).a as *mut ExprList_item
                    as *mut ExprList_item;
                while j < nOrderBy {
                    if (*pItem).u.x.iOrderByCol as ::core::ffi::c_int == i {
                        break;
                    }
                    j += 1;
                    pItem = pItem.offset(1);
                }
                if j == nOrderBy {
                    let mut pNew: *mut Expr = sqlite3Expr(
                        db,
                        TK_INTEGER,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    );
                    if pNew.is_null() {
                        return SQLITE_NOMEM_BKPT;
                    }
                    (*pNew).flags = ((*pNew).flags as ::core::ffi::c_uint
                        | EP_IntValue as ::core::ffi::c_uint) as u32_0;
                    (*pNew).u.iValue = i;
                    pOrderBy = sqlite3ExprListAppend(pParse, pOrderBy, pNew);
                    (*p).pOrderBy = pOrderBy;
                    if !pOrderBy.is_null() {
                        let c2rust_fresh29 = nOrderBy;
                        nOrderBy = nOrderBy + 1;
                        (*(&raw mut (*pOrderBy).a as *mut ExprList_item)
                            .offset(c2rust_fresh29 as isize))
                            .u
                            .x
                            .iOrderByCol = i as u16_0;
                    }
                }
                i += 1;
            }
        }
        aPermute = sqlite3DbMallocRawNN(
            db,
            (::core::mem::size_of::<u32_0>() as usize)
                .wrapping_mul((nOrderBy + 1 as ::core::ffi::c_int) as usize) as u64_0,
        ) as *mut u32_0;
        if !aPermute.is_null() {
            let mut pItem_0: *mut ExprList_item = ::core::ptr::null_mut::<
                ExprList_item,
            >();
            *aPermute.offset(0 as ::core::ffi::c_int as isize) = nOrderBy as u32_0;
            i = 1 as ::core::ffi::c_int;
            pItem_0 = &raw mut (*pOrderBy).a as *mut ExprList_item as *mut ExprList_item;
            while i <= nOrderBy {
                *aPermute.offset(i as isize) = ((*pItem_0).u.x.iOrderByCol
                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as u32_0;
                i += 1;
                pItem_0 = pItem_0.offset(1);
            }
            pKeyMerge = multiSelectOrderByKeyInfo(pParse, p, 1 as ::core::ffi::c_int);
        } else {
            pKeyMerge = ::core::ptr::null_mut::<KeyInfo>();
        }
        if op == TK_ALL {
            regPrev = 0 as ::core::ffi::c_int;
        } else {
            let mut nExpr: ::core::ffi::c_int = (*(*p).pEList).nExpr;
            regPrev = (*pParse).nMem + 1 as ::core::ffi::c_int;
            (*pParse).nMem += nExpr + 1 as ::core::ffi::c_int;
            sqlite3VdbeAddOp2(v, OP_Integer, 0 as ::core::ffi::c_int, regPrev);
            pKeyDup = sqlite3KeyInfoAlloc(db, nExpr, 1 as ::core::ffi::c_int);
            if !pKeyDup.is_null() {
                i = 0 as ::core::ffi::c_int;
                while i < nExpr {
                    let ref mut c2rust_fresh30 = *(&raw mut (*pKeyDup).aColl
                        as *mut *mut CollSeq)
                        .offset(i as isize);
                    *c2rust_fresh30 = multiSelectCollSeq(pParse, p, i);
                    *(*pKeyDup).aSortFlags.offset(i as isize) = 0 as u8_0;
                    i += 1;
                }
            }
        }
        nSelect = 1 as ::core::ffi::c_int;
        if (op == TK_ALL || op == TK_UNION)
            && (*db).dbOptFlags & 0x200000 as u32_0 == 0 as u32_0
        {
            pSplit = p;
            while !(*pSplit).pPrior.is_null() && (*pSplit).op as ::core::ffi::c_int == op
            {
                nSelect += 1;
                pSplit = (*pSplit).pPrior;
            }
        }
        if nSelect <= 3 as ::core::ffi::c_int {
            pSplit = p;
        } else {
            pSplit = p;
            i = 2 as ::core::ffi::c_int;
            while i < nSelect {
                pSplit = (*pSplit).pPrior;
                i += 2 as ::core::ffi::c_int;
            }
        }
        pPrior = (*pSplit).pPrior;
        (*pSplit).pPrior = ::core::ptr::null_mut::<Select>();
        (*pPrior).pNext = ::core::ptr::null_mut::<Select>();
        (*pPrior).pOrderBy = sqlite3ExprListDup(
            (*pParse).db,
            pOrderBy,
            0 as ::core::ffi::c_int,
        );
        sqlite3ResolveOrderGroupBy(
            pParse,
            p,
            (*p).pOrderBy,
            b"ORDER\0".as_ptr() as *const ::core::ffi::c_char,
        );
        sqlite3ResolveOrderGroupBy(
            pParse,
            pPrior,
            (*pPrior).pOrderBy,
            b"ORDER\0".as_ptr() as *const ::core::ffi::c_char,
        );
        computeLimitRegisters(pParse, p, labelEnd);
        if (*p).iLimit != 0 && op == TK_ALL {
            (*pParse).nMem += 1;
            regLimitA = (*pParse).nMem;
            (*pParse).nMem += 1;
            regLimitB = (*pParse).nMem;
            sqlite3VdbeAddOp2(
                v,
                OP_Copy,
                if (*p).iOffset != 0 {
                    (*p).iOffset + 1 as ::core::ffi::c_int
                } else {
                    (*p).iLimit
                },
                regLimitA,
            );
            sqlite3VdbeAddOp2(v, OP_Copy, regLimitA, regLimitB);
        } else {
            regLimitB = 0 as ::core::ffi::c_int;
            regLimitA = regLimitB;
        }
        sqlite3ExprDelete(db, (*p).pLimit);
        (*p).pLimit = ::core::ptr::null_mut::<Expr>();
        (*pParse).nMem += 1;
        regAddrA = (*pParse).nMem;
        (*pParse).nMem += 1;
        regAddrB = (*pParse).nMem;
        (*pParse).nMem += 1;
        regOutA = (*pParse).nMem;
        (*pParse).nMem += 1;
        regOutB = (*pParse).nMem;
        sqlite3SelectDestInit(&raw mut destA, SRT_Coroutine, regAddrA);
        sqlite3SelectDestInit(&raw mut destB, SRT_Coroutine, regAddrB);
        sqlite3VdbeExplain(
            pParse,
            1 as u8_0,
            b"MERGE (%s)\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3SelectOpName((*p).op as ::core::ffi::c_int),
        );
        addrSelectA = sqlite3VdbeCurrentAddr(v) + 1 as ::core::ffi::c_int;
        addr1 = sqlite3VdbeAddOp3(
            v,
            OP_InitCoroutine,
            regAddrA,
            0 as ::core::ffi::c_int,
            addrSelectA,
        );
        (*pPrior).iLimit = regLimitA;
        sqlite3VdbeExplain(
            pParse,
            1 as u8_0,
            b"LEFT\0".as_ptr() as *const ::core::ffi::c_char,
        );
        sqlite3Select(pParse, pPrior, &raw mut destA);
        sqlite3VdbeEndCoroutine(v, regAddrA);
        sqlite3VdbeJumpHere(v, addr1);
        addrSelectB = sqlite3VdbeCurrentAddr(v) + 1 as ::core::ffi::c_int;
        addr1 = sqlite3VdbeAddOp3(
            v,
            OP_InitCoroutine,
            regAddrB,
            0 as ::core::ffi::c_int,
            addrSelectB,
        );
        savedLimit = (*p).iLimit;
        savedOffset = (*p).iOffset;
        (*p).iLimit = regLimitB;
        (*p).iOffset = 0 as ::core::ffi::c_int;
        sqlite3VdbeExplain(
            pParse,
            1 as u8_0,
            b"RIGHT\0".as_ptr() as *const ::core::ffi::c_char,
        );
        sqlite3Select(pParse, p, &raw mut destB);
        (*p).iLimit = savedLimit;
        (*p).iOffset = savedOffset;
        sqlite3VdbeEndCoroutine(v, regAddrB);
        addrOutA = generateOutputSubroutine(
            pParse,
            p,
            &raw mut destA,
            pDest,
            regOutA,
            regPrev,
            pKeyDup,
            labelEnd,
        );
        if op == TK_ALL || op == TK_UNION {
            addrOutB = generateOutputSubroutine(
                pParse,
                p,
                &raw mut destB,
                pDest,
                regOutB,
                regPrev,
                pKeyDup,
                labelEnd,
            );
        }
        sqlite3KeyInfoUnref(pKeyDup);
        if op == TK_EXCEPT || op == TK_INTERSECT {
            addrEofA = labelEnd;
            addrEofA_noB = addrEofA;
        } else {
            addrEofA = sqlite3VdbeAddOp2(v, OP_Gosub, regOutB, addrOutB);
            addrEofA_noB = sqlite3VdbeAddOp2(v, OP_Yield, regAddrB, labelEnd);
            sqlite3VdbeGoto(v, addrEofA);
            (*p).nSelectRow = sqlite3LogEstAdd((*p).nSelectRow, (*pPrior).nSelectRow);
        }
        if op == TK_INTERSECT {
            addrEofB = addrEofA;
            if (*p).nSelectRow as ::core::ffi::c_int
                > (*pPrior).nSelectRow as ::core::ffi::c_int
            {
                (*p).nSelectRow = (*pPrior).nSelectRow;
            }
        } else {
            addrEofB = sqlite3VdbeAddOp2(v, OP_Gosub, regOutA, addrOutA);
            sqlite3VdbeAddOp2(v, OP_Yield, regAddrA, labelEnd);
            sqlite3VdbeGoto(v, addrEofB);
        }
        addrAltB = sqlite3VdbeAddOp2(v, OP_Gosub, regOutA, addrOutA);
        sqlite3VdbeAddOp2(v, OP_Yield, regAddrA, addrEofA);
        sqlite3VdbeGoto(v, labelCmpr);
        if op == TK_ALL {
            addrAeqB = addrAltB;
        } else if op == TK_INTERSECT {
            addrAeqB = addrAltB;
            addrAltB += 1;
        } else {
            addrAeqB = sqlite3VdbeAddOp2(v, OP_Yield, regAddrA, addrEofA);
            sqlite3VdbeGoto(v, labelCmpr);
        }
        addrAgtB = sqlite3VdbeCurrentAddr(v);
        if op == TK_ALL || op == TK_UNION {
            sqlite3VdbeAddOp2(v, OP_Gosub, regOutB, addrOutB);
        }
        sqlite3VdbeAddOp2(v, OP_Yield, regAddrB, addrEofB);
        sqlite3VdbeGoto(v, labelCmpr);
        sqlite3VdbeJumpHere(v, addr1);
        sqlite3VdbeAddOp2(v, OP_Yield, regAddrA, addrEofA_noB);
        sqlite3VdbeAddOp2(v, OP_Yield, regAddrB, addrEofB);
        sqlite3VdbeResolveLabel(v, labelCmpr);
        sqlite3VdbeAddOp4(
            v,
            OP_Permutation,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            aPermute as *mut ::core::ffi::c_char,
            P4_INTARRAY,
        );
        sqlite3VdbeAddOp4(
            v,
            OP_Compare,
            destA.iSdst,
            destB.iSdst,
            nOrderBy,
            pKeyMerge as *mut ::core::ffi::c_char,
            P4_KEYINFO,
        );
        sqlite3VdbeChangeP5(v, OPFLAG_PERMUTE as u16_0);
        sqlite3VdbeAddOp3(v, OP_Jump, addrAltB, addrAeqB, addrAgtB);
        sqlite3VdbeResolveLabel(v, labelEnd);
        if !(*pSplit).pPrior.is_null() {
            sqlite3ParserAddCleanup(
                pParse,
                Some(
                    sqlite3SelectDeleteGeneric
                        as unsafe extern "C" fn(
                            *mut sqlite3,
                            *mut ::core::ffi::c_void,
                        ) -> (),
                ),
                (*pSplit).pPrior as *mut ::core::ffi::c_void,
            );
        }
        (*pSplit).pPrior = pPrior;
        (*pPrior).pNext = pSplit;
        sqlite3ExprListDelete(db, (*pPrior).pOrderBy);
        (*pPrior).pOrderBy = ::core::ptr::null_mut::<ExprList>();
        sqlite3VdbeExplainPop(pParse);
        return ((*pParse).nErr != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn substExpr(
    mut pSubst: *mut SubstContext,
    mut pExpr: *mut Expr,
) -> *mut Expr {
    unsafe {
        if pExpr.is_null() {
            return ::core::ptr::null_mut::<Expr>();
        }
        if (*pExpr).flags
            & (0x1 as ::core::ffi::c_int | 0x2 as ::core::ffi::c_int) as u32_0
            != 0 as u32_0 && (*pExpr).w.iJoin == (*pSubst).iTable
        {
            (*pExpr).w.iJoin = (*pSubst).iNewTable;
        }
        if (*pExpr).op as ::core::ffi::c_int == TK_COLUMN
            && (*pExpr).iTable == (*pSubst).iTable
            && !((*pExpr).flags & 0x20 as ::core::ffi::c_int as u32_0 != 0 as u32_0)
        {
            let mut pNew: *mut Expr = ::core::ptr::null_mut::<Expr>();
            let mut iColumn: ::core::ffi::c_int = 0;
            let mut pCopy: *mut Expr = ::core::ptr::null_mut::<Expr>();
            let mut ifNullRow: Expr = Expr {
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
            iColumn = (*pExpr).iColumn as ::core::ffi::c_int;
            pCopy = (*(&raw mut (*(*pSubst).pEList).a as *mut ExprList_item)
                .offset(iColumn as isize))
                .pExpr;
            if sqlite3ExprIsVector(pCopy) != 0 {
                sqlite3VectorErrorMsg((*pSubst).pParse, pCopy);
            } else {
                let mut db: *mut sqlite3 = (*(*pSubst).pParse).db;
                if (*pSubst).isOuterJoin != 0
                    && ((*pCopy).op as ::core::ffi::c_int != TK_COLUMN
                        || (*pCopy).iTable != (*pSubst).iNewTable)
                {
                    memset(
                        &raw mut ifNullRow as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ::core::mem::size_of::<Expr>() as size_t,
                    );
                    ifNullRow.op = TK_IF_NULL_ROW as u8_0;
                    ifNullRow.pLeft = pCopy;
                    ifNullRow.iTable = (*pSubst).iNewTable;
                    ifNullRow.iColumn = -99 as ::core::ffi::c_int as ynVar;
                    ifNullRow.flags = EP_IfNullRow as u32_0;
                    pCopy = &raw mut ifNullRow;
                }
                pNew = sqlite3ExprDup(db, pCopy, 0 as ::core::ffi::c_int);
                if (*db).mallocFailed != 0 {
                    sqlite3ExprDelete(db, pNew);
                    return pExpr;
                }
                if (*pSubst).isOuterJoin != 0 {
                    (*pNew).flags = ((*pNew).flags as ::core::ffi::c_uint
                        | 0x200000 as ::core::ffi::c_int as u32_0 as ::core::ffi::c_uint)
                        as u32_0;
                }
                if (*pNew).op as ::core::ffi::c_int == TK_TRUEFALSE {
                    (*pNew).u.iValue = sqlite3ExprTruthValue(pNew);
                    (*pNew).op = TK_INTEGER as u8_0;
                    (*pNew).flags = ((*pNew).flags as ::core::ffi::c_uint
                        | 0x800 as ::core::ffi::c_int as u32_0 as ::core::ffi::c_uint)
                        as u32_0;
                }
                let mut pNat: *mut CollSeq = sqlite3ExprCollSeq((*pSubst).pParse, pNew);
                let mut pColl: *mut CollSeq = sqlite3ExprCollSeq(
                    (*pSubst).pParse,
                    (*(&raw mut (*(*pSubst).pCList).a as *mut ExprList_item)
                        .offset(iColumn as isize))
                        .pExpr,
                );
                if pNat != pColl
                    || (*pNew).op as ::core::ffi::c_int != TK_COLUMN
                        && (*pNew).op as ::core::ffi::c_int != TK_COLLATE
                {
                    pNew = sqlite3ExprAddCollateString(
                        (*pSubst).pParse,
                        pNew,
                        if !pColl.is_null() {
                            (*pColl).zName as *const ::core::ffi::c_char
                        } else {
                            b"BINARY\0".as_ptr() as *const ::core::ffi::c_char
                        },
                    );
                }
                (*pNew).flags = ((*pNew).flags as ::core::ffi::c_uint
                    & !(0x200 as ::core::ffi::c_int as u32_0) as ::core::ffi::c_uint)
                    as u32_0;
                if (*pExpr).flags
                    & (0x1 as ::core::ffi::c_int | 0x2 as ::core::ffi::c_int) as u32_0
                    != 0 as u32_0
                {
                    sqlite3SetJoinExpr(
                        pNew,
                        (*pExpr).w.iJoin,
                        (*pExpr).flags & (EP_OuterON | EP_InnerON) as u32_0,
                    );
                }
                sqlite3ExprDelete(db, pExpr);
                pExpr = pNew;
            }
        } else {
            if (*pExpr).op as ::core::ffi::c_int == TK_IF_NULL_ROW
                && (*pExpr).iTable == (*pSubst).iTable
            {
                (*pExpr).iTable = (*pSubst).iNewTable;
            }
            if (*pExpr).op as ::core::ffi::c_int == TK_AGG_FUNCTION
                && (*pExpr).op2 as ::core::ffi::c_int >= (*pSubst).nSelDepth
            {
                (*pExpr).op2 = (*pExpr).op2.wrapping_sub(1);
            }
            (*pExpr).pLeft = substExpr(pSubst, (*pExpr).pLeft);
            (*pExpr).pRight = substExpr(pSubst, (*pExpr).pRight);
            if (*pExpr).flags & EP_xIsSelect as u32_0 != 0 as u32_0 {
                substSelect(pSubst, (*pExpr).x.pSelect, 1 as ::core::ffi::c_int);
            } else {
                substExprList(pSubst, (*pExpr).x.pList);
            }
            if (*pExpr).flags & 0x1000000 as ::core::ffi::c_int as u32_0 != 0 as u32_0 {
                let mut pWin: *mut Window = (*pExpr).y.pWin;
                (*pWin).pFilter = substExpr(pSubst, (*pWin).pFilter);
                substExprList(pSubst, (*pWin).pPartition);
                substExprList(pSubst, (*pWin).pOrderBy);
            }
        }
        return pExpr;
    }
}
unsafe extern "C" fn substExprList(
    mut pSubst: *mut SubstContext,
    mut pList: *mut ExprList,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        if pList.is_null() {
            return;
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*pList).nExpr {
            let ref mut c2rust_fresh17 = (*(&raw mut (*pList).a as *mut ExprList_item)
                .offset(i as isize))
                .pExpr;
            *c2rust_fresh17 = substExpr(
                pSubst,
                (*(&raw mut (*pList).a as *mut ExprList_item).offset(i as isize)).pExpr,
            );
            i += 1;
        }
    }
}
unsafe extern "C" fn substSelect(
    mut pSubst: *mut SubstContext,
    mut p: *mut Select,
    mut doPrior: ::core::ffi::c_int,
) {
    unsafe {
        let mut pSrc: *mut SrcList = ::core::ptr::null_mut::<SrcList>();
        let mut pItem: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
        let mut i: ::core::ffi::c_int = 0;
        if p.is_null() {
            return;
        }
        (*pSubst).nSelDepth += 1;
        loop {
            substExprList(pSubst, (*p).pEList);
            substExprList(pSubst, (*p).pGroupBy);
            substExprList(pSubst, (*p).pOrderBy);
            (*p).pHaving = substExpr(pSubst, (*p).pHaving);
            (*p).pWhere = substExpr(pSubst, (*p).pWhere);
            pSrc = (*p).pSrc;
            i = (*pSrc).nSrc;
            pItem = &raw mut (*pSrc).a as *mut SrcItem;
            while i > 0 as ::core::ffi::c_int {
                if (*pItem).fg.isSubquery() != 0 {
                    substSelect(
                        pSubst,
                        (*(*pItem).u4.pSubq).pSelect,
                        1 as ::core::ffi::c_int,
                    );
                }
                if (*pItem).fg.isTabFunc() != 0 {
                    substExprList(pSubst, (*pItem).u1.pFuncArg);
                }
                i -= 1;
                pItem = pItem.offset(1);
            }
            if !(doPrior != 0
                && {
                    p = (*p).pPrior;
                    !p.is_null()
                })
            {
                break;
            }
        }
        (*pSubst).nSelDepth -= 1;
    }
}
unsafe extern "C" fn recomputeColumnsUsedExpr(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pItem: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
        if (*pExpr).op as ::core::ffi::c_int != TK_COLUMN {
            return WRC_Continue;
        }
        pItem = (*pWalker).u.pSrcItem;
        if (*pItem).iCursor != (*pExpr).iTable {
            return WRC_Continue;
        }
        if ((*pExpr).iColumn as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            return WRC_Continue;
        }
        (*pItem).colUsed |= sqlite3ExprColUsed(pExpr) as ::core::ffi::c_ulonglong;
        return WRC_Continue;
    }
}
unsafe extern "C" fn recomputeColumnsUsed(
    mut pSelect: *mut Select,
    mut pSrcItem: *mut SrcItem,
) {
    unsafe {
        let mut w: Walker = Walker {
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

        };
        if (*pSrcItem).pSTab.is_null() {
            return;
        }
        memset(
            &raw mut w as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<Walker>() as size_t,
        );
        w.xExprCallback = Some(
            recomputeColumnsUsedExpr
                as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int>;
        w.xSelectCallback = Some(
            sqlite3SelectWalkNoop
                as unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
            >;
        w.u.pSrcItem = pSrcItem;
        (*pSrcItem).colUsed = 0 as Bitmask;
        sqlite3WalkSelect(&raw mut w, pSelect);
    }
}
unsafe extern "C" fn srclistRenumberCursors(
    mut pParse: *mut Parse,
    mut aCsrMap: *mut ::core::ffi::c_int,
    mut pSrc: *mut SrcList,
    mut iExcept: ::core::ffi::c_int,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut pItem: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
        i = 0 as ::core::ffi::c_int;
        pItem = &raw mut (*pSrc).a as *mut SrcItem;
        while i < (*pSrc).nSrc {
            if i != iExcept {
                let mut p: *mut Select = ::core::ptr::null_mut::<Select>();
                if (*pItem).fg.isRecursive() == 0
                    || *aCsrMap
                        .offset(((*pItem).iCursor + 1 as ::core::ffi::c_int) as isize)
                        == 0 as ::core::ffi::c_int
                {
                    let c2rust_fresh23 = (*pParse).nTab;
                    (*pParse).nTab = (*pParse).nTab + 1;
                    *aCsrMap
                        .offset(((*pItem).iCursor + 1 as ::core::ffi::c_int) as isize) = c2rust_fresh23;
                }
                (*pItem).iCursor = *aCsrMap
                    .offset(((*pItem).iCursor + 1 as ::core::ffi::c_int) as isize);
                if (*pItem).fg.isSubquery() != 0 {
                    p = (*(*pItem).u4.pSubq).pSelect;
                    while !p.is_null() {
                        srclistRenumberCursors(
                            pParse,
                            aCsrMap,
                            (*p).pSrc,
                            -1 as ::core::ffi::c_int,
                        );
                        p = (*p).pPrior;
                    }
                }
            }
            i += 1;
            pItem = pItem.offset(1);
        }
    }
}
unsafe extern "C" fn renumberCursorDoMapping(
    mut pWalker: *mut Walker,
    mut piCursor: *mut ::core::ffi::c_int,
) {
    unsafe {
        let mut aCsrMap: *mut ::core::ffi::c_int = (*pWalker).u.aiCol;
        let mut iCsr: ::core::ffi::c_int = *piCursor;
        if iCsr < *aCsrMap.offset(0 as ::core::ffi::c_int as isize)
            && *aCsrMap.offset((iCsr + 1 as ::core::ffi::c_int) as isize)
                > 0 as ::core::ffi::c_int
        {
            *piCursor = *aCsrMap.offset((iCsr + 1 as ::core::ffi::c_int) as isize);
        }
    }
}
unsafe extern "C" fn renumberCursorsCb(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut op: ::core::ffi::c_int = (*pExpr).op as ::core::ffi::c_int;
        if op == TK_COLUMN || op == TK_IF_NULL_ROW {
            renumberCursorDoMapping(pWalker, &raw mut (*pExpr).iTable);
        }
        if (*pExpr).flags & 0x1 as ::core::ffi::c_int as u32_0 != 0 as u32_0 {
            renumberCursorDoMapping(pWalker, &raw mut (*pExpr).w.iJoin);
        }
        return WRC_Continue;
    }
}
unsafe extern "C" fn renumberCursors(
    mut pParse: *mut Parse,
    mut p: *mut Select,
    mut iExcept: ::core::ffi::c_int,
    mut aCsrMap: *mut ::core::ffi::c_int,
) {
    unsafe {
        let mut w: Walker = Walker {
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

        };
        srclistRenumberCursors(pParse, aCsrMap, (*p).pSrc, iExcept);
        memset(
            &raw mut w as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<Walker>() as size_t,
        );
        w.u.aiCol = aCsrMap;
        w.xExprCallback = Some(
            renumberCursorsCb
                as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int>;
        w.xSelectCallback = Some(
            sqlite3SelectWalkNoop
                as unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
            >;
        sqlite3WalkSelect(&raw mut w, p);
    }
}
unsafe extern "C" fn findLeftmostExprlist(mut pSel: *mut Select) -> *mut ExprList {
    unsafe {
        while !(*pSel).pPrior.is_null() {
            pSel = (*pSel).pPrior;
        }
        return (*pSel).pEList;
    }
}
unsafe extern "C" fn compoundHasDifferentAffinities(
    mut p: *mut Select,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ii: ::core::ffi::c_int = 0;
        let mut pList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        pList = (*p).pEList;
        ii = 0 as ::core::ffi::c_int;
        while ii < (*pList).nExpr {
            let mut aff: ::core::ffi::c_char = 0;
            let mut pSub1: *mut Select = ::core::ptr::null_mut::<Select>();
            aff = sqlite3ExprAffinity(
                (*(&raw mut (*pList).a as *mut ExprList_item).offset(ii as isize)).pExpr,
            );
            pSub1 = (*p).pPrior;
            while !pSub1.is_null() {
                if sqlite3ExprAffinity(
                    (*(&raw mut (*(*pSub1).pEList).a as *mut ExprList_item)
                        .offset(ii as isize))
                        .pExpr,
                ) as ::core::ffi::c_int != aff as ::core::ffi::c_int
                {
                    return 1 as ::core::ffi::c_int;
                }
                pSub1 = (*pSub1).pPrior;
            }
            ii += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn flattenSubquery(
    mut pParse: *mut Parse,
    mut p: *mut Select,
    mut iFrom: ::core::ffi::c_int,
    mut isAgg: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zSavedAuthContext: *const ::core::ffi::c_char = (*pParse).zAuthContext;
        let mut pParent: *mut Select = ::core::ptr::null_mut::<Select>();
        let mut pSub: *mut Select = ::core::ptr::null_mut::<Select>();
        let mut pSub1: *mut Select = ::core::ptr::null_mut::<Select>();
        let mut pSrc: *mut SrcList = ::core::ptr::null_mut::<SrcList>();
        let mut pSubSrc: *mut SrcList = ::core::ptr::null_mut::<SrcList>();
        let mut iParent: ::core::ffi::c_int = 0;
        let mut iNewParent: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut isOuterJoin: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0;
        let mut pWhere: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut pSubitem: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut w: Walker = Walker {
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

        };
        let mut aCsrMap: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
            ::core::ffi::c_int,
        >();
        if (*db).dbOptFlags & 0x1 as u32_0 != 0 as u32_0 {
            return 0 as ::core::ffi::c_int;
        }
        pSrc = (*p).pSrc;
        pSubitem = (&raw mut (*pSrc).a as *mut SrcItem).offset(iFrom as isize)
            as *mut SrcItem;
        iParent = (*pSubitem).iCursor;
        pSub = (*(*pSubitem).u4.pSubq).pSelect;
        if !(*p).pWin.is_null() || !(*pSub).pWin.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        pSubSrc = (*pSub).pSrc;
        if !(*pSub).pLimit.is_null() && !(*p).pLimit.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if !(*pSub).pLimit.is_null() && !(*(*pSub).pLimit).pRight.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if (*p).selFlags & SF_Compound as u32_0 != 0 as u32_0
            && !(*pSub).pLimit.is_null()
        {
            return 0 as ::core::ffi::c_int;
        }
        if (*pSubSrc).nSrc == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        if (*pSub).selFlags & SF_Distinct as u32_0 != 0 {
            return 0 as ::core::ffi::c_int;
        }
        if !(*pSub).pLimit.is_null()
            && ((*pSrc).nSrc > 1 as ::core::ffi::c_int || isAgg != 0)
        {
            return 0 as ::core::ffi::c_int;
        }
        if !(*p).pOrderBy.is_null() && !(*pSub).pOrderBy.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if isAgg != 0 && !(*pSub).pOrderBy.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if !(*pSub).pLimit.is_null() && !(*p).pWhere.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if !(*pSub).pLimit.is_null()
            && (*p).selFlags & SF_Distinct as u32_0 != 0 as u32_0
        {
            return 0 as ::core::ffi::c_int;
        }
        if (*pSub).selFlags & 0x2000 as u32_0 != 0 {
            return 0 as ::core::ffi::c_int;
        }
        if (*pSubitem).fg.jointype as ::core::ffi::c_int & (JT_OUTER | JT_LTORJ)
            != 0 as ::core::ffi::c_int
        {
            if (*pSubSrc).nSrc > 1 as ::core::ffi::c_int
                || (*p).selFlags & SF_Distinct as u32_0 != 0 as u32_0
                || (*pSubitem).fg.jointype as ::core::ffi::c_int & JT_RIGHT
                    != 0 as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            isOuterJoin = 1 as ::core::ffi::c_int;
        }
        if iFrom > 0 as ::core::ffi::c_int
            && (*(&raw mut (*pSubSrc).a as *mut SrcItem)
                .offset(0 as ::core::ffi::c_int as isize))
                .fg
                .jointype as ::core::ffi::c_int & JT_LTORJ != 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        if !(*pSub).pPrior.is_null() {
            let mut ii: ::core::ffi::c_int = 0;
            if !(*pSub).pOrderBy.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            if isAgg != 0 || (*p).selFlags & SF_Distinct as u32_0 != 0 as u32_0
                || isOuterJoin > 0 as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            pSub1 = pSub;
            while !pSub1.is_null() {
                if (*pSub1).selFlags & (SF_Distinct | SF_Aggregate) as u32_0
                    != 0 as u32_0
                    || !(*pSub1).pPrior.is_null()
                        && (*pSub1).op as ::core::ffi::c_int != TK_ALL
                    || (*(*pSub1).pSrc).nSrc < 1 as ::core::ffi::c_int
                    || !(*pSub1).pWin.is_null()
                {
                    return 0 as ::core::ffi::c_int;
                }
                if iFrom > 0 as ::core::ffi::c_int
                    && (*(&raw mut (*(*pSub1).pSrc).a as *mut SrcItem)
                        .offset(0 as ::core::ffi::c_int as isize))
                        .fg
                        .jointype as ::core::ffi::c_int & JT_LTORJ
                        != 0 as ::core::ffi::c_int
                {
                    return 0 as ::core::ffi::c_int;
                }
                pSub1 = (*pSub1).pPrior;
            }
            if !(*p).pOrderBy.is_null() {
                ii = 0 as ::core::ffi::c_int;
                while ii < (*(*p).pOrderBy).nExpr {
                    if (*(&raw mut (*(*p).pOrderBy).a as *mut ExprList_item)
                        .offset(ii as isize))
                        .u
                        .x
                        .iOrderByCol as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                    ii += 1;
                }
            }
            if (*p).selFlags & SF_Recursive as u32_0 != 0 {
                return 0 as ::core::ffi::c_int;
            }
            if compoundHasDifferentAffinities(pSub) != 0 {
                return 0 as ::core::ffi::c_int;
            }
            if (*pSrc).nSrc > 1 as ::core::ffi::c_int {
                if (*pParse).nSelect > 500 as ::core::ffi::c_int {
                    return 0 as ::core::ffi::c_int;
                }
                if (*db).dbOptFlags & 0x800000 as u32_0 != 0 as u32_0 {
                    return 0 as ::core::ffi::c_int;
                }
                aCsrMap = sqlite3DbMallocZero(
                    db,
                    (((*pParse).nTab as ::core::ffi::c_longlong
                        + 1 as ::core::ffi::c_longlong) as u64_0)
                        .wrapping_mul(
                            ::core::mem::size_of::<::core::ffi::c_int>() as u64_0,
                        ),
                ) as *mut ::core::ffi::c_int;
                if !aCsrMap.is_null() {
                    *aCsrMap.offset(0 as ::core::ffi::c_int as isize) = (*pParse).nTab;
                }
            }
        }
        (*pParse).zAuthContext = (*pSubitem).zName;
        sqlite3AuthCheck(
            pParse,
            SQLITE_SELECT,
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        (*pParse).zAuthContext = zSavedAuthContext;
        if (*pSubitem).fg.isSubquery() != 0 {
            pSub1 = sqlite3SubqueryDetach(db, pSubitem);
        } else {
            pSub1 = ::core::ptr::null_mut::<Select>();
        }
        sqlite3DbFree(db, (*pSubitem).zName as *mut ::core::ffi::c_void);
        sqlite3DbFree(db, (*pSubitem).zAlias as *mut ::core::ffi::c_void);
        (*pSubitem).zName = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*pSubitem).zAlias = ::core::ptr::null_mut::<::core::ffi::c_char>();
        pSub = (*pSub).pPrior;
        while !pSub.is_null() {
            let mut pNew: *mut Select = ::core::ptr::null_mut::<Select>();
            let mut pOrderBy: *mut ExprList = (*p).pOrderBy;
            let mut pLimit: *mut Expr = (*p).pLimit;
            let mut pPrior: *mut Select = (*p).pPrior;
            let mut pItemTab: *mut Table = (*pSubitem).pSTab;
            (*pSubitem).pSTab = ::core::ptr::null_mut::<Table>();
            (*p).pOrderBy = ::core::ptr::null_mut::<ExprList>();
            (*p).pPrior = ::core::ptr::null_mut::<Select>();
            (*p).pLimit = ::core::ptr::null_mut::<Expr>();
            pNew = sqlite3SelectDup(db, p, 0 as ::core::ffi::c_int);
            (*p).pLimit = pLimit;
            (*p).pOrderBy = pOrderBy;
            (*p).op = TK_ALL as u8_0;
            (*pSubitem).pSTab = pItemTab;
            if pNew.is_null() {
                (*p).pPrior = pPrior;
            } else {
                (*pParse).nSelect += 1;
                (*pNew).selId = (*pParse).nSelect as u32_0;
                if !aCsrMap.is_null()
                    && (*db).mallocFailed as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                {
                    renumberCursors(pParse, pNew, iFrom, aCsrMap);
                }
                (*pNew).pPrior = pPrior;
                if !pPrior.is_null() {
                    (*pPrior).pNext = pNew;
                }
                (*pNew).pNext = p;
                (*p).pPrior = pNew;
            }
            pSub = (*pSub).pPrior;
        }
        sqlite3DbFree(db, aCsrMap as *mut ::core::ffi::c_void);
        if (*db).mallocFailed != 0 {
            sqlite3SrcItemAttachSubquery(
                pParse,
                pSubitem,
                pSub1,
                0 as ::core::ffi::c_int,
            );
            return 1 as ::core::ffi::c_int;
        }
        if !(*pSubitem).pSTab.is_null() {
            let mut pTabToDel: *mut Table = (*pSubitem).pSTab;
            if (*pTabToDel).nTabRef == 1 as u32_0 {
                let mut pToplevel: *mut Parse = if !(*pParse).pToplevel.is_null() {
                    (*pParse).pToplevel
                } else {
                    pParse
                };
                sqlite3ParserAddCleanup(
                    pToplevel,
                    Some(
                        sqlite3DeleteTableGeneric
                            as unsafe extern "C" fn(
                                *mut sqlite3,
                                *mut ::core::ffi::c_void,
                            ) -> (),
                    ),
                    pTabToDel as *mut ::core::ffi::c_void,
                );
            } else {
                (*pTabToDel).nTabRef = (*pTabToDel).nTabRef.wrapping_sub(1);
            }
            (*pSubitem).pSTab = ::core::ptr::null_mut::<Table>();
        }
        pSub = pSub1;
        pParent = p;
        while !pParent.is_null() {
            let mut nSubSrc: ::core::ffi::c_int = 0;
            let mut jointype: u8_0 = (*pSubitem).fg.jointype;
            pSubSrc = (*pSub).pSrc;
            nSubSrc = (*pSubSrc).nSrc;
            pSrc = (*pParent).pSrc;
            if nSubSrc > 1 as ::core::ffi::c_int {
                pSrc = sqlite3SrcListEnlarge(
                    pParse,
                    pSrc,
                    nSubSrc - 1 as ::core::ffi::c_int,
                    iFrom + 1 as ::core::ffi::c_int,
                );
                if pSrc.is_null() {
                    break;
                }
                (*pParent).pSrc = pSrc;
                pSubitem = (&raw mut (*pSrc).a as *mut SrcItem).offset(iFrom as isize)
                    as *mut SrcItem;
            }
            iNewParent = (*(&raw mut (*pSubSrc).a as *mut SrcItem)
                .offset(0 as ::core::ffi::c_int as isize))
                .iCursor;
            i = 0 as ::core::ffi::c_int;
            while i < nSubSrc {
                let mut pItem: *mut SrcItem = (&raw mut (*pSrc).a as *mut SrcItem)
                    .offset((i + iFrom) as isize) as *mut SrcItem;
                if (*pItem).fg.isUsing() != 0 {
                    sqlite3IdListDelete(db, (*pItem).u3.pUsing);
                }
                *pItem = *(&raw mut (*pSubSrc).a as *mut SrcItem).offset(i as isize);
                (*pItem).fg.jointype = ((*pItem).fg.jointype as ::core::ffi::c_int
                    | jointype as ::core::ffi::c_int & JT_LTORJ) as u8_0;
                memset(
                    (&raw mut (*pSubSrc).a as *mut SrcItem).offset(i as isize)
                        as *mut SrcItem as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<SrcItem>() as size_t,
                );
                i += 1;
            }
            (*pSubitem).fg.jointype = ((*pSubitem).fg.jointype as ::core::ffi::c_int
                | jointype as ::core::ffi::c_int) as u8_0;
            if !(*pSub).pOrderBy.is_null()
                && (*pParent).selFlags & SF_NoopOrderBy as u32_0 == 0 as u32_0
            {
                let mut pOrderBy_0: *mut ExprList = (*pSub).pOrderBy;
                i = 0 as ::core::ffi::c_int;
                while i < (*pOrderBy_0).nExpr {
                    (*(&raw mut (*pOrderBy_0).a as *mut ExprList_item)
                        .offset(i as isize))
                        .u
                        .x
                        .iOrderByCol = 0 as u16_0;
                    i += 1;
                }
                (*pParent).pOrderBy = pOrderBy_0;
                (*pSub).pOrderBy = ::core::ptr::null_mut::<ExprList>();
            }
            pWhere = (*pSub).pWhere;
            (*pSub).pWhere = ::core::ptr::null_mut::<Expr>();
            if isOuterJoin > 0 as ::core::ffi::c_int {
                sqlite3SetJoinExpr(pWhere, iNewParent, EP_OuterON as u32_0);
            }
            if !pWhere.is_null() {
                if !(*pParent).pWhere.is_null() {
                    (*pParent).pWhere = sqlite3PExpr(
                        pParse,
                        TK_AND,
                        pWhere,
                        (*pParent).pWhere,
                    );
                } else {
                    (*pParent).pWhere = pWhere;
                }
            }
            if (*db).mallocFailed as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                let mut x: SubstContext = SubstContext {
                    pParse: ::core::ptr::null_mut::<Parse>(),
                    iTable: 0,
                    iNewTable: 0,
                    isOuterJoin: 0,
                    nSelDepth: 0,
                    pEList: ::core::ptr::null_mut::<ExprList>(),
                    pCList: ::core::ptr::null_mut::<ExprList>()

                };
                x.pParse = pParse;
                x.iTable = iParent;
                x.iNewTable = iNewParent;
                x.isOuterJoin = isOuterJoin;
                x.nSelDepth = 0 as ::core::ffi::c_int;
                x.pEList = (*pSub).pEList;
                x.pCList = findLeftmostExprlist(pSub);
                substSelect(&raw mut x, pParent, 0 as ::core::ffi::c_int);
            }
            (*pParent).selFlags = ((*pParent).selFlags as ::core::ffi::c_uint
                | ((*pSub).selFlags & SF_Compound as u32_0) as ::core::ffi::c_uint)
                as u32_0;
            if !(*pSub).pLimit.is_null() {
                (*pParent).pLimit = (*pSub).pLimit;
                (*pSub).pLimit = ::core::ptr::null_mut::<Expr>();
            }
            i = 0 as ::core::ffi::c_int;
            while i < nSubSrc {
                recomputeColumnsUsed(
                    pParent,
                    (&raw mut (*pSrc).a as *mut SrcItem).offset((i + iFrom) as isize)
                        as *mut SrcItem,
                );
                i += 1;
            }
            pParent = (*pParent).pPrior;
            pSub = (*pSub).pPrior;
        }
        sqlite3AggInfoPersistWalkerInit(&raw mut w, pParse);
        sqlite3WalkSelect(&raw mut w, pSub1);
        sqlite3SelectDelete(db, pSub1);
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn constInsert(
    mut pConst: *mut WhereConst,
    mut pColumn: *mut Expr,
    mut pValue: *mut Expr,
    mut pExpr: *mut Expr,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        if (*pColumn).flags & 0x20 as ::core::ffi::c_int as u32_0 != 0 as u32_0 {
            return;
        }
        if sqlite3ExprAffinity(pValue) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            return;
        }
        if sqlite3IsBinary(sqlite3ExprCompareCollSeq((*pConst).pParse, pExpr)) == 0 {
            return;
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*pConst).nConst {
            let mut pE2: *const Expr = *(*pConst)
                .apExpr
                .offset((i * 2 as ::core::ffi::c_int) as isize);
            if (*pE2).iTable == (*pColumn).iTable
                && (*pE2).iColumn as ::core::ffi::c_int
                    == (*pColumn).iColumn as ::core::ffi::c_int
            {
                return;
            }
            i += 1;
        }
        if sqlite3ExprAffinity(pColumn) as ::core::ffi::c_int <= SQLITE_AFF_BLOB {
            (*pConst).bHasAffBlob = 1 as ::core::ffi::c_int;
        }
        (*pConst).nConst += 1;
        (*pConst).apExpr = sqlite3DbReallocOrFree(
            (*(*pConst).pParse).db,
            (*pConst).apExpr as *mut ::core::ffi::c_void,
            (((*pConst).nConst * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<*mut Expr>() as usize) as u64_0,
        ) as *mut *mut Expr;
        if (*pConst).apExpr.is_null() {
            (*pConst).nConst = 0 as ::core::ffi::c_int;
        } else {
            let ref mut c2rust_fresh19 = *(*pConst)
                .apExpr
                .offset(
                    ((*pConst).nConst * 2 as ::core::ffi::c_int
                        - 2 as ::core::ffi::c_int) as isize,
                );
            *c2rust_fresh19 = pColumn;
            let ref mut c2rust_fresh20 = *(*pConst)
                .apExpr
                .offset(
                    ((*pConst).nConst * 2 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as isize,
                );
            *c2rust_fresh20 = pValue;
        };
    }
}
unsafe extern "C" fn findConstInWhere(
    mut pConst: *mut WhereConst,
    mut pExpr: *mut Expr,
) {
    unsafe {
        let mut pRight: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut pLeft: *mut Expr = ::core::ptr::null_mut::<Expr>();
        if pExpr.is_null() {
            return;
        }
        if (*pExpr).flags & (*pConst).mExcludeOn != 0 as u32_0 {
            return;
        }
        if (*pExpr).op as ::core::ffi::c_int == TK_AND {
            findConstInWhere(pConst, (*pExpr).pRight);
            findConstInWhere(pConst, (*pExpr).pLeft);
            return;
        }
        if (*pExpr).op as ::core::ffi::c_int != TK_EQ {
            return;
        }
        pRight = (*pExpr).pRight;
        pLeft = (*pExpr).pLeft;
        if (*pRight).op as ::core::ffi::c_int == TK_COLUMN
            && sqlite3ExprIsConstant((*pConst).pParse, pLeft) != 0
        {
            constInsert(pConst, pRight, pLeft, pExpr);
        }
        if (*pLeft).op as ::core::ffi::c_int == TK_COLUMN
            && sqlite3ExprIsConstant((*pConst).pParse, pRight) != 0
        {
            constInsert(pConst, pLeft, pRight, pExpr);
        }
    }
}
unsafe extern "C" fn propagateConstantExprRewriteOne(
    mut pConst: *mut WhereConst,
    mut pExpr: *mut Expr,
    mut bIgnoreAffBlob: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        if *(*pConst).pOomFault.offset(0 as ::core::ffi::c_int as isize) != 0 {
            return WRC_Prune;
        }
        if (*pExpr).op as ::core::ffi::c_int != TK_COLUMN {
            return WRC_Continue;
        }
        if (*pExpr).flags & (0x20 as u32_0 | (*pConst).mExcludeOn) != 0 as u32_0 {
            return WRC_Continue;
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*pConst).nConst {
            let mut pColumn: *mut Expr = *(*pConst)
                .apExpr
                .offset((i * 2 as ::core::ffi::c_int) as isize);
            if !(pColumn == pExpr) {
                if !((*pColumn).iTable != (*pExpr).iTable) {
                    if !((*pColumn).iColumn as ::core::ffi::c_int
                        != (*pExpr).iColumn as ::core::ffi::c_int)
                    {
                        if bIgnoreAffBlob != 0
                            && sqlite3ExprAffinity(pColumn) as ::core::ffi::c_int
                                <= SQLITE_AFF_BLOB
                        {
                            break;
                        }
                        (*pConst).nChng += 1;
                        (*pExpr).flags = ((*pExpr).flags as ::core::ffi::c_uint
                            & !(0x800000 as ::core::ffi::c_int as u32_0)
                                as ::core::ffi::c_uint) as u32_0;
                        (*pExpr).flags = ((*pExpr).flags as ::core::ffi::c_uint
                            | 0x20 as ::core::ffi::c_int as u32_0 as ::core::ffi::c_uint)
                            as u32_0;
                        (*pExpr).pLeft = sqlite3ExprDup(
                            (*(*pConst).pParse).db,
                            *(*pConst)
                                .apExpr
                                .offset(
                                    (i * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                        as isize,
                                ),
                            0 as ::core::ffi::c_int,
                        );
                        if (*(*(*pConst).pParse).db).mallocFailed != 0 {
                            return WRC_Prune;
                        }
                        break;
                    }
                }
            }
            i += 1;
        }
        return WRC_Prune;
    }
}
unsafe extern "C" fn propagateConstantExprRewrite(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pConst: *mut WhereConst = (*pWalker).u.pConst as *mut WhereConst;
        if (*pConst).bHasAffBlob != 0 {
            if (*pExpr).op as ::core::ffi::c_int >= TK_EQ
                && (*pExpr).op as ::core::ffi::c_int <= TK_GE
                || (*pExpr).op as ::core::ffi::c_int == TK_IS
            {
                propagateConstantExprRewriteOne(
                    pConst,
                    (*pExpr).pLeft,
                    0 as ::core::ffi::c_int,
                );
                if *(*pConst).pOomFault.offset(0 as ::core::ffi::c_int as isize) != 0 {
                    return WRC_Prune;
                }
                if sqlite3ExprAffinity((*pExpr).pLeft) as ::core::ffi::c_int
                    != SQLITE_AFF_TEXT
                {
                    propagateConstantExprRewriteOne(
                        pConst,
                        (*pExpr).pRight,
                        0 as ::core::ffi::c_int,
                    );
                }
            }
        }
        return propagateConstantExprRewriteOne(pConst, pExpr, (*pConst).bHasAffBlob);
    }
}
unsafe extern "C" fn propagateConstants(
    mut pParse: *mut Parse,
    mut p: *mut Select,
) -> ::core::ffi::c_int {
    unsafe {
        let mut x: WhereConst = WhereConst {
            pParse: ::core::ptr::null_mut::<Parse>(),
            pOomFault: ::core::ptr::null_mut::<u8_0>(),
            nConst: 0,
            nChng: 0,
            bHasAffBlob: 0,
            mExcludeOn: 0,
            apExpr: ::core::ptr::null_mut::<*mut Expr>()

        };
        let mut w: Walker = Walker {
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

        };
        let mut nChng: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        x.pParse = pParse;
        x.pOomFault = &raw mut (*(*pParse).db).mallocFailed;
        loop {
            x.nConst = 0 as ::core::ffi::c_int;
            x.nChng = 0 as ::core::ffi::c_int;
            x.apExpr = ::core::ptr::null_mut::<*mut Expr>();
            x.bHasAffBlob = 0 as ::core::ffi::c_int;
            if !(*p).pSrc.is_null() && (*(*p).pSrc).nSrc > 0 as ::core::ffi::c_int
                && (*(&raw mut (*(*p).pSrc).a as *mut SrcItem)
                    .offset(0 as ::core::ffi::c_int as isize))
                    .fg
                    .jointype as ::core::ffi::c_int & JT_LTORJ != 0 as ::core::ffi::c_int
            {
                x.mExcludeOn = (EP_InnerON | EP_OuterON) as u32_0;
            } else {
                x.mExcludeOn = EP_OuterON as u32_0;
            }
            findConstInWhere(&raw mut x, (*p).pWhere);
            if x.nConst != 0 {
                memset(
                    &raw mut w as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<Walker>() as size_t,
                );
                w.pParse = pParse;
                w.xExprCallback = Some(
                    propagateConstantExprRewrite
                        as unsafe extern "C" fn(
                            *mut Walker,
                            *mut Expr,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut Walker,
                            *mut Expr,
                        ) -> ::core::ffi::c_int,
                    >;
                w.xSelectCallback = Some(
                    sqlite3SelectWalkNoop
                        as unsafe extern "C" fn(
                            *mut Walker,
                            *mut Select,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut Walker,
                            *mut Select,
                        ) -> ::core::ffi::c_int,
                    >;
                w.xSelectCallback2 = None;
                w.walkerDepth = 0 as ::core::ffi::c_int;
                w.u.pConst = &raw mut x as *mut WhereConst;
                sqlite3WalkExpr(&raw mut w, (*p).pWhere);
                sqlite3DbFree((*x.pParse).db, x.apExpr as *mut ::core::ffi::c_void);
                nChng += x.nChng;
            }
            if !(x.nChng != 0) {
                break;
            }
        }
        return nChng;
    }
}
unsafe extern "C" fn pushDownWindowCheck(
    mut pParse: *mut Parse,
    mut pSubq: *mut Select,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3ExprIsConstantOrGroupBy(
            pParse,
            pExpr,
            (*(*pSubq).pWin).pPartition,
        );
    }
}
unsafe extern "C" fn pushDownWhereTerms(
    mut pParse: *mut Parse,
    mut pSubq: *mut Select,
    mut pWhere: *mut Expr,
    mut pSrcList: *mut SrcList,
    mut iSrc: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pNew: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut pSrc: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
        let mut nChng: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        pSrc = (&raw mut (*pSrcList).a as *mut SrcItem).offset(iSrc as isize)
            as *mut SrcItem;
        if pWhere.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if (*pSubq).selFlags & (SF_Recursive | SF_MultiPart) as u32_0 != 0 {
            return 0 as ::core::ffi::c_int;
        }
        if (*pSrc).fg.jointype as ::core::ffi::c_int & (JT_LTORJ | JT_RIGHT) != 0 {
            return 0 as ::core::ffi::c_int;
        }
        if !(*pSubq).pPrior.is_null() {
            let mut pSel: *mut Select = ::core::ptr::null_mut::<Select>();
            let mut notUnionAll: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            pSel = pSubq;
            while !pSel.is_null() {
                let mut op: u8_0 = (*pSel).op;
                if op as ::core::ffi::c_int != TK_ALL
                    && op as ::core::ffi::c_int != TK_SELECT
                {
                    notUnionAll = 1 as ::core::ffi::c_int;
                }
                if !(*pSel).pWin.is_null() {
                    return 0 as ::core::ffi::c_int;
                }
                pSel = (*pSel).pPrior;
            }
            if notUnionAll != 0 {
                pSel = pSubq;
                while !pSel.is_null() {
                    let mut ii: ::core::ffi::c_int = 0;
                    let mut pList: *const ExprList = (*pSel).pEList;
                    ii = 0 as ::core::ffi::c_int;
                    while ii < (*pList).nExpr {
                        let mut pColl: *mut CollSeq = sqlite3ExprCollSeq(
                            pParse,
                            (*(&raw const (*pList).a as *const ExprList_item)
                                .offset(ii as isize))
                                .pExpr,
                        );
                        if sqlite3IsBinary(pColl) == 0 {
                            return 0 as ::core::ffi::c_int;
                        }
                        ii += 1;
                    }
                    pSel = (*pSel).pPrior;
                }
            }
        } else if !(*pSubq).pWin.is_null() && (*(*pSubq).pWin).pPartition.is_null() {
            return 0 as ::core::ffi::c_int
        }
        if !(*pSubq).pLimit.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        while (*pWhere).op as ::core::ffi::c_int == TK_AND {
            nChng += pushDownWhereTerms(pParse, pSubq, (*pWhere).pRight, pSrcList, iSrc);
            pWhere = (*pWhere).pLeft;
        }
        if sqlite3ExprIsSingleTableConstraint(
            pWhere,
            pSrcList,
            iSrc,
            1 as ::core::ffi::c_int,
        ) != 0
        {
            nChng += 1;
            (*pSubq).selFlags = ((*pSubq).selFlags as ::core::ffi::c_uint
                | SF_PushDown as ::core::ffi::c_uint) as u32_0;
            while !pSubq.is_null() {
                let mut x: SubstContext = SubstContext {
                    pParse: ::core::ptr::null_mut::<Parse>(),
                    iTable: 0,
                    iNewTable: 0,
                    isOuterJoin: 0,
                    nSelDepth: 0,
                    pEList: ::core::ptr::null_mut::<ExprList>(),
                    pCList: ::core::ptr::null_mut::<ExprList>()

                };
                pNew = sqlite3ExprDup((*pParse).db, pWhere, 0 as ::core::ffi::c_int);
                unsetJoinExpr(pNew, -1 as ::core::ffi::c_int, 1 as ::core::ffi::c_int);
                x.pParse = pParse;
                x.iTable = (*pSrc).iCursor;
                x.iNewTable = (*pSrc).iCursor;
                x.isOuterJoin = 0 as ::core::ffi::c_int;
                x.nSelDepth = 0 as ::core::ffi::c_int;
                x.pEList = (*pSubq).pEList;
                x.pCList = findLeftmostExprlist(pSubq);
                pNew = substExpr(&raw mut x, pNew);
                if !(*pSubq).pWin.is_null()
                    && 0 as ::core::ffi::c_int
                        == pushDownWindowCheck(pParse, pSubq, pNew)
                {
                    sqlite3ExprDelete((*pParse).db, pNew);
                    nChng -= 1;
                    break;
                } else {
                    if (*pSubq).selFlags & SF_Aggregate as u32_0 != 0 {
                        (*pSubq).pHaving = sqlite3ExprAnd(
                            pParse,
                            (*pSubq).pHaving,
                            pNew,
                        );
                    } else {
                        (*pSubq).pWhere = sqlite3ExprAnd(pParse, (*pSubq).pWhere, pNew);
                    }
                    pSubq = (*pSubq).pPrior;
                }
            }
        }
        return nChng;
    }
}
unsafe extern "C" fn disableUnusedSubqueryResultColumns(
    mut pItem: *mut SrcItem,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nCol: ::core::ffi::c_int = 0;
        let mut pSub: *mut Select = ::core::ptr::null_mut::<Select>();
        let mut pX: *mut Select = ::core::ptr::null_mut::<Select>();
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut j: ::core::ffi::c_int = 0;
        let mut nChng: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut colUsed: Bitmask = 0;
        if (*pItem).fg.isCorrelated() as ::core::ffi::c_int != 0
            || (*pItem).fg.isCte() as ::core::ffi::c_int != 0
        {
            return 0 as ::core::ffi::c_int;
        }
        pTab = (*pItem).pSTab;
        pSub = (*(*pItem).u4.pSubq).pSelect;
        pX = pSub;
        while !pX.is_null() {
            if (*pX).selFlags & (SF_Distinct | SF_Aggregate) as u32_0 != 0 as u32_0 {
                return 0 as ::core::ffi::c_int;
            }
            if !(*pX).pPrior.is_null() && (*pX).op as ::core::ffi::c_int != TK_ALL {
                return 0 as ::core::ffi::c_int;
            }
            if !(*pX).pWin.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            pX = (*pX).pPrior;
        }
        colUsed = (*pItem).colUsed;
        if !(*pSub).pOrderBy.is_null() {
            let mut pList: *mut ExprList = (*pSub).pOrderBy;
            j = 0 as ::core::ffi::c_int;
            while j < (*pList).nExpr {
                let mut iCol: u16_0 = (*(&raw mut (*pList).a as *mut ExprList_item)
                    .offset(j as isize))
                    .u
                    .x
                    .iOrderByCol;
                if iCol as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    iCol = iCol.wrapping_sub(1);
                    colUsed
                        |= ((1 as ::core::ffi::c_int as Bitmask)
                            << (if iCol as ::core::ffi::c_int >= BMS {
                                BMS - 1 as ::core::ffi::c_int
                            } else {
                                iCol as ::core::ffi::c_int
                            })) as ::core::ffi::c_ulonglong;
                }
                j += 1;
            }
        }
        nCol = (*pTab).nCol as ::core::ffi::c_int;
        j = 0 as ::core::ffi::c_int;
        while j < nCol {
            let mut m: Bitmask = if j < BMS - 1 as ::core::ffi::c_int {
                (1 as ::core::ffi::c_int as Bitmask) << j
            } else {
                TOPBIT
            };
            if !(m & colUsed != 0 as ::core::ffi::c_ulonglong) {
                pX = pSub;
                while !pX.is_null() {
                    let mut pY: *mut Expr = (*(&raw mut (*(*pX).pEList).a
                        as *mut ExprList_item)
                        .offset(j as isize))
                        .pExpr;
                    if !((*pY).op as ::core::ffi::c_int == TK_NULL) {
                        (*pY).op = TK_NULL as u8_0;
                        (*pY).flags = ((*pY).flags as ::core::ffi::c_uint
                            & !((0x2000 as ::core::ffi::c_int
                                | 0x80000 as ::core::ffi::c_int) as u32_0)
                                as ::core::ffi::c_uint) as u32_0;
                        (*pX).selFlags = ((*pX).selFlags as ::core::ffi::c_uint
                            | SF_PushDown as ::core::ffi::c_uint) as u32_0;
                        nChng += 1;
                    }
                    pX = (*pX).pPrior;
                }
            }
            j += 1;
        }
        return nChng;
    }
}
unsafe extern "C" fn minMaxQuery(
    mut db: *mut sqlite3,
    mut pFunc: *mut Expr,
    mut ppMinMax: *mut *mut ExprList,
) -> u8_0 {
    unsafe {
        let mut eRet: ::core::ffi::c_int = WHERE_ORDERBY_NORMAL;
        let mut pEList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        let mut zFunc: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut pOrderBy: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        let mut sortFlags: u8_0 = 0 as u8_0;
        pEList = (*pFunc).x.pList;
        if pEList.is_null() || (*pEList).nExpr != 1 as ::core::ffi::c_int
            || (*pFunc).flags & 0x1000000 as ::core::ffi::c_int as u32_0 != 0 as u32_0
            || (*db).dbOptFlags & 0x10000 as u32_0 != 0 as u32_0
        {
            return eRet as u8_0;
        }
        zFunc = (*pFunc).u.zToken;
        if sqlite3StrICmp(zFunc, b"min\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            eRet = WHERE_ORDERBY_MIN;
            if sqlite3ExprCanBeNull(
                (*(&raw mut (*pEList).a as *mut ExprList_item)
                    .offset(0 as ::core::ffi::c_int as isize))
                    .pExpr,
            ) != 0
            {
                sortFlags = KEYINFO_ORDER_BIGNULL as u8_0;
            }
        } else if sqlite3StrICmp(zFunc, b"max\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            eRet = WHERE_ORDERBY_MAX;
            sortFlags = KEYINFO_ORDER_DESC as u8_0;
        } else {
            return eRet as u8_0
        }
        pOrderBy = sqlite3ExprListDup(db, pEList, 0 as ::core::ffi::c_int);
        *ppMinMax = pOrderBy;
        if !pOrderBy.is_null() {
            (*(&raw mut (*pOrderBy).a as *mut ExprList_item)
                .offset(0 as ::core::ffi::c_int as isize))
                .fg
                .sortFlags = sortFlags;
        }
        return eRet as u8_0;
    }
}
unsafe extern "C" fn isSimpleCount(
    mut p: *mut Select,
    mut pAggInfo: *mut AggInfo,
) -> *mut Table {
    unsafe {
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut pExpr: *mut Expr = ::core::ptr::null_mut::<Expr>();
        if !(*p).pWhere.is_null() || (*(*p).pEList).nExpr != 1 as ::core::ffi::c_int
            || (*(*p).pSrc).nSrc != 1 as ::core::ffi::c_int
            || (*(&raw mut (*(*p).pSrc).a as *mut SrcItem)
                .offset(0 as ::core::ffi::c_int as isize))
                .fg
                .isSubquery() as ::core::ffi::c_int != 0
            || (*pAggInfo).nFunc != 1 as ::core::ffi::c_int || !(*p).pHaving.is_null()
        {
            return ::core::ptr::null_mut::<Table>();
        }
        pTab = (*(&raw mut (*(*p).pSrc).a as *mut SrcItem)
            .offset(0 as ::core::ffi::c_int as isize))
            .pSTab;
        if !((*pTab).eTabType as ::core::ffi::c_int == TABTYP_NORM) {
            return ::core::ptr::null_mut::<Table>();
        }
        pExpr = (*(&raw mut (*(*p).pEList).a as *mut ExprList_item)
            .offset(0 as ::core::ffi::c_int as isize))
            .pExpr;
        if (*pExpr).op as ::core::ffi::c_int != TK_AGG_FUNCTION {
            return ::core::ptr::null_mut::<Table>();
        }
        if (*pExpr).pAggInfo != pAggInfo {
            return ::core::ptr::null_mut::<Table>();
        }
        if (*(*(*pAggInfo).aFunc.offset(0 as ::core::ffi::c_int as isize)).pFunc)
            .funcFlags & SQLITE_FUNC_COUNT as u32_0 == 0 as u32_0
        {
            return ::core::ptr::null_mut::<Table>();
        }
        if (*pExpr).flags
            & (0x4 as ::core::ffi::c_int | 0x1000000 as ::core::ffi::c_int) as u32_0
            != 0 as u32_0
        {
            return ::core::ptr::null_mut::<Table>();
        }
        return pTab;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3IndexedByLookup(
    mut pParse: *mut Parse,
    mut pFrom: *mut SrcItem,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pTab: *mut Table = (*pFrom).pSTab;
        let mut zIndexedBy: *mut ::core::ffi::c_char = (*pFrom).u1.zIndexedBy;
        let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
        pIdx = (*pTab).pIndex;
        while !pIdx.is_null() && sqlite3StrICmp((*pIdx).zName, zIndexedBy) != 0 {
            pIdx = (*pIdx).pNext;
        }
        if pIdx.is_null() {
            sqlite3ErrorMsg(
                pParse,
                b"no such index: %s\0".as_ptr() as *const ::core::ffi::c_char,
                zIndexedBy,
                0 as ::core::ffi::c_int,
            );
            (*pParse).set_checkSchema(1 as bft as bft);
            return SQLITE_ERROR;
        }
        (*pFrom).u2.pIBIndex = pIdx;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn convertCompoundSelectToSubquery(
    mut pWalker: *mut Walker,
    mut p: *mut Select,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut pNew: *mut Select = ::core::ptr::null_mut::<Select>();
        let mut pX: *mut Select = ::core::ptr::null_mut::<Select>();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut a: *mut ExprList_item = ::core::ptr::null_mut::<ExprList_item>();
        let mut pNewSrc: *mut SrcList = ::core::ptr::null_mut::<SrcList>();
        let mut pParse: *mut Parse = ::core::ptr::null_mut::<Parse>();
        let mut dummy: Token = Token {
            z: ::core::ptr::null::<::core::ffi::c_char>(),
            n: 0

        };
        if (*p).pPrior.is_null() {
            return WRC_Continue;
        }
        if (*p).pOrderBy.is_null() {
            return WRC_Continue;
        }
        pX = p;
        while !pX.is_null()
            && ((*pX).op as ::core::ffi::c_int == TK_ALL
                || (*pX).op as ::core::ffi::c_int == TK_SELECT)
        {
            pX = (*pX).pPrior;
        }
        if pX.is_null() {
            return WRC_Continue;
        }
        a = &raw mut (*(*p).pOrderBy).a as *mut ExprList_item as *mut ExprList_item;
        if (*a.offset(0 as ::core::ffi::c_int as isize)).u.x.iOrderByCol != 0 {
            return WRC_Continue;
        }
        i = (*(*p).pOrderBy).nExpr - 1 as ::core::ffi::c_int;
        while i >= 0 as ::core::ffi::c_int {
            if (*(*a.offset(i as isize)).pExpr).flags & EP_Collate as u32_0 != 0 {
                break;
            }
            i -= 1;
        }
        if i < 0 as ::core::ffi::c_int {
            return WRC_Continue;
        }
        pParse = (*pWalker).pParse;
        db = (*pParse).db;
        pNew = sqlite3DbMallocZero(db, ::core::mem::size_of::<Select>() as u64_0)
            as *mut Select;
        if pNew.is_null() {
            return WRC_Abort;
        }
        memset(
            &raw mut dummy as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<Token>() as size_t,
        );
        pNewSrc = sqlite3SrcListAppendFromTerm(
            pParse,
            ::core::ptr::null_mut::<SrcList>(),
            ::core::ptr::null_mut::<Token>(),
            ::core::ptr::null_mut::<Token>(),
            &raw mut dummy,
            pNew,
            ::core::ptr::null_mut::<OnOrUsing>(),
        );
        if (*pParse).nErr != 0 {
            sqlite3SrcListDelete(db, pNewSrc);
            return WRC_Abort;
        }
        *pNew = *p;
        (*p).pSrc = pNewSrc;
        (*p).pEList = sqlite3ExprListAppend(
            pParse,
            ::core::ptr::null_mut::<ExprList>(),
            sqlite3Expr(db, TK_ASTERISK, ::core::ptr::null::<::core::ffi::c_char>()),
        );
        (*p).op = TK_SELECT as u8_0;
        (*p).pWhere = ::core::ptr::null_mut::<Expr>();
        (*pNew).pGroupBy = ::core::ptr::null_mut::<ExprList>();
        (*pNew).pHaving = ::core::ptr::null_mut::<Expr>();
        (*pNew).pOrderBy = ::core::ptr::null_mut::<ExprList>();
        (*p).pPrior = ::core::ptr::null_mut::<Select>();
        (*p).pNext = ::core::ptr::null_mut::<Select>();
        (*p).pWith = ::core::ptr::null_mut::<With>();
        (*p).pWinDefn = ::core::ptr::null_mut::<Window>();
        (*p).selFlags = ((*p).selFlags as ::core::ffi::c_uint
            & !(SF_Compound as u32_0) as ::core::ffi::c_uint) as u32_0;
        (*p).selFlags = ((*p).selFlags as ::core::ffi::c_uint
            | SF_Converted as ::core::ffi::c_uint) as u32_0;
        (*(*pNew).pPrior).pNext = pNew;
        (*pNew).pLimit = ::core::ptr::null_mut::<Expr>();
        return WRC_Continue;
    }
}
unsafe extern "C" fn cannotBeFunction(
    mut pParse: *mut Parse,
    mut pFrom: *mut SrcItem,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pFrom).fg.isTabFunc() != 0 {
            sqlite3ErrorMsg(
                pParse,
                b"'%s' is not a function\0".as_ptr() as *const ::core::ffi::c_char,
                (*pFrom).zName,
            );
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn searchWith(
    mut pWith: *mut With,
    mut pItem: *mut SrcItem,
    mut ppContext: *mut *mut With,
) -> *mut Cte {
    unsafe {
        let mut zName: *const ::core::ffi::c_char = (*pItem).zName;
        let mut p: *mut With = ::core::ptr::null_mut::<With>();
        p = pWith;
        while !p.is_null() {
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i < (*p).nCte {
                if sqlite3StrICmp(
                    zName,
                    (*(&raw mut (*p).a as *mut Cte).offset(i as isize)).zName,
                ) == 0 as ::core::ffi::c_int
                {
                    *ppContext = p;
                    return (&raw mut (*p).a as *mut Cte).offset(i as isize) as *mut Cte;
                }
                i += 1;
            }
            if (*p).bView != 0 {
                break;
            }
            p = (*p).pOuter;
        }
        return ::core::ptr::null_mut::<Cte>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WithPush(
    mut pParse: *mut Parse,
    mut pWith: *mut With,
    mut bFree: u8_0,
) -> *mut With {
    unsafe {
        if !pWith.is_null() {
            if bFree != 0 {
                pWith = sqlite3ParserAddCleanup(
                    pParse,
                    Some(
                        sqlite3WithDeleteGeneric
                            as unsafe extern "C" fn(
                                *mut sqlite3,
                                *mut ::core::ffi::c_void,
                            ) -> (),
                    ),
                    pWith as *mut ::core::ffi::c_void,
                ) as *mut With;
                if pWith.is_null() {
                    return ::core::ptr::null_mut::<With>();
                }
            }
            if (*pParse).nErr == 0 as ::core::ffi::c_int {
                (*pWith).pOuter = (*pParse).pWith;
                (*pParse).pWith = pWith;
            }
        }
        return pWith;
    }
}
unsafe extern "C" fn resolveFromTermToCte(
    mut pParse: *mut Parse,
    mut pWalker: *mut Walker,
    mut pFrom: *mut SrcItem,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCte: *mut Cte = ::core::ptr::null_mut::<Cte>();
        let mut pWith: *mut With = ::core::ptr::null_mut::<With>();
        if (*pParse).pWith.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if (*pParse).nErr != 0 {
            return 0 as ::core::ffi::c_int;
        }
        if (*pFrom).fg.fixedSchema() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && !(*pFrom).u4.zDatabase.is_null()
        {
            return 0 as ::core::ffi::c_int;
        }
        if (*pFrom).fg.notCte() != 0 {
            return 0 as ::core::ffi::c_int;
        }
        pCte = searchWith((*pParse).pWith, pFrom, &raw mut pWith) as *mut Cte;
        if !pCte.is_null() {
            let mut db: *mut sqlite3 = (*pParse).db;
            let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
            let mut pEList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
            let mut pSel: *mut Select = ::core::ptr::null_mut::<Select>();
            let mut pLeft: *mut Select = ::core::ptr::null_mut::<Select>();
            let mut pRecTerm: *mut Select = ::core::ptr::null_mut::<Select>();
            let mut bMayRecursive: ::core::ffi::c_int = 0;
            let mut pSavedWith: *mut With = ::core::ptr::null_mut::<With>();
            let mut iRecTab: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
            let mut pCteUse: *mut CteUse = ::core::ptr::null_mut::<CteUse>();
            if !(*pCte).zCteErr.is_null() {
                sqlite3ErrorMsg(pParse, (*pCte).zCteErr, (*pCte).zName);
                return 2 as ::core::ffi::c_int;
            }
            if cannotBeFunction(pParse, pFrom) != 0 {
                return 2 as ::core::ffi::c_int;
            }
            pTab = sqlite3DbMallocZero(db, ::core::mem::size_of::<Table>() as u64_0)
                as *mut Table;
            if pTab.is_null() {
                return 2 as ::core::ffi::c_int;
            }
            pCteUse = (*pCte).pUse;
            if pCteUse.is_null() {
                pCteUse = sqlite3DbMallocZero(
                    db,
                    ::core::mem::size_of::<CteUse>() as u64_0,
                ) as *mut CteUse;
                (*pCte).pUse = pCteUse;
                if pCteUse.is_null()
                    || sqlite3ParserAddCleanup(
                            pParse,
                            Some(
                                sqlite3DbFree
                                    as unsafe extern "C" fn(
                                        *mut sqlite3,
                                        *mut ::core::ffi::c_void,
                                    ) -> (),
                            ),
                            pCteUse as *mut ::core::ffi::c_void,
                        )
                        .is_null()
                {
                    sqlite3DbFree(db, pTab as *mut ::core::ffi::c_void);
                    return 2 as ::core::ffi::c_int;
                }
                (*pCteUse).eM10d = (*pCte).eM10d;
            }
            (*pFrom).pSTab = pTab;
            (*pTab).nTabRef = 1 as u32_0;
            (*pTab).zName = sqlite3DbStrDup(db, (*pCte).zName);
            (*pTab).iPKey = -1 as ::core::ffi::c_int as i16_0;
            (*pTab).nRowLogEst = 200 as LogEst;
            (*pTab).tabFlags = ((*pTab).tabFlags as ::core::ffi::c_uint
                | (TF_Ephemeral | TF_NoVisibleRowid) as ::core::ffi::c_uint) as u32_0;
            sqlite3SrcItemAttachSubquery(
                pParse,
                pFrom,
                (*pCte).pSelect,
                1 as ::core::ffi::c_int,
            );
            if (*db).mallocFailed != 0 {
                return 2 as ::core::ffi::c_int;
            }
            pSel = (*(*pFrom).u4.pSubq).pSelect;
            (*pSel).selFlags = ((*pSel).selFlags as ::core::ffi::c_uint
                | SF_CopyCte as ::core::ffi::c_uint) as u32_0;
            if (*pFrom).fg.isIndexedBy() != 0 {
                sqlite3ErrorMsg(
                    pParse,
                    b"no such index: \"%s\"\0".as_ptr() as *const ::core::ffi::c_char,
                    (*pFrom).u1.zIndexedBy,
                );
                return 2 as ::core::ffi::c_int;
            }
            (*pFrom).fg.set_isCte(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*pFrom).u2.pCteUse = pCteUse;
            (*pCteUse).nUse += 1;
            pRecTerm = pSel;
            bMayRecursive = ((*pSel).op as ::core::ffi::c_int == TK_ALL
                || (*pSel).op as ::core::ffi::c_int == TK_UNION) as ::core::ffi::c_int;
            while bMayRecursive != 0
                && (*pRecTerm).op as ::core::ffi::c_int
                    == (*pSel).op as ::core::ffi::c_int
            {
                let mut i: ::core::ffi::c_int = 0;
                let mut pSrc: *mut SrcList = (*pRecTerm).pSrc;
                i = 0 as ::core::ffi::c_int;
                while i < (*pSrc).nSrc {
                    let mut pItem: *mut SrcItem = (&raw mut (*pSrc).a as *mut SrcItem)
                        .offset(i as isize) as *mut SrcItem;
                    if !(*pItem).zName.is_null() && (*pItem).fg.hadSchema() == 0
                        && (*pItem).fg.isSubquery() == 0
                        && ((*pItem).fg.fixedSchema() as ::core::ffi::c_int != 0
                            || (*pItem).u4.zDatabase.is_null())
                        && 0 as ::core::ffi::c_int
                            == sqlite3StrICmp((*pItem).zName, (*pCte).zName)
                    {
                        (*pItem).pSTab = pTab;
                        (*pTab).nTabRef = (*pTab).nTabRef.wrapping_add(1);
                        (*pItem)
                            .fg
                            .set_isRecursive(
                                1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                            );
                        if (*pRecTerm).selFlags & SF_Recursive as u32_0 != 0 {
                            sqlite3ErrorMsg(
                                pParse,
                                b"multiple references to recursive table: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                (*pCte).zName,
                            );
                            return 2 as ::core::ffi::c_int;
                        }
                        (*pRecTerm).selFlags = ((*pRecTerm).selFlags
                            as ::core::ffi::c_uint | SF_Recursive as ::core::ffi::c_uint)
                            as u32_0;
                        if iRecTab < 0 as ::core::ffi::c_int {
                            let c2rust_fresh6 = (*pParse).nTab;
                            (*pParse).nTab = (*pParse).nTab + 1;
                            iRecTab = c2rust_fresh6;
                        }
                        (*pItem).iCursor = iRecTab;
                    }
                    i += 1;
                }
                if (*pRecTerm).selFlags & SF_Recursive as u32_0 == 0 as u32_0 {
                    break;
                }
                pRecTerm = (*pRecTerm).pPrior;
            }
            (*pCte).zCteErr = b"circular reference: %s\0".as_ptr()
                as *const ::core::ffi::c_char;
            pSavedWith = (*pParse).pWith;
            (*pParse).pWith = pWith;
            if (*pSel).selFlags & SF_Recursive as u32_0 != 0 {
                let mut rc: ::core::ffi::c_int = 0;
                (*pRecTerm).pWith = (*pSel).pWith;
                rc = sqlite3WalkSelect(pWalker, pRecTerm);
                (*pRecTerm).pWith = ::core::ptr::null_mut::<With>();
                if rc != 0 {
                    (*pParse).pWith = pSavedWith;
                    return 2 as ::core::ffi::c_int;
                }
            } else if sqlite3WalkSelect(pWalker, pSel) != 0 {
                (*pParse).pWith = pSavedWith;
                return 2 as ::core::ffi::c_int;
            }
            (*pParse).pWith = pWith;
            pLeft = pSel;
            while !(*pLeft).pPrior.is_null() {
                pLeft = (*pLeft).pPrior;
            }
            pEList = (*pLeft).pEList;
            if !(*pCte).pCols.is_null() {
                if !pEList.is_null() && (*pEList).nExpr != (*(*pCte).pCols).nExpr {
                    sqlite3ErrorMsg(
                        pParse,
                        b"table %s has %d values for %d columns\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*pCte).zName,
                        (*pEList).nExpr,
                        (*(*pCte).pCols).nExpr,
                    );
                    (*pParse).pWith = pSavedWith;
                    return 2 as ::core::ffi::c_int;
                }
                pEList = (*pCte).pCols;
            }
            sqlite3ColumnsFromExprList(
                pParse,
                pEList,
                &raw mut (*pTab).nCol,
                &raw mut (*pTab).aCol,
            );
            if bMayRecursive != 0 {
                if (*pSel).selFlags & SF_Recursive as u32_0 != 0 {
                    (*pCte).zCteErr = b"multiple recursive references: %s\0".as_ptr()
                        as *const ::core::ffi::c_char;
                } else {
                    (*pCte).zCteErr = b"recursive reference in a subquery: %s\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                sqlite3WalkSelect(pWalker, pSel);
            }
            (*pCte).zCteErr = ::core::ptr::null::<::core::ffi::c_char>();
            (*pParse).pWith = pSavedWith;
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SelectPopWith(
    mut pWalker: *mut Walker,
    mut p: *mut Select,
) {
    unsafe {
        let mut pParse: *mut Parse = (*pWalker).pParse;
        if !(*pParse).pWith.is_null() && (*p).pPrior.is_null() {
            let mut pWith: *mut With = (*findRightmost(p)).pWith;
            if !pWith.is_null() {
                (*pParse).pWith = (*pWith).pOuter;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExpandSubquery(
    mut pParse: *mut Parse,
    mut pFrom: *mut SrcItem,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pSel: *mut Select = ::core::ptr::null_mut::<Select>();
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        pSel = (*(*pFrom).u4.pSubq).pSelect;
        pTab = sqlite3DbMallocZero(
            (*pParse).db,
            ::core::mem::size_of::<Table>() as u64_0,
        ) as *mut Table;
        (*pFrom).pSTab = pTab;
        if pTab.is_null() {
            return SQLITE_NOMEM;
        }
        (*pTab).nTabRef = 1 as u32_0;
        if !(*pFrom).zAlias.is_null() {
            (*pTab).zName = sqlite3DbStrDup((*pParse).db, (*pFrom).zAlias);
        } else {
            (*pTab).zName = sqlite3MPrintf(
                (*pParse).db,
                b"%!S\0".as_ptr() as *const ::core::ffi::c_char,
                pFrom,
            );
        }
        while !(*pSel).pPrior.is_null() {
            pSel = (*pSel).pPrior;
        }
        sqlite3ColumnsFromExprList(
            pParse,
            (*pSel).pEList,
            &raw mut (*pTab).nCol,
            &raw mut (*pTab).aCol,
        );
        (*pTab).iPKey = -1 as ::core::ffi::c_int as i16_0;
        (*pTab).eTabType = TABTYP_VIEW as u8_0;
        (*pTab).nRowLogEst = 200 as LogEst;
        (*pTab).tabFlags = ((*pTab).tabFlags as ::core::ffi::c_uint
            | (TF_Ephemeral | TF_NoVisibleRowid) as ::core::ffi::c_uint) as u32_0;
        return if (*pParse).nErr != 0 { SQLITE_ERROR } else { SQLITE_OK };
    }
}
unsafe extern "C" fn inAnyUsingClause(
    mut zName: *const ::core::ffi::c_char,
    mut pBase: *mut SrcItem,
    mut N: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        while N > 0 as ::core::ffi::c_int {
            N -= 1;
            pBase = pBase.offset(1);
            if (*pBase).fg.isUsing() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                continue;
            }
            if (*pBase).u3.pUsing.is_null() {
                continue;
            }
            if sqlite3IdListIndex((*pBase).u3.pUsing, zName) >= 0 as ::core::ffi::c_int {
                return 1 as ::core::ffi::c_int;
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn selectExpander(
    mut pWalker: *mut Walker,
    mut p: *mut Select,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pParse: *mut Parse = (*pWalker).pParse;
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut k: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        let mut pTabList: *mut SrcList = ::core::ptr::null_mut::<SrcList>();
        let mut pEList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        let mut pFrom: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pE: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut pRight: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut pExpr: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut selFlags: u16_0 = (*p).selFlags as u16_0;
        let mut elistFlags: u32_0 = 0 as u32_0;
        (*p).selFlags = ((*p).selFlags as ::core::ffi::c_uint
            | SF_Expanded as ::core::ffi::c_uint) as u32_0;
        if (*db).mallocFailed != 0 {
            return WRC_Abort;
        }
        if selFlags as ::core::ffi::c_int & SF_Expanded != 0 as ::core::ffi::c_int {
            return WRC_Prune;
        }
        if (*pWalker).eCode != 0 {
            (*pParse).nSelect += 1;
            (*p).selId = (*pParse).nSelect as u32_0;
        }
        pTabList = (*p).pSrc;
        pEList = (*p).pEList;
        if !(*pParse).pWith.is_null() && (*p).selFlags & SF_View as u32_0 != 0 {
            if (*p).pWith.is_null() {
                (*p).pWith = sqlite3DbMallocZero(
                    db,
                    (16 as usize)
                        .wrapping_add(
                            (1 as usize)
                                .wrapping_mul(::core::mem::size_of::<Cte>() as usize),
                        ) as u64_0,
                ) as *mut With;
                if (*p).pWith.is_null() {
                    return WRC_Abort;
                }
            }
            (*(*p).pWith).bView = 1 as ::core::ffi::c_int;
        }
        sqlite3WithPush(pParse, (*p).pWith, 0 as u8_0);
        sqlite3SrcListAssignCursors(pParse, pTabList);
        i = 0 as ::core::ffi::c_int;
        pFrom = &raw mut (*pTabList).a as *mut SrcItem;
        while i < (*pTabList).nSrc {
            let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
            if (*pFrom).pSTab.is_null() {
                if (*pFrom).zName.is_null() {
                    let mut pSel: *mut Select = ::core::ptr::null_mut::<Select>();
                    pSel = (*(*pFrom).u4.pSubq).pSelect;
                    if sqlite3WalkSelect(pWalker, pSel) != 0 {
                        return WRC_Abort;
                    }
                    if sqlite3ExpandSubquery(pParse, pFrom) != 0 {
                        return WRC_Abort;
                    }
                } else {
                    rc = resolveFromTermToCte(pParse, pWalker, pFrom);
                    if rc != 0 as ::core::ffi::c_int {
                        if rc > 1 as ::core::ffi::c_int {
                            return WRC_Abort;
                        }
                        pTab = (*pFrom).pSTab;
                    } else {
                        pTab = sqlite3LocateTableItem(pParse, 0 as u32_0, pFrom);
                        (*pFrom).pSTab = pTab;
                        if pTab.is_null() {
                            return WRC_Abort;
                        }
                        if (*pTab).nTabRef >= 0xffff as u32_0 {
                            sqlite3ErrorMsg(
                                pParse,
                                b"too many references to \"%s\": max 65535\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                (*pTab).zName,
                            );
                            (*pFrom).pSTab = ::core::ptr::null_mut::<Table>();
                            return WRC_Abort;
                        }
                        (*pTab).nTabRef = (*pTab).nTabRef.wrapping_add(1);
                        if !((*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB)
                            && cannotBeFunction(pParse, pFrom) != 0
                        {
                            return WRC_Abort;
                        }
                        if !((*pTab).eTabType as ::core::ffi::c_int == TABTYP_NORM) {
                            let mut nCol: i16_0 = 0;
                            let mut eCodeOrig: u8_0 = (*pWalker).eCode as u8_0;
                            if sqlite3ViewGetColumnNames(pParse, pTab) != 0 {
                                return WRC_Abort;
                            }
                            if (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VIEW {
                                if (*db).flags as ::core::ffi::c_ulonglong
                                    & SQLITE_EnableView as ::core::ffi::c_ulonglong
                                    == 0 as ::core::ffi::c_ulonglong
                                    && (*pTab).pSchema
                                        != (*(*db).aDb.offset(1 as ::core::ffi::c_int as isize))
                                            .pSchema
                                {
                                    sqlite3ErrorMsg(
                                        pParse,
                                        b"access to view \"%s\" prohibited\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        (*pTab).zName,
                                    );
                                }
                                sqlite3SrcItemAttachSubquery(
                                    pParse,
                                    pFrom,
                                    (*pTab).u.view.pSelect,
                                    1 as ::core::ffi::c_int,
                                );
                            } else if (*pTab).eTabType as ::core::ffi::c_int
                                == 1 as ::core::ffi::c_int
                                && (*pFrom).fg.fromDDL() as ::core::ffi::c_int != 0
                                && !(*pTab).u.vtab.p.is_null()
                                && (*(*pTab).u.vtab.p).eVtabRisk as ::core::ffi::c_int
                                    > ((*db).flags as ::core::ffi::c_ulonglong
                                        & SQLITE_TrustedSchema as ::core::ffi::c_ulonglong
                                        != 0 as ::core::ffi::c_ulonglong) as ::core::ffi::c_int
                            {
                                sqlite3ErrorMsg(
                                    pParse,
                                    b"unsafe use of virtual table \"%s\"\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    (*pTab).zName,
                                );
                            }
                            nCol = (*pTab).nCol;
                            (*pTab).nCol = -1 as ::core::ffi::c_int as i16_0;
                            (*pWalker).eCode = 1 as u16_0;
                            if (*pFrom).fg.isSubquery() != 0 {
                                sqlite3WalkSelect(pWalker, (*(*pFrom).u4.pSubq).pSelect);
                            }
                            (*pWalker).eCode = eCodeOrig as u16_0;
                            (*pTab).nCol = nCol;
                        }
                    }
                }
                if (*pFrom).fg.isIndexedBy() as ::core::ffi::c_int != 0
                    && sqlite3IndexedByLookup(pParse, pFrom) != 0
                {
                    return WRC_Abort;
                }
            }
            i += 1;
            pFrom = pFrom.offset(1);
        }
        if (*pParse).nErr != 0 || sqlite3ProcessJoin(pParse, p) != 0 {
            return WRC_Abort;
        }
        k = 0 as ::core::ffi::c_int;
        while k < (*pEList).nExpr {
            pE = (*(&raw mut (*pEList).a as *mut ExprList_item).offset(k as isize))
                .pExpr;
            if (*pE).op as ::core::ffi::c_int == TK_ASTERISK {
                break;
            }
            if (*pE).op as ::core::ffi::c_int == TK_DOT
                && (*(*pE).pRight).op as ::core::ffi::c_int == TK_ASTERISK
            {
                break;
            }
            elistFlags = (elistFlags as ::core::ffi::c_uint
                | (*pE).flags as ::core::ffi::c_uint) as u32_0;
            k += 1;
        }
        if k < (*pEList).nExpr {
            let mut a: *mut ExprList_item = &raw mut (*pEList).a as *mut ExprList_item;
            let mut pNew: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
            let mut flags: ::core::ffi::c_int = (*(*pParse).db).flags
                as ::core::ffi::c_int;
            let mut longNames: ::core::ffi::c_int = (flags & SQLITE_FullColNames
                != 0 as ::core::ffi::c_int
                && flags & SQLITE_ShortColNames == 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            k = 0 as ::core::ffi::c_int;
            while k < (*pEList).nExpr {
                pE = (*a.offset(k as isize)).pExpr;
                elistFlags = (elistFlags as ::core::ffi::c_uint
                    | (*pE).flags as ::core::ffi::c_uint) as u32_0;
                pRight = (*pE).pRight;
                if (*pE).op as ::core::ffi::c_int != TK_ASTERISK
                    && ((*pE).op as ::core::ffi::c_int != TK_DOT
                        || (*pRight).op as ::core::ffi::c_int != TK_ASTERISK)
                {
                    pNew = sqlite3ExprListAppend(
                        pParse,
                        pNew,
                        (*a.offset(k as isize)).pExpr,
                    );
                    if !pNew.is_null() {
                        let ref mut c2rust_fresh0 = (*(&raw mut (*pNew).a
                            as *mut ExprList_item)
                            .offset(((*pNew).nExpr - 1 as ::core::ffi::c_int) as isize))
                            .zEName;
                        *c2rust_fresh0 = (*a.offset(k as isize)).zEName;
                        let ref mut c2rust_fresh1 = (*(&raw mut (*pNew).a
                            as *mut ExprList_item)
                            .offset(((*pNew).nExpr - 1 as ::core::ffi::c_int) as isize))
                            .fg;
                        (*c2rust_fresh1)
                            .set_eEName(
                                (*a.offset(k as isize)).fg.eEName() as ::core::ffi::c_uint,
                            );
                        let ref mut c2rust_fresh2 = (*a.offset(k as isize)).zEName;
                        *c2rust_fresh2 = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    }
                    let ref mut c2rust_fresh3 = (*a.offset(k as isize)).pExpr;
                    *c2rust_fresh3 = ::core::ptr::null_mut::<Expr>();
                } else {
                    let mut tableSeen: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut zTName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                        ::core::ffi::c_char,
                    >();
                    let mut iErrOfst: ::core::ffi::c_int = 0;
                    if (*pE).op as ::core::ffi::c_int == TK_DOT {
                        zTName = (*(*pE).pLeft).u.zToken;
                        iErrOfst = (*(*pE).pRight).w.iOfst;
                    } else {
                        iErrOfst = (*pE).w.iOfst;
                    }
                    let mut c2rust_current_block_197: u64;
                    i = 0 as ::core::ffi::c_int;
                    pFrom = &raw mut (*pTabList).a as *mut SrcItem;
                    while i < (*pTabList).nSrc {
                        let mut nAdd: ::core::ffi::c_int = 0;
                        let mut pTab_0: *mut Table = (*pFrom).pSTab;
                        let mut pNestedFrom: *mut ExprList = ::core::ptr::null_mut::<
                            ExprList,
                        >();
                        let mut zTabName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                            ::core::ffi::c_char,
                        >();
                        let mut zSchemaName: *const ::core::ffi::c_char = ::core::ptr::null::<
                            ::core::ffi::c_char,
                        >();
                        let mut iDb: ::core::ffi::c_int = 0;
                        let mut pUsing: *mut IdList = ::core::ptr::null_mut::<IdList>();
                        zTabName = (*pFrom).zAlias;
                        if zTabName.is_null() {
                            zTabName = (*pTab_0).zName;
                        }
                        if (*db).mallocFailed != 0 {
                            break;
                        }
                        if (*pFrom).fg.isNestedFrom() != 0 {
                            pNestedFrom = (*(*(*pFrom).u4.pSubq).pSelect).pEList;
                            c2rust_current_block_197 = 17623951255125923504;
                        } else if !zTName.is_null()
                            && sqlite3StrICmp(zTName, zTabName)
                                != 0 as ::core::ffi::c_int
                        {
                            c2rust_current_block_197 = 1428307939028130064;
                        } else {
                            pNestedFrom = ::core::ptr::null_mut::<ExprList>();
                            iDb = sqlite3SchemaToIndex(db, (*pTab_0).pSchema);
                            zSchemaName = if iDb >= 0 as ::core::ffi::c_int {
                                (*(*db).aDb.offset(iDb as isize)).zDbSName
                                    as *const ::core::ffi::c_char
                            } else {
                                b"*\0".as_ptr() as *const ::core::ffi::c_char
                            };
                            c2rust_current_block_197 = 17623951255125923504;
                        }
                        match c2rust_current_block_197 {
                            17623951255125923504 => {
                                if (i + 1 as ::core::ffi::c_int) < (*pTabList).nSrc
                                    && (*pFrom.offset(1 as ::core::ffi::c_int as isize))
                                        .fg
                                        .isUsing() as ::core::ffi::c_int != 0
                                    && selFlags as ::core::ffi::c_int & SF_NestedFrom
                                        != 0 as ::core::ffi::c_int
                                {
                                    let mut ii: ::core::ffi::c_int = 0;
                                    pUsing = (*pFrom.offset(1 as ::core::ffi::c_int as isize))
                                        .u3
                                        .pUsing;
                                    ii = 0 as ::core::ffi::c_int;
                                    while ii < (*pUsing).nId {
                                        let mut zUName: *const ::core::ffi::c_char = (*(&raw mut (*pUsing)
                                            .a as *mut IdList_item)
                                            .offset(ii as isize))
                                            .zName;
                                        pRight = sqlite3Expr(db, TK_ID, zUName);
                                        sqlite3ExprSetErrorOffset(pRight, iErrOfst);
                                        pNew = sqlite3ExprListAppend(pParse, pNew, pRight);
                                        if !pNew.is_null() {
                                            let mut pX: *mut ExprList_item = (&raw mut (*pNew).a
                                                as *mut ExprList_item)
                                                .offset(((*pNew).nExpr - 1 as ::core::ffi::c_int) as isize)
                                                as *mut ExprList_item;
                                            (*pX).zEName = sqlite3MPrintf(
                                                db,
                                                b"..%s\0".as_ptr() as *const ::core::ffi::c_char,
                                                zUName,
                                            );
                                            (*pX)
                                                .fg
                                                .set_eEName(
                                                    ENAME_TAB as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                                );
                                            (*pX)
                                                .fg
                                                .set_bUsingTerm(
                                                    1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                                );
                                        }
                                        ii += 1;
                                    }
                                } else {
                                    pUsing = ::core::ptr::null_mut::<IdList>();
                                }
                                nAdd = (*pTab_0).nCol as ::core::ffi::c_int;
                                if (*pTab_0).tabFlags & TF_NoVisibleRowid as u32_0
                                    == 0 as u32_0
                                    && selFlags as ::core::ffi::c_int & SF_NestedFrom
                                        != 0 as ::core::ffi::c_int
                                {
                                    nAdd += 1;
                                }
                                let mut c2rust_current_block_196: u64;
                                j = 0 as ::core::ffi::c_int;
                                while j < nAdd {
                                    let mut zName: *const ::core::ffi::c_char = ::core::ptr::null::<
                                        ::core::ffi::c_char,
                                    >();
                                    let mut pX_0: *mut ExprList_item = ::core::ptr::null_mut::<
                                        ExprList_item,
                                    >();
                                    if j == (*pTab_0).nCol as ::core::ffi::c_int {
                                        zName = sqlite3RowidAlias(pTab_0);
                                        if zName.is_null() {
                                            c2rust_current_block_196 = 8507773468922410051;
                                        } else {
                                            c2rust_current_block_196 = 4338462691184853296;
                                        }
                                    } else {
                                        zName = (*(*pTab_0).aCol.offset(j as isize)).zCnName;
                                        if !pNestedFrom.is_null()
                                            && (*(&raw mut (*pNestedFrom).a as *mut ExprList_item)
                                                .offset(j as isize))
                                                .fg
                                                .eEName() as ::core::ffi::c_int == ENAME_ROWID
                                        {
                                            c2rust_current_block_196 = 8507773468922410051;
                                        } else if !zTName.is_null() && !pNestedFrom.is_null()
                                            && sqlite3MatchEName(
                                                (&raw mut (*pNestedFrom).a as *mut ExprList_item)
                                                    .offset(j as isize) as *mut ExprList_item,
                                                ::core::ptr::null::<::core::ffi::c_char>(),
                                                zTName,
                                                ::core::ptr::null::<::core::ffi::c_char>(),
                                                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                                            ) == 0 as ::core::ffi::c_int
                                        {
                                            c2rust_current_block_196 = 8507773468922410051;
                                        } else if (*p).selFlags & SF_IncludeHidden as u32_0
                                            == 0 as u32_0
                                            && (*(*pTab_0).aCol.offset(j as isize)).colFlags
                                                as ::core::ffi::c_int & COLFLAG_HIDDEN
                                                != 0 as ::core::ffi::c_int
                                        {
                                            c2rust_current_block_196 = 8507773468922410051;
                                        } else if (*(*pTab_0).aCol.offset(j as isize)).colFlags
                                            as ::core::ffi::c_int & COLFLAG_NOEXPAND
                                            != 0 as ::core::ffi::c_int && zTName.is_null()
                                            && selFlags as ::core::ffi::c_int
                                                & 0x800 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                        {
                                            c2rust_current_block_196 = 8507773468922410051;
                                        } else {
                                            c2rust_current_block_196 = 4338462691184853296;
                                        }
                                    }
                                    match c2rust_current_block_196 {
                                        4338462691184853296 => {
                                            tableSeen = 1 as ::core::ffi::c_int;
                                            if i > 0 as ::core::ffi::c_int && zTName.is_null()
                                                && selFlags as ::core::ffi::c_int & SF_NestedFrom
                                                    == 0 as ::core::ffi::c_int
                                            {
                                                if (*pFrom).fg.isUsing() as ::core::ffi::c_int != 0
                                                    && sqlite3IdListIndex((*pFrom).u3.pUsing, zName)
                                                        >= 0 as ::core::ffi::c_int
                                                {
                                                    c2rust_current_block_196 = 8507773468922410051;
                                                } else {
                                                    c2rust_current_block_196 = 17727836384662615028;
                                                }
                                            } else {
                                                c2rust_current_block_196 = 17727836384662615028;
                                            }
                                            match c2rust_current_block_196 {
                                                8507773468922410051 => {}
                                                _ => {
                                                    pRight = sqlite3Expr(db, TK_ID, zName);
                                                    if (*pTabList).nSrc > 1 as ::core::ffi::c_int
                                                        && ((*pFrom).fg.jointype as ::core::ffi::c_int & JT_LTORJ
                                                            == 0 as ::core::ffi::c_int
                                                            || selFlags as ::core::ffi::c_int & SF_NestedFrom
                                                                != 0 as ::core::ffi::c_int
                                                            || inAnyUsingClause(
                                                                zName,
                                                                pFrom,
                                                                (*pTabList).nSrc - i - 1 as ::core::ffi::c_int,
                                                            ) == 0)
                                                        || (*pParse).eParseMode as ::core::ffi::c_int
                                                            >= PARSE_MODE_RENAME
                                                    {
                                                        let mut pLeft: *mut Expr = ::core::ptr::null_mut::<Expr>();
                                                        pLeft = sqlite3Expr(db, TK_ID, zTabName);
                                                        pExpr = sqlite3PExpr(pParse, TK_DOT, pLeft, pRight);
                                                        if (*pParse).eParseMode as ::core::ffi::c_int
                                                            >= PARSE_MODE_RENAME && !(*pE).pLeft.is_null()
                                                        {
                                                            sqlite3RenameTokenRemap(
                                                                pParse,
                                                                pLeft as *const ::core::ffi::c_void,
                                                                (*pE).pLeft as *const ::core::ffi::c_void,
                                                            );
                                                        }
                                                        if !zSchemaName.is_null() {
                                                            pLeft = sqlite3Expr(db, TK_ID, zSchemaName);
                                                            pExpr = sqlite3PExpr(pParse, TK_DOT, pLeft, pExpr);
                                                        }
                                                    } else {
                                                        pExpr = pRight;
                                                    }
                                                    sqlite3ExprSetErrorOffset(pExpr, iErrOfst);
                                                    pNew = sqlite3ExprListAppend(pParse, pNew, pExpr);
                                                    if pNew.is_null() {
                                                        break;
                                                    }
                                                    pX_0 = (&raw mut (*pNew).a as *mut ExprList_item)
                                                        .offset(((*pNew).nExpr - 1 as ::core::ffi::c_int) as isize)
                                                        as *mut ExprList_item as *mut ExprList_item;
                                                    if selFlags as ::core::ffi::c_int & SF_NestedFrom
                                                        != 0 as ::core::ffi::c_int
                                                        && !((*pParse).eParseMode as ::core::ffi::c_int
                                                            >= PARSE_MODE_RENAME)
                                                    {
                                                        if !pNestedFrom.is_null()
                                                            && (ViewCanHaveRowid == 0 || j < (*pNestedFrom).nExpr)
                                                        {
                                                            (*pX_0).zEName = sqlite3DbStrDup(
                                                                db,
                                                                (*(&raw mut (*pNestedFrom).a as *mut ExprList_item)
                                                                    .offset(j as isize))
                                                                    .zEName,
                                                            );
                                                        } else {
                                                            (*pX_0).zEName = sqlite3MPrintf(
                                                                db,
                                                                b"%s.%s.%s\0".as_ptr() as *const ::core::ffi::c_char,
                                                                zSchemaName,
                                                                zTabName,
                                                                zName,
                                                            );
                                                        }
                                                        (*pX_0)
                                                            .fg
                                                            .set_eEName(
                                                                (if j == (*pTab_0).nCol as ::core::ffi::c_int {
                                                                    ENAME_ROWID
                                                                } else {
                                                                    ENAME_TAB
                                                                }) as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                                            );
                                                        if (*pFrom).fg.isUsing() as ::core::ffi::c_int != 0
                                                            && sqlite3IdListIndex((*pFrom).u3.pUsing, zName)
                                                                >= 0 as ::core::ffi::c_int
                                                            || !pUsing.is_null()
                                                                && sqlite3IdListIndex(pUsing, zName)
                                                                    >= 0 as ::core::ffi::c_int
                                                            || j < (*pTab_0).nCol as ::core::ffi::c_int
                                                                && (*(*pTab_0).aCol.offset(j as isize)).colFlags
                                                                    as ::core::ffi::c_int & COLFLAG_NOEXPAND != 0
                                                        {
                                                            (*pX_0)
                                                                .fg
                                                                .set_bNoExpand(
                                                                    1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                                                );
                                                        }
                                                    } else if longNames != 0 {
                                                        (*pX_0).zEName = sqlite3MPrintf(
                                                            db,
                                                            b"%s.%s\0".as_ptr() as *const ::core::ffi::c_char,
                                                            zTabName,
                                                            zName,
                                                        );
                                                        (*pX_0)
                                                            .fg
                                                            .set_eEName(
                                                                ENAME_NAME as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                                            );
                                                    } else {
                                                        (*pX_0).zEName = sqlite3DbStrDup(db, zName);
                                                        (*pX_0)
                                                            .fg
                                                            .set_eEName(
                                                                ENAME_NAME as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                                            );
                                                    }
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                    j += 1;
                                }
                            }
                            _ => {}
                        }
                        i += 1;
                        pFrom = pFrom.offset(1);
                    }
                    if tableSeen == 0 {
                        if !zTName.is_null() {
                            sqlite3ErrorMsg(
                                pParse,
                                b"no such table: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                zTName,
                            );
                        } else {
                            sqlite3ErrorMsg(
                                pParse,
                                b"no tables specified\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        }
                    }
                }
                k += 1;
            }
            sqlite3ExprListDelete(db, pEList);
            (*p).pEList = pNew;
        }
        if !(*p).pEList.is_null() {
            if (*(*p).pEList).nExpr > (*db).aLimit[SQLITE_LIMIT_COLUMN as usize] {
                sqlite3ErrorMsg(
                    pParse,
                    b"too many columns in result set\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return WRC_Abort;
            }
            if elistFlags & (EP_HasFunc | EP_Subquery) as u32_0 != 0 as u32_0 {
                (*p).selFlags = ((*p).selFlags as ::core::ffi::c_uint
                    | SF_ComplexResult as ::core::ffi::c_uint) as u32_0;
            }
        }
        return WRC_Continue;
    }
}
unsafe extern "C" fn sqlite3SelectExpand(
    mut pParse: *mut Parse,
    mut pSelect: *mut Select,
) {
    unsafe {
        let mut w: Walker = Walker {
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

        };
        w.xExprCallback = Some(
            sqlite3ExprWalkNoop
                as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int>;
        w.pParse = pParse;
        if (*pParse).hasCompound != 0 {
            w.xSelectCallback = Some(
                convertCompoundSelectToSubquery
                    as unsafe extern "C" fn(
                        *mut Walker,
                        *mut Select,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
                >;
            w.xSelectCallback2 = None;
            sqlite3WalkSelect(&raw mut w, pSelect);
        }
        w.xSelectCallback = Some(
            selectExpander
                as unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
            >;
        w.xSelectCallback2 = Some(
            sqlite3SelectPopWith as unsafe extern "C" fn(*mut Walker, *mut Select) -> (),
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Select) -> ()>;
        w.eCode = 0 as u16_0;
        sqlite3WalkSelect(&raw mut w, pSelect);
    }
}
unsafe extern "C" fn selectAddSubqueryTypeInfo(
    mut pWalker: *mut Walker,
    mut p: *mut Select,
) {
    unsafe {
        let mut pParse: *mut Parse = ::core::ptr::null_mut::<Parse>();
        let mut i: ::core::ffi::c_int = 0;
        let mut pTabList: *mut SrcList = ::core::ptr::null_mut::<SrcList>();
        let mut pFrom: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
        if (*p).selFlags & SF_HasTypeInfo as u32_0 != 0 {
            return;
        }
        (*p).selFlags = ((*p).selFlags as ::core::ffi::c_uint
            | SF_HasTypeInfo as ::core::ffi::c_uint) as u32_0;
        pParse = (*pWalker).pParse;
        pTabList = (*p).pSrc;
        i = 0 as ::core::ffi::c_int;
        pFrom = &raw mut (*pTabList).a as *mut SrcItem;
        while i < (*pTabList).nSrc {
            let mut pTab: *mut Table = (*pFrom).pSTab;
            if (*pTab).tabFlags & TF_Ephemeral as u32_0 != 0 as u32_0
                && (*pFrom).fg.isSubquery() as ::core::ffi::c_int != 0
            {
                let mut pSel: *mut Select = (*(*pFrom).u4.pSubq).pSelect;
                sqlite3SubqueryColumnTypes(
                    pParse,
                    pTab,
                    pSel,
                    SQLITE_AFF_NONE as ::core::ffi::c_char,
                );
            }
            i += 1;
            pFrom = pFrom.offset(1);
        }
    }
}
unsafe extern "C" fn sqlite3SelectAddTypeInfo(
    mut pParse: *mut Parse,
    mut pSelect: *mut Select,
) {
    unsafe {
        let mut w: Walker = Walker {
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

        };
        w.xSelectCallback = Some(
            sqlite3SelectWalkNoop
                as unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
            >;
        w.xSelectCallback2 = Some(
            selectAddSubqueryTypeInfo
                as unsafe extern "C" fn(*mut Walker, *mut Select) -> (),
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Select) -> ()>;
        w.xExprCallback = Some(
            sqlite3ExprWalkNoop
                as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int>;
        w.pParse = pParse;
        sqlite3WalkSelect(&raw mut w, pSelect);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SelectPrep(
    mut pParse: *mut Parse,
    mut p: *mut Select,
    mut pOuterNC: *mut NameContext,
) {
    unsafe {
        if (*(*pParse).db).mallocFailed != 0 {
            return;
        }
        if (*p).selFlags & SF_HasTypeInfo as u32_0 != 0 {
            return;
        }
        sqlite3SelectExpand(pParse, p);
        if (*pParse).nErr != 0 {
            return;
        }
        sqlite3ResolveSelectNames(pParse, p, pOuterNC);
        if (*pParse).nErr != 0 {
            return;
        }
        sqlite3SelectAddTypeInfo(pParse, p);
    }
}
unsafe extern "C" fn analyzeAggFuncArgs(
    mut pAggInfo: *mut AggInfo,
    mut pNC: *mut NameContext,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        (*pNC).ncFlags |= NC_InAggFunc;
        i = 0 as ::core::ffi::c_int;
        while i < (*pAggInfo).nFunc {
            let mut pExpr: *mut Expr = (*(*pAggInfo).aFunc.offset(i as isize)).pFExpr;
            sqlite3ExprAnalyzeAggList(pNC, (*pExpr).x.pList);
            if !(*pExpr).pLeft.is_null() {
                sqlite3ExprAnalyzeAggList(pNC, (*(*pExpr).pLeft).x.pList);
            }
            if (*pExpr).flags & 0x1000000 as ::core::ffi::c_int as u32_0 != 0 as u32_0 {
                sqlite3ExprAnalyzeAggregates(pNC, (*(*pExpr).y.pWin).pFilter);
            }
            i += 1;
        }
        (*pNC).ncFlags &= !NC_InAggFunc;
    }
}
unsafe extern "C" fn optimizeAggregateUseOfIndexedExpr(
    mut pParse: *mut Parse,
    mut pSelect: *mut Select,
    mut pAggInfo: *mut AggInfo,
    mut pNC: *mut NameContext,
) {
    unsafe {
        (*pAggInfo).nColumn = (*pAggInfo).nAccumulator;
        if (*pAggInfo).nSortingColumn > 0 as u32_0 {
            let mut mx: ::core::ffi::c_int = (*(*pSelect).pGroupBy).nExpr
                - 1 as ::core::ffi::c_int;
            let mut j: ::core::ffi::c_int = 0;
            let mut k: ::core::ffi::c_int = 0;
            j = 0 as ::core::ffi::c_int;
            while j < (*pAggInfo).nColumn {
                k = (*(*pAggInfo).aCol.offset(j as isize)).iSorterColumn;
                if k > mx {
                    mx = k;
                }
                j += 1;
            }
            (*pAggInfo).nSortingColumn = (mx + 1 as ::core::ffi::c_int) as u32_0;
        }
        analyzeAggFuncArgs(pAggInfo, pNC);
    }
}
unsafe extern "C" fn aggregateIdxEprRefToColCallback(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pAggInfo: *mut AggInfo = ::core::ptr::null_mut::<AggInfo>();
        let mut pCol: *mut AggInfo_col = ::core::ptr::null_mut::<AggInfo_col>();
        if (*pExpr).pAggInfo.is_null() {
            return WRC_Continue;
        }
        if (*pExpr).op as ::core::ffi::c_int == TK_AGG_COLUMN {
            return WRC_Continue;
        }
        if (*pExpr).op as ::core::ffi::c_int == TK_AGG_FUNCTION {
            return WRC_Continue;
        }
        if (*pExpr).op as ::core::ffi::c_int == TK_IF_NULL_ROW {
            return WRC_Continue;
        }
        pAggInfo = (*pExpr).pAggInfo;
        if (*pExpr).iAgg as ::core::ffi::c_int >= (*pAggInfo).nColumn {
            return WRC_Continue;
        }
        pCol = (*pAggInfo).aCol.offset((*pExpr).iAgg as isize) as *mut AggInfo_col
            as *mut AggInfo_col;
        (*pExpr).op = TK_AGG_COLUMN as u8_0;
        (*pExpr).iTable = (*pCol).iTable;
        (*pExpr).iColumn = (*pCol).iColumn as ynVar;
        (*pExpr).flags = ((*pExpr).flags as ::core::ffi::c_uint
            & !((0x2000 as ::core::ffi::c_int | 0x200 as ::core::ffi::c_int
                | 0x80000 as ::core::ffi::c_int) as u32_0) as ::core::ffi::c_uint)
            as u32_0;
        return WRC_Prune;
    }
}
unsafe extern "C" fn aggregateConvertIndexedExprRefToColumn(mut pAggInfo: *mut AggInfo) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut w: Walker = Walker {
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

        };
        memset(
            &raw mut w as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<Walker>() as size_t,
        );
        w.xExprCallback = Some(
            aggregateIdxEprRefToColCallback
                as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int>;
        i = 0 as ::core::ffi::c_int;
        while i < (*pAggInfo).nFunc {
            sqlite3WalkExpr(&raw mut w, (*(*pAggInfo).aFunc.offset(i as isize)).pFExpr);
            i += 1;
        }
    }
}
unsafe extern "C" fn assignAggregateRegisters(
    mut pParse: *mut Parse,
    mut pAggInfo: *mut AggInfo,
) {
    unsafe {
        (*pAggInfo).iFirstReg = (*pParse).nMem + 1 as ::core::ffi::c_int;
        (*pParse).nMem += (*pAggInfo).nColumn + (*pAggInfo).nFunc;
    }
}
unsafe extern "C" fn resetAccumulator(
    mut pParse: *mut Parse,
    mut pAggInfo: *mut AggInfo,
) {
    unsafe {
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        let mut i: ::core::ffi::c_int = 0;
        let mut pFunc: *mut AggInfo_func = ::core::ptr::null_mut::<AggInfo_func>();
        let mut nReg: ::core::ffi::c_int = (*pAggInfo).nFunc + (*pAggInfo).nColumn;
        if nReg == 0 as ::core::ffi::c_int {
            return;
        }
        if (*pParse).nErr != 0 {
            return;
        }
        sqlite3VdbeAddOp3(
            v,
            OP_Null,
            0 as ::core::ffi::c_int,
            (*pAggInfo).iFirstReg,
            (*pAggInfo).iFirstReg + nReg - 1 as ::core::ffi::c_int,
        );
        pFunc = (*pAggInfo).aFunc as *mut AggInfo_func;
        i = 0 as ::core::ffi::c_int;
        while i < (*pAggInfo).nFunc {
            if (*pFunc).iDistinct >= 0 as ::core::ffi::c_int {
                let mut pE: *mut Expr = (*pFunc).pFExpr;
                if (*pE).x.pList.is_null()
                    || (*(*pE).x.pList).nExpr != 1 as ::core::ffi::c_int
                {
                    sqlite3ErrorMsg(
                        pParse,
                        b"DISTINCT aggregates must have exactly one argument\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    (*pFunc).iDistinct = -1 as ::core::ffi::c_int;
                } else {
                    let mut pKeyInfo: *mut KeyInfo = sqlite3KeyInfoFromExprList(
                        pParse,
                        (*pE).x.pList,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                    );
                    (*pFunc).iDistAddr = sqlite3VdbeAddOp4(
                        v,
                        OP_OpenEphemeral,
                        (*pFunc).iDistinct,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        pKeyInfo as *mut ::core::ffi::c_char,
                        P4_KEYINFO,
                    );
                    sqlite3VdbeExplain(
                        pParse,
                        0 as u8_0,
                        b"USE TEMP B-TREE FOR %s(DISTINCT)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*(*pFunc).pFunc).zName,
                    );
                }
            }
            if (*pFunc).iOBTab >= 0 as ::core::ffi::c_int {
                let mut pOBList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
                let mut pKeyInfo_0: *mut KeyInfo = ::core::ptr::null_mut::<KeyInfo>();
                let mut nExtra: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                pOBList = (*(*(*pFunc).pFExpr).pLeft).x.pList;
                if (*pFunc).bOBUnique == 0 {
                    nExtra += 1;
                }
                if (*pFunc).bOBPayload != 0 {
                    nExtra += (*(*(*pFunc).pFExpr).x.pList).nExpr;
                }
                if (*pFunc).bUseSubtype != 0 {
                    nExtra += (*(*(*pFunc).pFExpr).x.pList).nExpr;
                }
                pKeyInfo_0 = sqlite3KeyInfoFromExprList(
                    pParse,
                    pOBList,
                    0 as ::core::ffi::c_int,
                    nExtra,
                );
                if (*pFunc).bOBUnique == 0 && (*pParse).nErr == 0 as ::core::ffi::c_int {
                    (*pKeyInfo_0).nKeyField = (*pKeyInfo_0).nKeyField.wrapping_add(1);
                }
                sqlite3VdbeAddOp4(
                    v,
                    OP_OpenEphemeral,
                    (*pFunc).iOBTab,
                    (*pOBList).nExpr + nExtra,
                    0 as ::core::ffi::c_int,
                    pKeyInfo_0 as *mut ::core::ffi::c_char,
                    P4_KEYINFO,
                );
                sqlite3VdbeExplain(
                    pParse,
                    0 as u8_0,
                    b"USE TEMP B-TREE FOR %s(ORDER BY)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*(*pFunc).pFunc).zName,
                );
            }
            i += 1;
            pFunc = pFunc.offset(1);
        }
    }
}
unsafe extern "C" fn finalizeAggFunctions(
    mut pParse: *mut Parse,
    mut pAggInfo: *mut AggInfo,
) {
    unsafe {
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        let mut i: ::core::ffi::c_int = 0;
        let mut pF: *mut AggInfo_func = ::core::ptr::null_mut::<AggInfo_func>();
        i = 0 as ::core::ffi::c_int;
        pF = (*pAggInfo).aFunc as *mut AggInfo_func;
        while i < (*pAggInfo).nFunc {
            let mut pList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
            if (*pParse).nErr != 0 {
                return;
            }
            pList = (*(*pF).pFExpr).x.pList;
            if (*pF).iOBTab >= 0 as ::core::ffi::c_int {
                let mut iTop: ::core::ffi::c_int = 0;
                let mut nArg: ::core::ffi::c_int = 0;
                let mut nKey: ::core::ffi::c_int = 0;
                let mut regAgg: ::core::ffi::c_int = 0;
                let mut j: ::core::ffi::c_int = 0;
                nArg = (*pList).nExpr;
                regAgg = sqlite3GetTempRange(pParse, nArg);
                if (*pF).bOBPayload as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    nKey = 0 as ::core::ffi::c_int;
                } else {
                    nKey = (*(*(*(*pF).pFExpr).pLeft).x.pList).nExpr;
                    if (*pF).bOBUnique == 0 {
                        nKey += 1;
                    }
                }
                iTop = sqlite3VdbeAddOp1(v, OP_Rewind, (*pF).iOBTab);
                j = nArg - 1 as ::core::ffi::c_int;
                while j >= 0 as ::core::ffi::c_int {
                    sqlite3VdbeAddOp3(v, OP_Column, (*pF).iOBTab, nKey + j, regAgg + j);
                    j -= 1;
                }
                if (*pF).bUseSubtype != 0 {
                    let mut regSubtype: ::core::ffi::c_int = sqlite3GetTempReg(pParse);
                    let mut iBaseCol: ::core::ffi::c_int = nKey + nArg
                        + ((*pF).bOBPayload as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                            && (*pF).bOBUnique as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
                    j = nArg - 1 as ::core::ffi::c_int;
                    while j >= 0 as ::core::ffi::c_int {
                        sqlite3VdbeAddOp3(
                            v,
                            OP_Column,
                            (*pF).iOBTab,
                            iBaseCol + j,
                            regSubtype,
                        );
                        sqlite3VdbeAddOp2(v, OP_SetSubtype, regSubtype, regAgg + j);
                        j -= 1;
                    }
                    sqlite3ReleaseTempReg(pParse, regSubtype);
                }
                sqlite3VdbeAddOp3(
                    v,
                    OP_AggStep,
                    0 as ::core::ffi::c_int,
                    regAgg,
                    (*pAggInfo).iFirstReg + (*pAggInfo).nColumn + i,
                );
                sqlite3VdbeAppendP4(
                    v,
                    (*pF).pFunc as *mut ::core::ffi::c_void,
                    P4_FUNCDEF,
                );
                sqlite3VdbeChangeP5(v, nArg as u16_0);
                sqlite3VdbeAddOp2(
                    v,
                    OP_Next,
                    (*pF).iOBTab,
                    iTop + 1 as ::core::ffi::c_int,
                );
                sqlite3VdbeJumpHere(v, iTop);
                sqlite3ReleaseTempRange(pParse, regAgg, nArg);
            }
            sqlite3VdbeAddOp2(
                v,
                OP_AggFinal,
                (*pAggInfo).iFirstReg + (*pAggInfo).nColumn + i,
                if !pList.is_null() { (*pList).nExpr } else { 0 as ::core::ffi::c_int },
            );
            sqlite3VdbeAppendP4(v, (*pF).pFunc as *mut ::core::ffi::c_void, P4_FUNCDEF);
            i += 1;
            pF = pF.offset(1);
        }
    }
}
unsafe extern "C" fn updateAccumulator(
    mut pParse: *mut Parse,
    mut regAcc: ::core::ffi::c_int,
    mut pAggInfo: *mut AggInfo,
    mut eDistinctType: ::core::ffi::c_int,
) {
    unsafe {
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        let mut i: ::core::ffi::c_int = 0;
        let mut regHit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut addrHitTest: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pF: *mut AggInfo_func = ::core::ptr::null_mut::<AggInfo_func>();
        let mut pC: *mut AggInfo_col = ::core::ptr::null_mut::<AggInfo_col>();
        if (*pParse).nErr != 0 {
            return;
        }
        (*pAggInfo).directMode = 1 as u8_0;
        i = 0 as ::core::ffi::c_int;
        pF = (*pAggInfo).aFunc as *mut AggInfo_func;
        while i < (*pAggInfo).nFunc {
            let mut nArg: ::core::ffi::c_int = 0;
            let mut addrNext: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut regAgg: ::core::ffi::c_int = 0;
            let mut regAggSz: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut regDistinct: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut pList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
            pList = (*(*pF).pFExpr).x.pList;
            if (*(*pF).pFExpr).flags & 0x1000000 as ::core::ffi::c_int as u32_0
                != 0 as u32_0
            {
                let mut pFilter: *mut Expr = (*(*(*pF).pFExpr).y.pWin).pFilter;
                if (*pAggInfo).nAccumulator != 0
                    && (*(*pF).pFunc).funcFlags & SQLITE_FUNC_NEEDCOLL as u32_0 != 0
                    && regAcc != 0
                {
                    if regHit == 0 as ::core::ffi::c_int {
                        (*pParse).nMem += 1;
                        regHit = (*pParse).nMem;
                    }
                    sqlite3VdbeAddOp2(v, OP_Copy, regAcc, regHit);
                }
                addrNext = sqlite3VdbeMakeLabel(pParse);
                sqlite3ExprIfFalse(pParse, pFilter, addrNext, SQLITE_JUMPIFNULL);
            }
            if (*pF).iOBTab >= 0 as ::core::ffi::c_int {
                let mut jj: ::core::ffi::c_int = 0;
                let mut pOBList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
                nArg = (*pList).nExpr;
                pOBList = (*(*(*pF).pFExpr).pLeft).x.pList;
                regAggSz = (*pOBList).nExpr;
                if (*pF).bOBUnique == 0 {
                    regAggSz += 1;
                }
                if (*pF).bOBPayload != 0 {
                    regAggSz += nArg;
                }
                if (*pF).bUseSubtype != 0 {
                    regAggSz += nArg;
                }
                regAggSz += 1;
                regAgg = sqlite3GetTempRange(pParse, regAggSz);
                regDistinct = regAgg;
                sqlite3ExprCodeExprList(
                    pParse,
                    pOBList,
                    regAgg,
                    0 as ::core::ffi::c_int,
                    SQLITE_ECEL_DUP as u8_0,
                );
                jj = (*pOBList).nExpr;
                if (*pF).bOBUnique == 0 {
                    sqlite3VdbeAddOp2(v, OP_Sequence, (*pF).iOBTab, regAgg + jj);
                    jj += 1;
                }
                if (*pF).bOBPayload != 0 {
                    regDistinct = regAgg + jj;
                    sqlite3ExprCodeExprList(
                        pParse,
                        pList,
                        regDistinct,
                        0 as ::core::ffi::c_int,
                        SQLITE_ECEL_DUP as u8_0,
                    );
                    jj += nArg;
                }
                if (*pF).bUseSubtype != 0 {
                    let mut kk: ::core::ffi::c_int = 0;
                    let mut regBase: ::core::ffi::c_int = if (*pF).bOBPayload
                        as ::core::ffi::c_int != 0
                    {
                        regDistinct
                    } else {
                        regAgg
                    };
                    kk = 0 as ::core::ffi::c_int;
                    while kk < nArg {
                        sqlite3VdbeAddOp2(v, OP_GetSubtype, regBase + kk, regAgg + jj);
                        kk += 1;
                        jj += 1;
                    }
                }
            } else if !pList.is_null() {
                nArg = (*pList).nExpr;
                regAgg = sqlite3GetTempRange(pParse, nArg);
                regDistinct = regAgg;
                sqlite3ExprCodeExprList(
                    pParse,
                    pList,
                    regAgg,
                    0 as ::core::ffi::c_int,
                    SQLITE_ECEL_DUP as u8_0,
                );
            } else {
                nArg = 0 as ::core::ffi::c_int;
                regAgg = 0 as ::core::ffi::c_int;
            }
            if (*pF).iDistinct >= 0 as ::core::ffi::c_int && !pList.is_null() {
                if addrNext == 0 as ::core::ffi::c_int {
                    addrNext = sqlite3VdbeMakeLabel(pParse);
                }
                (*pF).iDistinct = codeDistinct(
                    pParse,
                    eDistinctType,
                    (*pF).iDistinct,
                    addrNext,
                    pList,
                    regDistinct,
                );
            }
            if (*pF).iOBTab >= 0 as ::core::ffi::c_int {
                sqlite3VdbeAddOp3(
                    v,
                    OP_MakeRecord,
                    regAgg,
                    regAggSz - 1 as ::core::ffi::c_int,
                    regAgg + regAggSz - 1 as ::core::ffi::c_int,
                );
                sqlite3VdbeAddOp4Int(
                    v,
                    OP_IdxInsert,
                    (*pF).iOBTab,
                    regAgg + regAggSz - 1 as ::core::ffi::c_int,
                    regAgg,
                    regAggSz - 1 as ::core::ffi::c_int,
                );
                sqlite3ReleaseTempRange(pParse, regAgg, regAggSz);
            } else {
                if (*(*pF).pFunc).funcFlags & SQLITE_FUNC_NEEDCOLL as u32_0 != 0 {
                    let mut pColl: *mut CollSeq = ::core::ptr::null_mut::<CollSeq>();
                    let mut pItem: *mut ExprList_item = ::core::ptr::null_mut::<
                        ExprList_item,
                    >();
                    let mut j: ::core::ffi::c_int = 0;
                    j = 0 as ::core::ffi::c_int;
                    pItem = &raw mut (*pList).a as *mut ExprList_item
                        as *mut ExprList_item;
                    while pColl.is_null() && j < nArg {
                        pColl = sqlite3ExprCollSeq(pParse, (*pItem).pExpr);
                        j += 1;
                        pItem = pItem.offset(1);
                    }
                    if pColl.is_null() {
                        pColl = (*(*pParse).db).pDfltColl;
                    }
                    if regHit == 0 as ::core::ffi::c_int && (*pAggInfo).nAccumulator != 0
                    {
                        (*pParse).nMem += 1;
                        regHit = (*pParse).nMem;
                    }
                    sqlite3VdbeAddOp4(
                        v,
                        OP_CollSeq,
                        regHit,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        pColl as *mut ::core::ffi::c_char,
                        P4_COLLSEQ,
                    );
                }
                sqlite3VdbeAddOp3(
                    v,
                    OP_AggStep,
                    0 as ::core::ffi::c_int,
                    regAgg,
                    (*pAggInfo).iFirstReg + (*pAggInfo).nColumn + i,
                );
                sqlite3VdbeAppendP4(
                    v,
                    (*pF).pFunc as *mut ::core::ffi::c_void,
                    P4_FUNCDEF,
                );
                sqlite3VdbeChangeP5(v, nArg as u16_0);
                sqlite3ReleaseTempRange(pParse, regAgg, nArg);
            }
            if addrNext != 0 {
                sqlite3VdbeResolveLabel(v, addrNext);
            }
            if (*pParse).nErr != 0 {
                return;
            }
            i += 1;
            pF = pF.offset(1);
        }
        if regHit == 0 as ::core::ffi::c_int && (*pAggInfo).nAccumulator != 0 {
            regHit = regAcc;
        }
        if regHit != 0 {
            addrHitTest = sqlite3VdbeAddOp1(v, OP_If, regHit);
        }
        i = 0 as ::core::ffi::c_int;
        pC = (*pAggInfo).aCol as *mut AggInfo_col;
        while i < (*pAggInfo).nAccumulator {
            sqlite3ExprCode(pParse, (*pC).pCExpr, (*pAggInfo).iFirstReg + i);
            if (*pParse).nErr != 0 {
                return;
            }
            i += 1;
            pC = pC.offset(1);
        }
        (*pAggInfo).directMode = 0 as u8_0;
        if addrHitTest != 0 {
            sqlite3VdbeJumpHereOrPopInst(v, addrHitTest);
        }
    }
}
unsafe extern "C" fn explainSimpleCount(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut pIdx: *mut Index,
) {
    unsafe {
        if (*pParse).explain as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            let mut bCover: ::core::ffi::c_int = (!pIdx.is_null()
                && ((*pTab).tabFlags & TF_WithoutRowid as u32_0 == 0 as u32_0
                    || !((*pIdx).idxType() as ::core::ffi::c_int
                        == SQLITE_IDXTYPE_PRIMARYKEY))) as ::core::ffi::c_int;
            sqlite3VdbeExplain(
                pParse,
                0 as u8_0,
                b"SCAN %s%s%s\0".as_ptr() as *const ::core::ffi::c_char,
                (*pTab).zName,
                if bCover != 0 {
                    b" USING COVERING INDEX \0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
                if bCover != 0 {
                    (*pIdx).zName as *const ::core::ffi::c_char
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        }
    }
}
unsafe extern "C" fn havingToWhereExprCb(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pExpr).op as ::core::ffi::c_int != TK_AND {
            let mut pS: *mut Select = (*pWalker).u.pSelect;
            if sqlite3ExprIsConstantOrGroupBy((*pWalker).pParse, pExpr, (*pS).pGroupBy)
                != 0
                && ((*pExpr).flags & (EP_OuterON | EP_IsFalse) as u32_0
                    == EP_IsFalse as u32_0) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int && (*pExpr).pAggInfo.is_null()
            {
                let mut db: *mut sqlite3 = (*(*pWalker).pParse).db;
                let mut pNew: *mut Expr = sqlite3Expr(
                    db,
                    TK_INTEGER,
                    b"1\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if !pNew.is_null() {
                    let mut pWhere: *mut Expr = (*pS).pWhere;
                    let mut t: Expr = *pNew;
                    *pNew = *pExpr;
                    *pExpr = t;
                    pNew = sqlite3ExprAnd((*pWalker).pParse, pWhere, pNew);
                    (*pS).pWhere = pNew;
                    (*pWalker).eCode = 1 as u16_0;
                }
            }
            return WRC_Prune;
        }
        return WRC_Continue;
    }
}
unsafe extern "C" fn havingToWhere(mut pParse: *mut Parse, mut p: *mut Select) {
    unsafe {
        let mut sWalker: Walker = Walker {
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

        };
        memset(
            &raw mut sWalker as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<Walker>() as size_t,
        );
        sWalker.pParse = pParse;
        sWalker.xExprCallback = Some(
            havingToWhereExprCb
                as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int>;
        sWalker.u.pSelect = p;
        sqlite3WalkExpr(&raw mut sWalker, (*p).pHaving);
    }
}
unsafe extern "C" fn isSelfJoinView(
    mut pTabList: *mut SrcList,
    mut pThis: *mut SrcItem,
    mut iFirst: ::core::ffi::c_int,
    mut iEnd: ::core::ffi::c_int,
) -> *mut SrcItem {
    unsafe {
        let mut pItem: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
        let mut pSel: *mut Select = ::core::ptr::null_mut::<Select>();
        pSel = (*(*pThis).u4.pSubq).pSelect;
        if (*pSel).selFlags & SF_PushDown as u32_0 != 0 {
            return ::core::ptr::null_mut::<SrcItem>();
        }
        while iFirst < iEnd {
            let mut pS1: *mut Select = ::core::ptr::null_mut::<Select>();
            let c2rust_fresh16 = iFirst;
            iFirst = iFirst + 1;
            pItem = (&raw mut (*pTabList).a as *mut SrcItem)
                .offset(c2rust_fresh16 as isize) as *mut SrcItem;
            if (*pItem).fg.isSubquery() == 0 {
                continue;
            }
            if (*pItem).fg.viaCoroutine() != 0 {
                continue;
            }
            if (*pItem).zName.is_null() {
                continue;
            }
            if (*(*pItem).pSTab).pSchema != (*(*pThis).pSTab).pSchema {
                continue;
            }
            if sqlite3_stricmp((*pItem).zName, (*pThis).zName) != 0 as ::core::ffi::c_int
            {
                continue;
            }
            pS1 = (*(*pItem).u4.pSubq).pSelect;
            if (*(*pItem).pSTab).pSchema.is_null() && (*pSel).selId != (*pS1).selId {
                continue;
            }
            if (*pS1).selFlags & SF_PushDown as u32_0 != 0 {
                continue;
            }
            return pItem;
        }
        return ::core::ptr::null_mut::<SrcItem>();
    }
}
unsafe extern "C" fn agginfoFree(
    mut db: *mut sqlite3,
    mut pArg: *mut ::core::ffi::c_void,
) {
    unsafe {
        let mut p: *mut AggInfo = pArg as *mut AggInfo;
        sqlite3DbFree(db, (*p).aCol as *mut ::core::ffi::c_void);
        sqlite3DbFree(db, (*p).aFunc as *mut ::core::ffi::c_void);
        sqlite3DbFreeNN(db, p as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn countOfViewOptimization(
    mut pParse: *mut Parse,
    mut p: *mut Select,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pSub: *mut Select = ::core::ptr::null_mut::<Select>();
        let mut pPrior: *mut Select = ::core::ptr::null_mut::<Select>();
        let mut pExpr: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut pCount: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut pFrom: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
        if (*p).selFlags & SF_Aggregate as u32_0 == 0 as u32_0 {
            return 0 as ::core::ffi::c_int;
        }
        if (*(*p).pEList).nExpr != 1 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        if !(*p).pWhere.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if !(*p).pHaving.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if !(*p).pGroupBy.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if !(*p).pOrderBy.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        pExpr = (*(&raw mut (*(*p).pEList).a as *mut ExprList_item)
            .offset(0 as ::core::ffi::c_int as isize))
            .pExpr;
        if (*pExpr).op as ::core::ffi::c_int != TK_AGG_FUNCTION {
            return 0 as ::core::ffi::c_int;
        }
        if sqlite3_stricmp(
            (*pExpr).u.zToken,
            b"count\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0
        {
            return 0 as ::core::ffi::c_int;
        }
        if !(*pExpr).x.pList.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if (*(*p).pSrc).nSrc != 1 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        if (*pExpr).flags & 0x1000000 as ::core::ffi::c_int as u32_0 != 0 as u32_0 {
            return 0 as ::core::ffi::c_int;
        }
        pFrom = &raw mut (*(*p).pSrc).a as *mut SrcItem;
        if (*pFrom).fg.isSubquery() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        pSub = (*(*pFrom).u4.pSubq).pSelect;
        if (*pSub).pPrior.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if (*pSub).selFlags & SF_CopyCte as u32_0 != 0 {
            return 0 as ::core::ffi::c_int;
        }
        loop {
            if (*pSub).op as ::core::ffi::c_int != TK_ALL && !(*pSub).pPrior.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            if !(*pSub).pWhere.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            if !(*pSub).pLimit.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            if (*pSub).selFlags & (SF_Aggregate | SF_Distinct) as u32_0 != 0 {
                return 0 as ::core::ffi::c_int;
            }
            pSub = (*pSub).pPrior;
            if pSub.is_null() {
                break;
            }
        }
        db = (*pParse).db;
        pCount = pExpr;
        pExpr = ::core::ptr::null_mut::<Expr>();
        pSub = sqlite3SubqueryDetach(db, pFrom);
        sqlite3SrcListDelete(db, (*p).pSrc);
        (*p).pSrc = sqlite3DbMallocZero((*pParse).db, SZ_SRCLIST_1 as u64_0)
            as *mut SrcList;
        while !pSub.is_null() {
            let mut pTerm: *mut Expr = ::core::ptr::null_mut::<Expr>();
            pPrior = (*pSub).pPrior;
            (*pSub).pPrior = ::core::ptr::null_mut::<Select>();
            (*pSub).pNext = ::core::ptr::null_mut::<Select>();
            (*pSub).selFlags = ((*pSub).selFlags as ::core::ffi::c_uint
                | SF_Aggregate as ::core::ffi::c_uint) as u32_0;
            (*pSub).selFlags = ((*pSub).selFlags as ::core::ffi::c_uint
                & !(SF_Compound as u32_0) as ::core::ffi::c_uint) as u32_0;
            (*pSub).nSelectRow = 0 as LogEst;
            sqlite3ParserAddCleanup(
                pParse,
                Some(
                    sqlite3ExprListDeleteGeneric
                        as unsafe extern "C" fn(
                            *mut sqlite3,
                            *mut ::core::ffi::c_void,
                        ) -> (),
                ),
                (*pSub).pEList as *mut ::core::ffi::c_void,
            );
            pTerm = if !pPrior.is_null() {
                sqlite3ExprDup(db, pCount, 0 as ::core::ffi::c_int)
            } else {
                pCount
            };
            (*pSub).pEList = sqlite3ExprListAppend(
                pParse,
                ::core::ptr::null_mut::<ExprList>(),
                pTerm,
            );
            pTerm = sqlite3PExpr(
                pParse,
                TK_SELECT,
                ::core::ptr::null_mut::<Expr>(),
                ::core::ptr::null_mut::<Expr>(),
            );
            sqlite3PExprAddSelect(pParse, pTerm, pSub);
            if pExpr.is_null() {
                pExpr = pTerm;
            } else {
                pExpr = sqlite3PExpr(pParse, TK_PLUS, pTerm, pExpr);
            }
            pSub = pPrior;
        }
        let ref mut c2rust_fresh18 = (*(&raw mut (*(*p).pEList).a as *mut ExprList_item)
            .offset(0 as ::core::ffi::c_int as isize))
            .pExpr;
        *c2rust_fresh18 = pExpr;
        (*p).selFlags = ((*p).selFlags as ::core::ffi::c_uint
            & !(SF_Aggregate as u32_0) as ::core::ffi::c_uint) as u32_0;
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn sameSrcAlias(
    mut p0: *mut SrcItem,
    mut pSrc: *mut SrcList,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*pSrc).nSrc {
            let mut p1: *mut SrcItem = (&raw mut (*pSrc).a as *mut SrcItem)
                .offset(i as isize) as *mut SrcItem;
            if !(p1 == p0) {
                if (*p0).pSTab == (*p1).pSTab
                    && 0 as ::core::ffi::c_int
                        == sqlite3_stricmp((*p0).zAlias, (*p1).zAlias)
                {
                    return 1 as ::core::ffi::c_int;
                }
                if (*p1).fg.isSubquery() as ::core::ffi::c_int != 0
                    && (*(*(*p1).u4.pSubq).pSelect).selFlags & SF_NestedFrom as u32_0
                        != 0 as u32_0
                    && sameSrcAlias(p0, (*(*(*p1).u4.pSubq).pSelect).pSrc) != 0
                {
                    return 1 as ::core::ffi::c_int;
                }
            }
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn fromClauseTermCanBeCoroutine(
    mut pParse: *mut Parse,
    mut pTabList: *mut SrcList,
    mut i: ::core::ffi::c_int,
    mut selFlags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pItem: *mut SrcItem = (&raw mut (*pTabList).a as *mut SrcItem)
            .offset(i as isize) as *mut SrcItem;
        if (*pItem).fg.isCte() != 0 {
            let mut pCteUse: *const CteUse = (*pItem).u2.pCteUse;
            if (*pCteUse).eM10d as ::core::ffi::c_int == M10d_Yes {
                return 0 as ::core::ffi::c_int;
            }
            if (*pCteUse).nUse >= 2 as ::core::ffi::c_int
                && (*pCteUse).eM10d as ::core::ffi::c_int != M10d_No
            {
                return 0 as ::core::ffi::c_int;
            }
        }
        if (*(&raw mut (*pTabList).a as *mut SrcItem)
            .offset(0 as ::core::ffi::c_int as isize))
            .fg
            .jointype as ::core::ffi::c_int & JT_LTORJ != 0
        {
            return 0 as ::core::ffi::c_int;
        }
        if (*(*pParse).db).dbOptFlags & 0x2000000 as u32_0 != 0 as u32_0 {
            return 0 as ::core::ffi::c_int;
        }
        if !isSelfJoinView(
                pTabList,
                pItem,
                i + 1 as ::core::ffi::c_int,
                (*pTabList).nSrc,
            )
            .is_null()
        {
            return 0 as ::core::ffi::c_int;
        }
        if i == 0 as ::core::ffi::c_int {
            if (*pTabList).nSrc == 1 as ::core::ffi::c_int {
                return 1 as ::core::ffi::c_int;
            }
            if (*(&raw mut (*pTabList).a as *mut SrcItem)
                .offset(1 as ::core::ffi::c_int as isize))
                .fg
                .jointype as ::core::ffi::c_int & JT_CROSS != 0
            {
                return 1 as ::core::ffi::c_int;
            }
            if selFlags & SF_UpdateFrom != 0 {
                return 0 as ::core::ffi::c_int;
            }
            return 1 as ::core::ffi::c_int;
        }
        if selFlags & SF_UpdateFrom != 0 {
            return 0 as ::core::ffi::c_int;
        }
        loop {
            if (*pItem).fg.jointype as ::core::ffi::c_int & (JT_OUTER | JT_CROSS) != 0 {
                return 0 as ::core::ffi::c_int;
            }
            if i == 0 as ::core::ffi::c_int {
                break;
            }
            i -= 1;
            pItem = pItem.offset(-1);
            if (*pItem).fg.isSubquery() != 0 {
                return 0 as ::core::ffi::c_int;
            }
        }
        return 1 as ::core::ffi::c_int;
    }
}
#[inline(never)]
unsafe extern "C" fn existsToJoin(
    mut pParse: *mut Parse,
    mut p: *mut Select,
    mut pWhere: *mut Expr,
) {
    unsafe {
        if (*pParse).nErr == 0 as ::core::ffi::c_int && !pWhere.is_null()
            && !((*pWhere).flags
                & (0x1 as ::core::ffi::c_int | 0x2 as ::core::ffi::c_int) as u32_0
                != 0 as u32_0) && !(*p).pSrc.is_null() && (*(*p).pSrc).nSrc < BMS
        {
            if (*pWhere).op as ::core::ffi::c_int == TK_AND {
                let mut pRight: *mut Expr = (*pWhere).pRight;
                existsToJoin(pParse, p, (*pWhere).pLeft);
                existsToJoin(pParse, p, pRight);
            } else if (*pWhere).op as ::core::ffi::c_int == TK_EXISTS {
                let mut pSub: *mut Select = (*pWhere).x.pSelect;
                let mut pSubWhere: *mut Expr = (*pSub).pWhere;
                if (*(*pSub).pSrc).nSrc == 1 as ::core::ffi::c_int
                    && (*pSub).selFlags & SF_Aggregate as u32_0 == 0 as u32_0
                    && (*(&raw mut (*(*pSub).pSrc).a as *mut SrcItem)
                        .offset(0 as ::core::ffi::c_int as isize))
                        .fg
                        .isSubquery() == 0 && (*pSub).pLimit.is_null()
                    && (*pSub).pPrior.is_null()
                {
                    let mut db: *mut sqlite3 = (*pParse).db;
                    let mut aCsrMap: *mut ::core::ffi::c_int = sqlite3DbMallocZero(
                        db,
                        (((*pParse).nTab + 2 as ::core::ffi::c_int) as usize)
                            .wrapping_mul(
                                ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                            ) as u64_0,
                    ) as *mut ::core::ffi::c_int;
                    if aCsrMap.is_null() {
                        return;
                    }
                    *aCsrMap.offset(0 as ::core::ffi::c_int as isize) = (*pParse).nTab
                        + 1 as ::core::ffi::c_int;
                    renumberCursors(pParse, pSub, -1 as ::core::ffi::c_int, aCsrMap);
                    sqlite3DbFree(db, aCsrMap as *mut ::core::ffi::c_void);
                    memset(
                        pWhere as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ::core::mem::size_of::<Expr>() as size_t,
                    );
                    (*pWhere).op = TK_INTEGER as u8_0;
                    (*pWhere).u.iValue = 1 as ::core::ffi::c_int;
                    (*pWhere).flags = ((*pWhere).flags as ::core::ffi::c_uint
                        | 0x800 as ::core::ffi::c_int as u32_0 as ::core::ffi::c_uint)
                        as u32_0;
                    let ref mut c2rust_fresh21 = (*(&raw mut (*(*pSub).pSrc).a
                        as *mut SrcItem)
                        .offset(0 as ::core::ffi::c_int as isize))
                        .fg;
                    (*c2rust_fresh21)
                        .set_fromExists(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    let ref mut c2rust_fresh22 = (*(&raw mut (*(*pSub).pSrc).a
                        as *mut SrcItem)
                        .offset(0 as ::core::ffi::c_int as isize))
                        .fg
                        .jointype;
                    *c2rust_fresh22 = (*c2rust_fresh22 as ::core::ffi::c_int | JT_CROSS)
                        as u8_0;
                    (*p).pSrc = sqlite3SrcListAppendList(
                        pParse,
                        (*p).pSrc,
                        (*pSub).pSrc,
                    );
                    if !pSubWhere.is_null() {
                        (*p).pWhere = sqlite3PExpr(
                            pParse,
                            TK_AND,
                            (*p).pWhere,
                            pSubWhere,
                        );
                        (*pSub).pWhere = ::core::ptr::null_mut::<Expr>();
                    }
                    (*pSub).pSrc = ::core::ptr::null_mut::<SrcList>();
                    sqlite3ParserAddCleanup(
                        pParse,
                        Some(
                            sqlite3SelectDeleteGeneric
                                as unsafe extern "C" fn(
                                    *mut sqlite3,
                                    *mut ::core::ffi::c_void,
                                ) -> (),
                        ),
                        pSub as *mut ::core::ffi::c_void,
                    );
                    existsToJoin(pParse, p, pSubWhere);
                }
            }
        }
    }
}
unsafe extern "C" fn selectCheckOnClausesExpr(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCtx: *mut CheckOnCtx = (*pWalker).u.pCheckOnCtx as *mut CheckOnCtx;
        if (*pExpr).flags & 0x1 as ::core::ffi::c_int as u32_0 != 0 as u32_0
            || (*pExpr).flags & 0x2 as ::core::ffi::c_int as u32_0 != 0 as u32_0
                && (*(&raw mut (*(*pCtx).pSrc).a as *mut SrcItem)
                    .offset(0 as ::core::ffi::c_int as isize))
                    .fg
                    .jointype as ::core::ffi::c_int & JT_LTORJ != 0 as ::core::ffi::c_int
        {
            if (*pCtx).iJoin == 0 as ::core::ffi::c_int {
                (*pCtx).iJoin = (*pExpr).w.iJoin;
                sqlite3WalkExprNN(pWalker, pExpr);
                (*pCtx).iJoin = 0 as ::core::ffi::c_int;
                return WRC_Prune;
            }
        }
        if (*pExpr).op as ::core::ffi::c_int == TK_COLUMN {
            loop {
                let mut pSrc: *mut SrcList = (*pCtx).pSrc;
                let mut iTab: ::core::ffi::c_int = (*pExpr).iTable;
                if iTab
                    >= (*(&raw mut (*pSrc).a as *mut SrcItem)
                        .offset(0 as ::core::ffi::c_int as isize))
                        .iCursor
                    && iTab
                        <= (*(&raw mut (*pSrc).a as *mut SrcItem)
                            .offset(((*pSrc).nSrc - 1 as ::core::ffi::c_int) as isize))
                            .iCursor
                {
                    if (*pCtx).iJoin != 0 && iTab > (*pCtx).iJoin {
                        sqlite3ErrorMsg(
                            (*pWalker).pParse,
                            b"ON clause references tables to its right\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        return WRC_Abort;
                    }
                    break;
                } else {
                    pCtx = (*pCtx).pParent;
                    if pCtx.is_null() {
                        break;
                    }
                }
            }
        }
        return WRC_Continue;
    }
}
unsafe extern "C" fn selectCheckOnClausesSelect(
    mut pWalker: *mut Walker,
    mut pSelect: *mut Select,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCtx: *mut CheckOnCtx = (*pWalker).u.pCheckOnCtx as *mut CheckOnCtx;
        if (*pSelect).pSrc == (*pCtx).pSrc
            || (*(*pSelect).pSrc).nSrc == 0 as ::core::ffi::c_int
        {
            return WRC_Continue
        } else {
            let mut sCtx: CheckOnCtx = CheckOnCtx {
                pSrc: ::core::ptr::null_mut::<SrcList>(),
                iJoin: 0,
                pParent: ::core::ptr::null_mut::<CheckOnCtx>()

            };
            memset(
                &raw mut sCtx as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<CheckOnCtx>() as size_t,
            );
            sCtx.pSrc = (*pSelect).pSrc;
            sCtx.pParent = pCtx;
            (*pWalker).u.pCheckOnCtx = &raw mut sCtx as *mut CheckOnCtx;
            sqlite3WalkSelect(pWalker, pSelect);
            (*pWalker).u.pCheckOnCtx = pCtx as *mut CheckOnCtx;
            (*pSelect).selFlags = ((*pSelect).selFlags as ::core::ffi::c_uint
                & !SF_OnToWhere as ::core::ffi::c_uint) as u32_0;
            return WRC_Prune;
        };
    }
}
unsafe extern "C" fn selectCheckOnClauses(
    mut pParse: *mut Parse,
    mut pSelect: *mut Select,
) {
    unsafe {
        let mut w: Walker = Walker {
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

        };
        let mut sCtx: CheckOnCtx = CheckOnCtx {
            pSrc: ::core::ptr::null_mut::<SrcList>(),
            iJoin: 0,
            pParent: ::core::ptr::null_mut::<CheckOnCtx>()

        };
        memset(
            &raw mut w as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<Walker>() as size_t,
        );
        w.pParse = pParse;
        w.xExprCallback = Some(
            selectCheckOnClausesExpr
                as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int>;
        w.xSelectCallback = Some(
            selectCheckOnClausesSelect
                as unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
            >;
        w.u.pCheckOnCtx = &raw mut sCtx as *mut CheckOnCtx;
        memset(
            &raw mut sCtx as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<CheckOnCtx>() as size_t,
        );
        sCtx.pSrc = (*pSelect).pSrc;
        sqlite3WalkExprNN(&raw mut w, (*pSelect).pWhere);
        (*pSelect).selFlags = ((*pSelect).selFlags as ::core::ffi::c_uint
            & !SF_OnToWhere as ::core::ffi::c_uint) as u32_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Select(
    mut pParse: *mut Parse,
    mut p: *mut Select,
    mut pDest: *mut SelectDest,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut pWInfo: *mut WhereInfo = ::core::ptr::null_mut::<WhereInfo>();
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        let mut isAgg: ::core::ffi::c_int = 0;
        let mut pEList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        let mut pTabList: *mut SrcList = ::core::ptr::null_mut::<SrcList>();
        let mut pWhere: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut pGroupBy: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        let mut pHaving: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut pAggInfo: *mut AggInfo = ::core::ptr::null_mut::<AggInfo>();
        let mut rc: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut sDistinct: DistinctCtx = DistinctCtx {
            isTnct: 0,
            eTnctType: 0,
            tabTnct: 0,
            addrTnct: 0

        };
        let mut sSort: SortCtx = SortCtx {
            pOrderBy: ::core::ptr::null_mut::<ExprList>(),
            nOBSat: 0,
            iECursor: 0,
            regReturn: 0,
            labelBkOut: 0,
            addrSortIndex: 0,
            labelDone: 0,
            labelOBLopt: 0,
            sortFlags: 0,
            pDeferredRowLoad: ::core::ptr::null_mut::<RowLoadInfo>()

        };
        let mut iEnd: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut pMinMaxOrderBy: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        let mut minMaxFlag: u8_0 = 0;
        db = (*pParse).db;
        v = sqlite3GetVdbe(pParse);
        if p.is_null() || (*pParse).nErr != 0 {
            return 1 as ::core::ffi::c_int;
        }
        if sqlite3AuthCheck(
            pParse,
            SQLITE_SELECT,
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ) != 0
        {
            return 1 as ::core::ffi::c_int;
        }
        if (*pDest).eDest as ::core::ffi::c_int <= SRT_DistQueue {
            if !(*p).pOrderBy.is_null() {
                sqlite3ParserAddCleanup(
                    pParse,
                    Some(
                        sqlite3ExprListDeleteGeneric
                            as unsafe extern "C" fn(
                                *mut sqlite3,
                                *mut ::core::ffi::c_void,
                            ) -> (),
                    ),
                    (*p).pOrderBy as *mut ::core::ffi::c_void,
                );
                (*p).pOrderBy = ::core::ptr::null_mut::<ExprList>();
            }
            (*p).selFlags = ((*p).selFlags as ::core::ffi::c_uint
                & !(SF_Distinct as u32_0) as ::core::ffi::c_uint) as u32_0;
            (*p).selFlags = ((*p).selFlags as ::core::ffi::c_uint
                | SF_NoopOrderBy as ::core::ffi::c_uint) as u32_0;
        }
        sqlite3SelectPrep(pParse, p, ::core::ptr::null_mut::<NameContext>());
        if !((*pParse).nErr != 0) {
            if (*p).selFlags & SF_OnToWhere as u32_0 != 0 {
                selectCheckOnClauses(pParse, p);
                if (*pParse).nErr != 0 {
                    c2rust_current_block = 9427106068226466434;
                } else {
                    c2rust_current_block = 1608152415753874203;
                }
            } else {
                c2rust_current_block = 1608152415753874203;
            }
            match c2rust_current_block {
                9427106068226466434 => {}
                _ => {
                    if (*p).selFlags & SF_UFSrcCheck as u32_0 != 0 {
                        let mut p0: *mut SrcItem = (&raw mut (*(*p).pSrc).a
                            as *mut SrcItem)
                            .offset(0 as ::core::ffi::c_int as isize) as *mut SrcItem;
                        if sameSrcAlias(p0, (*p).pSrc) != 0 {
                            sqlite3ErrorMsg(
                                pParse,
                                b"target object/alias may not appear in FROM clause: %s\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                if !(*p0).zAlias.is_null() {
                                    (*p0).zAlias
                                } else {
                                    (*(*p0).pSTab).zName
                                },
                            );
                            c2rust_current_block = 9427106068226466434;
                        } else {
                            (*p).selFlags = ((*p).selFlags as ::core::ffi::c_uint
                                & !(SF_UFSrcCheck as u32_0) as ::core::ffi::c_uint)
                                as u32_0;
                            c2rust_current_block = 11913429853522160501;
                        }
                    } else {
                        c2rust_current_block = 11913429853522160501;
                    }
                    match c2rust_current_block {
                        9427106068226466434 => {}
                        _ => {
                            if (*pDest).eDest as ::core::ffi::c_int == SRT_Output {
                                sqlite3GenerateColumnNames(pParse, p);
                            }
                            if !(sqlite3WindowRewrite(pParse, p) != 0) {
                                pTabList = (*p).pSrc;
                                isAgg = ((*p).selFlags & SF_Aggregate as u32_0
                                    != 0 as u32_0) as ::core::ffi::c_int;
                                memset(
                                    &raw mut sSort as *mut ::core::ffi::c_void,
                                    0 as ::core::ffi::c_int,
                                    ::core::mem::size_of::<SortCtx>() as size_t,
                                );
                                sSort.pOrderBy = (*p).pOrderBy;
                                i = 0 as ::core::ffi::c_int;
                                loop {
                                    if !((*p).pPrior.is_null() && i < (*pTabList).nSrc) {
                                        c2rust_current_block = 1852451392920375136;
                                        break;
                                    }
                                    let mut pItem: *mut SrcItem = (&raw mut (*pTabList).a
                                        as *mut SrcItem)
                                        .offset(i as isize) as *mut SrcItem;
                                    let mut pSub: *mut Select = if (*pItem).fg.isSubquery()
                                        as ::core::ffi::c_int != 0
                                    {
                                        (*(*pItem).u4.pSubq).pSelect
                                    } else {
                                        ::core::ptr::null_mut::<Select>()
                                    };
                                    let mut pTab: *mut Table = (*pItem).pSTab;
                                    if (*pItem).fg.jointype as ::core::ffi::c_int
                                        & (JT_LEFT | JT_LTORJ) != 0 as ::core::ffi::c_int
                                        && sqlite3ExprImpliesNonNullRow(
                                            (*p).pWhere,
                                            (*pItem).iCursor,
                                            (*pItem).fg.jointype as ::core::ffi::c_int & JT_LTORJ,
                                        ) != 0 && (*db).dbOptFlags & 0x2000 as u32_0 == 0 as u32_0
                                    {
                                        if (*pItem).fg.jointype as ::core::ffi::c_int & JT_LEFT != 0
                                        {
                                            if (*pItem).fg.jointype as ::core::ffi::c_int & JT_RIGHT
                                                != 0
                                            {
                                                (*pItem).fg.jointype = ((*pItem).fg.jointype
                                                    as ::core::ffi::c_int & !JT_LEFT) as u8_0;
                                            } else {
                                                (*pItem).fg.jointype = ((*pItem).fg.jointype
                                                    as ::core::ffi::c_int & !(JT_LEFT | JT_OUTER)) as u8_0;
                                                unsetJoinExpr(
                                                    (*p).pWhere,
                                                    (*pItem).iCursor,
                                                    0 as ::core::ffi::c_int,
                                                );
                                            }
                                        }
                                        if (*pItem).fg.jointype as ::core::ffi::c_int & JT_LTORJ
                                            != 0
                                        {
                                            j = i + 1 as ::core::ffi::c_int;
                                            while j < (*pTabList).nSrc {
                                                let mut pI2: *mut SrcItem = (&raw mut (*pTabList).a
                                                    as *mut SrcItem)
                                                    .offset(j as isize) as *mut SrcItem;
                                                if (*pI2).fg.jointype as ::core::ffi::c_int & JT_RIGHT != 0
                                                {
                                                    if (*pI2).fg.jointype as ::core::ffi::c_int & JT_LEFT != 0 {
                                                        (*pI2).fg.jointype = ((*pI2).fg.jointype
                                                            as ::core::ffi::c_int & !JT_RIGHT) as u8_0;
                                                    } else {
                                                        (*pI2).fg.jointype = ((*pI2).fg.jointype
                                                            as ::core::ffi::c_int & !(JT_RIGHT | JT_OUTER)) as u8_0;
                                                        unsetJoinExpr(
                                                            (*p).pWhere,
                                                            (*pI2).iCursor,
                                                            1 as ::core::ffi::c_int,
                                                        );
                                                    }
                                                }
                                                j += 1;
                                            }
                                            j = (*pTabList).nSrc - 1 as ::core::ffi::c_int;
                                            while j >= 0 as ::core::ffi::c_int {
                                                let ref mut c2rust_fresh7 = (*(&raw mut (*pTabList).a
                                                    as *mut SrcItem)
                                                    .offset(j as isize))
                                                    .fg
                                                    .jointype;
                                                *c2rust_fresh7 = (*c2rust_fresh7 as ::core::ffi::c_int
                                                    & !JT_LTORJ) as u8_0;
                                                if (*(&raw mut (*pTabList).a as *mut SrcItem)
                                                    .offset(j as isize))
                                                    .fg
                                                    .jointype as ::core::ffi::c_int & JT_RIGHT != 0
                                                {
                                                    break;
                                                }
                                                j -= 1;
                                            }
                                        }
                                    }
                                    if !pSub.is_null() {
                                        if (*pTab).nCol as ::core::ffi::c_int
                                            != (*(*pSub).pEList).nExpr
                                        {
                                            sqlite3ErrorMsg(
                                                pParse,
                                                b"expected %d columns for '%s' but got %d\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                (*pTab).nCol as ::core::ffi::c_int,
                                                (*pTab).zName,
                                                (*(*pSub).pEList).nExpr,
                                            );
                                            c2rust_current_block = 9427106068226466434;
                                            break;
                                        } else if !((*pItem).fg.isCte() as ::core::ffi::c_int != 0
                                            && (*(*pItem).u2.pCteUse).eM10d as ::core::ffi::c_int
                                                == M10d_Yes)
                                        {
                                            if !((*pSub).selFlags & SF_Aggregate as u32_0 != 0 as u32_0)
                                            {
                                                if !(*pSub).pOrderBy.is_null()
                                                    && (!(*p).pOrderBy.is_null()
                                                        || (*pTabList).nSrc > 1 as ::core::ffi::c_int)
                                                    && (*pSub).pLimit.is_null()
                                                    && (*pSub).selFlags
                                                        & (SF_OrderByReqd | SF_Recursive) as u32_0 == 0 as u32_0
                                                    && (*p).selFlags & SF_OrderByReqd as u32_0 == 0 as u32_0
                                                    && (*db).dbOptFlags & 0x40000 as u32_0 == 0 as u32_0
                                                {
                                                    sqlite3ParserAddCleanup(
                                                        pParse,
                                                        Some(
                                                            sqlite3ExprListDeleteGeneric
                                                                as unsafe extern "C" fn(
                                                                    *mut sqlite3,
                                                                    *mut ::core::ffi::c_void,
                                                                ) -> (),
                                                        ),
                                                        (*pSub).pOrderBy as *mut ::core::ffi::c_void,
                                                    );
                                                    (*pSub).pOrderBy = ::core::ptr::null_mut::<ExprList>();
                                                }
                                                if !(!(*pSub).pOrderBy.is_null()
                                                    && i == 0 as ::core::ffi::c_int
                                                    && (*p).selFlags & SF_ComplexResult as u32_0 != 0 as u32_0
                                                    && ((*pTabList).nSrc == 1 as ::core::ffi::c_int
                                                        || (*(&raw mut (*pTabList).a as *mut SrcItem)
                                                            .offset(1 as ::core::ffi::c_int as isize))
                                                            .fg
                                                            .jointype as ::core::ffi::c_int & (JT_OUTER | JT_CROSS)
                                                            != 0 as ::core::ffi::c_int))
                                                {
                                                    if flattenSubquery(pParse, p, i, isAgg) != 0 {
                                                        if (*pParse).nErr != 0 {
                                                            c2rust_current_block = 9427106068226466434;
                                                            break;
                                                        }
                                                        i = -1 as ::core::ffi::c_int;
                                                    }
                                                    pTabList = (*p).pSrc;
                                                    if (*db).mallocFailed != 0 {
                                                        c2rust_current_block = 9427106068226466434;
                                                        break;
                                                    }
                                                    if !((*pDest).eDest as ::core::ffi::c_int <= SRT_Fifo) {
                                                        sSort.pOrderBy = (*p).pOrderBy;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    i += 1;
                                }
                                match c2rust_current_block {
                                    9427106068226466434 => {}
                                    _ => {
                                        if !(*p).pPrior.is_null() {
                                            rc = multiSelect(pParse, p, pDest);
                                            if (*p).pNext.is_null() {
                                                sqlite3VdbeExplainPop(pParse);
                                            }
                                            return rc;
                                        }
                                        if (*pParse).bHasExists as ::core::ffi::c_int != 0
                                            && (*db).dbOptFlags & 0x40000000 as u32_0 == 0 as u32_0
                                        {
                                            existsToJoin(pParse, p, (*p).pWhere);
                                            pTabList = (*p).pSrc;
                                        }
                                        !(*p).pWhere.is_null()
                                            && (*(*p).pWhere).op as ::core::ffi::c_int == TK_AND
                                            && (*db).dbOptFlags & 0x8000 as u32_0 == 0 as u32_0
                                            && propagateConstants(pParse, p) != 0;
                                        if (*db).dbOptFlags
                                            & (0x1 as ::core::ffi::c_int | 0x200 as ::core::ffi::c_int)
                                                as u32_0 == 0 as u32_0
                                            && countOfViewOptimization(pParse, p) != 0
                                        {
                                            if (*db).mallocFailed != 0 {
                                                c2rust_current_block = 9427106068226466434;
                                            } else {
                                                pTabList = (*p).pSrc;
                                                c2rust_current_block = 2220405792722996547;
                                            }
                                        } else {
                                            c2rust_current_block = 2220405792722996547;
                                        }
                                        match c2rust_current_block {
                                            9427106068226466434 => {}
                                            _ => {
                                                i = 0 as ::core::ffi::c_int;
                                                loop {
                                                    if !(i < (*pTabList).nSrc) {
                                                        c2rust_current_block = 10996290961880923853;
                                                        break;
                                                    }
                                                    let mut pItem_0: *mut SrcItem = (&raw mut (*pTabList).a
                                                        as *mut SrcItem)
                                                        .offset(i as isize) as *mut SrcItem;
                                                    let mut pPrior: *mut SrcItem = ::core::ptr::null_mut::<
                                                        SrcItem,
                                                    >();
                                                    let mut dest: SelectDest = SelectDest {
                                                        eDest: 0,
                                                        iSDParm: 0,
                                                        iSDParm2: 0,
                                                        iSdst: 0,
                                                        nSdst: 0,
                                                        zAffSdst: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                                        pOrderBy: ::core::ptr::null_mut::<ExprList>()

                                                    };
                                                    let mut pSubq: *mut Subquery = ::core::ptr::null_mut::<
                                                        Subquery,
                                                    >();
                                                    let mut pSub_0: *mut Select = ::core::ptr::null_mut::<
                                                        Select,
                                                    >();
                                                    let mut zSavedAuthContext: *const ::core::ffi::c_char = ::core::ptr::null::<
                                                        ::core::ffi::c_char,
                                                    >();
                                                    if (*pItem_0).colUsed == 0 as ::core::ffi::c_ulonglong
                                                        && !(*pItem_0).zName.is_null()
                                                    {
                                                        let mut zDb: *const ::core::ffi::c_char = ::core::ptr::null::<
                                                            ::core::ffi::c_char,
                                                        >();
                                                        if (*pItem_0).fg.fixedSchema() != 0 {
                                                            let mut iDb: ::core::ffi::c_int = sqlite3SchemaToIndex(
                                                                (*pParse).db,
                                                                (*pItem_0).u4.pSchema,
                                                            );
                                                            zDb = (*(*db).aDb.offset(iDb as isize)).zDbSName;
                                                        } else if (*pItem_0).fg.isSubquery() != 0 {
                                                            zDb = ::core::ptr::null::<::core::ffi::c_char>();
                                                        } else {
                                                            zDb = (*pItem_0).u4.zDatabase;
                                                        }
                                                        sqlite3AuthCheck(
                                                            pParse,
                                                            SQLITE_READ,
                                                            (*pItem_0).zName,
                                                            b"\0".as_ptr() as *const ::core::ffi::c_char,
                                                            zDb,
                                                        );
                                                    }
                                                    if !((*pItem_0).fg.isSubquery() as ::core::ffi::c_int
                                                        == 0 as ::core::ffi::c_int)
                                                    {
                                                        pSubq = (*pItem_0).u4.pSubq;
                                                        pSub_0 = (*pSubq).pSelect;
                                                        if !((*pSubq).addrFillSub != 0 as ::core::ffi::c_int) {
                                                            (*pParse).nHeight += sqlite3SelectExprHeight(p);
                                                            (*db).dbOptFlags & 0x1000 as u32_0 == 0 as u32_0
                                                                && ((*pItem_0).fg.isCte() as ::core::ffi::c_int
                                                                    == 0 as ::core::ffi::c_int
                                                                    || (*(*pItem_0).u2.pCteUse).eM10d as ::core::ffi::c_int
                                                                        != M10d_Yes
                                                                        && (*(*pItem_0).u2.pCteUse).nUse < 2 as ::core::ffi::c_int)
                                                                && pushDownWhereTerms(
                                                                    pParse,
                                                                    pSub_0,
                                                                    (*p).pWhere,
                                                                    pTabList,
                                                                    i,
                                                                ) != 0;
                                                            (*db).dbOptFlags & 0x4000000 as u32_0 == 0 as u32_0
                                                                && disableUnusedSubqueryResultColumns(pItem_0) != 0;
                                                            zSavedAuthContext = (*pParse).zAuthContext;
                                                            (*pParse).zAuthContext = (*pItem_0).zName;
                                                            if fromClauseTermCanBeCoroutine(
                                                                pParse,
                                                                pTabList,
                                                                i,
                                                                (*p).selFlags as ::core::ffi::c_int,
                                                            ) != 0
                                                            {
                                                                let mut addrTop: ::core::ffi::c_int = sqlite3VdbeCurrentAddr(
                                                                    v,
                                                                ) + 1 as ::core::ffi::c_int;
                                                                (*pParse).nMem += 1;
                                                                (*pSubq).regReturn = (*pParse).nMem;
                                                                sqlite3VdbeAddOp3(
                                                                    v,
                                                                    OP_InitCoroutine,
                                                                    (*pSubq).regReturn,
                                                                    0 as ::core::ffi::c_int,
                                                                    addrTop,
                                                                );
                                                                (*pSubq).addrFillSub = addrTop;
                                                                sqlite3SelectDestInit(
                                                                    &raw mut dest,
                                                                    SRT_Coroutine,
                                                                    (*pSubq).regReturn,
                                                                );
                                                                sqlite3VdbeExplain(
                                                                    pParse,
                                                                    1 as u8_0,
                                                                    b"CO-ROUTINE %!S\0".as_ptr() as *const ::core::ffi::c_char,
                                                                    pItem_0,
                                                                );
                                                                sqlite3Select(pParse, pSub_0, &raw mut dest);
                                                                (*(*pItem_0).pSTab).nRowLogEst = (*pSub_0).nSelectRow;
                                                                (*pItem_0)
                                                                    .fg
                                                                    .set_viaCoroutine(
                                                                        1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                                                    );
                                                                (*pSubq).regResult = dest.iSdst;
                                                                sqlite3VdbeEndCoroutine(v, (*pSubq).regReturn);
                                                                sqlite3VdbeJumpHere(v, addrTop - 1 as ::core::ffi::c_int);
                                                                sqlite3ClearTempRegCache(pParse);
                                                            } else if (*pItem_0).fg.isCte() as ::core::ffi::c_int != 0
                                                                && (*(*pItem_0).u2.pCteUse).addrM9e
                                                                    > 0 as ::core::ffi::c_int
                                                            {
                                                                let mut pCteUse: *mut CteUse = (*pItem_0).u2.pCteUse;
                                                                sqlite3VdbeAddOp2(
                                                                    v,
                                                                    OP_Gosub,
                                                                    (*pCteUse).regRtn,
                                                                    (*pCteUse).addrM9e,
                                                                );
                                                                if (*pItem_0).iCursor != (*pCteUse).iCur {
                                                                    sqlite3VdbeAddOp2(
                                                                        v,
                                                                        OP_OpenDup,
                                                                        (*pItem_0).iCursor,
                                                                        (*pCteUse).iCur,
                                                                    );
                                                                }
                                                                (*pSub_0).nSelectRow = (*pCteUse).nRowEst;
                                                            } else {
                                                                pPrior = isSelfJoinView(
                                                                    pTabList,
                                                                    pItem_0,
                                                                    0 as ::core::ffi::c_int,
                                                                    i,
                                                                );
                                                                if !pPrior.is_null() {
                                                                    let mut pPriorSubq: *mut Subquery = ::core::ptr::null_mut::<
                                                                        Subquery,
                                                                    >();
                                                                    pPriorSubq = (*pPrior).u4.pSubq;
                                                                    if (*pPriorSubq).addrFillSub != 0 {
                                                                        sqlite3VdbeAddOp2(
                                                                            v,
                                                                            OP_Gosub,
                                                                            (*pPriorSubq).regReturn,
                                                                            (*pPriorSubq).addrFillSub,
                                                                        );
                                                                    }
                                                                    sqlite3VdbeAddOp2(
                                                                        v,
                                                                        OP_OpenDup,
                                                                        (*pItem_0).iCursor,
                                                                        (*pPrior).iCursor,
                                                                    );
                                                                    (*pSub_0).nSelectRow = (*(*pPriorSubq).pSelect).nSelectRow;
                                                                } else {
                                                                    let mut topAddr: ::core::ffi::c_int = 0;
                                                                    let mut onceAddr: ::core::ffi::c_int = 0
                                                                        as ::core::ffi::c_int;
                                                                    (*pParse).nMem += 1;
                                                                    (*pSubq).regReturn = (*pParse).nMem;
                                                                    topAddr = sqlite3VdbeAddOp0(v, OP_Goto);
                                                                    (*pSubq).addrFillSub = topAddr + 1 as ::core::ffi::c_int;
                                                                    (*pItem_0)
                                                                        .fg
                                                                        .set_isMaterialized(
                                                                            1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                                                        );
                                                                    if (*pItem_0).fg.isCorrelated() as ::core::ffi::c_int
                                                                        == 0 as ::core::ffi::c_int
                                                                    {
                                                                        onceAddr = sqlite3VdbeAddOp0(v, OP_Once);
                                                                    }
                                                                    sqlite3SelectDestInit(
                                                                        &raw mut dest,
                                                                        SRT_EphemTab,
                                                                        (*pItem_0).iCursor,
                                                                    );
                                                                    sqlite3VdbeExplain(
                                                                        pParse,
                                                                        1 as u8_0,
                                                                        b"MATERIALIZE %!S\0".as_ptr() as *const ::core::ffi::c_char,
                                                                        pItem_0,
                                                                    );
                                                                    sqlite3Select(pParse, pSub_0, &raw mut dest);
                                                                    (*(*pItem_0).pSTab).nRowLogEst = (*pSub_0).nSelectRow;
                                                                    if onceAddr != 0 {
                                                                        sqlite3VdbeJumpHere(v, onceAddr);
                                                                    }
                                                                    sqlite3VdbeAddOp2(
                                                                        v,
                                                                        OP_Return,
                                                                        (*pSubq).regReturn,
                                                                        topAddr + 1 as ::core::ffi::c_int,
                                                                    );
                                                                    sqlite3VdbeJumpHere(v, topAddr);
                                                                    sqlite3ClearTempRegCache(pParse);
                                                                    if (*pItem_0).fg.isCte() as ::core::ffi::c_int != 0
                                                                        && (*pItem_0).fg.isCorrelated() as ::core::ffi::c_int
                                                                            == 0 as ::core::ffi::c_int
                                                                    {
                                                                        let mut pCteUse_0: *mut CteUse = (*pItem_0).u2.pCteUse;
                                                                        (*pCteUse_0).addrM9e = (*pSubq).addrFillSub;
                                                                        (*pCteUse_0).regRtn = (*pSubq).regReturn;
                                                                        (*pCteUse_0).iCur = (*pItem_0).iCursor;
                                                                        (*pCteUse_0).nRowEst = (*pSub_0).nSelectRow;
                                                                    }
                                                                }
                                                            }
                                                            if (*db).mallocFailed != 0 {
                                                                c2rust_current_block = 9427106068226466434;
                                                                break;
                                                            }
                                                            (*pParse).nHeight -= sqlite3SelectExprHeight(p);
                                                            (*pParse).zAuthContext = zSavedAuthContext;
                                                        }
                                                    }
                                                    i += 1;
                                                }
                                                match c2rust_current_block {
                                                    9427106068226466434 => {}
                                                    _ => {
                                                        pEList = (*p).pEList;
                                                        pWhere = (*p).pWhere;
                                                        pGroupBy = (*p).pGroupBy;
                                                        pHaving = (*p).pHaving;
                                                        sDistinct.isTnct = ((*p).selFlags & SF_Distinct as u32_0
                                                            != 0 as u32_0) as ::core::ffi::c_int as u8_0;
                                                        if (*p).selFlags & (SF_Distinct | SF_Aggregate) as u32_0
                                                            == SF_Distinct as u32_0
                                                            && sqlite3ExprListCompare(
                                                                sSort.pOrderBy,
                                                                pEList,
                                                                -1 as ::core::ffi::c_int,
                                                            ) == 0 as ::core::ffi::c_int
                                                            && (*db).dbOptFlags & 0x4 as u32_0 == 0 as u32_0
                                                            && (*p).pWin.is_null()
                                                        {
                                                            (*p).selFlags = ((*p).selFlags as ::core::ffi::c_uint
                                                                & !(SF_Distinct as u32_0) as ::core::ffi::c_uint) as u32_0;
                                                            (*p).pGroupBy = sqlite3ExprListDup(
                                                                db,
                                                                pEList,
                                                                0 as ::core::ffi::c_int,
                                                            );
                                                            pGroupBy = (*p).pGroupBy;
                                                            if !pGroupBy.is_null() {
                                                                i = 0 as ::core::ffi::c_int;
                                                                while i < (*pGroupBy).nExpr {
                                                                    (*(&raw mut (*pGroupBy).a as *mut ExprList_item)
                                                                        .offset(i as isize))
                                                                        .u
                                                                        .x
                                                                        .iOrderByCol = (i + 1 as ::core::ffi::c_int) as u16_0;
                                                                    i += 1;
                                                                }
                                                            }
                                                            (*p).selFlags = ((*p).selFlags as ::core::ffi::c_uint
                                                                | SF_Aggregate as ::core::ffi::c_uint) as u32_0;
                                                            sDistinct.isTnct = 2 as u8_0;
                                                        }
                                                        if !sSort.pOrderBy.is_null() {
                                                            let mut pKeyInfo: *mut KeyInfo = ::core::ptr::null_mut::<
                                                                KeyInfo,
                                                            >();
                                                            pKeyInfo = sqlite3KeyInfoFromExprList(
                                                                pParse,
                                                                sSort.pOrderBy,
                                                                0 as ::core::ffi::c_int,
                                                                (*pEList).nExpr,
                                                            );
                                                            let c2rust_fresh8 = (*pParse).nTab;
                                                            (*pParse).nTab = (*pParse).nTab + 1;
                                                            sSort.iECursor = c2rust_fresh8;
                                                            sSort.addrSortIndex = sqlite3VdbeAddOp4(
                                                                v,
                                                                OP_OpenEphemeral,
                                                                sSort.iECursor,
                                                                (*sSort.pOrderBy).nExpr + 1 as ::core::ffi::c_int
                                                                    + (*pEList).nExpr,
                                                                0 as ::core::ffi::c_int,
                                                                pKeyInfo as *mut ::core::ffi::c_char,
                                                                P4_KEYINFO,
                                                            );
                                                        } else {
                                                            sSort.addrSortIndex = -1 as ::core::ffi::c_int;
                                                        }
                                                        if (*pDest).eDest as ::core::ffi::c_int == SRT_EphemTab {
                                                            sqlite3VdbeAddOp2(
                                                                v,
                                                                OP_OpenEphemeral,
                                                                (*pDest).iSDParm,
                                                                (*pEList).nExpr,
                                                            );
                                                            if (*p).selFlags & SF_NestedFrom as u32_0 != 0 {
                                                                let mut ii: ::core::ffi::c_int = 0;
                                                                ii = (*pEList).nExpr - 1 as ::core::ffi::c_int;
                                                                while ii > 0 as ::core::ffi::c_int
                                                                    && (*(&raw mut (*pEList).a as *mut ExprList_item)
                                                                        .offset(ii as isize))
                                                                        .fg
                                                                        .bUsed() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                                                {
                                                                    sqlite3ExprDelete(
                                                                        db,
                                                                        (*(&raw mut (*pEList).a as *mut ExprList_item)
                                                                            .offset(ii as isize))
                                                                            .pExpr,
                                                                    );
                                                                    sqlite3DbFree(
                                                                        db,
                                                                        (*(&raw mut (*pEList).a as *mut ExprList_item)
                                                                            .offset(ii as isize))
                                                                            .zEName as *mut ::core::ffi::c_void,
                                                                    );
                                                                    (*pEList).nExpr -= 1;
                                                                    ii -= 1;
                                                                }
                                                                ii = 0 as ::core::ffi::c_int;
                                                                while ii < (*pEList).nExpr {
                                                                    if (*(&raw mut (*pEList).a as *mut ExprList_item)
                                                                        .offset(ii as isize))
                                                                        .fg
                                                                        .bUsed() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                                                    {
                                                                        (*(*(&raw mut (*pEList).a as *mut ExprList_item)
                                                                            .offset(ii as isize))
                                                                            .pExpr)
                                                                            .op = TK_NULL as u8_0;
                                                                    }
                                                                    ii += 1;
                                                                }
                                                            }
                                                        }
                                                        iEnd = sqlite3VdbeMakeLabel(pParse);
                                                        if (*p).selFlags & SF_FixedLimit as u32_0 == 0 as u32_0 {
                                                            (*p).nSelectRow = 320 as LogEst;
                                                        }
                                                        if !(*p).pLimit.is_null() {
                                                            computeLimitRegisters(pParse, p, iEnd);
                                                        }
                                                        if (*p).iLimit == 0 as ::core::ffi::c_int
                                                            && sSort.addrSortIndex >= 0 as ::core::ffi::c_int
                                                        {
                                                            sqlite3VdbeChangeOpcode(
                                                                v,
                                                                sSort.addrSortIndex,
                                                                OP_SorterOpen as u8_0,
                                                            );
                                                            sSort.sortFlags = (sSort.sortFlags as ::core::ffi::c_int
                                                                | SORTFLAG_UseSorter) as u8_0;
                                                        }
                                                        if (*p).selFlags & SF_Distinct as u32_0 != 0 {
                                                            let c2rust_fresh9 = (*pParse).nTab;
                                                            (*pParse).nTab = (*pParse).nTab + 1;
                                                            sDistinct.tabTnct = c2rust_fresh9;
                                                            sDistinct.addrTnct = sqlite3VdbeAddOp4(
                                                                v,
                                                                OP_OpenEphemeral,
                                                                sDistinct.tabTnct,
                                                                0 as ::core::ffi::c_int,
                                                                0 as ::core::ffi::c_int,
                                                                sqlite3KeyInfoFromExprList(
                                                                    pParse,
                                                                    (*p).pEList,
                                                                    0 as ::core::ffi::c_int,
                                                                    0 as ::core::ffi::c_int,
                                                                ) as *mut ::core::ffi::c_char,
                                                                P4_KEYINFO,
                                                            );
                                                            sqlite3VdbeChangeP5(v, BTREE_UNORDERED as u16_0);
                                                            sDistinct.eTnctType = WHERE_DISTINCT_UNORDERED as u8_0;
                                                        } else {
                                                            sDistinct.eTnctType = WHERE_DISTINCT_NOOP as u8_0;
                                                        }
                                                        if isAgg == 0 && pGroupBy.is_null() {
                                                            let mut wctrlFlags: u16_0 = ((if sDistinct.isTnct
                                                                as ::core::ffi::c_int != 0
                                                            {
                                                                WHERE_WANT_DISTINCT
                                                            } else {
                                                                0 as ::core::ffi::c_int
                                                            }) as u32_0 | (*p).selFlags & SF_FixedLimit as u32_0)
                                                                as u16_0;
                                                            let mut pWin: *mut Window = (*p).pWin;
                                                            if !pWin.is_null() {
                                                                sqlite3WindowCodeInit(pParse, p);
                                                            }
                                                            pWInfo = sqlite3WhereBegin(
                                                                pParse,
                                                                pTabList,
                                                                pWhere,
                                                                sSort.pOrderBy,
                                                                (*p).pEList,
                                                                p,
                                                                wctrlFlags,
                                                                (*p).nSelectRow as ::core::ffi::c_int,
                                                            );
                                                            if pWInfo.is_null() {
                                                                c2rust_current_block = 9427106068226466434;
                                                            } else {
                                                                if (sqlite3WhereOutputRowCount(pWInfo)
                                                                    as ::core::ffi::c_int)
                                                                    < (*p).nSelectRow as ::core::ffi::c_int
                                                                {
                                                                    (*p).nSelectRow = sqlite3WhereOutputRowCount(pWInfo);
                                                                    if (*pDest).eDest as ::core::ffi::c_int <= SRT_DistQueue
                                                                        && (*pDest).eDest as ::core::ffi::c_int >= SRT_DistFifo
                                                                    {
                                                                        (*p).nSelectRow = ((*p).nSelectRow as ::core::ffi::c_int
                                                                            - 30 as ::core::ffi::c_int) as LogEst;
                                                                    }
                                                                }
                                                                if sDistinct.isTnct as ::core::ffi::c_int != 0
                                                                    && sqlite3WhereIsDistinct(pWInfo) != 0
                                                                {
                                                                    sDistinct.eTnctType = sqlite3WhereIsDistinct(pWInfo)
                                                                        as u8_0;
                                                                }
                                                                if !sSort.pOrderBy.is_null() {
                                                                    sSort.nOBSat = sqlite3WhereIsOrdered(pWInfo);
                                                                    sSort.labelOBLopt = sqlite3WhereOrderByLimitOptLabel(
                                                                        pWInfo,
                                                                    );
                                                                    if sSort.nOBSat == (*sSort.pOrderBy).nExpr {
                                                                        sSort.pOrderBy = ::core::ptr::null_mut::<ExprList>();
                                                                    }
                                                                }
                                                                if sSort.addrSortIndex >= 0 as ::core::ffi::c_int
                                                                    && sSort.pOrderBy.is_null()
                                                                {
                                                                    sqlite3VdbeChangeToNoop(v, sSort.addrSortIndex);
                                                                }
                                                                if !pWin.is_null() {
                                                                    let mut addrGosub: ::core::ffi::c_int = sqlite3VdbeMakeLabel(
                                                                        pParse,
                                                                    );
                                                                    let mut iCont: ::core::ffi::c_int = sqlite3VdbeMakeLabel(
                                                                        pParse,
                                                                    );
                                                                    let mut iBreak: ::core::ffi::c_int = sqlite3VdbeMakeLabel(
                                                                        pParse,
                                                                    );
                                                                    (*pParse).nMem += 1;
                                                                    let mut regGosub: ::core::ffi::c_int = (*pParse).nMem;
                                                                    sqlite3WindowCodeStep(
                                                                        pParse,
                                                                        p,
                                                                        pWInfo,
                                                                        regGosub,
                                                                        addrGosub,
                                                                    );
                                                                    sqlite3VdbeAddOp2(
                                                                        v,
                                                                        OP_Goto,
                                                                        0 as ::core::ffi::c_int,
                                                                        iBreak,
                                                                    );
                                                                    sqlite3VdbeResolveLabel(v, addrGosub);
                                                                    sSort.labelOBLopt = 0 as ::core::ffi::c_int;
                                                                    selectInnerLoop(
                                                                        pParse,
                                                                        p,
                                                                        -1 as ::core::ffi::c_int,
                                                                        &raw mut sSort,
                                                                        &raw mut sDistinct,
                                                                        pDest,
                                                                        iCont,
                                                                        iBreak,
                                                                    );
                                                                    sqlite3VdbeResolveLabel(v, iCont);
                                                                    sqlite3VdbeAddOp1(v, OP_Return, regGosub);
                                                                    sqlite3VdbeResolveLabel(v, iBreak);
                                                                } else {
                                                                    selectInnerLoop(
                                                                        pParse,
                                                                        p,
                                                                        -1 as ::core::ffi::c_int,
                                                                        &raw mut sSort,
                                                                        &raw mut sDistinct,
                                                                        pDest,
                                                                        sqlite3WhereContinueLabel(pWInfo),
                                                                        sqlite3WhereBreakLabel(pWInfo),
                                                                    );
                                                                    sqlite3WhereEnd(pWInfo);
                                                                }
                                                                c2rust_current_block = 6198430992841073810;
                                                            }
                                                        } else {
                                                            let mut sNC: NameContext = NameContext {
                                                                pParse: ::core::ptr::null_mut::<Parse>(),
                                                                pSrcList: ::core::ptr::null_mut::<SrcList>(),
                                                                uNC: C2Rust_Unnamed_23 {
                                                                    pEList: ::core::ptr::null_mut::<ExprList>()

                                                                },
                                                                pNext: ::core::ptr::null_mut::<NameContext>(),
                                                                nRef: 0,
                                                                nNcErr: 0,
                                                                ncFlags: 0,
                                                                nNestedSelect: 0,
                                                                pWinSelect: ::core::ptr::null_mut::<Select>()

                                                            };
                                                            let mut iAMem: ::core::ffi::c_int = 0;
                                                            let mut iBMem: ::core::ffi::c_int = 0;
                                                            let mut iUseFlag: ::core::ffi::c_int = 0;
                                                            let mut iAbortFlag: ::core::ffi::c_int = 0;
                                                            let mut groupBySort: ::core::ffi::c_int = 0;
                                                            let mut addrEnd: ::core::ffi::c_int = 0;
                                                            let mut sortPTab: ::core::ffi::c_int = 0
                                                                as ::core::ffi::c_int;
                                                            let mut sortOut: ::core::ffi::c_int = 0
                                                                as ::core::ffi::c_int;
                                                            let mut orderByGrp: ::core::ffi::c_int = 0
                                                                as ::core::ffi::c_int;
                                                            if !pGroupBy.is_null() {
                                                                let mut k: ::core::ffi::c_int = 0;
                                                                let mut pItem_1: *mut ExprList_item = ::core::ptr::null_mut::<
                                                                    ExprList_item,
                                                                >();
                                                                k = (*(*p).pEList).nExpr;
                                                                pItem_1 = &raw mut (*(*p).pEList).a as *mut ExprList_item
                                                                    as *mut ExprList_item;
                                                                while k > 0 as ::core::ffi::c_int {
                                                                    (*pItem_1).u.x.iAlias = 0 as u16_0;
                                                                    k -= 1;
                                                                    pItem_1 = pItem_1.offset(1);
                                                                }
                                                                k = (*pGroupBy).nExpr;
                                                                pItem_1 = &raw mut (*pGroupBy).a as *mut ExprList_item
                                                                    as *mut ExprList_item;
                                                                while k > 0 as ::core::ffi::c_int {
                                                                    (*pItem_1).u.x.iAlias = 0 as u16_0;
                                                                    k -= 1;
                                                                    pItem_1 = pItem_1.offset(1);
                                                                }
                                                                if (*p).nSelectRow as ::core::ffi::c_int
                                                                    > 66 as ::core::ffi::c_int
                                                                {
                                                                    (*p).nSelectRow = 66 as LogEst;
                                                                }
                                                                if !sSort.pOrderBy.is_null()
                                                                    && (*pGroupBy).nExpr == (*sSort.pOrderBy).nExpr
                                                                {
                                                                    let mut ii_0: ::core::ffi::c_int = 0;
                                                                    ii_0 = 0 as ::core::ffi::c_int;
                                                                    while ii_0 < (*pGroupBy).nExpr {
                                                                        let mut sortFlags: u8_0 = 0;
                                                                        sortFlags = ((*(&raw mut (*sSort.pOrderBy).a
                                                                            as *mut ExprList_item)
                                                                            .offset(ii_0 as isize))
                                                                            .fg
                                                                            .sortFlags as ::core::ffi::c_int & KEYINFO_ORDER_DESC)
                                                                            as u8_0;
                                                                        (*(&raw mut (*pGroupBy).a as *mut ExprList_item)
                                                                            .offset(ii_0 as isize))
                                                                            .fg
                                                                            .sortFlags = sortFlags;
                                                                        ii_0 += 1;
                                                                    }
                                                                    if sqlite3ExprListCompare(
                                                                        pGroupBy,
                                                                        sSort.pOrderBy,
                                                                        -1 as ::core::ffi::c_int,
                                                                    ) == 0 as ::core::ffi::c_int
                                                                    {
                                                                        orderByGrp = 1 as ::core::ffi::c_int;
                                                                    }
                                                                }
                                                            } else {
                                                                (*p).nSelectRow = 0 as LogEst;
                                                            }
                                                            addrEnd = sqlite3VdbeMakeLabel(pParse);
                                                            pAggInfo = sqlite3DbMallocZero(
                                                                db,
                                                                ::core::mem::size_of::<AggInfo>() as u64_0,
                                                            ) as *mut AggInfo;
                                                            if !pAggInfo.is_null() {
                                                                sqlite3ParserAddCleanup(
                                                                    pParse,
                                                                    Some(
                                                                        agginfoFree
                                                                            as unsafe extern "C" fn(
                                                                                *mut sqlite3,
                                                                                *mut ::core::ffi::c_void,
                                                                            ) -> (),
                                                                    ),
                                                                    pAggInfo as *mut ::core::ffi::c_void,
                                                                );
                                                            }
                                                            if (*db).mallocFailed != 0 {
                                                                c2rust_current_block = 9427106068226466434;
                                                            } else {
                                                                (*pAggInfo).selId = (*p).selId;
                                                                memset(
                                                                    &raw mut sNC as *mut ::core::ffi::c_void,
                                                                    0 as ::core::ffi::c_int,
                                                                    ::core::mem::size_of::<NameContext>() as size_t,
                                                                );
                                                                sNC.pParse = pParse;
                                                                sNC.pSrcList = pTabList;
                                                                sNC.uNC.pAggInfo = pAggInfo;
                                                                (*pAggInfo).nSortingColumn = (if !pGroupBy.is_null() {
                                                                    (*pGroupBy).nExpr
                                                                } else {
                                                                    0 as ::core::ffi::c_int
                                                                }) as u32_0;
                                                                (*pAggInfo).pGroupBy = pGroupBy;
                                                                sqlite3ExprAnalyzeAggList(&raw mut sNC, pEList);
                                                                sqlite3ExprAnalyzeAggList(&raw mut sNC, sSort.pOrderBy);
                                                                if !pHaving.is_null() {
                                                                    if !pGroupBy.is_null() {
                                                                        havingToWhere(pParse, p);
                                                                        pWhere = (*p).pWhere;
                                                                    }
                                                                    sqlite3ExprAnalyzeAggregates(&raw mut sNC, pHaving);
                                                                }
                                                                (*pAggInfo).nAccumulator = (*pAggInfo).nColumn;
                                                                if (*p).pGroupBy.is_null() && (*p).pHaving.is_null()
                                                                    && (*pAggInfo).nFunc == 1 as ::core::ffi::c_int
                                                                {
                                                                    minMaxFlag = minMaxQuery(
                                                                        db,
                                                                        (*(*pAggInfo)
                                                                            .aFunc
                                                                            .offset(0 as ::core::ffi::c_int as isize))
                                                                            .pFExpr,
                                                                        &raw mut pMinMaxOrderBy,
                                                                    );
                                                                } else {
                                                                    minMaxFlag = WHERE_ORDERBY_NORMAL as u8_0;
                                                                }
                                                                analyzeAggFuncArgs(pAggInfo, &raw mut sNC);
                                                                if (*db).mallocFailed != 0 {
                                                                    c2rust_current_block = 9427106068226466434;
                                                                } else {
                                                                    if !pGroupBy.is_null() {
                                                                        let mut pKeyInfo_0: *mut KeyInfo = ::core::ptr::null_mut::<
                                                                            KeyInfo,
                                                                        >();
                                                                        let mut addr1: ::core::ffi::c_int = 0;
                                                                        let mut addrOutputRow: ::core::ffi::c_int = 0;
                                                                        let mut regOutputRow: ::core::ffi::c_int = 0;
                                                                        let mut addrSetAbort: ::core::ffi::c_int = 0;
                                                                        let mut addrTopOfLoop: ::core::ffi::c_int = 0;
                                                                        let mut addrSortingIdx: ::core::ffi::c_int = 0;
                                                                        let mut addrReset: ::core::ffi::c_int = 0;
                                                                        let mut regReset: ::core::ffi::c_int = 0;
                                                                        let mut pDistinct: *mut ExprList = ::core::ptr::null_mut::<
                                                                            ExprList,
                                                                        >();
                                                                        let mut distFlag: u16_0 = 0 as u16_0;
                                                                        let mut eDist: ::core::ffi::c_int = WHERE_DISTINCT_NOOP;
                                                                        if (*pAggInfo).nFunc == 1 as ::core::ffi::c_int
                                                                            && (*(*pAggInfo)
                                                                                .aFunc
                                                                                .offset(0 as ::core::ffi::c_int as isize))
                                                                                .iDistinct >= 0 as ::core::ffi::c_int
                                                                            && !(*(*pAggInfo)
                                                                                .aFunc
                                                                                .offset(0 as ::core::ffi::c_int as isize))
                                                                                .pFExpr
                                                                                .is_null()
                                                                            && (*(*(*pAggInfo)
                                                                                .aFunc
                                                                                .offset(0 as ::core::ffi::c_int as isize))
                                                                                .pFExpr)
                                                                                .flags & 0x1000 as u32_0 == 0 as u32_0
                                                                            && !(*(*(*pAggInfo)
                                                                                .aFunc
                                                                                .offset(0 as ::core::ffi::c_int as isize))
                                                                                .pFExpr)
                                                                                .x
                                                                                .pList
                                                                                .is_null()
                                                                        {
                                                                            let mut pExpr: *mut Expr = (*(&raw mut (*(*(*(*pAggInfo)
                                                                                .aFunc
                                                                                .offset(0 as ::core::ffi::c_int as isize))
                                                                                .pFExpr)
                                                                                .x
                                                                                .pList)
                                                                                .a as *mut ExprList_item)
                                                                                .offset(0 as ::core::ffi::c_int as isize))
                                                                                .pExpr;
                                                                            pExpr = sqlite3ExprDup(db, pExpr, 0 as ::core::ffi::c_int);
                                                                            pDistinct = sqlite3ExprListDup(
                                                                                db,
                                                                                pGroupBy,
                                                                                0 as ::core::ffi::c_int,
                                                                            );
                                                                            pDistinct = sqlite3ExprListAppend(pParse, pDistinct, pExpr);
                                                                            distFlag = (if !pDistinct.is_null() {
                                                                                WHERE_WANT_DISTINCT | WHERE_AGG_DISTINCT
                                                                            } else {
                                                                                0 as ::core::ffi::c_int
                                                                            }) as u16_0;
                                                                        }
                                                                        let c2rust_fresh10 = (*pParse).nTab;
                                                                        (*pParse).nTab = (*pParse).nTab + 1;
                                                                        (*pAggInfo).sortingIdx = c2rust_fresh10;
                                                                        pKeyInfo_0 = sqlite3KeyInfoFromExprList(
                                                                            pParse,
                                                                            pGroupBy,
                                                                            0 as ::core::ffi::c_int,
                                                                            (*pAggInfo).nColumn,
                                                                        );
                                                                        addrSortingIdx = sqlite3VdbeAddOp4(
                                                                            v,
                                                                            OP_SorterOpen,
                                                                            (*pAggInfo).sortingIdx,
                                                                            (*pAggInfo).nSortingColumn as ::core::ffi::c_int,
                                                                            0 as ::core::ffi::c_int,
                                                                            pKeyInfo_0 as *mut ::core::ffi::c_char,
                                                                            P4_KEYINFO,
                                                                        );
                                                                        (*pParse).nMem += 1;
                                                                        iUseFlag = (*pParse).nMem;
                                                                        (*pParse).nMem += 1;
                                                                        iAbortFlag = (*pParse).nMem;
                                                                        (*pParse).nMem += 1;
                                                                        regOutputRow = (*pParse).nMem;
                                                                        addrOutputRow = sqlite3VdbeMakeLabel(pParse);
                                                                        (*pParse).nMem += 1;
                                                                        regReset = (*pParse).nMem;
                                                                        addrReset = sqlite3VdbeMakeLabel(pParse);
                                                                        iAMem = (*pParse).nMem + 1 as ::core::ffi::c_int;
                                                                        (*pParse).nMem += (*pGroupBy).nExpr;
                                                                        iBMem = (*pParse).nMem + 1 as ::core::ffi::c_int;
                                                                        (*pParse).nMem += (*pGroupBy).nExpr;
                                                                        sqlite3VdbeAddOp2(
                                                                            v,
                                                                            OP_Integer,
                                                                            0 as ::core::ffi::c_int,
                                                                            iAbortFlag,
                                                                        );
                                                                        sqlite3VdbeAddOp3(
                                                                            v,
                                                                            OP_Null,
                                                                            0 as ::core::ffi::c_int,
                                                                            iAMem,
                                                                            iAMem + (*pGroupBy).nExpr - 1 as ::core::ffi::c_int,
                                                                        );
                                                                        sqlite3ExprNullRegisterRange(
                                                                            pParse,
                                                                            iAMem,
                                                                            (*pGroupBy).nExpr,
                                                                        );
                                                                        sqlite3VdbeAddOp2(v, OP_Gosub, regReset, addrReset);
                                                                        pWInfo = sqlite3WhereBegin(
                                                                            pParse,
                                                                            pTabList,
                                                                            pWhere,
                                                                            pGroupBy,
                                                                            pDistinct,
                                                                            p,
                                                                            ((if sDistinct.isTnct as ::core::ffi::c_int
                                                                                == 2 as ::core::ffi::c_int
                                                                            {
                                                                                WHERE_DISTINCTBY
                                                                            } else {
                                                                                WHERE_GROUPBY
                                                                            })
                                                                                | (if orderByGrp != 0 {
                                                                                    WHERE_SORTBYGROUP
                                                                                } else {
                                                                                    0 as ::core::ffi::c_int
                                                                                }) | distFlag as ::core::ffi::c_int) as u16_0,
                                                                            0 as ::core::ffi::c_int,
                                                                        );
                                                                        if pWInfo.is_null() {
                                                                            sqlite3ExprListDelete(db, pDistinct);
                                                                            c2rust_current_block = 9427106068226466434;
                                                                        } else {
                                                                            if !(*pParse).pIdxEpr.is_null() {
                                                                                optimizeAggregateUseOfIndexedExpr(
                                                                                    pParse,
                                                                                    p,
                                                                                    pAggInfo,
                                                                                    &raw mut sNC,
                                                                                );
                                                                            }
                                                                            assignAggregateRegisters(pParse, pAggInfo);
                                                                            eDist = sqlite3WhereIsDistinct(pWInfo);
                                                                            if sqlite3WhereIsOrdered(pWInfo) == (*pGroupBy).nExpr {
                                                                                groupBySort = 0 as ::core::ffi::c_int;
                                                                            } else {
                                                                                let mut regBase: ::core::ffi::c_int = 0;
                                                                                let mut regRecord: ::core::ffi::c_int = 0;
                                                                                let mut nCol: ::core::ffi::c_int = 0;
                                                                                let mut nGroupBy: ::core::ffi::c_int = 0;
                                                                                sqlite3VdbeExplain(
                                                                                    pParse,
                                                                                    0 as u8_0,
                                                                                    b"USE TEMP B-TREE FOR %s\0".as_ptr()
                                                                                        as *const ::core::ffi::c_char,
                                                                                    if sDistinct.isTnct as ::core::ffi::c_int != 0
                                                                                        && (*p).selFlags & 0x1 as u32_0 == 0 as u32_0
                                                                                    {
                                                                                        b"DISTINCT\0".as_ptr() as *const ::core::ffi::c_char
                                                                                    } else {
                                                                                        b"GROUP BY\0".as_ptr() as *const ::core::ffi::c_char
                                                                                    },
                                                                                );
                                                                                groupBySort = 1 as ::core::ffi::c_int;
                                                                                nGroupBy = (*pGroupBy).nExpr;
                                                                                nCol = nGroupBy;
                                                                                j = nGroupBy;
                                                                                i = 0 as ::core::ffi::c_int;
                                                                                while i < (*pAggInfo).nColumn {
                                                                                    if (*(*pAggInfo).aCol.offset(i as isize)).iSorterColumn >= j
                                                                                    {
                                                                                        nCol += 1;
                                                                                        j += 1;
                                                                                    }
                                                                                    i += 1;
                                                                                }
                                                                                regBase = sqlite3GetTempRange(pParse, nCol);
                                                                                sqlite3ExprCodeExprList(
                                                                                    pParse,
                                                                                    pGroupBy,
                                                                                    regBase,
                                                                                    0 as ::core::ffi::c_int,
                                                                                    0 as u8_0,
                                                                                );
                                                                                j = nGroupBy;
                                                                                (*pAggInfo).directMode = 1 as u8_0;
                                                                                i = 0 as ::core::ffi::c_int;
                                                                                while i < (*pAggInfo).nColumn {
                                                                                    let mut pCol: *mut AggInfo_col = (*pAggInfo)
                                                                                        .aCol
                                                                                        .offset(i as isize) as *mut AggInfo_col;
                                                                                    if (*pCol).iSorterColumn >= j {
                                                                                        sqlite3ExprCode(pParse, (*pCol).pCExpr, j + regBase);
                                                                                        j += 1;
                                                                                    }
                                                                                    i += 1;
                                                                                }
                                                                                (*pAggInfo).directMode = 0 as u8_0;
                                                                                regRecord = sqlite3GetTempReg(pParse);
                                                                                sqlite3VdbeAddOp3(
                                                                                    v,
                                                                                    OP_MakeRecord,
                                                                                    regBase,
                                                                                    nCol,
                                                                                    regRecord,
                                                                                );
                                                                                sqlite3VdbeAddOp2(
                                                                                    v,
                                                                                    OP_SorterInsert,
                                                                                    (*pAggInfo).sortingIdx,
                                                                                    regRecord,
                                                                                );
                                                                                sqlite3ReleaseTempReg(pParse, regRecord);
                                                                                sqlite3ReleaseTempRange(pParse, regBase, nCol);
                                                                                sqlite3WhereEnd(pWInfo);
                                                                                let c2rust_fresh11 = (*pParse).nTab;
                                                                                (*pParse).nTab = (*pParse).nTab + 1;
                                                                                sortPTab = c2rust_fresh11;
                                                                                (*pAggInfo).sortingIdxPTab = sortPTab;
                                                                                sortOut = sqlite3GetTempReg(pParse);
                                                                                sqlite3VdbeAddOp3(
                                                                                    v,
                                                                                    OP_OpenPseudo,
                                                                                    sortPTab,
                                                                                    sortOut,
                                                                                    nCol,
                                                                                );
                                                                                sqlite3VdbeAddOp2(
                                                                                    v,
                                                                                    OP_SorterSort,
                                                                                    (*pAggInfo).sortingIdx,
                                                                                    addrEnd,
                                                                                );
                                                                                (*pAggInfo).useSortingIdx = 1 as u8_0;
                                                                            }
                                                                            if !(*pParse).pIdxEpr.is_null() {
                                                                                aggregateConvertIndexedExprRefToColumn(pAggInfo);
                                                                            }
                                                                            if orderByGrp != 0
                                                                                && (*db).dbOptFlags & 0x4 as u32_0 == 0 as u32_0
                                                                                && (groupBySort != 0 || sqlite3WhereIsSorted(pWInfo) != 0)
                                                                            {
                                                                                sSort.pOrderBy = ::core::ptr::null_mut::<ExprList>();
                                                                                sqlite3VdbeChangeToNoop(v, sSort.addrSortIndex);
                                                                            }
                                                                            addrTopOfLoop = sqlite3VdbeCurrentAddr(v);
                                                                            if groupBySort != 0 {
                                                                                sqlite3VdbeAddOp3(
                                                                                    v,
                                                                                    OP_SorterData,
                                                                                    (*pAggInfo).sortingIdx,
                                                                                    sortOut,
                                                                                    sortPTab,
                                                                                );
                                                                            }
                                                                            j = 0 as ::core::ffi::c_int;
                                                                            while j < (*pGroupBy).nExpr {
                                                                                let mut iOrderByCol: ::core::ffi::c_int = (*(&raw mut (*pGroupBy)
                                                                                    .a as *mut ExprList_item)
                                                                                    .offset(j as isize))
                                                                                    .u
                                                                                    .x
                                                                                    .iOrderByCol as ::core::ffi::c_int;
                                                                                if groupBySort != 0 {
                                                                                    sqlite3VdbeAddOp3(v, OP_Column, sortPTab, j, iBMem + j);
                                                                                } else {
                                                                                    (*pAggInfo).directMode = 1 as u8_0;
                                                                                    sqlite3ExprCode(
                                                                                        pParse,
                                                                                        (*(&raw mut (*pGroupBy).a as *mut ExprList_item)
                                                                                            .offset(j as isize))
                                                                                            .pExpr,
                                                                                        iBMem + j,
                                                                                    );
                                                                                }
                                                                                if iOrderByCol != 0 {
                                                                                    let mut pX: *mut Expr = (*(&raw mut (*(*p).pEList).a
                                                                                        as *mut ExprList_item)
                                                                                        .offset((iOrderByCol - 1 as ::core::ffi::c_int) as isize))
                                                                                        .pExpr;
                                                                                    let mut pBase: *mut Expr = sqlite3ExprSkipCollateAndLikely(
                                                                                        pX,
                                                                                    );
                                                                                    while !pBase.is_null()
                                                                                        && (*pBase).op as ::core::ffi::c_int == TK_IF_NULL_ROW
                                                                                    {
                                                                                        pX = (*pBase).pLeft;
                                                                                        pBase = sqlite3ExprSkipCollateAndLikely(pX);
                                                                                    }
                                                                                    if !pBase.is_null()
                                                                                        && (*pBase).op as ::core::ffi::c_int != TK_AGG_COLUMN
                                                                                        && (*pBase).op as ::core::ffi::c_int != TK_REGISTER
                                                                                    {
                                                                                        sqlite3ExprToRegister(pX, iAMem + j);
                                                                                    }
                                                                                }
                                                                                j += 1;
                                                                            }
                                                                            sqlite3VdbeAddOp4(
                                                                                v,
                                                                                OP_Compare,
                                                                                iAMem,
                                                                                iBMem,
                                                                                (*pGroupBy).nExpr,
                                                                                sqlite3KeyInfoRef(pKeyInfo_0) as *mut ::core::ffi::c_char,
                                                                                P4_KEYINFO,
                                                                            );
                                                                            addr1 = sqlite3VdbeCurrentAddr(v);
                                                                            sqlite3VdbeAddOp3(
                                                                                v,
                                                                                OP_Jump,
                                                                                addr1 + 1 as ::core::ffi::c_int,
                                                                                0 as ::core::ffi::c_int,
                                                                                addr1 + 1 as ::core::ffi::c_int,
                                                                            );
                                                                            sqlite3VdbeAddOp2(v, OP_Gosub, regOutputRow, addrOutputRow);
                                                                            sqlite3ExprCodeMove(
                                                                                pParse,
                                                                                iBMem,
                                                                                iAMem,
                                                                                (*pGroupBy).nExpr,
                                                                            );
                                                                            sqlite3VdbeAddOp2(v, OP_IfPos, iAbortFlag, addrEnd);
                                                                            sqlite3VdbeAddOp2(v, OP_Gosub, regReset, addrReset);
                                                                            sqlite3VdbeJumpHere(v, addr1);
                                                                            updateAccumulator(pParse, iUseFlag, pAggInfo, eDist);
                                                                            sqlite3VdbeAddOp2(
                                                                                v,
                                                                                OP_Integer,
                                                                                1 as ::core::ffi::c_int,
                                                                                iUseFlag,
                                                                            );
                                                                            if groupBySort != 0 {
                                                                                sqlite3VdbeAddOp2(
                                                                                    v,
                                                                                    OP_SorterNext,
                                                                                    (*pAggInfo).sortingIdx,
                                                                                    addrTopOfLoop,
                                                                                );
                                                                            } else {
                                                                                sqlite3WhereEnd(pWInfo);
                                                                                sqlite3VdbeChangeToNoop(v, addrSortingIdx);
                                                                            }
                                                                            sqlite3ExprListDelete(db, pDistinct);
                                                                            sqlite3VdbeAddOp2(v, OP_Gosub, regOutputRow, addrOutputRow);
                                                                            sqlite3VdbeGoto(v, addrEnd);
                                                                            addrSetAbort = sqlite3VdbeCurrentAddr(v);
                                                                            sqlite3VdbeAddOp2(
                                                                                v,
                                                                                OP_Integer,
                                                                                1 as ::core::ffi::c_int,
                                                                                iAbortFlag,
                                                                            );
                                                                            sqlite3VdbeAddOp1(v, OP_Return, regOutputRow);
                                                                            sqlite3VdbeResolveLabel(v, addrOutputRow);
                                                                            addrOutputRow = sqlite3VdbeCurrentAddr(v);
                                                                            sqlite3VdbeAddOp2(
                                                                                v,
                                                                                OP_IfPos,
                                                                                iUseFlag,
                                                                                addrOutputRow + 2 as ::core::ffi::c_int,
                                                                            );
                                                                            sqlite3VdbeAddOp1(v, OP_Return, regOutputRow);
                                                                            finalizeAggFunctions(pParse, pAggInfo);
                                                                            sqlite3ExprIfFalse(
                                                                                pParse,
                                                                                pHaving,
                                                                                addrOutputRow + 1 as ::core::ffi::c_int,
                                                                                SQLITE_JUMPIFNULL,
                                                                            );
                                                                            selectInnerLoop(
                                                                                pParse,
                                                                                p,
                                                                                -1 as ::core::ffi::c_int,
                                                                                &raw mut sSort,
                                                                                &raw mut sDistinct,
                                                                                pDest,
                                                                                addrOutputRow + 1 as ::core::ffi::c_int,
                                                                                addrSetAbort,
                                                                            );
                                                                            sqlite3VdbeAddOp1(v, OP_Return, regOutputRow);
                                                                            sqlite3VdbeResolveLabel(v, addrReset);
                                                                            resetAccumulator(pParse, pAggInfo);
                                                                            sqlite3VdbeAddOp2(
                                                                                v,
                                                                                OP_Integer,
                                                                                0 as ::core::ffi::c_int,
                                                                                iUseFlag,
                                                                            );
                                                                            sqlite3VdbeAddOp1(v, OP_Return, regReset);
                                                                            if distFlag as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                                                                                && eDist != WHERE_DISTINCT_NOOP
                                                                            {
                                                                                let mut pF: *mut AggInfo_func = (*pAggInfo)
                                                                                    .aFunc
                                                                                    .offset(0 as ::core::ffi::c_int as isize)
                                                                                    as *mut AggInfo_func;
                                                                                fixDistinctOpenEph(
                                                                                    pParse,
                                                                                    eDist,
                                                                                    (*pF).iDistinct,
                                                                                    (*pF).iDistAddr,
                                                                                );
                                                                            }
                                                                            c2rust_current_block = 2969779015960460938;
                                                                        }
                                                                    } else {
                                                                        let mut pTab_0: *mut Table = ::core::ptr::null_mut::<
                                                                            Table,
                                                                        >();
                                                                        pTab_0 = isSimpleCount(p, pAggInfo);
                                                                        if !pTab_0.is_null() {
                                                                            let iDb_0: ::core::ffi::c_int = sqlite3SchemaToIndex(
                                                                                (*pParse).db,
                                                                                (*pTab_0).pSchema,
                                                                            ) as ::core::ffi::c_int;
                                                                            let c2rust_fresh12 = (*pParse).nTab;
                                                                            (*pParse).nTab = (*pParse).nTab + 1;
                                                                            let iCsr: ::core::ffi::c_int = c2rust_fresh12;
                                                                            let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
                                                                            let mut pKeyInfo_1: *mut KeyInfo = ::core::ptr::null_mut::<
                                                                                KeyInfo,
                                                                            >();
                                                                            let mut pBest: *mut Index = ::core::ptr::null_mut::<
                                                                                Index,
                                                                            >();
                                                                            let mut iRoot: Pgno = (*pTab_0).tnum;
                                                                            sqlite3CodeVerifySchema(pParse, iDb_0);
                                                                            sqlite3TableLock(
                                                                                pParse,
                                                                                iDb_0,
                                                                                (*pTab_0).tnum,
                                                                                0 as u8_0,
                                                                                (*pTab_0).zName,
                                                                            );
                                                                            if !((*pTab_0).tabFlags & TF_WithoutRowid as u32_0
                                                                                == 0 as u32_0)
                                                                            {
                                                                                pBest = sqlite3PrimaryKeyIndex(pTab_0);
                                                                            }
                                                                            if (*(&raw mut (*(*p).pSrc).a as *mut SrcItem)
                                                                                .offset(0 as ::core::ffi::c_int as isize))
                                                                                .fg
                                                                                .notIndexed() == 0
                                                                            {
                                                                                pIdx = (*pTab_0).pIndex;
                                                                                while !pIdx.is_null() {
                                                                                    if (*pIdx).bUnordered() as ::core::ffi::c_int
                                                                                        == 0 as ::core::ffi::c_int
                                                                                        && ((*pIdx).szIdxRow as ::core::ffi::c_int)
                                                                                            < (*pTab_0).szTabRow as ::core::ffi::c_int
                                                                                        && (*pIdx).pPartIdxWhere.is_null()
                                                                                        && (pBest.is_null()
                                                                                            || ((*pIdx).szIdxRow as ::core::ffi::c_int)
                                                                                                < (*pBest).szIdxRow as ::core::ffi::c_int)
                                                                                    {
                                                                                        pBest = pIdx;
                                                                                    }
                                                                                    pIdx = (*pIdx).pNext;
                                                                                }
                                                                            }
                                                                            if !pBest.is_null() {
                                                                                iRoot = (*pBest).tnum;
                                                                                pKeyInfo_1 = sqlite3KeyInfoOfIndex(pParse, pBest);
                                                                            }
                                                                            sqlite3VdbeAddOp4Int(
                                                                                v,
                                                                                OP_OpenRead,
                                                                                iCsr,
                                                                                iRoot as ::core::ffi::c_int,
                                                                                iDb_0,
                                                                                1 as ::core::ffi::c_int,
                                                                            );
                                                                            if !pKeyInfo_1.is_null() {
                                                                                sqlite3VdbeChangeP4(
                                                                                    v,
                                                                                    -1 as ::core::ffi::c_int,
                                                                                    pKeyInfo_1 as *mut ::core::ffi::c_char,
                                                                                    P4_KEYINFO,
                                                                                );
                                                                            }
                                                                            assignAggregateRegisters(pParse, pAggInfo);
                                                                            sqlite3VdbeAddOp2(
                                                                                v,
                                                                                OP_Count,
                                                                                iCsr,
                                                                                (*pAggInfo).iFirstReg + (*pAggInfo).nColumn
                                                                                    + 0 as ::core::ffi::c_int,
                                                                            );
                                                                            sqlite3VdbeAddOp1(v, OP_Close, iCsr);
                                                                            explainSimpleCount(pParse, pTab_0, pBest);
                                                                            c2rust_current_block = 14953953536506160536;
                                                                        } else {
                                                                            let mut regAcc: ::core::ffi::c_int = 0
                                                                                as ::core::ffi::c_int;
                                                                            let mut pDistinct_0: *mut ExprList = ::core::ptr::null_mut::<
                                                                                ExprList,
                                                                            >();
                                                                            let mut distFlag_0: u16_0 = 0 as u16_0;
                                                                            let mut eDist_0: ::core::ffi::c_int = 0;
                                                                            if (*pAggInfo).nAccumulator != 0 {
                                                                                i = 0 as ::core::ffi::c_int;
                                                                                while i < (*pAggInfo).nFunc {
                                                                                    if !((*(*(*pAggInfo).aFunc.offset(i as isize)).pFExpr).flags
                                                                                        & 0x1000000 as ::core::ffi::c_int as u32_0 != 0 as u32_0)
                                                                                    {
                                                                                        if (*(*(*pAggInfo).aFunc.offset(i as isize)).pFunc)
                                                                                            .funcFlags & SQLITE_FUNC_NEEDCOLL as u32_0 != 0
                                                                                        {
                                                                                            break;
                                                                                        }
                                                                                    }
                                                                                    i += 1;
                                                                                }
                                                                                if i == (*pAggInfo).nFunc {
                                                                                    (*pParse).nMem += 1;
                                                                                    regAcc = (*pParse).nMem;
                                                                                    sqlite3VdbeAddOp2(
                                                                                        v,
                                                                                        OP_Integer,
                                                                                        0 as ::core::ffi::c_int,
                                                                                        regAcc,
                                                                                    );
                                                                                }
                                                                            } else if (*pAggInfo).nFunc == 1 as ::core::ffi::c_int
                                                                                && (*(*pAggInfo)
                                                                                    .aFunc
                                                                                    .offset(0 as ::core::ffi::c_int as isize))
                                                                                    .iDistinct >= 0 as ::core::ffi::c_int
                                                                            {
                                                                                pDistinct_0 = (*(*(*pAggInfo)
                                                                                    .aFunc
                                                                                    .offset(0 as ::core::ffi::c_int as isize))
                                                                                    .pFExpr)
                                                                                    .x
                                                                                    .pList;
                                                                                distFlag_0 = (if !pDistinct_0.is_null() {
                                                                                    WHERE_WANT_DISTINCT | WHERE_AGG_DISTINCT
                                                                                } else {
                                                                                    0 as ::core::ffi::c_int
                                                                                }) as u16_0;
                                                                            }
                                                                            assignAggregateRegisters(pParse, pAggInfo);
                                                                            resetAccumulator(pParse, pAggInfo);
                                                                            pWInfo = sqlite3WhereBegin(
                                                                                pParse,
                                                                                pTabList,
                                                                                pWhere,
                                                                                pMinMaxOrderBy,
                                                                                pDistinct_0,
                                                                                p,
                                                                                (minMaxFlag as ::core::ffi::c_int
                                                                                    | distFlag_0 as ::core::ffi::c_int) as u16_0,
                                                                                0 as ::core::ffi::c_int,
                                                                            );
                                                                            if pWInfo.is_null() {
                                                                                c2rust_current_block = 9427106068226466434;
                                                                            } else {
                                                                                eDist_0 = sqlite3WhereIsDistinct(pWInfo);
                                                                                updateAccumulator(pParse, regAcc, pAggInfo, eDist_0);
                                                                                if eDist_0 != WHERE_DISTINCT_NOOP {
                                                                                    let mut pF_0: *mut AggInfo_func = (*pAggInfo).aFunc
                                                                                        as *mut AggInfo_func;
                                                                                    if !pF_0.is_null() {
                                                                                        fixDistinctOpenEph(
                                                                                            pParse,
                                                                                            eDist_0,
                                                                                            (*pF_0).iDistinct,
                                                                                            (*pF_0).iDistAddr,
                                                                                        );
                                                                                    }
                                                                                }
                                                                                if regAcc != 0 {
                                                                                    sqlite3VdbeAddOp2(
                                                                                        v,
                                                                                        OP_Integer,
                                                                                        1 as ::core::ffi::c_int,
                                                                                        regAcc,
                                                                                    );
                                                                                }
                                                                                if minMaxFlag != 0 {
                                                                                    sqlite3WhereMinMaxOptEarlyOut(v, pWInfo);
                                                                                }
                                                                                sqlite3WhereEnd(pWInfo);
                                                                                finalizeAggFunctions(pParse, pAggInfo);
                                                                                c2rust_current_block = 14953953536506160536;
                                                                            }
                                                                        }
                                                                        match c2rust_current_block {
                                                                            9427106068226466434 => {}
                                                                            _ => {
                                                                                sSort.pOrderBy = ::core::ptr::null_mut::<ExprList>();
                                                                                sqlite3ExprIfFalse(
                                                                                    pParse,
                                                                                    pHaving,
                                                                                    addrEnd,
                                                                                    SQLITE_JUMPIFNULL,
                                                                                );
                                                                                selectInnerLoop(
                                                                                    pParse,
                                                                                    p,
                                                                                    -1 as ::core::ffi::c_int,
                                                                                    ::core::ptr::null_mut::<SortCtx>(),
                                                                                    ::core::ptr::null_mut::<DistinctCtx>(),
                                                                                    pDest,
                                                                                    addrEnd,
                                                                                    addrEnd,
                                                                                );
                                                                                c2rust_current_block = 2969779015960460938;
                                                                            }
                                                                        }
                                                                    }
                                                                    match c2rust_current_block {
                                                                        9427106068226466434 => {}
                                                                        _ => {
                                                                            sqlite3VdbeResolveLabel(v, addrEnd);
                                                                            c2rust_current_block = 6198430992841073810;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        match c2rust_current_block {
                                                            9427106068226466434 => {}
                                                            _ => {
                                                                if sDistinct.eTnctType as ::core::ffi::c_int
                                                                    == WHERE_DISTINCT_UNORDERED
                                                                {
                                                                    explainTempTable(
                                                                        pParse,
                                                                        b"DISTINCT\0".as_ptr() as *const ::core::ffi::c_char,
                                                                    );
                                                                }
                                                                if !sSort.pOrderBy.is_null() {
                                                                    generateSortTail(
                                                                        pParse,
                                                                        p,
                                                                        &raw mut sSort,
                                                                        (*pEList).nExpr,
                                                                        pDest,
                                                                    );
                                                                }
                                                                sqlite3VdbeResolveLabel(v, iEnd);
                                                                rc = ((*pParse).nErr > 0 as ::core::ffi::c_int)
                                                                    as ::core::ffi::c_int;
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
        sqlite3ExprListDelete(db, pMinMaxOrderBy);
        sqlite3VdbeExplainPop(pParse);
        return rc;
    }
}
pub const BMS: ::core::ffi::c_int = (::core::mem::size_of::<Bitmask>() as usize)
    .wrapping_mul(8 as usize) as ::core::ffi::c_int;
pub const TOPBIT: Bitmask = (1 as ::core::ffi::c_int as Bitmask)
    << BMS - 1 as ::core::ffi::c_int;
pub const BTREE_UNORDERED: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const P4_COLLSEQ: ::core::ffi::c_int = -2 as ::core::ffi::c_int;
pub const P4_FUNCDEF: ::core::ffi::c_int = -7 as ::core::ffi::c_int;
pub const P4_KEYINFO: ::core::ffi::c_int = -8 as ::core::ffi::c_int;
pub const P4_INTARRAY: ::core::ffi::c_int = -14 as ::core::ffi::c_int;
pub const COLNAME_NAME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const COLNAME_DECLTYPE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TK_EXISTS: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const TK_CAST: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const TK_AND: ::core::ffi::c_int = 44 as ::core::ffi::c_int;
pub const TK_IS: ::core::ffi::c_int = 45 as ::core::ffi::c_int;
pub const TK_EQ: ::core::ffi::c_int = 54 as ::core::ffi::c_int;
pub const TK_GE: ::core::ffi::c_int = 58 as ::core::ffi::c_int;
pub const TK_ID: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const TK_PLUS: ::core::ffi::c_int = 107 as ::core::ffi::c_int;
pub const TK_COLLATE: ::core::ffi::c_int = 114 as ::core::ffi::c_int;
pub const TK_NULL: ::core::ffi::c_int = 122 as ::core::ffi::c_int;
pub const TK_UNION: ::core::ffi::c_int = 135 as ::core::ffi::c_int;
pub const TK_ALL: ::core::ffi::c_int = 136 as ::core::ffi::c_int;
pub const TK_EXCEPT: ::core::ffi::c_int = 137;
pub const TK_INTERSECT: ::core::ffi::c_int = 138;
pub const TK_SELECT: ::core::ffi::c_int = 139;
pub const TK_DOT: ::core::ffi::c_int = 142 as ::core::ffi::c_int;
pub const TK_INTEGER: ::core::ffi::c_int = 156 as ::core::ffi::c_int;
pub const TK_COLUMN: ::core::ffi::c_int = 168;
pub const TK_AGG_FUNCTION: ::core::ffi::c_int = 169 as ::core::ffi::c_int;
pub const TK_AGG_COLUMN: ::core::ffi::c_int = 170 as ::core::ffi::c_int;
pub const TK_TRUEFALSE: ::core::ffi::c_int = 171 as ::core::ffi::c_int;
pub const TK_FUNCTION: ::core::ffi::c_int = 172 as ::core::ffi::c_int;
pub const TK_REGISTER: ::core::ffi::c_int = 176 as ::core::ffi::c_int;
pub const TK_IF_NULL_ROW: ::core::ffi::c_int = 179 as ::core::ffi::c_int;
pub const TK_ASTERISK: ::core::ffi::c_int = 180 as ::core::ffi::c_int;
pub const SQLITE_FullColNames: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SQLITE_ShortColNames: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SQLITE_TrustedSchema: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const SQLITE_EnableView: ::core::ffi::c_uint = 0x80000000 as ::core::ffi::c_uint;
pub const SQLITE_FUNC_NEEDCOLL: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const SQLITE_FUNC_COUNT: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const SQLITE_N_STDTYPE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const COLFLAG_HIDDEN: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const COLFLAG_HASTYPE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const COLFLAG_HASCOLL: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const COLFLAG_NOEXPAND: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const COLFLAG_NOINSERT: ::core::ffi::c_int = 0x62 as ::core::ffi::c_int;
pub const SQLITE_AFF_NONE: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SQLITE_AFF_BLOB: ::core::ffi::c_int = 0x41 as ::core::ffi::c_int;
pub const SQLITE_AFF_TEXT: ::core::ffi::c_int = 0x42 as ::core::ffi::c_int;
pub const SQLITE_AFF_NUMERIC: ::core::ffi::c_int = 0x43 as ::core::ffi::c_int;
pub const SQLITE_AFF_FLEXNUM: ::core::ffi::c_int = 0x46 as ::core::ffi::c_int;
pub const SQLITE_AFF_DEFER: ::core::ffi::c_int = 0x58 as ::core::ffi::c_int;
pub const SQLITE_JUMPIFNULL: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SQLITE_NULLEQ: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const TF_WithoutRowid: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const TF_NoVisibleRowid: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const TF_Ephemeral: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const TABTYP_NORM: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TABTYP_VTAB: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TABTYP_VIEW: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ViewCanHaveRowid: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const KEYINFO_ORDER_DESC: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const KEYINFO_ORDER_BIGNULL: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_IDXTYPE_PRIMARYKEY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EP_OuterON: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const EP_InnerON: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const EP_HasFunc: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const EP_Collate: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const EP_IntValue: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const EP_xIsSelect: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const EP_IfNullRow: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;
pub const EP_Subquery: ::core::ffi::c_int = 0x400000 as ::core::ffi::c_int;
pub const EP_IsFalse: ::core::ffi::c_int = 0x20000000 as ::core::ffi::c_int;
pub const ENAME_NAME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ENAME_TAB: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ENAME_ROWID: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SZ_SRCLIST_1: usize = (8 as usize)
    .wrapping_add(::core::mem::size_of::<SrcItem>() as usize);
pub const JT_INNER: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const JT_CROSS: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const JT_NATURAL: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const JT_LEFT: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const JT_RIGHT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const JT_OUTER: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const JT_LTORJ: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const JT_ERROR: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const WHERE_ORDERBY_NORMAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const WHERE_ORDERBY_MIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const WHERE_ORDERBY_MAX: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const WHERE_GROUPBY: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const WHERE_DISTINCTBY: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const WHERE_WANT_DISTINCT: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const WHERE_SORTBYGROUP: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const WHERE_AGG_DISTINCT: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const WHERE_DISTINCT_NOOP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const WHERE_DISTINCT_UNIQUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WHERE_DISTINCT_ORDERED: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const WHERE_DISTINCT_UNORDERED: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const NC_InAggFunc: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;
pub const SF_Distinct: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SF_Aggregate: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SF_UsesEphemeral: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const SF_Expanded: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SF_HasTypeInfo: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const SF_Compound: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const SF_Values: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const SF_MultiValue: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const SF_NestedFrom: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SF_Recursive: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const SF_FixedLimit: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const SF_Converted: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const SF_IncludeHidden: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;
pub const SF_ComplexResult: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;
pub const SF_View: ::core::ffi::c_int = 0x200000 as ::core::ffi::c_int;
pub const SF_NoopOrderBy: ::core::ffi::c_int = 0x400000 as ::core::ffi::c_int;
pub const SF_UFSrcCheck: ::core::ffi::c_int = 0x800000 as ::core::ffi::c_int;
pub const SF_PushDown: ::core::ffi::c_int = 0x1000000 as ::core::ffi::c_int;
pub const SF_MultiPart: ::core::ffi::c_int = 0x2000000 as ::core::ffi::c_int;
pub const SF_CopyCte: ::core::ffi::c_int = 0x4000000 as ::core::ffi::c_int;
pub const SF_OrderByReqd: ::core::ffi::c_int = 0x8000000 as ::core::ffi::c_int;
pub const SF_UpdateFrom: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
pub const SF_OnToWhere: ::core::ffi::c_int = 0x40000000 as ::core::ffi::c_int;
pub const SRT_Union: ::core::ffi::c_int = 1;
pub const SRT_Except: ::core::ffi::c_int = 2;
pub const SRT_Exists: ::core::ffi::c_int = 3;
pub const SRT_DistFifo: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SRT_DistQueue: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SRT_Queue: ::core::ffi::c_int = 7;
pub const SRT_Fifo: ::core::ffi::c_int = 8;
pub const SRT_Output: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SRT_Mem: ::core::ffi::c_int = 10;
pub const SRT_Set: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SRT_EphemTab: ::core::ffi::c_int = 12;
pub const SRT_Coroutine: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const SRT_Table: ::core::ffi::c_int = 14;
pub const SRT_Upfrom: ::core::ffi::c_int = 15;
pub const PARSE_MODE_RENAME: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OPFLAG_APPEND: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const OPFLAG_USESEEKRESULT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const OPFLAG_PERMUTE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const WRC_Continue: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const WRC_Prune: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WRC_Abort: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const M10d_Yes: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const M10d_No: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_NOMEM_BKPT: ::core::ffi::c_int = SQLITE_NOMEM;
pub const SQLITE_ECEL_DUP: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_ECEL_REF: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SQLITE_ECEL_OMITREF: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
