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

set -euo pipefail

# ---------------- Modify these variables ----------------
RELEASE_VERSION="0.4.0"
RC_NUMBER="2"
GPG_KEY_UID="YOUR_KEY_UID"
ASF_USERNAME="${ASF_USERNAME:-}"
ASF_PASSWORD="${ASF_PASSWORD:-}"
# --------------------------------------------------------

WORK_BASE_DIR="teaclave-release-tmp"
TARGET_DIR="apache-teaclave-trustzone-sdk-${RELEASE_VERSION}-incubating"
SVN_TARGET_DIR="trustzone-sdk-${RELEASE_VERSION}-rc.${RC_NUMBER}"

show_usage() {
    echo "Usage: $0 <command>"
    echo
    echo "  prepare       : Package, sign, and verify release artifacts."
    echo "  upload        : Verify existing artifacts and upload to Apache dist/dev SVN."
    echo
    echo "Set this variables in script before run:"
    echo "  RELEASE_VERSION"
    echo "  RC_NUMBER"
    echo "  GPG_KEY_UID"
    echo "  ASF_USERNAME (can override via env)"
    echo "  ASF_PASSWORD (can override via env)"
    echo
}

verify_artifacts() {
    echo "[INFO] Verifying GPG signature..."
    if ! gpg --verify "${TARGET_DIR}.tar.gz.asc" "${TARGET_DIR}.tar.gz"; then
        echo "[ERROR] GPG verification failed. Aborting."
        exit 1
    fi

    echo "[INFO] Verifying SHA512 checksum..."
    if ! sha512sum -c "${TARGET_DIR}.tar.gz.sha512"; then
        echo "[ERROR] SHA512 checksum verification failed. Aborting."
        exit 1
    fi

    echo "[INFO] Verification passed."
}

case "${1:-}" in
    prepare)
        echo "[INFO] Preparing release artifacts..."

        mkdir -p "$WORK_BASE_DIR" && cd "$WORK_BASE_DIR"

        git clone https://github.com/apache/incubator-teaclave-trustzone-sdk.git "${TARGET_DIR}"
        cd "${TARGET_DIR}"
        git checkout "v${RELEASE_VERSION}-rc.${RC_NUMBER}"
        find . -name ".git*" -print0 | xargs -0 rm -rf
        find . -name ".DS_Store" -print0 | xargs -0 rm -rf
        cd ..

        tar czvf "${TARGET_DIR}.tar.gz" "${TARGET_DIR}"
        gpg -u "${GPG_KEY_UID}" --armor --detach-sign "${TARGET_DIR}.tar.gz"
        sha512sum "${TARGET_DIR}.tar.gz" > "${TARGET_DIR}.tar.gz.sha512"

        verify_artifacts

        echo "[INFO] Artifacts are ready and verified."
        echo "To upload, run:"
        echo "  $0 upload"
        ;;
    
    upload)
        echo "[INFO] Uploading artifacts..."

        cd "$WORK_BASE_DIR"
        verify_artifacts

        echo "[INFO] Uploading to SVN..."
        svn co --depth=files "https://dist.apache.org/repos/dist/dev/incubator/teaclave" svn-dev-teaclave
        cd svn-dev-teaclave
        mkdir "${SVN_TARGET_DIR}"
        cp ../"${TARGET_DIR}.tar."* "${SVN_TARGET_DIR}"
        svn add "${SVN_TARGET_DIR}"
        svn ci --username "${ASF_USERNAME}" --password "${ASF_PASSWORD}" -m "Add ${SVN_TARGET_DIR}"

        echo "[SUCCESS] Uploaded ${SVN_TARGET_DIR} to Apache dist/dev SVN."
        ;;
    
    *)
        show_usage
        ;;
esac
