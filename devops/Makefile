#TODO
# - docker-compose re-runs build always (it's fast because of cache but anyway)

RELEASE ?= 1
OSNAME ?= xenial

ifeq ($(RELEASE),1)
CARGO_FLAGS = --release
BUILD_TYPE = release
else
BUILD_TYPE = debug
endif

ifndef PROJECT_DIR
PROJECT_DIR := $(shell git rev-parse --show-toplevel)
endif

# project variables
SRC_DIR_NAME := libsovtoken
SRC_DIR := $(PROJECT_DIR)/$(SRC_DIR_NAME)
# TODO
#  - fail make run if the folloing fails
#  - src version is not needed for all targets
SRC_VERSION := $(shell grep '^version = "[^"]\+"' $(SRC_DIR)/Cargo.toml | cut -d '"' -f 2)

# cargo related variables
CARGO_TARGET_DIR ?= target/$(OSNAME)
CRATE_P_VERSION ?= $(SRC_VERSION)
CARGO_LOGIN_TOKEN ?=

# libs related variables
LIBNAME := libsovtoken
LIB_TARGET_DIR = $(SRC_DIR)/$(CARGO_TARGET_DIR)/$(BUILD_TYPE)
LIB_STATIC = $(LIB_TARGET_DIR)/$(LIBNAME).a
LIB_DYNAMIC = $(LIB_TARGET_DIR)/$(LIBNAME).so
LIBS = $(LIB_DYNAMIC) $(LIB_STATIC)

# package related variables
PRE_PACKAGE_GOALS := pre_package $(LIB_DYNAMIC)
FPM_P_MAINTAINER := Evernym
FPM_P_URL := https://github.com/evernym/libsovtoken
FPM_P_LICENSE := Apache License 2.0
FPM_P_VENDOR := Evernym
FPM_P_DESCRIPTION := libsovtoken writen in Rust
FPM_P_NAME = $(LIBNAME)
FPM_P_VERSION ?= $(SRC_VERSION)
FPM_P_DEPENDS = libindy
FPM_P_OUTPUT_DIR = $(LIB_TARGET_DIR)
FPM_ARGS = $(LIB_DYNAMIC)=/usr/lib/

# docker related variables
DOCKER_NAME ?= evernym/libsovtoken
CI_DOCKER_TAG ?=
LSTBASE_DOCKER_TAG ?=

LSTBASE_DOCKERFILE_PATH = docker/base/$(OSNAME)/Dockerfile
CI_DOCKERFILE_PATH = docker/ci/$(OSNAME)/Dockerfile

DOCKER_BUILD_NETWORK ?= host

EXPORT_ENV += OSNAME RELEASE CARGO_TARGET_DIR CRATE_P_VERSION CARGO_LOGIN_TOKEN

.PHONY: all \
		image_lstbase_parent_version image_lstbase_version image_lstbase \
		image_ci_parent_version image_ci_version image_ci \
		rust_version test_dry test build pre_package publish_crate clean


all: test_dry test package

include ext/Makefile

# log variables
ifdef DEBUG
$(call log_var,RELEASE)
$(call log_var,OSNAME)
$(call log_var,PROJECT_DIR)
$(call log_var,CARGO_TARGET_DIR)
$(call log_var,CRATE_P_VERSION)
$(call log_var,FPM_P_VERSION)
$(call log_var,DOCKER_NAME)
$(call log_var,CI_DOCKER_TAG)
$(call log_var,LSTBASE_DOCKER_TAG)
$(call log_var,DOCKER_BUILD_NETWORK)
endif

export OSNAME


##########################
# libsovtoken base image #
# ###################### #

image_lstbase_parent_version:
	$(eval RUST_DOCKER_VERSION=$(call docker_from_version,'evernym/rust', \
		$(LSTBASE_DOCKERFILE_PATH)))
	echo $(RUST_DOCKER_VERSION)

image_lstbase_version:
	echo $(call docker_env_value,'LIBSOVTOKEN_BASE_ENV_VERSION',$(LSTBASE_DOCKERFILE_PATH))

