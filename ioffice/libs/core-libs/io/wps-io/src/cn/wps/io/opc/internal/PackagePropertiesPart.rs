#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.opc.OPCPackage
// Java: import cn.wps.io.opc.PackagePart
// Java: import cn.wps.io.opc.PackagePartName
// Java: import cn.wps.io.opc.PackageProperties
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidFormatException
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidOperationException
// Java: import cn.wps.io.xwpf.util.Nullable
// Java: import java.io.InputStream
// Java: import java.io.OutputStream
// Java: import java.text.ParsePosition
// Java: import java.text.SimpleDateFormat
// Java: import java.util.Date
// Java: import java.util.TimeZone

use crate::io::opc::PackagePart::PackagePart;
use crate::io::opc::PackagePartName::PackagePartName;
use crate::io::opc::OPCPackage::OPCPackage;
use crate::io::opc::internal::ContentType::ContentType;
use crate::io::xwpf::util::Nullable::Nullable;

// Java field: String NAMESPACE_DC_URI
pub const NAMESPACE_DC_URI: &str = "http://purl.org/dc/elements/1.1/";
// Java field: String NAMESPACE_CP_URI
pub const NAMESPACE_CP_URI: &str = "http://schemas.openxmlformats.org/package/2006/metadata/core-properties";
// Java field: String NAMESPACE_DCTERMS_URI
pub const NAMESPACE_DCTERMS_URI: &str = "http://purl.org/dc/terms/";
// Java field: String NAMESPACE_XSI_URI
pub const NAMESPACE_XSI_URI: &str = "http://www.w3.org/2001/XMLSchema-instance";

// Java: public class PackagePropertiesPart extends PackagePart implements PackageProperties
pub struct PackagePropertiesPart {
    pub(crate) base: PackagePart,
    // Java field: Nullable<String> category
    pub(crate) category: Nullable<String>,
    // Java field: Nullable<String> contentStatus
    pub(crate) contentStatus: Nullable<String>,
    // Java field: Nullable<String> contentType
    pub(crate) contentType: Nullable<String>,
    // Java field: Nullable<Date> created
    pub(crate) created: Nullable<String>,
    // Java field: Nullable<String> creator
    pub(crate) creator: Nullable<String>,
    // Java field: Nullable<String> description
    pub(crate) description: Nullable<String>,
    // Java field: Nullable<String> identifier
    pub(crate) identifier: Nullable<String>,
    // Java field: Nullable<String> keywords
    pub(crate) keywords: Nullable<String>,
    // Java field: Nullable<String> language
    pub(crate) language: Nullable<String>,
    // Java field: Nullable<String> lastModifiedBy
    pub(crate) lastModifiedBy: Nullable<String>,
    // Java field: Nullable<Date> lastPrinted
    pub(crate) lastPrinted: Nullable<String>,
    // Java field: Nullable<Date> modified
    pub(crate) modified: Nullable<String>,
    // Java field: Nullable<String> revision
    pub(crate) revision: Nullable<String>,
    // Java field: Nullable<String> subject
    pub(crate) subject: Nullable<String>,
    // Java field: Nullable<String> title
    pub(crate) title: Nullable<String>,
    // Java field: Nullable<String> version
    pub(crate) version: Nullable<String>,
}

// Java: public class PackagePropertiesPart extends PackagePart implements PackageProperties
impl PackagePropertiesPart {
    // Java: PackagePropertiesPart(OPCPackage pack, PackagePartName partName)
    pub fn PackagePropertiesPart(pack: Option<*const OPCPackage>, part_name: PackagePartName) -> Self {
        // Java: super(pack, partName, "application/vnd.openxmlformats-package.core-properties+xml");
        PackagePropertiesPart {
            base: PackagePart::new(pack, part_name, ContentType::new("application/vnd.openxmlformats-package.core-properties+xml").unwrap()),
            category: Nullable::new(),
            contentStatus: Nullable::new(),
            contentType: Nullable::new(),
            created: Nullable::new(),
            creator: Nullable::new(),
            description: Nullable::new(),
            identifier: Nullable::new(),
            keywords: Nullable::new(),
            language: Nullable::new(),
            lastModifiedBy: Nullable::new(),
            lastPrinted: Nullable::new(),
            modified: Nullable::new(),
            revision: Nullable::new(),
            subject: Nullable::new(),
            title: Nullable::new(),
            version: Nullable::new(),
        }
    }

