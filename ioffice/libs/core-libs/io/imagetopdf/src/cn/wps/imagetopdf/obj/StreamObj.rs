// Java: import cn.wps.imagetopdf.MyStringBuilder
use crate::cn::wps::imagetopdf::MyStringBuilder::MyStringBuilder;
// Java: import java.io.ByteArrayOutputStream
// Java: import java.io.IOException
use std::io::Write;
// Java: import java.util.zip.DeflaterOutputStream
use flate2::write::DeflateEncoder;
use flate2::Compression;

use super::DictObj::DictObj;

// Java: public class StreamObj extends DictObj
pub struct StreamObj {
    pub(crate) base: DictObj,
    // Java field: byte[] stream
    pub(crate) stream: Vec<u8>,
    // Java field: String startstream
    pub(crate) startstream: &'static str,
    // Java field: String endstream
    pub(crate) endstream: &'static str,
    // Java field: boolean compressed
    pub(crate) compressed: bool,
}

// Java: public class StreamObj extends DictObj
impl StreamObj {
    // Java: StreamObj(String dict, byte[] stream)
    pub fn new(dict: &str, stream: Vec<u8>) -> Self {
        // Java: super(dict);
        let base = DictObj::new(dict);
        // Java: this.stream = stream;
        StreamObj { base, stream, compressed: false, startstream: "stream", endstream: "endstream" }
    }

    // Java: StreamObj(String dict)
    pub fn from_dict(dict: &str) -> Self {
        // Java: super(dict);
        let base = DictObj::new(dict);
        StreamObj { base, stream: Vec::new(), compressed: false, startstream: "stream", endstream: "endstream" }
    }

    // Java: byte[] getBytes()
    pub fn getBytes(&mut self) -> Vec<u8> {
        // Java: MyStringBuilder sb = new MyStringBuilder();
        let mut sb = MyStringBuilder::new();
        // Java: sb.app(this.getStartobj());
        sb.app_str(&self.base.getStartobj());
        // Java: sb.app(this.dict);
        sb.app_str(&self.base.dict);
        // Java: if (this.compressed)
        if self.compressed {
            // Java: ByteArrayOutputStream bao = new ByteArrayOutputStream();
            // Java: DeflaterOutputStream zip = new DeflaterOutputStream(bao);
            // Java: try
            let compressed_result: Result<Vec<u8>, std::io::Error> = (|| {
                let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
                // Java: zip.write(this.stream);
                encoder.write_all(&self.stream)?;
                // Java: zip.close();
                encoder.finish()
            })();
            match compressed_result {
                Ok(compressed_data) => {
                    // Java: this.stream = bao.toByteArray();
                    self.stream = compressed_data;
                    // Java: sb.app("/Filter /FlateDecode");
                    sb.app_str("/Filter /FlateDecode");
                }
                // Java: catch (IOException e)
                Err(_e) => {
                    // Java: e.printStackTrace();
                }
            }
        }
        // Java: sb.app("/Length " + this.stream.length);
        sb.app_str(&format!("/Length {}", self.stream.len()));
        // Java: sb.app(">>");
        sb.app_str(">>");
        // Java: sb.app("stream");
        sb.app_str("stream");
        // Java: byte[] bytes1 = sb.getBytes();
        let bytes1 = sb.getBytes();
        // Java: byte[] bytes3 = "\nendstream\nendobj\n".getBytes();
        let bytes3 = b"\nendstream\nendobj\n";
        // Java: int len1 = bytes1.length;
        let len1 = bytes1.len();
        // Java: int len2 = this.stream.length;
        let len2 = self.stream.len();
        // Java: int len3 = bytes3.length;
        let len3 = bytes3.len();
        // Java: byte[] all = new byte[len1 + len2 + len3];
        let mut all = Vec::with_capacity(len1 + len2 + len3);
        // Java: System.arraycopy(bytes1, 0, all, 0, len1);
        all.extend_from_slice(&bytes1);
        // Java: System.arraycopy(this.stream, 0, all, len1, len2);
        all.extend_from_slice(&self.stream);
        // Java: System.arraycopy(bytes3, 0, all, len1 + len2, len3);
        all.extend_from_slice(bytes3);
        // Java: return all;
        all
    }

    // Java: void compress()
    pub fn compress(&mut self) {
        // Java: this.compressed = true;
        self.compressed = true;
    }
}