docker/base/$(OSNAME)/Cargo.toml: $(SRC_DIR)/Cargo.toml docker/base/$(OSNAME)
	$(CP) -f $< $@

image_lstbase: image_lstbase_parent_version image_rust docker/base/$(OSNAME)/Cargo.toml
	$(eval DOCKER_TAG=$(if $(LSTBASE_DOCKER_TAG),$(LSTBASE_DOCKER_TAG), \
		$(call docker_env_value,'LIBSOVTOKEN_BASE_ENV_VERSION', \
			$(LSTBASE_DOCKERFILE_PATH))-$(OSNAME)-base))
	OSNAME=$(OSNAME) \
	DOCKER_BUILD_NETWORK=$(DOCKER_BUILD_NETWORK) \
	DOCKER_UID=$(DOCKER_UID) \
	LSTBASE_DOCKER_NAME=$(DOCKER_NAME) \
	LSTBASE_DOCKER_TAG=$(DOCKER_TAG) \
		docker-compose -f docker/docker-compose.yml build base

##########################
# libsovtoken ci image #
# ###################### #

image_ci_parent_version:
	$(eval LSTBASE_DOCKER_TAG=$(call docker_from_tag,'evernym/libsovtoken',$(CI_DOCKERFILE_PATH)))
	echo $(LSTBASE_DOCKER_TAG)

image_ci_version:
	echo $(call docker_env_value,'LIBSOVTOKEN_CI_ENV_VERSION',$(CI_DOCKERFILE_PATH))

image_ci: image_ci_parent_version image_lstbase
	$(eval DOCKER_TAG=$(if $(CI_DOCKER_TAG),$(CI_DOCKER_TAG), \
		$(call docker_env_value,'LIBSOVTOKEN_CI_ENV_VERSION', \
			$(CI_DOCKERFILE_PATH))-$(OSNAME)-ci))
	OSNAME=$(OSNAME) \
	DOCKER_BUILD_NETWORK=$(DOCKER_BUILD_NETWORK) \
	CI_DOCKER_NAME=$(DOCKER_NAME) \
	CI_DOCKER_TAG=$(DOCKER_TAG) \
		docker-compose -f docker/docker-compose.yml build ci

######

LSTBASE_IMAGE_BASED_TARGETS = build $(LIBS) package publish_crate clean
$(patsubst %,image_%, $(LSTBASE_IMAGE_BASED_TARGETS)): image_lstbase

CI_IMAGE_BASED_TARGETS = test_dry test
$(patsubst %,image_%, $(CI_IMAGE_BASED_TARGETS)): image_ci

######

rust_version:
	cargo --version
	rustc --version

test_dry: $(SRC_DIR) rust_version
	echo "Running tests in dry mode (compile, but don't run tests)"
	cd $< && cargo clean && cargo update && RUST_TEST_THREADS=1 RUST_BACKTRACE=1 cargo test $(CARGO_FLAGS) --no-run

test: $(SRC_DIR) rust_version
	echo "Running tests"
	cd $< && cargo clean && cargo update && RUST_TEST_THREADS=1 RUST_BACKTRACE=1 RUST_LOG=trace cargo test $(CARGO_FLAGS)

build $(LIBS): $(SRC_DIR) rust_version
	echo "Compiling the project"
	cd $< && cargo clean && cargo update && cargo build $(CARGO_FLAGS)

pre_package: $(LIB_DYNAMIC)
	rm -f $(LIB_TARGET_DIR)/$(LIBNAME)*$(SRC_VERSION)*.$(FPM_P_OUTPUT_TYPE)

publish_crate: $(SRC_DIR) rust_version
	echo "Publishing crate"
ifneq ($(CARGO_LOGIN_TOKEN),)
	cargo login $(CARGO_LOGIN_TOKEN)
else
	$(warning CARGO_LOGIN_TOKEN is not defined, skip login)
endif
ifneq ($(CRATE_P_VERSION),)
	sed -i 's/^version = ".*"$$/version = "$(CRATE_P_VERSION)"/' $</Cargo.toml
endif
	cd $< && cargo package --allow-dirty
	cd $< && cargo publish --allow-dirty

clean: $(SRC_DIR)
	cd $< && cargo clean