    // rust: no-java（Rust 惯用别名，crate 内沿用 `new`）
    pub fn new(pack: Option<*const OPCPackage>, part_name: PackagePartName) -> Self {
        Self::PackagePropertiesPart(pack, part_name)
    }

    // Java: Nullable<String> getCategoryProperty()
    pub fn getCategoryProperty(&self) -> &Nullable<String> {
        // Java: return this.category;
        &self.category
    }

    // Java: Nullable<String> getContentStatusProperty()
    pub fn getContentStatusProperty(&self) -> &Nullable<String> {
        // Java: return this.contentStatus;
        &self.contentStatus
    }

    // Java: Nullable<String> getContentTypeProperty()
    pub fn getContentTypeProperty(&self) -> &Nullable<String> {
        // Java: return this.contentType;
        &self.contentType
    }

    // Java: Nullable<Date> getCreatedProperty()
    pub fn getCreatedProperty(&self) -> &Nullable<String> {
        // Java: return this.created;
        &self.created
    }

    // Java: String getCreatedPropertyString()
    pub fn getCreatedPropertyString(&self) -> String {
        // Java: return this.getDateValue(this.created);
        self.getDateValue(&self.created)
    }

    // Java: Nullable<String> getCreatorProperty()
    pub fn getCreatorProperty(&self) -> &Nullable<String> {
        // Java: return this.creator;
        &self.creator
    }

    // Java: Nullable<String> getDescriptionProperty()
    pub fn getDescriptionProperty(&self) -> &Nullable<String> {
        // Java: return this.description;
        &self.description
    }

    // Java: Nullable<String> getIdentifierProperty()
    pub fn getIdentifierProperty(&self) -> &Nullable<String> {
        // Java: return this.identifier;
        &self.identifier
    }

    // Java: Nullable<String> getKeywordsProperty()
    pub fn getKeywordsProperty(&self) -> &Nullable<String> {
        // Java: return this.keywords;
        &self.keywords
    }

    // Java: Nullable<String> getLanguageProperty()
    pub fn getLanguageProperty(&self) -> &Nullable<String> {
        // Java: return this.language;
        &self.language
    }

    // Java: Nullable<String> getLastModifiedByProperty()
    pub fn getLastModifiedByProperty(&self) -> &Nullable<String> {
        // Java: return this.lastModifiedBy;
        &self.lastModifiedBy
    }

    // Java: Nullable<Date> getLastPrintedProperty()
    pub fn getLastPrintedProperty(&self) -> &Nullable<String> {
        // Java: return this.lastPrinted;
        &self.lastPrinted
    }

    // Java: String getLastPrintedPropertyString()
    pub fn getLastPrintedPropertyString(&self) -> String {
        // Java: return this.getDateValue(this.lastPrinted);
        self.getDateValue(&self.lastPrinted)
    }

    // Java: Nullable<Date> getModifiedProperty()
    pub fn getModifiedProperty(&self) -> &Nullable<String> {
        // Java: return this.modified;
        &self.modified
    }

    // Java: String getModifiedPropertyString()
    pub fn getModifiedPropertyString(&self) -> String {
        // Java: return this.modified.hasValue() ? this.getDateValue(this.modified) : this.getDateValue(new Nullable(new Date()));
        if self.modified.hasValue() {
            self.getDateValue(&self.modified)
        } else {
            String::new()
        }
    }

    // Java: Nullable<String> getRevisionProperty()
    pub fn getRevisionProperty(&self) -> &Nullable<String> {
        // Java: return this.revision;
        &self.revision
    }

