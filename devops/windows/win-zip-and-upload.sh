#!/bin/bash

set -e
set -x

if [ "$1" = "--help" ] ; then
  echo "Usage: <version> <key> <type> <number>"
  return
fi

version="$1"
key="$2"
type="$3"
number="$4"

[ -z $version ] && exit 1
[ -z $key ] && exit 2
[ -z $type ] && exit 3
[ -z $number ] && exit 4

PACKAGE_NAME="libsovtoken"
TEMP_ARCH_DIR=./${PACKAGE_NAME}-zip

mkdir ${TEMP_ARCH_DIR}

cp ./target/release/*.dll ${TEMP_ARCH_DIR}/

pushd ${TEMP_ARCH_DIR}
    zip -r ${PACKAGE_NAME}_${version}.zip ./*
    mv ${PACKAGE_NAME}_${version}.zip ..
popd

rm -rf ${TEMP_ARCH_DIR}

cat <<EOF | sftp -v -oStrictHostKeyChecking=no -i $key repo@$SOVRIN_REPO_HOST
mkdir /var/repository/repos/windows/$PACKAGE_NAME
mkdir /var/repository/repos/windows/$PACKAGE_NAME/master
mkdir /var/repository/repos/windows/$PACKAGE_NAME/stable
mkdir /var/repository/repos/windows/$PACKAGE_NAME/test
mkdir /var/repository/repos/windows/$PACKAGE_NAME/$type/$version-$number
cd /var/repository/repos/windows/$PACKAGE_NAME/$type/$version-$number
put -r ${PACKAGE_NAME}_"${version}".zip
ls -l /var/repository/repos/windows/$PACKAGE_NAME/$type/$version-$number
EOF
