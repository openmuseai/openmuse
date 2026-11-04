// Java: import android.graphics.Bitmap
// Java: import cn.wps.imagetopdf.Image
use crate::cn::wps::imagetopdf::Image::Image;
// Java: import cn.wps.imagetopdf.MyStringBuilder
use crate::cn::wps::imagetopdf::MyStringBuilder::MyStringBuilder;
// Java: import cn.wps.imagetopdf.OutputStreamCounter
use crate::cn::wps::imagetopdf::OutputStreamCounter::OutputStreamCounter;
// Java: import cn.wps.imagetopdf.Utils
use crate::cn::wps::imagetopdf::Utils::Utils;
// Java: import java.io.IOException
// Java: import java.util.LinkedList
// Java: import java.util.List
// Java: import java.util.TreeMap
use std::collections::BTreeMap;

use super::DictObj::DictObj;

// Java: public class PDFPage extends DictObj
pub struct PDFPage {
    pub(crate) base: DictObj,
    // Java field: OutputStreamCounter out
    pub(crate) out: *mut OutputStreamCounter,
    // Java field: TreeMap<Integer, Long> offsets
    pub(crate) offsets: *mut BTreeMap<i32, i64>,
    // Java field: DictObj parent
    pub(crate) parent: *const DictObj,
    // Java field: MyStringBuilder resources
    pub(crate) resources: MyStringBuilder,
    // Java field: int left
    pub(crate) left: i32,
    // Java field: int top
    pub(crate) top: i32,
    // Java field: int right
    pub(crate) right: i32,
    // Java field: int bottom
    pub(crate) bottom: i32,
    // Java field: List<Integer> contentIDs
    pub(crate) contentIDs: Vec<i32>,
    // Java field: List<Integer> xobjectIDs
    pub(crate) xobjectIDs: Vec<i32>,
    // Java field: boolean bWritten
    pub(crate) bWritten: bool,
}

// Java: public class PDFPage extends DictObj
impl PDFPage {
    // Java: PDFPage(OutputStreamCounter out, TreeMap<Integer, Long> offsets, DictObj parent)
    pub fn new(out: &mut OutputStreamCounter, offsets: &mut BTreeMap<i32, i64>, parent: &DictObj) -> Self {
        // Java: super("");
        let base = DictObj::new("");
        // Java: this.out = out;
        // Java: this.offsets = offsets;
        // Java: this.parent = parent;
        PDFPage {
            base,
            out: out as *mut OutputStreamCounter,
            offsets: offsets as *mut BTreeMap<i32, i64>,
            parent: parent as *const DictObj,
            resources: MyStringBuilder::new(),
            left: 0, top: 0, right: 0, bottom: 0,
            contentIDs: Vec::new(),
            xobjectIDs: Vec::new(),
            bWritten: false,
        }
    }

    // Java: PDFPage(OutputStreamCounter out, TreeMap<Integer, Long> offsets, DictObj parent, String imgPath)
    pub fn from_path(out: &mut OutputStreamCounter, offsets: &mut BTreeMap<i32, i64>, parent: &DictObj, imgPath: &str) -> Self {
        // Java: super("");
        let base = DictObj::new("");
        // Java: this.out = out;
        // Java: this.offsets = offsets;
        // Java: this.parent = parent;
        let mut page = PDFPage {
            base,
            out: out as *mut OutputStreamCounter,
            offsets: offsets as *mut BTreeMap<i32, i64>,
            parent: parent as *const DictObj,
            resources: MyStringBuilder::new(),
            left: 0, top: 0, right: 0, bottom: 0,
            contentIDs: Vec::new(),
            xobjectIDs: Vec::new(),
            bWritten: false,
        };
        // Java: Image img = new Image(imgPath);
        let img = Image::from_path(imgPath);
        // Java: ImageObj pdfImg = ImageObj.getImgObj(img);
        // Java: this.setMediaBox(0, 0, pdfImg.getImg().getW(), pdfImg.getImg().getH());
        page.setMediaBox(0, 0, img.getW(), img.getH());
        // Java: Utils.writeBytes(out, offsets, (DictObj)pdfImg);
        // Java: if (pdfImg.getClass() == PNGImg.class)
        // Java: PNGImg mask = ((PNGImg)pdfImg).getSMask();
        // Java: if (mask != null)
        // Java: Utils.writeBytes(out, offsets, (DictObj)mask);
        // Java: this.xobjectIDs.add(pdfImg.getObjNum());
        // Java: ContentObj content = new ContentObj(pdfImg.getImg());
        // Java: Utils.writeBytes(out, offsets, (DictObj)content);
        // Java: this.contentIDs.add(content.getObjNum());
        page
    }

