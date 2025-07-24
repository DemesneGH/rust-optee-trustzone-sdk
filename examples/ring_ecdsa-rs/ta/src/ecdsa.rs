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

use anyhow::{anyhow, Result};
use ring::rand;
use ring::signature::{self, EcdsaSigningAlgorithm, KeyPair};
pub use ring::signature::{ECDSA_P256_SHA256_ASN1, ECDSA_P256_SHA256_ASN1_SIGNING};

/// Default signature algorithm for P-256 curve
pub static DEFAULT_SIGNING_ALGORITHM: &EcdsaSigningAlgorithm = &ECDSA_P256_SHA256_ASN1_SIGNING;
pub static DEFAULT_VERIFICATION_ALGORITHM: &ring::signature::EcdsaVerificationAlgorithm =
    &ECDSA_P256_SHA256_ASN1;

/// EcdsaKeyPair stores a pair of ECDSA (private, public) key with configurable algorithm support.
pub struct EcdsaKeyPair {
    key_pair: signature::EcdsaKeyPair,
    pkcs8_bytes: Vec<u8>,
    signing_algorithm: &'static EcdsaSigningAlgorithm,
    verification_algorithm: &'static ring::signature::EcdsaVerificationAlgorithm,
}

impl EcdsaKeyPair {
    /// Generate an ECDSA key pair using the default P-256 algorithm.
    pub fn new() -> Result<Self> {
        Self::new_with_algorithm(DEFAULT_SIGNING_ALGORITHM, DEFAULT_VERIFICATION_ALGORITHM)
    }

    /// Generate an ECDSA key pair with the specified algorithm.
    pub fn new_with_algorithm(
        signing_algo: &'static EcdsaSigningAlgorithm,
        verification_algo: &'static ring::signature::EcdsaVerificationAlgorithm,
    ) -> Result<Self> {
        let rng = rand::SystemRandom::new();
        let pkcs8_bytes = signature::EcdsaKeyPair::generate_pkcs8(signing_algo, &rng)
            .map_err(|e| anyhow!("failed to generate PKCS#8 bytes: {:?}", e))?;
        let key_pair =
            signature::EcdsaKeyPair::from_pkcs8(signing_algo, pkcs8_bytes.as_ref(), &rng)
                .map_err(|e| anyhow!("failed to create key pair from PKCS#8: {:?}", e))?;
        Ok(Self {
            key_pair,
            pkcs8_bytes: pkcs8_bytes.as_ref().to_vec(),
            signing_algorithm: signing_algo,
            verification_algorithm: verification_algo,
        })
    }

    /// Create an ECDSA key pair from existing PKCS#8 bytes with default P-256 algorithm.
    pub fn from_bytes(pkcs8_bytes: &[u8]) -> Result<Self> {
        Self::from_bytes_with_algorithm(
            pkcs8_bytes,
            DEFAULT_SIGNING_ALGORITHM,
            DEFAULT_VERIFICATION_ALGORITHM,
        )
    }

    /// Create an ECDSA key pair from existing PKCS#8 bytes with specified algorithm.
    pub fn from_bytes_with_algorithm(
        pkcs8_bytes: &[u8],
        signing_algo: &'static EcdsaSigningAlgorithm,
        verification_algo: &'static ring::signature::EcdsaVerificationAlgorithm,
    ) -> Result<Self> {
        let rng = rand::SystemRandom::new();
        let key_pair = signature::EcdsaKeyPair::from_pkcs8(signing_algo, pkcs8_bytes, &rng)
            .map_err(|e| anyhow!("failed to create key pair from PKCS#8: {:?}", e))?;
        Ok(Self {
            key_pair,
            pkcs8_bytes: pkcs8_bytes.to_vec(),
            signing_algorithm: signing_algo,
            verification_algorithm: verification_algo,
        })
    }

    /// Get the public key bytes.
    pub fn pub_key(&self) -> &[u8] {
        self.key_pair.public_key().as_ref()
    }

    /// Get the private key PKCS#8 bytes.
    pub fn prv_key(&self) -> &[u8] {
        self.pkcs8_bytes.as_ref()
    }

    /// Get the signing algorithm used by this key pair.
    pub fn signing_algorithm(&self) -> &'static EcdsaSigningAlgorithm {
        self.signing_algorithm
    }

    /// Get the verification algorithm used by this key pair.
    pub fn verification_algorithm(&self) -> &'static ring::signature::EcdsaVerificationAlgorithm {
        self.verification_algorithm
    }

    /// Sign a message using this key pair.
    pub fn sign(&self, msg: &[u8]) -> Result<Vec<u8>> {
        let rng = rand::SystemRandom::new();
        match self.key_pair.sign(&rng, msg) {
            Ok(sig) => Ok(sig.as_ref().to_vec()),
            Err(e) => Err(anyhow!("failed to sign message: {:?}", e)),
        }
    }
}

/// Verify a signature using the default P-256 algorithm.
pub fn verify_signature(pub_key: &[u8], msg: &[u8], signature: &[u8]) -> Result<()> {
    verify_signature_with_algorithm(pub_key, msg, signature, DEFAULT_VERIFICATION_ALGORITHM)
}

/// Verify a signature using the specified algorithm.
pub fn verify_signature_with_algorithm(
    pub_key: &[u8],
    msg: &[u8],
    signature: &[u8],
    algo: &'static ring::signature::EcdsaVerificationAlgorithm,
) -> Result<()> {
    let peer_pub_key = signature::UnparsedPublicKey::new(algo, pub_key);
    match peer_pub_key.verify(msg, signature) {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!("signature verification failed: {:?}", e)),
    }
}
