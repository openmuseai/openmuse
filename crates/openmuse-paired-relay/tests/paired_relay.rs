use openmuse_paired_relay::{
    DeviceKeyPair, DevicePresence, DeviceRegistration, MAX_RELAY_CIPHERTEXT_BYTES, OpaqueRelay,
    PairedPermission, PairedRelayError, PairingHandshake, PairingOffer, WorkspaceGrant,
};

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

fn paired_channels() -> (
    openmuse_paired_relay::SecureChannel,
    openmuse_paired_relay::SecureChannel,
) {
    let mobile_keys = DeviceKeyPair::from_seed([7; 32]);
    let desktop_keys = DeviceKeyPair::from_seed([9; 32]);
    let mobile_offer =
        PairingOffer::issue(&mobile_keys, "account:a", "mobile:1", [11; 32], 1).unwrap();
    let desktop_offer =
        PairingOffer::issue(&desktop_keys, "account:a", "desktop:1", [13; 32], 1).unwrap();
    let mobile = PairingHandshake::begin(
        &mobile_keys,
        &mobile_offer,
        &desktop_offer,
        &registration(&mobile_offer),
        &registration(&desktop_offer),
    )
    .unwrap();
    let desktop = PairingHandshake::begin(
        &desktop_keys,
        &desktop_offer,
        &mobile_offer,
        &registration(&desktop_offer),
        &registration(&mobile_offer),
    )
    .unwrap();
    assert_eq!(mobile.confirmation_code(), desktop.confirmation_code());
    let code = mobile.confirmation_code().to_owned();
    (
        mobile.confirm(&code).unwrap(),
        desktop.confirm(&code).unwrap(),
    )
}

#[test]
fn signed_manual_pairing_derives_directional_e2e_channels() {
    let (mut mobile, mut desktop) = paired_channels();
    assert_eq!(mobile.channel_ref(), desktop.channel_ref());
    let request = mobile.seal(b"workspace.read:secret.txt").unwrap();
    assert!(
        !request
            .ciphertext
            .windows(b"secret.txt".len())
            .any(|window| window == b"secret.txt")
    );
    assert_eq!(
        desktop.open(&request).unwrap(),
        b"workspace.read:secret.txt"
    );
    let response = desktop.seal(b"private contents").unwrap();
    assert_eq!(mobile.open(&response).unwrap(), b"private contents");
}

#[test]
fn tamper_replay_wrong_route_and_out_of_order_fail_closed() {
    let (mut mobile, mut desktop) = paired_channels();
    let first = mobile.seal(b"first").unwrap();
    let second = mobile.seal(b"second").unwrap();
    assert_eq!(desktop.open(&second), Err(PairedRelayError::EnvelopeDenied));
    let mut tampered = first.clone();
    tampered.ciphertext[0] ^= 1;
    assert_eq!(
        desktop.open(&tampered),
        Err(PairedRelayError::EnvelopeDenied)
    );
    assert_eq!(desktop.open(&first).unwrap(), b"first");
    assert_eq!(desktop.open(&first), Err(PairedRelayError::EnvelopeDenied));
    assert_eq!(desktop.open(&second).unwrap(), b"second");
}

#[test]
fn relay_routes_only_opaque_ciphertext_over_outbound_connections() {
    let (mut mobile, mut desktop) = paired_channels();
    let mut relay = OpaqueRelay::default();
    relay.connect_outbound("account:a", "mobile:1", 1).unwrap();
    relay.connect_outbound("account:a", "desktop:1", 4).unwrap();
    let envelope = mobile.seal(b"never visible to relay").unwrap();
    relay.route("account:a", 1, envelope.clone()).unwrap();
    let routed = relay.receive("account:a", "desktop:1", 4).unwrap().unwrap();
    assert_eq!(routed, envelope);
    assert_eq!(desktop.open(&routed).unwrap(), b"never visible to relay");
    assert_eq!(relay.metrics().routed_envelopes, 1);
    assert_eq!(
        relay.metrics().routed_ciphertext_bytes as usize,
        envelope.ciphertext.len()
    );
    relay.disconnect("desktop:1", 4);
    assert_eq!(
        relay.route("account:a", 1, mobile.seal(b"offline").unwrap()),
        Err(PairedRelayError::RelayDenied)
    );
}

