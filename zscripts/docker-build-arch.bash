#!/bin/bash

set -euo pipefail

DOCKERFILE="Dockerfile.Arch"
NAME="cpmech/tritet_arch"

# build Docker image
docker build \
    -f "${DOCKERFILE}" \
    -t "${NAME}" \
    .

echo
echo "... SUCCESS: image ${NAME} created ..."
echo
