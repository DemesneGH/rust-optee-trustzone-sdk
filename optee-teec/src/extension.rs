// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use crate::raw;
use crate::{Error, ErrorKind, Result};
use core::ffi::c_char;

#[repr(C)]
pub struct PluginMethod {
    pub name: *const c_char,
    pub uuid: raw::TEEC_UUID,
    pub init: fn() -> raw::TEEC_Result,
    pub invoke: unsafe fn(
        cmd: u32,
        sub_cmd: u32,
        data: *mut c_char,
        in_len: u32,
        out_len: *mut u32,
    ) -> raw::TEEC_Result,
}

/// struct PluginParameters {
/// @cmd: u32,              plugin cmd, defined in proto/
/// @sub_cmd: u32,          plugin subcmd, defined in proto/
/// @inout: &'a mut [u8],   input/output buffer shared with TA and plugin
/// @outlen,                length of output sent to TA
/// }
pub struct PluginParameters<'a> {
    pub cmd: u32,
    pub sub_cmd: u32,
    inout: &'a mut [u8],
    outlen: usize,
}
impl<'a> PluginParameters<'a> {
    pub fn new(cmd: u32, sub_cmd: u32, inout: &'a mut [u8]) -> Self {
        Self {
            cmd,
            sub_cmd,
            inout,
            outlen: 0_usize,
        }
    }

    /// Returns the current input data as a slice.
    pub fn get_input_slice(&self) -> &[u8] {
        self.inout
    }

    /// Sets the output buffer content from a slice.
    ///
    /// # Errors
    /// Returns `ErrorKind::ShortBuffer` if the buffer is too small to hold the data.
    /// In this case, `outlen` is set to the required size.
    pub fn set_output_from_slice(&mut self, sendslice: &[u8]) -> Result<()> {
        if self.inout.len() < sendslice.len() {
            println!("Output buffer too small: required {}, available {}", sendslice.len(), self.inout.len());
            self.outlen = sendslice.len();
            return Err(Error::new(ErrorKind::ShortBuffer));
        }
        self.outlen = sendslice.len();
        self.inout[..self.outlen].copy_from_slice(sendslice);
        Ok(())
    }

    /// Returns the length of the output data.
    pub fn get_out_len(&self) -> usize {
        self.outlen
    }
}
