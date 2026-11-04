// Java: import cn.wps.imagetopdf.PDFFile
use crate::cn::wps::imagetopdf::PDFFile::PDFFile;
// Java: import cn.wps.imagetopdf.obj.PDFInfo
// Java: import cn.wps.imagetopdf.obj.PDFPage
// Java: import cn.wps.moffice.kfs.File
// Java: import java.io.FilenameFilter
// Java: import java.io.IOException
// Java: import java.util.Date
// Java: import java.util.Locale
use std::fs;
use std::path::Path;

// Java: public class Tester
pub struct Tester {
}

// Java: public class Tester
impl Tester {
    // Java: Tester()
    pub fn new() -> Self {
        // Java: super();
        Tester {}
    }

    // Java: static void main(String[] args)
    pub fn main() {
        // Java: String filepath = "/Users/wps/Desktop/DIYPdf3.pdf";
        let filepath = "/Users/wps/Desktop/DIYPdf3.pdf";
        // Java: String imgDir = "/Users/wps/Pictures/Photo Booth";
        let img_dir = "/Users/wps/Pictures/Photo Booth";
        // Java: try
        match PDFFile::new(filepath) {
            Ok(mut pdf) => {
                // Java: System.out.println("new PDF:");
                println!("new PDF:");
                // Java: PDFFile pdf = new PDFFile(filepath);
                // Java: addImgs(imgDir, pdf);
                Self::addImgs_dir(img_dir, &mut pdf);
                // Java: PDFInfo info = pdf.getInfo();
                let info = pdf.getInfo();
                // Java: info.setLocale(Locale.SIMPLIFIED_CHINESE);
                info.setLocale("zh_CN");
                // Java: info.setAuthor("wps");
                info.setAuthor("wps");
                // Java: info.setProducer("Kingsoft Office");
                info.setProducer("Kingsoft Office");
                // Java: info.setCreationDate(new Date());
                info.setCreationDate("20260522");
                // Java: info.setModData(new Date());
                info.setModData("20260522");
                // Java: pdf.close();
                pdf.close();
                // Java: System.out.println("new PDF OK!");
                println!("new PDF OK!");
            }
            // Java: catch (IOException e)
            Err(e) => {
                // Java: e.printStackTrace();
                eprintln!("{}", e);
            }
        }
    }

    // Java: static int addImgs(String[] paths, PDFFile pdfFile)
    pub fn addImgs_paths(paths: &[&str], pdfFile: &mut PDFFile) -> i32 {
        // Java: if (paths != null && paths.length != 0)
        if !paths.is_empty() {
            // Java: int affect = 0;
            let mut affect = 0;
            // Java: for(String path : paths)
            for path in paths {
                // Java: PDFPage page = pdfFile.getPages().addPage(path);
                pdfFile.getPages().addPage_path(path);
                // Java: if (page != null)
                {
                    // Java: ++affect;
                    affect += 1;
                }
            }
            // Java: return affect;
            affect
        // Java: else
        } else {
            // Java: return -1;
            -1
        }
    }

    // Java: static int addImgs(String dirPath, PDFFile pdfFile)
    pub fn addImgs_dir(dirPath: &str, pdfFile: &mut PDFFile) -> i32 {
        // Java: File dir = new File(dirPath);
        let dir = Path::new(dirPath);
        // Java: if (!dir.isDirectory())
        if !dir.is_dir() {
            // Java: return -1;
            return -1;
        // Java: else
        } else {
            // Java: String[] imgs = dir.list(new FilenameFilter()
            // Java: public boolean accept(java.io.File dir, String name)
            // Java: name = name.toLowerCase();
            // Java: return name.endsWith(".jpg") || name.endsWith(".png");
            let entries = fs::read_dir(dir);
            let imgs: Vec<String> = match entries {
                Ok(entries) => entries
                    .filter_map(|e| e.ok())
                    .map(|e| e.file_name().to_string_lossy().to_string())
                    .filter(|name| {
                        let lower = name.to_lowercase();
                        lower.ends_with(".jpg") || lower.ends_with(".png")
                    })
                    .collect(),
                Err(_) => return -1,
            };
            // Java: if (imgs.length == 0)
            if imgs.is_empty() {
                // Java: System.out.println(dirPath + " -- 该目录下没有JPEG或PNG图片");
                println!("{} -- 该目录下没有JPEG或PNG图片", dirPath);
                // Java: return -1;
                return -1;
            // Java: else
            } else {
                // Java: for(int i = 0; i < imgs.length; ++i)
                let full_paths: Vec<String> = imgs.iter().map(|img| {
                    // Java: imgs[i] = dirPath + "/" + imgs[i];
                    format!("{}/{}", dirPath, img)
                }).collect();
                // Java: return addImgs(imgs, pdfFile);
                let refs: Vec<&str> = full_paths.iter().map(|s| s.as_str()).collect();
                Self::addImgs_paths(&refs, pdfFile)
            }
        }
    }
}
