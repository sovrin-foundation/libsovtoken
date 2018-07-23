# Libsovtoken devops routine

This folder includes devops related routine and consists of the following parts:
- [Makefile](Makefile) automates devops tasks like test, package and publish to [crates.io](https://crates.io/) which could be performed either on-host or in-docker
- [docker](docker) folder holds docker related routine
- [aws-codebuild](aws-codebuild) folder consists of files that describes AWS CodeBuild based CI/CD pipelines
- [ext](ext) folder is a [git-subrepo][d003158e] of shared [library](https://github.com/evernym/jenkins-shared/tree/devops-shared) which provides makefile based approach of devops tasks automation. Please check its [README.md](ext/README.md) for more information.

  [d003158e]: https://github.com/ingydotnet/git-subrepo "git-subrepo"

## Docker

Aurora wallet is shipped with dockerfiles for ubuntu [xenial](docker/ci/xenial/Dockerfile) and [centos7](docker/ci/xenial/Dockerfile) which describe images with necessary environment for CI/CD tasks on these OSes.

## CI pipeline

CI pipeline is described by [Jenkinsfile.ci](aws-codebuild/Jenkinsfile.ci). It uses [Jenkins shared library](https://github.com/evernym/jenkins-shared/tree/aws-codebuild) API to build projects on [AWS CodeBuild](https://aws.amazon.com/codebuild/). CI utilizes docker containers from [docker/ci](docker/ci) folder to run tests on both ubuntu `xenial` and `centos7`.

CI pipeline stages:
- clone the GitHub repository
- upload current HEAD as zip archive to AWS S3 bucket used by CodeBuild project
- launch a CodeBuild project using `AwsCodeBuildHelper.build` API. It includes a set of sub-stages:
  - (optional) create/update the CodeBuild project
  - (optional) create an AWS ECR repository to use by CodeBuild project
  - (optional) build docker image and push it to the AWS ECR repository
  - run the CodeBuild project to perform cargo testing
  - download logs
- archive logs

## CD pipeline

CD pipeline is described by [Jenkinsfile.cd](aws-codebuild/Jenkinsfile.cd). It uses [Jenkins shared library](https://github.com/evernym/jenkins-shared/tree/aws-codebuild) API as well. For now CD generates artifacts (debian package) only for ubuntu `xenial`.

CD pipeline stages:
- clone the GitHub repository
- resolve the following parameters:
  - current source version from [Cargo.toml](../libsovtoken/Cargo.toml)
  - last revision number among the debian packages with the same source version in [Evernym debian repo](https://repo.corp.evernym.com/deb/dists/evernym-agency-dev-ubuntu/)
- evaluate new debian package version basing on source version, last revision number and current build number
- upload current HEAD as zip archive to AWS S3 bucket used by CodeBuild project
- launch a CodeBuild project using the same `AwsCodeBuildHelper.build` API as CI does. The main difference here is that CD pipeline doesn't build an image for AWS ECR repository assuming that it has been done previously by CI pipeline. Its sub-stages:
  - (optional) create/update CodeBuild project (TODO shouldn't do that in any case assuming CI did that)
  - run the CodeBuild project to perform debian packaging
  - download logs
- archive logs
- upload created debian package to [Evernym debian repo](https://repo.corp.evernym.com/deb/dists/evernym-agency-dev-ubuntu/)

## Makefile

### Requirements

- make
- docker
- docker-compose

### Targets
- `test_dry` runs tests in "dry" mode: `cargo test --no-run`
- `test` runs tests: `cargo test`
- `build` runs `cargo build`
- `publish_crate` publishes the code to crates.io performing cargo `login`, `package` and `publish` commands
- `image_lst_ci` builds docker image with necessary environment for performing both CI and CD tasks
- `image_lst_ci_version` prints current version of the docker image (dockerfile) built by `image_lst_ci` target
- please refer to [ext/README.md](ext/README.md) for list of targets inherited from there


Each target could be run in two ways - with or without `_in_docker` postfix: e.g. `test_in_docker` and `test`. In former case the target is run inside docker container (though it makes sense not for all targets), otherwise current host's environment is used.

### Environment variables

- `PROJECT_DIR`: absolute path of the top level project dir. Default: resolved as `git rev-parse --show-toplevel`
- `RELEASE`: adds `--release` flag to cargo `test` and `build` commands if set to `1`. Default: `1`
- `OSNAME`: switches OS context, possible values: `xenial`, `centos7`. Default: `xenial`
- `CARGO_TARGET_DIR`: sets [CARGO_TARGET_DIR](https://doc.rust-lang.org/cargo/reference/environment-variables.html) environment variable. Default: `target/$(OSNAME)`
- `CRATE_P_VERSION`: if set overwrites `version` field of `[package]` section in [Cargo.toml](../libsovtoken/Cargo.toml) before crate publishing. Default: not set
- `CARGO_LOGIN_TOKEN`: token to perform `cargo login` during crate publishing. Default: not set
- `DOCKER_NAME`: name of the image built by `image_lst_ci` target. Default: `evernym/libsovtoken`
- `DOCKER_TAG`: tag of the image built by `image_lst_ci` target. Default: `<VERSION>-$(OSNAME)-ci`, where `VERSION` is value of `CI_ENV_VERSION` environment variable in accordant dockerfile
- please refer to [ext/README.md](ext/README.md) for list of environment variables inherited from there
