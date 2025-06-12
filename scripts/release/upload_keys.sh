#!/bin/bash

# Licensed to the Apache Software Foundation (ASF) under one
# or more contributor license agreements.  See the NOTICE file
# distributed with this work for additional information
# regarding copyright ownership.  The ASF licenses this file
# to you under the Apache License, Version 2.0 (the
# "License"); you may not use this file except in compliance
# with the License.  You may obtain a copy of the License at
#
#   http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing,
# software distributed under the License is distributed on an
# "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
# KIND, either express or implied.  See the License for the
# specific language governing permissions and limitations
# under the License.

set -e

# Script name
SCRIPT_NAME="$(basename "$0")"

# Your ASF credentials
ASF_USERNAME="${ASF_USERNAME:-}"
ASF_PASSWORD="${ASF_PASSWORD:-}"

# SVN repository URLs
SVN_DEV="https://dist.apache.org/repos/dist/dev/incubator/teaclave"
SVN_RELEASE="https://dist.apache.org/repos/dist/release/incubator/teaclave"

show_help() {
    cat <<EOF
Usage: $SCRIPT_NAME [pull|push]

Commands:
  pull      Shallow checkout the KEYS file from the dev SVN repo into 'svn-dev-teaclave'.
            You can then edit 'svn-dev-teaclave/KEYS' manually.
  push      Commit the updated KEYS file to the dev repo and sync it to the release repo.

Environment Variables:
  ASF_USERNAME  Your Apache SVN username.
  ASF_PASSWORD  Your Apache SVN password.

Examples:
  ASF_USERNAME=your_id ASF_PASSWORD=your_pass $SCRIPT_NAME pull
  (edit svn-dev-teaclave/KEYS)
  ASF_USERNAME=your_id ASF_PASSWORD=your_pass $SCRIPT_NAME push
EOF
}

if [[ "$1" == "-h" || "$1" == "--help" || -z "$1" ]]; then
    show_help
    exit 0
fi

case "$1" in
    pull)
        svn co --depth=files "$SVN_DEV" svn-dev-teaclave
        echo "Checked out dev SVN repo to 'svn-dev-teaclave'."
        echo "Next step: edit 'svn-dev-teaclave/KEYS' as needed, then run:"
        echo "  $SCRIPT_NAME push"
        ;;
    push)
        cd svn-dev-teaclave

        # Check if KEYS file has changes
        if svn status KEYS | grep -q '^[AM]'; then
            # Commit updated KEYS file to dev repo
            svn ci --username "$ASF_USERNAME" --password "$ASF_PASSWORD" -m "Update KEYS"
            # Replace KEYS file in the release repo
            svn rm --username "$ASF_USERNAME" --password "$ASF_PASSWORD" "$SVN_RELEASE/KEYS" -m "Update KEYS"
            svn cp --username "$ASF_USERNAME" --password "$ASF_PASSWORD" "$SVN_DEV/KEYS" "$SVN_RELEASE/" -m "Update KEYS"
            echo "KEYS file successfully updated and published."

        else
            echo "No changes to KEYS file. Nothing to commit."
            exit 0
        fi
        ;;
    *)
        echo "Unknown command: $1"
        echo
        show_help
        exit 1
        ;;
esac
