#!/usr/bin/env bash

ROOT="tests/inputs"
OUTDIR="tests/expected"

[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

# Cf https://github.com/coreutils/coreutils/blob/master/tests/misc/uniq.pl
echo -ne "a\na\n"    > $ROOT/t1.txt
echo -ne "a\na"      > $ROOT/t2.txt
echo -ne "a\nb"      > $ROOT/t3.txt
echo -ne "a\na\nb"   > $ROOT/t4.txt
echo -ne "b\na\na\n" > $ROOT/t5.txt
echo -ne "a\nb\nc\n" > $ROOT/t6.txt

for FILE in $ROOT/*.txt; do
    BASENAME=$(basename "$FILE")
    uniq      $FILE > ${OUTDIR}/${BASENAME}.out
    uniq -c   $FILE > ${OUTDIR}/${BASENAME}.c.out
    uniq    < $FILE > ${OUTDIR}/${BASENAME}.stdin.out
    uniq -c < $FILE > ${OUTDIR}/${BASENAME}.stdin.c.out
done
