use openmuse_paired_relay::{
    DEVICE_PUBLIC_BYTES, DeviceKeyPair, openmuse_paired_abi_version, openmuse_paired_device_public,
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
