use openmuse_storage_contract::{
    AddressingStyle, BlobDigest, MIN_MULTIPART_PART_BYTES, PROFILE_MAJOR, PROFILE_NAME,
    ProviderCapabilitySnapshot, ProviderKind, StorageErrorCode,
};

#[test]
fn digest_is_lowercase_sha256_not_an_etag_shape() {
    assert!(BlobDigest::sha256("a".repeat(64)).is_ok());
    let error = BlobDigest::sha256("A".repeat(64)).expect_err("uppercase digest must fail");
    assert_eq!(error.code, StorageErrorCode::Denied);
    assert!(BlobDigest::sha256("opaque-etag").is_err());
}

#[test]
fn profile_requires_the_bounded_openmuse_s3_subset() {
    let mut snapshot = ProviderCapabilitySnapshot {
        profile: PROFILE_NAME.to_owned(),
        profile_major: PROFILE_MAJOR,
        provider_ref: "provider.fixture".to_owned(),
        provider_kind: ProviderKind::Minio,
        provider_version: "fixture".to_owned(),
        tls: true,
        addressing_styles: vec![AddressingStyle::Path],
        checksum_sha256: true,
        range_read: true,
        multipart: true,
        maintenance_list: true,
        minimum_part_size: MIN_MULTIPART_PART_BYTES,
    };
    snapshot.validate_profile_v1().expect("complete profile");
    snapshot.checksum_sha256 = false;
    let error = snapshot
        .validate_profile_v1()
        .expect_err("missing checksum must fail");
    assert_eq!(error.code, StorageErrorCode::Unavailable);
}

#[test]
fn storage_error_codes_stay_within_the_cross_domain_vocabulary() {
    let values = [
        StorageErrorCode::Denied,
        StorageErrorCode::NotFound,
        StorageErrorCode::Conflict,
        StorageErrorCode::Expired,
        StorageErrorCode::StaleGeneration,
        StorageErrorCode::Unavailable,
        StorageErrorCode::Transient,
        StorageErrorCode::IntegrityFailed,
    ];
    assert_eq!(values.len(), 8);
}
