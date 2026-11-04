// Java: import java.nio.ByteBuffer
// Java: import java.security.Key
// Java: import javax.crypto.SecretKey
// Java: import javax.crypto.spec.IvParameterSpec
// Java: import javax.crypto.spec.SecretKeySpec

use super::RC4Engine::RC4Engine;
use super::KeyParameter::KeyParameter;
use super::StreamCipher::StreamCipher;

// Java: public class RC4Cipher
pub struct RC4Cipher {
    // Java field: RC4Engine rc4
    rc4: Option<RC4Engine>,
}

// Java: public class RC4Cipher
impl RC4Cipher {
    // Java: RC4Cipher()
    pub fn new() -> Self {
        // Java: super();
        RC4Cipher { rc4: None }
    }

    // Java: void doFinal(ByteBuffer nio, ByteBuffer decryptedBuf)
    pub fn doFinal_buffers(&mut self, nio: &[u8], decryptedBuf: &mut Vec<u8>) {
        let rc4 = self.rc4.as_mut().unwrap();
        // Java: while(nio.hasRemaining())
        for &b in nio.iter() {
            // Java: decryptedBuf.put(this.rc4.returnByte(nio.get()));
            decryptedBuf.push(rc4.returnByte(b));
        }
    }

    // Java: byte[] doFinal(byte[] input)
    pub fn doFinal_bytes(&mut self, input: &[u8]) -> Vec<u8> {
        // Java: byte[] result = new byte[input.length];
        let mut result = vec![0u8; input.len()];
        // Java: this.rc4.processBytes(input, 0, input.length, result, 0);
        self.rc4.as_mut().unwrap().processBytes(input, 0, input.len() as i32, &mut result, 0);
        // Java: return result;
        result
    }

    // Java: void doFinal(byte[] src, int from, int length, byte[] dest)
    pub fn doFinal_range(&mut self, src: &[u8], from: i32, length: i32, dest: &mut [u8]) {
        // Java: this.rc4.processBytes(src, from, length, dest, 0);
        self.rc4.as_mut().unwrap().processBytes(src, from, length, dest, 0);
    }

    // Java: void init(int mode, Key key)
    pub fn init_with_key(&mut self, mode: i32, key: &[u8]) {
        // Java: this.rc4 = new RC4Engine();
        self.rc4 = Some(RC4Engine::new());
        // Java: this.rc4.init(mode == 1, new KeyParameter(key.getEncoded()));
        self.rc4.as_mut().unwrap().init(mode == 1, &KeyParameter::new(key));
    }

    // Java: void init(int mode, SecretKey key)
    pub fn init_with_secret_key(&mut self, mode: i32, key: &[u8]) {
        // Java: this.rc4 = new RC4Engine();
        self.rc4 = Some(RC4Engine::new());
        // Java: this.rc4.init(mode == 1, new KeyParameter(key.getEncoded()));
        self.rc4.as_mut().unwrap().init(mode == 1, &KeyParameter::new(key));
    }

    // Java: void init(int mode, SecretKeySpec keySpec)
    pub fn init_with_key_spec(&mut self, mode: i32, keySpec: &[u8]) {
        // Java: this.rc4 = new RC4Engine();
        self.rc4 = Some(RC4Engine::new());
        // Java: this.rc4.init(mode == 1, new KeyParameter(keySpec.getEncoded()));
        self.rc4.as_mut().unwrap().init(mode == 1, &KeyParameter::new(keySpec));
    }

    // Java: void init(int mode, SecretKeySpec keySpec, IvParameterSpec Iv)
    pub fn init_with_iv(&mut self, _mode: i32, _keySpec: &[u8], _iv: &[u8]) {
        // Java: throw new RuntimeException("not yet supported");
        panic!("not yet supported");
    }
}
