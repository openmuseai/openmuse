// Java: import cn.wps.ole.IOLEStream
// Java: import org.apache.poi.poifs.stream.POIOutputStream

use crate::org::apache::poi::poifs::stream::POIOutputStream::{POIOutputStream, IOLEStreamWriter};

// Java: public class HWPFOutputStream extends POIOutputStream
pub struct HWPFOutputStream {
    inner: POIOutputStream,
}

// Java: public class HWPFOutputStream extends POIOutputStream
impl HWPFOutputStream {
    // Java: HWPFOutputStream(IOLEStream pStream)
    pub fn new(pStream: Box<dyn IOLEStreamWriter>) -> Self {
        // Java: super(pStream);
        HWPFOutputStream {
            inner: POIOutputStream::new(pStream),
        }
    }

    pub fn as_poi_output_stream(&mut self) -> &mut POIOutputStream {
        &mut self.inner
    }
}

impl std::ops::Deref for HWPFOutputStream {
    type Target = POIOutputStream;
    fn deref(&self) -> &POIOutputStream { &self.inner }
}

impl std::ops::DerefMut for HWPFOutputStream {
    fn deref_mut(&mut self) -> &mut POIOutputStream { &mut self.inner }
}

impl std::io::Write for HWPFOutputStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write_all(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
