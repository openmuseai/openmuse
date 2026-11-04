// Java: import cn.wps.base.assertion.Assert
use util::cn::wps::core::runtime::Assert;
// Java: import cn.wps.base.log.IOLog
use util::cn::wps::base::log::IOLog;
// Java: import cn.wps.base.log.Log
use kfs_kfs_app::cn::wps::moffice::kfs::log::Log;
// Java: import cn.wps.io.file.checker.html.HtmlChecker
use io_libfileformat::io::file::checker::html::HtmlChecker;
// Java: import cn.wps.io.file.checker.html.MHTChecker
use io_libfileformat::cn::wps::io::file::checker::html::MHTChecker;
// Java: import cn.wps.io.file.checker.ole.DocChecker
use io_libfileformat::io::file::checker::ole::DocChecker;
// Java: import cn.wps.io.file.checker.ole.PPTChecker
use io_libfileformat::io::file::checker::ole::PPTChecker;
// Java: import cn.wps.io.file.checker.ole.XlsChecker
use io_libfileformat::io::file::checker::ole::XlsChecker;
// Java: import cn.wps.io.file.checker.ooxml.DocmChecker
use io_libfileformat::io::file::checker::ooxml::DocmChecker;
// Java: import cn.wps.io.file.checker.ooxml.DocxChecker
use io_libfileformat::io::file::checker::ooxml::DocxChecker;
// Java: import cn.wps.io.file.checker.ooxml.DotmChecker
use io_libfileformat::io::file::checker::ooxml::DotmChecker;
// Java: import cn.wps.io.file.checker.ooxml.DotxChecker
use io_libfileformat::io::file::checker::ooxml::DotxChecker;
// Java: import cn.wps.io.file.checker.ooxml.OOXMLChecker
use io_libfileformat::io::file::checker::ooxml::OOXMLChecker;
// Java: import cn.wps.io.file.checker.ooxml.PotmChecker
use io_libfileformat::io::file::checker::ooxml::PotmChecker;
// Java: import cn.wps.io.file.checker.ooxml.PotxChecker
use io_libfileformat::io::file::checker::ooxml::PotxChecker;
// Java: import cn.wps.io.file.checker.ooxml.PpsxChecker
use io_libfileformat::io::file::checker::ooxml::PpsxChecker;
// Java: import cn.wps.io.file.checker.ooxml.PptxChecker
use io_libfileformat::io::file::checker::ooxml::PptxChecker;
// Java: import cn.wps.io.file.checker.ooxml.Uof3Checker
use io_libfileformat::io::file::checker::ooxml::Uof3Checker;
// Java: import cn.wps.io.file.checker.ooxml.Uop3Checker
use io_libfileformat::io::file::checker::ooxml::Uop3Checker;
// Java: import cn.wps.io.file.checker.ooxml.Uos3Checker
use io_libfileformat::io::file::checker::ooxml::Uos3Checker;
// Java: import cn.wps.io.file.checker.ooxml.Uot3Checker
use io_libfileformat::io::file::checker::ooxml::Uot3Checker;
// Java: import cn.wps.io.file.checker.ooxml.XlamChecker
use io_libfileformat::io::file::checker::ooxml::XlamChecker;
// Java: import cn.wps.io.file.checker.ooxml.XlsbChecker
use io_libfileformat::io::file::checker::ooxml::XlsbChecker;
// Java: import cn.wps.io.file.checker.ooxml.XlsmChecker
use io_libfileformat::io::file::checker::ooxml::XlsmChecker;
// Java: import cn.wps.io.file.checker.ooxml.XlsxChecker
use io_libfileformat::io::file::checker::ooxml::XlsxChecker;
// Java: import cn.wps.io.file.checker.ooxml.XltmChecker
use io_libfileformat::io::file::checker::ooxml::XltmChecker;
// Java: import cn.wps.io.file.checker.ooxml.XltxChecker
use io_libfileformat::io::file::checker::ooxml::XltxChecker;
// Java: import cn.wps.io.file.checker.pdf.PdfChecker
use io_libfileformat::cn::wps::io::file::checker::pdf::PdfChecker;
// Java: import cn.wps.io.file.checker.rtf.RtfChecker
use io_libfileformat::io::file::checker::rtf::RtfChecker;
// Java: import cn.wps.io.file.checker.xml.WordXml07Checker
use io_libfileformat::cn::wps::io::file::checker::xml::WordXml07Checker;
// Java: import cn.wps.io.file.checker.xml.XmlChecker
use io_libfileformat::cn::wps::io::file::checker::xml::XmlChecker;
// Java: import cn.wps.io.file.csv.CsvChecker
use io_libfileformat::io::file::csv::CsvChecker;
// Java: import cn.wps.io.file.parser.FileParser
use io_libfileformat::io::file::parser::FileParser;
// Java: import cn.wps.log.KmoLog
use util::cn::wps::log::KmoLog;
// Java: import cn.wps.moffice.crash.FileDamagedException
use util::cn::wps::moffice::crash::FileDamagedException;
// Java: import cn.wps.moffice.kfs.File
use kfs_kfs_app::cn::wps::moffice::kfs::mfs::File;
// Java: import cn.wps.moffice.kfs.FileInputStream
use kfs_kfs_app::cn::wps::moffice::kfs::mfs::FileInputStream;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
use kfs_kfs_app::cn::wps::moffice::kfs::RandomAccessFile;
// Java: import cn.wps.moffice.kfs.ifs.RafStream
use kfs_kfs_app::cn::wps::moffice::kfs::ifs::RafStream;
// Java: import cn.wps.moffice.kfs.zip.ZipEntry
use kfs_kfs_app::cn::wps::moffice::kfs::zip::ZipEntry;
// Java: import cn.wps.moffice.kfs.zip.ZipInputStream
use kfs_kfs_app::cn::wps::moffice::kfs::zip::ZipInputStream;
// Java: import cn.wps.moffice.util.FileUtil
use util::cn::wps::moffice::util::FileUtil;
// Java: import java.io.BufferedInputStream
// use std::io::BufReader; // java.io.BufferedInputStream
// Java: import java.io.FileNotFoundException
// use placeholder; // java.io.FileNotFoundException
// Java: import java.io.IOException
// use placeholder; // java.io.IOException
// Java: import java.io.InputStream
// use placeholder; // java.io.InputStream
// Java: import java.io.PushbackInputStream
// use placeholder; // java.io.PushbackInputStream
// Java: import java.util.concurrent.ExecutionException
// use placeholder; // java.util.concurrent.ExecutionException
// Java: import java.util.concurrent.ExecutorService
// use placeholder; // java.util.concurrent.ExecutorService
// Java: import java.util.concurrent.Executors
// use placeholder; // java.util.concurrent.Executors
// Java: import java.util.concurrent.FutureTask
// use placeholder; // java.util.concurrent.FutureTask
// Java: import java.util.concurrent.TimeUnit
// use placeholder; // java.util.concurrent.TimeUnit
// Java: import java.util.concurrent.TimeoutException
// use placeholder; // java.util.concurrent.TimeoutException
// Java: import org.apache.poi.hwpf.HWPFDocumentCore
use io_poi_compact::org::apache::poi::hwpf::HWPFDocumentCore;
// Java: import org.apache.poi.poifs.filesystem.DirectoryNode
use io_poi_compact::org::apache::poi::poifs::filesystem::DirectoryNode;
// Java: import org.apache.poi.poifs.filesystem.DocumentEntry
use io_poi_compact::org::apache::poi::poifs::filesystem::DocumentEntry;
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
use io_poi_compact::org::apache::poi::poifs::filesystem::POIFSFileSystem;

