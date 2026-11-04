#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import cn.wps.base.assertion.Assert
use crate::base::assertion::Assert::Assert;
// Java: import cn.wps.base.log.Log
use crate::base::log::Log::Log;
// Java: import cn.wps.moffice.kfs.FileInputStream
// Java: import java.io.ByteArrayOutputStream
// Java: import java.io.Closeable
// Java: import java.io.FileDescriptor
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.OutputStream
// Java: import java.nio.ByteBuffer
// Java: import java.nio.channels.ReadableByteChannel
// Java: import java.util.zip.CRC32
// Java: import java.util.zip.Checksum
use std::io::{self, Read, Write};

// Java field: String TAG
const TAG: Option<&str> = None;
// Java field: int IO_BUFFER_SIZE
pub const IO_BUFFER_SIZE: i32 = 4096;

// Java: public class IOUtils
pub struct IOUtils;

// Java: public class IOUtils
impl IOUtils {
    // Java: IOUtils()
    pub fn new() -> Self {
        // Java: super();
        Self
    }

    // Java: static int getFileSize(FileDescriptor mFileDescriptor)
    pub fn getFileSize(_mFileDescriptor: i64) -> i32 {
        // Java: FileInputStream fis = new FileInputStream(mFileDescriptor);
        // Java: int ret = 0;
        let ret: i32 = 0;
        // Java: try
        {
            // Java: ret = fis.available();
        }
        // Java: catch (IOException e)
        {
            // Java: Log.e(TAG, "IOException", e);
        }
        // Java: finally
        {
            // Java: try
            {
                // Java: fis.close();
            }
            // Java: catch (IOException e)
            {
                // Java: Log.e(TAG, "IOException", e);
            }
        }
        // Java: return ret;
        ret
    }

    // Java: static void copy(InputStream in, OutputStream out)
    pub fn copy(input: &mut dyn Read, out: &mut dyn Write) -> io::Result<()> {
        // Java: Assert.assertNotNull("in should not be null.", in);
        // Java: Assert.assertNotNull("out should not be null.", out);
        // Java: _buf: &[u8] = new byte[4096];
        let mut buf = [0u8; 4096];
        // Java: int read = -1;
        let mut read: i32;
        // Java: while((read = in.read(buf)) != -1)
        loop {
            read = match input.read(&mut buf) {
                Ok(0) => -1,
                Ok(n) => n as i32,
                Err(_) => -1,
            };
            if read == -1 { break; }
            // Java: out.write(buf, 0, read);
            out.write_all(&buf[..read as usize])?;
        }
        Ok(())
    }

    // Java: static void copy(InputStream in, OutputStream out, _buf: &[u8])
    pub fn copy_with_buf(input: &mut dyn Read, out: &mut dyn Write, buf: &mut [u8]) -> io::Result<()> {
        // Java: Assert.assertNotNull("in should not be null.", in);
        // Java: Assert.assertNotNull("out should not be null.", out);
        // Java: int read = -1;
        let mut read: i32;
        // Java: while((read = in.read(buf)) != -1)
        loop {
            read = match input.read(buf) {
                Ok(0) => -1,
                Ok(n) => n as i32,
                Err(_) => -1,
            };
            if read == -1 { break; }
            // Java: out.write(buf, 0, read);
            out.write_all(&buf[..read as usize])?;
        }
        Ok(())
    }

    // Java: static void closeStream(Closeable stream)
    pub fn closeStream<T>(_stream: Option<T>) {
        // Java: if (stream != null)
        {
            // Java: try
            {
                // Java: stream.close();
            }
            // Java: catch (IOException e)
            {
                // Java: Log.e(TAG, "Could not close stream", e);
            }
        }
    }

    // Java: static _getBytes: &[u8](_buffer: &[char], int start, int len)
    pub fn getBytes(buffer: Option<&[char]>, start: i32, len: i32) -> Option<Vec<i8>> {
        // Java: if (null != buffer && len > 0)
        if let Some(buffer) = buffer {
            if len > 0 {
                // Java: int end = start + len;
                let end = start + len;
                // Java: _ret: &[u8] = new byte[len];
                let mut ret = vec![0i8; len as usize];
                // Java: int index = 0;
                let mut index = 0usize;
                // Java: for(int i = start; i < end && i < buffer.length; ++i)
                let mut i = start;
                while i < end && (i as usize) < buffer.len() {
                    // Java: ret[index++] = (byte)buffer[i];
                    ret[index] = buffer[i as usize] as i8;
                    index += 1;
                    i += 1;
                }
                // Java: return ret;
                return Some(ret);
            }
        }
        // Java: else
        // Java: return null;
        None
    }

