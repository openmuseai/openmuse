#![allow(non_snake_case, dead_code, unused_variables)]

use std::io::{self, Read, Write};

// Java field: String TAG
pub(crate) const TAG: &str = "StreamUtil";

// Java: public class StreamUtil
pub struct StreamUtil;

// Java: public class StreamUtil
impl StreamUtil {
    // Java: StreamUtil()
    pub fn new() -> Self {
        // Java: super();
        Self
    }

    // Java: static void copyStream(InputStream input, OutputStream output)
    pub fn copyStream(input: &mut dyn Read, output: &mut dyn Write) {
        // Java: DumpPicBuffer dumpBuf = DumpPicBuffer.obtain();
        // Java: Assert.assertNotNull("dumpBuf should not be null.", dumpBuf);
        // Java: _buf: &[u8] = dumpBuf.getBuf();
        // Java: Assert.assertNotNull("buf should not be null.", buf);
        let mut buf = vec![0u8; 8192];
        // Java: try
        {
            // Java: int nRead;
            // Java: while((nRead = input.read(buf)) > 0)
            loop {
                let n_read = input.read(&mut buf).unwrap_or(0);
                if n_read == 0 { break; }
                // Java: output.write(buf, 0, nRead);
                let _ = output.write_all(&buf[..n_read]);
            }
            // Java: dumpBuf.recycle();
        }
        // Java: catch (IOException var5)
        // Java: dumpBuf.recycle();
    }

    // Java: static void copyStream(InputStream input, int size, OutputStream output)
    pub fn copyStream_sized(input: &mut dyn Read, size: i32, output: &mut dyn Write) -> io::Result<()> {
        // Java: if (input == null)
        // Java: Log.e(TAG, "inputstream is null");
        // Java: else
        {
            // Java: DumpPicBuffer dumpBuf = DumpPicBuffer.obtain();
            // Java: Assert.assertNotNull("dumpBuf should not be null.", dumpBuf);
            // Java: _buf: &[u8] = dumpBuf.getBuf();
            // Java: Assert.assertNotNull("buf should not be null.", buf);
            let mut buf = vec![0u8; 8192];
            // Java: try
            {
                // Java: while(size > 0)
                while size > 0 {
                    // Java: int nToRead = Math.min(buf.length, size);
                    let n_to_read = std::cmp::min(buf.len() as i32, size) as usize;
                    // Java: int nRead = input.read(buf, 0, nToRead);
                    let n_read = input.read(&mut buf[..n_to_read])?;
                    // Java: if (nRead <= 0)
                    if n_read <= 0 {
                        // Java: throw new EOFException("EOF!!");
                        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF!!"));
                    }
                    // Java: output.write(buf, 0, nRead);
                    output.write_all(&buf[..n_read])?;
                    // Java: size -= nRead;
                    size -= n_read as i32;
                }
                // Java: dumpBuf.recycle();
            }
            // Java: catch (ZipException var7)
            // Java: dumpBuf.recycle();
            // Java: Log.e(TAG, "read inputStream failed");
            // Java: catch (IOException e)
            // Java: dumpBuf.recycle();
            // Java: throw e;
        }
        Ok(())
    }

    // Java: static void close(Closeable closeable)
    pub fn close_closeable(closeable: &mut dyn Drop) {
        // Java: if (null != closeable)
        // Java: try
        // Java: closeable.close();
        // Java: catch (IOException e)
        // Java: Log.e(TAG, "IOException", e);
    }

    // Java: static void close(ZipFile zf)
    pub fn close_zipfile() {
        // Java: if (null != zf)
        // Java: try
        // Java: zf.close();
        // Java: catch (IOException e)
        // Java: Log.e(TAG, "IOException", e);
    }
}