// Java field: String TAG
pub(crate) const TAG: &str = "";
// Java field: String CONTENT_TYPES_PART_NAME
pub(crate) const CONTENT_TYPES_PART_NAME: &str = "";
// Java field: String UOF_PART_NAME
pub(crate) const UOF_PART_NAME: &str = "";
// Java field: String UOF_ENCRYPT_PART_NAME
pub(crate) const UOF_ENCRYPT_PART_NAME: &str = "";
// Java field: String UOF_SECURITY_ENCRYPT_PART_NAME
pub(crate) const UOF_SECURITY_ENCRYPT_PART_NAME: &str = "";
// Java field: char EXTENSION_SEPARATOR
pub(crate) const EXTENSION_SEPARATOR: char = '.';
// Java field: String INVALID_NAME
pub(crate) const INVALID_NAME: &str = "";

// Java: public class Tools
pub struct Tools {
}

// Java: public class Tools
impl Tools {
    // Java: Tools()
    pub fn Tools(&self) {
      // Java: super();
      let _ = ();
    }

    // Java: static ZipInputStream getContentTypeZipInputStream(File file, RandomAccessFile raf)
    pub fn getContentTypeZipInputStream(file: Option<File::File>, raf: Option<RandomAccessFile::RandomAccessFile>) -> Option<ZipInputStream::ZipInputStream> {
      // Java: ZipInputStream inp = null != raf ? createZipInputStream(raf) : createZipInputStream(file);
      let inp = todo!();
      // Java: Assert.assertNotNull("inp should not be null", inp);
      Assert::assertNotNull("inp should not be null", &inp);
      // Java: if (null == inp)
      if false {
         // Java: return null;
         return None;
      // Java: else
      } else {
         // Java: ZipEntry contentTypeZipEntry = findContentTypeZipEntry(inp);
         let contentTypeZipEntry = todo!();
         // Java: return null == contentTypeZipEntry ? null : inp;
         return todo!();
      }
    }

    // Java: static ZipEntry findContentTypeZipEntry(ZipInputStream inp)
    pub fn findContentTypeZipEntry(&self, ZipInputStream inp) -> ZipEntry {
      // Java: Assert.assertNotNull("inp should not be null", inp);
      Assert::assertNotNull("inp should not be null", &inp);
      // Java: if (null == inp)
      if false {
         // Java: return null;
         return None;
      // Java: else
      } else {
         // Java: ZipEntry ret = null;
         let ret = todo!();
         // Java: while(true)
         loop {
            // Java: ZipEntry entry = null;
            let entry = todo!();
            // Java: entry = getNextEntry(inp);
            let entry = todo!();
            // Java: if (null == entry)
            if false {
               // Java: break;
               break;
            }
            // Java: String name = entry.getName();
            let name = todo!();
            // Java: checkEntryNameLegal(name);
            Self::checkEntryNameLegal(&name);
            // Java: Assert.assertNotNull("name should not be null", name);
            Assert::assertNotNull("name should not be null", &name);
            // Java: if (null != name && "[Content_Types].xml".equals(name))
            if false && false {
               // Java: ret = entry;
               let ret = entry;
               // Java: break;
               break;
            }
         }
         // Java: return ret;
         return ret;
      }
    }

