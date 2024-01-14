#!/usr/bin/env bash
#shellchek=bash

set -euo pipefail

echo "Load log4sh library..."

# Print stack trace, implemented by built-in `caller`.
function stack_trace() {
  printf 'error: %s\n' "$1" >&2
  local i=0
  while true; do
    s=$(caller $i)
    if [ -z "$s" ]; then
      break
    fi
    # shellcheck disable=SC2068
    printf '  at %s:%s (File: %s)\n' ${s[@]};
    ((i++))
  done
}

# Print stack trace, and throw error.
function throw() {
  stack_trace "$1" >&2
  exit 1
}

function __log() {
  local level="$1"
  shift 1
  echo -e "[${level}] $*"
}

function log_trace() {
  __log 'TRACE' "$@"
}

function log_debug() {
  __log 'DEBUG' "$@"
}

function log_info() {
  __log 'INFO' "$@"
}

function log_warn() {
  __log 'WARN' "$@"
}


function log_error() {
  __log 'ERROR' "$@"
}
