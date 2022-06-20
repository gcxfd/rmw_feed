#!/usr/bin/env bash

DIR=$(dirname $(realpath "$0"))
cd $DIR
set -ex

out=li.txt
qshell listbucket2 xvc-com -o $out
qshell batchdelete --force xvc-com -i $out
rm -rf $out .id .uploaded/
