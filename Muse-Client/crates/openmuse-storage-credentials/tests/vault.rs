use futures::executor::block_on;
use openmuse_storage_contract::StorageErrorCode;
use openmuse_storage_credentials::{CredentialVaultPort, InMemoryCredentialVault, SecretBytes};

#[test]
fn leases_are_audience_bound_rotatable_revocable_and_redacted() {
    block_on(async {
        let vault = InMemoryCredentialVault::default();
        assert_eq!(
            vault
                .rotate(
                    "credential.primary",
                    "adapter.s3",
                    "access-one",
                    SecretBytes::new(b"secret-one".to_vec()).unwrap(),
                    Some(SecretBytes::new(b"token-one".to_vec()).unwrap()),
                )
                .unwrap(),
            1
        );
        let first = vault
            .lease("credential.primary", "adapter.s3", 1000, 100)
            .await
            .unwrap();
        let debug = format!("{first:?}");
        assert!(!debug.contains("access-one"));
        assert!(!debug.contains("secret-one"));
        assert!(!debug.contains("token-one"));
        first.ensure_live(1099, 1).unwrap();
        assert_eq!(
            first.ensure_live(1100, 1).unwrap_err().code,
            StorageErrorCode::Expired
        );
        assert_eq!(
            vault
                .lease("credential.primary", "adapter.other", 1000, 100)
                .await
                .unwrap_err()
                .code,
            StorageErrorCode::Denied
        );

        assert_eq!(
            vault
                .rotate(
                    "credential.primary",
                    "adapter.s3",
                    "access-two",
                    SecretBytes::new(b"secret-two".to_vec()).unwrap(),
                    None,
                )
                .unwrap(),
            2
        );
        let second = vault
            .lease("credential.primary", "adapter.s3", 1000, 100)
            .await
            .unwrap();
        assert_eq!(second.generation, 2);
        assert_eq!(
            first.ensure_live(1001, second.generation).unwrap_err().code,
            StorageErrorCode::StaleGeneration
        );
        vault.revoke("credential.primary").unwrap();
        assert_eq!(
            vault
                .lease("credential.primary", "adapter.s3", 1000, 100)
                .await
                .unwrap_err()
                .code,
            StorageErrorCode::Denied
        );
    });
}