    // Java: Nullable<String> getSubjectProperty()
    pub fn getSubjectProperty(&self) -> &Nullable<String> {
        // Java: return this.subject;
        &self.subject
    }

    // Java: Nullable<String> getTitleProperty()
    pub fn getTitleProperty(&self) -> &Nullable<String> {
        // Java: return this.title;
        &self.title
    }

    // Java: Nullable<String> getVersionProperty()
    pub fn getVersionProperty(&self) -> &Nullable<String> {
        // Java: return this.version;
        &self.version
    }

    // Java: void setCategoryProperty(String category)
    pub fn setCategoryProperty(&mut self, category: Option<&str>) {
        // Java: this.category = setStringValue(category);
        self.category = Self::setStringValue(category);
    }

    // Java: void setContentStatusProperty(String contentStatus)
    pub fn setContentStatusProperty(&mut self, content_status: Option<&str>) {
        // Java: this.contentStatus = setStringValue(contentStatus);
        self.contentStatus = Self::setStringValue(content_status);
    }

    // Java: void setContentTypeProperty(String contentType)
    pub fn setContentTypeProperty(&mut self, content_type: Option<&str>) {
        // Java: this.contentType = setStringValue(contentType);
        self.contentType = Self::setStringValue(content_type);
    }

    // Java: void setCreatedProperty(String created)
    pub fn setCreatedProperty_str(&mut self, created: Option<&str>) {
        // Java: try
        // Java: this.created = this.setDateValue(created);
        // Java: catch (InvalidFormatException var3)
        if let Some(s) = created {
            if let Ok(v) = self.setDateValue(s) {
                self.created = v;
            }
        }
    }

    // Java: void setCreatedProperty(Nullable<Date> created)
    pub fn setCreatedProperty(&mut self, created: Nullable<String>) {
        // Java: if (created.hasValue())
        if created.hasValue() {
            // Java: this.created = created;
            self.created = created;
        }
    }

    // Java: void setCreatorProperty(String creator)
    pub fn setCreatorProperty(&mut self, creator: Option<&str>) {
        // Java: this.creator = setStringValue(creator);
        self.creator = Self::setStringValue(creator);
    }

    // Java: void setDescriptionProperty(String description)
    pub fn setDescriptionProperty(&mut self, description: Option<&str>) {
        // Java: this.description = setStringValue(description);
        self.description = Self::setStringValue(description);
    }

    // Java: void setIdentifierProperty(String identifier)
    pub fn setIdentifierProperty(&mut self, identifier: Option<&str>) {
        // Java: this.identifier = setStringValue(identifier);
        self.identifier = Self::setStringValue(identifier);
    }

    // Java: void setKeywordsProperty(String keywords)
    pub fn setKeywordsProperty(&mut self, keywords: Option<&str>) {
        // Java: this.keywords = setStringValue(keywords);
        self.keywords = Self::setStringValue(keywords);
    }

    // Java: void setLanguageProperty(String language)
    pub fn setLanguageProperty(&mut self, language: Option<&str>) {
        // Java: this.language = setStringValue(language);
        self.language = Self::setStringValue(language);
    }

    // Java: void setLastModifiedByProperty(String lastModifiedBy)
    pub fn setLastModifiedByProperty(&mut self, last_modified_by: Option<&str>) {
        // Java: this.lastModifiedBy = setStringValue(lastModifiedBy);
        self.lastModifiedBy = Self::setStringValue(last_modified_by);
    }

    // Java: void setLastPrintedProperty(String lastPrinted)
    pub fn setLastPrintedProperty_str(&mut self, last_printed: Option<&str>) {
        // Java: try
        // Java: this.lastPrinted = this.setDateValue(lastPrinted);
        // Java: catch (InvalidFormatException var3)
        if let Some(s) = last_printed {
            if let Ok(v) = self.setDateValue(s) {
                self.lastPrinted = v;
            }
        }
    }

