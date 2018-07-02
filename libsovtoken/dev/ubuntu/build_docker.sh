#!/bin/bash

RUST_DIR="../.."
MODE="build"
DOCKERFILE="Dockerfile"
DOCKERIMAGE="libsovtoken"
APT_INSTALL="stable"
INDY_CHECKOUT_URL=""
INDY_GIT_CHECKOUT_DIR="/var/tmp/indy-sdk"
INDY_GIT_CHECKOUT_OVERWRITE=0
INDY_LOCAL_DIR=""
GIT_SHALLOW_CLONE=0
GIT_BRANCH="master"
REBUILD=0
HOST=$(uname -s)
case "${HOST}" in
    Linux*)   CPUS=$(grep -c ^processor /proc/cpuinfo) ;;
    CYGWIN*)  CPUS=$(grep -c ^processor /proc/cpuinfo) ;;
    MINGW*)   CPUS=$(grep -c ^processor /proc/cpuinfo) ;;
    FreeBSD*) CPUS=$(sysctl -n hw.physicalcpu) ;;
    Darwin*)  CPUS=$(sysctl -n hw.physicalcpu) ;;
    *) CPUS=2 ;;
esac
CPUS=$((CPUS / 2))
RUST_FLUSH_CACHE=0
PUBLISH_URL="https://kraken.corp.evernym.com/repo/agency_dev/upload"
PUBLISH_USER="jenkins:jenkins"

__usage() {
    cat <<EOT
    Usage: $0 [options]

    Options:
        -h  Display this message
        -a  Install libindy using apt package manager from a specific channel. Default: '${APT_INSTALL}'.
            Can be 'master|stable|rc'. This is the default method for install libindy.
            Options -i or -g will cause this option to be ignored.
        -b  Use named branch for git clone. Default: '${GIT_BRANCH}'
            Can be 'master|tags/v1.4|stable'. If the current branch in the local clone is
            not set to branch, then it will try to set it. If it fails, the script will abort.
        -c  Run a custom command instead of cargo \$mode.
            This is useful when you need to use more options with cargo
            like 'cargo test -- --nocapture' or 'cargo build --verbose' or 'cargo check'
        -d  Directory to find libsovtoken/src/Cargo.toml. Default: '${RUST_DIR}'
        -D  Local directory where to clone libindy. Default: '${INDY_GIT_CHECKOUT_DIR}'.
            This option will be selected over -g if both are used.
        -f  Dockerfile to use to for building docker instance. Default: '${DOCKERFILE}'
        -g  Use git to clone libindy from this URL and compile from source.
            Example: https://github.com/hyperledger/indy-sdk.git.
        -i  Compile libindy from local source directory. This is root folder to indy-sdk.
        -j  The number of cpus to give docker to run. Default: ${CPUS}. 0.000 means no limit.
        -m  The mode to run cargo inside docker. Default: '${MODE}'.
            Valid options are 'build|release|test|check|package|publish'.
        -n  Name to give the built docker image. Default: '${DOCKERIMAGE}'
        -o  When combined with -g, force git clone in existing directory overwriting existing contents.
            Default: '${INDY_GIT_CHECKOUT_OVERWRITE}'
        -p  When mode is set to 'publish', this is the credentials to use to publish the deb file. Default: '${PUBLISH_USER}'
        -r  Combined with -i or -g, will force rebuilding of libindy. Default: '${REBUILD}'
        -R  Force reloading of cargo registries. Default: '${RUST_FLUSH_CACHE}'
        -s  Shallow cloning libindy git installations
EOT
}

__complete_build_file() {
cat >> "${BUILD_DIR}/build.sh" << EOF
${CMD}

if [ ! -L "/home/token_user/.cargo/git" ] ; then
    rm -rf /rust/git
    mv /home/token_user/.cargo/git /rust/git
fi
if [ ! -L "/home/token_user/.cargo/registry" ] ; then
    rm -rf /rust/registry
    mv /home/token_user/.cargo/registry /rust/registry
fi
EOF
}

__echocmd() {
    echo $1
    eval $1
}


