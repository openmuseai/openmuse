#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments, unused_unsafe, unreachable_code)]

// Java: import cn.wps.kfc.ooxfr.OxfrCustomPart
use kfc_kfc_alpha::kfc::ooxfr::OxfrCustomPart;
// Java: import cn.wps.kfc.ooxfr.OxfrPackage
use kfc_kfc_alpha::kfc::ooxfr::OxfrPackage;
// Java: import cn.wps.kfc.xpxml.hlxml.HLXmlReader
use kfc_kfc_alpha::kfc::xpxml::hlxml::HLXmlReader;
// Java: import cn.wps.moffice.kfs.File
use kfs_kfs_app::kfs::File;
// Java: import cn.wps.moffice.kfs.FileInputStream
use kfs_kfs_app::kfs::FileInputStream;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
use kfs_kfs_app::kfs::RandomAccessFile;
// Java: import java.io.FileNotFoundException
// LLM JavaLine Aggregation: Rust uses std::io::Error for all IO exceptions
// Java: import java.io.IOException
// LLM JavaLine Aggregation: Rust uses std::io::Error for IOException
// Java: import java.io.InputStream
use std::io::Read;
// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import java.util.Map
// LLM JavaLine Aggregation: java.util.Map is represented by HashMap in Rust
// Java: import org.apache.poi.hpsf.CustomProperties
use io_poi_compact::org::apache::poi::hpsf::CustomProperties;
// Java: import org.apache.poi.hpsf.CustomProperty
use io_poi_compact::org::apache::poi::hpsf::CustomProperty;
// Java: import org.apache.poi.hpsf.DocumentSummaryInformation
use io_poi_compact::org::apache::poi::hpsf::DocumentSummaryInformation;
// Java: import org.apache.poi.hpsf.Property
use io_poi_compact::org::apache::poi::hpsf::Property;
// Java: import org.apache.poi.hpsf.PropertySet
use io_poi_compact::org::apache::poi::hpsf::PropertySet;
// Java: import org.apache.poi.hpsf.PropertySetFactory
use io_poi_compact::org::apache::poi::hpsf::PropertySetFactory;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use io_poi_compact::org::apache::poi::poifs::filesystem::DocumentInputStream;
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
use io_poi_compact::org::apache::poi::poifs::filesystem::POIFSFileSystem;

// Java field: byte[] BINARY_FILE_HEADER
pub static BINARY_FILE_HEADER: Vec<i8> = Vec::new();
// Java field: byte[] OOXML_FILE_HEADER
static OOXML_FILE_HEADER: Vec<i8> = Vec::new();
// Java field: String DocumentSummaryInformation_STREAM
pub(crate) const DocumentSummaryInformation_STREAM: Option<&str> = Some("\u{0005}DocumentSummaryInformation");
// Java field: String PROP_ICV
const PROP_ICV: Option<&str> = Some("ICV");

// Java: public class ICVReader
pub struct ICVReader {
}

// Java: public class ICVReader
impl ICVReader {
    // Java: ICVReader()
    pub fn new() -> Self {
      // Java: super();
      ICVReader {}
    }

    // Java: static String readIcv(String path)
    pub fn readIcv(path: Option<&str>) -> Option<String> {
      // Java: if (null == path)
      if path.is_none() {
         // Java: return null;
         return None;
      }
      // Java: else
      else {
         let path_str = path.unwrap();
         // Java: File file = new File(path);
         let file = File::File::new(path_str);
         // Java: if (!file.exists())
         if !file.exists() {
            // Java: return null;
            return None;
         }
         // Java: else
         else {
            // Java: byte[] header = new byte[16];
            let mut header: Vec<i8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
            // Java: try
            let try_result: std::io::Result<()> = Self::readFileHeader(&mut header, Some(path_str), None);
            match try_result {
               // Java: readFileHeader(header, path, (RandomAccessFile)null);
               Ok(_) => {}
               // Java: catch (IOException e)
               Err(e) => {
                  // Java: e.printStackTrace();
                  eprintln!("{:?}", e);
               }
            }
            // Java: if (isX(header))
            if Self::isX(&header) {
               // Java: return readXIcv(path);
               return Self::readXIcv_path(path_str);
            }
            // Java: else
            else {
               // Java: return is03(header) ? read03Icv(path) : null;
               return if Self::is03(&header) { Self::read03Icv_path(path_str) } else { None };
            }
         }
      }
    }