    // Java: static ZipEntry getNextEntry(ZipInputStream inp)
    pub fn getNextEntry(&self, ZipInputStream inp) -> ZipEntry {
      // Java: ExecutorService executor = Executors.newSingleThreadExecutor();
      let executor = todo!();
      // Java: FutureTask<ZipEntry> futureTask = new ZipEntryFutureTask(new ZipEntryCallable(inp));
      let futureTask = todo!();
      // Java: executor.execute(futureTask);
      executor.execute(todo!());
      // Java: ZipEntry ret = null;
      let ret = todo!();
      // Java: try
      match (|| -> Result<(), Box<dyn std::error::Error>> {
         // Java: ret = (ZipEntry)futureTask.get(5000L, TimeUnit.MILLISECONDS);
         let ret = futureTask.get(5000);
         Ok(())
      // Java: catch (InterruptedException var10)
      })() { Ok(()) => {} Err(_e) => {
         // Java: futureTask.cancel(true);
         futureTask.cancel(true);
      // Java: catch (ExecutionException var11)
      // LLM JavaLine Aggregation: merged into the next Rust statement
         // Java: futureTask.cancel(true);
         futureTask.cancel(true);
      // Java: catch (TimeoutException var12)
      // LLM JavaLine Aggregation: merged into the next Rust statement
         // Java: futureTask.cancel(true);
         futureTask.cancel(true);
      }}
      // Java: finally
      {
         // Java: executor.shutdown();
         executor.shutdown();
      }
      // Java: return ret;
      return ret;
    }

    // Java: static FileInputStream createFileInputStream(File file)
    pub fn createFileInputStream(&self, File file) -> FileInputStream {
      // Java: FileInputStream is = null;
      let is = None;
      // Java: try
      match (|| -> Result<(), Box<dyn std::error::Error>> {
         // Java: is = new FileInputStream(file);
         let is = todo!();
         Ok(())
      // Java: catch (FileNotFoundException e)
      })() { Ok(()) => {} Err(_e) => {
         // Java: Log.e(TAG, "FileNotFoundException", e);
         Log::e(TAG, "FileNotFoundException");
      }}
      // Java: return is;
      return is;
    }

    // Java: static ZipInputStream createZipInputStream(File file)
    pub fn createZipInputStream(&self, File file) -> ZipInputStream {
      // Java: Assert.assertNotNull("file should not be null", file);
      Assert::assertNotNull("file should not be null", &file);
      // Java: FileInputStream is = createFileInputStream(file);
      let is = todo!();
      // Java: return null == is ? null : new ZipInputStream(is);
      return todo!();
    }

    // Java: static ZipInputStream createZipInputStream(RandomAccessFile raf)
    pub fn createZipInputStream(&self, RandomAccessFile raf) -> ZipInputStream {
      // Java: Assert.assertNotNull("raf should not be null", raf);
      Assert::assertNotNull("raf should not be null", &raf);
      // Java: InputStream inputStream = null;
      let inputStream = None;
      // Java: try
      match (|| -> Result<(), Box<dyn std::error::Error>> {
         // Java: inputStream = new RafStream(raf);
         let inputStream = todo!();
         Ok(())
      // Java: catch (IOException e)
      })() { Ok(()) => {} Err(_e) => {
         // Java: IOLog.e(TAG, e.getMessage());
         IOLog::e(TAG, "error");
      }}
      // Java: return null == inputStream ? null : new ZipInputStream(inputStream);
      return todo!();
    }

    // Java: static POIFSFileSystem createPOIFSFileSystem(FileInputStream is)
    pub fn createPOIFSFileSystem(&self, FileInputStream is) -> POIFSFileSystem {
      // Java: Assert.assertNotNull("is should not be null", is);
      Assert::assertNotNull("is should not be null", &is);
      // Java: POIFSFileSystem poifsFileSystem = null;
      let poifsFileSystem = None;
      // Java: try
      match (|| -> Result<(), Box<dyn std::error::Error>> {
         // Java: poifsFileSystem = HWPFDocumentCore.verifyAndBuildPOIFS(is);
         let poifsFileSystem = HWPFDocumentCore::verifyAndBuildPOIFS(todo!());
         Ok(())
      // Java: catch (Throwable e)
      })() { Ok(()) => {} Err(_e) => {
         // Java: Log.e(TAG, "Throwable:", e);
         Log::e(TAG, "Throwable:");
      }}
      // Java: return poifsFileSystem;
      return poifsFileSystem;
    }

    // Java: static POIFSFileSystem createPOIFSFileSystem(RandomAccessFile raf)
    pub fn createPOIFSFileSystem(&self, RandomAccessFile raf) -> POIFSFileSystem {
      // Java: Assert.assertNotNull("raf should not be null", raf);
      Assert::assertNotNull("raf should not be null", &raf);
      // Java: POIFSFileSystem poifsFileSystem = null;
      let poifsFileSystem = None;
      // Java: try
      match (|| -> Result<(), Box<dyn std::error::Error>> {
         // Java: raf.seek(0L);
         raf.seek(0);
         // Java: poifsFileSystem = HWPFDocumentCore.verifyAndBuildPOIFS(raf);
         let poifsFileSystem = HWPFDocumentCore::verifyAndBuildPOIFS(todo!());
         Ok(())
      // Java: catch (Throwable e)
      })() { Ok(()) => {} Err(_e) => {
         // Java: Log.e(TAG, "Throwable:", e);
         Log::e(TAG, "Throwable:");
      }}
      // Java: return poifsFileSystem;
      return poifsFileSystem;
    }