while getopts ':a:b:c:d:D:f:g:hi:j:m:n:op:rRs' opt
do
    case "${opt}" in
        a) APT_INSTALL="${OPTARG}" ;;
        b) GIT_BRANCH="${OPTARG}" ;;
        c) COMMANDS="${OPTARG}" ;;
        d) RUST_DIR="${OPTARG}" ;;
        D) INDY_GIT_CHECKOUT_DIR="${OPTARG}" ;;
        f) DOCKERFILE="${OPTARG}" ;;
        g) INDY_CHECKOUT_URL="${OPTARG}" ;;
        h) __usage; exit 0 ;;
        i) INDY_LOCAL_DIR="${OPTARG}" ;;
        j) CPUS=${OPTARG} ;;
        m) MODE="${OPTARG}" ;;
        n) DOCKERIMAGE="${OPTARG}" ;;
        o) INDY_GIT_CHECKOUT_OVERWRITE=1 ;;
        p) PUBLISH_URL=${OPTARG} ;;
        r) REBUILD=1 ;;
        R) RUST_FLUSH_CACHE=1 ;;
        s) GIT_SHALLOW_CLONE=1 ;;
        \?) echo STDERR "Option does not exist: ${OPTARG}"
            exit 1
            ;;
    esac
done
shift $((OPTIND-1))

if [ ! -z "${COMMANDS}" ] ; then
    echo "Running custom command ${COMMANDS}"
    CMD="${COMMANDS}"
else
    case "${MODE}" in
        test) CMD="cargo test --color=always -- --nocapture" ;;
        build) CMD="cargo build --color=always" ;;
        release) CMD="cargo build --color=always --release" ;;
        check) CMD="cargo check --color=always" ;;
        package) CMD="cargo build --color=always --release && cargo deb --no-build" ;;
        publish) CMD="cargo build --color=always --release && cargo deb --no-build"
            if [ -z "${PUBLISH_USER}" ] ; then
                echo STDERR "The publish user cannot be blank"
                exit 1
            fi
            ;;
        \?) echo STDERR "Unknown MODE specified"
            exit 1
            ;;
    esac
fi


INDY_INSTALL_METHOD="package"

if [ ! -z "${INDY_LOCAL_DIR}" ] ; then
    INDY_INSTALL_METHOD="build"

    if [ ! -d "${INDY_LOCAL_DIR}" ] ; then
        echo STDERR "${INDY_LOCAL_DIR} does not exist"
        exit 1
    fi
elif [ ! -z "${INDY_CHECKOUT_URL}" ] ; then
    INDY_INSTALL_METHOD="build"

    CLONE=1
    CHECK_REV=1
    if [ -d "${INDY_GIT_CHECKOUT_DIR}" ] ; then
        echo "${INDY_GIT_CHECKOUT_DIR} exists"

        if [ ${INDY_GIT_CHECKOUT_OVERWRITE} -eq 1 ] ; then
            echo "Overwriting ${INDY_GIT_CHECKOUT_DIR}"
            rm -rf ${INDY_GIT_CHECKOUT_DIR}
        else
            CLONE=0
        fi
    fi

    if [ ${CLONE} -eq 1 ] ; then
        CHECK_REV=0
        REBUILD=1
        GIT="git clone"
        if [ ${GIT_SHALLOW_CLONE} -eq 1 ] ; then
            GIT="${GIT} --depth 1"
        fi
        GIT="${GIT} --branch ${GIT_BRANCH} ${INDY_CHECKOUT_URL} ${INDY_GIT_CHECKOUT_DIR}"
        __echocmd "${GIT}"
    fi

    if [ ${CHECK_REV} -eq 1 ] ; then
        GIT_REV=$(git --git-dir "${INDY_GIT_CHECKOUT_DIR}/.git" branch | head -n 1 | sed -e 's/^..//g')
        echo "Current indy-sdk branch set to ${GIT_REV}"
        MATCH=$(echo ${GIT_REV} | egrep "${GIT_BRANCH}")

        if [ -z "${MATCH}" ] ; then
            echo "Changing branch to ${GIT_BRANCH}"
            git --git-dir "${INDY_GIT_CHECKOUT_DIR}/.git" checkout ${GIT_BRANCH}
            REBUILD=1

            if [ $? -ne 0 ] ; then
                echo STDERR "Could not change branch to ${GIT_BRANCH}"
                exit 1
            fi
        fi
    fi
    INDY_LOCAL_DIR=${INDY_GIT_CHECKOUT_DIR}
fi

if [ ! -z "$@" ] ; then
    echo STDERR "Ignoring other parameters $@"