    // Java: static void readFileHeader(byte[] header, String path, RandomAccessFile raf)
    fn readFileHeader(header: &mut Vec<i8>, path: Option<&str>, mut raf: Option<&mut RandomAccessFile::RandomAccessFile>) -> std::io::Result<()> {
      // Java: if (null != path)
      if path.is_some() {
         let path = path.unwrap();
         // Java: FileInputStream stream = null;
         let mut stream: Option<FileInputStream::FileInputStream> = None;
         // Java: try
         let _try_result: std::io::Result<()> = (|| -> std::io::Result<()> {
            // Java: stream = new FileInputStream(path);
            stream = Some(FileInputStream::FileInputStream::from_path(Some(path))?);
            // Java: stream.read(header);
            let s = stream.as_mut().unwrap();
            let buf: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(header.as_mut_ptr() as *mut u8, header.len()) };
            s.read_buf(buf)?;
            Ok(())
         })();
         // Java: finally
         // LLM JavaLine Aggregation: finally block runs cleanup
         {
            // Java: if (stream != null)
            if stream.is_some() {
               // Java: stream.close();
               drop(stream.take());
            }
         }
      }
      // Java: else if (null != raf)
      else if raf.is_some() {
         let raf = raf.as_mut().unwrap();
         // Java: raf.seek(0L);
         raf.seek(0)?;
         // Java: raf.read(header);
         let buf: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(header.as_mut_ptr() as *mut u8, header.len()) };
         raf.read_buffer(buf)?;
         // Java: raf.seek(0L);
         raf.seek(0)?;
      }
      Ok(())
    }

    // Java: static boolean isX(byte[] header)
    fn isX(header: &[i8]) -> bool {
      // Java: if (header != null && header.length >= 4)
      if !header.is_empty() && header.len() >= 4 {
         // Java: return header[0] == OOXML_FILE_HEADER[0] && header[1] == OOXML_FILE_HEADER[1] && header[2] == OOXML_FILE_HEADER[2] && header[3] == OOXML_FILE_HEADER[3];
         return header[0] == OOXML_FILE_HEADER[0] && header[1] == OOXML_FILE_HEADER[1] && header[2] == OOXML_FILE_HEADER[2] && header[3] == OOXML_FILE_HEADER[3];
      }
      // Java: else
      else {
         // Java: return false;
         return false;
      }
    }

    // Java: static boolean is03(byte[] header)
    fn is03(header: &[i8]) -> bool {
      // Java: if (header != null && header.length >= 8)
      if !header.is_empty() && header.len() >= 8 {
         // Java: return header[0] == BINARY_FILE_HEADER[0] && header[1] == BINARY_FILE_HEADER[1] && header[2] == BINARY_FILE_HEADER[2] && header[3] == BINARY_FILE_HEADER[3] && header[4] == BINARY_FILE_HEADER[4] && header[5] == BINARY_FILE_HEADER[5] && header[6] == BINARY_FILE_HEADER[6] && header[7] == BINARY_FILE_HEADER[7];
         return header[0] == BINARY_FILE_HEADER[0] && header[1] == BINARY_FILE_HEADER[1] && header[2] == BINARY_FILE_HEADER[2] && header[3] == BINARY_FILE_HEADER[3] && header[4] == BINARY_FILE_HEADER[4] && header[5] == BINARY_FILE_HEADER[5] && header[6] == BINARY_FILE_HEADER[6] && header[7] == BINARY_FILE_HEADER[7];
      }
      // Java: else
      else {
         // Java: return false;
         return false;
      }
    }

    // Java: static String readIcv(RandomAccessFile raf)
    pub fn readIcv_raf(raf: Option<&mut RandomAccessFile::RandomAccessFile>) -> Option<String> {
      // Java: if (null == raf)
      if raf.is_none() {
         // Java: return null;
         return None;
      }
      // Java: else
      else {
         let raf = raf.unwrap();
         // Java: byte[] header = new byte[16];
         let mut header: Vec<i8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
         // Java: try
         let try_result: std::io::Result<()> = Self::readFileHeader(&mut header, None, Some(raf));
         match try_result {
            // Java: readFileHeader(header, (String)null, raf);
            Ok(_) => {}
            // Java: catch (IOException e)
            Err(e) => {
               // Java: e.printStackTrace();
               eprintln!("{:?}", e);
            }
         }
         // Java: if (isX(header))
         if Self::isX(&header) {
            // Java: return readXIcv(raf);
            return Self::readXIcv_raf(raf);
         }
         // Java: else
         else {
            // Java: return is03(header) ? read03Icv(raf) : null;
            return if Self::is03(&header) { Self::read03Icv_raf(raf) } else { None };
         }
      }
    }

    // Java: static String read03Icv(RandomAccessFile raf)
    fn read03Icv_raf(raf: &mut RandomAccessFile::RandomAccessFile) -> Option<String> {
      // Java: POIFSFileSystem poifs = null;
      let mut poifs: Option<POIFSFileSystem::POIFSFileSystem> = None;
      // Java: try
      let result: std::io::Result<Option<String>> = (|| -> std::io::Result<Option<String>> {
         // Java: poifs = new POIFSFileSystem(raf);
         let mut raf_reader = RafReader(raf);
         poifs = POIFSFileSystem::POIFSFileSystem::from_stream(&mut raf_reader).ok();
         // Java: DocumentInputStream documentInputStream = null;
         let mut documentInputStream: Option<DocumentInputStream::DocumentInputStream> = None;
         // Java: if (null != poifs && poifs.getRoot().hasEntry("\u0005DocumentSummaryInformation"))
         if poifs.is_some() && poifs.as_mut().unwrap().getRoot().hasEntry(DocumentSummaryInformation_STREAM.unwrap()) {
            let poifs_ref = poifs.as_mut().unwrap();
            // Java: documentInputStream = poifs.createDocumentInputStream("\u0005DocumentSummaryInformation");
            documentInputStream = poifs_ref.getRoot().createDocumentInputStream_by_name(DocumentSummaryInformation_STREAM.unwrap()).ok();
            // Java: if (null == documentInputStream)
            if documentInputStream.is_none() {
               // Java: return null;
               return Ok(None);
            }
            // Java: PropertySet set = PropertySetFactory.create(documentInputStream);
            let mut doc_is = documentInputStream.unwrap();
            let mut doc_reader = DocISReader(&mut doc_is);
            let set = match PropertySetFactory::PropertySetFactory::create(&mut doc_reader) {
               Ok(s) => s,
               Err(_) => return Ok(None),
            };
            // Java: documentInputStream.close();
            drop(doc_reader);
            drop(doc_is);
            // Java: if (!(set instanceof DocumentSummaryInformation))
            if !set.isDocumentSummaryInformation() {
               // Java: Object var21 = null;
               let var21: Option<String> = None;
               // Java: return (String)var21;
               return Ok(var21);
            }
            // Java: DocumentSummaryInformation info = (DocumentSummaryInformation)set;
            let info = DocumentSummaryInformation::DocumentSummaryInformation::new(&set);
            // Java: CustomProperties customProperties = info.getCustomProperties();
            let customProperties = info.getCustomProperties();
            // Java: for(CustomProperty ioProperty : customProperties.values())
            if let Some(cp_box) = customProperties {
                if let Some(cps) = cp_box.downcast_ref::<CustomProperties::CustomProperties>() {
                    for ioProperty in cps.values() {
                       // Java: if (null != ioProperty)
                       if true {
                          // Java: String name = ioProperty.getName();
                          let name = ioProperty.getName();
                          // Java: if (name.equals("ICV"))
                          if name.as_deref() == PROP_ICV {
                             // Java: long type = ioProperty.getType();
                             // LLM JavaLine Aggregation: type is a reserved keyword in Rust, inlined into if condition
                             // Java: Object value = ioProperty.getValue();
                             let value = ioProperty.getValue();
                             // Java: if (null == value)
                             if matches!(value, Property::PropertyValue::None) {
                                // Java: Object var22 = null;
                                let var22: Option<String> = None;
                                // Java: return (String)var22;
                                return Ok(var22);
                             }
                             // Java: if (type == 31L)
                             if ioProperty.getType() == 31 {
                                // Java: String var12 = (String)ioProperty.getValue();
                                let var12 = match ioProperty.getValue() {
                                    Property::PropertyValue::Str(s) => s.clone(),
                                    _ => String::new(),
                                };
                                // Java: return var12;
                                return Ok(Some(var12));
                             }
                          }
                       }
                    }
                }
            }
            // Java: return null;
            return Ok(None);
         }
         // Java: return null;
         Ok(None)
      })();
      // Java: catch (Exception var16)
      match &result {
         Err(_) => {
            // Java: return null;
            let _catch: Option<String> = None;
         }
         _ => {}
      }
      // Java: finally
      {
         // Java: if (null != poifs)
         if let Some(ref mut p) = poifs {
            // Java: poifs.dispose();
            p.dispose();
         }
      }
      match result {
         Ok(v) => v,
         Err(_) => None,
      }
    }

    // Java: static String read03Icv(String path)
    fn read03Icv_path(path: &str) -> Option<String> {
      // Java: RandomAccessFile raf = null;
      let mut raf: Option<RandomAccessFile::RandomAccessFile> = None;
      // Java: try
      let try_result: std::io::Result<RandomAccessFile::RandomAccessFile> = RandomAccessFile::RandomAccessFile::from_file(path, "r");
      match try_result {
         Ok(mut r) => {
            // Java: raf = new RandomAccessFile(path, "r");
            raf = Some(r);
            // Java: return read03Icv(raf);
            return Self::read03Icv_raf(raf.as_mut().unwrap());
         }
         // Java: catch (FileNotFoundException e)
         Err(e) => {
            // Java: e.printStackTrace();
            eprintln!("{:?}", e);
            // Java: return null;
            return None;
         }
      }
    }

    // Java: static String readXIcv(RandomAccessFile raf)
    fn readXIcv_raf(raf: &mut RandomAccessFile::RandomAccessFile) -> Option<String> {
      // Java: try
      let try_result: std::io::Result<OxfrPackage::OxfrPackage> = OxfrPackage::OxfrPackage::new_from_raf(raf);
      match try_result {
         Ok(mut v) => {
            // Java: OxfrPackage oxfrPackage = new OxfrPackage(raf);
            let mut oxfrPackage = v;
            // Java: return readXIcv(oxfrPackage);
            return Self::readXIcv_package(&mut oxfrPackage);
         }
         // Java: catch (IOException e)
         Err(e) => {
            // Java: e.printStackTrace();
            eprintln!("{:?}", e);
            // Java: return null;
            return None;
         }
      }
    }

    // Java: static String readXIcv(String path)
    fn readXIcv_path(path: &str) -> Option<String> {
      // Java: try
      let try_result: std::io::Result<Option<String>> = (|| -> std::io::Result<Option<String>> {
         // Java: OxfrPackage oxfrPackage = new OxfrPackage(path);
         let mut oxfrPackage = OxfrPackage::OxfrPackage::new(path);
         // Java: return readXIcv(oxfrPackage);
         Ok(Self::readXIcv_package(&mut oxfrPackage))
      })();
      match try_result {
         Ok(v) => { return v; }
         // Java: catch (IOException e)
         Err(e) => {
            // Java: e.printStackTrace();
            eprintln!("{:?}", e);
            // Java: return null;
            return None;
         }
      }
    }

    // Java: static String readXIcv(OxfrPackage oxfrPackage)
    fn readXIcv_package(oxfrPackage: &mut OxfrPackage::OxfrPackage) -> Option<String> {
      // Java: try
      let outer_result: std::io::Result<Option<String>> = (|| -> std::io::Result<Option<String>> {
         // Java: OxfrCustomPart customPart = oxfrPackage.getCustomPart();
         let customPart = oxfrPackage.getCustomPart();
         // Java: InputStream stream = customPart.getInputStream();
         let stream = customPart.getInputStream();
         // Java: try
         let inner_result: std::io::Result<Option<String>> = (|| -> std::io::Result<Option<String>> {
            // Java: if (stream != null)
            if !stream.is_empty() {
               // Java: Map<String, Property> mCustomProps = new HashMap();
               let mut mCustomProps: HashMap<String, Property::Property> = HashMap::new();
               // Java: HLXmlReader.parser(stream, new PropertiesHandler(mCustomProps));
               let mut cursor = std::io::Cursor::new(&stream);
               let handler = Box::new(crate::cn::wps::moffice::io::util::PropertiesHandler::PropertiesHandler::new(mCustomProps.clone()));
               let _ = HLXmlReader::HLXmlReader::parser_str_handler(&mut cursor, handler);
               // Java: return (String)((Property)mCustomProps.get("ICV")).getValue();
               let icv_result = mCustomProps.get(PROP_ICV.unwrap()).map(|p| {
                   match p.getValue() {
                       Property::PropertyValue::Str(s) => s.clone(),
                       _ => String::new(),
                   }
               });
               return Ok(icv_result);
            }
            Ok(None)
         })();
         match inner_result {
            Ok(v) => {
               if v.is_some() { return Ok(v); }
            }
            // Java: catch (Throwable var4)
            Err(_) => {
               // Java: return null;
               return Ok(None);
            }
         }
         Ok(None)
      })();
      match outer_result {
         Ok(v) => {
            if v.is_some() { return v; }
         }
         // Java: catch (IOException e)
         Err(e) => {
            // Java: e.printStackTrace();
            eprintln!("{:?}", e);
         }
      }
      // Java: return null;
      None
    }

}

struct RafReader<'a>(&'a mut RandomAccessFile::RandomAccessFile);
impl<'a> Read for RafReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let i8_buf: &mut [i8] = unsafe { std::slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut i8, buf.len()) };
        let n = self.0.read_buffer(i8_buf)?;
        Ok(if n < 0 { 0 } else { n as usize })
    }
}

struct DocISReader<'a>(&'a mut DocumentInputStream::DocumentInputStream);
impl<'a> Read for DocISReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.0.read(buf);
        Ok(if n < 0 { 0 } else { n as usize })
    }
}
