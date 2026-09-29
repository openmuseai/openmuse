//! Paired Desktop cryptographic channel and opaque outbound relay reference.
//!
//! This crate owns no Workspace or DSH business payload. Device keys are passed
//! in by a platform keystore adapter; the relay only sees authenticated routing
//! metadata and AEAD ciphertext.

use chacha20poly1305::{
    ChaCha20Poly1305, Key, Nonce,
    aead::{Aead, KeyInit, Payload},
};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use hkdf::Hkdf;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, VecDeque};
use x25519_dalek::{PublicKey as AgreementPublicKey, StaticSecret};
use zeroize::Zeroize;

pub const MAX_RELAY_CIPHERTEXT_BYTES: usize = 64 * 1024 + 16;
pub const ABI_VERSION: u32 = 1;
pub const DEVICE_PUBLIC_BYTES: usize = 64;
pub const MAX_PLATFORM_REF_BYTES: usize = 256;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PairedPermission {
    Read,
    Propose,
    Apply,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DevicePresence {
    Online,
    Offline,
    Sleeping,
    Replaced,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceGrant {
    pub grant_ref: String,
    pub account_ref: String,
    pub desktop_device_ref: String,
    pub mobile_device_ref: String,
    pub workspace_ref: String,
    pub permissions: Vec<PairedPermission>,
    pub generation: u64,
    pub expires_at_ms: u64,
    pub revoked: bool,
}

impl WorkspaceGrant {
    #[allow(clippy::too_many_arguments)]
    pub fn authorize(
        &self,
        account_ref: &str,
        mobile_device_ref: &str,
        desktop_device_ref: &str,
        workspace_ref: &str,
        permission: PairedPermission,
        generation: u64,
        now_ms: u64,
        presence: DevicePresence,
    ) -> Result<(), PairedRelayError> {
        if self.revoked
            || self.account_ref != account_ref
            || self.mobile_device_ref != mobile_device_ref
            || self.desktop_device_ref != desktop_device_ref
            || self.workspace_ref != workspace_ref
            || !self.permissions.contains(&permission)
            || self.generation != generation
            || now_ms >= self.expires_at_ms
            || presence != DevicePresence::Online
        {
            return Err(PairedRelayError::GrantDenied);
        }
        Ok(())
    }
}

pub struct DeviceKeyPair {
    signing: SigningKey,
    agreement: StaticSecret,
}

impl DeviceKeyPair {
    /// Deterministic constructor for a secret supplied by a platform keystore.
    /// Production callers must generate and retain a random 32-byte seed there.
    pub fn from_seed(mut seed: [u8; 32]) -> Self {
        let mut signing_seed = domain_hash(b"openmuse.device.signing.v1", &seed);
        let mut agreement_seed = domain_hash(b"openmuse.device.agreement.v1", &seed);
        seed.zeroize();
        let signing = SigningKey::from_bytes(&signing_seed);
        let agreement = StaticSecret::from(agreement_seed);
        signing_seed.zeroize();
        agreement_seed.zeroize();
        Self { signing, agreement }
    }

    pub fn signing_public(&self) -> [u8; 32] {
        self.signing.verifying_key().to_bytes()
    }

    pub fn agreement_public(&self) -> [u8; 32] {
        AgreementPublicKey::from(&self.agreement).to_bytes()
    }
}

/// Public device identity encoded as Ed25519 bytes followed by X25519 bytes.
pub fn device_public_from_seed(mut seed: [u8; 32]) -> [u8; DEVICE_PUBLIC_BYTES] {
    let keys = DeviceKeyPair::from_seed(seed);
    seed.zeroize();
    let mut output = [0_u8; DEVICE_PUBLIC_BYTES];
    output[..32].copy_from_slice(&keys.signing_public());
    output[32..].copy_from_slice(&keys.agreement_public());
    output
}

pub fn issue_pairing_offer_from_seed(
    mut seed: [u8; 32],
    account_ref: &str,
    device_ref: &str,
    nonce: [u8; 32],
    registration_generation: u64,
) -> Result<PairingOffer, PairedRelayError> {
    if account_ref.contains('\0')
        || device_ref.contains('\0')
        || account_ref.len() > MAX_PLATFORM_REF_BYTES
        || device_ref.len() > MAX_PLATFORM_REF_BYTES
    {
        seed.zeroize();
        return Err(PairedRelayError::InvalidOffer);
    }
    let keys = DeviceKeyPair::from_seed(seed);
    seed.zeroize();
    PairingOffer::issue(
        &keys,
        account_ref,
        device_ref,
        nonce,
        registration_generation,
    )
}

#[repr(C)]
pub struct OpenMusePairedBuffer {
    pub ptr: *mut u8,
    pub len: usize,
    pub capacity: usize,
    pub status: i32,
}

impl OpenMusePairedBuffer {
    fn from_vec(mut bytes: Vec<u8>, status: i32) -> Self {
        let result = Self {
            ptr: bytes.as_mut_ptr(),
            len: bytes.len(),
            capacity: bytes.capacity(),
            status,
        };
        std::mem::forget(bytes);
        result
    }

    fn failure(message: &str) -> Self {
        Self::from_vec(message.as_bytes().to_vec(), 1)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn openmuse_paired_abi_version() -> u32 {
    ABI_VERSION
}

#[unsafe(no_mangle)]
/// Derive only the public identity from a platform-owned device seed.
///
/// # Safety
/// `seed` must reference exactly `seed_len` readable bytes and `output` must
/// reference `output_len` writable bytes for this call. The seed is copied to
/// a zeroized Rust buffer and is never retained.
pub unsafe extern "C" fn openmuse_paired_device_public(
    seed: *const u8,
    seed_len: usize,
    output: *mut u8,
    output_len: usize,
) -> i32 {
    std::panic::catch_unwind(|| {
        if seed.is_null() || output.is_null() || seed_len != 32 || output_len < DEVICE_PUBLIC_BYTES
        {
            return 1;
        }
        let mut owned_seed = [0_u8; 32];
        owned_seed.copy_from_slice(unsafe { std::slice::from_raw_parts(seed, seed_len) });
        let public = device_public_from_seed(owned_seed);
        owned_seed.zeroize();
        unsafe {
            std::ptr::copy_nonoverlapping(public.as_ptr(), output, DEVICE_PUBLIC_BYTES);
        }
        0
    })
    .unwrap_or(2)
}

#[unsafe(no_mangle)]
/// Issue a signed pairing offer while keeping the device seed native-only.
///
/// # Safety
/// Every pointer must reference its declared readable length for this call.
/// The returned buffer must be freed exactly once with
/// [`openmuse_paired_buffer_free`].
pub unsafe extern "C" fn openmuse_paired_issue_offer(
    seed: *const u8,
    seed_len: usize,
    account_ref: *const u8,
    account_ref_len: usize,
    device_ref: *const u8,
    device_ref_len: usize,
    nonce: *const u8,
    nonce_len: usize,
    registration_generation: u64,
) -> OpenMusePairedBuffer {
    match std::panic::catch_unwind(|| {
        if seed.is_null()
            || account_ref.is_null()
            || device_ref.is_null()
            || nonce.is_null()
            || seed_len != 32
            || nonce_len != 32
            || account_ref_len == 0
            || device_ref_len == 0
            || account_ref_len > MAX_PLATFORM_REF_BYTES
            || device_ref_len > MAX_PLATFORM_REF_BYTES
        {
            return Err(PairedRelayError::InvalidOffer);
        }
        let account_ref = std::str::from_utf8(unsafe {
            std::slice::from_raw_parts(account_ref, account_ref_len)
        })
        .map_err(|_| PairedRelayError::InvalidOffer)?;
        let device_ref =
            std::str::from_utf8(unsafe { std::slice::from_raw_parts(device_ref, device_ref_len) })
                .map_err(|_| PairedRelayError::InvalidOffer)?;
        let mut owned_seed = [0_u8; 32];
        owned_seed.copy_from_slice(unsafe { std::slice::from_raw_parts(seed, seed_len) });
        let mut owned_nonce = [0_u8; 32];
        owned_nonce.copy_from_slice(unsafe { std::slice::from_raw_parts(nonce, nonce_len) });
        let offer_result = issue_pairing_offer_from_seed(
            owned_seed,
            account_ref,
            device_ref,
            owned_nonce,
            registration_generation,
        );
        owned_seed.zeroize();
        let offer = offer_result?;
        serde_json::to_vec(&offer).map_err(|_| PairedRelayError::CryptoFailure)
    }) {
        Ok(Ok(bytes)) => OpenMusePairedBuffer::from_vec(bytes, 0),
        Ok(Err(error)) => OpenMusePairedBuffer::failure(&error.to_string()),
        Err(_) => OpenMusePairedBuffer::failure("paired operation panic contained"),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn openmuse_paired_buffer_free(buffer: OpenMusePairedBuffer) {
    if !buffer.ptr.is_null() {
        unsafe { drop(Vec::from_raw_parts(buffer.ptr, buffer.len, buffer.capacity)) };
    }
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_openmuse_openmuse_1mobile_PairedCryptoNative_devicePublic(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    seed: jni::objects::JByteArray,
) -> jni::sys::jbyteArray {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut bytes = env.convert_byte_array(&seed).ok()?;
        if bytes.len() != 32 {
            bytes.zeroize();
            return None;
        }
        let mut owned_seed = [0_u8; 32];
        owned_seed.copy_from_slice(&bytes);
        bytes.zeroize();
        let public = device_public_from_seed(owned_seed);
        owned_seed.zeroize();
        env.byte_array_from_slice(&public)
            .ok()
            .map(jni::objects::JByteArray::into_raw)
    }));
    result.ok().flatten().unwrap_or(std::ptr::null_mut())
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_openmuse_openmuse_1mobile_PairedCryptoNative_issueOffer(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    seed: jni::objects::JByteArray,
    account_ref: jni::objects::JString,
    device_ref: jni::objects::JString,
    nonce: jni::objects::JByteArray,
    registration_generation: jni::sys::jlong,
) -> jni::sys::jbyteArray {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let account: String = env.get_string(&account_ref).ok()?.into();
        let device: String = env.get_string(&device_ref).ok()?.into();
        let mut seed_bytes = env.convert_byte_array(&seed).ok()?;
        let nonce_bytes = env.convert_byte_array(&nonce).ok()?;
        if seed_bytes.len() != 32 || nonce_bytes.len() != 32 || registration_generation <= 0 {
            seed_bytes.zeroize();
            return None;
        }
        let mut owned_seed = [0_u8; 32];
        owned_seed.copy_from_slice(&seed_bytes);
        seed_bytes.zeroize();
        let mut owned_nonce = [0_u8; 32];
        owned_nonce.copy_from_slice(&nonce_bytes);
        let offer_result = issue_pairing_offer_from_seed(
            owned_seed,
            &account,
            &device,
            owned_nonce,
            registration_generation as u64,
        );
        owned_seed.zeroize();
        let offer = offer_result.ok()?;
        let json = serde_json::to_vec(&offer).ok()?;
        env.byte_array_from_slice(&json)
            .ok()
            .map(jni::objects::JByteArray::into_raw)
    }));
    result.ok().flatten().unwrap_or(std::ptr::null_mut())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PairingOffer {
    pub account_ref: String,
    pub device_ref: String,
    pub signing_public: [u8; 32],
    pub agreement_public: [u8; 32],
    pub nonce: [u8; 32],
    pub registration_generation: u64,
    pub signature: Vec<u8>,
}

/// Trusted account-service result. Pairing never treats a device's self-claimed
/// `account_ref` as proof that it belongs to the logged-in account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceRegistration {
    pub account_ref: String,
    pub device_ref: String,
    pub signing_public: [u8; 32],
    pub agreement_public: [u8; 32],
    pub generation: u64,
    pub revoked: bool,
}

impl PairingOffer {
    pub fn issue(
        keys: &DeviceKeyPair,
        account_ref: impl Into<String>,
        device_ref: impl Into<String>,
        nonce: [u8; 32],
        registration_generation: u64,
    ) -> Result<Self, PairedRelayError> {
        let mut value = Self {
            account_ref: account_ref.into(),
            device_ref: device_ref.into(),
            signing_public: keys.signing_public(),
            agreement_public: keys.agreement_public(),
            nonce,
            registration_generation,
            signature: Vec::new(),
        };
        value.validate_fields()?;
        value.signature = keys
            .signing
            .sign(&value.signing_message())
            .to_bytes()
            .to_vec();
        Ok(value)
    }

    pub fn verify(&self) -> Result<(), PairedRelayError> {
        self.validate_fields()?;
        let verifying = VerifyingKey::from_bytes(&self.signing_public)
            .map_err(|_| PairedRelayError::InvalidOffer)?;
        let signature =
            Signature::from_slice(&self.signature).map_err(|_| PairedRelayError::InvalidOffer)?;
        verifying
            .verify(&self.signing_message(), &signature)
            .map_err(|_| PairedRelayError::InvalidOffer)
    }

    fn validate_fields(&self) -> Result<(), PairedRelayError> {
        if self.account_ref.is_empty()
            || self.device_ref.is_empty()
            || self.account_ref.len() > 256
            || self.device_ref.len() > 256
            || self.nonce == [0; 32]
            || self.registration_generation == 0
        {
            return Err(PairedRelayError::InvalidOffer);
        }
        Ok(())
    }

    fn signing_message(&self) -> Vec<u8> {
        let mut out = b"openmuse.pairing-offer.v1\0".to_vec();
        append_field(&mut out, self.account_ref.as_bytes());
        append_field(&mut out, self.device_ref.as_bytes());
        append_field(&mut out, &self.signing_public);
        append_field(&mut out, &self.agreement_public);
        append_field(&mut out, &self.nonce);
        append_field(&mut out, &self.registration_generation.to_be_bytes());
        out
    }

    fn verify_registration(
        &self,
        registration: &DeviceRegistration,
    ) -> Result<(), PairedRelayError> {
        if registration.revoked
            || registration.account_ref != self.account_ref
            || registration.device_ref != self.device_ref
            || registration.signing_public != self.signing_public
            || registration.agreement_public != self.agreement_public
            || registration.generation != self.registration_generation
        {
            return Err(PairedRelayError::PairingDenied);
        }
        Ok(())
    }
}

pub struct PairingHandshake {
    local_device_ref: String,
    remote_device_ref: String,
    transcript_hash: [u8; 32],
    shared_secret: [u8; 32],
    confirmation_code: String,
}

impl PairingHandshake {
    pub fn begin(
        local_keys: &DeviceKeyPair,
        local_offer: &PairingOffer,
        remote_offer: &PairingOffer,
        local_registration: &DeviceRegistration,
        remote_registration: &DeviceRegistration,
    ) -> Result<Self, PairedRelayError> {
        local_offer.verify()?;
        remote_offer.verify()?;
        local_offer.verify_registration(local_registration)?;
        remote_offer.verify_registration(remote_registration)?;
        if local_offer.account_ref != remote_offer.account_ref
            || local_offer.device_ref == remote_offer.device_ref
            || local_offer.signing_public != local_keys.signing_public()
            || local_offer.agreement_public != local_keys.agreement_public()
        {
            return Err(PairedRelayError::PairingDenied);
        }
        let (first, second) = ordered_offers(local_offer, remote_offer);
        let mut transcript = b"openmuse.pairing-transcript.v1\0".to_vec();
        append_field(&mut transcript, &first.signing_message());
        append_field(&mut transcript, &first.signature);
        append_field(&mut transcript, &second.signing_message());
        append_field(&mut transcript, &second.signature);
        let transcript_hash: [u8; 32] = Sha256::digest(&transcript).into();
        let remote_public = AgreementPublicKey::from(remote_offer.agreement_public);
        let shared_secret = local_keys
            .agreement
            .diffie_hellman(&remote_public)
            .to_bytes();
        if shared_secret == [0; 32] {
            return Err(PairedRelayError::PairingDenied);
        }
        let sas =
            u32::from_be_bytes(transcript_hash[..4].try_into().expect("four bytes")) % 1_000_000;
        Ok(Self {
            local_device_ref: local_offer.device_ref.clone(),
            remote_device_ref: remote_offer.device_ref.clone(),
            transcript_hash,
            shared_secret,
            confirmation_code: format!("{sas:06}"),
        })
    }

    pub fn confirmation_code(&self) -> &str {
        &self.confirmation_code
    }

    pub fn confirm(mut self, entered_code: &str) -> Result<SecureChannel, PairedRelayError> {
        if entered_code != self.confirmation_code {
            return Err(PairedRelayError::ConfirmationMismatch);
        }
        let shared_secret = self.shared_secret;
        self.shared_secret.zeroize();
        SecureChannel::derive(
            std::mem::take(&mut self.local_device_ref),
            std::mem::take(&mut self.remote_device_ref),
            self.transcript_hash,
            shared_secret,
        )
    }
}

impl Drop for PairingHandshake {
    fn drop(&mut self) {
        self.shared_secret.zeroize();
        self.confirmation_code.zeroize();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RelayEnvelope {
    pub channel_ref: String,
    pub sender_device_ref: String,
    pub recipient_device_ref: String,
    pub sequence: u64,
    pub ciphertext: Vec<u8>,
}

pub struct SecureChannel {
    channel_ref: String,
    local_device_ref: String,
    remote_device_ref: String,
    send_key: [u8; 32],
    receive_key: [u8; 32],
    send_nonce_prefix: [u8; 4],
    receive_nonce_prefix: [u8; 4],
    next_send_sequence: u64,
    next_receive_sequence: u64,
}

impl SecureChannel {
    fn derive(
        local_device_ref: String,
        remote_device_ref: String,
        transcript_hash: [u8; 32],
        mut shared_secret: [u8; 32],
    ) -> Result<Self, PairedRelayError> {
        let hkdf = Hkdf::<Sha256>::new(Some(&transcript_hash), &shared_secret);
        shared_secret.zeroize();
        let send_label = direction_label(&local_device_ref, &remote_device_ref);
        let receive_label = direction_label(&remote_device_ref, &local_device_ref);
        let mut send_material = [0_u8; 36];
        let mut receive_material = [0_u8; 36];
        hkdf.expand(&send_label, &mut send_material)
            .map_err(|_| PairedRelayError::CryptoFailure)?;
        hkdf.expand(&receive_label, &mut receive_material)
            .map_err(|_| PairedRelayError::CryptoFailure)?;
        let channel_ref = format!("paired:{}", hex_prefix(&transcript_hash, 16));
        let send_key = send_material[..32].try_into().expect("32 bytes");
        let receive_key = receive_material[..32].try_into().expect("32 bytes");
        let send_nonce_prefix = send_material[32..].try_into().expect("four bytes");
        let receive_nonce_prefix = receive_material[32..].try_into().expect("four bytes");
        send_material.zeroize();
        receive_material.zeroize();
        Ok(Self {
            channel_ref,
            local_device_ref,
            remote_device_ref,
            send_key,
            receive_key,
            send_nonce_prefix,
            receive_nonce_prefix,
            next_send_sequence: 0,
            next_receive_sequence: 0,
        })
    }

    pub fn channel_ref(&self) -> &str {
        &self.channel_ref
    }

    pub fn seal(&mut self, plaintext: &[u8]) -> Result<RelayEnvelope, PairedRelayError> {
        if plaintext.len() > MAX_RELAY_CIPHERTEXT_BYTES - 16 {
            return Err(PairedRelayError::FrameTooLarge);
        }
        let sequence = self.next_send_sequence;
        self.next_send_sequence = self
            .next_send_sequence
            .checked_add(1)
            .ok_or(PairedRelayError::SequenceExhausted)?;
        let aad = associated_data(
            &self.channel_ref,
            &self.local_device_ref,
            &self.remote_device_ref,
            sequence,
        );
        let nonce = nonce(self.send_nonce_prefix, sequence);
        let cipher = ChaCha20Poly1305::new(Key::from_slice(&self.send_key));
        let ciphertext = cipher
            .encrypt(
                Nonce::from_slice(&nonce),
                Payload {
                    msg: plaintext,
                    aad: &aad,
                },
            )
            .map_err(|_| PairedRelayError::CryptoFailure)?;
        Ok(RelayEnvelope {
            channel_ref: self.channel_ref.clone(),
            sender_device_ref: self.local_device_ref.clone(),
            recipient_device_ref: self.remote_device_ref.clone(),
            sequence,
            ciphertext,
        })
    }

    pub fn open(&mut self, envelope: &RelayEnvelope) -> Result<Vec<u8>, PairedRelayError> {
        if envelope.channel_ref != self.channel_ref
            || envelope.sender_device_ref != self.remote_device_ref
            || envelope.recipient_device_ref != self.local_device_ref
            || envelope.sequence != self.next_receive_sequence
            || envelope.ciphertext.len() > MAX_RELAY_CIPHERTEXT_BYTES
        {
            return Err(PairedRelayError::EnvelopeDenied);
        }
        let aad = associated_data(
            &envelope.channel_ref,
            &envelope.sender_device_ref,
            &envelope.recipient_device_ref,
            envelope.sequence,
        );
        let nonce = nonce(self.receive_nonce_prefix, envelope.sequence);
        let cipher = ChaCha20Poly1305::new(Key::from_slice(&self.receive_key));
        let plaintext = cipher
            .decrypt(
                Nonce::from_slice(&nonce),
                Payload {
                    msg: &envelope.ciphertext,
                    aad: &aad,
                },
            )
            .map_err(|_| PairedRelayError::EnvelopeDenied)?;
        self.next_receive_sequence = self
            .next_receive_sequence
            .checked_add(1)
            .ok_or(PairedRelayError::SequenceExhausted)?;
        Ok(plaintext)
    }
}

impl Drop for SecureChannel {
    fn drop(&mut self) {
        self.send_key.zeroize();
        self.receive_key.zeroize();
        self.send_nonce_prefix.zeroize();
        self.receive_nonce_prefix.zeroize();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RelayConnection {
    account_ref: String,
    generation: u64,
    queue: VecDeque<RelayEnvelope>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct RelayMetrics {
    pub routed_envelopes: u64,
    pub routed_ciphertext_bytes: u64,
    pub rejected_envelopes: u64,
}

#[derive(Default)]
pub struct OpaqueRelay {
    connections: HashMap<String, RelayConnection>,
    metrics: RelayMetrics,
}

impl OpaqueRelay {
    /// Represents a device-owned outbound connection. No inbound Desktop
    /// listener or Workspace credential is registered with the relay.
    pub fn connect_outbound(
        &mut self,
        account_ref: impl Into<String>,
        device_ref: impl Into<String>,
        generation: u64,
    ) -> Result<(), PairedRelayError> {
        let account_ref = account_ref.into();
        let device_ref = device_ref.into();
        if account_ref.is_empty() || device_ref.is_empty() || generation == 0 {
            return Err(PairedRelayError::RelayDenied);
        }
        if self
            .connections
            .get(&device_ref)
            .is_some_and(|connection| connection.generation >= generation)
        {
            return Err(PairedRelayError::RelayDenied);
        }
        self.connections.insert(
            device_ref,
            RelayConnection {
                account_ref,
                generation,
                queue: VecDeque::new(),
            },
        );
        Ok(())
    }

    pub fn disconnect(&mut self, device_ref: &str, generation: u64) {
        if self
            .connections
            .get(device_ref)
            .is_some_and(|connection| connection.generation == generation)
        {
            self.connections.remove(device_ref);
        }
    }

    pub fn route(
        &mut self,
        sender_account_ref: &str,
        sender_generation: u64,
        envelope: RelayEnvelope,
    ) -> Result<(), PairedRelayError> {
        let sender_valid = self
            .connections
            .get(&envelope.sender_device_ref)
            .is_some_and(|connection| {
                connection.account_ref == sender_account_ref
                    && connection.generation == sender_generation
            });
        let recipient_valid = self
            .connections
            .get(&envelope.recipient_device_ref)
            .is_some_and(|connection| connection.account_ref == sender_account_ref);
        if !sender_valid
            || !recipient_valid
            || envelope.sender_device_ref == envelope.recipient_device_ref
            || envelope.channel_ref.is_empty()
            || envelope.ciphertext.is_empty()
            || envelope.ciphertext.len() > MAX_RELAY_CIPHERTEXT_BYTES
        {
            self.metrics.rejected_envelopes += 1;
            return Err(PairedRelayError::RelayDenied);
        }
        let size = envelope.ciphertext.len() as u64;
        self.connections
            .get_mut(&envelope.recipient_device_ref)
            .expect("recipient checked")
            .queue
            .push_back(envelope);
        self.metrics.routed_envelopes += 1;
        self.metrics.routed_ciphertext_bytes += size;
        Ok(())
    }

    pub fn receive(
        &mut self,
        account_ref: &str,
        device_ref: &str,
        generation: u64,
    ) -> Result<Option<RelayEnvelope>, PairedRelayError> {
        let connection = self
            .connections
            .get_mut(device_ref)
            .ok_or(PairedRelayError::RelayDenied)?;
        if connection.account_ref != account_ref || connection.generation != generation {
            return Err(PairedRelayError::RelayDenied);
        }
        Ok(connection.queue.pop_front())
    }

    pub fn metrics(&self) -> &RelayMetrics {
        &self.metrics
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum PairedRelayError {
    #[error("invalid pairing offer")]
    InvalidOffer,
    #[error("pairing denied")]
    PairingDenied,
    #[error("confirmation code mismatch")]
    ConfirmationMismatch,
    #[error("workspace grant denied")]
    GrantDenied,
    #[error("relay denied")]
    RelayDenied,
    #[error("envelope denied")]
    EnvelopeDenied,
    #[error("frame too large")]
    FrameTooLarge,
    #[error("sequence exhausted")]
    SequenceExhausted,
    #[error("cryptographic operation failed")]
    CryptoFailure,
}

fn ordered_offers<'a>(
    a: &'a PairingOffer,
    b: &'a PairingOffer,
) -> (&'a PairingOffer, &'a PairingOffer) {
    if a.device_ref < b.device_ref {
        (a, b)
    } else {
        (b, a)
    }
}

fn append_field(out: &mut Vec<u8>, value: &[u8]) {
    out.extend_from_slice(&(value.len() as u64).to_be_bytes());
    out.extend_from_slice(value);
}

fn domain_hash(domain: &[u8], value: &[u8]) -> [u8; 32] {
    let mut hash = Sha256::new();
    hash.update(domain);
    hash.update([0]);
    hash.update(value);
    hash.finalize().into()
}

fn direction_label(sender: &str, recipient: &str) -> Vec<u8> {
    let mut value = b"openmuse.paired-channel.direction.v1\0".to_vec();
    append_field(&mut value, sender.as_bytes());
    append_field(&mut value, recipient.as_bytes());
    value
}

fn associated_data(channel_ref: &str, sender: &str, recipient: &str, sequence: u64) -> Vec<u8> {
    let mut value = b"openmuse.relay-envelope.v1\0".to_vec();
    append_field(&mut value, channel_ref.as_bytes());
    append_field(&mut value, sender.as_bytes());
    append_field(&mut value, recipient.as_bytes());
    append_field(&mut value, &sequence.to_be_bytes());
    value
}

fn nonce(prefix: [u8; 4], sequence: u64) -> [u8; 12] {
    let mut value = [0_u8; 12];
    value[..4].copy_from_slice(&prefix);
    value[4..].copy_from_slice(&sequence.to_be_bytes());
    value
}

fn hex_prefix(value: &[u8], bytes: usize) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes * 2);
    for byte in value.iter().take(bytes) {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}
