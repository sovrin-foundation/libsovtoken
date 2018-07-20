#!/bin/sh

function abspath() {
    perl -e 'use Cwd "abs_path"; print abs_path(shift)' $1
}

function get_sovtoken_dir() {
    TMP=$(abspath "${PWD}/../../..")
    echo ${TMP}
}
function get_work_dir() {
    TMP=$(get_sovtoken_dir)
    TMP="${TMP}/.macosbuild"
    echo ${TMP}
}
