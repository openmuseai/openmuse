use openmuse_paired_relay::{
    DEVICE_PUBLIC_BYTES, DeviceKeyPair, DeviceRegistration, OpenMusePairedBuffer, PairingOffer,
    issue_pairing_offer_from_seed, openmuse_paired_abi_version, openmuse_paired_begin_handshake,
    openmuse_paired_buffer_free, openmuse_paired_channel_open, openmuse_paired_channel_seal,
    openmuse_paired_confirm_handshake, openmuse_paired_device_public, openmuse_paired_issue_offer,
    openmuse_paired_native_handle_close,
};
use serde::Deserialize;

#[test]
fn ffi_exports_public_identity_without_secret_material() {
    let seed = [7_u8; 32];
    let expected = DeviceKeyPair::from_seed(seed);
    let mut output = [0_u8; DEVICE_PUBLIC_BYTES];
    let status = unsafe {
        openmuse_paired_device_public(seed.as_ptr(), seed.len(), output.as_mut_ptr(), output.len())
    };
    assert_eq!(openmuse_paired_abi_version(), 1);
    assert_eq!(status, 0);
    assert_eq!(&output[..32], &expected.signing_public());
    assert_eq!(&output[32..], &expected.agreement_public());
    assert_ne!(&output[..32], &seed);
    assert_ne!(&output[32..], &seed);
}

#[test]
fn ffi_issues_a_verifiable_offer_without_returning_the_seed() {
    let seed = [11_u8; 32];
    let account = b"account:test";
    let device = b"mobile:test";
    let nonce = [19_u8; 32];
    let buffer = unsafe {
        openmuse_paired_issue_offer(
            seed.as_ptr(),
            seed.len(),
            account.as_ptr(),
            account.len(),
            device.as_ptr(),
            device.len(),
            nonce.as_ptr(),
            nonce.len(),
            7,
        )
    };
    assert_eq!(buffer.status, 0);
    let bytes = unsafe { std::slice::from_raw_parts(buffer.ptr, buffer.len) };
    assert!(!bytes.windows(seed.len()).any(|value| value == seed));
    let offer: PairingOffer = serde_json::from_slice(bytes).expect("offer json");
    offer.verify().expect("valid signature");
    assert_eq!(offer.account_ref, "account:test");
    assert_eq!(offer.device_ref, "mobile:test");
    assert_eq!(offer.nonce, nonce);
    assert_eq!(offer.registration_generation, 7);
    openmuse_paired_buffer_free(buffer);
}

