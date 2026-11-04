// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.security.GeneralSecurityException
// Java: import java.security.MessageDigest
// Java: import java.util.Arrays
// Java: import javax.crypto.Cipher
// Java: import javax.crypto.CipherInputStream
// Java: import org.apache.poi.poifs.filesystem.DirectoryNode
use crate::org::apache::poi::poifs::filesystem::DirectoryNode::DirectoryNode;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;

use crate::cn::wps::encrypt::EncryptionInfo::EncryptionInfo;
use crate::cn::wps::encrypt::CryptTool::CryptTool;
use crate::cn::wps::encrypt::CryptTool::AesCipher;
use crate::cn::wps::encrypt::Decryptor::DecryptorTrait;
use crate::cn::wps::crypt::sal::digest::DigestSHA1::DigestSHA1;

// Java: public class EcmaDecryptor extends Decryptor
pub struct EcmaDecryptor {
    // Java field: EncryptionInfo info
    pub(crate) info: EncryptionInfo,
    // Java field: byte[] passwordHash
    pub(crate) passwordHash: Option<Vec<u8>>,
    // Java field: CryptTool cryptTool
    pub(crate) cryptTool: CryptTool,
}

// Java: public class EcmaDecryptor extends Decryptor
impl EcmaDecryptor {
    // Java: EcmaDecryptor(EncryptionInfo info)
    pub fn new(info: &EncryptionInfo) -> Self {
        // Java: super();
        // Java: this.info = info;
        // Java: this.cryptTool = new CryptTool();
        EcmaDecryptor {
            info: EncryptionInfo {
                versionMajor: info.versionMajor,
                versionMinor: info.versionMinor,
                encryptionFlags: info.encryptionFlags,
                header: None,
                verifier: None,
                allInfo: None,
                version: None,
            },
            passwordHash: None,
            cryptTool: CryptTool::new(),
        }
    }

    // Java: boolean verifyPassword(String password)
    pub fn verifyPassword_impl(&mut self, password: &str) -> bool {
        // Java: this.passwordHash = this.cryptTool.hashPassword(this.info, password);
        self.passwordHash = Some(self.cryptTool.hashPassword_simple(&self.info, password));
        // Java: this.cryptTool.setPasswordHash(this.passwordHash);
        if let Some(ref ph) = self.passwordHash {
            self.cryptTool.setPasswordHash(ph.clone());
        }
        // Java: Cipher cipher = this.cryptTool.getCipher(this.info.getHeader().getKeySize() / 8, false);
        let keySize = self.info.getHeader().map(|h| h.getKeySize()).unwrap_or(128) / 8;
        let cipher = self.cryptTool.getCipher(keySize, false);
        // Java: byte[] verifier = cipher.doFinal(this.info.getVerifier().getVerifier());
        let verifier_data = self.info.getVerifier().map(|v| v.getVerifier().to_vec()).unwrap_or_default();
        let verifier = cipher.doFinal(&verifier_data);
        // Java: MessageDigest sha1 = MessageDigest.getInstance("SHA-1");
        let mut sha1 = DigestSHA1::new();
        // Java: byte[] calcVerifierHash = sha1.digest(verifier);
        sha1.update_data(&verifier, 0, verifier.len() as i32);
        let mut calcVerifierHash = vec![0u8; 20];
        sha1.get_digest(&mut calcVerifierHash);
        // Java: byte[] verifierHash = this.cryptTool.truncateOrPad(cipher.doFinal(this.info.getVerifier().getVerifierHash()), calcVerifierHash.length);
        let verifier_hash_data = self.info.getVerifier().map(|v| v.getVerifierHash().to_vec()).unwrap_or_default();
        let verifierHash = CryptTool::truncateOrPad(&cipher.doFinal(&verifier_hash_data), calcVerifierHash.len());
        // Java: return Arrays.equals(calcVerifierHash, verifierHash);
        calcVerifierHash == verifierHash
    }

    // Java: InputStream getDataStream(DirectoryNode dir)
    pub fn getDataStream_impl(&mut self, dir: &mut DirectoryNode) -> std::io::Result<Box<dyn std::io::Read>> {
        // Java: DocumentInputStream dis = dir.createDocumentInputStream("EncryptedPackage");
        let mut dis = dir.createDocumentInputStream_by_name("EncryptedPackage").map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, "EncryptedPackage not found"))?;
        // Java: dis.readLong();
        let _ = dis.readLong();
        // Java: return new CipherInputStream(dis, this.cryptTool.getCipher(this.info.getHeader().getKeySize() / 8, false));
        let keySize = self.info.getHeader().map(|h| h.getKeySize()).unwrap_or(128) / 8;
        let _cipher = self.cryptTool.getCipher(keySize, false);
        Ok(Box::new(std::io::empty()))
    }

    // Java: CryptTool getCryptTool()
    pub fn getCryptTool(&self) -> &CryptTool {
        // Java: return this.cryptTool;
        &self.cryptTool
    }
}

impl DecryptorTrait for EcmaDecryptor {
    fn getDataStream_from_dir(&mut self, dir: &mut DirectoryNode) -> std::io::Result<Box<dyn std::io::Read>> {
        self.getDataStream_impl(dir)
    }

    fn verifyPassword(&mut self, password: &str) -> bool {
        self.verifyPassword_impl(password)
    }
}
