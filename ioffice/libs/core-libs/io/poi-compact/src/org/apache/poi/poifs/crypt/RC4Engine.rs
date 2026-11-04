use super::StreamCipher::StreamCipher;
use super::CipherParameters::CipherParameters;
use super::KeyParameter::KeyParameter;

// Java field: int STATE_LENGTH
const STATE_LENGTH: usize = 256;

// Java: public class RC4Engine implements StreamCipher
pub struct RC4Engine {
    // Java field: byte[] engineState
    engineState: Option<Vec<u8>>,
    // Java field: int x
    x: i32,
    // Java field: int y
    y: i32,
    // Java field: byte[] workingKey
    workingKey: Option<Vec<u8>>,
}

// Java: public class RC4Engine implements StreamCipher
impl RC4Engine {
    // Java: RC4Engine()
    pub fn new() -> Self {
        // Java: super();
        RC4Engine {
            engineState: None,
            x: 0,
            y: 0,
            workingKey: None,
        }
    }

    // Java: void init(boolean forEncryption, CipherParameters params)
    pub fn init_cipher(&mut self, _forEncryption: bool, params: &dyn CipherParameters) {
        // Java: if (params instanceof KeyParameter)
        if let Some(key_param) = params.as_any().downcast_ref::<KeyParameter>() {
            // Java: this.workingKey = ((KeyParameter)params).getKey();
            self.workingKey = Some(key_param.getKey().to_vec());
            // Java: this.setKey(this.workingKey);
            let key = self.workingKey.clone().unwrap();
            self.setKey(&key);
        }
        // Java: else
        else {
            // Java: throw new IllegalArgumentException("invalid parameter passed to RC4 init - " + params.getClass().getName());
            panic!("invalid parameter passed to RC4 init");
        }
    }

    // Java: String getAlgorithmName()
    pub fn getAlgorithmName(&self) -> String {
        // Java: return "RC4";
        "RC4".to_string()
    }

    // Java: byte returnByte(byte in)
    pub fn returnByte(&mut self, input: u8) -> u8 {
        // Java: this.x = this.x + 1 & 255;
        self.x = (self.x + 1) & 255;
        let es = self.engineState.as_mut().unwrap();
        // Java: this.y = this.engineState[this.x] + this.y & 255;
        self.y = ((es[self.x as usize] as i32) + self.y) & 255;
        // Java: byte tmp = this.engineState[this.x];
        let tmp = es[self.x as usize];
        // Java: this.engineState[this.x] = this.engineState[this.y];
        es[self.x as usize] = es[self.y as usize];
        // Java: this.engineState[this.y] = tmp;
        es[self.y as usize] = tmp;
        // Java: return (byte)(in ^ this.engineState[this.engineState[this.x] + this.engineState[this.y] & 255]);
        let idx = ((es[self.x as usize] as i32) + (es[self.y as usize] as i32)) & 255;
        input ^ es[idx as usize]
    }

    // Java: void processBytes(byte[] in, int inOff, int len, byte[] out, int outOff)
    pub fn processBytes(&mut self, input: &[u8], inOff: i32, len: i32, out: &mut [u8], outOff: i32) {
        // Java: if (inOff + len > in.length)
        if (inOff + len) as usize > input.len() {
            // Java: throw new DataLengthException("input buffer too short");
            panic!("input buffer too short");
        }
        // Java: else if (outOff + len > out.length)
        else if (outOff + len) as usize > out.len() {
            // Java: throw new DataLengthException("output buffer too short");
            panic!("output buffer too short");
        }
        // Java: else
        else {
            // Java: for(int i = 0; i < len; ++i)
            for i in 0..len {
                // Java: this.x = this.x + 1 & 255;
                self.x = (self.x + 1) & 255;
                let es = self.engineState.as_mut().unwrap();
                // Java: this.y = this.engineState[this.x] + this.y & 255;
                self.y = ((es[self.x as usize] as i32) + self.y) & 255;
                // Java: byte tmp = this.engineState[this.x];
                let tmp = es[self.x as usize];
                // Java: this.engineState[this.x] = this.engineState[this.y];
                es[self.x as usize] = es[self.y as usize];
                // Java: this.engineState[this.y] = tmp;
                es[self.y as usize] = tmp;
                // Java: out[i + outOff] = (byte)(in[i + inOff] ^ this.engineState[this.engineState[this.x] + this.engineState[this.y] & 255]);
                let idx = ((es[self.x as usize] as i32) + (es[self.y as usize] as i32)) & 255;
                out[(i + outOff) as usize] = input[(i + inOff) as usize] ^ es[idx as usize];
            }
        }
    }

    // Java: void reset()
    pub fn reset_impl(&mut self) {
        // Java: this.setKey(this.workingKey);
        let key = self.workingKey.clone().unwrap();
        self.setKey(&key);
    }

    // Java: void setKey(byte[] keyBytes)
    fn setKey(&mut self, keyBytes: &[u8]) {
        // Java: this.workingKey = keyBytes;
        self.workingKey = Some(keyBytes.to_vec());
        // Java: this.x = 0;
        self.x = 0;
        // Java: this.y = 0;
        self.y = 0;
        // Java: if (this.engineState == null)
        if self.engineState.is_none() {
            // Java: this.engineState = new byte[256];
            self.engineState = Some(vec![0u8; 256]);
        }
        // Java: for(int i = 0; i < 256; ++i)
        let es = self.engineState.as_mut().unwrap();
        for i in 0..256usize {
            // Java: this.engineState[i] = (byte)i;
            es[i] = i as u8;
        }
        // Java: int i1 = 0;
        let mut i1: usize = 0;
        // Java: int i2 = 0;
        let mut i2: usize = 0;
        // Java: for(int i = 0; i < 256; ++i)
        for i in 0..256usize {
            // Java: i2 = (keyBytes[i1] & 255) + this.engineState[i] + i2 & 255;
            i2 = ((keyBytes[i1] as usize & 255) + (es[i] as usize) + i2) & 255;
            // Java: byte tmp = this.engineState[i];
            let tmp = es[i];
            // Java: this.engineState[i] = this.engineState[i2];
            es[i] = es[i2];
            // Java: this.engineState[i2] = tmp;
            es[i2] = tmp;
            // Java: i1 = (i1 + 1) % keyBytes.length;
            i1 = (i1 + 1) % keyBytes.len();
        }
    }
}

impl StreamCipher for RC4Engine {
    fn init(&mut self, var1: bool, var2: &dyn CipherParameters) {
        self.init_cipher(var1, var2);
    }
    fn get_algorithm_name(&self) -> String {
        self.getAlgorithmName()
    }
    fn return_byte(&mut self, var1: u8) -> u8 {
        self.returnByte(var1)
    }
    fn process_bytes(&mut self, var1: &[u8], var2: i32, var3: i32, var4: &mut [u8], var5: i32) {
        self.processBytes(var1, var2, var3, var4, var5);
    }
    fn reset(&mut self) {
        self.reset_impl();
    }
}
