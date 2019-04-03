#!/bin/bash

set -e
set -x

if [ "$1" = "--help" ] ; then
  echo "Usage: <version> <key> <branchName> <number> <artifact> <artifact_name>"
  return
fi

version="$1"
key="$2"
branchName="$3"
buildNumber="$4"
artifact="$5"
file="$6"

[ -z $version ] && exit 1
[ -z $key ] && exit 2
[ -z $branchName ] && exit 3
[ -z $buildNumber ] && exit 4
[ -z $artifact ] && exit 5
[ -z $file ] && exit 6

ssh -v -oStrictHostKeyChecking=no -i $key repo@$SOVRIN_REPO_HOST mkdir -p /var/repository/repos/android/${artifact}/${branchName}/${version}-${buildNumber}

cat <<EOF | sftp -v -oStrictHostKeyChecking=no -i $key repo@$SOVRIN_REPO_HOST
cd /var/repository/repos/android/${artifact}/${branchName}/$version-$buildNumber
put -r ${file}
ls -l /var/repository/repos/android/${artifact}/${branchName}/$version-$buildNumber
EOF