    // Java: static POIFSFileSystem createPOIFSFileSystem(File file)
    pub fn createPOIFSFileSystem(&self, File file) -> POIFSFileSystem {
      // Java: Assert.assertNotNull("is should not be null", file);
      Assert::assertNotNull("is should not be null", &file);
      // Java: POIFSFileSystem poifsFileSystem = null;
      let poifsFileSystem = None;
      // Java: try
      match (|| -> Result<(), Box<dyn std::error::Error>> {
         // Java: poifsFileSystem = HWPFDocumentCore.verifyAndBuildPOIFS(new RandomAccessFile(file, "r"));
         let poifsFileSystem = HWPFDocumentCore::verifyAndBuildPOIFS(todo!());
         Ok(())
      // Java: catch (Throwable e)
      })() { Ok(()) => {} Err(_e) => {
         // Java: Log.e(TAG, "Throwable:", e);
         Log::e(TAG, "Throwable:");
      }}
      // Java: return poifsFileSystem;
      return poifsFileSystem;
    }

    // Java: static DocumentEntry createDocumentEntry(String name, DirectoryNode directory)
    pub fn createDocumentEntry(&self, String name, DirectoryNode directory) -> DocumentEntry {
      // Java: Assert.assertNotNull("name should not be null", name);
      Assert::assertNotNull("name should not be null", &name);
      // Java: Assert.assertNotNull("directory should not be null", directory);
      Assert::assertNotNull("directory should not be null", &directory);
      // Java: DocumentEntry documentProps = null;
      let documentProps = None;
      // Java: try
      match (|| -> Result<(), Box<dyn std::error::Error>> {
         // Java: documentProps = (DocumentEntry)directory.getEntry(name);
         let documentProps = directory.getEntry(&name);
         Ok(())
      // Java: catch (FileNotFoundException var4)
      })() { Ok(()) => {} Err(_e) => {
         // Java: Log.d(TAG, "no such entry: " + name);
         Log::d(TAG, "no such entry");
      }}
      // Java: return documentProps;
      return documentProps;
    }

    // Java: static void newDocChecker(FileParser fileParser)
    pub fn newDocChecker(&self, FileParser fileParser) {
      // Java: if (null != fileParser)
      if false {
         // Java: if (null == fileParser.get_docChecker())
         if false {
            // Java: File file = fileParser.getFile();
            let file = todo!();
            // Java: RandomAccessFile raf = fileParser.getRaf();
            let raf = todo!();
            // Java: fileParser.set_docChecker(new DocChecker(file, raf, fileParser.getPOIFSFileSystem()));
            fileParser.set_docChecker(todo!());
         }
      }
    }

    // Java: static void newDocxChecker(FileParser fileParser)
    pub fn newDocxChecker(&self, FileParser fileParser) {
      // Java: if (null != fileParser)
      if false {
         // Java: if (null == fileParser.get_docxChecker())
         if false {
            // Java: File file = fileParser.getFile();
            let file = todo!();
            // Java: RandomAccessFile raf = fileParser.getRaf();
            let raf = todo!();
            // Java: fileParser.set_docxChecker(new DocxChecker(file, raf, fileParser.getPOIFSFileSystem()));
            todo!();
         }
      }
    }

    // Java: static void newDotxChecker(FileParser fileParser)
    pub fn newDotxChecker(&self, FileParser fileParser) {
      // Java: if (null != fileParser)
      if false {
         // Java: if (null == fileParser.get_dotxChecker())
         if false {
            // Java: File file = fileParser.getFile();
            let file = todo!();
            // Java: RandomAccessFile raf = fileParser.getRaf();
            let raf = todo!();
            // Java: fileParser.set_dotxChecker(new DotxChecker(file, raf, fileParser.getPOIFSFileSystem()));
            todo!();
         }
      }
    }

    // Java: static void newDocmChecker(FileParser fileParser)
    pub fn newDocmChecker(&self, FileParser fileParser) {
      // Java: if (null != fileParser)
      if false {
         // Java: if (null == fileParser.get_docmChecker())
         if false {
            // Java: File file = fileParser.getFile();
            let file = todo!();
            // Java: RandomAccessFile raf = fileParser.getRaf();
            let raf = todo!();
            // Java: fileParser.set_docmChecker(new DocmChecker(file, raf, fileParser.getPOIFSFileSystem()));
            todo!();
         }
      }
    }

    // Java: static void newDotmChecker(FileParser fileParser)
    pub fn newDotmChecker(&self, FileParser fileParser) {
      // Java: if (null != fileParser)
      if false {
         // Java: if (null == fileParser.get_dotmChecker())
         if false {
            // Java: File file = fileParser.getFile();
            let file = todo!();
            // Java: RandomAccessFile raf = fileParser.getRaf();
            let raf = todo!();
            // Java: fileParser.set_dotmChecker(new DotmChecker(file, raf, fileParser.getPOIFSFileSystem()));
            todo!();
         }
      }
    }

