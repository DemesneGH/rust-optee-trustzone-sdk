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

#![no_main]

use optee_utee::{
    ta_close_session, ta_create, ta_destroy, ta_invoke_command, ta_open_session, trace_println,
};
use optee_utee::{Error, ErrorKind, Parameters, Result};
use proto::Command;

mod ecdsa;

#[ta_create]
fn create() -> Result<()> {
    trace_println!("[+] TA create");
    Ok(())
}

#[ta_open_session]
fn open_session(_params: &mut Parameters) -> Result<()> {
    trace_println!("[+] TA open session");
    Ok(())
}

#[ta_close_session]
fn close_session() {
    trace_println!("[+] TA close session");
}

#[ta_destroy]
fn destroy() {
    trace_println!("[+] TA destroy");
}

fn test_sign_and_verify() -> Result<()> {
    trace_println!("[+] Starting ECDSA test workflow");

    // Step 1: Generate ECDSA keypair
    trace_println!("[+] Generating ECDSA keypair...");
    let key_pair = match ecdsa::EcdsaKeyPair::new() {
        Ok(kp) => {
            trace_println!("[+] Keypair generated successfully");
            kp
        }
        Err(e) => {
            trace_println!("[-] Failed to generate keypair: {:?}", e);
            return Err(Error::new(ErrorKind::Generic));
        }
    };

    // Step 2: Prepare a test message to sign
    let message = b"Hello, OP-TEE Trusted Application with ECDSA!";
    trace_println!(
        "[+] Message to sign: {:?}",
        core::str::from_utf8(message).unwrap_or("Invalid UTF-8")
    );

    // Step 3: Sign the message
    trace_println!("[+] Signing message...");
    let signature = match key_pair.sign(message) {
        Ok(sig) => {
            trace_println!(
                "[+] Message signed successfully, signature length: {}",
                sig.len()
            );
            sig
        }
        Err(e) => {
            trace_println!("[-] Failed to sign message: {:?}", e);
            return Err(Error::new(ErrorKind::Generic));
        }
    };

    // Step 4: Verify the signature
    trace_println!("[+] Verifying signature...");
    let public_key = key_pair.pub_key();
    match ecdsa::verify_signature(public_key, message, &signature) {
        Ok(()) => {
            trace_println!("[+] Signature verification successful!");
        }
        Err(e) => {
            trace_println!("[-] Signature verification failed: {:?}", e);
            return Err(Error::new(ErrorKind::Generic));
        }
    }

    trace_println!("[+] ECDSA workflow completed successfully!");
    Ok(())
}

#[ta_invoke_command]
fn invoke_command(cmd_id: u32, _params: &mut Parameters) -> Result<()> {
    trace_println!("[+] TA invoke command");
    match Command::from(cmd_id) {
        Command::Test => {
            test_sign_and_verify()?;

            trace_println!("[+] Test passed");
            Ok(())
        }
        _ => {
            return Err(Error::new(ErrorKind::NotSupported));
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/user_ta_header.rs"));
