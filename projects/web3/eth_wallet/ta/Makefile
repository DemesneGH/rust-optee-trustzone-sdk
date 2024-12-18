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

# STD-ONLY example

UUID ?= $(shell cat "../uuid.txt")

TARGET_TA ?= aarch64-unknown-linux-gnu
CROSS_COMPILE_TA ?= aarch64-linux-gnu-
OBJCOPY := $(CROSS_COMPILE_TA)objcopy
LINKER_CFG := target.$(TARGET_TA).linker=\"$(CROSS_COMPILE_TA)ld.bfd\"

TA_SIGN_KEY ?= $(TA_DEV_KIT_DIR)/keys/default_ta.pem
SIGN := $(TA_DEV_KIT_DIR)/scripts/sign_encrypt.py
OUT_DIR := $(CURDIR)/target/$(TARGET_TA)/release

ifeq ($(STD),)
all:
	@echo "Please `export STD=y` then rerun `source environment` to build the STD version"
else
all: ta strip sign
endif

# set the cross compile for building inner libraries, such as C libraries in ring
ta:
	@CROSS_COMPILE=$(CROSS_COMPILE_TA) xargo build --target $(TARGET_TA) --release --config $(LINKER_CFG)

strip: ta
	@$(OBJCOPY) --strip-unneeded $(OUT_DIR)/ta $(OUT_DIR)/stripped_ta

sign: strip
	@$(SIGN) --uuid $(UUID) --key $(TA_SIGN_KEY) --in $(OUT_DIR)/stripped_ta --out $(OUT_DIR)/$(UUID).ta
	@echo "SIGN =>  ${UUID}"

clean:
	@cargo clean