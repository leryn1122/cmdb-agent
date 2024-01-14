# Project
SHELL := /bin/bash
PROJECT := cmdb-agent
VERSION := 0.1.0
BUILD_DATE := $(shell date +%Y%m%d)
GIT_VERSION := $(shell git describe --long --all 2>/dev/null)
SHA := $(shell git rev-parse --short=8 HEAD 2>/dev/null)

# Toolchain
CARGO := cargo
SUDO_CARGO := sudo -E cargo

# Main


# Docker
DOCKER := docker
DOCKER_CONTEXT := .
DOCKERFILE := ci/docker/Dockerfile
REGISTRY := harbor.leryn.top
IMAGE_NAME := infra/$(PROJECT)
FULL_IMAGE_NAME = $(REGISTRY)/$(IMAGE_NAME):$(VERSION)

##@ General

.PHONY: help
help: ## Print help info
	@ awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_0-9-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

##@ Development

.PHONY: install
install: ## Install dependencies
	$(CARGO) fetch

.PHONY: check
check: ## Check
	$(CARGO) clippy

.PHONY: format
format: ## Format against code
	$(CARGO) fmt

.PHONY: clean
clean: ## Clean target artifact
	$(CARGO) clean

.PHONY: unittest
unittest: ## Run all unit tests
	@ $(CARGO) test \
	  -- \
	  --color always \
	  --show-output \
	  --nocapture

.PHONY: test
test: ## Run all integrity tests
	@ $(SHELL) scripts/debug/run-kind-sample.sh

##@ Build
.PHONY: build
build: ## Run the target artifact
	$(CARGO) build --release

.PHONY: run
run: ## Run local demo
	$(CARGO) run -- \
	  --log-level=TRACE \
	  --config-file=etc/cmdb/agent.toml

.PHONY: image
image: ## Build the OCI image
	which docker
	DOCKER_BUILDKIT=1 $(DOCKER) build \
	  -t $(FULL_IMAGE_NAME) \
	  -f $(DOCKERFILE) \
	  $(DOCKER_CONTEXT)