#[test]
fn ffi_rejects_wrong_lengths_and_nulls() {
    let seed = [1_u8; 32];
    let mut output = [0_u8; DEVICE_PUBLIC_BYTES];
    assert_eq!(
        unsafe {
            openmuse_paired_device_public(
                seed.as_ptr(),
                seed.len() - 1,
                output.as_mut_ptr(),
                output.len(),
            )
        },
        1
    );
    assert_eq!(
        unsafe {
            openmuse_paired_device_public(
                std::ptr::null(),
                seed.len(),
                output.as_mut_ptr(),
                output.len(),
            )
        },
        1
    );
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct HandshakeDescriptor {
    handshake_handle: u64,
    confirmation_code: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChannelDescriptor {
    channel_handle: u64,
    channel_ref: String,
}

#[test]
fn opaque_handles_complete_a_bidirectional_channel_without_exporting_keys() {
    let mobile_seed = [31_u8; 32];
    let desktop_seed = [47_u8; 32];
    let mobile_offer =
        issue_pairing_offer_from_seed(mobile_seed, "account:a", "mobile:a", [3; 32], 1)
            .expect("mobile offer");
    let desktop_offer =
        issue_pairing_offer_from_seed(desktop_seed, "account:a", "desktop:a", [5; 32], 1)
            .expect("desktop offer");
    let mobile_registration = registration(&mobile_offer);
    let desktop_registration = registration(&desktop_offer);
    let mobile_offer_json = serde_json::to_vec(&mobile_offer).unwrap();
    let desktop_offer_json = serde_json::to_vec(&desktop_offer).unwrap();
    let mobile_registration_json = serde_json::to_vec(&mobile_registration).unwrap();
    let desktop_registration_json = serde_json::to_vec(&desktop_registration).unwrap();

    let mobile_handshake = unsafe {
        openmuse_paired_begin_handshake(
            mobile_seed.as_ptr(),
            mobile_seed.len(),
            mobile_offer_json.as_ptr(),
            mobile_offer_json.len(),
            desktop_offer_json.as_ptr(),
            desktop_offer_json.len(),
            mobile_registration_json.as_ptr(),
            mobile_registration_json.len(),
            desktop_registration_json.as_ptr(),
            desktop_registration_json.len(),
        )
    };
    let desktop_handshake = unsafe {
        openmuse_paired_begin_handshake(
            desktop_seed.as_ptr(),
            desktop_seed.len(),
            desktop_offer_json.as_ptr(),
            desktop_offer_json.len(),
            mobile_offer_json.as_ptr(),
            mobile_offer_json.len(),
            desktop_registration_json.as_ptr(),
            desktop_registration_json.len(),
            mobile_registration_json.as_ptr(),
            mobile_registration_json.len(),
        )
    };
    let mobile_handshake: HandshakeDescriptor = take_json(mobile_handshake);
    let desktop_handshake: HandshakeDescriptor = take_json(desktop_handshake);
    assert_eq!(
        mobile_handshake.confirmation_code,
        desktop_handshake.confirmation_code
    );

    let code = mobile_handshake.confirmation_code.as_bytes();
    let mobile_channel: ChannelDescriptor = take_json(unsafe {
        openmuse_paired_confirm_handshake(
            mobile_handshake.handshake_handle,
            code.as_ptr(),
            code.len(),
        )
    });
    let desktop_channel: ChannelDescriptor = take_json(unsafe {
        openmuse_paired_confirm_handshake(
            desktop_handshake.handshake_handle,
            code.as_ptr(),
            code.len(),
        )
    });
    assert_eq!(mobile_channel.channel_ref, desktop_channel.channel_ref);

    let plaintext = b"bounded DSH capability response";
    let envelope = take_bytes(unsafe {
        openmuse_paired_channel_seal(
            mobile_channel.channel_handle,
            plaintext.as_ptr(),
            plaintext.len(),
        )
    });
    assert!(
        !envelope
            .windows(plaintext.len())
            .any(|value| value == plaintext)
    );
    let opened = take_bytes(unsafe {
        openmuse_paired_channel_open(
            desktop_channel.channel_handle,
            envelope.as_ptr(),
            envelope.len(),
        )
    });
    assert_eq!(opened, plaintext);

    let replay = unsafe {
        openmuse_paired_channel_open(
            desktop_channel.channel_handle,
            envelope.as_ptr(),
            envelope.len(),
        )
    };
    assert_ne!(replay.status, 0);
    openmuse_paired_buffer_free(replay);
    assert_eq!(
        openmuse_paired_native_handle_close(mobile_channel.channel_handle),
        0
    );
    assert_eq!(
        openmuse_paired_native_handle_close(desktop_channel.channel_handle),
        0
    );
}

fn registration(offer: &PairingOffer) -> DeviceRegistration {
    DeviceRegistration {
        account_ref: offer.account_ref.clone(),
        device_ref: offer.device_ref.clone(),
        signing_public: offer.signing_public,
        agreement_public: offer.agreement_public,
        generation: offer.registration_generation,
        revoked: false,
    }
}

fn take_json<T: for<'de> Deserialize<'de>>(buffer: OpenMusePairedBuffer) -> T {
    let bytes = take_bytes(buffer);
    serde_json::from_slice(&bytes).expect("valid JSON response")
}

fn take_bytes(buffer: OpenMusePairedBuffer) -> Vec<u8> {
    assert_eq!(buffer.status, 0);
    let bytes = unsafe { std::slice::from_raw_parts(buffer.ptr, buffer.len) }.to_vec();
    openmuse_paired_buffer_free(buffer);
    bytes
}