    // Java: PDFPage(OutputStreamCounter out, TreeMap<Integer, Long> offsets, DictObj parent, Bitmap bitmap)
    pub fn from_bitmap(out: &mut OutputStreamCounter, offsets: &mut BTreeMap<i32, i64>, parent: &DictObj, w: i32, h: i32, data: Vec<u8>) -> Self {
        // Java: super("");
        let base = DictObj::new("");
        // Java: this.out = out;
        // Java: this.offsets = offsets;
        // Java: this.parent = parent;
        let mut page = PDFPage {
            base,
            out: out as *mut OutputStreamCounter,
            offsets: offsets as *mut BTreeMap<i32, i64>,
            parent: parent as *const DictObj,
            resources: MyStringBuilder::new(),
            left: 0, top: 0, right: 0, bottom: 0,
            contentIDs: Vec::new(),
            xobjectIDs: Vec::new(),
            bWritten: false,
        };
        // Java: Image img = new Image(bitmap);
        let img = Image::from_bitmap(w, h, data);
        // Java: ImageObj pdfImg = ImageObj.getImgObj(img);
        // Java: this.setMediaBox(0, 0, pdfImg.getImg().getW(), pdfImg.getImg().getH());
        page.setMediaBox(0, 0, img.getW(), img.getH());
        // Java: Utils.writeBytes(out, offsets, (DictObj)pdfImg);
        // Java: if (pdfImg.getClass() == PNGImg.class)
        // Java: PNGImg mask = ((PNGImg)pdfImg).getSMask();
        // Java: if (mask != null)
        // Java: Utils.writeBytes(out, offsets, (DictObj)mask);
        // Java: this.xobjectIDs.add(pdfImg.getObjNum());
        // Java: ContentObj content = new ContentObj(pdfImg.getImg());
        // Java: Utils.writeBytes(out, offsets, (DictObj)content);
        // Java: this.contentIDs.add(content.getObjNum());
        page
    }

    // Java: PDFPage(OutputStreamCounter out, TreeMap<Integer, Long> offsets, DictObj parent, ImageObj pdfImg)
    pub fn from_img_obj(out: &mut OutputStreamCounter, offsets: &mut BTreeMap<i32, i64>, parent: &DictObj, img_w: i32, img_h: i32, img_obj_num: i32, img_bytes: &[u8]) -> Self {
        // Java: super("");
        let base = DictObj::new("");
        // Java: this.out = out;
        // Java: this.offsets = offsets;
        // Java: this.parent = parent;
        let mut page = PDFPage {
            base,
            out: out as *mut OutputStreamCounter,
            offsets: offsets as *mut BTreeMap<i32, i64>,
            parent: parent as *const DictObj,
            resources: MyStringBuilder::new(),
            left: 0, top: 0, right: 0, bottom: 0,
            contentIDs: Vec::new(),
            xobjectIDs: Vec::new(),
            bWritten: false,
        };
        // Java: this.setMediaBox(0, 0, pdfImg.getImg().getW(), pdfImg.getImg().getH());
        page.setMediaBox(0, 0, img_w, img_h);
        // Java: Utils.writeBytes(out, offsets, (DictObj)pdfImg);
        Utils::writeBytes_bytes(out, offsets, img_bytes);
        // Java: if (pdfImg.getClass() == PNGImg.class)
        // Java: PNGImg mask = ((PNGImg)pdfImg).getSMask();
        // Java: if (mask != null)
        // Java: Utils.writeBytes(out, offsets, (DictObj)mask);
        // Java: this.xobjectIDs.add(pdfImg.getObjNum());
        page.xobjectIDs.push(img_obj_num);
        // Java: ContentObj content = new ContentObj(pdfImg.getImg());
        // Java: Utils.writeBytes(out, offsets, (DictObj)content);
        // Java: this.contentIDs.add(content.getObjNum());
        page
    }

