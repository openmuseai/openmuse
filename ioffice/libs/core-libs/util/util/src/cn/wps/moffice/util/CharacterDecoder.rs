#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import java.io.ByteArrayInputStream
// Java: import java.io.ByteArrayOutputStream
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.OutputStream
// Java: import java.io.PushbackInputStream
// Java: import java.nio.ByteBuffer

use std::io::{Read, Write, Cursor};

// Java: public class CharacterDecoder
pub trait CharacterDecoder {
    // Java: CharacterDecoder()
    // Java: super();

    // Java: abstract int bytesPerAtom()
    fn bytesPerAtom(&self) -> i32;

    // Java: abstract int bytesPerLine()
    fn bytesPerLine(&self) -> i32;

    // Java: void decodeBufferPrefix(PushbackInputStream aStream, OutputStream bStream)
    fn decodeBufferPrefix(&self, _a_stream: &mut dyn Read, _b_stream: &mut dyn Write) {
    }

    // Java: void decodeBufferSuffix(PushbackInputStream aStream, OutputStream bStream)
    fn decodeBufferSuffix(&self, _a_stream: &mut dyn Read, _b_stream: &mut dyn Write) {
    }

    // Java: int decodeLinePrefix(PushbackInputStream aStream, OutputStream bStream)
    fn decodeLinePrefix(&self, _a_stream: &mut dyn Read, _b_stream: &mut dyn Write) -> i32 {
        // Java: return this.bytesPerLine();
        self.bytesPerLine()
    }

    // Java: void decodeLineSuffix(PushbackInputStream aStream, OutputStream bStream)
    fn decodeLineSuffix(&self, _a_stream: &mut dyn Read, _b_stream: &mut dyn Write) {
    }

    // Java: void decodeAtom(PushbackInputStream aStream, OutputStream bStream, int l)
    fn decodeAtom(&self, _a_stream: &mut dyn Read, _b_stream: &mut dyn Write, _l: i32) {
        // Java: throw new IOException();
        panic!("IOException");
    }

    // Java: int readFully(InputStream in, _buffer: &[u8], int offset, int len)
    fn readFully(&self, input: &mut dyn Read, buffer: &mut [u8], offset: usize, len: usize) -> i32 {
        // Java: for(int i = 0; i < len; ++i)
        for i in 0..len
        {
            // Java: int q = in.read();
            let mut single = [0u8; 1];
            let read_result = input.read(&mut single);
            let q = match read_result {
                Ok(0) => -1i32,
                Ok(_) => single[0] as i32,
                Err(_) => -1i32,
            };
            // Java: if (q == -1)
            if q == -1
            {
                // Java: return i == 0 ? -1 : i;
                return if i == 0 { -1 } else { i as i32 };
            }
            // Java: buffer[i + offset] = (byte)q;
            buffer[i + offset] = q as u8;
        }
        // Java: return len;
        len as i32
    }

    // Java: void decodeBuffer(InputStream aStream, OutputStream bStream)
    fn decodeBuffer_stream(&self, a_stream: &mut dyn Read, b_stream: &mut dyn Write) {
        // Java: int totalBytes = 0;
        let mut total_bytes = 0i32;
        // Java: PushbackInputStream ps = new PushbackInputStream(aStream);
        // Java: this.decodeBufferPrefix(ps, bStream);
        // Java: while(true)
        // Java: try
        // Java: int length = this.decodeLinePrefix(ps, bStream);
        // Java: int i;
        // Java: for(i = 0; i + this.bytesPerAtom() < length; i += this.bytesPerAtom())
        // Java: this.decodeAtom(ps, bStream, this.bytesPerAtom());
        // Java: totalBytes += this.bytesPerAtom();
        // Java: if (i + this.bytesPerAtom() == length)
        // Java: this.decodeAtom(ps, bStream, this.bytesPerAtom());
        // Java: totalBytes += this.bytesPerAtom();
        // Java: else
        // Java: this.decodeAtom(ps, bStream, length - i);
        // Java: totalBytes += length - i;
        // Java: this.decodeLineSuffix(ps, bStream);
        // Java: catch (IOException var8)
        // Java: this.decodeBufferSuffix(ps, bStream);
        // Java: return;
    }

    // Java: _decodeBuffer: &[u8](String inputString)
    fn decodeBuffer_string(&self, input_string: &str) -> Vec<u8> {
        // Java: _inputBuffer: &[u8] = new byte[inputString.length()];
        // Java: inputString.getBytes(0, inputString.length(), inputBuffer, 0);
        let input_buffer = input_string.as_bytes().to_vec();
        // Java: ByteArrayInputStream inStream = new ByteArrayInputStream(inputBuffer);
        let mut in_stream = Cursor::new(input_buffer);
        // Java: ByteArrayOutputStream outStream = new ByteArrayOutputStream();
        let mut out_stream = Vec::new();
        // Java: this.decodeBuffer(inStream, outStream);
        self.decodeBuffer_stream(&mut in_stream, &mut out_stream);
        // Java: return outStream.toByteArray();
        out_stream
    }

    // Java: _decodeBuffer: &[u8](InputStream in)
    fn decodeBuffer_input(&self, input: &mut dyn Read) -> Vec<u8> {
        // Java: ByteArrayOutputStream outStream = new ByteArrayOutputStream();
        let mut out_stream = Vec::new();
        // Java: this.decodeBuffer(in, outStream);
        self.decodeBuffer_stream(input, &mut out_stream);
        // Java: return outStream.toByteArray();
        out_stream
    }

    // Java: ByteBuffer decodeBufferToByteBuffer(String inputString)
    fn decodeBufferToByteBuffer_string(&self, input_string: &str) -> Vec<u8> {
        // Java: return ByteBuffer.wrap(this.decodeBuffer(inputString));
        self.decodeBuffer_string(input_string)
    }

    // Java: ByteBuffer decodeBufferToByteBuffer(InputStream in)
    fn decodeBufferToByteBuffer_input(&self, input: &mut dyn Read) -> Vec<u8> {
        // Java: return ByteBuffer.wrap(this.decodeBuffer(in));
        self.decodeBuffer_input(input)
    }
}
