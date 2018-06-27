# FPM ARGUMENTS
_FPM_ARGS := $(FPM_ARGS)
FPM_ARGS := 

FPM_P_INPUT_TYPE ?= dir
FPM_ARGS += --input-type $(FPM_P_INPUT_TYPE)

ifeq ($(OSNAME),xenial)
FPM_P_OUTPUT_TYPE ?= deb
else ifeq ($(OSNAME),centos7)
FPM_P_OUTPUT_TYPE ?= rpm
endif
FPM_ARGS += --output-type $(FPM_P_OUTPUT_TYPE)

ifdef FPM_P_OUTPUT_DIR
FPM_ARGS += --package $(FPM_P_OUTPUT_DIR)
endif

ifdef FPM_P_NAME
FPM_ARGS += --name $(FPM_P_NAME)
endif

ifdef FPM_P_VERSION
FPM_ARGS += --version $(FPM_P_VERSION)
endif

ifdef FPM_P_DEPENDS
FPM_ARGS += $(patsubst %,--depends "%", $(FPM_P_DEPENDS))
endif

ifdef FPM_P_MAINTAINER
FPM_ARGS += --maintainer "$(FPM_P_MAINTAINER)"
endif

ifdef FPM_P_URL
FPM_ARGS += --url $(FPM_P_URL)
endif

ifdef FPM_P_LICENSE
FPM_ARGS += --license "$(FPM_P_LICENSE)"
endif

ifdef FPM_P_VENDOR
FPM_ARGS += --vendor "$(FPM_P_VENDOR)"
endif

ifdef FPM_P_DESCRIPTION
FPM_ARGS += --description "$(FPM_P_DESCRIPTION)"
endif

ifdef FPM_P_POSTINSTALL
FPM_ARGS += --after-install $(FPM_P_POSTINSTALL)
endif

ifdef FPM_P_POSTUNINSTALL
FPM_ARGS += --after-remove $(FPM_P_POSTUNINSTALL)
endif

ifdef FPM_P_PREINSTALL
FPM_ARGS += --before-install $(FPM_P_PREINSTALL)
endif

ifdef FPM_P_PREUNINSTALL
FPM_ARGS += --before-remove $(FPM_P_PREUNINSTALL)
endif

FPM_VERBOSE ?= --verbose
FPM_ARGS += $(FPM_VERBOSE)

FPM_ARGS += $(_FPM_ARGS)
