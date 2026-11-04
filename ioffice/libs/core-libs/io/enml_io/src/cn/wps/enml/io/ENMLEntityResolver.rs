#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.base.assertion.KSAssert
use util::base::assertion::KSAssert::KSAssert;
// Java: import cn.wps.base.log.Log
use util::base::log::Log::Log;
// Java: import java.io.InputStream
// Java: import org.xml.sax.EntityResolver
// Java: import org.xml.sax.InputSource

pub struct InputSource {
    data: Vec<u8>,
}

impl InputSource {
    pub fn new(data: Vec<u8>) -> Self {
        InputSource { data }
    }
}

pub trait EntityResolver {
    fn resolveEntity(&self, public_id: &str, system_id: &str) -> Option<InputSource>;
}

// Java field: String TAG
pub(crate) const TAG: Option<&str> = None;

// Java: public class ENMLEntityResolver implements EntityResolver
pub struct ENMLEntityResolver {
}

// Java: public class ENMLEntityResolver implements EntityResolver
impl ENMLEntityResolver {
    // Java: ENMLEntityResolver()
    pub fn new() -> Self {
      // Java: super();
      ENMLEntityResolver {}
    }

    // Java: InputSource resolveEntity(String publicId, String systemId)
    pub fn resolveEntity(&self, publicId: &str, systemId: &str) -> Option<InputSource> {
      // Java: Log.i(TAG, "Start resolving entity, public id: " + publicId + ", system id: " + systemId);
      Log::i(TAG.unwrap_or(""), &format!("Start resolving entity, public id: {}, system id: {}", publicId, systemId));
      // Java: String resourceFile = null;
      let mut resourceFile: Option<&str> = None;
      // Java: InputStream is = null;
      // Java: if (systemId.equals("http://xml.evernote.com/pub/enml2.dtd"))
      if systemId == "http://xml.evernote.com/pub/enml2.dtd" {
         // Java: resourceFile = "enml2.dtd";
         resourceFile = Some("enml2.dtd");
      }
      // Java: else if (!publicId.equals("-//W3C//ENTITIES Latin 1 for XHTML//EN") && !systemId.equals("http://www.w3.org/TR/xhtml1/DTD/xhtml-lat1.ent"))
      else if publicId != "-//W3C//ENTITIES Latin 1 for XHTML//EN" && systemId != "http://www.w3.org/TR/xhtml1/DTD/xhtml-lat1.ent" {
         // Java: if (!publicId.equals("-//W3C//ENTITIES Symbols for XHTML//EN") && !systemId.equals("http://www.w3.org/TR/xhtml1/DTD/xhtml-symbol.ent"))
         if publicId != "-//W3C//ENTITIES Symbols for XHTML//EN" && systemId != "http://www.w3.org/TR/xhtml1/DTD/xhtml-symbol.ent" {
            // Java: if (!publicId.equals("-//W3C//ENTITIES Special for XHTML//EN") && !systemId.equals("http://www.w3.org/TR/xhtml1/DTD/xhtml-special.ent"))
            if publicId != "-//W3C//ENTITIES Special for XHTML//EN" && systemId != "http://www.w3.org/TR/xhtml1/DTD/xhtml-special.ent" {
               // Java: KSAssert.fail("Unexpected entity, public id: " + publicId + ", system id: " + systemId);
               panic!("Unexpected entity, public id: {}, system id: {}", publicId, systemId);
               // Java: return null;
               return None;
            }
            // Java: resourceFile = "xhtml-special.ent";
            resourceFile = Some("xhtml-special.ent");
         }
         // Java: else
         else {
            // Java: resourceFile = "xhtml-symbol.ent";
            resourceFile = Some("xhtml-symbol.ent");
         }
      }
      // Java: else
      else {
         // Java: resourceFile = "xhtml-lat1.ent";
         resourceFile = Some("xhtml-lat1.ent");
      }
      // Java: is = this.getClass().getResourceAsStream(resourceFile);
      // Java: KSAssert.assertNotNull("Failed to get InputStream for file: " + resourceFile, is);
      // Java: return new InputSource(is);
      todo!("getResourceAsStream not available in Rust")
    }
}

impl EntityResolver for ENMLEntityResolver {
    fn resolveEntity(&self, public_id: &str, system_id: &str) -> Option<InputSource> {
        ENMLEntityResolver::resolveEntity(self, public_id, system_id)
    }
}
