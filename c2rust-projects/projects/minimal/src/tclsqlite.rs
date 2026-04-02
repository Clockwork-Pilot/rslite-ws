use ::libc;
extern "C" {
    pub type Tcl_Channel_;
    pub type Tcl_ChannelTypeVersion_;
    pub type Tcl_Command_;
    pub type sqlite3;
    pub type sqlite3_stmt;
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type sqlite3_blob;
    pub type sqlite3_backup;
    fn Tcl_PkgProvideEx(
        interp: *mut Tcl_Interp,
        name: *const ::core::ffi::c_char,
        version: *const ::core::ffi::c_char,
        clientData: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn Tcl_Alloc(size: ::core::ffi::c_uint) -> *mut ::core::ffi::c_char;
    fn Tcl_Free(ptr: *mut ::core::ffi::c_char);
    fn Tcl_DuplicateObj(objPtr: *mut Tcl_Obj) -> *mut Tcl_Obj;
    fn TclFreeObj(objPtr: *mut Tcl_Obj);
    fn Tcl_GetBooleanFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        intPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetByteArrayFromObj(
        objPtr: *mut Tcl_Obj,
        lengthPtr: *mut ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_uchar;
    fn Tcl_GetDoubleFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        doublePtr: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetIntFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        intPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetStringFromObj(
        objPtr: *mut Tcl_Obj,
        lengthPtr: *mut ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn Tcl_ListObjAppendElement(
        interp: *mut Tcl_Interp,
        listPtr: *mut Tcl_Obj,
        objPtr: *mut Tcl_Obj,
    ) -> ::core::ffi::c_int;
    fn Tcl_ListObjGetElements(
        interp: *mut Tcl_Interp,
        listPtr: *mut Tcl_Obj,
        objcPtr: *mut ::core::ffi::c_int,
        objvPtr: *mut *mut *mut Tcl_Obj,
    ) -> ::core::ffi::c_int;
    fn Tcl_ListObjIndex(
        interp: *mut Tcl_Interp,
        listPtr: *mut Tcl_Obj,
        index: ::core::ffi::c_int,
        objPtrPtr: *mut *mut Tcl_Obj,
    ) -> ::core::ffi::c_int;
    fn Tcl_ListObjLength(
        interp: *mut Tcl_Interp,
        listPtr: *mut Tcl_Obj,
        lengthPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_NewByteArrayObj(
        bytes: *const ::core::ffi::c_uchar,
        length: ::core::ffi::c_int,
    ) -> *mut Tcl_Obj;
    fn Tcl_NewDoubleObj(doubleValue: ::core::ffi::c_double) -> *mut Tcl_Obj;
    fn Tcl_NewIntObj(intValue: ::core::ffi::c_int) -> *mut Tcl_Obj;
    fn Tcl_NewListObj(
        objc: ::core::ffi::c_int,
        objv: *const *mut Tcl_Obj,
    ) -> *mut Tcl_Obj;
    fn Tcl_NewObj() -> *mut Tcl_Obj;
    fn Tcl_NewStringObj(
        bytes: *const ::core::ffi::c_char,
        length: ::core::ffi::c_int,
    ) -> *mut Tcl_Obj;
    fn Tcl_SetIntObj(objPtr: *mut Tcl_Obj, intValue: ::core::ffi::c_int);
    fn Tcl_SetObjLength(objPtr: *mut Tcl_Obj, length: ::core::ffi::c_int);
    fn Tcl_AppendResult(interp: *mut Tcl_Interp, ...);
    fn Tcl_BackgroundError(interp: *mut Tcl_Interp);
    fn Tcl_Close(interp: *mut Tcl_Interp, chan: Tcl_Channel) -> ::core::ffi::c_int;
    fn Tcl_CreateChannel(
        typePtr: *const Tcl_ChannelType,
        chanName: *const ::core::ffi::c_char,
        instanceData: ClientData,
        mask: ::core::ffi::c_int,
    ) -> Tcl_Channel;
    fn Tcl_CreateObjCommand(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
        proc: Option<Tcl_ObjCmdProc>,
        clientData: ClientData,
        deleteProc: Option<Tcl_CmdDeleteProc>,
    ) -> Tcl_Command;
    fn Tcl_DeleteCommand(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn Tcl_DStringAppend(
        dsPtr: *mut Tcl_DString,
        bytes: *const ::core::ffi::c_char,
        length: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn Tcl_DStringAppendElement(
        dsPtr: *mut Tcl_DString,
        element: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn Tcl_DStringFree(dsPtr: *mut Tcl_DString);
    fn Tcl_DStringInit(dsPtr: *mut Tcl_DString);
    fn Tcl_Eval(
        interp: *mut Tcl_Interp,
        script: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetChannelName(chan: Tcl_Channel) -> *const ::core::ffi::c_char;
    fn Tcl_GetObjResult(interp: *mut Tcl_Interp) -> *mut Tcl_Obj;
    fn Tcl_GetsObj(chan: Tcl_Channel, objPtr: *mut Tcl_Obj) -> ::core::ffi::c_int;
    fn Tcl_GetStringResult(interp: *mut Tcl_Interp) -> *const ::core::ffi::c_char;
    fn Tcl_GlobalEval(
        interp: *mut Tcl_Interp,
        command: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn Tcl_ObjGetVar2(
        interp: *mut Tcl_Interp,
        part1Ptr: *mut Tcl_Obj,
        part2Ptr: *mut Tcl_Obj,
        flags: ::core::ffi::c_int,
    ) -> *mut Tcl_Obj;
    fn Tcl_ObjSetVar2(
        interp: *mut Tcl_Interp,
        part1Ptr: *mut Tcl_Obj,
        part2Ptr: *mut Tcl_Obj,
        newValuePtr: *mut Tcl_Obj,
        flags: ::core::ffi::c_int,
    ) -> *mut Tcl_Obj;
    fn Tcl_OpenFileChannel(
        interp: *mut Tcl_Interp,
        fileName: *const ::core::ffi::c_char,
        modeString: *const ::core::ffi::c_char,
        permissions: ::core::ffi::c_int,
    ) -> Tcl_Channel;
    fn Tcl_RegisterChannel(interp: *mut Tcl_Interp, chan: Tcl_Channel);
    fn Tcl_ResetResult(interp: *mut Tcl_Interp);
    fn Tcl_SetChannelOption(
        interp: *mut Tcl_Interp,
        chan: Tcl_Channel,
        optionName: *const ::core::ffi::c_char,
        newValue: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn Tcl_SetResult(
        interp: *mut Tcl_Interp,
        result: *mut ::core::ffi::c_char,
        freeProc: Option<Tcl_FreeProc>,
    );
    fn Tcl_SetObjResult(interp: *mut Tcl_Interp, resultObjPtr: *mut Tcl_Obj);
    fn Tcl_TranslateFileName(
        interp: *mut Tcl_Interp,
        name: *const ::core::ffi::c_char,
        bufferPtr: *mut Tcl_DString,
    ) -> *mut ::core::ffi::c_char;
    fn Tcl_UnregisterChannel(
        interp: *mut Tcl_Interp,
        chan: Tcl_Channel,
    ) -> ::core::ffi::c_int;
    fn Tcl_UnsetVar2(
        interp: *mut Tcl_Interp,
        part1: *const ::core::ffi::c_char,
        part2: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_VarEval(interp: *mut Tcl_Interp, ...) -> ::core::ffi::c_int;
    fn Tcl_WrongNumArgs(
        interp: *mut Tcl_Interp,
        objc: ::core::ffi::c_int,
        objv: *const *mut Tcl_Obj,
        message: *const ::core::ffi::c_char,
    );
    fn Tcl_GetVersion(
        major: *mut ::core::ffi::c_int,
        minor: *mut ::core::ffi::c_int,
        patchLevel: *mut ::core::ffi::c_int,
        type_0: *mut ::core::ffi::c_int,
    );
    fn Tcl_EvalObjEx(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetIndexFromObjStruct(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        tablePtr: *const ::core::ffi::c_void,
        offset: ::core::ffi::c_int,
        msg: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
        indexPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetVar2Ex(
        interp: *mut Tcl_Interp,
        part1: *const ::core::ffi::c_char,
        part2: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> *mut Tcl_Obj;
    fn Tcl_GetString(objPtr: *mut Tcl_Obj) -> *mut ::core::ffi::c_char;
    fn Tcl_GetWideIntFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        widePtr: *mut Tcl_WideInt,
    ) -> ::core::ffi::c_int;
    fn Tcl_NewWideIntObj(wideValue: Tcl_WideInt) -> *mut Tcl_Obj;
    fn Tcl_SetWideIntObj(objPtr: *mut Tcl_Obj, wideValue: Tcl_WideInt);
    fn Tcl_DictObjPut(
        interp: *mut Tcl_Interp,
        dictPtr: *mut Tcl_Obj,
        keyPtr: *mut Tcl_Obj,
        valuePtr: *mut Tcl_Obj,
    ) -> ::core::ffi::c_int;
    fn Tcl_DictObjRemove(
        interp: *mut Tcl_Interp,
        dictPtr: *mut Tcl_Obj,
        keyPtr: *mut Tcl_Obj,
    ) -> ::core::ffi::c_int;
    fn Tcl_NewDictObj() -> *mut Tcl_Obj;
    fn Tcl_NRCreateCommand(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
        proc: Option<Tcl_ObjCmdProc>,
        nreProc: Option<Tcl_ObjCmdProc>,
        clientData: ClientData,
        deleteProc: Option<Tcl_CmdDeleteProc>,
    ) -> Tcl_Command;
    fn Tcl_NREvalObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_NRAddCallback(
        interp: *mut Tcl_Interp,
        postProcPtr: Option<Tcl_NRPostProc>,
        data0: ClientData,
        data1: ClientData,
        data2: ClientData,
        data3: ClientData,
    );
    fn Tcl_NRCallObjProc(
        interp: *mut Tcl_Interp,
        objProc: Option<Tcl_ObjCmdProc>,
        clientData: ClientData,
        objc: ::core::ffi::c_int,
        objv: *const *mut Tcl_Obj,
    ) -> ::core::ffi::c_int;
    fn sqlite3_libversion() -> *const ::core::ffi::c_char;
    fn sqlite3_sourceid() -> *const ::core::ffi::c_char;
    fn sqlite3_close(_: *mut sqlite3) -> ::core::ffi::c_int;
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
    fn sqlite3_db_config(
        _: *mut sqlite3,
        op: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn sqlite3_last_insert_rowid(_: *mut sqlite3) -> sqlite3_int64;
    fn sqlite3_changes64(_: *mut sqlite3) -> sqlite3_int64;
    fn sqlite3_total_changes64(_: *mut sqlite3) -> sqlite3_int64;
    fn sqlite3_interrupt(_: *mut sqlite3);
    fn sqlite3_complete(sql: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn sqlite3_busy_handler(
        _: *mut sqlite3,
        _: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3_busy_timeout(
        _: *mut sqlite3,
        ms: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_snprintf(
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_set_authorizer(
        _: *mut sqlite3,
        xAuth: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        pUserData: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3_trace_v2(
        _: *mut sqlite3,
        uMask: ::core::ffi::c_uint,
        xCallback: Option<
            unsafe extern "C" fn(
                ::core::ffi::c_uint,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
        pCtx: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3_progress_handler(
        _: *mut sqlite3,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
        _: *mut ::core::ffi::c_void,
    );
    fn sqlite3_open_v2(
        filename: *const ::core::ffi::c_char,
        ppDb: *mut *mut sqlite3,
        flags: ::core::ffi::c_int,
        zVfs: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_errcode(db: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
    fn sqlite3_errstr(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn sqlite3_error_offset(db: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3_prepare(
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
    fn sqlite3_sql(pStmt: *mut sqlite3_stmt) -> *const ::core::ffi::c_char;
    fn sqlite3_bind_blob(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_void,
        n: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_double(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_int(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_int64(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: sqlite3_int64,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_null(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_text(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_text64(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: sqlite3_uint64,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        encoding: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_parameter_count(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_bind_parameter_name(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    fn sqlite3_column_count(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_column_name(
        _: *mut sqlite3_stmt,
        N: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_column_blob(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_void;
    fn sqlite3_column_double(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
    fn sqlite3_column_int64(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> sqlite3_int64;
    fn sqlite3_column_text(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_uchar;
    fn sqlite3_column_bytes(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_column_type(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_create_function(
        db: *mut sqlite3,
        zFunctionName: *const ::core::ffi::c_char,
        nArg: ::core::ffi::c_int,
        eTextRep: ::core::ffi::c_int,
        pApp: *mut ::core::ffi::c_void,
        xFunc: Option<
            unsafe extern "C" fn(
                *mut sqlite3_context,
                ::core::ffi::c_int,
                *mut *mut sqlite3_value,
            ) -> (),
        >,
        xStep: Option<
            unsafe extern "C" fn(
                *mut sqlite3_context,
                ::core::ffi::c_int,
                *mut *mut sqlite3_value,
            ) -> (),
        >,
        xFinal: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3_value_blob(_: *mut sqlite3_value) -> *const ::core::ffi::c_void;
    fn sqlite3_value_double(_: *mut sqlite3_value) -> ::core::ffi::c_double;
    fn sqlite3_value_int64(_: *mut sqlite3_value) -> sqlite3_int64;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_value_bytes(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_type(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_user_data(_: *mut sqlite3_context) -> *mut ::core::ffi::c_void;
    fn sqlite3_result_blob(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_result_double(_: *mut sqlite3_context, _: ::core::ffi::c_double);
    fn sqlite3_result_error(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    );
    fn sqlite3_result_int64(_: *mut sqlite3_context, _: sqlite3_int64);
    fn sqlite3_result_null(_: *mut sqlite3_context);
    fn sqlite3_result_text64(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: sqlite3_uint64,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        encoding: ::core::ffi::c_uchar,
    );
    fn sqlite3_create_collation(
        _: *mut sqlite3,
        zName: *const ::core::ffi::c_char,
        eTextRep: ::core::ffi::c_int,
        pArg: *mut ::core::ffi::c_void,
        xCompare: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *const ::core::ffi::c_void,
                ::core::ffi::c_int,
                *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    ) -> ::core::ffi::c_int;
    fn sqlite3_collation_needed(
        _: *mut sqlite3,
        _: *mut ::core::ffi::c_void,
        _: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut sqlite3,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
            ) -> (),
        >,
    ) -> ::core::ffi::c_int;
    fn sqlite3_sleep(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3_commit_hook(
        _: *mut sqlite3,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
        _: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_rollback_hook(
        _: *mut sqlite3,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        _: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_update_hook(
        _: *mut sqlite3,
        _: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                sqlite3_int64,
            ) -> (),
        >,
        _: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_blob_open(
        _: *mut sqlite3,
        zDb: *const ::core::ffi::c_char,
        zTable: *const ::core::ffi::c_char,
        zColumn: *const ::core::ffi::c_char,
        iRow: sqlite3_int64,
        flags: ::core::ffi::c_int,
        ppBlob: *mut *mut sqlite3_blob,
    ) -> ::core::ffi::c_int;
    fn sqlite3_blob_close(_: *mut sqlite3_blob) -> ::core::ffi::c_int;
    fn sqlite3_blob_bytes(_: *mut sqlite3_blob) -> ::core::ffi::c_int;
    fn sqlite3_blob_read(
        _: *mut sqlite3_blob,
        Z: *mut ::core::ffi::c_void,
        N: ::core::ffi::c_int,
        iOffset: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_blob_write(
        _: *mut sqlite3_blob,
        z: *const ::core::ffi::c_void,
        n: ::core::ffi::c_int,
        iOffset: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_file_control(
        _: *mut sqlite3,
        zDbName: *const ::core::ffi::c_char,
        op: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3_stmt_status(
        _: *mut sqlite3_stmt,
        op: ::core::ffi::c_int,
        resetFlg: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_backup_init(
        pDest: *mut sqlite3,
        zDestName: *const ::core::ffi::c_char,
        pSource: *mut sqlite3,
        zSourceName: *const ::core::ffi::c_char,
    ) -> *mut sqlite3_backup;
    fn sqlite3_backup_step(
        p: *mut sqlite3_backup,
        nPage: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_backup_finish(p: *mut sqlite3_backup) -> ::core::ffi::c_int;
    fn sqlite3_stricmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_wal_hook(
        _: *mut sqlite3,
        _: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut sqlite3,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        _: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_serialize(
        db: *mut sqlite3,
        zSchema: *const ::core::ffi::c_char,
        piSize: *mut sqlite3_int64,
        mFlags: ::core::ffi::c_uint,
    ) -> *mut ::core::ffi::c_uchar;
    fn sqlite3_deserialize(
        db: *mut sqlite3,
        zSchema: *const ::core::ffi::c_char,
        pData: *mut ::core::ffi::c_uchar,
        szDb: sqlite3_int64,
        szBuf: sqlite3_int64,
        mFlags: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn atoi(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
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
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
}
pub type size_t = usize;
pub type ClientData = *mut ::core::ffi::c_void;
pub type Tcl_WideInt = ::core::ffi::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_Interp {
    pub resultDontUse: *mut ::core::ffi::c_char,
    pub freeProcDontUse: Option<unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ()>,
    pub errorLineDontUse: ::core::ffi::c_int

}
pub type Tcl_Channel = *mut Tcl_Channel_;
pub type Tcl_ChannelTypeVersion = *mut Tcl_ChannelTypeVersion_;
pub type Tcl_Command = *mut Tcl_Command_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_Obj {
    pub refCount: ::core::ffi::c_int,
    pub bytes: *mut ::core::ffi::c_char,
    pub length: ::core::ffi::c_int,
    pub typePtr: *const Tcl_ObjType,
    pub internalRep: C2Rust_Unnamed

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub longValue: ::core::ffi::c_long,
    pub doubleValue: ::core::ffi::c_double,
    pub otherValuePtr: *mut ::core::ffi::c_void,
    pub wideValue: Tcl_WideInt,
    pub twoPtrValue: C2Rust_Unnamed_1,
    pub ptrAndLongRep: C2Rust_Unnamed_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub ptr: *mut ::core::ffi::c_void,
    pub value: ::core::ffi::c_ulong

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub ptr1: *mut ::core::ffi::c_void,
    pub ptr2: *mut ::core::ffi::c_void

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_ObjType {
    pub name: *const ::core::ffi::c_char,
    pub freeIntRepProc: Option<Tcl_FreeInternalRepProc>,
    pub dupIntRepProc: Option<Tcl_DupInternalRepProc>,
    pub updateStringProc: Option<Tcl_UpdateStringProc>,
    pub setFromAnyProc: Option<Tcl_SetFromAnyProc>

}
pub type Tcl_SetFromAnyProc = unsafe extern "C" fn(
    *mut Tcl_Interp,
    *mut Tcl_Obj,
) -> ::core::ffi::c_int;
pub type Tcl_UpdateStringProc = unsafe extern "C" fn(*mut Tcl_Obj) -> ();
pub type Tcl_DupInternalRepProc = unsafe extern "C" fn(*mut Tcl_Obj, *mut Tcl_Obj) -> ();
pub type Tcl_FreeInternalRepProc = unsafe extern "C" fn(*mut Tcl_Obj) -> ();
pub type Tcl_CmdDeleteProc = unsafe extern "C" fn(ClientData) -> ();
pub type Tcl_FreeProc = unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ();
pub type Tcl_ObjCmdProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    ::core::ffi::c_int,
    *const *mut Tcl_Obj,
) -> ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_DString {
    pub string: *mut ::core::ffi::c_char,
    pub length: ::core::ffi::c_int,
    pub spaceAvl: ::core::ffi::c_int,
    pub staticSpace: [::core::ffi::c_char; 200]

}
pub type Tcl_DriverBlockModeProc = unsafe extern "C" fn(
    ClientData,
    ::core::ffi::c_int,
) -> ::core::ffi::c_int;
pub type Tcl_DriverCloseProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
) -> ::core::ffi::c_int;
pub type Tcl_DriverClose2Proc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    ::core::ffi::c_int,
) -> ::core::ffi::c_int;
pub type Tcl_DriverInputProc = unsafe extern "C" fn(
    ClientData,
    *mut ::core::ffi::c_char,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int;
pub type Tcl_DriverOutputProc = unsafe extern "C" fn(
    ClientData,
    *const ::core::ffi::c_char,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int;
pub type Tcl_DriverSeekProc = unsafe extern "C" fn(
    ClientData,
    ::core::ffi::c_long,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int;
pub type Tcl_DriverSetOptionProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    *const ::core::ffi::c_char,
    *const ::core::ffi::c_char,
) -> ::core::ffi::c_int;
pub type Tcl_DriverGetOptionProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    *const ::core::ffi::c_char,
    *mut Tcl_DString,
) -> ::core::ffi::c_int;
pub type Tcl_DriverWatchProc = unsafe extern "C" fn(
    ClientData,
    ::core::ffi::c_int,
) -> ();
pub type Tcl_DriverGetHandleProc = unsafe extern "C" fn(
    ClientData,
    ::core::ffi::c_int,
    *mut ClientData,
) -> ::core::ffi::c_int;
pub type Tcl_DriverFlushProc = unsafe extern "C" fn(ClientData) -> ::core::ffi::c_int;
pub type Tcl_DriverHandlerProc = unsafe extern "C" fn(
    ClientData,
    ::core::ffi::c_int,
) -> ::core::ffi::c_int;
pub type Tcl_DriverWideSeekProc = unsafe extern "C" fn(
    ClientData,
    Tcl_WideInt,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_int,
) -> Tcl_WideInt;
pub type Tcl_DriverThreadActionProc = unsafe extern "C" fn(
    ClientData,
    ::core::ffi::c_int,
) -> ();
pub type Tcl_DriverTruncateProc = unsafe extern "C" fn(
    ClientData,
    Tcl_WideInt,
) -> ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_ChannelType {
    pub typeName: *const ::core::ffi::c_char,
    pub version: Tcl_ChannelTypeVersion,
    pub closeProc: Option<Tcl_DriverCloseProc>,
    pub inputProc: Option<Tcl_DriverInputProc>,
    pub outputProc: Option<Tcl_DriverOutputProc>,
    pub seekProc: Option<Tcl_DriverSeekProc>,
    pub setOptionProc: Option<Tcl_DriverSetOptionProc>,
    pub getOptionProc: Option<Tcl_DriverGetOptionProc>,
    pub watchProc: Option<Tcl_DriverWatchProc>,
    pub getHandleProc: Option<Tcl_DriverGetHandleProc>,
    pub close2Proc: Option<Tcl_DriverClose2Proc>,
    pub blockModeProc: Option<Tcl_DriverBlockModeProc>,
    pub flushProc: Option<Tcl_DriverFlushProc>,
    pub handlerProc: Option<Tcl_DriverHandlerProc>,
    pub wideSeekProc: Option<Tcl_DriverWideSeekProc>,
    pub threadActionProc: Option<Tcl_DriverThreadActionProc>,
    pub truncateProc: Option<Tcl_DriverTruncateProc>

}
pub type Tcl_NRPostProc = unsafe extern "C" fn(
    *mut ClientData,
    *mut Tcl_Interp,
    ::core::ffi::c_int,
) -> ::core::ffi::c_int;
pub type Tcl_Size = ::core::ffi::c_int;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite_uint64 = ::core::ffi::c_ulonglong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_uint64 = sqlite_uint64;
pub type sqlite3_destructor_type = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
>;
pub type u8_0 = ::core::ffi::c_uchar;
pub type uintptr_t = usize;
pub type uptr = uintptr_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SqliteDb {
    pub db: *mut sqlite3,
    pub interp: *mut Tcl_Interp,
    pub zBusy: *mut ::core::ffi::c_char,
    pub zCommit: *mut ::core::ffi::c_char,
    pub zTrace: *mut ::core::ffi::c_char,
    pub zTraceV2: *mut ::core::ffi::c_char,
    pub zProfile: *mut ::core::ffi::c_char,
    pub zProgress: *mut ::core::ffi::c_char,
    pub zBindFallback: *mut ::core::ffi::c_char,
    pub zAuth: *mut ::core::ffi::c_char,
    pub disableAuth: ::core::ffi::c_int,
    pub zNull: *mut ::core::ffi::c_char,
    pub pFunc: *mut SqlFunc,
    pub pUpdateHook: *mut Tcl_Obj,
    pub pPreUpdateHook: *mut Tcl_Obj,
    pub pRollbackHook: *mut Tcl_Obj,
    pub pWalHook: *mut Tcl_Obj,
    pub pUnlockNotify: *mut Tcl_Obj,
    pub pCollate: *mut SqlCollate,
    pub rc: ::core::ffi::c_int,
    pub pCollateNeeded: *mut Tcl_Obj,
    pub stmtList: *mut SqlPreparedStmt,
    pub stmtLast: *mut SqlPreparedStmt,
    pub maxStmt: ::core::ffi::c_int,
    pub nStmt: ::core::ffi::c_int,
    pub pIncrblob: *mut IncrblobChannel,
    pub nStep: ::core::ffi::c_int,
    pub nSort: ::core::ffi::c_int,
    pub nIndex: ::core::ffi::c_int,
    pub nVMStep: ::core::ffi::c_int,
    pub nTransaction: ::core::ffi::c_int,
    pub openFlags: ::core::ffi::c_int,
    pub nRef: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IncrblobChannel {
    pub pBlob: *mut sqlite3_blob,
    pub pDb: *mut SqliteDb,
    pub iSeek: sqlite3_int64,
    pub isClosed: ::core::ffi::c_uint,
    pub channel: Tcl_Channel,
    pub pNext: *mut IncrblobChannel,
    pub pPrev: *mut IncrblobChannel

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SqlPreparedStmt {
    pub pNext: *mut SqlPreparedStmt,
    pub pPrev: *mut SqlPreparedStmt,
    pub pStmt: *mut sqlite3_stmt,
    pub nSql: ::core::ffi::c_int,
    pub zSql: *const ::core::ffi::c_char,
    pub nParm: ::core::ffi::c_int,
    pub apParm: *mut *mut Tcl_Obj

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SqlCollate {
    pub interp: *mut Tcl_Interp,
    pub zScript: *mut ::core::ffi::c_char,
    pub pNext: *mut SqlCollate

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SqlFunc {
    pub interp: *mut Tcl_Interp,
    pub pScript: *mut Tcl_Obj,
    pub pDb: *mut SqliteDb,
    pub useEvalObjv: ::core::ffi::c_int,
    pub eType: ::core::ffi::c_int,
    pub zName: *mut ::core::ffi::c_char,
    pub pNext: *mut SqlFunc

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DbEvalContext {
    pub pDb: *mut SqliteDb,
    pub pSql: *mut Tcl_Obj,
    pub zSql: *const ::core::ffi::c_char,
    pub pPreStmt: *mut SqlPreparedStmt,
    pub nCol: ::core::ffi::c_int,
    pub evalFlags: ::core::ffi::c_int,
    pub pVarName: *mut Tcl_Obj,
    pub apColName: *mut *mut Tcl_Obj

}
pub const DB_VERSION: DB_enum = 40;
pub const DB_ROLLBACK_HOOK: DB_enum = 30;
pub const DB_UPDATE_HOOK: DB_enum = 39;
pub const DB_WAL_HOOK: DB_enum = 41;
pub const DB_PREUPDATE: DB_enum = 25;
pub const DB_UNLOCK_NOTIFY: DB_enum = 38;
pub const TTYPE_IMMEDIATE: TTYPE_enum = 2;
pub const TTYPE_EXCLUSIVE: TTYPE_enum = 1;
pub const TTYPE_DEFERRED: TTYPE_enum = 0;
pub type TTYPE_enum = ::core::ffi::c_uint;
pub const DB_TRANSACTION: DB_enum = 37;
pub const TTYPE_CLOSE: TTYPE_enum_0 = 3;
pub const TTYPE_ROW: TTYPE_enum_0 = 2;
pub const TTYPE_PROFILE: TTYPE_enum_0 = 1;
pub const TTYPE_STMT: TTYPE_enum_0 = 0;
pub type TTYPE_enum_0 = ::core::ffi::c_uint;
pub const DB_TRACE_V2: DB_enum = 36;
pub const DB_TRACE: DB_enum = 35;
pub const DB_TOTAL_CHANGES: DB_enum = 34;
pub const DB_TIMEOUT: DB_enum = 33;
pub const DB_STATUS: DB_enum = 32;
pub const DB_SERIALIZE: DB_enum = 31;
pub const DB_RESTORE: DB_enum = 29;
pub const DB_REKEY: DB_enum = 28;
pub const DB_PROFILE: DB_enum = 26;
pub const DB_PROGRESS: DB_enum = 27;
pub const DB_LAST_INSERT_ROWID: DB_enum = 22;
pub const DB_NULLVALUE: DB_enum = 23;
pub const DB_INTERRUPT: DB_enum = 21;
pub const DB_INCRBLOB: DB_enum = 20;
pub const DB_FUNCTION: DB_enum = 19;
pub const DB_EVAL: DB_enum = 17;
pub const DB_ONECOLUMN: DB_enum = 24;
pub const DB_EXISTS: DB_enum = 18;
pub const DB_ERROROFFSET: DB_enum = 16;
pub const DB_ERRORCODE: DB_enum = 15;
pub const DB_ENABLE_LOAD_EXTENSION: DB_enum = 14;
pub const DB_DESERIALIZE: DB_enum = 13;
pub const DB_COPY: DB_enum = 12;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DbConfigChoices {
    pub zName: *const ::core::ffi::c_char,
    pub op: ::core::ffi::c_int

}
pub const DB_CONFIG: DB_enum = 11;
pub const DB_COMPLETE: DB_enum = 10;
pub const DB_COMMIT_HOOK: DB_enum = 9;
pub const DB_COLLATION_NEEDED: DB_enum = 8;
pub const DB_COLLATE: DB_enum = 7;
pub const DB_CLOSE: DB_enum = 6;
pub const DB_CHANGES: DB_enum = 5;
pub const DB_CACHE: DB_enum = 4;
pub const DB_BUSY: DB_enum = 3;
pub const DB_BIND_FALLBACK: DB_enum = 2;
pub const DB_BACKUP: DB_enum = 1;
pub type sqlite3_auth_cb = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        ::core::ffi::c_int,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
>;
pub const DB_AUTHORIZER: DB_enum = 0;
pub type DB_enum = ::core::ffi::c_uint;
pub const TCL_VERSION: [::core::ffi::c_char; 4] = unsafe {
    ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"8.6\0")
};
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>()
    as *mut ::core::ffi::c_void;
pub const SEEK_SET: ::core::ffi::c_int = 0;
pub const SEEK_CUR: ::core::ffi::c_int = 1;
pub const SEEK_END: ::core::ffi::c_int = 2;
pub const EIO: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const TCL_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TCL_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TCL_RETURN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const TCL_BREAK: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const TCL_CONTINUE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const TCL_EVAL_DIRECT: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;
pub const TCL_STATIC: Option<Tcl_FreeProc> = None;
pub const TCL_READABLE: ::core::ffi::c_int = (1 as ::core::ffi::c_int)
    << 1 as ::core::ffi::c_int;
pub const TCL_WRITABLE: ::core::ffi::c_int = (1 as ::core::ffi::c_int)
    << 2 as ::core::ffi::c_int;
pub const TCL_CLOSE_WRITE: ::core::ffi::c_int = (1 as ::core::ffi::c_int)
    << 2 as ::core::ffi::c_int;
pub const TCL_CHANNEL_VERSION_5: Tcl_ChannelTypeVersion = 0x5 as ::core::ffi::c_int
    as Tcl_ChannelTypeVersion;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_BUSY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_LOCKED: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_OPEN_READONLY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_OPEN_READWRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_OPEN_CREATE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SQLITE_OPEN_URI: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SQLITE_OPEN_NOMUTEX: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
pub const SQLITE_OPEN_FULLMUTEX: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const SQLITE_OPEN_NOFOLLOW: ::core::ffi::c_int = 0x1000000 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_SIZE_LIMIT: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_ENABLE_FKEY: ::core::ffi::c_int = 1002 as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_ENABLE_TRIGGER: ::core::ffi::c_int = 1003
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_ENABLE_FTS3_TOKENIZER: ::core::ffi::c_int = 1004
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_ENABLE_LOAD_EXTENSION: ::core::ffi::c_int = 1005
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_NO_CKPT_ON_CLOSE: ::core::ffi::c_int = 1006
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_ENABLE_QPSG: ::core::ffi::c_int = 1007 as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_TRIGGER_EQP: ::core::ffi::c_int = 1008 as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_RESET_DATABASE: ::core::ffi::c_int = 1009
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_DEFENSIVE: ::core::ffi::c_int = 1010 as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_WRITABLE_SCHEMA: ::core::ffi::c_int = 1011
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_LEGACY_ALTER_TABLE: ::core::ffi::c_int = 1012
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_DQS_DML: ::core::ffi::c_int = 1013 as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_DQS_DDL: ::core::ffi::c_int = 1014 as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_ENABLE_VIEW: ::core::ffi::c_int = 1015 as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_LEGACY_FILE_FORMAT: ::core::ffi::c_int = 1016
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_TRUSTED_SCHEMA: ::core::ffi::c_int = 1017
    as ::core::ffi::c_int;
pub const SQLITE_DENY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_IGNORE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_CREATE_INDEX: ::core::ffi::c_int = 1;
pub const SQLITE_CREATE_TABLE: ::core::ffi::c_int = 2;
pub const SQLITE_CREATE_TEMP_INDEX: ::core::ffi::c_int = 3;
pub const SQLITE_CREATE_TEMP_TABLE: ::core::ffi::c_int = 4;
pub const SQLITE_CREATE_TEMP_TRIGGER: ::core::ffi::c_int = 5;
pub const SQLITE_CREATE_TEMP_VIEW: ::core::ffi::c_int = 6;
pub const SQLITE_CREATE_TRIGGER: ::core::ffi::c_int = 7;
pub const SQLITE_CREATE_VIEW: ::core::ffi::c_int = 8;
pub const SQLITE_DELETE: ::core::ffi::c_int = 9;
pub const SQLITE_DROP_INDEX: ::core::ffi::c_int = 10;
pub const SQLITE_DROP_TABLE: ::core::ffi::c_int = 11;
pub const SQLITE_DROP_TEMP_INDEX: ::core::ffi::c_int = 12;
pub const SQLITE_DROP_TEMP_TABLE: ::core::ffi::c_int = 13;
pub const SQLITE_DROP_TEMP_TRIGGER: ::core::ffi::c_int = 14;
pub const SQLITE_DROP_TEMP_VIEW: ::core::ffi::c_int = 15;
pub const SQLITE_DROP_TRIGGER: ::core::ffi::c_int = 16;
pub const SQLITE_DROP_VIEW: ::core::ffi::c_int = 17;
pub const SQLITE_INSERT: ::core::ffi::c_int = 18;
pub const SQLITE_PRAGMA: ::core::ffi::c_int = 19;
pub const SQLITE_READ: ::core::ffi::c_int = 20;
pub const SQLITE_SELECT: ::core::ffi::c_int = 21;
pub const SQLITE_TRANSACTION: ::core::ffi::c_int = 22;
pub const SQLITE_UPDATE: ::core::ffi::c_int = 23;
pub const SQLITE_ATTACH: ::core::ffi::c_int = 24;
pub const SQLITE_DETACH: ::core::ffi::c_int = 25;
pub const SQLITE_ALTER_TABLE: ::core::ffi::c_int = 26;
pub const SQLITE_REINDEX: ::core::ffi::c_int = 27;
pub const SQLITE_ANALYZE: ::core::ffi::c_int = 28;
pub const SQLITE_CREATE_VTABLE: ::core::ffi::c_int = 29;
pub const SQLITE_DROP_VTABLE: ::core::ffi::c_int = 30;
pub const SQLITE_FUNCTION: ::core::ffi::c_int = 31;
pub const SQLITE_SAVEPOINT: ::core::ffi::c_int = 32;
pub const SQLITE_COPY: ::core::ffi::c_int = 0;
pub const SQLITE_RECURSIVE: ::core::ffi::c_int = 33;
pub const SQLITE_TRACE_STMT: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_TRACE_PROFILE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_TRACE_ROW: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SQLITE_TRACE_CLOSE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SQLITE_PREPARE_PERSISTENT: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_INTEGER: ::core::ffi::c_int = 1;
pub const SQLITE_FLOAT: ::core::ffi::c_int = 2;
pub const SQLITE_BLOB: ::core::ffi::c_int = 4;
pub const SQLITE_NULL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_TEXT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_DETERMINISTIC: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SQLITE_DIRECTONLY: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;
pub const SQLITE_INNOCUOUS: ::core::ffi::c_int = 0x200000 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const SQLITE_STMTSTATUS_FULLSCAN_STEP: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_STMTSTATUS_SORT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_STMTSTATUS_AUTOINDEX: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SQLITE_STMTSTATUS_VM_STEP: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_SERIALIZE_NOCOPY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_DESERIALIZE_FREEONCLOSE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_DESERIALIZE_RESIZEABLE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_DESERIALIZE_READONLY: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const NUM_PREPARED_STMTS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const MAX_PREPARED_STMTS: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
unsafe extern "C" fn strlen30(mut z: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    unsafe {
        let mut z2: *const ::core::ffi::c_char = z;
        while *z2 != 0 {
            z2 = z2.offset(1);
        }
        return 0x3fffffff as ::core::ffi::c_int
            & z2.offset_from(z) as ::core::ffi::c_long as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn closeIncrblobChannels(mut pDb: *mut SqliteDb) {
    unsafe {
        let mut p: *mut IncrblobChannel = ::core::ptr::null_mut::<IncrblobChannel>();
        let mut pNext: *mut IncrblobChannel = ::core::ptr::null_mut::<IncrblobChannel>();
        p = (*pDb).pIncrblob;
        while !p.is_null() {
            pNext = (*p).pNext;
            Tcl_UnregisterChannel((*pDb).interp, (*p).channel);
            p = pNext;
        }
    }
}
unsafe extern "C" fn incrblobClose2(
    mut instanceData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut IncrblobChannel = instanceData as *mut IncrblobChannel;
        let mut rc: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = (*(*p).pDb).db;
        if flags != 0 {
            (*p).isClosed |= flags as ::core::ffi::c_uint;
            return TCL_OK;
        }
        rc = sqlite3_blob_close((*p).pBlob);
        if !(*p).pNext.is_null() {
            (*(*p).pNext).pPrev = (*p).pPrev;
        }
        if !(*p).pPrev.is_null() {
            (*(*p).pPrev).pNext = (*p).pNext;
        }
        if (*(*p).pDb).pIncrblob == p {
            (*(*p).pDb).pIncrblob = (*p).pNext;
        }
        Tcl_Free(p as *mut ::core::ffi::c_char);
        if rc != SQLITE_OK {
            Tcl_SetResult(
                interp,
                sqlite3_errmsg(db) as *mut ::core::ffi::c_char,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<Tcl_FreeProc>,
                >(1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn incrblobClose(
    mut instanceData: ClientData,
    mut interp: *mut Tcl_Interp,
) -> ::core::ffi::c_int {
    unsafe {
        return incrblobClose2(instanceData, interp, 0 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn incrblobInput(
    mut instanceData: ClientData,
    mut buf: *mut ::core::ffi::c_char,
    mut bufSize: ::core::ffi::c_int,
    mut errorCodePtr: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut IncrblobChannel = instanceData as *mut IncrblobChannel;
        let mut nRead: sqlite3_int64 = bufSize as sqlite3_int64;
        let mut nBlob: sqlite3_int64 = 0;
        let mut rc: ::core::ffi::c_int = 0;
        nBlob = sqlite3_blob_bytes((*p).pBlob) as sqlite3_int64;
        if (*p).iSeek + nRead > nBlob {
            nRead = nBlob - (*p).iSeek;
        }
        if nRead <= 0 as ::core::ffi::c_longlong {
            return 0 as ::core::ffi::c_int;
        }
        rc = sqlite3_blob_read(
            (*p).pBlob,
            buf as *mut ::core::ffi::c_void,
            nRead as ::core::ffi::c_int,
            (*p).iSeek as ::core::ffi::c_int,
        );
        if rc != SQLITE_OK {
            *errorCodePtr = rc;
            return -1 as ::core::ffi::c_int;
        }
        (*p).iSeek += nRead as ::core::ffi::c_longlong;
        return nRead as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn incrblobOutput(
    mut instanceData: ClientData,
    mut buf: *const ::core::ffi::c_char,
    mut toWrite: ::core::ffi::c_int,
    mut errorCodePtr: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut IncrblobChannel = instanceData as *mut IncrblobChannel;
        let mut nWrite: sqlite3_int64 = toWrite as sqlite3_int64;
        let mut nBlob: sqlite3_int64 = 0;
        let mut rc: ::core::ffi::c_int = 0;
        nBlob = sqlite3_blob_bytes((*p).pBlob) as sqlite3_int64;
        if (*p).iSeek + nWrite > nBlob {
            *errorCodePtr = EINVAL;
            return -1 as ::core::ffi::c_int;
        }
        if nWrite <= 0 as ::core::ffi::c_longlong {
            return 0 as ::core::ffi::c_int;
        }
        rc = sqlite3_blob_write(
            (*p).pBlob,
            buf as *mut ::core::ffi::c_void,
            nWrite as ::core::ffi::c_int,
            (*p).iSeek as ::core::ffi::c_int,
        );
        if rc != SQLITE_OK {
            *errorCodePtr = EIO;
            return -1 as ::core::ffi::c_int;
        }
        (*p).iSeek += nWrite as ::core::ffi::c_longlong;
        return nWrite as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn incrblobWideSeek(
    mut instanceData: ClientData,
    mut offset: Tcl_WideInt,
    mut seekMode: ::core::ffi::c_int,
    mut errorCodePtr: *mut ::core::ffi::c_int,
) -> Tcl_WideInt {
    unsafe {
        let mut p: *mut IncrblobChannel = instanceData as *mut IncrblobChannel;
        match seekMode {
            SEEK_SET => {
                (*p).iSeek = offset as sqlite3_int64;
            }
            SEEK_CUR => {
                (*p).iSeek += offset as ::core::ffi::c_longlong;
            }
            SEEK_END => {
                (*p).iSeek = (sqlite3_blob_bytes((*p).pBlob) as Tcl_WideInt + offset)
                    as sqlite3_int64;
            }
            _ => {
                '_c2rust_label: {
                    if (b"Bad seekMode\0".as_ptr() as *const ::core::ffi::c_char)
                        .is_null()
                    {} else {
                        __assert_fail(
                            b"!\"Bad seekMode\"\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/sqlite/src/tclsqlite.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            407 as ::core::ffi::c_uint,
                            b"Tcl_WideInt incrblobWideSeek(ClientData, Tcl_WideInt, int, int *)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                };
            }
        }
        return (*p).iSeek as Tcl_WideInt;
    }
}
unsafe extern "C" fn incrblobSeek(
    mut instanceData: ClientData,
    mut offset: ::core::ffi::c_long,
    mut seekMode: ::core::ffi::c_int,
    mut errorCodePtr: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return incrblobWideSeek(
            instanceData,
            offset as Tcl_WideInt,
            seekMode,
            errorCodePtr,
        ) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn incrblobWatch(
    mut instanceData: ClientData,
    mut mode: ::core::ffi::c_int,
) {}
unsafe extern "C" fn incrblobHandle(
    mut instanceData: ClientData,
    mut dir: ::core::ffi::c_int,
    mut hPtr: *mut ClientData,
) -> ::core::ffi::c_int {
    unsafe {
        return TCL_ERROR;
    }
}
static mut IncrblobChannelType: Tcl_ChannelType = unsafe {
    Tcl_ChannelType {
        typeName: b"incrblob\0".as_ptr() as *const ::core::ffi::c_char,
        version: TCL_CHANNEL_VERSION_5,
        closeProc: Some(
            incrblobClose
                as unsafe extern "C" fn(
                    ClientData,
                    *mut Tcl_Interp,
                ) -> ::core::ffi::c_int,
        ),
        inputProc: Some(
            incrblobInput
                as unsafe extern "C" fn(
                    ClientData,
                    *mut ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        outputProc: Some(
            incrblobOutput
                as unsafe extern "C" fn(
                    ClientData,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        seekProc: Some(
            incrblobSeek
                as unsafe extern "C" fn(
                    ClientData,
                    ::core::ffi::c_long,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        setOptionProc: None,
        getOptionProc: None,
        watchProc: Some(
            incrblobWatch as unsafe extern "C" fn(ClientData, ::core::ffi::c_int) -> (),
        ),
        getHandleProc: Some(
            incrblobHandle
                as unsafe extern "C" fn(
                    ClientData,
                    ::core::ffi::c_int,
                    *mut ClientData,
                ) -> ::core::ffi::c_int,
        ),
        close2Proc: Some(
            incrblobClose2
                as unsafe extern "C" fn(
                    ClientData,
                    *mut Tcl_Interp,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        blockModeProc: None,
        flushProc: None,
        handlerProc: None,
        wideSeekProc: Some(
            incrblobWideSeek
                as unsafe extern "C" fn(
                    ClientData,
                    Tcl_WideInt,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                ) -> Tcl_WideInt,
        ),
        threadActionProc: None,
        truncateProc: None

    }
};
unsafe extern "C" fn createIncrblobChannel(
    mut interp: *mut Tcl_Interp,
    mut pDb: *mut SqliteDb,
    mut zDb: *const ::core::ffi::c_char,
    mut zTable: *const ::core::ffi::c_char,
    mut zColumn: *const ::core::ffi::c_char,
    mut iRow: sqlite_int64,
    mut isReadonly: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut IncrblobChannel = ::core::ptr::null_mut::<IncrblobChannel>();
        let mut db: *mut sqlite3 = (*pDb).db;
        let mut pBlob: *mut sqlite3_blob = ::core::ptr::null_mut::<sqlite3_blob>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut flags: ::core::ffi::c_int = TCL_READABLE
            | (if isReadonly != 0 { 0 as ::core::ffi::c_int } else { TCL_WRITABLE });
        static mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut zChannel: [::core::ffi::c_char; 64] = [0; 64];
        rc = sqlite3_blob_open(
            db,
            zDb,
            zTable,
            zColumn,
            iRow as sqlite3_int64,
            (isReadonly == 0) as ::core::ffi::c_int,
            &raw mut pBlob,
        );
        if rc != SQLITE_OK {
            Tcl_SetResult(
                interp,
                sqlite3_errmsg((*pDb).db) as *mut ::core::ffi::c_char,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<Tcl_FreeProc>,
                >(1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            return TCL_ERROR;
        }
        p = Tcl_Alloc(::core::mem::size_of::<IncrblobChannel>() as ::core::ffi::c_uint)
            as *mut IncrblobChannel;
        memset(
            p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<IncrblobChannel>() as size_t,
        );
        (*p).pBlob = pBlob;
        if flags & TCL_WRITABLE == 0 as ::core::ffi::c_int {
            (*p).isClosed |= TCL_CLOSE_WRITE as ::core::ffi::c_uint;
        }
        count += 1;
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as ::core::ffi::c_int,
            &raw mut zChannel as *mut ::core::ffi::c_char,
            b"incrblob_%d\0".as_ptr() as *const ::core::ffi::c_char,
            count,
        );
        (*p).channel = Tcl_CreateChannel(
            &raw mut IncrblobChannelType,
            &raw mut zChannel as *mut ::core::ffi::c_char,
            p as ClientData,
            flags,
        );
        Tcl_RegisterChannel(interp, (*p).channel);
        (*p).pNext = (*pDb).pIncrblob;
        (*p).pPrev = ::core::ptr::null_mut::<IncrblobChannel>();
        if !(*p).pNext.is_null() {
            (*(*p).pNext).pPrev = p;
        }
        (*pDb).pIncrblob = p;
        (*p).pDb = pDb;
        Tcl_SetResult(
            interp,
            Tcl_GetChannelName((*p).channel) as *mut ::core::ffi::c_char,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<Tcl_FreeProc>,
            >(1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn safeToUseEvalObjv(mut pCmd: *mut Tcl_Obj) -> ::core::ffi::c_int {
    unsafe {
        let mut z: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut n: Tcl_Size = 0;
        z = Tcl_GetStringFromObj(pCmd, &raw mut n);
        loop {
            let c2rust_fresh0 = n;
            n = n - 1;
            if !(c2rust_fresh0 > 0 as ::core::ffi::c_int) {
                break;
            }
            let c2rust_fresh1 = z;
            z = z.offset(1);
            let mut c: ::core::ffi::c_int = *c2rust_fresh1 as ::core::ffi::c_int;
            if c == '$' as i32 || c == '[' as i32 || c == ';' as i32 {
                return 0 as ::core::ffi::c_int;
            }
        }
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn findSqlFunc(
    mut pDb: *mut SqliteDb,
    mut zName: *const ::core::ffi::c_char,
) -> *mut SqlFunc {
    unsafe {
        let mut p: *mut SqlFunc = ::core::ptr::null_mut::<SqlFunc>();
        let mut pNew: *mut SqlFunc = ::core::ptr::null_mut::<SqlFunc>();
        let mut nName: ::core::ffi::c_int = strlen30(zName);
        pNew = Tcl_Alloc(
            (::core::mem::size_of::<SqlFunc>() as usize)
                .wrapping_add(nName as usize)
                .wrapping_add(1 as usize) as ::core::ffi::c_uint,
        ) as *mut SqlFunc;
        (*pNew).zName = pNew.offset(1 as ::core::ffi::c_int as isize) as *mut SqlFunc
            as *mut ::core::ffi::c_char;
        memcpy(
            (*pNew).zName as *mut ::core::ffi::c_void,
            zName as *const ::core::ffi::c_void,
            (nName + 1 as ::core::ffi::c_int) as size_t,
        );
        p = (*pDb).pFunc;
        while !p.is_null() {
            if sqlite3_stricmp((*p).zName, (*pNew).zName) == 0 as ::core::ffi::c_int {
                Tcl_Free(pNew as *mut ::core::ffi::c_char);
                return p;
            }
            p = (*p).pNext;
        }
        (*pNew).interp = (*pDb).interp;
        (*pNew).pDb = pDb;
        (*pNew).pScript = ::core::ptr::null_mut::<Tcl_Obj>();
        (*pNew).pNext = (*pDb).pFunc;
        (*pDb).pFunc = pNew;
        return pNew;
    }
}
unsafe extern "C" fn dbFreeStmt(mut pStmt: *mut SqlPreparedStmt) {
    unsafe {
        sqlite3_finalize((*pStmt).pStmt);
        Tcl_Free(pStmt as *mut ::core::ffi::c_char);
    }
}
unsafe extern "C" fn flushStmtCache(mut pDb: *mut SqliteDb) {
    unsafe {
        let mut pPreStmt: *mut SqlPreparedStmt = ::core::ptr::null_mut::<
            SqlPreparedStmt,
        >();
        let mut pNext: *mut SqlPreparedStmt = ::core::ptr::null_mut::<SqlPreparedStmt>();
        pPreStmt = (*pDb).stmtList;
        while !pPreStmt.is_null() {
            pNext = (*pPreStmt).pNext;
            dbFreeStmt(pPreStmt);
            pPreStmt = pNext;
        }
        (*pDb).nStmt = 0 as ::core::ffi::c_int;
        (*pDb).stmtLast = ::core::ptr::null_mut::<SqlPreparedStmt>();
        (*pDb).stmtList = ::core::ptr::null_mut::<SqlPreparedStmt>();
    }
}
unsafe extern "C" fn addDatabaseRef(mut pDb: *mut SqliteDb) {
    unsafe {
        (*pDb).nRef += 1;
    }
}
unsafe extern "C" fn delDatabaseRef(mut pDb: *mut SqliteDb) {
    unsafe {
        '_c2rust_label: {
            if (*pDb).nRef > 0 as ::core::ffi::c_int {} else {
                __assert_fail(
                    b"pDb->nRef>0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/sqlite/src/tclsqlite.c\0".as_ptr() as *const ::core::ffi::c_char,
                    601 as ::core::ffi::c_uint,
                    b"void delDatabaseRef(SqliteDb *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        };
        (*pDb).nRef -= 1;
        if (*pDb).nRef == 0 as ::core::ffi::c_int {
            flushStmtCache(pDb);
            closeIncrblobChannels(pDb);
            sqlite3_close((*pDb).db);
            while !(*pDb).pFunc.is_null() {
                let mut pFunc: *mut SqlFunc = (*pDb).pFunc;
                (*pDb).pFunc = (*pFunc).pNext;
                '_c2rust_label_0: {
                    if (*pFunc).pDb == pDb {} else {
                        __assert_fail(
                            b"pFunc->pDb==pDb\0".as_ptr() as *const ::core::ffi::c_char,
                            b"/sqlite/src/tclsqlite.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            610 as ::core::ffi::c_uint,
                            b"void delDatabaseRef(SqliteDb *)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    }
                };
                let mut _objPtr: *mut Tcl_Obj = (*pFunc).pScript;
                let c2rust_fresh2 = (*_objPtr).refCount;
                (*_objPtr).refCount = (*_objPtr).refCount - 1;
                if c2rust_fresh2 <= 1 as ::core::ffi::c_int {
                    TclFreeObj(_objPtr);
                }
                Tcl_Free(pFunc as *mut ::core::ffi::c_char);
            }
            while !(*pDb).pCollate.is_null() {
                let mut pCollate: *mut SqlCollate = (*pDb).pCollate;
                (*pDb).pCollate = (*pCollate).pNext;
                Tcl_Free(pCollate as *mut ::core::ffi::c_char);
            }
            if !(*pDb).zBusy.is_null() {
                Tcl_Free((*pDb).zBusy);
            }
            if !(*pDb).zTrace.is_null() {
                Tcl_Free((*pDb).zTrace);
            }
            if !(*pDb).zTraceV2.is_null() {
                Tcl_Free((*pDb).zTraceV2);
            }
            if !(*pDb).zProfile.is_null() {
                Tcl_Free((*pDb).zProfile);
            }
            if !(*pDb).zBindFallback.is_null() {
                Tcl_Free((*pDb).zBindFallback);
            }
            if !(*pDb).zAuth.is_null() {
                Tcl_Free((*pDb).zAuth);
            }
            if !(*pDb).zNull.is_null() {
                Tcl_Free((*pDb).zNull);
            }
            if !(*pDb).pUpdateHook.is_null() {
                let mut _objPtr_0: *mut Tcl_Obj = (*pDb).pUpdateHook;
                let c2rust_fresh3 = (*_objPtr_0).refCount;
                (*_objPtr_0).refCount = (*_objPtr_0).refCount - 1;
                if c2rust_fresh3 <= 1 as ::core::ffi::c_int {
                    TclFreeObj(_objPtr_0);
                }
            }
            if !(*pDb).pPreUpdateHook.is_null() {
                let mut _objPtr_1: *mut Tcl_Obj = (*pDb).pPreUpdateHook;
                let c2rust_fresh4 = (*_objPtr_1).refCount;
                (*_objPtr_1).refCount = (*_objPtr_1).refCount - 1;
                if c2rust_fresh4 <= 1 as ::core::ffi::c_int {
                    TclFreeObj(_objPtr_1);
                }
            }
            if !(*pDb).pRollbackHook.is_null() {
                let mut _objPtr_2: *mut Tcl_Obj = (*pDb).pRollbackHook;
                let c2rust_fresh5 = (*_objPtr_2).refCount;
                (*_objPtr_2).refCount = (*_objPtr_2).refCount - 1;
                if c2rust_fresh5 <= 1 as ::core::ffi::c_int {
                    TclFreeObj(_objPtr_2);
                }
            }
            if !(*pDb).pWalHook.is_null() {
                let mut _objPtr_3: *mut Tcl_Obj = (*pDb).pWalHook;
                let c2rust_fresh6 = (*_objPtr_3).refCount;
                (*_objPtr_3).refCount = (*_objPtr_3).refCount - 1;
                if c2rust_fresh6 <= 1 as ::core::ffi::c_int {
                    TclFreeObj(_objPtr_3);
                }
            }
            if !(*pDb).pCollateNeeded.is_null() {
                let mut _objPtr_4: *mut Tcl_Obj = (*pDb).pCollateNeeded;
                let c2rust_fresh7 = (*_objPtr_4).refCount;
                (*_objPtr_4).refCount = (*_objPtr_4).refCount - 1;
                if c2rust_fresh7 <= 1 as ::core::ffi::c_int {
                    TclFreeObj(_objPtr_4);
                }
            }
            Tcl_Free(pDb as *mut ::core::ffi::c_char);
        }
    }
}
unsafe extern "C" fn DbDeleteCmd(mut db: *mut ::core::ffi::c_void) {
    unsafe {
        let mut pDb: *mut SqliteDb = db as *mut SqliteDb;
        delDatabaseRef(pDb);
    }
}
unsafe extern "C" fn DbBusyHandler(
    mut cd: *mut ::core::ffi::c_void,
    mut nTries: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pDb: *mut SqliteDb = cd as *mut SqliteDb;
        let mut rc: ::core::ffi::c_int = 0;
        let mut zVal: [::core::ffi::c_char; 30] = [0; 30];
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 30]>() as ::core::ffi::c_int,
            &raw mut zVal as *mut ::core::ffi::c_char,
            b"%d\0".as_ptr() as *const ::core::ffi::c_char,
            nTries,
        );
        rc = Tcl_VarEval(
            (*pDb).interp,
            (*pDb).zBusy,
            b" \0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut zVal as *mut ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        );
        if rc != TCL_OK || atoi(Tcl_GetStringResult((*pDb).interp)) != 0 {
            return 0 as ::core::ffi::c_int;
        }
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn DbProgressHandler(
    mut cd: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pDb: *mut SqliteDb = cd as *mut SqliteDb;
        let mut rc: ::core::ffi::c_int = 0;
        '_c2rust_label: {
            if !(*pDb).zProgress.is_null() {} else {
                __assert_fail(
                    b"pDb->zProgress\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/sqlite/src/tclsqlite.c\0".as_ptr() as *const ::core::ffi::c_char,
                    693 as ::core::ffi::c_uint,
                    b"int DbProgressHandler(void *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        };
        rc = Tcl_Eval((*pDb).interp, (*pDb).zProgress);
        if rc != TCL_OK || atoi(Tcl_GetStringResult((*pDb).interp)) != 0 {
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn DbTraceV2Handler(
    mut type_0: ::core::ffi::c_uint,
    mut cd: *mut ::core::ffi::c_void,
    mut pd: *mut ::core::ffi::c_void,
    mut xd: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pDb: *mut SqliteDb = cd as *mut SqliteDb;
        let mut pCmd: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        match type_0 {
            1 => {
                let mut pStmt: *mut sqlite3_stmt = pd as *mut sqlite3_stmt;
                let mut zSql: *mut ::core::ffi::c_char = xd as *mut ::core::ffi::c_char;
                pCmd = Tcl_NewStringObj((*pDb).zTraceV2, -1 as ::core::ffi::c_int);
                (*pCmd).refCount += 1;
                Tcl_ListObjAppendElement(
                    (*pDb).interp,
                    pCmd,
                    Tcl_NewWideIntObj(pStmt as uptr as Tcl_WideInt),
                );
                Tcl_ListObjAppendElement(
                    (*pDb).interp,
                    pCmd,
                    Tcl_NewStringObj(zSql, -1 as ::core::ffi::c_int),
                );
                Tcl_EvalObjEx((*pDb).interp, pCmd, TCL_EVAL_DIRECT);
                let mut _objPtr: *mut Tcl_Obj = pCmd;
                let c2rust_fresh8 = (*_objPtr).refCount;
                (*_objPtr).refCount = (*_objPtr).refCount - 1;
                if c2rust_fresh8 <= 1 as ::core::ffi::c_int {
                    TclFreeObj(_objPtr);
                }
                Tcl_ResetResult((*pDb).interp);
            }
            2 => {
                let mut pStmt_0: *mut sqlite3_stmt = pd as *mut sqlite3_stmt;
                let mut ns: sqlite3_int64 = *(xd as *mut sqlite3_int64);
                pCmd = Tcl_NewStringObj((*pDb).zTraceV2, -1 as ::core::ffi::c_int);
                (*pCmd).refCount += 1;
                Tcl_ListObjAppendElement(
                    (*pDb).interp,
                    pCmd,
                    Tcl_NewWideIntObj(pStmt_0 as uptr as Tcl_WideInt),
                );
                Tcl_ListObjAppendElement((*pDb).interp, pCmd, Tcl_NewWideIntObj(ns));
                Tcl_EvalObjEx((*pDb).interp, pCmd, TCL_EVAL_DIRECT);
                let mut _objPtr_0: *mut Tcl_Obj = pCmd;
                let c2rust_fresh9 = (*_objPtr_0).refCount;
                (*_objPtr_0).refCount = (*_objPtr_0).refCount - 1;
                if c2rust_fresh9 <= 1 as ::core::ffi::c_int {
                    TclFreeObj(_objPtr_0);
                }
                Tcl_ResetResult((*pDb).interp);
            }
            4 => {
                let mut pStmt_1: *mut sqlite3_stmt = pd as *mut sqlite3_stmt;
                pCmd = Tcl_NewStringObj((*pDb).zTraceV2, -1 as ::core::ffi::c_int);
                (*pCmd).refCount += 1;
                Tcl_ListObjAppendElement(
                    (*pDb).interp,
                    pCmd,
                    Tcl_NewWideIntObj(pStmt_1 as uptr as Tcl_WideInt),
                );
                Tcl_EvalObjEx((*pDb).interp, pCmd, TCL_EVAL_DIRECT);
                let mut _objPtr_1: *mut Tcl_Obj = pCmd;
                let c2rust_fresh10 = (*_objPtr_1).refCount;
                (*_objPtr_1).refCount = (*_objPtr_1).refCount - 1;
                if c2rust_fresh10 <= 1 as ::core::ffi::c_int {
                    TclFreeObj(_objPtr_1);
                }
                Tcl_ResetResult((*pDb).interp);
            }
            8 => {
                let mut db: *mut sqlite3 = pd as *mut sqlite3;
                pCmd = Tcl_NewStringObj((*pDb).zTraceV2, -1 as ::core::ffi::c_int);
                (*pCmd).refCount += 1;
                Tcl_ListObjAppendElement(
                    (*pDb).interp,
                    pCmd,
                    Tcl_NewWideIntObj(db as uptr as Tcl_WideInt),
                );
                Tcl_EvalObjEx((*pDb).interp, pCmd, TCL_EVAL_DIRECT);
                let mut _objPtr_2: *mut Tcl_Obj = pCmd;
                let c2rust_fresh11 = (*_objPtr_2).refCount;
                (*_objPtr_2).refCount = (*_objPtr_2).refCount - 1;
                if c2rust_fresh11 <= 1 as ::core::ffi::c_int {
                    TclFreeObj(_objPtr_2);
                }
                Tcl_ResetResult((*pDb).interp);
            }
            _ => {}
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn DbCommitHandler(
    mut cd: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pDb: *mut SqliteDb = cd as *mut SqliteDb;
        let mut rc: ::core::ffi::c_int = 0;
        rc = Tcl_Eval((*pDb).interp, (*pDb).zCommit);
        if rc != TCL_OK || atoi(Tcl_GetStringResult((*pDb).interp)) != 0 {
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn DbRollbackHandler(mut clientData: *mut ::core::ffi::c_void) {
    unsafe {
        let mut pDb: *mut SqliteDb = clientData as *mut SqliteDb;
        '_c2rust_label: {
            if !(*pDb).pRollbackHook.is_null() {} else {
                __assert_fail(
                    b"pDb->pRollbackHook\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/sqlite/src/tclsqlite.c\0".as_ptr() as *const ::core::ffi::c_char,
                    838 as ::core::ffi::c_uint,
                    b"void DbRollbackHandler(void *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        };
        if TCL_OK
            != Tcl_EvalObjEx(
                (*pDb).interp,
                (*pDb).pRollbackHook,
                0 as ::core::ffi::c_int,
            )
        {
            Tcl_BackgroundError((*pDb).interp);
        }
    }
}
unsafe extern "C" fn DbWalHandler(
    mut clientData: *mut ::core::ffi::c_void,
    mut db: *mut sqlite3,
    mut zDb: *const ::core::ffi::c_char,
    mut nEntry: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ret: ::core::ffi::c_int = SQLITE_OK;
        let mut p: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut pDb: *mut SqliteDb = clientData as *mut SqliteDb;
        let mut interp: *mut Tcl_Interp = (*pDb).interp;
        '_c2rust_label: {
            if !(*pDb).pWalHook.is_null() {} else {
                __assert_fail(
                    b"pDb->pWalHook\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/sqlite/src/tclsqlite.c\0".as_ptr() as *const ::core::ffi::c_char,
                    857 as ::core::ffi::c_uint,
                    b"int DbWalHandler(void *, sqlite3 *, const char *, int)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        };
        '_c2rust_label_0: {
            if db == (*pDb).db {} else {
                __assert_fail(
                    b"db==pDb->db\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/sqlite/src/tclsqlite.c\0".as_ptr() as *const ::core::ffi::c_char,
                    859 as ::core::ffi::c_uint,
                    b"int DbWalHandler(void *, sqlite3 *, const char *, int)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        };
        p = Tcl_DuplicateObj((*pDb).pWalHook);
        (*p).refCount += 1;
        Tcl_ListObjAppendElement(
            interp,
            p,
            Tcl_NewStringObj(zDb, -1 as ::core::ffi::c_int),
        );
        Tcl_ListObjAppendElement(interp, p, Tcl_NewIntObj(nEntry));
        if TCL_OK != Tcl_EvalObjEx(interp, p, 0 as ::core::ffi::c_int)
            || TCL_OK
                != Tcl_GetIntFromObj(interp, Tcl_GetObjResult(interp), &raw mut ret)
        {
            Tcl_BackgroundError(interp);
        }
        let mut _objPtr: *mut Tcl_Obj = p;
        let c2rust_fresh12 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh12 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
        return ret;
    }
}
unsafe extern "C" fn DbUpdateHandler(
    mut p: *mut ::core::ffi::c_void,
    mut op: ::core::ffi::c_int,
    mut zDb: *const ::core::ffi::c_char,
    mut zTbl: *const ::core::ffi::c_char,
    mut rowid: sqlite_int64,
) {
    unsafe {
        let mut pDb: *mut SqliteDb = p as *mut SqliteDb;
        let mut pCmd: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        static mut azStr: [*const ::core::ffi::c_char; 3] = [
            b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
            b"INSERT\0".as_ptr() as *const ::core::ffi::c_char,
            b"UPDATE\0".as_ptr() as *const ::core::ffi::c_char,
        ];
        '_c2rust_label: {
            if (9 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                / 9 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {} else {
                __assert_fail(
                    b"(SQLITE_DELETE-1)/9 == 0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/sqlite/src/tclsqlite.c\0".as_ptr() as *const ::core::ffi::c_char,
                    948 as ::core::ffi::c_uint,
                    b"void DbUpdateHandler(void *, int, const char *, const char *, sqlite_int64)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
        };
        '_c2rust_label_0: {
            if (18 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                / 9 as ::core::ffi::c_int == 1 as ::core::ffi::c_int
            {} else {
                __assert_fail(
                    b"(SQLITE_INSERT-1)/9 == 1\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/sqlite/src/tclsqlite.c\0".as_ptr() as *const ::core::ffi::c_char,
                    949 as ::core::ffi::c_uint,
                    b"void DbUpdateHandler(void *, int, const char *, const char *, sqlite_int64)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
        };
        '_c2rust_label_1: {
            if (23 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                / 9 as ::core::ffi::c_int == 2 as ::core::ffi::c_int
            {} else {
                __assert_fail(
                    b"(SQLITE_UPDATE-1)/9 == 2\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/sqlite/src/tclsqlite.c\0".as_ptr() as *const ::core::ffi::c_char,
                    950 as ::core::ffi::c_uint,
                    b"void DbUpdateHandler(void *, int, const char *, const char *, sqlite_int64)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
        };
        '_c2rust_label_2: {
            if !(*pDb).pUpdateHook.is_null() {} else {
                __assert_fail(
                    b"pDb->pUpdateHook\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/sqlite/src/tclsqlite.c\0".as_ptr() as *const ::core::ffi::c_char,
                    952 as ::core::ffi::c_uint,
                    b"void DbUpdateHandler(void *, int, const char *, const char *, sqlite_int64)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
        };
        '_c2rust_label_3: {
            if op == 18 as ::core::ffi::c_int || op == 23 as ::core::ffi::c_int
                || op == 9 as ::core::ffi::c_int
            {} else {
                __assert_fail(
                    b"op==SQLITE_INSERT || op==SQLITE_UPDATE || op==SQLITE_DELETE\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"/sqlite/src/tclsqlite.c\0".as_ptr() as *const ::core::ffi::c_char,
                    953 as ::core::ffi::c_uint,
                    b"void DbUpdateHandler(void *, int, const char *, const char *, sqlite_int64)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
        };
        pCmd = Tcl_DuplicateObj((*pDb).pUpdateHook);
        (*pCmd).refCount += 1;
        Tcl_ListObjAppendElement(
            ::core::ptr::null_mut::<Tcl_Interp>(),
            pCmd,
            Tcl_NewStringObj(
                azStr[((op - 1 as ::core::ffi::c_int) / 9 as ::core::ffi::c_int)
                    as usize],
                -1 as ::core::ffi::c_int,
            ),
        );
        Tcl_ListObjAppendElement(
            ::core::ptr::null_mut::<Tcl_Interp>(),
            pCmd,
            Tcl_NewStringObj(zDb, -1 as ::core::ffi::c_int),
        );
        Tcl_ListObjAppendElement(
            ::core::ptr::null_mut::<Tcl_Interp>(),
            pCmd,
            Tcl_NewStringObj(zTbl, -1 as ::core::ffi::c_int),
        );
        Tcl_ListObjAppendElement(
            ::core::ptr::null_mut::<Tcl_Interp>(),
            pCmd,
            Tcl_NewWideIntObj(rowid as Tcl_WideInt),
        );
        Tcl_EvalObjEx((*pDb).interp, pCmd, TCL_EVAL_DIRECT);
        let mut _objPtr: *mut Tcl_Obj = pCmd;
        let c2rust_fresh13 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh13 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
    }
}
unsafe extern "C" fn tclCollateNeeded(
    mut pCtx: *mut ::core::ffi::c_void,
    mut db: *mut sqlite3,
    mut enc: ::core::ffi::c_int,
    mut zName: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut pDb: *mut SqliteDb = pCtx as *mut SqliteDb;
        let mut pScript: *mut Tcl_Obj = Tcl_DuplicateObj((*pDb).pCollateNeeded);
        (*pScript).refCount += 1;
        Tcl_ListObjAppendElement(
            ::core::ptr::null_mut::<Tcl_Interp>(),
            pScript,
            Tcl_NewStringObj(zName, -1 as ::core::ffi::c_int),
        );
        Tcl_EvalObjEx((*pDb).interp, pScript, 0 as ::core::ffi::c_int);
        let mut _objPtr: *mut Tcl_Obj = pScript;
        let c2rust_fresh14 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh14 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
    }
}
unsafe extern "C" fn tclSqlCollate(
    mut pCtx: *mut ::core::ffi::c_void,
    mut nA: ::core::ffi::c_int,
    mut zA: *const ::core::ffi::c_void,
    mut nB: ::core::ffi::c_int,
    mut zB: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut SqlCollate = pCtx as *mut SqlCollate;
        let mut pCmd: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        pCmd = Tcl_NewStringObj((*p).zScript, -1 as ::core::ffi::c_int);
        (*pCmd).refCount += 1;
        Tcl_ListObjAppendElement(
            (*p).interp,
            pCmd,
            Tcl_NewStringObj(zA as *const ::core::ffi::c_char, nA),
        );
        Tcl_ListObjAppendElement(
            (*p).interp,
            pCmd,
            Tcl_NewStringObj(zB as *const ::core::ffi::c_char, nB),
        );
        Tcl_EvalObjEx((*p).interp, pCmd, TCL_EVAL_DIRECT);
        let mut _objPtr: *mut Tcl_Obj = pCmd;
        let c2rust_fresh15 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh15 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
        return atoi(Tcl_GetStringResult((*p).interp));
    }
}
unsafe extern "C" fn tclSqlFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut SqlFunc = sqlite3_user_data(context) as *mut SqlFunc;
        let mut pCmd: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut i: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        if argc == 0 as ::core::ffi::c_int {
            pCmd = (*p).pScript;
            (*pCmd).refCount += 1;
            rc = Tcl_EvalObjEx((*p).interp, pCmd, 0 as ::core::ffi::c_int);
            let mut _objPtr: *mut Tcl_Obj = pCmd;
            let c2rust_fresh16 = (*_objPtr).refCount;
            (*_objPtr).refCount = (*_objPtr).refCount - 1;
            if c2rust_fresh16 <= 1 as ::core::ffi::c_int {
                TclFreeObj(_objPtr);
            }
        } else {
            let mut aArg: *mut *mut Tcl_Obj = ::core::ptr::null_mut::<*mut Tcl_Obj>();
            let mut nArg: Tcl_Size = 0;
            if Tcl_ListObjGetElements(
                (*p).interp,
                (*p).pScript,
                &raw mut nArg,
                &raw mut aArg,
            ) != 0
            {
                sqlite3_result_error(
                    context,
                    Tcl_GetStringResult((*p).interp),
                    -1 as ::core::ffi::c_int,
                );
                return;
            }
            pCmd = Tcl_NewListObj(
                nArg as ::core::ffi::c_int,
                aArg as *const *mut Tcl_Obj,
            );
            (*pCmd).refCount += 1;
            i = 0 as ::core::ffi::c_int;
            while i < argc {
                let mut pIn: *mut sqlite3_value = *argv.offset(i as isize);
                let mut pVal: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
                match sqlite3_value_type(pIn) {
                    SQLITE_BLOB => {
                        let mut bytes: ::core::ffi::c_int = sqlite3_value_bytes(pIn);
                        pVal = Tcl_NewByteArrayObj(
                            sqlite3_value_blob(pIn) as *const ::core::ffi::c_uchar,
                            bytes,
                        );
                    }
                    SQLITE_INTEGER => {
                        let mut v: sqlite_int64 = sqlite3_value_int64(pIn)
                            as sqlite_int64;
                        if v
                            >= -2147483647 as ::core::ffi::c_int
                                as ::core::ffi::c_longlong
                            && v <= 2147483647 as ::core::ffi::c_longlong
                        {
                            pVal = Tcl_NewIntObj(v as ::core::ffi::c_int);
                        } else {
                            pVal = Tcl_NewWideIntObj(v as Tcl_WideInt);
                        }
                    }
                    SQLITE_FLOAT => {
                        let mut r: ::core::ffi::c_double = sqlite3_value_double(pIn);
                        pVal = Tcl_NewDoubleObj(r);
                    }
                    SQLITE_NULL => {
                        pVal = Tcl_NewStringObj(
                            (*(*p).pDb).zNull,
                            -1 as ::core::ffi::c_int,
                        );
                    }
                    _ => {
                        let mut bytes_0: ::core::ffi::c_int = sqlite3_value_bytes(pIn);
                        pVal = Tcl_NewStringObj(
                            sqlite3_value_text(pIn) as *mut ::core::ffi::c_char,
                            bytes_0,
                        );
                    }
                }
                rc = Tcl_ListObjAppendElement((*p).interp, pCmd, pVal);
                if rc != 0 {
                    let mut _objPtr_0: *mut Tcl_Obj = pCmd;
                    let c2rust_fresh17 = (*_objPtr_0).refCount;
                    (*_objPtr_0).refCount = (*_objPtr_0).refCount - 1;
                    if c2rust_fresh17 <= 1 as ::core::ffi::c_int {
                        TclFreeObj(_objPtr_0);
                    }
                    sqlite3_result_error(
                        context,
                        Tcl_GetStringResult((*p).interp),
                        -1 as ::core::ffi::c_int,
                    );
                    return;
                }
                i += 1;
            }
            if (*p).useEvalObjv == 0 {
                Tcl_GetString(pCmd);
            }
            rc = Tcl_EvalObjEx((*p).interp, pCmd, TCL_EVAL_DIRECT);
            let mut _objPtr_1: *mut Tcl_Obj = pCmd;
            let c2rust_fresh18 = (*_objPtr_1).refCount;
            (*_objPtr_1).refCount = (*_objPtr_1).refCount - 1;
            if c2rust_fresh18 <= 1 as ::core::ffi::c_int {
                TclFreeObj(_objPtr_1);
            }
        }
        if TCL_BREAK == rc {
            sqlite3_result_null(context);
        } else if rc != 0 && rc != TCL_RETURN {
            sqlite3_result_error(
                context,
                Tcl_GetStringResult((*p).interp),
                -1 as ::core::ffi::c_int,
            );
        } else {
            let mut pVar: *mut Tcl_Obj = Tcl_GetObjResult((*p).interp);
            let mut n: Tcl_Size = 0;
            let mut data: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
            let mut zType: *const ::core::ffi::c_char = if !(*pVar).typePtr.is_null() {
                (*(*pVar).typePtr).name
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            };
            let mut c: ::core::ffi::c_char = *zType
                .offset(0 as ::core::ffi::c_int as isize);
            let mut eType: ::core::ffi::c_int = (*p).eType;
            if eType == SQLITE_NULL {
                if c as ::core::ffi::c_int == 'b' as i32
                    && strcmp(
                        zType,
                        b"bytearray\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int && (*pVar).bytes.is_null()
                {
                    eType = SQLITE_BLOB;
                } else if c as ::core::ffi::c_int == 'b' as i32
                    && (*pVar).bytes.is_null()
                    && strcmp(zType, b"boolean\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    || c as ::core::ffi::c_int == 'b' as i32 && (*pVar).bytes.is_null()
                        && strcmp(
                            zType,
                            b"booleanString\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                    || c as ::core::ffi::c_int == 'w' as i32
                        && strcmp(
                            zType,
                            b"wideInt\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                    || c as ::core::ffi::c_int == 'i' as i32
                        && strcmp(zType, b"int\0".as_ptr() as *const ::core::ffi::c_char)
                            == 0 as ::core::ffi::c_int
                {
                    eType = SQLITE_INTEGER;
                } else if c as ::core::ffi::c_int == 'd' as i32
                    && strcmp(zType, b"double\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                {
                    eType = SQLITE_FLOAT;
                } else {
                    eType = SQLITE_TEXT;
                }
            }
            let mut c2rust_current_block_71: u64;
            match eType {
                SQLITE_BLOB => {
                    data = Tcl_GetByteArrayFromObj(pVar, &raw mut n) as *mut u8_0;
                    sqlite3_result_blob(
                        context,
                        data as *const ::core::ffi::c_void,
                        n as ::core::ffi::c_int,
                        ::core::mem::transmute::<
                            ::libc::intptr_t,
                            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                    );
                    c2rust_current_block_71 = 10778260831612459202;
                }
                SQLITE_INTEGER => {
                    let mut v_0: Tcl_WideInt = 0;
                    if TCL_OK
                        == Tcl_GetWideIntFromObj(
                            ::core::ptr::null_mut::<Tcl_Interp>(),
                            pVar,
                            &raw mut v_0,
                        )
                    {
                        sqlite3_result_int64(context, v_0 as sqlite3_int64);
                        c2rust_current_block_71 = 10778260831612459202;
                    } else {
                        c2rust_current_block_71 = 6367734732029634840;
                    }
                }
                SQLITE_FLOAT => {
                    c2rust_current_block_71 = 6367734732029634840;
                }
                _ => {
                    c2rust_current_block_71 = 10435735846551762309;
                }
            }
            match c2rust_current_block_71 {
                6367734732029634840 => {
                    let mut r_0: ::core::ffi::c_double = 0.;
                    if TCL_OK
                        == Tcl_GetDoubleFromObj(
                            ::core::ptr::null_mut::<Tcl_Interp>(),
                            pVar,
                            &raw mut r_0,
                        )
                    {
                        sqlite3_result_double(context, r_0);
                        c2rust_current_block_71 = 10778260831612459202;
                    } else {
                        c2rust_current_block_71 = 10435735846551762309;
                    }
                }
                _ => {}
            }
            match c2rust_current_block_71 {
                10435735846551762309 => {
                    data = Tcl_GetStringFromObj(pVar, &raw mut n)
                        as *mut ::core::ffi::c_uchar as *mut u8_0;
                    sqlite3_result_text64(
                        context,
                        data as *mut ::core::ffi::c_char,
                        n as sqlite3_uint64,
                        ::core::mem::transmute::<
                            ::libc::intptr_t,
                            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                        SQLITE_UTF8 as ::core::ffi::c_uchar,
                    );
                }
                _ => {}
            }
        };
    }
}
unsafe extern "C" fn auth_callback(
    mut pArg: *mut ::core::ffi::c_void,
    mut code: ::core::ffi::c_int,
    mut zArg1: *const ::core::ffi::c_char,
    mut zArg2: *const ::core::ffi::c_char,
    mut zArg3: *const ::core::ffi::c_char,
    mut zArg4: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zCode: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut str: Tcl_DString = Tcl_DString {
            string: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            length: 0,
            spaceAvl: 0,
            staticSpace: [0; 200]

        };
        let mut rc: ::core::ffi::c_int = 0;
        let mut zReply: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut pDb: *mut SqliteDb = pArg as *mut SqliteDb;
        if (*pDb).disableAuth != 0 {
            return SQLITE_OK;
        }
        match code {
            SQLITE_COPY => {
                zCode = b"SQLITE_COPY\0".as_ptr() as *const ::core::ffi::c_char;
            }
            SQLITE_CREATE_INDEX => {
                zCode = b"SQLITE_CREATE_INDEX\0".as_ptr() as *const ::core::ffi::c_char;
            }
            SQLITE_CREATE_TABLE => {
                zCode = b"SQLITE_CREATE_TABLE\0".as_ptr() as *const ::core::ffi::c_char;
            }
            SQLITE_CREATE_TEMP_INDEX => {
                zCode = b"SQLITE_CREATE_TEMP_INDEX\0".as_ptr()
                    as *const ::core::ffi::c_char;
            }
            SQLITE_CREATE_TEMP_TABLE => {
                zCode = b"SQLITE_CREATE_TEMP_TABLE\0".as_ptr()
                    as *const ::core::ffi::c_char;
            }
            SQLITE_CREATE_TEMP_TRIGGER => {
                zCode = b"SQLITE_CREATE_TEMP_TRIGGER\0".as_ptr()
                    as *const ::core::ffi::c_char;
            }
            SQLITE_CREATE_TEMP_VIEW => {
                zCode = b"SQLITE_CREATE_TEMP_VIEW\0".as_ptr()
                    as *const ::core::ffi::c_char;
            }
            SQLITE_CREATE_TRIGGER => {
                zCode = b"SQLITE_CREATE_TRIGGER\0".as_ptr()
                    as *const ::core::ffi::c_char;
            }
            SQLITE_CREATE_VIEW => {
                zCode = b"SQLITE_CREATE_VIEW\0".as_ptr() as *const ::core::ffi::c_char;
            }
            SQLITE_DELETE => {
                zCode = b"SQLITE_DELETE\0".as_ptr() as *const ::core::ffi::c_char;
            }
            SQLITE_DROP_INDEX => {
                zCode = b"SQLITE_DROP_INDEX\0".as_ptr() as *const ::core::ffi::c_char;
            }
            SQLITE_DROP_TABLE => {
                zCode = b"SQLITE_DROP_TABLE\0".as_ptr() as *const ::core::ffi::c_char;
            }
            SQLITE_DROP_TEMP_INDEX => {
                zCode = b"SQLITE_DROP_TEMP_INDEX\0".as_ptr()
                    as *const ::core::ffi::c_char;
            }
            SQLITE_DROP_TEMP_TABLE => {
                zCode = b"SQLITE_DROP_TEMP_TABLE\0".as_ptr()
                    as *const ::core::ffi::c_char;
            }
            SQLITE_DROP_TEMP_TRIGGER => {
                zCode = b"SQLITE_DROP_TEMP_TRIGGER\0".as_ptr()
                    as *const ::core::ffi::c_char;
            }
            SQLITE_DROP_TEMP_VIEW => {
                zCode = b"SQLITE_DROP_TEMP_VIEW\0".as_ptr()
                    as *const ::core::ffi::c_char;
            }
            SQLITE_DROP_TRIGGER => {
                zCode = b"SQLITE_DROP_TRIGGER\0".as_ptr() as *const ::core::ffi::c_char;
            }
            SQLITE_DROP_VIEW => {
                zCode = b"SQLITE_DROP_VIEW\0".as_ptr() as *const ::core::ffi::c_char;
            }
            SQLITE_INSERT => {
                zCode = b"SQLITE_INSERT\0".as_ptr() as *const ::core::ffi::c_char;
            }
            SQLITE_PRAGMA => {
                zCode = b"SQLITE_PRAGMA\0".as_ptr() as *const ::core::ffi::c_char;
            }
            SQLITE_READ => {
                zCode = b"SQLITE_READ\0".as_ptr() as *const ::core::ffi::c_char;
            }
            SQLITE_SELECT => {
                zCode = b"SQLITE_SELECT\0".as_ptr() as *const ::core::ffi::c_char;
            }
            SQLITE_TRANSACTION => {
                zCode = b"SQLITE_TRANSACTION\0".as_ptr() as *const ::core::ffi::c_char;
            }
            SQLITE_UPDATE => {
                zCode = b"SQLITE_UPDATE\0".as_ptr() as *const ::core::ffi::c_char;
            }
            SQLITE_ATTACH => {
                zCode = b"SQLITE_ATTACH\0".as_ptr() as *const ::core::ffi::c_char;
            }
            SQLITE_DETACH => {
                zCode = b"SQLITE_DETACH\0".as_ptr() as *const ::core::ffi::c_char;
            }
            SQLITE_ALTER_TABLE => {
                zCode = b"SQLITE_ALTER_TABLE\0".as_ptr() as *const ::core::ffi::c_char;
            }
            SQLITE_REINDEX => {
                zCode = b"SQLITE_REINDEX\0".as_ptr() as *const ::core::ffi::c_char;
            }
            SQLITE_ANALYZE => {
                zCode = b"SQLITE_ANALYZE\0".as_ptr() as *const ::core::ffi::c_char;
            }
            SQLITE_CREATE_VTABLE => {
                zCode = b"SQLITE_CREATE_VTABLE\0".as_ptr() as *const ::core::ffi::c_char;
            }
            SQLITE_DROP_VTABLE => {
                zCode = b"SQLITE_DROP_VTABLE\0".as_ptr() as *const ::core::ffi::c_char;
            }
            SQLITE_FUNCTION => {
                zCode = b"SQLITE_FUNCTION\0".as_ptr() as *const ::core::ffi::c_char;
            }
            SQLITE_SAVEPOINT => {
                zCode = b"SQLITE_SAVEPOINT\0".as_ptr() as *const ::core::ffi::c_char;
            }
            SQLITE_RECURSIVE => {
                zCode = b"SQLITE_RECURSIVE\0".as_ptr() as *const ::core::ffi::c_char;
            }
            _ => {
                zCode = b"????\0".as_ptr() as *const ::core::ffi::c_char;
            }
        }
        Tcl_DStringInit(&raw mut str);
        Tcl_DStringAppend(&raw mut str, (*pDb).zAuth, -1 as ::core::ffi::c_int);
        Tcl_DStringAppendElement(&raw mut str, zCode);
        Tcl_DStringAppendElement(
            &raw mut str,
            if !zArg1.is_null() {
                zArg1
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        Tcl_DStringAppendElement(
            &raw mut str,
            if !zArg2.is_null() {
                zArg2
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        Tcl_DStringAppendElement(
            &raw mut str,
            if !zArg3.is_null() {
                zArg3
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        Tcl_DStringAppendElement(
            &raw mut str,
            if !zArg4.is_null() {
                zArg4
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        rc = Tcl_GlobalEval((*pDb).interp, str.string);
        Tcl_DStringFree(&raw mut str);
        zReply = if rc == TCL_OK {
            Tcl_GetStringResult((*pDb).interp)
        } else {
            b"SQLITE_DENY\0".as_ptr() as *const ::core::ffi::c_char
        };
        if strcmp(zReply, b"SQLITE_OK\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            rc = SQLITE_OK;
        } else if strcmp(zReply, b"SQLITE_DENY\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            rc = SQLITE_DENY;
        } else if strcmp(
            zReply,
            b"SQLITE_IGNORE\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            rc = SQLITE_IGNORE;
        } else {
            rc = 999 as ::core::ffi::c_int;
        }
        return rc;
    }
}
unsafe extern "C" fn DbTransPostCmd(
    mut data: *mut ClientData,
    mut interp: *mut Tcl_Interp,
    mut result: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        static mut azEnd: [*const ::core::ffi::c_char; 4] = [
            b"RELEASE _tcl_transaction\0".as_ptr() as *const ::core::ffi::c_char,
            b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char,
            b"ROLLBACK TO _tcl_transaction ; RELEASE _tcl_transaction\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"ROLLBACK\0".as_ptr() as *const ::core::ffi::c_char,
        ];
        let mut pDb: *mut SqliteDb = *data.offset(0 as ::core::ffi::c_int as isize)
            as *mut SqliteDb;
        let mut rc: ::core::ffi::c_int = result;
        let mut zEnd: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        (*pDb).nTransaction -= 1;
        zEnd = azEnd[((rc == TCL_ERROR) as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            + ((*pDb).nTransaction == 0 as ::core::ffi::c_int) as ::core::ffi::c_int)
            as usize];
        (*pDb).disableAuth += 1;
        if sqlite3_exec(
            (*pDb).db,
            zEnd,
            None,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ) != 0
        {
            if rc != TCL_ERROR {
                Tcl_AppendResult(
                    interp,
                    sqlite3_errmsg((*pDb).db),
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                );
                rc = TCL_ERROR;
            }
            sqlite3_exec(
                (*pDb).db,
                b"ROLLBACK\0".as_ptr() as *const ::core::ffi::c_char,
                None,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
        }
        (*pDb).disableAuth -= 1;
        delDatabaseRef(pDb);
        return rc;
    }
}
unsafe extern "C" fn dbPrepare(
    mut pDb: *mut SqliteDb,
    mut zSql: *const ::core::ffi::c_char,
    mut ppStmt: *mut *mut sqlite3_stmt,
    mut pzOut: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut prepFlags: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        if (*pDb).maxStmt > 5 as ::core::ffi::c_int {
            prepFlags = SQLITE_PREPARE_PERSISTENT as ::core::ffi::c_uint;
        }
        return sqlite3_prepare_v3(
            (*pDb).db,
            zSql,
            -1 as ::core::ffi::c_int,
            prepFlags,
            ppStmt,
            pzOut,
        );
    }
}
unsafe extern "C" fn dbPrepareAndBind(
    mut pDb: *mut SqliteDb,
    mut zIn: *const ::core::ffi::c_char,
    mut pzOut: *mut *const ::core::ffi::c_char,
    mut ppPreStmt: *mut *mut SqlPreparedStmt,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zSql: *const ::core::ffi::c_char = zIn;
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut pPreStmt: *mut SqlPreparedStmt = ::core::ptr::null_mut::<
            SqlPreparedStmt,
        >();
        let mut nSql: ::core::ffi::c_int = 0;
        let mut nVar: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iParm: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut c: ::core::ffi::c_char = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut needResultReset: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut interp: *mut Tcl_Interp = (*pDb).interp;
        *ppPreStmt = ::core::ptr::null_mut::<SqlPreparedStmt>();
        loop {
            c = *zSql.offset(0 as ::core::ffi::c_int as isize);
            if !(c as ::core::ffi::c_int == ' ' as i32
                || c as ::core::ffi::c_int == '\t' as i32
                || c as ::core::ffi::c_int == '\r' as i32
                || c as ::core::ffi::c_int == '\n' as i32)
            {
                break;
            }
            zSql = zSql.offset(1);
        }
        nSql = strlen30(zSql);
        pPreStmt = (*pDb).stmtList;
        while !pPreStmt.is_null() {
            let mut n: ::core::ffi::c_int = (*pPreStmt).nSql;
            if nSql >= n
                && memcmp(
                    (*pPreStmt).zSql as *const ::core::ffi::c_void,
                    zSql as *const ::core::ffi::c_void,
                    n as size_t,
                ) == 0 as ::core::ffi::c_int
                && (*zSql.offset(n as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                    || *zSql.offset((n - 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int == ';' as i32)
            {
                pStmt = (*pPreStmt).pStmt;
                *pzOut = zSql.offset((*pPreStmt).nSql as isize)
                    as *const ::core::ffi::c_char;
                if !(*pPreStmt).pPrev.is_null() {
                    (*(*pPreStmt).pPrev).pNext = (*pPreStmt).pNext;
                } else {
                    (*pDb).stmtList = (*pPreStmt).pNext;
                }
                if !(*pPreStmt).pNext.is_null() {
                    (*(*pPreStmt).pNext).pPrev = (*pPreStmt).pPrev;
                } else {
                    (*pDb).stmtLast = (*pPreStmt).pPrev;
                }
                (*pDb).nStmt -= 1;
                nVar = sqlite3_bind_parameter_count(pStmt);
                break;
            } else {
                pPreStmt = (*pPreStmt).pNext;
            }
        }
        if pPreStmt.is_null() {
            let mut nByte: ::core::ffi::c_int = 0;
            if SQLITE_OK != dbPrepare(pDb, zSql, &raw mut pStmt, pzOut) {
                Tcl_SetObjResult(
                    interp,
                    Tcl_NewStringObj(sqlite3_errmsg((*pDb).db), -1 as ::core::ffi::c_int),
                );
                return TCL_ERROR;
            }
            if pStmt.is_null() {
                if SQLITE_OK != sqlite3_errcode((*pDb).db) {
                    Tcl_SetObjResult(
                        interp,
                        Tcl_NewStringObj(
                            sqlite3_errmsg((*pDb).db),
                            -1 as ::core::ffi::c_int,
                        ),
                    );
                    return TCL_ERROR;
                } else {
                    return TCL_OK
                }
            }
            '_c2rust_label: {
                if pPreStmt.is_null() {} else {
                    __assert_fail(
                        b"pPreStmt==0\0".as_ptr() as *const ::core::ffi::c_char,
                        b"/sqlite/src/tclsqlite.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1458 as ::core::ffi::c_uint,
                        b"int dbPrepareAndBind(SqliteDb *, const char *, const char **, SqlPreparedStmt **)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            };
            nVar = sqlite3_bind_parameter_count(pStmt);
            nByte = (::core::mem::size_of::<SqlPreparedStmt>() as usize)
                .wrapping_add(
                    (nVar as usize)
                        .wrapping_mul(::core::mem::size_of::<*mut Tcl_Obj>() as usize),
                ) as ::core::ffi::c_int;
            pPreStmt = Tcl_Alloc(nByte as ::core::ffi::c_uint) as *mut SqlPreparedStmt;
            memset(
                pPreStmt as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                nByte as size_t,
            );
            (*pPreStmt).pStmt = pStmt;
            (*pPreStmt).nSql = (*pzOut).offset_from(zSql) as ::core::ffi::c_long
                as ::core::ffi::c_int;
            (*pPreStmt).zSql = sqlite3_sql(pStmt);
            (*pPreStmt).apParm = pPreStmt.offset(1 as ::core::ffi::c_int as isize)
                as *mut SqlPreparedStmt as *mut *mut Tcl_Obj;
        }
        '_c2rust_label_0: {
            if !pPreStmt.is_null() {} else {
                __assert_fail(
                    b"pPreStmt\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/sqlite/src/tclsqlite.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1477 as ::core::ffi::c_uint,
                    b"int dbPrepareAndBind(SqliteDb *, const char *, const char **, SqlPreparedStmt **)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
        };
        '_c2rust_label_1: {
            if strlen30((*pPreStmt).zSql) == (*pPreStmt).nSql {} else {
                __assert_fail(
                    b"strlen30(pPreStmt->zSql)==pPreStmt->nSql\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/sqlite/src/tclsqlite.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1478 as ::core::ffi::c_uint,
                    b"int dbPrepareAndBind(SqliteDb *, const char *, const char **, SqlPreparedStmt **)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
        };
        '_c2rust_label_2: {
            if 0 as ::core::ffi::c_int
                == memcmp(
                    (*pPreStmt).zSql as *const ::core::ffi::c_void,
                    zSql as *const ::core::ffi::c_void,
                    (*pPreStmt).nSql as size_t,
                )
            {} else {
                __assert_fail(
                    b"0==memcmp(pPreStmt->zSql, zSql, pPreStmt->nSql)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/sqlite/src/tclsqlite.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1479 as ::core::ffi::c_uint,
                    b"int dbPrepareAndBind(SqliteDb *, const char *, const char **, SqlPreparedStmt **)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
        };
        i = 1 as ::core::ffi::c_int;
        while i <= nVar {
            let mut zVar: *const ::core::ffi::c_char = sqlite3_bind_parameter_name(
                pStmt,
                i,
            );
            if !zVar.is_null()
                && (*zVar.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '$' as i32
                    || *zVar.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == ':' as i32
                    || *zVar.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == '@' as i32)
            {
                let mut pVar: *mut Tcl_Obj = Tcl_GetVar2Ex(
                    interp,
                    zVar.offset(1 as ::core::ffi::c_int as isize)
                        as *const ::core::ffi::c_char,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    0 as ::core::ffi::c_int,
                );
                if pVar.is_null() && !(*pDb).zBindFallback.is_null() {
                    let mut pCmd: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
                    let mut rx: ::core::ffi::c_int = 0;
                    pCmd = Tcl_NewStringObj(
                        (*pDb).zBindFallback,
                        -1 as ::core::ffi::c_int,
                    );
                    (*pCmd).refCount += 1;
                    Tcl_ListObjAppendElement(
                        interp,
                        pCmd,
                        Tcl_NewStringObj(zVar, -1 as ::core::ffi::c_int),
                    );
                    if needResultReset != 0 {
                        Tcl_ResetResult(interp);
                    }
                    needResultReset = 1 as ::core::ffi::c_int;
                    rx = Tcl_EvalObjEx(interp, pCmd, TCL_EVAL_DIRECT);
                    let mut _objPtr: *mut Tcl_Obj = pCmd;
                    let c2rust_fresh19 = (*_objPtr).refCount;
                    (*_objPtr).refCount = (*_objPtr).refCount - 1;
                    if c2rust_fresh19 <= 1 as ::core::ffi::c_int {
                        TclFreeObj(_objPtr);
                    }
                    if rx == TCL_OK {
                        pVar = Tcl_GetObjResult(interp);
                    } else if rx == TCL_ERROR {
                        rc = TCL_ERROR;
                        break;
                    } else {
                        pVar = ::core::ptr::null_mut::<Tcl_Obj>();
                    }
                }
                if !pVar.is_null() {
                    let mut n_0: Tcl_Size = 0;
                    let mut data: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
                    let mut zType: *const ::core::ffi::c_char = if !(*pVar)
                        .typePtr
                        .is_null()
                    {
                        (*(*pVar).typePtr).name
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    };
                    c = *zType.offset(0 as ::core::ffi::c_int as isize);
                    if *zVar.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == '@' as i32
                        || c as ::core::ffi::c_int == 'b' as i32
                            && strcmp(
                                zType,
                                b"bytearray\0".as_ptr() as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int && (*pVar).bytes.is_null()
                    {
                        data = Tcl_GetByteArrayFromObj(pVar, &raw mut n_0) as *mut u8_0;
                        sqlite3_bind_blob(
                            pStmt,
                            i,
                            data as *const ::core::ffi::c_void,
                            n_0 as ::core::ffi::c_int,
                            SQLITE_STATIC,
                        );
                        (*pVar).refCount += 1;
                        let c2rust_fresh20 = iParm;
                        iParm = iParm + 1;
                        let ref mut c2rust_fresh21 = *(*pPreStmt)
                            .apParm
                            .offset(c2rust_fresh20 as isize);
                        *c2rust_fresh21 = pVar;
                    } else if c as ::core::ffi::c_int == 'b' as i32
                        && (*pVar).bytes.is_null()
                        && (strcmp(
                            zType,
                            b"booleanString\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                            || strcmp(
                                zType,
                                b"boolean\0".as_ptr() as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int)
                    {
                        let mut nn: ::core::ffi::c_int = 0;
                        Tcl_GetBooleanFromObj(interp, pVar, &raw mut nn);
                        sqlite3_bind_int(pStmt, i, nn);
                    } else if c as ::core::ffi::c_int == 'd' as i32
                        && strcmp(
                            zType,
                            b"double\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                    {
                        let mut r: ::core::ffi::c_double = 0.;
                        Tcl_GetDoubleFromObj(interp, pVar, &raw mut r);
                        sqlite3_bind_double(pStmt, i, r);
                    } else if c as ::core::ffi::c_int == 'w' as i32
                        && strcmp(
                            zType,
                            b"wideInt\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        || c as ::core::ffi::c_int == 'i' as i32
                            && strcmp(
                                zType,
                                b"int\0".as_ptr() as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int
                    {
                        let mut v: Tcl_WideInt = 0;
                        Tcl_GetWideIntFromObj(interp, pVar, &raw mut v);
                        sqlite3_bind_int64(pStmt, i, v as sqlite3_int64);
                    } else {
                        data = Tcl_GetStringFromObj(pVar, &raw mut n_0)
                            as *mut ::core::ffi::c_uchar as *mut u8_0;
                        sqlite3_bind_text64(
                            pStmt,
                            i,
                            data as *mut ::core::ffi::c_char,
                            n_0 as sqlite3_uint64,
                            SQLITE_STATIC,
                            SQLITE_UTF8 as ::core::ffi::c_uchar,
                        );
                        (*pVar).refCount += 1;
                        let c2rust_fresh22 = iParm;
                        iParm = iParm + 1;
                        let ref mut c2rust_fresh23 = *(*pPreStmt)
                            .apParm
                            .offset(c2rust_fresh22 as isize);
                        *c2rust_fresh23 = pVar;
                    }
                } else {
                    sqlite3_bind_null(pStmt, i);
                }
                if needResultReset != 0 {
                    Tcl_ResetResult((*pDb).interp);
                }
            }
            i += 1;
        }
        (*pPreStmt).nParm = iParm;
        *ppPreStmt = pPreStmt;
        if needResultReset != 0 && rc == TCL_OK {
            Tcl_ResetResult((*pDb).interp);
        }
        return rc;
    }
}
unsafe extern "C" fn dbReleaseStmt(
    mut pDb: *mut SqliteDb,
    mut pPreStmt: *mut SqlPreparedStmt,
    mut discard: ::core::ffi::c_int,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*pPreStmt).nParm {
            let mut _objPtr: *mut Tcl_Obj = *(*pPreStmt).apParm.offset(i as isize);
            let c2rust_fresh24 = (*_objPtr).refCount;
            (*_objPtr).refCount = (*_objPtr).refCount - 1;
            if c2rust_fresh24 <= 1 as ::core::ffi::c_int {
                TclFreeObj(_objPtr);
            }
            i += 1;
        }
        (*pPreStmt).nParm = 0 as ::core::ffi::c_int;
        if (*pDb).maxStmt <= 0 as ::core::ffi::c_int || discard != 0 {
            dbFreeStmt(pPreStmt);
        } else {
            (*pPreStmt).pNext = (*pDb).stmtList;
            (*pPreStmt).pPrev = ::core::ptr::null_mut::<SqlPreparedStmt>();
            if !(*pDb).stmtList.is_null() {
                (*(*pDb).stmtList).pPrev = pPreStmt;
            }
            (*pDb).stmtList = pPreStmt;
            if (*pDb).stmtLast.is_null() {
                '_c2rust_label: {
                    if (*pDb).nStmt == 0 as ::core::ffi::c_int {} else {
                        __assert_fail(
                            b"pDb->nStmt==0\0".as_ptr() as *const ::core::ffi::c_char,
                            b"/sqlite/src/tclsqlite.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1589 as ::core::ffi::c_uint,
                            b"void dbReleaseStmt(SqliteDb *, SqlPreparedStmt *, int)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                };
                (*pDb).stmtLast = pPreStmt;
            } else {
                '_c2rust_label_0: {
                    if (*pDb).nStmt > 0 as ::core::ffi::c_int {} else {
                        __assert_fail(
                            b"pDb->nStmt>0\0".as_ptr() as *const ::core::ffi::c_char,
                            b"/sqlite/src/tclsqlite.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1592 as ::core::ffi::c_uint,
                            b"void dbReleaseStmt(SqliteDb *, SqlPreparedStmt *, int)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                };
            }
            (*pDb).nStmt += 1;
            while (*pDb).nStmt > (*pDb).maxStmt {
                let mut pLast: *mut SqlPreparedStmt = (*pDb).stmtLast;
                (*pDb).stmtLast = (*pLast).pPrev;
                (*(*pDb).stmtLast).pNext = ::core::ptr::null_mut::<SqlPreparedStmt>();
                (*pDb).nStmt -= 1;
                dbFreeStmt(pLast);
            }
        };
    }
}
pub const SQLITE_EVAL_WITHOUTNULLS: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_EVAL_ASDICT: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
unsafe extern "C" fn dbReleaseColumnNames(mut p: *mut DbEvalContext) {
    unsafe {
        if !(*p).apColName.is_null() {
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i < (*p).nCol {
                let mut _objPtr: *mut Tcl_Obj = *(*p).apColName.offset(i as isize);
                let c2rust_fresh25 = (*_objPtr).refCount;
                (*_objPtr).refCount = (*_objPtr).refCount - 1;
                if c2rust_fresh25 <= 1 as ::core::ffi::c_int {
                    TclFreeObj(_objPtr);
                }
                i += 1;
            }
            Tcl_Free((*p).apColName as *mut ::core::ffi::c_char);
            (*p).apColName = ::core::ptr::null_mut::<*mut Tcl_Obj>();
        }
        (*p).nCol = 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn dbEvalInit(
    mut p: *mut DbEvalContext,
    mut pDb: *mut SqliteDb,
    mut pSql: *mut Tcl_Obj,
    mut pVarName: *mut Tcl_Obj,
    mut evalFlags: ::core::ffi::c_int,
) {
    unsafe {
        memset(
            p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<DbEvalContext>() as size_t,
        );
        (*p).pDb = pDb;
        (*p).zSql = Tcl_GetString(pSql);
        (*p).pSql = pSql;
        (*pSql).refCount += 1;
        if !pVarName.is_null() {
            (*p).pVarName = pVarName;
            (*pVarName).refCount += 1;
        }
        (*p).evalFlags = evalFlags;
        addDatabaseRef((*p).pDb);
    }
}
unsafe extern "C" fn dbEvalRowInfo(
    mut p: *mut DbEvalContext,
    mut pnCol: *mut ::core::ffi::c_int,
    mut papColName: *mut *mut *mut Tcl_Obj,
) {
    unsafe {
        if (*p).apColName.is_null() {
            let mut pStmt: *mut sqlite3_stmt = (*(*p).pPreStmt).pStmt;
            let mut i: ::core::ffi::c_int = 0;
            let mut nCol: ::core::ffi::c_int = 0;
            let mut apColName: *mut *mut Tcl_Obj = ::core::ptr::null_mut::<
                *mut Tcl_Obj,
            >();
            nCol = sqlite3_column_count(pStmt);
            (*p).nCol = nCol;
            if nCol > 0 as ::core::ffi::c_int
                && (!papColName.is_null() || !(*p).pVarName.is_null())
            {
                apColName = Tcl_Alloc(
                    (::core::mem::size_of::<*mut Tcl_Obj>() as usize)
                        .wrapping_mul(nCol as usize) as ::core::ffi::c_uint,
                ) as *mut *mut Tcl_Obj;
                i = 0 as ::core::ffi::c_int;
                while i < nCol {
                    let ref mut c2rust_fresh26 = *apColName.offset(i as isize);
                    *c2rust_fresh26 = Tcl_NewStringObj(
                        sqlite3_column_name(pStmt, i),
                        -1 as ::core::ffi::c_int,
                    );
                    let ref mut c2rust_fresh27 = (**apColName.offset(i as isize))
                        .refCount;
                    *c2rust_fresh27 += 1;
                    i += 1;
                }
                (*p).apColName = apColName;
            }
            if !(*p).pVarName.is_null() {
                let mut interp: *mut Tcl_Interp = (*(*p).pDb).interp;
                let mut pColList: *mut Tcl_Obj = Tcl_NewObj();
                let mut pStar: *mut Tcl_Obj = Tcl_NewStringObj(
                    b"*\0".as_ptr() as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                );
                (*pColList).refCount += 1;
                (*pStar).refCount += 1;
                i = 0 as ::core::ffi::c_int;
                while i < nCol {
                    Tcl_ListObjAppendElement(
                        interp,
                        pColList,
                        *apColName.offset(i as isize),
                    );
                    i += 1;
                }
                if 0 as ::core::ffi::c_int == SQLITE_EVAL_ASDICT & (*p).evalFlags {
                    Tcl_ObjSetVar2(
                        interp,
                        (*p).pVarName,
                        pStar,
                        pColList,
                        0 as ::core::ffi::c_int,
                    );
                } else {
                    let mut pDict: *mut Tcl_Obj = Tcl_ObjGetVar2(
                        interp,
                        (*p).pVarName,
                        ::core::ptr::null_mut::<Tcl_Obj>(),
                        0 as ::core::ffi::c_int,
                    );
                    if pDict.is_null() {
                        pDict = Tcl_NewDictObj();
                    } else if (*pDict).refCount > 1 as ::core::ffi::c_int {
                        pDict = Tcl_DuplicateObj(pDict);
                    }
                    if Tcl_DictObjPut(interp, pDict, pStar, pColList) == TCL_OK {
                        Tcl_ObjSetVar2(
                            interp,
                            (*p).pVarName,
                            ::core::ptr::null_mut::<Tcl_Obj>(),
                            pDict,
                            0 as ::core::ffi::c_int,
                        );
                    }
                    (*pDict).refCount += 1;
                    let mut _objPtr: *mut Tcl_Obj = pDict;
                    let c2rust_fresh28 = (*_objPtr).refCount;
                    (*_objPtr).refCount = (*_objPtr).refCount - 1;
                    if c2rust_fresh28 <= 1 as ::core::ffi::c_int {
                        TclFreeObj(_objPtr);
                    }
                }
                let mut _objPtr_0: *mut Tcl_Obj = pStar;
                let c2rust_fresh29 = (*_objPtr_0).refCount;
                (*_objPtr_0).refCount = (*_objPtr_0).refCount - 1;
                if c2rust_fresh29 <= 1 as ::core::ffi::c_int {
                    TclFreeObj(_objPtr_0);
                }
                let mut _objPtr_1: *mut Tcl_Obj = pColList;
                let c2rust_fresh30 = (*_objPtr_1).refCount;
                (*_objPtr_1).refCount = (*_objPtr_1).refCount - 1;
                if c2rust_fresh30 <= 1 as ::core::ffi::c_int {
                    TclFreeObj(_objPtr_1);
                }
            }
        }
        if !papColName.is_null() {
            *papColName = (*p).apColName;
        }
        if !pnCol.is_null() {
            *pnCol = (*p).nCol;
        }
    }
}
unsafe extern "C" fn dbEvalStep(mut p: *mut DbEvalContext) -> ::core::ffi::c_int {
    unsafe {
        let mut zPrevSql: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        while *(*p).zSql.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0 || !(*p).pPreStmt.is_null()
        {
            let mut rc: ::core::ffi::c_int = 0;
            if (*p).pPreStmt.is_null() {
                zPrevSql = if (*p).zSql == zPrevSql {
                    ::core::ptr::null::<::core::ffi::c_char>()
                } else {
                    (*p).zSql
                };
                rc = dbPrepareAndBind(
                    (*p).pDb,
                    (*p).zSql,
                    &raw mut (*p).zSql,
                    &raw mut (*p).pPreStmt,
                );
                if rc != TCL_OK {
                    return rc;
                }
            } else {
                let mut rcs: ::core::ffi::c_int = 0;
                let mut pDb: *mut SqliteDb = (*p).pDb;
                let mut pPreStmt: *mut SqlPreparedStmt = (*p).pPreStmt;
                let mut pStmt: *mut sqlite3_stmt = (*pPreStmt).pStmt;
                rcs = sqlite3_step(pStmt);
                if rcs == SQLITE_ROW {
                    return TCL_OK;
                }
                if !(*p).pVarName.is_null() {
                    dbEvalRowInfo(
                        p,
                        ::core::ptr::null_mut::<::core::ffi::c_int>(),
                        ::core::ptr::null_mut::<*mut *mut Tcl_Obj>(),
                    );
                }
                rcs = sqlite3_reset(pStmt);
                (*pDb).nStep = sqlite3_stmt_status(
                    pStmt,
                    SQLITE_STMTSTATUS_FULLSCAN_STEP,
                    1 as ::core::ffi::c_int,
                );
                (*pDb).nSort = sqlite3_stmt_status(
                    pStmt,
                    SQLITE_STMTSTATUS_SORT,
                    1 as ::core::ffi::c_int,
                );
                (*pDb).nIndex = sqlite3_stmt_status(
                    pStmt,
                    SQLITE_STMTSTATUS_AUTOINDEX,
                    1 as ::core::ffi::c_int,
                );
                (*pDb).nVMStep = sqlite3_stmt_status(
                    pStmt,
                    SQLITE_STMTSTATUS_VM_STEP,
                    1 as ::core::ffi::c_int,
                );
                dbReleaseColumnNames(p);
                (*p).pPreStmt = ::core::ptr::null_mut::<SqlPreparedStmt>();
                if rcs != SQLITE_OK {
                    dbReleaseStmt(pDb, pPreStmt, 1 as ::core::ffi::c_int);
                    Tcl_SetObjResult(
                        (*pDb).interp,
                        Tcl_NewStringObj(
                            sqlite3_errmsg((*pDb).db),
                            -1 as ::core::ffi::c_int,
                        ),
                    );
                    return TCL_ERROR;
                } else {
                    dbReleaseStmt(pDb, pPreStmt, 0 as ::core::ffi::c_int);
                }
            }
        }
        return TCL_BREAK;
    }
}
unsafe extern "C" fn dbEvalFinalize(mut p: *mut DbEvalContext) {
    unsafe {
        if !(*p).pPreStmt.is_null() {
            sqlite3_reset((*(*p).pPreStmt).pStmt);
            dbReleaseStmt((*p).pDb, (*p).pPreStmt, 0 as ::core::ffi::c_int);
            (*p).pPreStmt = ::core::ptr::null_mut::<SqlPreparedStmt>();
        }
        if !(*p).pVarName.is_null() {
            let mut _objPtr: *mut Tcl_Obj = (*p).pVarName;
            let c2rust_fresh31 = (*_objPtr).refCount;
            (*_objPtr).refCount = (*_objPtr).refCount - 1;
            if c2rust_fresh31 <= 1 as ::core::ffi::c_int {
                TclFreeObj(_objPtr);
            }
            (*p).pVarName = ::core::ptr::null_mut::<Tcl_Obj>();
        }
        let mut _objPtr_0: *mut Tcl_Obj = (*p).pSql;
        let c2rust_fresh32 = (*_objPtr_0).refCount;
        (*_objPtr_0).refCount = (*_objPtr_0).refCount - 1;
        if c2rust_fresh32 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr_0);
        }
        dbReleaseColumnNames(p);
        delDatabaseRef((*p).pDb);
    }
}
unsafe extern "C" fn dbEvalColumnValue(
    mut p: *mut DbEvalContext,
    mut iCol: ::core::ffi::c_int,
) -> *mut Tcl_Obj {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = (*(*p).pPreStmt).pStmt;
        match sqlite3_column_type(pStmt, iCol) {
            SQLITE_BLOB => {
                let mut bytes: ::core::ffi::c_int = sqlite3_column_bytes(pStmt, iCol);
                let mut zBlob: *const ::core::ffi::c_char = sqlite3_column_blob(
                    pStmt,
                    iCol,
                ) as *const ::core::ffi::c_char;
                if zBlob.is_null() {
                    bytes = 0 as ::core::ffi::c_int;
                }
                return Tcl_NewByteArrayObj(zBlob as *mut u8_0, bytes);
            }
            SQLITE_INTEGER => {
                let mut v: sqlite_int64 = sqlite3_column_int64(pStmt, iCol)
                    as sqlite_int64;
                if v >= -2147483647 as ::core::ffi::c_int as ::core::ffi::c_longlong
                    && v <= 2147483647 as ::core::ffi::c_longlong
                {
                    return Tcl_NewIntObj(v as ::core::ffi::c_int)
                } else {
                    return Tcl_NewWideIntObj(v as Tcl_WideInt)
                }
            }
            SQLITE_FLOAT => return Tcl_NewDoubleObj(sqlite3_column_double(pStmt, iCol)),
            SQLITE_NULL => {
                return Tcl_NewStringObj((*(*p).pDb).zNull, -1 as ::core::ffi::c_int);
            }
            _ => {}
        }
        return Tcl_NewStringObj(
            sqlite3_column_text(pStmt, iCol) as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn DbUseNre() -> ::core::ffi::c_int {
    unsafe {
        let mut major: ::core::ffi::c_int = 0;
        let mut minor: ::core::ffi::c_int = 0;
        Tcl_GetVersion(
            &raw mut major,
            &raw mut minor,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        return (major == 8 as ::core::ffi::c_int && minor >= 6 as ::core::ffi::c_int
            || major > 8 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn DbEvalNextCmd(
    mut data: *mut ClientData,
    mut interp: *mut Tcl_Interp,
    mut result: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = result;
        let mut p: *mut DbEvalContext = *data.offset(0 as ::core::ffi::c_int as isize)
            as *mut DbEvalContext;
        let pScript: *mut Tcl_Obj = *data.offset(1 as ::core::ffi::c_int as isize)
            as *mut Tcl_Obj;
        let pVarName: *mut Tcl_Obj = (*p).pVarName;
        while (rc == TCL_OK || rc == TCL_CONTINUE)
            && {
                rc = dbEvalStep(p);
                TCL_OK == rc
            }
        {
            let mut i: ::core::ffi::c_int = 0;
            let mut nCol: ::core::ffi::c_int = 0;
            let mut apColName: *mut *mut Tcl_Obj = ::core::ptr::null_mut::<
                *mut Tcl_Obj,
            >();
            dbEvalRowInfo(p, &raw mut nCol, &raw mut apColName);
            i = 0 as ::core::ffi::c_int;
            while i < nCol {
                if pVarName.is_null() {
                    Tcl_ObjSetVar2(
                        interp,
                        *apColName.offset(i as isize),
                        ::core::ptr::null_mut::<Tcl_Obj>(),
                        dbEvalColumnValue(p, i),
                        0 as ::core::ffi::c_int,
                    );
                } else if (*p).evalFlags & SQLITE_EVAL_WITHOUTNULLS
                    != 0 as ::core::ffi::c_int
                    && sqlite3_column_type((*(*p).pPreStmt).pStmt, i) == SQLITE_NULL
                {
                    if 0 as ::core::ffi::c_int == SQLITE_EVAL_ASDICT & (*p).evalFlags {
                        Tcl_UnsetVar2(
                            interp,
                            Tcl_GetString(pVarName),
                            Tcl_GetString(*apColName.offset(i as isize)),
                            0 as ::core::ffi::c_int,
                        );
                    } else {
                        let mut pDict: *mut Tcl_Obj = Tcl_ObjGetVar2(
                            interp,
                            pVarName,
                            ::core::ptr::null_mut::<Tcl_Obj>(),
                            0 as ::core::ffi::c_int,
                        );
                        if !pDict.is_null() {
                            if (*pDict).refCount > 1 as ::core::ffi::c_int {
                                pDict = Tcl_DuplicateObj(pDict);
                            }
                            if Tcl_DictObjRemove(
                                interp,
                                pDict,
                                *apColName.offset(i as isize),
                            ) == TCL_OK
                            {
                                Tcl_ObjSetVar2(
                                    interp,
                                    pVarName,
                                    ::core::ptr::null_mut::<Tcl_Obj>(),
                                    pDict,
                                    0 as ::core::ffi::c_int,
                                );
                            }
                            (*pDict).refCount += 1;
                            let mut _objPtr: *mut Tcl_Obj = pDict;
                            let c2rust_fresh33 = (*_objPtr).refCount;
                            (*_objPtr).refCount = (*_objPtr).refCount - 1;
                            if c2rust_fresh33 <= 1 as ::core::ffi::c_int {
                                TclFreeObj(_objPtr);
                            }
                        }
                    }
                } else if 0 as ::core::ffi::c_int == SQLITE_EVAL_ASDICT & (*p).evalFlags
                {
                    Tcl_ObjSetVar2(
                        interp,
                        pVarName,
                        *apColName.offset(i as isize),
                        dbEvalColumnValue(p, i),
                        0 as ::core::ffi::c_int,
                    );
                } else {
                    let mut pDict_0: *mut Tcl_Obj = Tcl_ObjGetVar2(
                        interp,
                        pVarName,
                        ::core::ptr::null_mut::<Tcl_Obj>(),
                        0 as ::core::ffi::c_int,
                    );
                    if pDict_0.is_null() {
                        pDict_0 = Tcl_NewDictObj();
                    } else if (*pDict_0).refCount > 1 as ::core::ffi::c_int {
                        pDict_0 = Tcl_DuplicateObj(pDict_0);
                    }
                    if Tcl_DictObjPut(
                        interp,
                        pDict_0,
                        *apColName.offset(i as isize),
                        dbEvalColumnValue(p, i),
                    ) == TCL_OK
                    {
                        Tcl_ObjSetVar2(
                            interp,
                            pVarName,
                            ::core::ptr::null_mut::<Tcl_Obj>(),
                            pDict_0,
                            0 as ::core::ffi::c_int,
                        );
                    }
                    (*pDict_0).refCount += 1;
                    let mut _objPtr_0: *mut Tcl_Obj = pDict_0;
                    let c2rust_fresh34 = (*_objPtr_0).refCount;
                    (*_objPtr_0).refCount = (*_objPtr_0).refCount - 1;
                    if c2rust_fresh34 <= 1 as ::core::ffi::c_int {
                        TclFreeObj(_objPtr_0);
                    }
                }
                i += 1;
            }
            if DbUseNre() != 0 {
                Tcl_NRAddCallback(
                    interp,
                    Some(
                        DbEvalNextCmd
                            as unsafe extern "C" fn(
                                *mut ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                            ) -> ::core::ffi::c_int,
                    ),
                    p as ClientData,
                    pScript as ClientData,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                );
                return Tcl_NREvalObj(interp, pScript, 0 as ::core::ffi::c_int);
            } else {
                rc = Tcl_EvalObjEx(interp, pScript, 0 as ::core::ffi::c_int);
            }
        }
        let mut _objPtr_1: *mut Tcl_Obj = pScript;
        let c2rust_fresh35 = (*_objPtr_1).refCount;
        (*_objPtr_1).refCount = (*_objPtr_1).refCount - 1;
        if c2rust_fresh35 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr_1);
        }
        dbEvalFinalize(p);
        Tcl_Free(p as *mut ::core::ffi::c_char);
        if rc == TCL_OK || rc == TCL_BREAK {
            Tcl_ResetResult(interp);
            rc = TCL_OK;
        }
        return rc;
    }
}
unsafe extern "C" fn DbHookCmd(
    mut interp: *mut Tcl_Interp,
    mut pDb: *mut SqliteDb,
    mut pArg: *mut Tcl_Obj,
    mut ppHook: *mut *mut Tcl_Obj,
) {
    unsafe {
        let mut db: *mut sqlite3 = (*pDb).db;
        if !(*ppHook).is_null() {
            Tcl_SetObjResult(interp, *ppHook);
            if !pArg.is_null() {
                let mut _objPtr: *mut Tcl_Obj = *ppHook;
                let c2rust_fresh36 = (*_objPtr).refCount;
                (*_objPtr).refCount = (*_objPtr).refCount - 1;
                if c2rust_fresh36 <= 1 as ::core::ffi::c_int {
                    TclFreeObj(_objPtr);
                }
                *ppHook = ::core::ptr::null_mut::<Tcl_Obj>();
            }
        }
        if !pArg.is_null() {
            '_c2rust_label: {
                if (*ppHook).is_null() {} else {
                    __assert_fail(
                        b"!(*ppHook)\0".as_ptr() as *const ::core::ffi::c_char,
                        b"/sqlite/src/tclsqlite.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        2023 as ::core::ffi::c_uint,
                        b"void DbHookCmd(Tcl_Interp *, SqliteDb *, Tcl_Obj *, Tcl_Obj **)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            };
            if *Tcl_GetString(pArg).offset(0 as ::core::ffi::c_int as isize) != 0 {
                *ppHook = pArg;
                (**ppHook).refCount += 1;
            }
        }
        sqlite3_update_hook(
            db,
            if !(*pDb).pUpdateHook.is_null() {
                Some(
                    DbUpdateHandler
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            sqlite_int64,
                        ) -> (),
                )
            } else {
                None
            },
            pDb as *mut ::core::ffi::c_void,
        );
        sqlite3_rollback_hook(
            db,
            if !(*pDb).pRollbackHook.is_null() {
                Some(
                    DbRollbackHandler
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                )
            } else {
                None
            },
            pDb as *mut ::core::ffi::c_void,
        );
        sqlite3_wal_hook(
            db,
            if !(*pDb).pWalHook.is_null() {
                Some(
                    DbWalHandler
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut sqlite3,
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                )
            } else {
                None
            },
            pDb as *mut ::core::ffi::c_void,
        );
    }
}
unsafe extern "C" fn DbObjCmd(
    mut cd: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut pDb: *mut SqliteDb = cd as *mut SqliteDb;
        let mut choice: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = TCL_OK;
        static mut DB_strs: [*const ::core::ffi::c_char; 43] = [
            b"authorizer\0".as_ptr() as *const ::core::ffi::c_char,
            b"backup\0".as_ptr() as *const ::core::ffi::c_char,
            b"bind_fallback\0".as_ptr() as *const ::core::ffi::c_char,
            b"busy\0".as_ptr() as *const ::core::ffi::c_char,
            b"cache\0".as_ptr() as *const ::core::ffi::c_char,
            b"changes\0".as_ptr() as *const ::core::ffi::c_char,
            b"close\0".as_ptr() as *const ::core::ffi::c_char,
            b"collate\0".as_ptr() as *const ::core::ffi::c_char,
            b"collation_needed\0".as_ptr() as *const ::core::ffi::c_char,
            b"commit_hook\0".as_ptr() as *const ::core::ffi::c_char,
            b"complete\0".as_ptr() as *const ::core::ffi::c_char,
            b"config\0".as_ptr() as *const ::core::ffi::c_char,
            b"copy\0".as_ptr() as *const ::core::ffi::c_char,
            b"deserialize\0".as_ptr() as *const ::core::ffi::c_char,
            b"enable_load_extension\0".as_ptr() as *const ::core::ffi::c_char,
            b"errorcode\0".as_ptr() as *const ::core::ffi::c_char,
            b"erroroffset\0".as_ptr() as *const ::core::ffi::c_char,
            b"eval\0".as_ptr() as *const ::core::ffi::c_char,
            b"exists\0".as_ptr() as *const ::core::ffi::c_char,
            b"function\0".as_ptr() as *const ::core::ffi::c_char,
            b"incrblob\0".as_ptr() as *const ::core::ffi::c_char,
            b"interrupt\0".as_ptr() as *const ::core::ffi::c_char,
            b"last_insert_rowid\0".as_ptr() as *const ::core::ffi::c_char,
            b"nullvalue\0".as_ptr() as *const ::core::ffi::c_char,
            b"onecolumn\0".as_ptr() as *const ::core::ffi::c_char,
            b"preupdate\0".as_ptr() as *const ::core::ffi::c_char,
            b"profile\0".as_ptr() as *const ::core::ffi::c_char,
            b"progress\0".as_ptr() as *const ::core::ffi::c_char,
            b"rekey\0".as_ptr() as *const ::core::ffi::c_char,
            b"restore\0".as_ptr() as *const ::core::ffi::c_char,
            b"rollback_hook\0".as_ptr() as *const ::core::ffi::c_char,
            b"serialize\0".as_ptr() as *const ::core::ffi::c_char,
            b"status\0".as_ptr() as *const ::core::ffi::c_char,
            b"timeout\0".as_ptr() as *const ::core::ffi::c_char,
            b"total_changes\0".as_ptr() as *const ::core::ffi::c_char,
            b"trace\0".as_ptr() as *const ::core::ffi::c_char,
            b"trace_v2\0".as_ptr() as *const ::core::ffi::c_char,
            b"transaction\0".as_ptr() as *const ::core::ffi::c_char,
            b"unlock_notify\0".as_ptr() as *const ::core::ffi::c_char,
            b"update_hook\0".as_ptr() as *const ::core::ffi::c_char,
            b"version\0".as_ptr() as *const ::core::ffi::c_char,
            b"wal_hook\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        ];
        if objc < 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv as *const *mut Tcl_Obj,
                b"SUBCOMMAND ...\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if Tcl_GetIndexFromObjStruct(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut DB_strs as *mut *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<*mut ::core::ffi::c_char>() as ::core::ffi::c_int,
            b"option\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            &raw mut choice,
        ) != 0
        {
            return TCL_ERROR;
        }
        match choice as DB_enum as ::core::ffi::c_uint {
            0 => {
                if objc > 3 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv as *const *mut Tcl_Obj,
                        b"?CALLBACK?\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                } else if objc == 2 as ::core::ffi::c_int {
                    if !(*pDb).zAuth.is_null() {
                        Tcl_AppendResult(
                            interp,
                            (*pDb).zAuth,
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        );
                    }
                } else {
                    let mut zAuth: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                        ::core::ffi::c_char,
                    >();
                    let mut len: Tcl_Size = 0;
                    if !(*pDb).zAuth.is_null() {
                        Tcl_Free((*pDb).zAuth);
                    }
                    zAuth = Tcl_GetStringFromObj(
                        *objv.offset(2 as ::core::ffi::c_int as isize),
                        &raw mut len,
                    );
                    if !zAuth.is_null() && len > 0 as ::core::ffi::c_int {
                        (*pDb).zAuth = Tcl_Alloc(
                            (len as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_uint,
                        );
                        memcpy(
                            (*pDb).zAuth as *mut ::core::ffi::c_void,
                            zAuth as *const ::core::ffi::c_void,
                            (len as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                as size_t,
                        );
                    } else {
                        (*pDb).zAuth = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    }
                    if !(*pDb).zAuth.is_null() {
                        (*pDb).interp = interp;
                        sqlite3_set_authorizer(
                            (*pDb).db,
                            Some(
                                auth_callback
                                    as unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                        ::core::ffi::c_int,
                                        *const ::core::ffi::c_char,
                                        *const ::core::ffi::c_char,
                                        *const ::core::ffi::c_char,
                                        *const ::core::ffi::c_char,
                                    ) -> ::core::ffi::c_int,
                            ),
                            pDb as *mut ::core::ffi::c_void,
                        );
                    } else {
                        sqlite3_set_authorizer(
                            (*pDb).db,
                            None,
                            ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        );
                    }
                }
            }
            1 => {
                let mut zDestFile: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                let mut zSrcDb: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                let mut pDest: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
                let mut pBackup: *mut sqlite3_backup = ::core::ptr::null_mut::<
                    sqlite3_backup,
                >();
                if objc == 3 as ::core::ffi::c_int {
                    zSrcDb = b"main\0".as_ptr() as *const ::core::ffi::c_char;
                    zDestFile = Tcl_GetString(
                        *objv.offset(2 as ::core::ffi::c_int as isize),
                    );
                } else if objc == 4 as ::core::ffi::c_int {
                    zSrcDb = Tcl_GetString(
                        *objv.offset(2 as ::core::ffi::c_int as isize),
                    );
                    zDestFile = Tcl_GetString(
                        *objv.offset(3 as ::core::ffi::c_int as isize),
                    );
                } else {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv as *const *mut Tcl_Obj,
                        b"?DATABASE? FILENAME\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                rc = sqlite3_open_v2(
                    zDestFile,
                    &raw mut pDest,
                    SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE | (*pDb).openFlags,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
                if rc != SQLITE_OK {
                    Tcl_AppendResult(
                        interp,
                        b"cannot open target database: \0".as_ptr()
                            as *const ::core::ffi::c_char,
                        sqlite3_errmsg(pDest),
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    );
                    sqlite3_close(pDest);
                    return TCL_ERROR;
                }
                pBackup = sqlite3_backup_init(
                    pDest,
                    b"main\0".as_ptr() as *const ::core::ffi::c_char,
                    (*pDb).db,
                    zSrcDb,
                );
                if pBackup.is_null() {
                    Tcl_AppendResult(
                        interp,
                        b"backup failed: \0".as_ptr() as *const ::core::ffi::c_char,
                        sqlite3_errmsg(pDest),
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    );
                    sqlite3_close(pDest);
                    return TCL_ERROR;
                }
                loop {
                    rc = sqlite3_backup_step(pBackup, 100 as ::core::ffi::c_int);
                    if !(rc == SQLITE_OK) {
                        break;
                    }
                }
                sqlite3_backup_finish(pBackup);
                if rc == SQLITE_DONE {
                    rc = TCL_OK;
                } else {
                    Tcl_AppendResult(
                        interp,
                        b"backup failed: \0".as_ptr() as *const ::core::ffi::c_char,
                        sqlite3_errmsg(pDest),
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    );
                    rc = TCL_ERROR;
                }
                sqlite3_close(pDest);
            }
            2 => {
                if objc > 3 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv as *const *mut Tcl_Obj,
                        b"?CALLBACK?\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                } else if objc == 2 as ::core::ffi::c_int {
                    if !(*pDb).zBindFallback.is_null() {
                        Tcl_AppendResult(
                            interp,
                            (*pDb).zBindFallback,
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        );
                    }
                } else {
                    let mut zCallback: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                        ::core::ffi::c_char,
                    >();
                    let mut len_0: Tcl_Size = 0;
                    if !(*pDb).zBindFallback.is_null() {
                        Tcl_Free((*pDb).zBindFallback);
                    }
                    zCallback = Tcl_GetStringFromObj(
                        *objv.offset(2 as ::core::ffi::c_int as isize),
                        &raw mut len_0,
                    );
                    if !zCallback.is_null() && len_0 > 0 as ::core::ffi::c_int {
                        (*pDb).zBindFallback = Tcl_Alloc(
                            (len_0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_uint,
                        );
                        memcpy(
                            (*pDb).zBindFallback as *mut ::core::ffi::c_void,
                            zCallback as *const ::core::ffi::c_void,
                            (len_0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                as size_t,
                        );
                    } else {
                        (*pDb).zBindFallback = ::core::ptr::null_mut::<
                            ::core::ffi::c_char,
                        >();
                    }
                }
            }
            3 => {
                if objc > 3 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv as *const *mut Tcl_Obj,
                        b"CALLBACK\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                } else if objc == 2 as ::core::ffi::c_int {
                    if !(*pDb).zBusy.is_null() {
                        Tcl_AppendResult(
                            interp,
                            (*pDb).zBusy,
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        );
                    }
                } else {
                    let mut zBusy: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                        ::core::ffi::c_char,
                    >();
                    let mut len_1: Tcl_Size = 0;
                    if !(*pDb).zBusy.is_null() {
                        Tcl_Free((*pDb).zBusy);
                    }
                    zBusy = Tcl_GetStringFromObj(
                        *objv.offset(2 as ::core::ffi::c_int as isize),
                        &raw mut len_1,
                    );
                    if !zBusy.is_null() && len_1 > 0 as ::core::ffi::c_int {
                        (*pDb).zBusy = Tcl_Alloc(
                            (len_1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_uint,
                        );
                        memcpy(
                            (*pDb).zBusy as *mut ::core::ffi::c_void,
                            zBusy as *const ::core::ffi::c_void,
                            (len_1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                as size_t,
                        );
                    } else {
                        (*pDb).zBusy = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    }
                    if !(*pDb).zBusy.is_null() {
                        (*pDb).interp = interp;
                        sqlite3_busy_handler(
                            (*pDb).db,
                            Some(
                                DbBusyHandler
                                    as unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                        ::core::ffi::c_int,
                                    ) -> ::core::ffi::c_int,
                            ),
                            pDb as *mut ::core::ffi::c_void,
                        );
                    } else {
                        sqlite3_busy_handler(
                            (*pDb).db,
                            None,
                            ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        );
                    }
                }
            }
            4 => {
                let mut subCmd: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                let mut n: ::core::ffi::c_int = 0;
                if objc <= 2 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        1 as ::core::ffi::c_int,
                        objv as *const *mut Tcl_Obj,
                        b"cache option ?arg?\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                subCmd = Tcl_GetStringFromObj(
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
                if *subCmd as ::core::ffi::c_int == 'f' as i32
                    && strcmp(subCmd, b"flush\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                {
                    if objc != 3 as ::core::ffi::c_int {
                        Tcl_WrongNumArgs(
                            interp,
                            2 as ::core::ffi::c_int,
                            objv as *const *mut Tcl_Obj,
                            b"flush\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        return TCL_ERROR;
                    } else {
                        flushStmtCache(pDb);
                    }
                } else if *subCmd as ::core::ffi::c_int == 's' as i32
                    && strcmp(subCmd, b"size\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                {
                    if objc != 4 as ::core::ffi::c_int {
                        Tcl_WrongNumArgs(
                            interp,
                            2 as ::core::ffi::c_int,
                            objv as *const *mut Tcl_Obj,
                            b"size n\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        return TCL_ERROR;
                    } else if TCL_ERROR
                        == Tcl_GetIntFromObj(
                            interp,
                            *objv.offset(3 as ::core::ffi::c_int as isize),
                            &raw mut n,
                        )
                    {
                        Tcl_AppendResult(
                            interp,
                            b"cannot convert \"\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            Tcl_GetStringFromObj(
                                *objv.offset(3 as ::core::ffi::c_int as isize),
                                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                            ),
                            b"\" to integer\0".as_ptr() as *const ::core::ffi::c_char,
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        );
                        return TCL_ERROR;
                    } else {
                        if n < 0 as ::core::ffi::c_int {
                            flushStmtCache(pDb);
                            n = 0 as ::core::ffi::c_int;
                        } else if n > MAX_PREPARED_STMTS {
                            n = MAX_PREPARED_STMTS;
                        }
                        (*pDb).maxStmt = n;
                    }
                } else {
                    Tcl_AppendResult(
                        interp,
                        b"bad option \"\0".as_ptr() as *const ::core::ffi::c_char,
                        Tcl_GetStringFromObj(
                            *objv.offset(2 as ::core::ffi::c_int as isize),
                            ::core::ptr::null_mut::<::core::ffi::c_int>(),
                        ),
                        b"\": must be flush or size\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    );
                    return TCL_ERROR;
                }
            }
            5 => {
                let mut pResult: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
                if objc != 2 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv as *const *mut Tcl_Obj,
                        b"\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                pResult = Tcl_GetObjResult(interp);
                Tcl_SetWideIntObj(pResult, sqlite3_changes64((*pDb).db) as Tcl_WideInt);
            }
            6 => {
                Tcl_DeleteCommand(
                    interp,
                    Tcl_GetStringFromObj(
                        *objv.offset(0 as ::core::ffi::c_int as isize),
                        ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    ),
                );
            }
            7 => {
                let mut pCollate: *mut SqlCollate = ::core::ptr::null_mut::<
                    SqlCollate,
                >();
                let mut zName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                let mut zScript: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                let mut nScript: Tcl_Size = 0;
                if objc != 4 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv as *const *mut Tcl_Obj,
                        b"NAME SCRIPT\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                zName = Tcl_GetStringFromObj(
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
                zScript = Tcl_GetStringFromObj(
                    *objv.offset(3 as ::core::ffi::c_int as isize),
                    &raw mut nScript,
                );
                pCollate = Tcl_Alloc(
                    (::core::mem::size_of::<SqlCollate>() as usize)
                        .wrapping_add(nScript as usize)
                        .wrapping_add(1 as usize) as ::core::ffi::c_uint,
                ) as *mut SqlCollate;
                if pCollate.is_null() {
                    return TCL_ERROR;
                }
                (*pCollate).interp = interp;
                (*pCollate).pNext = (*pDb).pCollate;
                (*pCollate).zScript = pCollate.offset(1 as ::core::ffi::c_int as isize)
                    as *mut SqlCollate as *mut ::core::ffi::c_char;
                (*pDb).pCollate = pCollate;
                memcpy(
                    (*pCollate).zScript as *mut ::core::ffi::c_void,
                    zScript as *const ::core::ffi::c_void,
                    (nScript as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t,
                );
                if sqlite3_create_collation(
                    (*pDb).db,
                    zName,
                    SQLITE_UTF8,
                    pCollate as *mut ::core::ffi::c_void,
                    Some(
                        tclSqlCollate
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                ::core::ffi::c_int,
                                *const ::core::ffi::c_void,
                                ::core::ffi::c_int,
                                *const ::core::ffi::c_void,
                            ) -> ::core::ffi::c_int,
                    ),
                ) != 0
                {
                    Tcl_SetResult(
                        interp,
                        sqlite3_errmsg((*pDb).db) as *mut ::core::ffi::c_char,
                        ::core::mem::transmute::<
                            ::libc::intptr_t,
                            Option<Tcl_FreeProc>,
                        >(1 as ::core::ffi::c_int as ::libc::intptr_t),
                    );
                    return TCL_ERROR;
                }
            }
            8 => {
                if objc != 3 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv as *const *mut Tcl_Obj,
                        b"SCRIPT\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                if !(*pDb).pCollateNeeded.is_null() {
                    let mut _objPtr: *mut Tcl_Obj = (*pDb).pCollateNeeded;
                    let c2rust_fresh37 = (*_objPtr).refCount;
                    (*_objPtr).refCount = (*_objPtr).refCount - 1;
                    if c2rust_fresh37 <= 1 as ::core::ffi::c_int {
                        TclFreeObj(_objPtr);
                    }
                }
                (*pDb).pCollateNeeded = Tcl_DuplicateObj(
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                );
                (*(*pDb).pCollateNeeded).refCount += 1;
                sqlite3_collation_needed(
                    (*pDb).db,
                    pDb as *mut ::core::ffi::c_void,
                    Some(
                        tclCollateNeeded
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut sqlite3,
                                ::core::ffi::c_int,
                                *const ::core::ffi::c_char,
                            ) -> (),
                    ),
                );
            }
            9 => {
                if objc > 3 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv as *const *mut Tcl_Obj,
                        b"?CALLBACK?\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                } else if objc == 2 as ::core::ffi::c_int {
                    if !(*pDb).zCommit.is_null() {
                        Tcl_AppendResult(
                            interp,
                            (*pDb).zCommit,
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        );
                    }
                } else {
                    let mut zCommit: *const ::core::ffi::c_char = ::core::ptr::null::<
                        ::core::ffi::c_char,
                    >();
                    let mut len_2: Tcl_Size = 0;
                    if !(*pDb).zCommit.is_null() {
                        Tcl_Free((*pDb).zCommit);
                    }
                    zCommit = Tcl_GetStringFromObj(
                        *objv.offset(2 as ::core::ffi::c_int as isize),
                        &raw mut len_2,
                    );
                    if !zCommit.is_null() && len_2 > 0 as ::core::ffi::c_int {
                        (*pDb).zCommit = Tcl_Alloc(
                            (len_2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_uint,
                        );
                        memcpy(
                            (*pDb).zCommit as *mut ::core::ffi::c_void,
                            zCommit as *const ::core::ffi::c_void,
                            (len_2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                as size_t,
                        );
                    } else {
                        (*pDb).zCommit = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    }
                    if !(*pDb).zCommit.is_null() {
                        (*pDb).interp = interp;
                        sqlite3_commit_hook(
                            (*pDb).db,
                            Some(
                                DbCommitHandler
                                    as unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                    ) -> ::core::ffi::c_int,
                            ),
                            pDb as *mut ::core::ffi::c_void,
                        );
                    } else {
                        sqlite3_commit_hook(
                            (*pDb).db,
                            None,
                            ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        );
                    }
                }
            }
            10 => {
                let mut pResult_0: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
                let mut isComplete: ::core::ffi::c_int = 0;
                if objc != 3 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv as *const *mut Tcl_Obj,
                        b"SQL\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                isComplete = sqlite3_complete(
                    Tcl_GetStringFromObj(
                        *objv.offset(2 as ::core::ffi::c_int as isize),
                        ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    ),
                );
                pResult_0 = Tcl_GetObjResult(interp);
                Tcl_SetIntObj(
                    pResult_0,
                    (isComplete != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
                );
            }
            11 => {
                static mut aDbConfig: [DbConfigChoices; 16] = [
                    DbConfigChoices {
                        zName: b"defensive\0".as_ptr() as *const ::core::ffi::c_char,
                        op: SQLITE_DBCONFIG_DEFENSIVE

                    },
                    DbConfigChoices {
                        zName: b"dqs_ddl\0".as_ptr() as *const ::core::ffi::c_char,
                        op: SQLITE_DBCONFIG_DQS_DDL

                    },
                    DbConfigChoices {
                        zName: b"dqs_dml\0".as_ptr() as *const ::core::ffi::c_char,
                        op: SQLITE_DBCONFIG_DQS_DML

                    },
                    DbConfigChoices {
                        zName: b"enable_fkey\0".as_ptr() as *const ::core::ffi::c_char,
                        op: SQLITE_DBCONFIG_ENABLE_FKEY

                    },
                    DbConfigChoices {
                        zName: b"enable_qpsg\0".as_ptr() as *const ::core::ffi::c_char,
                        op: SQLITE_DBCONFIG_ENABLE_QPSG

                    },
                    DbConfigChoices {
                        zName: b"enable_trigger\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        op: SQLITE_DBCONFIG_ENABLE_TRIGGER

                    },
                    DbConfigChoices {
                        zName: b"enable_view\0".as_ptr() as *const ::core::ffi::c_char,
                        op: SQLITE_DBCONFIG_ENABLE_VIEW

                    },
                    DbConfigChoices {
                        zName: b"fts3_tokenizer\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        op: SQLITE_DBCONFIG_ENABLE_FTS3_TOKENIZER

                    },
                    DbConfigChoices {
                        zName: b"legacy_alter_table\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        op: SQLITE_DBCONFIG_LEGACY_ALTER_TABLE

                    },
                    DbConfigChoices {
                        zName: b"legacy_file_format\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        op: SQLITE_DBCONFIG_LEGACY_FILE_FORMAT

                    },
                    DbConfigChoices {
                        zName: b"load_extension\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        op: SQLITE_DBCONFIG_ENABLE_LOAD_EXTENSION

                    },
                    DbConfigChoices {
                        zName: b"no_ckpt_on_close\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        op: SQLITE_DBCONFIG_NO_CKPT_ON_CLOSE

                    },
                    DbConfigChoices {
                        zName: b"reset_database\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        op: SQLITE_DBCONFIG_RESET_DATABASE

                    },
                    DbConfigChoices {
                        zName: b"trigger_eqp\0".as_ptr() as *const ::core::ffi::c_char,
                        op: SQLITE_DBCONFIG_TRIGGER_EQP

                    },
                    DbConfigChoices {
                        zName: b"trusted_schema\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        op: SQLITE_DBCONFIG_TRUSTED_SCHEMA

                    },
                    DbConfigChoices {
                        zName: b"writable_schema\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        op: SQLITE_DBCONFIG_WRITABLE_SCHEMA

                    },
                ];
                let mut pResult_1: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
                let mut ii: ::core::ffi::c_int = 0;
                if objc > 4 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv as *const *mut Tcl_Obj,
                        b"?OPTION? ?BOOLEAN?\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                if objc == 2 as ::core::ffi::c_int {
                    pResult_1 = Tcl_NewListObj(
                        0 as ::core::ffi::c_int,
                        ::core::ptr::null::<*mut Tcl_Obj>(),
                    );
                    ii = 0 as ::core::ffi::c_int;
                    while (ii as usize)
                        < (::core::mem::size_of::<[DbConfigChoices; 16]>() as usize)
                            .wrapping_div(
                                ::core::mem::size_of::<DbConfigChoices>() as usize,
                            )
                    {
                        let mut v: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        sqlite3_db_config(
                            (*pDb).db,
                            aDbConfig[ii as usize].op,
                            -1 as ::core::ffi::c_int,
                            &raw mut v,
                        );
                        Tcl_ListObjAppendElement(
                            interp,
                            pResult_1,
                            Tcl_NewStringObj(
                                aDbConfig[ii as usize].zName,
                                -1 as ::core::ffi::c_int,
                            ),
                        );
                        Tcl_ListObjAppendElement(interp, pResult_1, Tcl_NewIntObj(v));
                        ii += 1;
                    }
                } else {
                    let mut zOpt: *const ::core::ffi::c_char = Tcl_GetString(
                        *objv.offset(2 as ::core::ffi::c_int as isize),
                    );
                    let mut onoff: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
                    let mut v_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    if *zOpt.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == '-' as i32
                    {
                        zOpt = zOpt.offset(1);
                    }
                    ii = 0 as ::core::ffi::c_int;
                    while (ii as usize)
                        < (::core::mem::size_of::<[DbConfigChoices; 16]>() as usize)
                            .wrapping_div(
                                ::core::mem::size_of::<DbConfigChoices>() as usize,
                            )
                    {
                        if strcmp(aDbConfig[ii as usize].zName, zOpt)
                            == 0 as ::core::ffi::c_int
                        {
                            break;
                        }
                        ii += 1;
                    }
                    if ii as usize
                        >= (::core::mem::size_of::<[DbConfigChoices; 16]>() as usize)
                            .wrapping_div(
                                ::core::mem::size_of::<DbConfigChoices>() as usize,
                            )
                    {
                        Tcl_AppendResult(
                            interp,
                            b"unknown config option: \"\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            zOpt,
                            b"\"\0".as_ptr() as *const ::core::ffi::c_char,
                            ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        );
                        return TCL_ERROR;
                    }
                    if objc == 4 as ::core::ffi::c_int {
                        if Tcl_GetBooleanFromObj(
                            interp,
                            *objv.offset(3 as ::core::ffi::c_int as isize),
                            &raw mut onoff,
                        ) != 0
                        {
                            return TCL_ERROR;
                        }
                    }
                    sqlite3_db_config(
                        (*pDb).db,
                        aDbConfig[ii as usize].op,
                        onoff,
                        &raw mut v_0,
                    );
                    pResult_1 = Tcl_NewIntObj(v_0);
                }
                Tcl_SetObjResult(interp, pResult_1);
            }
            12 => {
                let mut zTable: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                let mut zFile: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                let mut zConflict: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<
                    sqlite3_stmt,
                >();
                let mut nCol: ::core::ffi::c_int = 0;
                let mut nByte: ::core::ffi::c_int = 0;
                let mut i: ::core::ffi::c_int = 0;
                let mut j: ::core::ffi::c_int = 0;
                let mut nSep: ::core::ffi::c_int = 0;
                let mut nNull: ::core::ffi::c_int = 0;
                let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                let mut zLine: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                let mut azCol: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    *mut ::core::ffi::c_char,
                >();
                let mut zCommit_0: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                let mut in_0: Tcl_Channel = ::core::ptr::null_mut::<Tcl_Channel_>();
                let mut lineno: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut zLineNum: [::core::ffi::c_char; 80] = [0; 80];
                let mut str: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
                let mut pResult_2: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
                let mut zSep: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                let mut zNull: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                if objc < 5 as ::core::ffi::c_int || objc > 7 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv as *const *mut Tcl_Obj,
                        b"CONFLICT-ALGORITHM TABLE FILENAME ?SEPARATOR? ?NULLINDICATOR?\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                if objc >= 6 as ::core::ffi::c_int {
                    zSep = Tcl_GetStringFromObj(
                        *objv.offset(5 as ::core::ffi::c_int as isize),
                        ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    );
                } else {
                    zSep = b"\t\0".as_ptr() as *const ::core::ffi::c_char;
                }
                if objc >= 7 as ::core::ffi::c_int {
                    zNull = Tcl_GetStringFromObj(
                        *objv.offset(6 as ::core::ffi::c_int as isize),
                        ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    );
                } else {
                    zNull = b"\0".as_ptr() as *const ::core::ffi::c_char;
                }
                zConflict = Tcl_GetStringFromObj(
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
                zTable = Tcl_GetStringFromObj(
                    *objv.offset(3 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
                zFile = Tcl_GetStringFromObj(
                    *objv.offset(4 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
                nSep = strlen30(zSep);
                nNull = strlen30(zNull);
                if nSep == 0 as ::core::ffi::c_int {
                    Tcl_AppendResult(
                        interp,
                        b"Error: non-null separator required for copy\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    );
                    return TCL_ERROR;
                }
                if strcmp(
                    zConflict,
                    b"rollback\0".as_ptr() as *const ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
                    && strcmp(
                        zConflict,
                        b"abort\0".as_ptr() as *const ::core::ffi::c_char,
                    ) != 0 as ::core::ffi::c_int
                    && strcmp(
                        zConflict,
                        b"fail\0".as_ptr() as *const ::core::ffi::c_char,
                    ) != 0 as ::core::ffi::c_int
                    && strcmp(
                        zConflict,
                        b"ignore\0".as_ptr() as *const ::core::ffi::c_char,
                    ) != 0 as ::core::ffi::c_int
                    && strcmp(
                        zConflict,
                        b"replace\0".as_ptr() as *const ::core::ffi::c_char,
                    ) != 0 as ::core::ffi::c_int
                {
                    Tcl_AppendResult(
                        interp,
                        b"Error: \"\0".as_ptr() as *const ::core::ffi::c_char,
                        zConflict,
                        b"\", conflict-algorithm must be one of: rollback, abort, fail, ignore, or replace\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    );
                    return TCL_ERROR;
                }
                zSql = sqlite3_mprintf(
                    b"SELECT * FROM '%q'\0".as_ptr() as *const ::core::ffi::c_char,
                    zTable,
                );
                if zSql.is_null() {
                    Tcl_AppendResult(
                        interp,
                        b"Error: no such table: \0".as_ptr()
                            as *const ::core::ffi::c_char,
                        zTable,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    );
                    return TCL_ERROR;
                }
                nByte = strlen30(zSql);
                rc = sqlite3_prepare(
                    (*pDb).db,
                    zSql,
                    -1 as ::core::ffi::c_int,
                    &raw mut pStmt,
                    ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                );
                sqlite3_free(zSql as *mut ::core::ffi::c_void);
                if rc != 0 {
                    Tcl_AppendResult(
                        interp,
                        b"Error: \0".as_ptr() as *const ::core::ffi::c_char,
                        sqlite3_errmsg((*pDb).db),
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    );
                    nCol = 0 as ::core::ffi::c_int;
                } else {
                    nCol = sqlite3_column_count(pStmt);
                }
                sqlite3_finalize(pStmt);
                if nCol == 0 as ::core::ffi::c_int {
                    return TCL_ERROR;
                }
                zSql = malloc(
                    (nByte + 50 as ::core::ffi::c_int + nCol * 2 as ::core::ffi::c_int)
                        as size_t,
                ) as *mut ::core::ffi::c_char;
                if zSql.is_null() {
                    Tcl_AppendResult(
                        interp,
                        b"Error: can't malloc()\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    );
                    return TCL_ERROR;
                }
                sqlite3_snprintf(
                    nByte + 50 as ::core::ffi::c_int,
                    zSql,
                    b"INSERT OR %q INTO '%q' VALUES(?\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    zConflict,
                    zTable,
                );
                j = strlen30(zSql);
                i = 1 as ::core::ffi::c_int;
                while i < nCol {
                    let c2rust_fresh38 = j;
                    j = j + 1;
                    *zSql.offset(c2rust_fresh38 as isize) = ',' as i32
                        as ::core::ffi::c_char;
                    let c2rust_fresh39 = j;
                    j = j + 1;
                    *zSql.offset(c2rust_fresh39 as isize) = '?' as i32
                        as ::core::ffi::c_char;
                    i += 1;
                }
                let c2rust_fresh40 = j;
                j = j + 1;
                *zSql.offset(c2rust_fresh40 as isize) = ')' as i32
                    as ::core::ffi::c_char;
                *zSql.offset(j as isize) = 0 as ::core::ffi::c_char;
                rc = sqlite3_prepare(
                    (*pDb).db,
                    zSql,
                    -1 as ::core::ffi::c_int,
                    &raw mut pStmt,
                    ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                );
                free(zSql as *mut ::core::ffi::c_void);
                if rc != 0 {
                    Tcl_AppendResult(
                        interp,
                        b"Error: \0".as_ptr() as *const ::core::ffi::c_char,
                        sqlite3_errmsg((*pDb).db),
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    );
                    sqlite3_finalize(pStmt);
                    return TCL_ERROR;
                }
                in_0 = Tcl_OpenFileChannel(
                    interp,
                    zFile,
                    b"rb\0".as_ptr() as *const ::core::ffi::c_char,
                    0o666 as ::core::ffi::c_int,
                );
                if in_0.is_null() {
                    sqlite3_finalize(pStmt);
                    return TCL_ERROR;
                }
                Tcl_SetChannelOption(
                    ::core::ptr::null_mut::<Tcl_Interp>(),
                    in_0,
                    b"-translation\0".as_ptr() as *const ::core::ffi::c_char,
                    b"auto\0".as_ptr() as *const ::core::ffi::c_char,
                );
                azCol = malloc(
                    (::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t)
                        .wrapping_mul((nCol + 1 as ::core::ffi::c_int) as size_t),
                ) as *mut *mut ::core::ffi::c_char;
                if azCol.is_null() {
                    Tcl_AppendResult(
                        interp,
                        b"Error: can't malloc()\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    );
                    Tcl_Close(interp, in_0);
                    return TCL_ERROR;
                }
                str = Tcl_NewObj();
                (*str).refCount += 1;
                sqlite3_exec(
                    (*pDb).db,
                    b"BEGIN\0".as_ptr() as *const ::core::ffi::c_char,
                    None,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                );
                zCommit_0 = b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char;
                while Tcl_GetsObj(in_0, str) >= 0 as ::core::ffi::c_int {
                    let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                        ::core::ffi::c_char,
                    >();
                    let mut byteLen: Tcl_Size = 0;
                    lineno += 1;
                    zLine = Tcl_GetByteArrayFromObj(str, &raw mut byteLen)
                        as *mut ::core::ffi::c_char;
                    let ref mut c2rust_fresh41 = *azCol
                        .offset(0 as ::core::ffi::c_int as isize);
                    *c2rust_fresh41 = zLine;
                    i = 0 as ::core::ffi::c_int;
                    z = zLine;
                    while *z != 0 {
                        if *z as ::core::ffi::c_int
                            == *zSep.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                            && strncmp(z, zSep, nSep as size_t)
                                == 0 as ::core::ffi::c_int
                        {
                            *z = 0 as ::core::ffi::c_char;
                            i += 1;
                            if i < nCol {
                                let ref mut c2rust_fresh42 = *azCol.offset(i as isize);
                                *c2rust_fresh42 = z.offset(nSep as isize)
                                    as *mut ::core::ffi::c_char;
                                z = z.offset((nSep - 1 as ::core::ffi::c_int) as isize);
                            }
                        }
                        z = z.offset(1);
                    }
                    if i + 1 as ::core::ffi::c_int != nCol {
                        let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                            ::core::ffi::c_char,
                        >();
                        let mut nErr: ::core::ffi::c_int = strlen30(zFile)
                            + 200 as ::core::ffi::c_int;
                        zErr = malloc(nErr as size_t) as *mut ::core::ffi::c_char;
                        if !zErr.is_null() {
                            sqlite3_snprintf(
                                nErr,
                                zErr,
                                b"Error: %s line %d: expected %d columns of data but found %d\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                zFile,
                                lineno,
                                nCol,
                                i + 1 as ::core::ffi::c_int,
                            );
                            Tcl_AppendResult(
                                interp,
                                zErr,
                                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            );
                            free(zErr as *mut ::core::ffi::c_void);
                        }
                        zCommit_0 = b"ROLLBACK\0".as_ptr() as *const ::core::ffi::c_char;
                        break;
                    } else {
                        i = 0 as ::core::ffi::c_int;
                        while i < nCol {
                            if nNull > 0 as ::core::ffi::c_int
                                && strcmp(*azCol.offset(i as isize), zNull)
                                    == 0 as ::core::ffi::c_int
                                || strlen30(*azCol.offset(i as isize))
                                    == 0 as ::core::ffi::c_int
                            {
                                sqlite3_bind_null(pStmt, i + 1 as ::core::ffi::c_int);
                            } else {
                                sqlite3_bind_text(
                                    pStmt,
                                    i + 1 as ::core::ffi::c_int,
                                    *azCol.offset(i as isize),
                                    -1 as ::core::ffi::c_int,
                                    SQLITE_STATIC,
                                );
                            }
                            i += 1;
                        }
                        sqlite3_step(pStmt);
                        rc = sqlite3_reset(pStmt);
                        Tcl_SetObjLength(str, 0 as ::core::ffi::c_int);
                        if !(rc != SQLITE_OK) {
                            continue;
                        }
                        Tcl_AppendResult(
                            interp,
                            b"Error: \0".as_ptr() as *const ::core::ffi::c_char,
                            sqlite3_errmsg((*pDb).db),
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        );
                        zCommit_0 = b"ROLLBACK\0".as_ptr() as *const ::core::ffi::c_char;
                        break;
                    }
                }
                let mut _objPtr_0: *mut Tcl_Obj = str;
                let c2rust_fresh43 = (*_objPtr_0).refCount;
                (*_objPtr_0).refCount = (*_objPtr_0).refCount - 1;
                if c2rust_fresh43 <= 1 as ::core::ffi::c_int {
                    TclFreeObj(_objPtr_0);
                }
                free(azCol as *mut ::core::ffi::c_void);
                Tcl_Close(interp, in_0);
                sqlite3_finalize(pStmt);
                sqlite3_exec(
                    (*pDb).db,
                    zCommit_0,
                    None,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                );
                if *zCommit_0.offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int == 'C' as i32
                {
                    pResult_2 = Tcl_GetObjResult(interp);
                    Tcl_SetIntObj(pResult_2, lineno);
                    rc = TCL_OK;
                } else {
                    sqlite3_snprintf(
                        ::core::mem::size_of::<[::core::ffi::c_char; 80]>()
                            as ::core::ffi::c_int,
                        &raw mut zLineNum as *mut ::core::ffi::c_char,
                        b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                        lineno,
                    );
                    Tcl_AppendResult(
                        interp,
                        b", failed while processing line: \0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut zLineNum as *mut ::core::ffi::c_char,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    );
                    rc = TCL_ERROR;
                }
            }
            13 => {
                let mut zSchema: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                let mut pValue: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
                let mut pBA: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
                    ::core::ffi::c_uchar,
                >();
                let mut pData: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
                    ::core::ffi::c_uchar,
                >();
                let mut len_3: Tcl_Size = 0;
                let mut xrc: ::core::ffi::c_int = 0;
                let mut mxSize: sqlite3_int64 = 0 as sqlite3_int64;
                let mut i_0: ::core::ffi::c_int = 0;
                let mut isReadonly: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                if objc < 3 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv as *const *mut Tcl_Obj,
                        b"?DATABASE? VALUE\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    rc = TCL_ERROR;
                } else {
                    i_0 = 2 as ::core::ffi::c_int;
                    loop {
                        if !(i_0 < objc - 1 as ::core::ffi::c_int) {
                            c2rust_current_block = 2126221883176060805;
                            break;
                        }
                        let mut z_0: *const ::core::ffi::c_char = Tcl_GetString(
                            *objv.offset(i_0 as isize),
                        );
                        if strcmp(
                            z_0,
                            b"-maxsize\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                            && i_0 < objc - 2 as ::core::ffi::c_int
                        {
                            let mut x: Tcl_WideInt = 0;
                            i_0 += 1;
                            rc = Tcl_GetWideIntFromObj(
                                interp,
                                *objv.offset(i_0 as isize),
                                &raw mut x,
                            );
                            if rc != 0 {
                                c2rust_current_block = 9981768500251144183;
                                break;
                            }
                            mxSize = x as sqlite3_int64;
                        } else if strcmp(
                            z_0,
                            b"-readonly\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                            && i_0 < objc - 2 as ::core::ffi::c_int
                        {
                            i_0 += 1;
                            rc = Tcl_GetBooleanFromObj(
                                interp,
                                *objv.offset(i_0 as isize),
                                &raw mut isReadonly,
                            );
                            if rc != 0 {
                                c2rust_current_block = 9981768500251144183;
                                break;
                            }
                        } else if zSchema.is_null()
                            && i_0 == objc - 2 as ::core::ffi::c_int
                            && *z_0.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int != '-' as i32
                        {
                            zSchema = z_0;
                        } else {
                            Tcl_AppendResult(
                                interp,
                                b"unknown option: \0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                z_0,
                                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            );
                            rc = TCL_ERROR;
                            c2rust_current_block = 9981768500251144183;
                            break;
                        }
                        i_0 += 1;
                    }
                    match c2rust_current_block {
                        9981768500251144183 => {}
                        _ => {
                            pValue = *objv
                                .offset((objc - 1 as ::core::ffi::c_int) as isize);
                            pBA = Tcl_GetByteArrayFromObj(pValue, &raw mut len_3);
                            pData = sqlite3_malloc64(len_3 as sqlite3_uint64)
                                as *mut ::core::ffi::c_uchar;
                            if pData.is_null() && len_3 > 0 as ::core::ffi::c_int {
                                Tcl_AppendResult(
                                    interp,
                                    b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
                                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                );
                                rc = TCL_ERROR;
                            } else {
                                let mut flags: ::core::ffi::c_int = 0;
                                if len_3 > 0 as ::core::ffi::c_int {
                                    memcpy(
                                        pData as *mut ::core::ffi::c_void,
                                        pBA as *const ::core::ffi::c_void,
                                        len_3 as size_t,
                                    );
                                }
                                if isReadonly != 0 {
                                    flags = SQLITE_DESERIALIZE_FREEONCLOSE
                                        | SQLITE_DESERIALIZE_READONLY;
                                } else {
                                    flags = SQLITE_DESERIALIZE_FREEONCLOSE
                                        | SQLITE_DESERIALIZE_RESIZEABLE;
                                }
                                xrc = sqlite3_deserialize(
                                    (*pDb).db,
                                    zSchema,
                                    pData,
                                    len_3 as sqlite3_int64,
                                    len_3 as sqlite3_int64,
                                    flags as ::core::ffi::c_uint,
                                );
                                if xrc != 0 {
                                    Tcl_AppendResult(
                                        interp,
                                        b"unable to set MEMDB content\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                    );
                                    rc = TCL_ERROR;
                                }
                                if mxSize > 0 as ::core::ffi::c_longlong {
                                    sqlite3_file_control(
                                        (*pDb).db,
                                        zSchema,
                                        SQLITE_FCNTL_SIZE_LIMIT,
                                        &raw mut mxSize as *mut ::core::ffi::c_void,
                                    );
                                }
                            }
                        }
                    }
                }
            }
            14 => {
                Tcl_AppendResult(
                    interp,
                    b"extension loading is turned off at compile-time\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                );
                return TCL_ERROR;
            }
            15 => {
                Tcl_SetObjResult(interp, Tcl_NewIntObj(sqlite3_errcode((*pDb).db)));
            }
            16 => {
                Tcl_SetObjResult(interp, Tcl_NewIntObj(sqlite3_error_offset((*pDb).db)));
            }
            18 | 24 => {
                let mut pResult_3: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
                let mut sEval: DbEvalContext = DbEvalContext {
                    pDb: ::core::ptr::null_mut::<SqliteDb>(),
                    pSql: ::core::ptr::null_mut::<Tcl_Obj>(),
                    zSql: ::core::ptr::null::<::core::ffi::c_char>(),
                    pPreStmt: ::core::ptr::null_mut::<SqlPreparedStmt>(),
                    nCol: 0,
                    evalFlags: 0,
                    pVarName: ::core::ptr::null_mut::<Tcl_Obj>(),
                    apColName: ::core::ptr::null_mut::<*mut Tcl_Obj>()

                };
                if objc != 3 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv as *const *mut Tcl_Obj,
                        b"SQL\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                dbEvalInit(
                    &raw mut sEval,
                    pDb,
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<Tcl_Obj>(),
                    0 as ::core::ffi::c_int,
                );
                rc = dbEvalStep(&raw mut sEval);
                if choice == DB_ONECOLUMN as ::core::ffi::c_int {
                    if rc == TCL_OK {
                        pResult_3 = dbEvalColumnValue(
                            &raw mut sEval,
                            0 as ::core::ffi::c_int,
                        );
                    } else if rc == TCL_BREAK {
                        Tcl_ResetResult(interp);
                    }
                } else if rc == TCL_BREAK || rc == TCL_OK {
                    pResult_3 = Tcl_NewIntObj(
                        ((rc == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                            != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
                    );
                }
                dbEvalFinalize(&raw mut sEval);
                if !pResult_3.is_null() {
                    Tcl_SetObjResult(interp, pResult_3);
                }
                if rc == TCL_BREAK {
                    rc = TCL_OK;
                }
            }
            17 => {
                let mut evalFlags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut zOpt_0: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                while objc > 3 as ::core::ffi::c_int
                    && {
                        zOpt_0 = Tcl_GetString(
                            *objv.offset(2 as ::core::ffi::c_int as isize),
                        );
                        !zOpt_0.is_null()
                    }
                    && *zOpt_0.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == '-' as i32
                {
                    if strcmp(
                        zOpt_0,
                        b"-withoutnulls\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        evalFlags |= SQLITE_EVAL_WITHOUTNULLS;
                    } else if strcmp(
                        zOpt_0,
                        b"-asdict\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        evalFlags |= SQLITE_EVAL_ASDICT;
                    } else {
                        Tcl_AppendResult(
                            interp,
                            b"unknown option: \"\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            zOpt_0,
                            b"\"\0".as_ptr() as *const ::core::ffi::c_char,
                            ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        );
                        return TCL_ERROR;
                    }
                    objc -= 1;
                    objv = objv.offset(1);
                }
                if objc < 3 as ::core::ffi::c_int || objc > 5 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv as *const *mut Tcl_Obj,
                        b"?OPTIONS? SQL ?VAR-NAME? ?SCRIPT?\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                if objc == 3 as ::core::ffi::c_int {
                    let mut sEval_0: DbEvalContext = DbEvalContext {
                        pDb: ::core::ptr::null_mut::<SqliteDb>(),
                        pSql: ::core::ptr::null_mut::<Tcl_Obj>(),
                        zSql: ::core::ptr::null::<::core::ffi::c_char>(),
                        pPreStmt: ::core::ptr::null_mut::<SqlPreparedStmt>(),
                        nCol: 0,
                        evalFlags: 0,
                        pVarName: ::core::ptr::null_mut::<Tcl_Obj>(),
                        apColName: ::core::ptr::null_mut::<*mut Tcl_Obj>()

                    };
                    let mut pRet: *mut Tcl_Obj = Tcl_NewObj();
                    (*pRet).refCount += 1;
                    dbEvalInit(
                        &raw mut sEval_0,
                        pDb,
                        *objv.offset(2 as ::core::ffi::c_int as isize),
                        ::core::ptr::null_mut::<Tcl_Obj>(),
                        0 as ::core::ffi::c_int,
                    );
                    loop {
                        rc = dbEvalStep(&raw mut sEval_0);
                        if !(TCL_OK == rc) {
                            break;
                        }
                        let mut i_1: ::core::ffi::c_int = 0;
                        let mut nCol_0: ::core::ffi::c_int = 0;
                        dbEvalRowInfo(
                            &raw mut sEval_0,
                            &raw mut nCol_0,
                            ::core::ptr::null_mut::<*mut *mut Tcl_Obj>(),
                        );
                        i_1 = 0 as ::core::ffi::c_int;
                        while i_1 < nCol_0 {
                            Tcl_ListObjAppendElement(
                                interp,
                                pRet,
                                dbEvalColumnValue(&raw mut sEval_0, i_1),
                            );
                            i_1 += 1;
                        }
                    }
                    dbEvalFinalize(&raw mut sEval_0);
                    if rc == TCL_BREAK {
                        Tcl_SetObjResult(interp, pRet);
                        rc = TCL_OK;
                    }
                    let mut _objPtr_1: *mut Tcl_Obj = pRet;
                    let c2rust_fresh44 = (*_objPtr_1).refCount;
                    (*_objPtr_1).refCount = (*_objPtr_1).refCount - 1;
                    if c2rust_fresh44 <= 1 as ::core::ffi::c_int {
                        TclFreeObj(_objPtr_1);
                    }
                } else {
                    let mut cd2: [ClientData; 2] = [::core::ptr::null_mut::<
                        ::core::ffi::c_void,
                    >(); 2];
                    let mut p: *mut DbEvalContext = ::core::ptr::null_mut::<
                        DbEvalContext,
                    >();
                    let mut pVarName: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
                    let mut pScript: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
                    if objc >= 5 as ::core::ffi::c_int
                        && *Tcl_GetString(*objv.offset(3 as ::core::ffi::c_int as isize))
                            as ::core::ffi::c_int != 0
                    {
                        pVarName = *objv.offset(3 as ::core::ffi::c_int as isize);
                    }
                    pScript = *objv.offset((objc - 1 as ::core::ffi::c_int) as isize);
                    (*pScript).refCount += 1;
                    p = Tcl_Alloc(
                        ::core::mem::size_of::<DbEvalContext>() as ::core::ffi::c_uint,
                    ) as *mut DbEvalContext;
                    dbEvalInit(
                        p,
                        pDb,
                        *objv.offset(2 as ::core::ffi::c_int as isize),
                        pVarName,
                        evalFlags,
                    );
                    cd2[0 as ::core::ffi::c_int as usize] = p as *mut ::core::ffi::c_void
                        as ClientData;
                    cd2[1 as ::core::ffi::c_int as usize] = pScript
                        as *mut ::core::ffi::c_void as ClientData;
                    rc = DbEvalNextCmd(&raw mut cd2 as *mut ClientData, interp, TCL_OK);
                }
            }
            19 => {
                let mut flags_0: ::core::ffi::c_int = SQLITE_UTF8;
                let mut pFunc: *mut SqlFunc = ::core::ptr::null_mut::<SqlFunc>();
                let mut pScript_0: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
                let mut zName_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                let mut nArg: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
                let mut i_2: ::core::ffi::c_int = 0;
                let mut eType: ::core::ffi::c_int = SQLITE_NULL;
                if objc < 4 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv as *const *mut Tcl_Obj,
                        b"NAME ?SWITCHES? SCRIPT\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                i_2 = 3 as ::core::ffi::c_int;
                while i_2 < objc - 1 as ::core::ffi::c_int {
                    let mut z_1: *const ::core::ffi::c_char = Tcl_GetString(
                        *objv.offset(i_2 as isize),
                    );
                    let mut n_0: ::core::ffi::c_int = strlen30(z_1);
                    if n_0 > 1 as ::core::ffi::c_int
                        && strncmp(
                            z_1,
                            b"-argcount\0".as_ptr() as *const ::core::ffi::c_char,
                            n_0 as size_t,
                        ) == 0 as ::core::ffi::c_int
                    {
                        if i_2 == objc - 2 as ::core::ffi::c_int {
                            Tcl_AppendResult(
                                interp,
                                b"option requires an argument: \0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                z_1,
                                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            );
                            return TCL_ERROR;
                        }
                        if Tcl_GetIntFromObj(
                            interp,
                            *objv.offset((i_2 + 1 as ::core::ffi::c_int) as isize),
                            &raw mut nArg,
                        ) != 0
                        {
                            return TCL_ERROR;
                        }
                        if nArg < 0 as ::core::ffi::c_int {
                            Tcl_AppendResult(
                                interp,
                                b"number of arguments must be non-negative\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            );
                            return TCL_ERROR;
                        }
                        i_2 += 1;
                    } else if n_0 > 1 as ::core::ffi::c_int
                        && strncmp(
                            z_1,
                            b"-deterministic\0".as_ptr() as *const ::core::ffi::c_char,
                            n_0 as size_t,
                        ) == 0 as ::core::ffi::c_int
                    {
                        flags_0 |= SQLITE_DETERMINISTIC;
                    } else if n_0 > 1 as ::core::ffi::c_int
                        && strncmp(
                            z_1,
                            b"-directonly\0".as_ptr() as *const ::core::ffi::c_char,
                            n_0 as size_t,
                        ) == 0 as ::core::ffi::c_int
                    {
                        flags_0 |= SQLITE_DIRECTONLY;
                    } else if n_0 > 1 as ::core::ffi::c_int
                        && strncmp(
                            z_1,
                            b"-innocuous\0".as_ptr() as *const ::core::ffi::c_char,
                            n_0 as size_t,
                        ) == 0 as ::core::ffi::c_int
                    {
                        flags_0 |= SQLITE_INNOCUOUS;
                    } else if n_0 > 1 as ::core::ffi::c_int
                        && strncmp(
                            z_1,
                            b"-returntype\0".as_ptr() as *const ::core::ffi::c_char,
                            n_0 as size_t,
                        ) == 0 as ::core::ffi::c_int
                    {
                        let mut azType: [*const ::core::ffi::c_char; 6] = [
                            b"integer\0".as_ptr() as *const ::core::ffi::c_char,
                            b"real\0".as_ptr() as *const ::core::ffi::c_char,
                            b"text\0".as_ptr() as *const ::core::ffi::c_char,
                            b"blob\0".as_ptr() as *const ::core::ffi::c_char,
                            b"any\0".as_ptr() as *const ::core::ffi::c_char,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                        ];
                        '_c2rust_label: {
                            if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                                && 2 as ::core::ffi::c_int == 2 as ::core::ffi::c_int
                                && 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int
                            {} else {
                                __assert_fail(
                                    b"SQLITE_INTEGER==1 && SQLITE_FLOAT==2 && SQLITE_TEXT==3\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    b"/sqlite/src/tclsqlite.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    3033 as ::core::ffi::c_uint,
                                    b"int DbObjCmd(void *, Tcl_Interp *, int, Tcl_Obj *const *)\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                );
                            }
                        };
                        '_c2rust_label_0: {
                            if 4 as ::core::ffi::c_int == 4 as ::core::ffi::c_int
                                && 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int
                            {} else {
                                __assert_fail(
                                    b"SQLITE_BLOB==4 && SQLITE_NULL==5\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"/sqlite/src/tclsqlite.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    3034 as ::core::ffi::c_uint,
                                    b"int DbObjCmd(void *, Tcl_Interp *, int, Tcl_Obj *const *)\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                );
                            }
                        };
                        if i_2 == objc - 2 as ::core::ffi::c_int {
                            Tcl_AppendResult(
                                interp,
                                b"option requires an argument: \0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                z_1,
                                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            );
                            return TCL_ERROR;
                        }
                        i_2 += 1;
                        if Tcl_GetIndexFromObjStruct(
                            interp,
                            *objv.offset(i_2 as isize),
                            &raw mut azType as *mut *const ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<*mut ::core::ffi::c_char>()
                                as ::core::ffi::c_int,
                            b"type\0".as_ptr() as *const ::core::ffi::c_char,
                            0 as ::core::ffi::c_int,
                            &raw mut eType,
                        ) != 0
                        {
                            return TCL_ERROR;
                        }
                        eType += 1;
                    } else {
                        Tcl_AppendResult(
                            interp,
                            b"bad option \"\0".as_ptr() as *const ::core::ffi::c_char,
                            z_1,
                            b"\": must be -argcount, -deterministic, -directonly, -innocuous, or -returntype\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        );
                        return TCL_ERROR;
                    }
                    i_2 += 1;
                }
                pScript_0 = *objv.offset((objc - 1 as ::core::ffi::c_int) as isize);
                zName_0 = Tcl_GetStringFromObj(
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
                pFunc = findSqlFunc(pDb, zName_0);
                if pFunc.is_null() {
                    return TCL_ERROR;
                }
                if !(*pFunc).pScript.is_null() {
                    let mut _objPtr_2: *mut Tcl_Obj = (*pFunc).pScript;
                    let c2rust_fresh45 = (*_objPtr_2).refCount;
                    (*_objPtr_2).refCount = (*_objPtr_2).refCount - 1;
                    if c2rust_fresh45 <= 1 as ::core::ffi::c_int {
                        TclFreeObj(_objPtr_2);
                    }
                }
                (*pFunc).pScript = pScript_0;
                (*pScript_0).refCount += 1;
                (*pFunc).useEvalObjv = safeToUseEvalObjv(pScript_0);
                (*pFunc).eType = eType;
                rc = sqlite3_create_function(
                    (*pDb).db,
                    zName_0,
                    nArg,
                    flags_0,
                    pFunc as *mut ::core::ffi::c_void,
                    Some(
                        tclSqlFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    None,
                    None,
                );
                if rc != SQLITE_OK {
                    rc = TCL_ERROR;
                    Tcl_SetResult(
                        interp,
                        sqlite3_errmsg((*pDb).db) as *mut ::core::ffi::c_char,
                        ::core::mem::transmute::<
                            ::libc::intptr_t,
                            Option<Tcl_FreeProc>,
                        >(1 as ::core::ffi::c_int as ::libc::intptr_t),
                    );
                }
            }
            20 => {
                let mut isReadonly_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut zDb: *const ::core::ffi::c_char = b"main\0".as_ptr()
                    as *const ::core::ffi::c_char;
                let mut zTable_0: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                let mut zColumn: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                let mut iRow: Tcl_WideInt = 0;
                if objc > 3 as ::core::ffi::c_int
                    && strcmp(
                        Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize)),
                        b"-readonly\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                {
                    isReadonly_0 = 1 as ::core::ffi::c_int;
                }
                if objc != 5 as ::core::ffi::c_int + isReadonly_0
                    && objc != 6 as ::core::ffi::c_int + isReadonly_0
                {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv as *const *mut Tcl_Obj,
                        b"?-readonly? ?DB? TABLE COLUMN ROWID\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                if objc == 6 as ::core::ffi::c_int + isReadonly_0 {
                    zDb = Tcl_GetString(
                        *objv.offset((2 as ::core::ffi::c_int + isReadonly_0) as isize),
                    );
                }
                zTable_0 = Tcl_GetString(
                    *objv.offset((objc - 3 as ::core::ffi::c_int) as isize),
                );
                zColumn = Tcl_GetString(
                    *objv.offset((objc - 2 as ::core::ffi::c_int) as isize),
                );
                rc = Tcl_GetWideIntFromObj(
                    interp,
                    *objv.offset((objc - 1 as ::core::ffi::c_int) as isize),
                    &raw mut iRow,
                );
                if rc == TCL_OK {
                    rc = createIncrblobChannel(
                        interp,
                        pDb,
                        zDb,
                        zTable_0,
                        zColumn,
                        iRow,
                        isReadonly_0,
                    );
                }
            }
            21 => {
                sqlite3_interrupt((*pDb).db);
            }
            23 => {
                if objc != 2 as ::core::ffi::c_int && objc != 3 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv as *const *mut Tcl_Obj,
                        b"NULLVALUE\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                if objc == 3 as ::core::ffi::c_int {
                    let mut len_4: Tcl_Size = 0;
                    let mut zNull_0: *mut ::core::ffi::c_char = Tcl_GetStringFromObj(
                        *objv.offset(2 as ::core::ffi::c_int as isize),
                        &raw mut len_4,
                    );
                    if !(*pDb).zNull.is_null() {
                        Tcl_Free((*pDb).zNull);
                    }
                    if !zNull_0.is_null() && len_4 > 0 as ::core::ffi::c_int {
                        (*pDb).zNull = Tcl_Alloc(
                            (len_4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_uint,
                        );
                        memcpy(
                            (*pDb).zNull as *mut ::core::ffi::c_void,
                            zNull_0 as *const ::core::ffi::c_void,
                            len_4 as size_t,
                        );
                        *(*pDb).zNull.offset(len_4 as isize) = '\0' as i32
                            as ::core::ffi::c_char;
                    } else {
                        (*pDb).zNull = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    }
                }
                Tcl_SetObjResult(
                    interp,
                    Tcl_NewStringObj((*pDb).zNull, -1 as ::core::ffi::c_int),
                );
            }
            22 => {
                let mut pResult_4: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
                let mut rowid: Tcl_WideInt = 0;
                if objc != 2 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv as *const *mut Tcl_Obj,
                        b"\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                rowid = sqlite3_last_insert_rowid((*pDb).db) as Tcl_WideInt;
                pResult_4 = Tcl_GetObjResult(interp);
                Tcl_SetWideIntObj(pResult_4, rowid);
            }
            27 => {
                if objc == 2 as ::core::ffi::c_int {
                    if !(*pDb).zProgress.is_null() {
                        Tcl_AppendResult(
                            interp,
                            (*pDb).zProgress,
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        );
                    }
                    sqlite3_progress_handler(
                        (*pDb).db,
                        0 as ::core::ffi::c_int,
                        None,
                        ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    );
                } else if objc == 4 as ::core::ffi::c_int {
                    let mut zProgress: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                        ::core::ffi::c_char,
                    >();
                    let mut len_5: Tcl_Size = 0;
                    let mut N: ::core::ffi::c_int = 0;
                    if TCL_OK
                        != Tcl_GetIntFromObj(
                            interp,
                            *objv.offset(2 as ::core::ffi::c_int as isize),
                            &raw mut N,
                        )
                    {
                        return TCL_ERROR;
                    }
                    if !(*pDb).zProgress.is_null() {
                        Tcl_Free((*pDb).zProgress);
                    }
                    zProgress = Tcl_GetStringFromObj(
                        *objv.offset(3 as ::core::ffi::c_int as isize),
                        &raw mut len_5,
                    );
                    if !zProgress.is_null() && len_5 > 0 as ::core::ffi::c_int {
                        (*pDb).zProgress = Tcl_Alloc(
                            (len_5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_uint,
                        );
                        memcpy(
                            (*pDb).zProgress as *mut ::core::ffi::c_void,
                            zProgress as *const ::core::ffi::c_void,
                            (len_5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                as size_t,
                        );
                    } else {
                        (*pDb).zProgress = ::core::ptr::null_mut::<
                            ::core::ffi::c_char,
                        >();
                    }
                    if !(*pDb).zProgress.is_null() {
                        (*pDb).interp = interp;
                        sqlite3_progress_handler(
                            (*pDb).db,
                            N,
                            Some(
                                DbProgressHandler
                                    as unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                    ) -> ::core::ffi::c_int,
                            ),
                            pDb as *mut ::core::ffi::c_void,
                        );
                    } else {
                        sqlite3_progress_handler(
                            (*pDb).db,
                            0 as ::core::ffi::c_int,
                            None,
                            ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        );
                    }
                } else {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv as *const *mut Tcl_Obj,
                        b"N CALLBACK\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
            }
            26 => {
                if objc > 3 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv as *const *mut Tcl_Obj,
                        b"?CALLBACK?\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                } else if objc == 2 as ::core::ffi::c_int {
                    if !(*pDb).zProfile.is_null() {
                        Tcl_AppendResult(
                            interp,
                            (*pDb).zProfile,
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        );
                    }
                } else {
                    let mut zProfile: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                        ::core::ffi::c_char,
                    >();
                    let mut len_6: Tcl_Size = 0;
                    if !(*pDb).zProfile.is_null() {
                        Tcl_Free((*pDb).zProfile);
                    }
                    zProfile = Tcl_GetStringFromObj(
                        *objv.offset(2 as ::core::ffi::c_int as isize),
                        &raw mut len_6,
                    );
                    if !zProfile.is_null() && len_6 > 0 as ::core::ffi::c_int {
                        (*pDb).zProfile = Tcl_Alloc(
                            (len_6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_uint,
                        );
                        memcpy(
                            (*pDb).zProfile as *mut ::core::ffi::c_void,
                            zProfile as *const ::core::ffi::c_void,
                            (len_6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                as size_t,
                        );
                    } else {
                        (*pDb).zProfile = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    }
                }
            }
            28 => {
                if objc != 3 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv as *const *mut Tcl_Obj,
                        b"KEY\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
            }
            29 => {
                let mut zSrcFile: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                let mut zDestDb: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                let mut pSrc: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
                let mut pBackup_0: *mut sqlite3_backup = ::core::ptr::null_mut::<
                    sqlite3_backup,
                >();
                let mut nTimeout: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                if objc == 3 as ::core::ffi::c_int {
                    zDestDb = b"main\0".as_ptr() as *const ::core::ffi::c_char;
                    zSrcFile = Tcl_GetString(
                        *objv.offset(2 as ::core::ffi::c_int as isize),
                    );
                } else if objc == 4 as ::core::ffi::c_int {
                    zDestDb = Tcl_GetString(
                        *objv.offset(2 as ::core::ffi::c_int as isize),
                    );
                    zSrcFile = Tcl_GetString(
                        *objv.offset(3 as ::core::ffi::c_int as isize),
                    );
                } else {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv as *const *mut Tcl_Obj,
                        b"?DATABASE? FILENAME\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                rc = sqlite3_open_v2(
                    zSrcFile,
                    &raw mut pSrc,
                    SQLITE_OPEN_READONLY | (*pDb).openFlags,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
                if rc != SQLITE_OK {
                    Tcl_AppendResult(
                        interp,
                        b"cannot open source database: \0".as_ptr()
                            as *const ::core::ffi::c_char,
                        sqlite3_errmsg(pSrc),
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    );
                    sqlite3_close(pSrc);
                    return TCL_ERROR;
                }
                pBackup_0 = sqlite3_backup_init(
                    (*pDb).db,
                    zDestDb,
                    pSrc,
                    b"main\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if pBackup_0.is_null() {
                    Tcl_AppendResult(
                        interp,
                        b"restore failed: \0".as_ptr() as *const ::core::ffi::c_char,
                        sqlite3_errmsg((*pDb).db),
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    );
                    sqlite3_close(pSrc);
                    return TCL_ERROR;
                }
                loop {
                    rc = sqlite3_backup_step(pBackup_0, 100 as ::core::ffi::c_int);
                    if !(rc == SQLITE_OK || rc == SQLITE_BUSY) {
                        break;
                    }
                    if !(rc == SQLITE_BUSY) {
                        continue;
                    }
                    let c2rust_fresh46 = nTimeout;
                    nTimeout = nTimeout + 1;
                    if c2rust_fresh46 >= 3 as ::core::ffi::c_int {
                        break;
                    }
                    sqlite3_sleep(100 as ::core::ffi::c_int);
                }
                sqlite3_backup_finish(pBackup_0);
                if rc == SQLITE_DONE {
                    rc = TCL_OK;
                } else if rc == SQLITE_BUSY || rc == SQLITE_LOCKED {
                    Tcl_AppendResult(
                        interp,
                        b"restore failed: source database busy\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    );
                    rc = TCL_ERROR;
                } else {
                    Tcl_AppendResult(
                        interp,
                        b"restore failed: \0".as_ptr() as *const ::core::ffi::c_char,
                        sqlite3_errmsg((*pDb).db),
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    );
                    rc = TCL_ERROR;
                }
                sqlite3_close(pSrc);
            }
            31 => {
                let mut zSchema_0: *const ::core::ffi::c_char = if objc
                    >= 3 as ::core::ffi::c_int
                {
                    Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize))
                        as *const ::core::ffi::c_char
                } else {
                    b"main\0".as_ptr() as *const ::core::ffi::c_char
                };
                let mut sz: sqlite3_int64 = 0 as sqlite3_int64;
                let mut pData_0: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
                    ::core::ffi::c_uchar,
                >();
                if objc != 2 as ::core::ffi::c_int && objc != 3 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv as *const *mut Tcl_Obj,
                        b"?DATABASE?\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    rc = TCL_ERROR;
                } else {
                    let mut needFree: ::core::ffi::c_int = 0;
                    pData_0 = sqlite3_serialize(
                        (*pDb).db,
                        zSchema_0,
                        &raw mut sz,
                        SQLITE_SERIALIZE_NOCOPY as ::core::ffi::c_uint,
                    );
                    if !pData_0.is_null() {
                        needFree = 0 as ::core::ffi::c_int;
                    } else {
                        pData_0 = sqlite3_serialize(
                            (*pDb).db,
                            zSchema_0,
                            &raw mut sz,
                            0 as ::core::ffi::c_uint,
                        );
                        needFree = 1 as ::core::ffi::c_int;
                    }
                    Tcl_SetObjResult(
                        interp,
                        Tcl_NewByteArrayObj(pData_0, sz as ::core::ffi::c_int),
                    );
                    if needFree != 0 {
                        sqlite3_free(pData_0 as *mut ::core::ffi::c_void);
                    }
                }
            }
            32 => {
                let mut v_1: ::core::ffi::c_int = 0;
                let mut zOp: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                if objc != 3 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv as *const *mut Tcl_Obj,
                        b"(step|sort|autoindex)\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                zOp = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
                if strcmp(zOp, b"step\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    v_1 = (*pDb).nStep;
                } else if strcmp(zOp, b"sort\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    v_1 = (*pDb).nSort;
                } else if strcmp(
                    zOp,
                    b"autoindex\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    v_1 = (*pDb).nIndex;
                } else if strcmp(zOp, b"vmstep\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    v_1 = (*pDb).nVMStep;
                } else {
                    Tcl_AppendResult(
                        interp,
                        b"bad argument: should be autoindex, step, sort or vmstep\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    );
                    return TCL_ERROR;
                }
                Tcl_SetObjResult(interp, Tcl_NewIntObj(v_1));
            }
            33 => {
                let mut ms: ::core::ffi::c_int = 0;
                if objc != 3 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv as *const *mut Tcl_Obj,
                        b"MILLISECONDS\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                if Tcl_GetIntFromObj(
                    interp,
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut ms,
                ) != 0
                {
                    return TCL_ERROR;
                }
                sqlite3_busy_timeout((*pDb).db, ms);
            }
            34 => {
                let mut pResult_5: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
                if objc != 2 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv as *const *mut Tcl_Obj,
                        b"\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                pResult_5 = Tcl_GetObjResult(interp);
                Tcl_SetWideIntObj(
                    pResult_5,
                    sqlite3_total_changes64((*pDb).db) as Tcl_WideInt,
                );
            }
            35 => {
                if objc > 3 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv as *const *mut Tcl_Obj,
                        b"?CALLBACK?\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                } else if objc == 2 as ::core::ffi::c_int {
                    if !(*pDb).zTrace.is_null() {
                        Tcl_AppendResult(
                            interp,
                            (*pDb).zTrace,
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        );
                    }
                } else {
                    let mut zTrace: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                        ::core::ffi::c_char,
                    >();
                    let mut len_7: Tcl_Size = 0;
                    if !(*pDb).zTrace.is_null() {
                        Tcl_Free((*pDb).zTrace);
                    }
                    zTrace = Tcl_GetStringFromObj(
                        *objv.offset(2 as ::core::ffi::c_int as isize),
                        &raw mut len_7,
                    );
                    if !zTrace.is_null() && len_7 > 0 as ::core::ffi::c_int {
                        (*pDb).zTrace = Tcl_Alloc(
                            (len_7 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_uint,
                        );
                        memcpy(
                            (*pDb).zTrace as *mut ::core::ffi::c_void,
                            zTrace as *const ::core::ffi::c_void,
                            (len_7 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                as size_t,
                        );
                    } else {
                        (*pDb).zTrace = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    }
                }
            }
            36 => {
                if objc > 4 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv as *const *mut Tcl_Obj,
                        b"?CALLBACK? ?MASK?\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                } else if objc == 2 as ::core::ffi::c_int {
                    if !(*pDb).zTraceV2.is_null() {
                        Tcl_AppendResult(
                            interp,
                            (*pDb).zTraceV2,
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        );
                    }
                } else {
                    let mut zTraceV2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                        ::core::ffi::c_char,
                    >();
                    let mut len_8: Tcl_Size = 0;
                    let mut wMask: Tcl_WideInt = 0 as Tcl_WideInt;
                    if objc == 4 as ::core::ffi::c_int {
                        static mut TTYPE_strs: [*const ::core::ffi::c_char; 5] = [
                            b"statement\0".as_ptr() as *const ::core::ffi::c_char,
                            b"profile\0".as_ptr() as *const ::core::ffi::c_char,
                            b"row\0".as_ptr() as *const ::core::ffi::c_char,
                            b"close\0".as_ptr() as *const ::core::ffi::c_char,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                        ];
                        let mut i_3: Tcl_Size = 0;
                        if TCL_OK
                            != Tcl_ListObjLength(
                                interp,
                                *objv.offset(3 as ::core::ffi::c_int as isize),
                                &raw mut len_8,
                            )
                        {
                            return TCL_ERROR;
                        }
                        i_3 = 0 as ::core::ffi::c_int as Tcl_Size;
                        while i_3 < len_8 {
                            let mut pObj: *mut Tcl_Obj = ::core::ptr::null_mut::<
                                Tcl_Obj,
                            >();
                            let mut ttype: ::core::ffi::c_int = 0;
                            if TCL_OK
                                != Tcl_ListObjIndex(
                                    interp,
                                    *objv.offset(3 as ::core::ffi::c_int as isize),
                                    i_3 as ::core::ffi::c_int,
                                    &raw mut pObj,
                                )
                            {
                                return TCL_ERROR;
                            }
                            if Tcl_GetIndexFromObjStruct(
                                interp,
                                pObj,
                                &raw mut TTYPE_strs as *mut *const ::core::ffi::c_char
                                    as *const ::core::ffi::c_void,
                                ::core::mem::size_of::<*mut ::core::ffi::c_char>()
                                    as ::core::ffi::c_int,
                                b"trace type\0".as_ptr() as *const ::core::ffi::c_char,
                                0 as ::core::ffi::c_int,
                                &raw mut ttype,
                            ) != TCL_OK
                            {
                                let mut wType: Tcl_WideInt = 0;
                                let mut pError: *mut Tcl_Obj = Tcl_DuplicateObj(
                                    Tcl_GetObjResult(interp),
                                );
                                (*pError).refCount += 1;
                                if TCL_OK
                                    == Tcl_GetWideIntFromObj(interp, pObj, &raw mut wType)
                                {
                                    let mut _objPtr_3: *mut Tcl_Obj = pError;
                                    let c2rust_fresh47 = (*_objPtr_3).refCount;
                                    (*_objPtr_3).refCount = (*_objPtr_3).refCount - 1;
                                    if c2rust_fresh47 <= 1 as ::core::ffi::c_int {
                                        TclFreeObj(_objPtr_3);
                                    }
                                    wMask |= wType as ::core::ffi::c_longlong;
                                } else {
                                    Tcl_SetObjResult(interp, pError);
                                    let mut _objPtr_4: *mut Tcl_Obj = pError;
                                    let c2rust_fresh48 = (*_objPtr_4).refCount;
                                    (*_objPtr_4).refCount = (*_objPtr_4).refCount - 1;
                                    if c2rust_fresh48 <= 1 as ::core::ffi::c_int {
                                        TclFreeObj(_objPtr_4);
                                    }
                                    return TCL_ERROR;
                                }
                            } else {
                                match ttype as TTYPE_enum_0 as ::core::ffi::c_uint {
                                    0 => {
                                        wMask |= SQLITE_TRACE_STMT as ::core::ffi::c_longlong;
                                    }
                                    1 => {
                                        wMask |= SQLITE_TRACE_PROFILE as ::core::ffi::c_longlong;
                                    }
                                    2 => {
                                        wMask |= SQLITE_TRACE_ROW as ::core::ffi::c_longlong;
                                    }
                                    3 => {
                                        wMask |= SQLITE_TRACE_CLOSE as ::core::ffi::c_longlong;
                                    }
                                    _ => {}
                                }
                            }
                            i_3 += 1;
                        }
                    } else {
                        wMask = SQLITE_TRACE_STMT as Tcl_WideInt;
                    }
                    if !(*pDb).zTraceV2.is_null() {
                        Tcl_Free((*pDb).zTraceV2);
                    }
                    zTraceV2 = Tcl_GetStringFromObj(
                        *objv.offset(2 as ::core::ffi::c_int as isize),
                        &raw mut len_8,
                    );
                    if !zTraceV2.is_null() && len_8 > 0 as ::core::ffi::c_int {
                        (*pDb).zTraceV2 = Tcl_Alloc(
                            (len_8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_uint,
                        );
                        memcpy(
                            (*pDb).zTraceV2 as *mut ::core::ffi::c_void,
                            zTraceV2 as *const ::core::ffi::c_void,
                            (len_8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                as size_t,
                        );
                    } else {
                        (*pDb).zTraceV2 = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    }
                    if !(*pDb).zTraceV2.is_null() {
                        (*pDb).interp = interp;
                        sqlite3_trace_v2(
                            (*pDb).db,
                            wMask as ::core::ffi::c_uint,
                            Some(
                                DbTraceV2Handler
                                    as unsafe extern "C" fn(
                                        ::core::ffi::c_uint,
                                        *mut ::core::ffi::c_void,
                                        *mut ::core::ffi::c_void,
                                        *mut ::core::ffi::c_void,
                                    ) -> ::core::ffi::c_int,
                            ),
                            pDb as *mut ::core::ffi::c_void,
                        );
                    } else {
                        sqlite3_trace_v2(
                            (*pDb).db,
                            0 as ::core::ffi::c_uint,
                            None,
                            ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        );
                    }
                }
            }
            37 => {
                let mut pScript_1: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
                let mut zBegin: *const ::core::ffi::c_char = b"SAVEPOINT _tcl_transaction\0"
                    .as_ptr() as *const ::core::ffi::c_char;
                if objc != 3 as ::core::ffi::c_int && objc != 4 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv as *const *mut Tcl_Obj,
                        b"[TYPE] SCRIPT\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                if (*pDb).nTransaction == 0 as ::core::ffi::c_int
                    && objc == 4 as ::core::ffi::c_int
                {
                    static mut TTYPE_strs_0: [*const ::core::ffi::c_char; 4] = [
                        b"deferred\0".as_ptr() as *const ::core::ffi::c_char,
                        b"exclusive\0".as_ptr() as *const ::core::ffi::c_char,
                        b"immediate\0".as_ptr() as *const ::core::ffi::c_char,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    ];
                    let mut ttype_0: ::core::ffi::c_int = 0;
                    if Tcl_GetIndexFromObjStruct(
                        interp,
                        *objv.offset(2 as ::core::ffi::c_int as isize),
                        &raw mut TTYPE_strs_0 as *mut *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<*mut ::core::ffi::c_char>()
                            as ::core::ffi::c_int,
                        b"transaction type\0".as_ptr() as *const ::core::ffi::c_char,
                        0 as ::core::ffi::c_int,
                        &raw mut ttype_0,
                    ) != 0
                    {
                        return TCL_ERROR;
                    }
                    match ttype_0 as TTYPE_enum as ::core::ffi::c_uint {
                        1 => {
                            zBegin = b"BEGIN EXCLUSIVE\0".as_ptr()
                                as *const ::core::ffi::c_char;
                        }
                        2 => {
                            zBegin = b"BEGIN IMMEDIATE\0".as_ptr()
                                as *const ::core::ffi::c_char;
                        }
                        0 | _ => {}
                    }
                }
                pScript_1 = *objv.offset((objc - 1 as ::core::ffi::c_int) as isize);
                (*pDb).disableAuth += 1;
                rc = sqlite3_exec(
                    (*pDb).db,
                    zBegin,
                    None,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                );
                (*pDb).disableAuth -= 1;
                if rc != SQLITE_OK {
                    Tcl_AppendResult(
                        interp,
                        sqlite3_errmsg((*pDb).db),
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    );
                    return TCL_ERROR;
                }
                (*pDb).nTransaction += 1;
                addDatabaseRef(pDb);
                if DbUseNre() != 0 {
                    Tcl_NRAddCallback(
                        interp,
                        Some(
                            DbTransPostCmd
                                as unsafe extern "C" fn(
                                    *mut ClientData,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                ) -> ::core::ffi::c_int,
                        ),
                        cd as ClientData,
                        ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    );
                    Tcl_NREvalObj(interp, pScript_1, 0 as ::core::ffi::c_int);
                } else {
                    rc = DbTransPostCmd(
                        &raw mut cd,
                        interp,
                        Tcl_EvalObjEx(interp, pScript_1, 0 as ::core::ffi::c_int),
                    );
                }
            }
            38 => {
                Tcl_AppendResult(
                    interp,
                    b"unlock_notify not available in this build\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                );
                rc = TCL_ERROR;
            }
            25 => {
                Tcl_AppendResult(
                    interp,
                    b"preupdate_hook was omitted at compile-time\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                );
                rc = TCL_ERROR;
            }
            41 | 39 | 30 => {
                let mut ppHook: *mut *mut Tcl_Obj = ::core::ptr::null_mut::<
                    *mut Tcl_Obj,
                >();
                if choice == DB_WAL_HOOK as ::core::ffi::c_int {
                    ppHook = &raw mut (*pDb).pWalHook;
                }
                if choice == DB_UPDATE_HOOK as ::core::ffi::c_int {
                    ppHook = &raw mut (*pDb).pUpdateHook;
                }
                if choice == DB_ROLLBACK_HOOK as ::core::ffi::c_int {
                    ppHook = &raw mut (*pDb).pRollbackHook;
                }
                if objc > 3 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv as *const *mut Tcl_Obj,
                        b"?SCRIPT?\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                DbHookCmd(
                    interp,
                    pDb,
                    if objc == 3 as ::core::ffi::c_int {
                        *objv.offset(2 as ::core::ffi::c_int as isize)
                    } else {
                        ::core::ptr::null_mut::<Tcl_Obj>()
                    },
                    ppHook,
                );
            }
            40 => {
                let mut i_4: ::core::ffi::c_int = 0;
                i_4 = 2 as ::core::ffi::c_int;
                if i_4 < objc {
                    let mut zArg: *const ::core::ffi::c_char = Tcl_GetString(
                        *objv.offset(i_4 as isize),
                    );
                    Tcl_AppendResult(
                        interp,
                        b"unknown argument: \0".as_ptr() as *const ::core::ffi::c_char,
                        zArg,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    );
                    return TCL_ERROR;
                }
                if i_4 == 2 as ::core::ffi::c_int {
                    Tcl_SetResult(
                        interp,
                        sqlite3_libversion() as *mut ::core::ffi::c_char,
                        TCL_STATIC,
                    );
                }
            }
            _ => {}
        }
        return rc;
    }
}
unsafe extern "C" fn DbObjCmdAdaptor(
    mut cd: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        return Tcl_NRCallObjProc(
            interp,
            Some(
                DbObjCmd
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            cd as ClientData,
            objc,
            objv as *const *mut Tcl_Obj,
        );
    }
}
unsafe extern "C" fn sqliteCmdUsage(
    mut interp: *mut Tcl_Interp,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        Tcl_WrongNumArgs(
            interp,
            1 as ::core::ffi::c_int,
            objv as *const *mut Tcl_Obj,
            b"HANDLE ?FILENAME? ?-vfs VFSNAME? ?-readonly BOOLEAN? ?-create BOOLEAN? ?-nofollow BOOLEAN? ?-nomutex BOOLEAN? ?-fullmutex BOOLEAN? ?-uri BOOLEAN?\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        return TCL_ERROR;
    }
}
unsafe extern "C" fn DbMain(
    mut cd: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut SqliteDb = ::core::ptr::null_mut::<SqliteDb>();
        let mut zArg: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zErrMsg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut i: ::core::ffi::c_int = 0;
        let mut zFile: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zVfs: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut flags: ::core::ffi::c_int = 0;
        let mut bTranslateFileName: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut translatedFilename: Tcl_DString = Tcl_DString {
            string: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            length: 0,
            spaceAvl: 0,
            staticSpace: [0; 200]

        };
        let mut rc: ::core::ffi::c_int = 0;
        flags = SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE | SQLITE_OPEN_NOMUTEX;
        if objc == 1 as ::core::ffi::c_int {
            return sqliteCmdUsage(interp, objv);
        }
        if objc == 2 as ::core::ffi::c_int {
            zArg = Tcl_GetStringFromObj(
                *objv.offset(1 as ::core::ffi::c_int as isize),
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
            if strcmp(zArg, b"-version\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                Tcl_AppendResult(
                    interp,
                    sqlite3_libversion(),
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                );
                return TCL_OK;
            }
            if strcmp(zArg, b"-sourceid\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                Tcl_AppendResult(
                    interp,
                    sqlite3_sourceid(),
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                );
                return TCL_OK;
            }
            if strcmp(zArg, b"-has-codec\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                Tcl_AppendResult(
                    interp,
                    b"0\0".as_ptr() as *const ::core::ffi::c_char,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                );
                return TCL_OK;
            }
            if *zArg.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '-' as i32
            {
                return sqliteCmdUsage(interp, objv);
            }
        }
        i = 2 as ::core::ffi::c_int;
        while i < objc {
            zArg = Tcl_GetString(*objv.offset(i as isize));
            if *zArg.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != '-' as i32
            {
                if !zFile.is_null() {
                    return sqliteCmdUsage(interp, objv);
                }
                zFile = zArg;
            } else {
                if i == objc - 1 as ::core::ffi::c_int {
                    return sqliteCmdUsage(interp, objv);
                }
                i += 1;
                if !(strcmp(zArg, b"-key\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int)
                {
                    if strcmp(zArg, b"-vfs\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        zVfs = Tcl_GetString(*objv.offset(i as isize));
                    } else if strcmp(
                        zArg,
                        b"-readonly\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        let mut b: ::core::ffi::c_int = 0;
                        if Tcl_GetBooleanFromObj(
                            interp,
                            *objv.offset(i as isize),
                            &raw mut b,
                        ) != 0
                        {
                            return TCL_ERROR;
                        }
                        if b != 0 {
                            flags &= !(SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE);
                            flags |= SQLITE_OPEN_READONLY;
                        } else {
                            flags &= !SQLITE_OPEN_READONLY;
                            flags |= SQLITE_OPEN_READWRITE;
                        }
                    } else if strcmp(
                        zArg,
                        b"-create\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        let mut b_0: ::core::ffi::c_int = 0;
                        if Tcl_GetBooleanFromObj(
                            interp,
                            *objv.offset(i as isize),
                            &raw mut b_0,
                        ) != 0
                        {
                            return TCL_ERROR;
                        }
                        if b_0 != 0
                            && flags & SQLITE_OPEN_READONLY == 0 as ::core::ffi::c_int
                        {
                            flags |= SQLITE_OPEN_CREATE;
                        } else {
                            flags &= !SQLITE_OPEN_CREATE;
                        }
                    } else if strcmp(
                        zArg,
                        b"-nofollow\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        let mut b_1: ::core::ffi::c_int = 0;
                        if Tcl_GetBooleanFromObj(
                            interp,
                            *objv.offset(i as isize),
                            &raw mut b_1,
                        ) != 0
                        {
                            return TCL_ERROR;
                        }
                        if b_1 != 0 {
                            flags |= SQLITE_OPEN_NOFOLLOW;
                        } else {
                            flags &= !SQLITE_OPEN_NOFOLLOW;
                        }
                    } else if strcmp(
                        zArg,
                        b"-nomutex\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        let mut b_2: ::core::ffi::c_int = 0;
                        if Tcl_GetBooleanFromObj(
                            interp,
                            *objv.offset(i as isize),
                            &raw mut b_2,
                        ) != 0
                        {
                            return TCL_ERROR;
                        }
                        if b_2 != 0 {
                            flags |= SQLITE_OPEN_NOMUTEX;
                            flags &= !SQLITE_OPEN_FULLMUTEX;
                        } else {
                            flags &= !SQLITE_OPEN_NOMUTEX;
                        }
                    } else if strcmp(
                        zArg,
                        b"-fullmutex\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        let mut b_3: ::core::ffi::c_int = 0;
                        if Tcl_GetBooleanFromObj(
                            interp,
                            *objv.offset(i as isize),
                            &raw mut b_3,
                        ) != 0
                        {
                            return TCL_ERROR;
                        }
                        if b_3 != 0 {
                            flags |= SQLITE_OPEN_FULLMUTEX;
                            flags &= !SQLITE_OPEN_NOMUTEX;
                        } else {
                            flags &= !SQLITE_OPEN_FULLMUTEX;
                        }
                    } else if strcmp(
                        zArg,
                        b"-uri\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        let mut b_4: ::core::ffi::c_int = 0;
                        if Tcl_GetBooleanFromObj(
                            interp,
                            *objv.offset(i as isize),
                            &raw mut b_4,
                        ) != 0
                        {
                            return TCL_ERROR;
                        }
                        if b_4 != 0 {
                            flags |= SQLITE_OPEN_URI;
                        } else {
                            flags &= !SQLITE_OPEN_URI;
                        }
                    } else if strcmp(
                        zArg,
                        b"-translatefilename\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        if Tcl_GetBooleanFromObj(
                            interp,
                            *objv.offset(i as isize),
                            &raw mut bTranslateFileName,
                        ) != 0
                        {
                            return TCL_ERROR;
                        }
                    } else {
                        Tcl_AppendResult(
                            interp,
                            b"unknown option: \0".as_ptr() as *const ::core::ffi::c_char,
                            zArg,
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        );
                        return TCL_ERROR;
                    }
                }
            }
            i += 1;
        }
        zErrMsg = ::core::ptr::null_mut::<::core::ffi::c_char>();
        p = Tcl_Alloc(::core::mem::size_of::<SqliteDb>() as ::core::ffi::c_uint)
            as *mut SqliteDb;
        memset(
            p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<SqliteDb>() as size_t,
        );
        if zFile.is_null() {
            zFile = b"\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if bTranslateFileName != 0 {
            zFile = Tcl_TranslateFileName(interp, zFile, &raw mut translatedFilename);
        }
        rc = sqlite3_open_v2(zFile, &raw mut (*p).db, flags, zVfs);
        if bTranslateFileName != 0 {
            Tcl_DStringFree(&raw mut translatedFilename);
        }
        if !(*p).db.is_null() {
            if SQLITE_OK != sqlite3_errcode((*p).db) {
                zErrMsg = sqlite3_mprintf(
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    sqlite3_errmsg((*p).db),
                );
                sqlite3_close((*p).db);
                (*p).db = ::core::ptr::null_mut::<sqlite3>();
            }
        } else {
            zErrMsg = sqlite3_mprintf(
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3_errstr(rc),
            );
        }
        if (*p).db.is_null() {
            Tcl_SetResult(
                interp,
                zErrMsg,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<Tcl_FreeProc>,
                >(1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            Tcl_Free(p as *mut ::core::ffi::c_char);
            sqlite3_free(zErrMsg as *mut ::core::ffi::c_void);
            return TCL_ERROR;
        }
        (*p).maxStmt = NUM_PREPARED_STMTS;
        (*p).openFlags = flags & SQLITE_OPEN_URI;
        (*p).interp = interp;
        zArg = Tcl_GetStringFromObj(
            *objv.offset(1 as ::core::ffi::c_int as isize),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if DbUseNre() != 0 {
            Tcl_NRCreateCommand(
                interp,
                zArg,
                Some(
                    DbObjCmdAdaptor
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *const *mut Tcl_Obj,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    DbObjCmd
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *const *mut Tcl_Obj,
                        ) -> ::core::ffi::c_int,
                ),
                p as *mut ::core::ffi::c_char as ClientData,
                Some(DbDeleteCmd as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
            );
        } else {
            Tcl_CreateObjCommand(
                interp,
                zArg,
                Some(
                    DbObjCmd
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *const *mut Tcl_Obj,
                        ) -> ::core::ffi::c_int,
                ),
                p as *mut ::core::ffi::c_char as ClientData,
                Some(DbDeleteCmd as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
            );
        }
        (*p).nRef = 1 as ::core::ffi::c_int;
        return TCL_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Sqlite3_Init(
    mut interp: *mut Tcl_Interp,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = if !(b"8.6\0".as_ptr()
            as *const ::core::ffi::c_char)
            .is_null()
        {
            TCL_OK
        } else {
            TCL_ERROR
        };
        if rc == TCL_OK {
            Tcl_CreateObjCommand(
                interp,
                b"sqlite3\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *const *mut Tcl_Obj,
                        ) -> ::core::ffi::c_int,
                    >,
                    Option<Tcl_ObjCmdProc>,
                >(
                    Some(
                        DbMain
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                ),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                None,
            );
            Tcl_CreateObjCommand(
                interp,
                b"sqlite\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *const *mut Tcl_Obj,
                        ) -> ::core::ffi::c_int,
                    >,
                    Option<Tcl_ObjCmdProc>,
                >(
                    Some(
                        DbMain
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                ),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                None,
            );
            rc = Tcl_PkgProvideEx(
                interp,
                b"sqlite3\0".as_ptr() as *const ::core::ffi::c_char,
                b"3.51.2\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_void>(),
            );
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Tclsqlite3_Init(
    mut interp: *mut Tcl_Interp,
) -> ::core::ffi::c_int {
    unsafe {
        return Sqlite3_Init(interp);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Sqlite3_Unload(
    mut interp: *mut Tcl_Interp,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return TCL_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Tclsqlite3_Unload(
    mut interp: *mut Tcl_Interp,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return TCL_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Sqlite3_SafeInit(
    mut interp: *mut Tcl_Interp,
) -> ::core::ffi::c_int {
    unsafe {
        return TCL_ERROR;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Sqlite3_SafeUnload(
    mut interp: *mut Tcl_Interp,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return TCL_ERROR;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Sqlite_Init(mut interp: *mut Tcl_Interp) -> ::core::ffi::c_int {
    unsafe {
        return Sqlite3_Init(interp);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Tclsqlite_Init(
    mut interp: *mut Tcl_Interp,
) -> ::core::ffi::c_int {
    unsafe {
        return Sqlite3_Init(interp);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Sqlite_Unload(
    mut interp: *mut Tcl_Interp,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return TCL_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Tclsqlite_Unload(
    mut interp: *mut Tcl_Interp,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return TCL_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Sqlite_SafeInit(
    mut interp: *mut Tcl_Interp,
) -> ::core::ffi::c_int {
    unsafe {
        return TCL_ERROR;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Sqlite_SafeUnload(
    mut interp: *mut Tcl_Interp,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return TCL_ERROR;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_Init(
    mut interp: *mut Tcl_Interp,
) -> ::core::ffi::c_int {
    unsafe {
        return Sqlite3_Init(interp);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite_Init(mut interp: *mut Tcl_Interp) -> ::core::ffi::c_int {
    unsafe {
        return Sqlite3_Init(interp);
    }
}