    // Java: static void newXlsChecker(FileParser fileParser)
    pub fn newXlsChecker(&self, FileParser fileParser) {
      // Java: if (null != fileParser)
      if false {
         // Java: if (null == fileParser.get_xlsChecker())
         if false {
            // Java: File file = fileParser.getFile();
            let file = todo!();
            // Java: RandomAccessFile raf = fileParser.getRaf();
            let raf = todo!();
            // Java: fileParser.set_xlsChecker(new XlsChecker(file, raf, fileParser.getPOIFSFileSystem()));
            todo!();
         }
      }
    }

    // Java: static void newXlsxChecker(FileParser fileParser)
    pub fn newXlsxChecker(&self, FileParser fileParser) {
      // Java: if (null != fileParser)
      if false {
         // Java: if (null == fileParser.get_xlsxChecker())
         if false {
            // Java: File file = fileParser.getFile();
            let file = todo!();
            // Java: RandomAccessFile raf = fileParser.getRaf();
            let raf = todo!();
            // Java: fileParser.set_xlsxChecker(new XlsxChecker(file, raf, fileParser.getPOIFSFileSystem()));
            todo!();
         }
      }
    }

    // Java: static void newXltxChecker(FileParser fileParser)
    pub fn newXltxChecker(&self, FileParser fileParser) {
      // Java: if (null != fileParser)
      if false {
         // Java: if (null == fileParser.get_xltxChecker())
         if false {
            // Java: File file = fileParser.getFile();
            let file = todo!();
            // Java: RandomAccessFile raf = fileParser.getRaf();
            let raf = todo!();
            // Java: fileParser.set_xltxChecker(new XltxChecker(file, raf, fileParser.getPOIFSFileSystem()));
            todo!();
         }
      }
    }

    // Java: static void newXlsmChecker(FileParser fileParser)
    pub fn newXlsmChecker(&self, FileParser fileParser) {
      // Java: if (null != fileParser)
      if false {
         // Java: if (null == fileParser.get_xlsmChecker())
         if false {
            // Java: File file = fileParser.getFile();
            let file = todo!();
            // Java: RandomAccessFile raf = fileParser.getRaf();
            let raf = todo!();
            // Java: fileParser.set_xlsmChecker(new XlsmChecker(file, raf, fileParser.getPOIFSFileSystem()));
            todo!();
         }
      }
    }

    // Java: static void newXltmChecker(FileParser fileParser)
    pub fn newXltmChecker(&self, FileParser fileParser) {
      // Java: if (null != fileParser)
      if false {
         // Java: if (null == fileParser.get_xltmChecker())
         if false {
            // Java: File file = fileParser.getFile();
            let file = todo!();
            // Java: RandomAccessFile raf = fileParser.getRaf();
            let raf = todo!();
            // Java: fileParser.set_xltmChecker(new XltmChecker(file, raf, fileParser.getPOIFSFileSystem()));
            todo!();
         }
      }
    }

    // Java: static void newXlsbChecker(FileParser fileParser)
    pub fn newXlsbChecker(&self, FileParser fileParser) {
      // Java: if (null != fileParser)
      if false {
         // Java: if (null == fileParser.get_xlsbChecker())
         if false {
            // Java: File file = fileParser.getFile();
            let file = todo!();
            // Java: RandomAccessFile raf = fileParser.getRaf();
            let raf = todo!();
            // Java: fileParser.set_xlsbChecker(new XlsbChecker(file, raf, fileParser.getPOIFSFileSystem()));
            todo!();
         }
      }
    }

    // Java: static void newXlamChecker(FileParser fileParser)
    pub fn newXlamChecker(&self, FileParser fileParser) {
      // Java: if (null != fileParser)
      if false {
         // Java: if (null == fileParser.get_xlamChecker())
         if false {
            // Java: File file = fileParser.getFile();
            let file = todo!();
            // Java: RandomAccessFile raf = fileParser.getRaf();
            let raf = todo!();
            // Java: fileParser.set_xlamChecker(new XlamChecker(file, raf, fileParser.getPOIFSFileSystem()));
            todo!();
         }
      }
    }

    // Java: static void newPPTChecker(FileParser fileParser)
    pub fn newPPTChecker(&self, FileParser fileParser) {
      // Java: if (null != fileParser)
      if false {
         // Java: if (null == fileParser.get_pptChecker())
         if false {
            // Java: File file = fileParser.getFile();
            let file = todo!();
            // Java: RandomAccessFile raf = fileParser.getRaf();
            let raf = todo!();
            // Java: fileParser.set_pptChecker(new PPTChecker(file, raf, fileParser.getPOIFSFileSystem()));
            todo!();
         }
      }
    }

    // Java: static void newPptxChecker(FileParser fileParser)
    pub fn newPptxChecker(&self, FileParser fileParser) {
      // Java: if (null != fileParser)
      if false {
         // Java: if (null == fileParser.get_pptxChecker())
         if false {
            // Java: File file = fileParser.getFile();
            let file = todo!();
            // Java: RandomAccessFile raf = fileParser.getRaf();
            let raf = todo!();
            // Java: fileParser.set_pptxCheker(new PptxChecker(file, raf, fileParser.getPOIFSFileSystem()));
            todo!();
         }
      }
    }

    // Java: static void newPotxChecker(FileParser fileParser)
    pub fn newPotxChecker(&self, FileParser fileParser) {
      // Java: if (null != fileParser)
      if false {
         // Java: if (null == fileParser.get_potxChecker())
         if false {
            // Java: File file = fileParser.getFile();
            let file = todo!();
            // Java: RandomAccessFile raf = fileParser.getRaf();
            let raf = todo!();
            // Java: fileParser.set_potxCheker(new PotxChecker(file, raf, fileParser.getPOIFSFileSystem()));
            todo!();
         }
      }
    }

