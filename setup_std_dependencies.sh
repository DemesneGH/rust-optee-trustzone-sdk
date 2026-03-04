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

set -xe

##########################################
# move to project root
cd "$(dirname "$0")"

##########################################
# install Xargo if not exist
if ! command -v xargo >/dev/null 2>&1; then
	# xargo 0.3.26 does not compile on recent toolchains (TryLockError API change).
	# Build it with an older known-good nightly.
	# Since we're working on migrating to cargo -Z build-std, we can remove the xargo then.
	XARGO_BOOTSTRAP_TOOLCHAIN=${XARGO_BOOTSTRAP_TOOLCHAIN:-nightly-2024-05-15}
	rustup toolchain install "$XARGO_BOOTSTRAP_TOOLCHAIN" --profile minimal
	cargo +"$XARGO_BOOTSTRAP_TOOLCHAIN" install xargo --locked
fi

##########################################
# initialize submodules: rust / libc / patches
RUST_COMMIT=01f6ddf7588f42ae2d7eb0a2f21d44e8e96674cf  # rust 1.93.1 stable release
LIBC_COMMIT=e879ee90b6cd8f79b352d4d4d1f8ca05f94f2f53  # libc 0.2.182

if [ -d rust/ ]
then
	rm -r rust/
fi

mkdir rust && cd rust

# Clone official Rust source at specific commit
git clone https://github.com/rust-lang/rust.git && \
	(cd rust && \
	git checkout $RUST_COMMIT && \
	git submodule update --init library/stdarch && \
	git submodule update --init library/backtrace)

# Clone official libc at specific commit
git clone https://github.com/rust-lang/libc.git && \
	(cd libc && git checkout $LIBC_COMMIT)

# Clone patches repository
git clone --depth=1 https://github.com/DemesneGH/incubator-teaclave-crates.git patches

# Apply patches
(cd rust && git apply ../patches/rust-1.93.1-01f6ddf/optee-0001-std-adaptation.patch)
(cd libc && git apply ../patches/libc-0.2.182-e879ee9/optee-0001-libc-adaptation.patch)

echo "Rust and libc sources initialized and patched"
