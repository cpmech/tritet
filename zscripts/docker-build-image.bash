#!/bin/bash

set -e

# the first argument is the distro: "arch" or "rocky"
# the second argument is "1" to enable MUMPS
DISTRO=${1:-""}

# image name
NAME="cpmech/tritet_ubuntu"
DKFILE="Dockerfile.Ubuntu"
if [ "${DISTRO}" = "arch" ]; then
    NAME="cpmech/tritet_arch"
    DKFILE="Dockerfile.Arch"
fi
if [ "${DISTRO}" = "rocky" ]; then
    NAME="cpmech/tritet_rocky"
    DKFILE="Dockerfile.Rocky"
fi

# build Docker image
docker build -f $DKFILE -t $NAME .

echo
echo
echo
echo "... SUCCESS ..."
echo
echo "... image ${NAME} created ..."
echo