    // Java: static void newPpsxChecker(FileParser fileParser)
    pub fn newPpsxChecker(&self, FileParser fileParser) {
      // Java: if (null != fileParser)
      if false {
         // Java: if (null == fileParser.get_ppsxChecker())
         if false {
            // Java: File file = fileParser.getFile();
            let file = todo!();
            // Java: RandomAccessFile raf = fileParser.getRaf();
            let raf = todo!();
            // Java: fileParser.set_ppsxCheker(new PpsxChecker(file, raf, fileParser.getPOIFSFileSystem()));
            todo!();
         }
      }
    }

    // Java: static void newPotmChecker(FileParser fileParser)
    pub fn newPotmChecker(&self, FileParser fileParser) {
      // Java: if (null != fileParser)
      if false {
         // Java: if (null == fileParser.get_potmChecker())
         if false {
            // Java: File file = fileParser.getFile();
            let file = todo!();
            // Java: RandomAccessFile raf = fileParser.getRaf();
            let raf = todo!();
            // Java: fileParser.set_potmCheker(new PotmChecker(file, raf, fileParser.getPOIFSFileSystem()));
            todo!();
         }
      }
    }

    // Java: static void newOOXMLChecker(FileParser fileParser)
    pub fn newOOXMLChecker(&self, FileParser fileParser) {
      // Java: if (null != fileParser)
      if false {
         // Java: if (null == fileParser.get_OOXMLChecker())
         if false {
            // Java: File file = fileParser.getFile();
            let file = todo!();
            // Java: RandomAccessFile raf = fileParser.getRaf();
            let raf = todo!();
            // Java: fileParser.set_OOXMLChecker(new OOXMLChecker(file, raf, fileParser.getPOIFSFileSystem()));
            todo!();
         }
      }
    }

    // Java: static void newMHTChecker(FileParser fileParser)
    pub fn newMHTChecker(&self, FileParser fileParser) {
      // Java: if (null != fileParser)
      if false {
         // Java: if (null == fileParser.get_mhtChecker())
         if false {
            // Java: File file = fileParser.getFile();
            let file = todo!();
            // Java: RandomAccessFile raf = fileParser.getRaf();
            let raf = todo!();
            // Java: fileParser.set_mhtChecker(new MHTChecker(null != file ? new BufferedInputStream(createFileInputStream(file)) : null, raf));
            todo!();
         }
      }
    }

    // Java: static void newHtmlChecker(FileParser fileParser)
    pub fn newHtmlChecker(&self, FileParser fileParser) {
      // Java: if (null != fileParser)
      if false {
         // Java: if (null == fileParser.get_htmlChecker())
         if false {
            // Java: File file = fileParser.getFile();
            let file = todo!();
            // Java: if (null != file)
            if false {
               // Java: fileParser.set_htmlChecker(new HtmlChecker(file));
               fileParser.set_htmlChecker(todo!());
            }
         }
      }
    }

    // Java: static void newXmlChecker(FileParser fileParser)
    pub fn newXmlChecker(&self, FileParser fileParser) {
      // Java: if (null != fileParser)
      if false {
         // Java: if (null == fileParser.get_xmlChecker())
         if false {
            // Java: File file = fileParser.getFile();
            let file = todo!();
            // Java: RandomAccessFile raf = fileParser.getRaf();
            let raf = todo!();
            // Java: fileParser.set_xmlChecker(new XmlChecker(null != file ? new BufferedInputStream(createFileInputStream(file)) : null, raf));
            todo!();
         }
      }
    }

    // Java: static void newWordXml07Checker(FileParser fileParser)
    pub fn newWordXml07Checker(&self, FileParser fileParser) {
      // Java: if (null != fileParser)
      if false {
         // Java: if (null == fileParser.get_wordXml07Checker())
         if false {
            // Java: File file = fileParser.getFile();
            let file = todo!();
            // Java: RandomAccessFile raf = fileParser.getRaf();
            let raf = todo!();
            // Java: fileParser.set_WordXml07Checker(new WordXml07Checker(null != file ? new BufferedInputStream(createFileInputStream(file)) : null, raf));
            todo!();
         }
      }
    }

    // Java: static void newPdfChecker(FileParser fileParser)
    pub fn newPdfChecker(&self, FileParser fileParser) {
      // Java: if (null != fileParser)
      if false {
         // Java: if (null == fileParser.get_pdfChecker())
         if false {
            // Java: File file = fileParser.getFile();
            let file = todo!();
            // Java: RandomAccessFile raf = fileParser.getRaf();
            let raf = todo!();
            // Java: fileParser.set_pdfChecker(new PdfChecker(null != file ? new PushbackInputStream(createFileInputStream(file), 512) : null, raf));
            todo!();
         }
      }
    }

    // Java: static void newCsvChecker(FileParser fileParser)
    pub fn newCsvChecker(&self, FileParser fileParser) {
      // Java: if (null != fileParser)
      if false {
         // Java: if (null == fileParser.get_csvChecker())
         if false {
            // Java: File file = fileParser.getFile();
            let file = todo!();
            // Java: RandomAccessFile raf = fileParser.getRaf();
            let raf = todo!();
            // Java: fileParser.set_csvChecker(new CsvChecker(file, raf));
            todo!();
         }
      }
    }

