#!/bin/bash

set -euo pipefail

NAME="cpmech/tritet_arch"

docker run --rm -it "${NAME}:latest" /bin/bash
