// Java: import android.graphics.Bitmap
// Java: import cn.wps.imagetopdf.obj.DictObj
use crate::cn::wps::imagetopdf::obj::DictObj::DictObj;
// Java: import cn.wps.imagetopdf.obj.ImageObj
use crate::cn::wps::imagetopdf::obj::ImageObj::ImageObj;
// Java: import cn.wps.imagetopdf.obj.PDFPage
use crate::cn::wps::imagetopdf::obj::PDFPage::PDFPage;
// Java: import java.io.IOException
// Java: import java.util.LinkedList
// Java: import java.util.List
// Java: import java.util.TreeMap
use std::collections::BTreeMap;

use crate::cn::wps::imagetopdf::MyStringBuilder::MyStringBuilder;
use crate::cn::wps::imagetopdf::OutputStreamCounter::OutputStreamCounter;
use crate::cn::wps::imagetopdf::Utils::Utils;

// Java: public class PDFPages
pub struct PDFPages {
    // Java field: DictObj pages
    pub(crate) pages: DictObj,
    // Java field: OutputStreamCounter out
    pub(crate) out: *mut OutputStreamCounter,
    // Java field: TreeMap<Integer, Long> offsets
    pub(crate) offsets: *mut BTreeMap<i32, i64>,
    // Java field: PDFPage curPage
    pub(crate) curPage: Option<PDFPage>,
    // Java field: List<Integer> pageIDs
    pub(crate) pageIDs: Vec<i32>,
}

// Java: public class PDFPages
impl PDFPages {
    // Java: PDFPages(OutputStreamCounter out, TreeMap<Integer, Long> offsets)
    pub fn new(out: &mut OutputStreamCounter, offsets: &mut BTreeMap<i32, i64>) -> Self {
        // Java: super();
        // Java: this.out = out;
        // Java: this.offsets = offsets;
        PDFPages {
            pages: DictObj::new(""),
            out: out as *mut OutputStreamCounter,
            offsets: offsets as *mut BTreeMap<i32, i64>,
            curPage: None,
            pageIDs: Vec::new(),
        }
    }

    // Java: PDFPage addPage()
    pub fn addPage(&mut self) {
        // Java: if (this.curPage != null)
        if let Some(ref mut cur) = self.curPage {
            // Java: this.curPage.write();
            cur.write();
            // Java: this.curPage = null;
        }
        // Java: this.curPage = new PDFPage(this.out, this.offsets, this.pages);
        let page = unsafe { PDFPage::new(&mut *self.out, &mut *self.offsets, &self.pages) };
        // Java: this.pageIDs.add(this.curPage.getObjNum());
        self.pageIDs.push(page.base.getObjNum());
        self.curPage = Some(page);
        // Java: return this.curPage;
    }

    // Java: PDFPage addPage(String imgPath)
    pub fn addPage_path(&mut self, imgPath: &str) {
        // Java: if (this.curPage != null)
        if let Some(ref mut cur) = self.curPage {
            // Java: this.curPage.write();
            cur.write();
            // Java: this.curPage = null;
        }
        // Java: this.curPage = new PDFPage(this.out, this.offsets, this.pages, imgPath);
        let page = unsafe { PDFPage::from_path(&mut *self.out, &mut *self.offsets, &self.pages, imgPath) };
        // Java: this.pageIDs.add(this.curPage.getObjNum());
        self.pageIDs.push(page.base.getObjNum());
        self.curPage = Some(page);
        // Java: return this.curPage;
    }

    // Java: PDFPage addPage(Bitmap bitmap)
    pub fn addPage_bitmap(&mut self, w: i32, h: i32, data: Vec<u8>) {
        // Java: if (this.curPage != null)
        if let Some(ref mut cur) = self.curPage {
            // Java: this.curPage.write();
            cur.write();
            // Java: this.curPage = null;
        }
        // Java: this.curPage = new PDFPage(this.out, this.offsets, this.pages, bitmap);
        let page = unsafe { PDFPage::from_bitmap(&mut *self.out, &mut *self.offsets, &self.pages, w, h, data) };
        // Java: this.pageIDs.add(this.curPage.getObjNum());
        self.pageIDs.push(page.base.getObjNum());
        self.curPage = Some(page);
        // Java: return this.curPage;
    }

    // Java: PDFPage addPage(ImageObj img)
    pub fn addPage_img(&mut self, img: &mut ImageObj) {
        // Java: if (this.curPage != null)
        if let Some(ref mut cur) = self.curPage {
            // Java: this.curPage.write();
            cur.write();
            // Java: this.curPage = null;
        }
        // Java: this.curPage = new PDFPage(this.out, this.offsets, this.pages, img);
        let page = unsafe {
            PDFPage::from_img_obj(
                &mut *self.out,
                &mut *self.offsets,
                &self.pages,
                img.getImg().getW(),
                img.getImg().getH(),
                img.base.base.getObjNum(),
                &img.base.getBytes(),
            )
        };
        // Java: this.pageIDs.add(this.curPage.getObjNum());
        self.pageIDs.push(page.base.getObjNum());
        self.curPage = Some(page);
        // Java: return this.curPage;
    }

    // Java: int getObjNum()
    pub fn getObjNum(&self) -> i32 {
        // Java: return this.pages.getObjNum();
        self.pages.getObjNum()
    }

    // Java: void write()
    pub fn write(&mut self) {
        // Java: if (this.curPage != null)
        if let Some(ref mut cur) = self.curPage {
            // Java: this.curPage.write();
            cur.write();
            // Java: this.curPage = null;
        }
        self.curPage = None;
        // Java: MyStringBuilder sb = new MyStringBuilder();
        let mut sb = MyStringBuilder::new();
        // Java: sb.app("<</Type/Pages");
        sb.app_str("<</Type/Pages");
        // Java: sb.app("/Count " + this.getPageCounter());
        sb.app_str(&format!("/Count {}", self.getPageCounter()));
        // Java: StringBuilder kids = new StringBuilder();
        let mut kids = String::new();
        // Java: for(int i : this.getPages())
        for i in self.getPages() {
            // Java: kids.append(i + " 0 R ");
            kids.push_str(&format!("{} 0 R ", i));
        }
        // Java: sb.app("/Kids[ " + kids.toString() + "]\n>>");
        sb.app_str(&format!("/Kids[ {}]\\n>>", kids));
        // Java: this.pages.setDict(sb.toString());
        self.pages.setDict(&sb.toString());
        // Java: Utils.writeBytes(this.out, this.offsets, this.pages);
        unsafe {
            Utils::writeBytes_obj(&mut *self.out, &mut *self.offsets, &self.pages);
        }
    }

    // Java: int getPageCounter()
    pub fn getPageCounter(&self) -> i32 {
        // Java: return this.pageIDs.size();
        self.pageIDs.len() as i32
    }

    // Java: List<Integer> getPages()
    pub fn getPages(&self) -> &[i32] {
        // Java: return this.pageIDs;
        &self.pageIDs
    }
}
