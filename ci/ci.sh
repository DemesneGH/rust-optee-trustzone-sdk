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

pushd ../tests

if [ "$ARCH" = "arm" ]
then
    TA_ARCH=arm-unknown-optee
else
    TA_ARCH=aarch64-unknown-optee
fi

./test_hello_world.sh $TA_ARCH
./test_random.sh $TA_ARCH
./test_secure_storage.sh $TA_ARCH
./test_aes.sh $TA_ARCH
./test_serde.sh $TA_ARCH
./test_hotp.sh $TA_ARCH
./test_acipher.sh $TA_ARCH
./test_big_int.sh $TA_ARCH
./test_diffie_hellman.sh $TA_ARCH
./test_digest.sh $TA_ARCH
./test_authentication.sh $TA_ARCH
./test_time.sh $TA_ARCH
./test_tcp_client.sh $TA_ARCH
./test_udp_socket.sh $TA_ARCH
./test_message_passing_interface.sh $TA_ARCH
./test_signature_verification.sh $TA_ARCH
./test_supp_plugin.sh $TA_ARCH
./test_tls_client.sh $TA_ARCH
./test_tls_server.sh $TA_ARCH
echo "All tests passed!"
./cleanup_all.sh

popd