    // Java: static _toByteArray: &[u8](InputStream stream)
    pub fn toByteArray(stream: &mut dyn Read) -> io::Result<Vec<u8>> {
        // Java: ByteArrayOutputStream baos = new ByteArrayOutputStream();
        let mut baos = Vec::new();
        // Java: _buffer: &[u8] = new byte[4096];
        let mut buffer = [0u8; 4096];
        // Java: int read = 0;
        let mut read: i32 = 0;
        // Java: while(read != -1)
        while read != -1 {
            // Java: read = stream.read(buffer);
            read = match stream.read(&mut buffer) {
                Ok(0) => -1,
                Ok(n) => n as i32,
                Err(_) => -1,
            };
            // Java: if (read > 0)
            if read > 0 {
                // Java: baos.write(buffer, 0, read);
                baos.extend_from_slice(&buffer[..read as usize]);
            }
        }
        // Java: return baos.toByteArray();
        Ok(baos)
    }

    // Java: static _toByteArray: &[u8](ByteBuffer buffer, int length)
    pub fn toByteArray_from_buffer(buffer: &[u8], length: i32) -> Vec<u8> {
        // Java: if (buffer.hasArray() && buffer.arrayOffset() == 0)
        // Java: return buffer.array();
        // Java: else
        // Java: _data: &[u8] = new byte[length];
        let mut data = vec![0u8; length as usize];
        // Java: buffer.get(data);
        let copy_len = std::cmp::min(length as usize, buffer.len());
        data[..copy_len].copy_from_slice(&buffer[..copy_len]);
        // Java: return data;
        data
    }

    // Java: static int readFully(InputStream in, _b: &[u8])
    pub fn readFully_stream(input: &mut dyn Read, b: &mut [u8]) -> io::Result<i32> {
        // Java: return readFully((InputStream)in, b, 0, b.length);
        let len = b.len() as i32;
        Self::readFully_stream_range(input, b, 0, len)
    }

    // Java: static int readFully(ByteBuffer in, _b: &[u8])
    pub fn readFully_buffer(input: &[u8], b: &mut [u8]) -> io::Result<i32> {
        // Java: return readFully((ByteBuffer)in, b, 0, b.length);
        let len = b.len() as i32;
        Self::readFully_buffer_range(input, b, 0, len)
    }

    // Java: static int readFully(InputStream in, _b: &[u8], int off, int len)
    pub fn readFully_stream_range(input: &mut dyn Read, b: &mut [u8], off: i32, len: i32) -> io::Result<i32> {
        // Java: int total = 0;
        let mut total: i32 = 0;
        // Java: do
        loop {
            // Java: int got = in.read(b, off + total, len - total);
            let start = (off + total) as usize;
            let end = (off + len) as usize;
            let got = match input.read(&mut b[start..end]) {
                Ok(0) => -1i32,
                Ok(n) => n as i32,
                Err(_) => -1i32,
            };
            // Java: if (got < 0)
            if got < 0 {
                // Java: return total == 0 ? -1 : total;
                return Ok(if total == 0 { -1 } else { total });
            }
            // Java: total += got;
            total += got;
            if total == len { break; }
        }
        // Java: return total;
        Ok(total)
    }

    // Java: static int readFully(ByteBuffer in, _b: &[u8], int off, int len)
    pub fn readFully_buffer_range(input: &[u8], b: &mut [u8], off: i32, len: i32) -> io::Result<i32> {
        // Java: int remaining = in.remaining();
        let remaining = input.len() as i32;
        // Java: if (len <= remaining)
        if len <= remaining {
            // Java: in.get(b, off, len);
            b[off as usize..(off + len) as usize].copy_from_slice(&input[..len as usize]);
            // Java: return len;
            Ok(len)
        }
        // Java: else
        else {
            // Java: in.get(b, off, remaining);
            b[off as usize..(off + remaining) as usize].copy_from_slice(&input[..remaining as usize]);
            // Java: return remaining;
            Ok(remaining)
        }
    }

    // Java: static int readFully(ReadableByteChannel channel, ByteBuffer b)
    pub fn readFully_channel(channel: &mut dyn Read, capacity: i32) -> io::Result<i32> {
        // Java: int total = 0;
        let mut total: i32 = 0;
        let mut b = vec![0u8; capacity as usize];
        // Java: do
        loop {
            // Java: int got = channel.read(b);
            let got = match channel.read(&mut b[total as usize..]) {
                Ok(0) => -1i32,
                Ok(n) => n as i32,
                Err(_) => -1i32,
            };
            // Java: if (got < 0)
            if got < 0 {
                // Java: return total == 0 ? -1 : total;
                return Ok(if total == 0 { -1 } else { total });
            }
            // Java: total += got;
            total += got;
            if total == capacity { break; }
        }
        // Java: return total;
        Ok(total)
    }

    // Java: static long calculateChecksum(_data: &[u8])
    pub fn calculateChecksum(data: &[u8]) -> i64 {
        // Java: Checksum sum = new CRC32();
        let mut crc: u32 = 0xFFFFFFFF;
        // Java: sum.update(data, 0, data.length);
        for &byte in data {
            crc ^= byte as u32;
            for _ in 0..8 {
                if crc & 1 != 0 {
                    crc = (crc >> 1) ^ 0xEDB88320;
                } else {
                    crc >>= 1;
                }
            }
        }
        // Java: return sum.getValue();
        (crc ^ 0xFFFFFFFF) as i64
    }
}