fi

DOCKER_IMAGE_ID=$(docker image ls | grep ${DOCKERIMAGE} | perl -pe 's/\s+/ /g' | cut -d ' ' -f 3)

if [ "${RUST_DIR:0:1}" = '/' ] ; then
    BUILD_DIR=${RUST_DIR}
else
    BUILD_DIR="${PWD}/${RUST_DIR}"
fi

echo "Using libsovtoken in ${BUILD_DIR}"

if [ -z "${DOCKER_IMAGE_ID}" ] ; then
    echo "Docker image ${DOCKERIMAGE} does not exist"
    echo "Docker image will be built with ${DOCKERFILE}"
    INDY_INSTALL="indy_install.sh"
    if [ "${INDY_INSTALL_METHOD}" == "package" ] ; then
        cat > "${INDY_INSTALL}" << EOT
#!/bin/bash
set -xv
apt-get -qq update -y
apt-get -qq install -y software-properties-common apt-transport-https 2>&1 > /dev/null
apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 68DB5E88
add-apt-repository -y "deb https://repo.sovrin.org/sdk/deb xenial ${APT_INSTALL}"
apt-get -qq update -y && apt-get -qq install -y libindy 2>&1 > /dev/null
EOT
    else
        echo "" > "${INDY_INSTALL}"
    fi
    __echocmd "docker build -f ${DOCKERFILE} -t ${DOCKERIMAGE}:latest ${BUILD_DIR}/dev/ubuntu --build-arg indy_install=indy_install.sh"
    rm -f "${INDY_INSTALL}"
else
    echo "Using existing docker image ${DOCKERIMAGE}:latest"
fi

rm -f "${BUILD_DIR}/build.sh"
if [ -d "${HOME}/.dockercargo" ] ; then
    find "${HOME}/.dockercargo" -name .cargo-index-lock | xargs rm -f
else
    mkdir -p ${HOME}/.dockercargo
    chmod 777 ${HOME}/.dockercargo
fi

if [ ${RUST_FLUSH_CACHE} -eq 0 ] ; then
    cat > "${BUILD_DIR}/build.sh" << EOF
if [ -d "/rust/git" ] ; then
    echo "Reusing cargo/git"
    ln -fs /rust/git /home/token_user/.cargo/git
fi
if [ -d "/rust/registry" ] ; then
    echo "Reusing cargo/registry"
    ln -fs /rust/registry /home/token_user/.cargo/registry
fi
EOF
fi

DK_CMD="docker run --cpus=${CPUS} --rm -w /data -v ${HOME}/.dockercargo:/rust"

if [ "${INDY_INSTALL_METHOD}" == "build" ] ; then
    CLEAN_CMD=""
    if [ ${REBUILD} -eq 1 ] ; then
        CLEAN_CMD="cargo clean"
    fi

    cat >> "${BUILD_DIR}/build.sh" << EOF
export LD_LIBRARY_PATH=/usr/lib:/usr/local/lib:/indy-sdk/libindy/target/release
export LIBINDY_DIR=/indy-sdk/libindy/target/release
pushd /indy-sdk/libindy
${CLEAN_CMD}
cargo build --release
popd
EOF
    DK_CMD="${DK_CMD} -v \"${INDY_LOCAL_DIR}:/indy-sdk\""
else
    cat >> "${BUILD_DIR}/build.sh" << EOF
export LD_LIBRARY_PATH=/usr/lib:/usr/local/lib
export LIBINDY_DIR=/usr/lib
export RUST_TEST_THREADS=1
EOF
fi

__complete_build_file

DK_CMD="${DK_CMD} -v \"${BUILD_DIR}:/data\" -t ${DOCKERIMAGE}:latest bash build.sh"

__echocmd "${DK_CMD}"

rm -f "${BUILD_DIR}/build.sh"

if [ "${MODE}" = "publish" ] ; then
    DEB_FILE=$(ls ${BUILD_DIR}/target/debian | grep '\.deb$')
    if [ ! -r "${DEB_FILE}" ] ; then
        echo STDERR "Cannot publish deb file. It is not readable or doesn't exist."
        exit 1
    fi

    __echocmd "curl -v -u ${PUBLISH_USER} -X POST -F file=@\"${DEB_FILE}\" ${PUBLISH_URL}"
fi
