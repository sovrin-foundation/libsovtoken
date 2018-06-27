# DevOps tasks automation

This folder provides makefile based API to help with devops tasks automation. Tasks are defined as make targets and could be run either on host or in docker containers. The folder includes the following files / dirs:
- [Makefile](Makefile) defines pattern rule `%_in_docker` to run any target inside docker container and provides a set of other generally useful targets
- [docker](docker) folder holds docker related routine

## Requirements

- make
- docker
- docker-compose

## Docker

This module provides a set of dockerfiles based on ubuntu `xenial` and `centos7`.

They have an hierarchy: `rust` images based on `base` ones.

Each version of these images is represented as separate dockerfile named `Dockerfile.<VERSION>`.

### **base** images

`base` docker images include generally useful packages and `fpm` along with `rvm` and `ruby`.

Notes:
  - consider to use shell in login mode if you need `fpm` (`rvm` and `ruby`) available in the `$PATH`
  - consider to add all non-root users that will be using rvm to `rvm` group

Environment variables:
  - `BASE_ENV_VERSION` version of the dockerfile
  - `FPM_VERSION` version for the `fpm`
  - (`centos7` only) `RUBY_VERSION` version of the `ruby`

### **rust** images

`rust` docker images are based on the `base` images and adds `rust`.

Supported arguments:
  - `RUST_VERSION`: version of the `rust` to install. Default: depends on dockerfile version

  Environment variables:
    - `RUST_ENV_VERSION` version of the dockerfile
    - `RUST_VERSION` version of the `rust`

## Makefile

### Targets

- `%_in_docker` re-runs make for the matched target `%` inside `$(DOCKER_NAME):$(DOCKER_TAG)` docker image. Requires target `image_%` to be defined and expects that it builds necessary docker image. Environment variables `DOCKER_NAME` and `DOCKER_TAG` should be defined as well
- `package` creates a package from the source code using [fpm][a1feb9f1] tool. Could be configured by `FPM_*` environment variables
- `image_base` builds docker image with generally useful packages and `fpm` installed
- `image_rust` builds docker image with `rust` installed

  [a1feb9f1]: https://github.com/jordansissel/fpm "fpm"

Expects the following targets to be defined in child makefiles:
- `image_%`: should build image with necessary environment to execute target `%`

### Environment variables

- `OSNAME`: switches OS context, possible values: `xenial`, `centos7`. Default: `xenial`
- `EXPORT_ENV`: list of variables that should be exported. Could be expanded by child dockerfiles. The list is also passed to in-docker make targets environment. Default: `OSNAME PROJECT_NAME`
- `PROJECT_DIR`: absolute path of the top level project dir. Default: resolved as `git rev-parse --show-toplevel`
- `PRE_PACKAGE_GOALS`: space separated list of targets that should be updated before `package` target execution, i.e. ``$(PRE_PACKAGE_GOALS)`` is a prerequisite for the target. Default: not set
- `DOCKER_NAME`: name of the image to use in `%_in_docker` target's recipe. Default: not set
- `DOCKER_TAG`: tag of the image to use in `%_in_docker` target's recipe. Default: not set
- `DOCKER_UID`: `uid` of the user passed to `docker run` command. Default: resolved as `id -u`
- `BASE_DOCKER_VERSION` (**required**): impacts the tag of the image built by `image_base` target. The tag is evaluated as: `$(BASE_DOCKER_VERSION)-$(OSNAME)`. Default: not set
- `RUST_DOCKER_VERSION`(**required**): impacts the tag of the image built by `image_rust` target. The tag is evaluated as: `$(RUST_DOCKER_VERSION)-$(OSNAME)`. Default: not set

#### Variables for package build configuration

Variables to config packaing using [fpm][a1feb9f1] tool:
  - (please refer to [fpm wiki][3c28cd3e] for more information about the fpm command line options)
  - `FPM_P_NAME` (**required**): value for fpm's `--name` option. Default: `$(SRC_VERSION)`
  - `FPM_P_VERSION` (**required**): value for fpm's `--version` option. Default: not set
  - `FPM_P_INPUT_TYPE`: value for fpm's `--input-type` option. Default: `dir`
  - `FPM_P_OUTPUT_TYPE`: value for fpm's `--output-type` option. Default: `deb` if `OSNAME=xenial`, `rpm` if `OSNAME=centos7`, otherwise - not set
  - `FPM_P_OUTPUT_DIR`: value for fpm's `--package` option. Default: not set
  - `FPM_P_MAINTAINER`: value for fpm's `--maintainer` option. Default: not set
  - `FPM_P_URL`: value for fpm's `--url` option. Default: not set
  - `FPM_P_LICENSE`: value for fpm's `--license` option. Default: not set
  - `FPM_P_DESCRIPTION`: value for fpm's `--description` option. Default: not set
  - `FPM_ARGS`: string with any fpm arguments to add to the end of the fpm command line. Default: not set
  - ... (please refer to [fpm.mk](fpm.mk) for more details about fpm related environment variables)

  [3c28cd3e]: https://github.com/jordansissel/fpm/wiki "fpm wiki"

#### Helper function-like variables

The following variables could be used in `call` function and expects some arguments:
- `check_non_empty`: check if value is non empty and raises error otherwise. Arguments:
  - value to check
  - error message
- `check_defined`: check if variable is defined (have non empty value) and raises error otherwise
  - variable name
  - (optional) error message
- `docker_env_value`:  resolves the value of some `ENV` instruction in dockerfile. Arguments:
  - `ENV_V_NAME>` is key in `ENV` instruction
  - `<PATH_TO_DOCKERFILE>` path to dockerfile
- `docker_from_tag`: resolves the tag of the base image defined in `FROM` instruction in dockerfile. Arguments:
  - `<BASE_IMAGE_NAME>` is name of the base image in `FROM` instruction
  - `<PATH_TO_DOCKERFILE>`
- `docker_from_version`: resolves the version part of the base image's tag defined in `FROM` instruction in dockerfile. It is expected that the tag has the following format: `<VERSION>-...`. Arguments:
  - `<BASE_IMAGE_NAME>` is name of the base image in `FROM` instruction
  - `<PATH_TO_DOCKERFILE>`