    // Java: void setLastPrintedProperty(Nullable<Date> lastPrinted)
    pub fn setLastPrintedProperty(&mut self, last_printed: Nullable<String>) {
        // Java: if (lastPrinted.hasValue())
        if last_printed.hasValue() {
            // Java: this.lastPrinted = lastPrinted;
            self.lastPrinted = last_printed;
        }
    }

    // Java: void setModifiedProperty(String modified)
    pub fn setModifiedProperty_str(&mut self, modified: Option<&str>) {
        // Java: try
        // Java: this.modified = this.setDateValue(modified);
        // Java: catch (InvalidFormatException var3)
        if let Some(s) = modified {
            if let Ok(v) = self.setDateValue(s) {
                self.modified = v;
            }
        }
    }

    // Java: void setModifiedProperty(Nullable<Date> modified)
    pub fn setModifiedProperty(&mut self, modified: Nullable<String>) {
        // Java: if (modified.hasValue())
        if modified.hasValue() {
            // Java: this.modified = modified;
            self.modified = modified;
        }
    }

    // Java: void setRevisionProperty(String revision)
    pub fn setRevisionProperty(&mut self, revision: Option<&str>) {
        // Java: this.revision = setStringValue(revision);
        self.revision = Self::setStringValue(revision);
    }

    // Java: void setSubjectProperty(String subject)
    pub fn setSubjectProperty(&mut self, subject: Option<&str>) {
        // Java: this.subject = setStringValue(subject);
        self.subject = Self::setStringValue(subject);
    }

    // Java: void setTitleProperty(String title)
    pub fn setTitleProperty(&mut self, title: Option<&str>) {
        // Java: this.title = setStringValue(title);
        self.title = Self::setStringValue(title);
    }

    // Java: void setVersionProperty(String version)
    pub fn setVersionProperty(&mut self, version: Option<&str>) {
        // Java: this.version = setStringValue(version);
        self.version = Self::setStringValue(version);
    }

    // Java: static Nullable<String> setStringValue(String s)
    pub fn setStringValue(s: Option<&str>) -> Nullable<String> {
        // Java: return s != null && !s.equals("") ? new Nullable(s) : new Nullable();
        match s {
            Some(v) if !v.is_empty() => Nullable::new_with_value(v.to_string()),
            _ => Nullable::new(),
        }
    }

    // Java: Nullable<Date> setDateValue(String s)
    pub fn setDateValue(&self, s: &str) -> Result<Nullable<String>, String> {
        // Java: if (s != null && !s.equals(""))
        if !s.is_empty() {
            // Java: SimpleDateFormat df = new SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ss'Z'");
            let mut df = DateFormatUtc::new("yyyy-MM-dd'T'HH:mm:ss'Z'");
            // Java: df.setTimeZone(TimeZone.getTimeZone("UTC"));
            df.setTimeZone("UTC");
            // Java: Date d = df.parse(s, new ParsePosition(0));
            let d = df.parse(s);
            // Java: if (d == null)
            // Java: throw new InvalidFormatException("Date not well formated");
            // Java: else
            // Java: return new Nullable<Date>(d);
            // rust: 日期以原始字符串承载（登记 §1.7.1 ⑧）⇒ 仅镜像「可解析」判定
            let _ = d.ok_or_else(|| "InvalidFormatException: Date not well formated".to_string())?;
            Ok(Nullable::new_with_value(s.to_string()))
        // Java: else
        } else {
            // Java: return new Nullable<Date>();
            Ok(Nullable::new())
        }
    }

    // Java: String getDateValue(Nullable<Date> d)
    pub fn getDateValue(&self, d: &Nullable<String>) -> String {
        // Java: if (d == null)
        // Java: return "";
        // Java: else
        // Java: Date date = d.getValue();
        // Java: if (date == null)
        // Java: return "";
        // Java: else
        match d.getValue() {
            // rust: `d` 在 Rust 侧非空（`Nullable` 不会有 null 引用），空值与 null 合并处理（登记 §1.7.1 ⑧）
            None => String::new(),
            Some(date) => {
                // Java: SimpleDateFormat df = new SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ss'Z'");
                let mut df = DateFormatUtc::new("yyyy-MM-dd'T'HH:mm:ss'Z'");
                // Java: df.setTimeZone(TimeZone.getTimeZone("UTC"));
                df.setTimeZone("UTC");
                // Java: return df.format(date);
                df.format(date)
            }
        }
    }