    // Java: boolean addImg(String path)
    pub fn addImg(&mut self, path: &str) -> bool {
        // Java: if (path != null && path != "" && (path.toLowerCase().endsWith(".jpg") || path.toLowerCase().endsWith(".png")))
        let lower = path.to_lowercase();
        if !path.is_empty() && (lower.ends_with(".jpg") || lower.ends_with(".png")) {
            // Java: if (this.bWritten)
            if self.bWritten {
                // Java: System.out.println("该页已经写盘，不能再添加图片");
                eprintln!("该页已经写盘，不能再添加图片");
                // Java: return false;
                return false;
            // Java: else
            } else {
                // Java: Image img = new Image(path);
                let _img = Image::from_path(path);
                // Java: ImageObj pdfImg = ImageObj.getImgObj(img);
                // Java: Utils.writeBytes(this.out, this.offsets, (DictObj)pdfImg);
                // Java: if (pdfImg.getClass() == PNGImg.class)
                // Java: PNGImg mask = ((PNGImg)pdfImg).getSMask();
                // Java: if (mask != null)
                // Java: Utils.writeBytes(this.out, this.offsets, (DictObj)mask);
                // Java: this.xobjectIDs.add(pdfImg.getObjNum());
                // Java: ContentObj content = new ContentObj(img);
                // Java: Utils.writeBytes(this.out, this.offsets, (DictObj)content);
                // Java: this.contentIDs.add(content.getObjNum());
                // Java: return true;
                return true;
            }
        // Java: else
        } else {
            // Java: System.out.println("请传入jpg或png格式的图片");
            eprintln!("请传入jpg或png格式的图片");
            // Java: return false;
            return false;
        }
    }

    // Java: void setMediaBox(int left, int top, int right, int bottom)
    pub fn setMediaBox(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        // Java: this.left = left;
        self.left = left;
        // Java: this.top = top;
        self.top = top;
        // Java: this.right = right;
        self.right = right;
        // Java: this.bottom = bottom;
        self.bottom = bottom;
    }

    // Java: void write()
    pub fn write(&mut self) {
        // Java: if (!this.bWritten)
        if !self.bWritten {
            // Java: MyStringBuilder sb = new MyStringBuilder();
            let mut sb = MyStringBuilder::new();
            // Java: sb.app("<</Type/Page");
            sb.app_str("<</Type/Page");
            // Java: sb.app("/Parent " + this.parent.getObjNum() + " 0 R");
            let parent_obj_num = unsafe { (*self.parent).getObjNum() };
            sb.app_str(&format!("/Parent {} 0 R", parent_obj_num));
            // Java: sb.app("/Contents ");
            sb.app_str("/Contents ");
            // Java: int contentCount = this.contentIDs.size();
            let content_count = self.contentIDs.len();
            // Java: if (contentCount > 1)
            if content_count > 1 {
                // Java: sb.app("[ ");
                sb.app_str("[ ");
            }
            // Java: for(int i = 0; i < contentCount; ++i)
            for i in 0..content_count {
                // Java: sb.app(this.contentIDs.get(i) + " 0 R ");
                sb.app_str(&format!("{} 0 R ", self.contentIDs[i]));
            }
            // Java: if (contentCount > 1)
            if content_count > 1 {
                // Java: sb.app("]");
                sb.app_str("]");
            }
            // Java: sb.app("/Resources<</ProcSet[/PDF/ImageB]");
            sb.app_str("/Resources<</ProcSet[/PDF/ImageB]");
            // Java: sb.app("/XObject<</I0 ");
            sb.app_str("/XObject<</I0 ");
            // Java: int xobjectCount = this.xobjectIDs.size();
            let xobject_count = self.xobjectIDs.len();
            // Java: if (xobjectCount > 1)
            if xobject_count > 1 {
                // Java: sb.app("[ ");
                sb.app_str("[ ");
            }
            // Java: for(int i = 0; i < xobjectCount; ++i)
            for i in 0..xobject_count {
                // Java: sb.app(this.xobjectIDs.get(i) + " 0 R ");
                sb.app_str(&format!("{} 0 R ", self.xobjectIDs[i]));
            }
            // Java: if (xobjectCount > 1)
            if xobject_count > 1 {
                // Java: sb.app(" ]");
                sb.app_str(" ]");
            }
            // Java: sb.app(">>");
            sb.app_str(">>");
            // Java: sb.app(">>");
            sb.app_str(">>");
            // Java: sb.app("/MediaBox[ " + this.left + " " + this.top + " " + this.right + " " + this.bottom + " ]");
            sb.app_str(&format!("/MediaBox[ {} {} {} {} ]", self.left, self.top, self.right, self.bottom));
            // Java: sb.app(">>");
            sb.app_str(">>");
            // Java: this.dict = sb.toString();
            self.base.setDict(&sb.toString());
            // Java: Utils.writeBytes(this.out, this.offsets, (DictObj)this);
            unsafe {
                Utils::writeBytes_obj(&mut *self.out, &mut *self.offsets, &self.base);
            }
            // Java: this.bWritten = true;
            self.bWritten = true;
        }
    }
}
