SHELL := /bin/bash

# TODO
# 	- more fpm arguments
#	- package-static with libname.a

OSNAME ?= xenial

ifndef PROJECT_DIR
PROJECT_DIR := $(shell git rev-parse --show-toplevel)
endif

# TODO make it as Simply expanded variables (":=")
PROJECT_NAME ?= $(notdir $(PROJECT_DIR))
PRE_PACKAGE_GOALS ?=

# based on https://stackoverflow.com/questions/10858261/abort-makefile-if-variable-not-set
check_non_empty = $(if $(1),$(1),$(error $(2)))
check_defined = $(if $(value $1),, $(error Undefined $1$(if $2,	($2))$(if $(value @), \
                    required by target '$@')))

docker_env_value = $(call check_non_empty,$(shell grep '^ENV $(1)=' $(2) | cut -d '=' -f 2), \
				   "Value for ENV $(1) not found in dockerfile $(2)")
docker_from_tag = $(call check_non_empty,$(shell grep '^FROM $(1):' $(2) | cut -d ':' -f 2), \
					"Tag for FROM $(1) not found in dockerfile $(2)")
docker_from_version = $(firstword $(subst -, ,$(call docker_from_tag,$(1),$(2))))

# TODO DRY refactor: make common callable logic
BASE_DOCKERFILE_PATH = $(_THIS_MK_ABS_DIR)/docker/base/$(OSNAME)/Dockerfile.$(BASE_DOCKER_VERSION)
RUST_DOCKERFILE_PATH = $(_THIS_MK_ABS_DIR)/docker/rust/$(OSNAME)/Dockerfile.$(RUST_DOCKER_VERSION)
PYTHON3_DOCKERFILE_PATH = $(_THIS_MK_ABS_DIR)/docker/python3/$(OSNAME)/Dockerfile.$(PYTHON3_DOCKER_VERSION)

# required for in-docker targets
DOCKER_NAME ?=
DOCKER_TAG ?=

# TODO consider to use GID as well if necessary
ifndef DOCKER_UID
DOCKER_UID := $(shell id -u)
endif

# SELF POSITIONING
ROOT_MK_REL_DIR := $(subst $(PROJECT_DIR),.,$(CURDIR))

# is unlikely to be an absolute one
ROOT_MK_REL_PATH := $(word 1,$(MAKEFILE_LIST))
_THIS_MK_ABS_PATH := $(word $(words $(MAKEFILE_LIST)),$(MAKEFILE_LIST))
#	absolute include/read
ifneq ($(wildcard $(abspath $(_THIS_MK_ABS_PATH))),)
_THIS_MK_ABS_PATH := $(abspath $(_THIS_MK_ABS_PATH))
#	relative include/read
else ifneq ($(wildcard $(CURDIR)/$(_THIS_MK_ABS_PATH)),)
_THIS_MK_ABS_PATH := $(CURDIR)/$(_THIS_MK_ABS_PATH))
else
$(error "Failed to determine abspath to this makefile used as '$(_THIS_MK_ABS_PATH)'")
endif
_THIS_MK_ABS_DIR := $(dir $(_THIS_MK_ABS_PATH))
include $(_THIS_MK_ABS_DIR)/fpm.mk

CP = cp
DOCKER = docker
DOCKER_COMPOSE = docker-compose
FPM = fpm

.PHONY: _check_fpm_args package check_dockerfile clean_docker clean_build _default_docker_image_target

# list of conditionally (using '?=') defined variables that
# should be passed to in-docker re-run
EXPORT_ENV += OSNAME PROJECT_NAME

export $(EXPORT_ENV)

# PACKAGING ROUTINE
_check_fpm_args:
ifeq ($(FPM_P_INPUT_TYPE),dir)

ifeq ($(FPM_P_NAME),)
	$(error Package name is not specified)
endif
ifeq ($(FPM_P_VERSION),)
	$(error Package version is not specified)
endif

endif


# TODO deps with versions like "package (>= 1.0)"
package: _check_fpm_args $(PRE_PACKAGE_GOALS)
	$(FPM) $(FPM_ARGS)

image_rust_base_version:
	$(call check_defined,RUST_DOCKER_VERSION)
	$(eval BASE_DOCKER_VERSION=$(call docker_from_version,'evernym/base',$(RUST_DOCKERFILE_PATH)))
	echo $(BASE_DOCKER_VERSION)

image_python3_base_version:
	$(call check_defined,PYTHON3_DOCKER_VERSION)
	$(eval BASE_DOCKER_VERSION=$(call docker_from_version,'evernym/base',$(PYTHON3_DOCKERFILE_PATH)))
	echo $(BASE_DOCKER_VERSION)

# TODO refactor that once compose provides support for images
# build order: https://github.com/docker/compose/issues/5228
image_base:
	$(call check_defined,BASE_DOCKER_VERSION)
	BASE_DOCKER_VERSION=$(BASE_DOCKER_VERSION) \
		docker-compose -f $(_THIS_MK_ABS_DIR)/docker/docker-compose.yml build base

image_rust: image_rust_base_version image_base
	RUST_DOCKER_VERSION=$(RUST_DOCKER_VERSION) \
		docker-compose -f $(_THIS_MK_ABS_DIR)/docker/docker-compose.yml build rust

image_python3: image_python3_base_version image_base
	PYTHON3_DOCKER_VERSION=$(PYTHON3_DOCKER_VERSION) \
		docker-compose -f $(_THIS_MK_ABS_DIR)/docker/docker-compose.yml build python3

%_in_docker: image_%
	echo "Re-running goal $* in docker $(DOCKER_NAME):$(DOCKER_TAG)"
	$(DOCKER) run --rm -t -u $(DOCKER_UID) \
		-v $(PROJECT_DIR):/tmp/$(PROJECT_NAME) \
		$(DOCKER_NAME):$(DOCKER_TAG) \
		/bin/bash -cl \
			'MAKEFLAGS="$(MAKEFLAGS)" $(foreach var,$(EXPORT_ENV),$(var)="$($(var))") \
			$(MAKE) -C /tmp/$(PROJECT_NAME)/$(ROOT_MK_REL_DIR) -f $(ROOT_MK_REL_PATH) $*'

# CLEAN ROUTINE
# TODO update, consider to use docker-compose down --rmi
#clean_docker:
#	$(DOCKER) images -q $(DOCKER_NS)/$(PROJECT_NAME)-* | xargs -r $(DOCKER) rmi -f
#	-rm -f $(patsubst %,$(BUILD_DIR)/%, $(IMAGES))

#clean_build:
#	rm -rf $(BUILD_DIR)
