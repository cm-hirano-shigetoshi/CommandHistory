#!/usr/bin/env bash
set -euo pipefail

cut -c26- | rg -F "\"${PWD}\"" | sed 's/^[^\t]\+\t//' | awk '!a[$0]++' | rg -v '^ *$' | bat --language bash --color always --plain