    // Java: static void newRtfChecker(FileParser fileParser)
    pub fn newRtfChecker(&self, FileParser fileParser) {
      // Java: if (null != fileParser)
      if false {
         // Java: if (null == fileParser.get_rtfChecker())
         if false {
            // Java: File file = fileParser.getFile();
            let file = todo!();
            // Java: RandomAccessFile raf = fileParser.getRaf();
            let raf = todo!();
            // Java: if ((null == file || !file.exists() || !file.isFile()) && null == raf)
            if (false || false || false) && false {
               // Java: fileParser.set_rtfChecker((RtfChecker)null);
               todo!();
            // Java: else
            } else {
               // Java: fileParser.set_rtfChecker(new RtfChecker(file, raf));
               todo!();
            }
         }
      }
    }

    // Java: static void newUof3Checker(FileParser fileParser)
    pub fn newUof3Checker(&self, FileParser fileParser) {
      // Java: if (null != fileParser)
      if false {
         // Java: if (null == fileParser.get_uof3Checker())
         if false {
            // Java: File file = fileParser.getFile();
            let file = todo!();
            // Java: RandomAccessFile raf = fileParser.getRaf();
            let raf = todo!();
            // Java: fileParser.set_uof3Checker(new Uof3Checker(file, raf, fileParser.getPOIFSFileSystem()));
            todo!();
         }
      }
    }

    // Java: static void newUot3Checker(FileParser fileParser)
    pub fn newUot3Checker(&self, FileParser fileParser) {
      // Java: if (null != fileParser)
      if false {
         // Java: if (null == fileParser.get_uot3Checker())
         if false {
            // Java: File file = fileParser.getFile();
            let file = todo!();
            // Java: RandomAccessFile raf = fileParser.getRaf();
            let raf = todo!();
            // Java: fileParser.set_uot3Checker(new Uot3Checker(file, raf, fileParser.getPOIFSFileSystem()));
            todo!();
         }
      }
    }

    // Java: static void newUos3Checker(FileParser fileParser)
    pub fn newUos3Checker(&self, FileParser fileParser) {
      // Java: if (null != fileParser)
      if false {
         // Java: if (null == fileParser.get_uos3Checker())
         if false {
            // Java: File file = fileParser.getFile();
            let file = todo!();
            // Java: RandomAccessFile raf = fileParser.getRaf();
            let raf = todo!();
            // Java: fileParser.set_uos3Checker(new Uos3Checker(file, raf, fileParser.getPOIFSFileSystem()));
            todo!();
         }
      }
    }

    // Java: static void newUop3Checker(FileParser fileParser)
    pub fn newUop3Checker(&self, FileParser fileParser) {
      // Java: if (null != fileParser)
      if false {
         // Java: if (null == fileParser.get_uop3Checker())
         if false {
            // Java: File file = fileParser.getFile();
            let file = todo!();
            // Java: RandomAccessFile raf = fileParser.getRaf();
            let raf = todo!();
            // Java: fileParser.set_uop3Checker(new Uop3Checker(file, raf, fileParser.getPOIFSFileSystem()));
            todo!();
         }
      }
    }

    // Java: static String getLowerCaseExtension(String absolutePath)
    pub fn getLowerCaseExtension(&self, String absolutePath) -> String {
      // Java: Assert.assertNotNull("absolutePath should not be null.", absolutePath);
      Assert::assertNotNull("absolutePath should not be null.", &absolutePath);
      // Java: String ext = absolutePath.substring(absolutePath.lastIndexOf(46) + 1);
      let ext = todo!();
      // Java: return toLowerCaseLite(ext);
      return Self::toLowerCaseLite(&ext);
    }

    // Java: static String toLowerCaseLite(String value)
    pub fn toLowerCaseLite(&self, String value) -> String {
      // Java: Assert.assertNotNull("value should not be null.", value);
      Assert::assertNotNull("value should not be null.", &value);
      // Java: int size = value.length();
      let size = todo!();
      // Java: StringBuilder sb = new StringBuilder();
      let sb = todo!();
      // Java: for(int i = 0; i < size; ++i)
      for _i in 0..1 {
         // Java: char ch = value.charAt(i);
         let ch = todo!();
         // Java: if ('A' <= ch && ch <= 'Z')
         if false && false {
            // Java: ch = (char)(ch + 32);
            let ch = todo!();
            // Java: sb.append(ch);
            sb.append(ch);
         // Java: else
         } else {
            // Java: sb.append(ch);
            sb.append(ch);
         }
      }
      // Java: return sb.toString();
      return sb.toString();
    }

    // Java: static void checkEntryNameLegal(String entryName)
    pub fn checkEntryNameLegal(&self, String entryName) {
      // Java: if (entryName.contains("../"))
      if false {
         // Java: throw new FileDamagedException();
         panic!("FileDamagedException");
      }
    }

    // Java: static InputStream createRafStream(RandomAccessFile raf)
    pub fn createRafStream(&self, RandomAccessFile raf) -> InputStream {
      // Java: try
      match (|| -> Result<(), Box<dyn std::error::Error>> {
         // Java: return new RafStream(raf);
         return todo!();
         Ok(())
      // Java: catch (IOException var2)
      })() { Ok(()) => {} Err(_e) => {
         // Java: return null;
         return None;
      }}
    }

