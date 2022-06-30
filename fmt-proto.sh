#!/usr/bin/env bash

####################################################
# Script name : fmt-proto.sh
# Description : Format omnipf proto files.
# How to : $ ./fmt-proto.sh
####################################################

set -euo pipefail

CLANG_FORMAT_STYLE="{BasedOnStyle: Google, AlignConsecutiveDeclarations: true, AlignConsecutiveAssignments: true, ColumnLimit: 0, IndentWidth: 4}"

find ./proto/ -type f -name "*.proto" | xargs clang-format -i -style="$(echo ${CLANG_FORMAT_STYLE})"