    // Java: InputStream getInputStreamImpl(boolean isOriginInput)
    pub fn getInputStreamImpl(&self, _is_origin_input: bool) -> Box<dyn std::io::Read> {
        // Java: throw new InvalidOperationException("Operation not authorized");
        panic!("Operation not authorized")
    }

    // Java: OutputStream getOutputStreamImpl()
    pub fn getOutputStreamImpl(&self) -> Box<dyn std::io::Write> {
        // Java: throw new InvalidOperationException("Can't use output stream to set properties !");
        panic!("Can't use output stream to set properties !")
    }

    // Java: boolean save(OutputStream zos)
    pub fn save(&self, _zos: &mut dyn std::io::Write) -> bool {
        // Java: throw new InvalidOperationException("Operation not authorized");
        panic!("Operation not authorized")
    }

    // Java: boolean load(InputStream ios)
    pub fn load(&self, _ios: &mut dyn std::io::Read) -> bool {
        // Java: throw new InvalidOperationException("Operation not authorized");
        panic!("Operation not authorized")
    }

    // Java: void close()
    pub fn close(&self) {
    }

    // Java: void flush()
    pub fn flush(&self) {
    }
}

// rust: no-java —— `java.text.SimpleDateFormat` 是 JDK 类，不随包移植 ⇒ 就地实现本 function 需要的
//                  `yyyy-MM-dd'T'HH:mm:ss'Z'` 子集（仅 UTC、宽松解析、按前缀匹配）；登记 §1.7.1 ⑧
pub struct DateFormatUtc {
    // Java field: String pattern
    pattern: String,
}

impl DateFormatUtc {
    // rust: no-java
    pub fn new(pattern: &str) -> Self {
        DateFormatUtc {
            pattern: pattern.to_string(),
        }
    }

    // Java: void setTimeZone(TimeZone zone)
    pub fn setTimeZone(&mut self, zone: &str) {
        // rust: 仅实现 UTC（Java 侧三处调用都是 `TimeZone.getTimeZone("UTC")`）；登记 §1.7.1 ⑧
        debug_assert!(zone == "UTC");
    }

    // Java: Date parse(String source, ParsePosition pos)
    // rust: 宽松解析；返回 `None` 表示一个字段都没解析出来（Java 侧 ⇒ `parse` 返回 null）
    pub fn parse(&self, source: &str) -> Option<String> {
        let chars: Vec<char> = source.chars().collect();
        let mut i = 0usize;
        let mut matched_any = false;
        let mut in_quote = false;
        for p in self.pattern.chars() {
            if p == '\'' {
                in_quote = !in_quote;
                continue;
            }
            if in_quote {
                if i < chars.len() && chars[i] == p {
                    i += 1;
                } else {
                    break;
                }
                continue;
            }
            match p {
                'y' | 'M' | 'd' | 'H' | 'm' | 's' => {
                    let mut n = 0;
                    while i < chars.len() && chars[i].is_ascii_digit() {
                        i += 1;
                        n += 1;
                    }
                    if n == 0 {
                        break;
                    }
                    matched_any = true;
                }
                other => {
                    if i < chars.len() && chars[i] == other {
                        i += 1;
                    } else {
                        break;
                    }
                }
            }
        }
        if matched_any {
            Some(source.to_string())
        } else {
            None
        }
    }

    // Java: String format(Date date)
    // rust: 日期以原始字符串承载 ⇒ 原样返回（登记 §1.7.1 ⑧）
    pub fn format(&self, date: &str) -> String {
        date.to_string()
    }
}