    // Java: static ZipInputStream getUofInputStream(File file, RandomAccessFile raf)
    pub fn getUofInputStream(&self, File file, RandomAccessFile raf) -> ZipInputStream {
      // Java: ZipInputStream inp = null != raf ? createZipInputStream(raf) : createZipInputStream(file);
      let inp = todo!();
      // Java: Assert.assertNotNull("inp should not be null", inp);
      Assert::assertNotNull("inp should not be null", &inp);
      // Java: if (null == inp)
      if false {
         // Java: return null;
         return None;
      // Java: else
      } else {
         // Java: ZipEntry contentTypeZipEntry = findZipEntryInUof(inp, "uof.xml", (String)null);
         let contentTypeZipEntry = todo!();
         // Java: return null == contentTypeZipEntry ? null : inp;
         return todo!();
      }
    }

    // Java: static ZipEntry findZipEntryInUof(ZipInputStream inp, String partName, String partNameSec)
    pub fn findZipEntryInUof(&self, ZipInputStream inp, String partName, String partNameSec) -> ZipEntry {
      // Java: if (null == inp)
      if false {
         // Java: return null;
         return None;
      // Java: else
      } else {
         // Java: ZipEntry ret = null;
         let ret = None;
         // Java: while(true)
         loop {
            // Java: ZipEntry entry = null;
            let entry = None;
            // Java: entry = getNextEntry(inp);
            let entry = todo!();
            // Java: if (null == entry)
            if false {
               // Java: break;
               break;
            }
            // Java: String name = entry.getName();
            let name = todo!();
            // Java: if (null != name && (partName.equals(name) || name.equals(partNameSec)))
            if false && (false || false) {
               // Java: ret = entry;
               let ret = entry;
               // Java: break;
               break;
            }
         }
         // Java: return ret;
         return ret;
      }
    }

    // Java: static ZipInputStream getEncrytionXmlInUof(File file, RandomAccessFile raf)
    pub fn getEncrytionXmlInUof(&self, File file, RandomAccessFile raf) -> ZipInputStream {
      // Java: ZipInputStream inp = null != raf ? createZipInputStream(raf) : createZipInputStream(file);
      let inp = todo!();
      // Java: Assert.assertNotNull("inp should not be null", inp);
      Assert::assertNotNull("inp should not be null", &inp);
      // Java: if (null == inp)
      if false {
         // Java: return null;
         return None;
      // Java: else
      } else {
         // Java: ZipEntry encryptZipEntry = findZipEntryInUof(inp, "encryption.xml", "security/encryption.xml");
         let encryptZipEntry = todo!();
         // Java: return null == encryptZipEntry ? null : inp;
         return todo!();
      }
    }

    // Java: static String distinguishSuffixAndRename(String targetFile)
    pub fn distinguishSuffixAndRename(&self, String targetFile) -> String {
      // Java: try
      match (|| -> Result<(), Box<dyn std::error::Error>> {
         // Java: FileParser fileParser = new FileParser(new File(targetFile));
         let fileParser = todo!();
         // Java: fileParser.parse();
         fileParser.parse();
         // Java: int index = targetFile.lastIndexOf(".");
         let index = todo!();
         // Java: if (index <= 0)
         if false {
            // Java: return targetFile;
            return todo!();
         }
         // Java: String realPath = targetFile.substring(0, index);
         let realPath = todo!();
         // Java: if (fileParser.get_isDoc() != null && fileParser.get_isDoc())
         if false && false {
            // Java: realPath = realPath + ".doc";
            let realPath = todo!();
         // Java: else if (fileParser.get_isDocx() != null && fileParser.get_isDocx())
         } else if false && false {
            // Java: realPath = realPath + ".docx";
            let realPath = todo!();
         // Java: else if (fileParser.get_isPPT() != null && fileParser.get_isPPT())
         } else if false && false {
            // Java: realPath = realPath + ".ppt";
            let realPath = todo!();
         // Java: else if (fileParser.get_isPptx() != null && fileParser.get_isPptx())
         } else if false && false {
            // Java: realPath = realPath + ".pptx";
            let realPath = todo!();
         // Java: else if (fileParser.get_isXls() != null && fileParser.get_isXls())
         } else if false && false {
            // Java: realPath = realPath + ".xls";
            let realPath = todo!();
         // Java: else if (fileParser.get_isXlsx() != null && fileParser.get_isXlsx())
         } else if false && false {
            // Java: realPath = realPath + ".xlsx";
            let realPath = todo!();
         // Java: else
         } else {
            // Java: if (fileParser.get_isPdf() == null || !fileParser.get_isPdf())
            if false || false {
               // Java: return targetFile;
               return todo!();
            }
            // Java: realPath = realPath + ".pdf";
            let realPath = todo!();
         }
         // Java: FileUtil.moveFile(targetFile, realPath);
         FileUtil::moveFile(todo!());
         // Java: if (FileUtil.exist(realPath))
         if false {
            // Java: return realPath;
            return todo!();
         }
         Ok(())
      // Java: catch (Throwable e)
      })() { Ok(()) => {} Err(_e) => {
         // Java: e.printStackTrace();
         // LLM No Rust equivalent: Platform-Specific
         // Java: KmoLog.e(TAG, "parse File suffix", e);
         KmoLog::e(TAG, "error");
      }}
      // Java: return targetFile;
      return todo!();
    }

}
