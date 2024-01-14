#!/usr/bin/env bash

set -euo pipefail

function _main() {
  if [ "${1:0:1}" = '-' ]; then
    set -- cmdb-agent "$@"
  fi

  if command -v "$1" 2>/dev/null; then
    exec "$@"
  fi

  exec "$@"
}

_main "$@"
