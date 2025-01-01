#!/bin/bash

set -euo pipefail

exec /usr/local/bin/sops-gitops-github-action "$@"
