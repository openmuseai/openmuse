use openmuse_paired_relay::{
    DEVICE_PUBLIC_BYTES, DeviceKeyPair, PairingOffer, openmuse_paired_abi_version,
    openmuse_paired_buffer_free, openmuse_paired_device_public, openmuse_paired_issue_offer,
};

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
