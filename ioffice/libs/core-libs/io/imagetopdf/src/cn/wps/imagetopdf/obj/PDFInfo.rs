// Java: import cn.wps.imagetopdf.MyStringBuilder
use crate::cn::wps::imagetopdf::MyStringBuilder::MyStringBuilder;
// Java: import cn.wps.imagetopdf.OutputStreamCounter
use crate::cn::wps::imagetopdf::OutputStreamCounter::OutputStreamCounter;
// Java: import cn.wps.imagetopdf.Utils
use crate::cn::wps::imagetopdf::Utils::Utils;
// Java: import java.text.DateFormat
// Java: import java.util.Date
// Java: import java.util.Locale
// Java: import java.util.TreeMap
use std::collections::BTreeMap;

use super::DictObj::DictObj;

// Java: public class PDFInfo extends DictObj
pub struct PDFInfo {
    pub(crate) base: DictObj,
    // Java field: OutputStreamCounter out
    pub(crate) out: *mut OutputStreamCounter,
    // Java field: TreeMap<Integer, Long> offsets
    pub(crate) offsets: *mut BTreeMap<i32, i64>,
    // Java field: String creator
    pub(crate) creator: Option<String>,
    // Java field: String author
    pub(crate) author: Option<String>,
    // Java field: String producer
    pub(crate) producer: Option<String>,
    // Java field: Date creationDate
    pub(crate) creationDate: Option<String>,
    // Java field: Date modDate
    pub(crate) modDate: Option<String>,
    // Java field: String title
    pub(crate) title: Option<String>,
    // Java field: Locale locale
    pub(crate) locale: String,
}

// Java: public class PDFInfo extends DictObj
impl PDFInfo {
    // Java: PDFInfo(OutputStreamCounter out, TreeMap<Integer, Long> offsets)
    pub fn new(out: &mut OutputStreamCounter, offsets: &mut BTreeMap<i32, i64>) -> Self {
        // Java: super("");
        let base = DictObj::new("");
        // Java: this.locale = Locale.SIMPLIFIED_CHINESE;
        let locale = String::from("zh_CN");
        // Java: this.out = out;
        // Java: this.offsets = offsets;
        PDFInfo {
            base,
            out: out as *mut OutputStreamCounter,
            offsets: offsets as *mut BTreeMap<i32, i64>,
            creator: None,
            author: None,
            producer: None,
            creationDate: None,
            modDate: None,
            title: None,
            locale,
        }
    }

    // Java: void setLocale(Locale locale)
    pub fn setLocale(&mut self, locale: &str) {
        // Java: this.locale = locale;
        self.locale = String::from(locale);
    }

    // Java: void setTitle(String title)
    pub fn setTitle(&mut self, title: &str) {
        // Java: this.title = title;
        self.title = Some(String::from(title));
    }

    // Java: void setCreator(String creator)
    pub fn setCreator(&mut self, creator: &str) {
        // Java: this.creator = creator;
        self.creator = Some(String::from(creator));
    }

    // Java: void setAuthor(String author)
    pub fn setAuthor(&mut self, author: &str) {
        // Java: this.author = author;
        self.author = Some(String::from(author));
    }

    // Java: void setProducer(String producer)
    pub fn setProducer(&mut self, producer: &str) {
        // Java: this.producer = producer;
        self.producer = Some(String::from(producer));
    }

    // Java: void setCreationDate(Date creationDate)
    pub fn setCreationDate(&mut self, creationDate: &str) {
        // Java: this.creationDate = creationDate;
        self.creationDate = Some(String::from(creationDate));
    }

    // Java: void setModData(Date modDate)
    pub fn setModData(&mut self, modDate: &str) {
        // Java: this.modDate = modDate;
        self.modDate = Some(String::from(modDate));
    }

    // Java: void write()
    pub fn write(&mut self) {
        // Java: DateFormat df = DateFormat.getDateInstance(1, this.locale);
        // Java: MyStringBuilder sb = new MyStringBuilder("<<");
        let mut sb = MyStringBuilder::from_str("<<");
        // Java: if (this.producer != null)
        if let Some(ref producer) = self.producer {
            // Java: sb.app("/Producer(" + this.producer + ")");
            sb.app_str(&format!("/Producer({})", producer));
        }
        // Java: if (this.creator != null)
        if let Some(ref creator) = self.creator {
            // Java: sb.app("/Creator(" + this.creator + ")");
            sb.app_str(&format!("/Creator({})", creator));
        }
        // Java: if (this.title != null)
        if let Some(ref title) = self.title {
            // Java: sb.app("/Title(" + this.title + ")");
            sb.app_str(&format!("/Title({})", title));
        }
        // Java: if (this.author != null)
        if let Some(ref author) = self.author {
            // Java: sb.app("/Author(" + this.author + ")");
            sb.app_str(&format!("/Author({})", author));
        }
        // Java: if (this.creationDate != null)
        if let Some(ref creation_date) = self.creationDate {
            // Java: sb.app("/CreationDate(D:" + df.format(this.creationDate) + ")");
            sb.app_str(&format!("/CreationDate(D:{})", creation_date));
        }
        // Java: if (this.modDate != null)
        if let Some(ref mod_date) = self.modDate {
            // Java: sb.app("/ModDate(D:" + df.format(this.modDate) + ")");
            sb.app_str(&format!("/ModDate(D:{})", mod_date));
        }
        // Java: sb.app(">>");
        sb.app_str(">>");
        // Java: this.setDict(sb.toString());
        self.base.setDict(&sb.toString());
        // Java: Utils.writeBytes(this.out, this.offsets, (DictObj)this);
        unsafe {
            Utils::writeBytes_obj(&mut *self.out, &mut *self.offsets, &self.base);
        }
    }
}
