#!/usr/bin/env bash
set -e

SRC="/sqlite"
PROJ=/workspace

# Parse optional function names as arguments
EXTRA_SYMBOLS=()
for arg in "$@"; do
    EXTRA_SYMBOLS+=("$arg")
done

# Append extra symbols to a temp copy of sqlite3_symbols.txt
SYMBOLS_FILE="$PROJ/scripts/sqlite3_symbols.txt"
if [ ${#EXTRA_SYMBOLS[@]} -gt 0 ]; then
    SYMBOLS_FILE="$PROJ/target/sqlite3_symbols_tmp.txt"
    cp "$PROJ/scripts/sqlite3_symbols.txt" "$SYMBOLS_FILE"
    printf "%s\n" "${EXTRA_SYMBOLS[@]}" >> "$SYMBOLS_FILE"
    echo "→ Appended ${#EXTRA_SYMBOLS[@]} symbol(s) to temporary symbols file"
fi


INCLUDES=(
  -I"$SRC"
  -I"$SRC/src"
  -I"$SRC/ext/rtree"
  -I"$SRC/ext/icu"
  -I"$SRC/ext/fts3"
  -I"$SRC/ext/session"
  -I"$SRC/ext/misc"
)

mapfile -t FLAGS < <(sed 's/\r//' "$PROJ/scripts/defines_testfixture.txt" | grep -v '^$')

# Step 3: Compile rustfixture linking patched sqlite3 object + our Rust lib + TCL
TCL_CFLAGS=$(pkg-config --cflags tcl)
TCL_LIBS=$(pkg-config --libs tcl)

chmod +x $SRC/.tclenv.sh
. $SRC/.tclenv.sh || exit $?

cc -fPIC -O2 -g \
  "${FLAGS[@]}" \
  "${INCLUDES[@]}" \
  $TCL_CFLAGS \
  -DTCLSH_INIT_PROC=sqlite3TestInit \
  -o "$SRC/rustfixture" \
  "$SRC/src/test1.c"         "$SRC/src/test2.c"         "$SRC/src/test3.c" \
  "$SRC/src/test4.c"         "$SRC/src/test5.c"         "$SRC/src/test6.c" \
  "$SRC/src/test8.c"         "$SRC/src/test9.c" \
  "$SRC/src/test_autoext.c"  "$SRC/src/test_backup.c"   "$SRC/src/test_bestindex.c" \
  "$SRC/src/test_blob.c"     "$SRC/src/test_btree.c"    "$SRC/src/test_config.c" \
  "$SRC/src/test_delete.c"   "$SRC/src/test_demovfs.c"  "$SRC/src/test_devsym.c" \
  "$SRC/src/test_fs.c"       "$SRC/src/test_func.c"     "$SRC/src/test_hexio.c" \
  "$SRC/src/test_init.c"     "$SRC/src/test_intarray.c" "$SRC/src/test_journal.c" \
  "$SRC/src/test_malloc.c"   "$SRC/src/test_md5.c"      "$SRC/src/test_multiplex.c" \
  "$SRC/src/test_mutex.c"    "$SRC/src/test_onefile.c"  "$SRC/src/test_osinst.c" \
  "$SRC/src/test_pcache.c"   "$SRC/src/test_quota.c"    "$SRC/src/test_rtree.c" \
  "$SRC/src/test_schema.c"   "$SRC/src/test_superlock.c" "$SRC/src/test_syscall.c" \
  "$SRC/src/test_tclsh.c"    "$SRC/src/test_tclvar.c"   "$SRC/src/test_thread.c" \
  "$SRC/src/test_vdbecov.c"  "$SRC/src/test_vfs.c"      "$SRC/src/test_window.c" \
  "$SRC/src/test_wsd.c" \
  "$SRC/ext/fts3/fts3_term.c"       "$SRC/ext/fts3/fts3_test.c" \
  "$SRC/ext/session/test_session.c" \
  "$SRC/ext/recover/sqlite3recover.c" "$SRC/ext/recover/dbdata.c" "$SRC/ext/recover/test_recover.c" \
  "$SRC/ext/intck/test_intck.c"     "$SRC/ext/intck/sqlite3intck.c" \
  "$SRC/ext/rbu/test_rbu.c" \
  "$SRC/ext/expert/sqlite3expert.c" "$SRC/ext/expert/test_expert.c" \
  "$SRC/ext/misc/amatch.c"     "$SRC/ext/misc/appendvfs.c" "$SRC/ext/misc/basexx.c" \
  "$SRC/ext/misc/cksumvfs.c"   "$SRC/ext/misc/closure.c"   "$SRC/ext/misc/csv.c" \
  "$SRC/ext/misc/decimal.c"    "$SRC/ext/misc/eval.c"       "$SRC/ext/misc/explain.c" \
  "$SRC/ext/misc/fileio.c"     "$SRC/ext/misc/fuzzer.c" \
  "$SRC/ext/fts5/fts5_tcl.c"   "$SRC/ext/fts5/fts5_test_mi.c" "$SRC/ext/fts5/fts5_test_tok.c" \
  "$SRC/ext/misc/ieee754.c"    "$SRC/ext/misc/mmapwarm.c"  "$SRC/ext/misc/nextchar.c" \
  "$SRC/ext/misc/normalize.c"  "$SRC/ext/misc/prefixes.c"  "$SRC/ext/misc/qpvtab.c" \
  "$SRC/ext/misc/randomjson.c" "$SRC/ext/misc/regexp.c"    "$SRC/ext/misc/remember.c" \
  "$SRC/ext/misc/series.c"     "$SRC/ext/misc/spellfix.c"  "$SRC/ext/misc/stmtrand.c" \
  "$SRC/ext/misc/totype.c"     "$SRC/ext/misc/unionvtab.c" "$SRC/ext/misc/wholenumber.c" \
  "$SRC/ext/misc/zipfile.c" \
  "$SRC/ext/rtree/test_rtreedoc.c" \
  "$SRC/src/tclsqlite.c" \
  -L"$PROJ/sqlite-testfixture/release" -lsqlite_noamalgam \
  -Wl,-rpath,"$PROJ/sqlite-testfixture/release" \
  -Wl,--allow-multiple-definition \
  $TCL_LIB_SPEC $TCL_INCLUDE_SPEC \
  $TCL_LIBS \
  -lm -lz -lc

while IFS= read -r sym; do
  objcopy --strip-symbol __c_${sym} "$SRC/rustfixture"
done < "$SYMBOLS_FILE"

while IFS= read -r sym; do
  objcopy --strip-symbol ${sym} "$SRC/rustfixture"
done < "$SYMBOLS_FILE"
