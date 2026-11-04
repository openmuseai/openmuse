// Java: import cn.wps.kfc.xpxml.hlxml.HLXmlReader
use core_libs_kfc::kfc::xpxml::hlxml::HLXmlReader::HLXmlReader;
// Java: import cn.wps.kfc.xpxml.hlxml.Mode
use core_libs_kfc::kfc::xpxml::hlxml::Mode::Mode;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import java.io.IOException
// Java: import java.io.InputStream
use std::io::Read;

use super::MapManager::MapManager;

// Java field: String TAG
pub(crate) const TAG: Option<&str> = None;

// Java: public class UnifiedXmlReader
pub struct UnifiedXmlReader {
}

// Java: public class UnifiedXmlReader
impl UnifiedXmlReader {
    // Java: UnifiedXmlReader()
    pub fn UnifiedXmlReader() -> Self {
      // Java: super();
      UnifiedXmlReader {}
    }

    // Java: static void read(InputStream stream, ElementHandler handler)
    pub fn read(stream: &mut dyn Read, handler: Box<dyn ElementHandler>) -> std::io::Result<()> {
      // Java: read(stream, handler, -1);
      Self::read_with_prefix(stream, handler, -1)
    }

    // Java: static void read(InputStream stream, ElementHandler handler, int labelPrefix)
    pub fn read_with_prefix(stream: &mut dyn Read, handler: Box<dyn ElementHandler>, labelPrefix: i32) -> std::io::Result<()> {
      // Java: HLXmlReader reader = new HLXmlReader(MapManager.getInstance(), handler, Mode.uof);
      let mut reader = HLXmlReader::new_with_mode(Box::new(MapManager::getInstance().clone()), handler, Mode::Uof);
      // Java: reader.parser(stream, true, labelPrefix);
      reader.parser_stream_begin_prefix(stream, true, labelPrefix)
    }

}
