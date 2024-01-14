#!/usr/bin/env bash

set -euo pipefail

CURRENT_DIR="$(dirname "${0:-BASHSOURCE[0]}")"

source "${CURRENT_DIR}/../lib/log.sh"

if ! command -v kind 2>/dev/null; then
  throw "Command \`kind' not found"
fi

kind load docker-image harbor.leryn.top/infra/cmdb-agent:0.1.0

kind get kubeconfig --name=kind > ~/.kube/kind
export KUBECONFIG=~/.kube/kind

find deploy/raw   -name '*.yaml' -exec kubectl apply -f {} \;
find deploy/debug -name '*.yaml' -exec kubectl apply -f {} \;

unset KUBECONFIG

log_warn "Use \`kind' or \`k9s' to inspect the pod state."