#[test]
fn same_account_never_replaces_pairing_and_scoped_grant() {
    let grant = WorkspaceGrant {
        grant_ref: "grant:1".into(),
        account_ref: "account:a".into(),
        desktop_device_ref: "desktop:1".into(),
        mobile_device_ref: "mobile:1".into(),
        workspace_ref: "local:w1".into(),
        permissions: vec![PairedPermission::Read, PairedPermission::Propose],
        generation: 3,
        expires_at_ms: 100,
        revoked: false,
    };
    assert!(
        grant
            .authorize(
                "account:a",
                "mobile:1",
                "desktop:1",
                "local:w1",
                PairedPermission::Read,
                3,
                99,
                DevicePresence::Online,
            )
            .is_ok()
    );
    for (workspace, permission, generation, now, presence) in [
        (
            "local:w2",
            PairedPermission::Read,
            3,
            99,
            DevicePresence::Online,
        ),
        (
            "local:w1",
            PairedPermission::Apply,
            3,
            99,
            DevicePresence::Online,
        ),
        (
            "local:w1",
            PairedPermission::Read,
            2,
            99,
            DevicePresence::Online,
        ),
        (
            "local:w1",
            PairedPermission::Read,
            3,
            100,
            DevicePresence::Online,
        ),
        (
            "local:w1",
            PairedPermission::Read,
            3,
            99,
            DevicePresence::Sleeping,
        ),
    ] {
        assert_eq!(
            grant.authorize(
                "account:a",
                "mobile:1",
                "desktop:1",
                workspace,
                permission,
                generation,
                now,
                presence,
            ),
            Err(PairedRelayError::GrantDenied)
        );
    }
}

#[test]
fn frame_bound_forces_range_streaming_and_backpressure() {
    let (mut mobile, _) = paired_channels();
    assert_eq!(
        mobile.seal(&vec![0; MAX_RELAY_CIPHERTEXT_BYTES]),
        Err(PairedRelayError::FrameTooLarge)
    );
    let maximum_plaintext = vec![0; MAX_RELAY_CIPHERTEXT_BYTES - 16];
    assert_eq!(
        mobile.seal(&maximum_plaintext).unwrap().ciphertext.len(),
        MAX_RELAY_CIPHERTEXT_BYTES
    );
}

#[test]
fn forged_offer_and_wrong_confirmation_are_rejected() {
    let mobile_keys = DeviceKeyPair::from_seed([1; 32]);
    let desktop_keys = DeviceKeyPair::from_seed([2; 32]);
    let mobile_offer =
        PairingOffer::issue(&mobile_keys, "account:a", "mobile:1", [3; 32], 1).unwrap();
    let mut forged =
        PairingOffer::issue(&desktop_keys, "account:a", "desktop:1", [4; 32], 1).unwrap();
    forged.device_ref = "desktop:attacker".into();
    assert!(
        PairingHandshake::begin(
            &mobile_keys,
            &mobile_offer,
            &forged,
            &registration(&mobile_offer),
            &registration(&forged),
        )
        .is_err()
    );

    let desktop_offer =
        PairingOffer::issue(&desktop_keys, "account:a", "desktop:1", [4; 32], 1).unwrap();
    let handshake = PairingHandshake::begin(
        &mobile_keys,
        &mobile_offer,
        &desktop_offer,
        &registration(&mobile_offer),
        &registration(&desktop_offer),
    )
    .unwrap();
    assert!(matches!(
        handshake.confirm("000000"),
        Err(PairedRelayError::ConfirmationMismatch)
    ));
}

#[test]
fn self_claimed_same_account_without_trusted_registration_is_rejected() {
    let mobile_keys = DeviceKeyPair::from_seed([21; 32]);
    let desktop_keys = DeviceKeyPair::from_seed([22; 32]);
    let mobile_offer =
        PairingOffer::issue(&mobile_keys, "account:a", "mobile:1", [23; 32], 1).unwrap();
    let desktop_offer =
        PairingOffer::issue(&desktop_keys, "account:a", "desktop:1", [24; 32], 1).unwrap();
    let mut untrusted = registration(&desktop_offer);
    untrusted.account_ref = "account:other".into();
    assert!(matches!(
        PairingHandshake::begin(
            &mobile_keys,
            &mobile_offer,
            &desktop_offer,
            &registration(&mobile_offer),
            &untrusted,
        ),
        Err(PairedRelayError::PairingDenied)
    ));
}
