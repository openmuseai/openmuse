use super::CipherParameters::CipherParameters;

// Java: public class KeyParameter implements CipherParameters
pub struct KeyParameter {
    // Java field: byte[] key
    key: Vec<u8>,
}

// Java: public class KeyParameter implements CipherParameters
impl KeyParameter {
    // Java: KeyParameter(byte[] key)
    pub fn new(key: &[u8]) -> Self {
        // Java: this(key, 0, key.length);
        Self::new_with_offset(key, 0, key.len() as i32)
    }

    // Java: KeyParameter(byte[] key, int keyOff, int keyLen)
    pub fn new_with_offset(key: &[u8], keyOff: i32, keyLen: i32) -> Self {
        // Java: super();
        // Java: this.key = new byte[keyLen];
        let mut k = vec![0u8; keyLen as usize];
        // Java: System.arraycopy(key, keyOff, this.key, 0, keyLen);
        k.copy_from_slice(&key[keyOff as usize..(keyOff + keyLen) as usize]);
        KeyParameter { key: k }
    }

    // Java: byte[] getKey()
    pub fn getKey(&self) -> &[u8] {
        // Java: return this.key;
        &self.key
    }
}

impl CipherParameters for KeyParameter {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
