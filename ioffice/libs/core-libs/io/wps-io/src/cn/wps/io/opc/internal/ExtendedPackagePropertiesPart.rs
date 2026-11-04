#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.opc.ExtendedPackageProperties
// Java: import cn.wps.io.opc.OPCPackage
// Java: import cn.wps.io.opc.PackagePart
// Java: import cn.wps.io.opc.PackagePartName
// Java: import cn.wps.io.opc.ZipPackagePart
// Java: import cn.wps.io.opc.internal.marshallers.ZipPartMarshaller
// Java: import cn.wps.io.xwpf.XWPFPackage
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidFormatException
// Java: import cn.wps.io.xwpf.usermodel.exceptions.OpenXML4JException
// Java: import cn.wps.moffice.kfs.zip.ZipEntry
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.OutputStream

use crate::io::xwpf::util::ZipEntrySource::ZipEntryRef;
use crate::io::opc::PackagePart::PackagePart;
use crate::io::opc::PackagePartName::PackagePartName;
use crate::io::opc::OPCPackage::OPCPackage;
use crate::io::opc::internal::ContentType::ContentType;

// Java: public class ExtendedPackagePropertiesPart extends PackagePart implements ExtendedPackageProperties
pub struct ExtendedPackagePropertiesPart {
    pub(crate) base: PackagePart,
    // Java field: Integer mWords
    pub(crate) mWords: Option<i32>,
    // Java field: Integer mCharactersWithSpaces
    pub(crate) mCharactersWithSpaces: Option<i32>,
    // Java field: String mAppVersion
    pub(crate) mAppVersion: Option<String>,
    // Java field: String mApplication
    pub(crate) mApplication: Option<String>,
    // Java field: Integer mCharacters
    pub(crate) mCharacters: Option<i32>,
    // Java field: String mCompany
    pub(crate) mCompany: Option<String>,
    // Java field: Integer mDocSecurity
    pub(crate) mDocSecurity: Option<i32>,
    // Java field: Integer mHiddenSlides
    pub(crate) mHiddenSlides: Option<i32>,
    // Java field: String mHyperlinkBase
    pub(crate) mHyperlinkBase: Option<String>,
    // Java field: Boolean mHyperlinksChanged
    pub(crate) mHyperlinksChanged: Option<bool>,
    // Java field: Integer mLines
    pub(crate) mLines: Option<i32>,
    // Java field: Boolean mLinksUpToDate
    pub(crate) mLinksUpToDate: Option<bool>,
    // Java field: String mManager
    pub(crate) mManager: Option<String>,
    // Java field: Integer mMMClips
    pub(crate) mMMClips: Option<i32>,
    // Java field: Integer mNotes
    pub(crate) mNotes: Option<i32>,
    // Java field: Integer mPages
    pub(crate) mPages: Option<i32>,
    // Java field: Integer mParagraphs
    pub(crate) mParagraphs: Option<i32>,
    // Java field: String mPresentationFormat
    pub(crate) mPresentationFormat: Option<String>,
    // Java field: Boolean mScaleCrop
    pub(crate) mScaleCrop: Option<bool>,
    // Java field: Boolean mSharedDoc
    pub(crate) mSharedDoc: Option<bool>,
    // Java field: Integer mSlides
    pub(crate) mSlides: Option<i32>,
    // Java field: String mTemplate
    pub(crate) mTemplate: Option<String>,
    // Java field: Integer mTotalTime
    pub(crate) mTotalTime: Option<i32>,
    // Java field: ZipEntry mZipEntry
    //   ★2026-09-20 修正：由 `Option<Box<dyn std::any::Any + Send + Sync>>` 改为 Java 声明的
    //   **真实类型** `ZipEntry` 的 Rust 等价 `ZipEntryRef`（封闭枚举）
    pub(crate) mZipEntry: Option<ZipEntryRef>,
}

// Java: public class ExtendedPackagePropertiesPart extends PackagePart implements ExtendedPackageProperties
impl ExtendedPackagePropertiesPart {
    // Java: ExtendedPackagePropertiesPart(OPCPackage pack, PackagePartName partName)
    pub fn ExtendedPackagePropertiesPart(pack: Option<*const OPCPackage>, part_name: PackagePartName) -> Self {
        // Java: super(pack, partName, "application/vnd.openxmlformats-officedocument.extended-properties+xml");
        ExtendedPackagePropertiesPart {
            base: PackagePart::new(pack, part_name, ContentType::new("application/vnd.openxmlformats-officedocument.extended-properties+xml").unwrap()),
            mWords: None,
            mCharactersWithSpaces: None,
            mAppVersion: None,
            mApplication: None,
            mCharacters: None,
            mCompany: None,
            mDocSecurity: None,
            mHiddenSlides: None,
            mHyperlinkBase: None,
            mHyperlinksChanged: None,
            mLines: None,
            mLinksUpToDate: None,
            mManager: None,
            mMMClips: None,
            mNotes: None,
            mPages: None,
            mParagraphs: None,
            mPresentationFormat: None,
            mScaleCrop: None,
            mSharedDoc: None,
            mSlides: None,
            mTemplate: None,
            mTotalTime: None,
            mZipEntry: None,
        }
    }

    // rust: no-java（Rust 惯用别名，crate 内沿用 `new`）
    pub fn new(pack: Option<*const OPCPackage>, part_name: PackagePartName) -> Self {
        Self::ExtendedPackagePropertiesPart(pack, part_name)
    }

    // Java: void setWordCountProperty(int words)
    pub fn setWordCountProperty(&mut self, words: i32) {
        // Java: this.mWords = words;
        self.mWords = Some(words);
    }

    // Java: void setCharactersWithSpacesProperty(int charactersWithSpaces)
    pub fn setCharactersWithSpacesProperty(&mut self, characters_with_spaces: i32) {
        // Java: this.mCharactersWithSpaces = charactersWithSpaces;
        self.mCharactersWithSpaces = Some(characters_with_spaces);
    }

    // Java: InputStream getInputStreamImpl(boolean isOriginInput)
    pub fn getInputStreamImpl(&mut self, is_origin_input: bool) -> Box<dyn std::io::Read> {
        // Java: if (this.mZipEntry == null)
        if self.mZipEntry.is_none() {
            // Java: PackagePart packagePart = this._container.getPart(this._partName);
            // Java: if (packagePart != null && packagePart instanceof ZipPackagePart)
            // (unsafe: *const → *mut 对应 Java 共享引用上的可变操作，生命周期由包持有者保证)
            if let Some(c) = self.base._container {
                let pkg = unsafe { &mut *(c as *mut OPCPackage) };
                if let Some(packagePart) = pkg.getPart(&self.base._partName) {
                    // Java: ZipPackagePart zipPackagePart = (ZipPackagePart)packagePart;
                    let zipPackagePart = packagePart
                        .as_any()
                        .downcast_ref::<crate::io::opc::ZipPackagePart::ZipPackagePart>();
                    // Java: this.mZipEntry = zipPackagePart.getZipArchive();
                    // Rust: `mZipEntry` 是 owned 值，从借用无法 move ⇒ 保持壳语义（登记）
                    let _ = zipPackagePart;
                }
            }
        }
        // Java: return ((XWPFPackage)this._container).getZipArchive().getInputStream(this.mZipEntry, isOriginInput);
        // (getZipArchive 是 OPCPackage 方法，无需强转子类；unsafe 解引用共享引用)
        if let Some(c) = self.base._container {
            let opc = unsafe { &*c };
            if let Some(archive) = opc.getZipArchive().as_ref() {
                if let Some(src) = archive.as_any().downcast_ref::<crate::io::xwpf::util::ZipFileZipEntrySource::ZipFileZipEntrySource>() {
                    if let Some(ref ze) = self.mZipEntry {
                        // ★`ze` 现在就是 `&ZipEntryRef` ⇒ 原样透传（旧写法 `ze.as_ref()` 是抹掉类型的 `&dyn Any`）
                        if let Ok(Some(stream)) = src.getInputStream(ze, is_origin_input) {
                            return stream;
                        }
                    }
                }
            }
        }
        panic!("getInputStream");
    }

    // Java: OutputStream getOutputStreamImpl()
    pub fn getOutputStreamImpl(&self) -> Option<Box<dyn std::io::Write>> {
        // Java: return null;
        None
    }

    // Java: boolean save(OutputStream zos)
    pub fn save(&self, zos: &mut dyn std::io::Write) -> bool {
        // Java: return (new ZipPartMarshaller()).marshall(this, zos);
        crate::io::opc::internal::marshallers::ZipPartMarshaller::ZipPartMarshaller::new()
            .marshall(self, zos)
            .unwrap_or(false)
    }

    // Java: boolean load(InputStream ios)
    pub fn load(&self, _ios: &mut dyn std::io::Read) -> bool {
        // Java: return false;
        false
    }

    // Java: void close()
    pub fn close(&self) {
    }

    // Java: void flush()
    pub fn flush(&self) {
    }

    // Java: static String setStringValue(String s)
    fn setStringValue(s: Option<&str>) -> String {
        // Java: return s != null && !s.equals("") ? new String(s) : new String();
        match s {
            Some(v) if !v.is_empty() => v.to_string(),
            _ => String::new(),
        }
    }

    // Java: String getApplicationProperty()
    pub fn getApplicationProperty(&self) -> Option<&str> {
        // Java: return this.mApplication;
        self.mApplication.as_deref()
    }

    // Java: void setApplicationProperty(String application)
    pub fn setApplicationProperty(&mut self, application: &str) {
        // Java: this.mApplication = setStringValue(application);
        self.mApplication = Some(Self::setStringValue(Some(application)));
    }

    // Java: String getAppVersionProperty()
    pub fn getAppVersionProperty(&self) -> Option<&str> {
        // Java: return this.mAppVersion;
        self.mAppVersion.as_deref()
    }

    // Java: void setAppVersionProperty(String appVersion)
    pub fn setAppVersionProperty(&mut self, app_version: &str) {
        // Java: this.mAppVersion = setStringValue(appVersion);
        self.mAppVersion = Some(Self::setStringValue(Some(app_version)));
    }

    // Java: Integer getCharactersProperty()
    pub fn getCharactersProperty(&self) -> Option<i32> {
        // Java: return this.mCharacters;
        self.mCharacters
    }

    // Java: void setCharactersProperty(int character)
    pub fn setCharactersProperty(&mut self, character: i32) {
        // Java: this.mCharacters = character;
        self.mCharacters = Some(character);
    }

    // Java: String getCompanyProperty()
    pub fn getCompanyProperty(&self) -> Option<&str> {
        // Java: return this.mCompany;
        self.mCompany.as_deref()
    }

    // Java: void setCompanyProperty(String company)
    pub fn setCompanyProperty(&mut self, company: &str) {
        // Java: this.mCompany = setStringValue(company);
        self.mCompany = Some(Self::setStringValue(Some(company)));
    }

    // Java: String getDigSigProperty()
    pub fn getDigSigProperty(&self) -> Option<&str> {
        // Java: return null;
        None
    }

    // Java: void setDigSigProperty(String digSig)
    pub fn setDigSigProperty(&mut self, _dig_sig: &str) {
    }

    // Java: Integer getDocSecurityProperty()
    pub fn getDocSecurityProperty(&self) -> Option<i32> {
        // Java: return this.mDocSecurity;
        self.mDocSecurity
    }

    // Java: void setDocSecurityProperty(int docSecurity)
    pub fn setDocSecurityProperty(&mut self, doc_security: i32) {
        // Java: this.mDocSecurity = docSecurity;
        self.mDocSecurity = Some(doc_security);
    }

    // Java: String getHeadingPairsProperty()
    pub fn getHeadingPairsProperty(&self) -> Option<&str> {
        // Java: return null;
        None
    }

    // Java: void setHeadingPairsProperty(String headingPairs)
    pub fn setHeadingPairsProperty(&mut self, _heading_pairs: &str) {
    }

    // Java: Integer getHiddenSlidesProperty()
    pub fn getHiddenSlidesProperty(&self) -> Option<i32> {
        // Java: return this.mHiddenSlides;
        self.mHiddenSlides
    }

    // Java: void setHiddenSlidesProperty(int hiddenSlides)
    pub fn setHiddenSlidesProperty(&mut self, hidden_slides: i32) {
        // Java: this.mHiddenSlides = hiddenSlides;
        self.mHiddenSlides = Some(hidden_slides);
    }

    // Java: String getHLinksProperty()
    pub fn getHLinksProperty(&self) -> Option<&str> {
        // Java: return null;
        None
    }

    // Java: void setHLinksProperty(String hyperlink)
    pub fn setHLinksProperty(&mut self, _hyperlink: &str) {
    }

    // Java: String getHyperlinkBaseProperty()
    pub fn getHyperlinkBaseProperty(&self) -> Option<&str> {
        // Java: return this.mHyperlinkBase;
        self.mHyperlinkBase.as_deref()
    }

    // Java: void setHyperlinkBaseProperty(String hyperlinkBase)
    pub fn setHyperlinkBaseProperty(&mut self, hyperlink_base: &str) {
        // Java: this.mHyperlinkBase = setStringValue(hyperlinkBase);
        self.mHyperlinkBase = Some(Self::setStringValue(Some(hyperlink_base)));
    }

    // Java: Boolean getHyperlinksChangedProperty()
    pub fn getHyperlinksChangedProperty(&self) -> Option<bool> {
        // Java: return this.mHyperlinksChanged;
        self.mHyperlinksChanged
    }

    // Java: void setHyperlinksChangedProperty(boolean hyperlinksChange)
    pub fn setHyperlinksChangedProperty(&mut self, hyperlinks_change: bool) {
        // Java: this.mHyperlinksChanged = hyperlinksChange;
        self.mHyperlinksChanged = Some(hyperlinks_change);
    }

    // Java: Integer getLinesProperty()
    pub fn getLinesProperty(&self) -> Option<i32> {
        // Java: return this.mLines;
        self.mLines
    }

    // Java: void setLinesProperty(int lines)
    pub fn setLinesProperty(&mut self, lines: i32) {
        // Java: this.mLines = lines;
        self.mLines = Some(lines);
    }

    // Java: Boolean getLinksUpToDateProperty()
    pub fn getLinksUpToDateProperty(&self) -> Option<bool> {
        // Java: return this.mLinksUpToDate;
        self.mLinksUpToDate
    }

    // Java: void setLinksUpToDateProperty(boolean linksUpToDate)
    pub fn setLinksUpToDateProperty(&mut self, links_up_to_date: bool) {
        // Java: this.mLinksUpToDate = linksUpToDate;
        self.mLinksUpToDate = Some(links_up_to_date);
    }

    // Java: String getManagerProperty()
    pub fn getManagerProperty(&self) -> Option<&str> {
        // Java: return this.mManager;
        self.mManager.as_deref()
    }

    // Java: void setManagerProperty(String manager)
    pub fn setManagerProperty(&mut self, manager: &str) {
        // Java: this.mManager = manager;
        self.mManager = Some(manager.to_string());
    }

    // Java: Integer getMMClipsProperty()
    pub fn getMMClipsProperty(&self) -> Option<i32> {
        // Java: return this.mMMClips;
        self.mMMClips
    }

    // Java: void setMMClipsProperty(int mmClips)
    pub fn setMMClipsProperty(&mut self, mm_clips: i32) {
        // Java: this.mMMClips = mmClips;
        self.mMMClips = Some(mm_clips);
    }

    // Java: Integer getNotesProperty()
    pub fn getNotesProperty(&self) -> Option<i32> {
        // Java: return this.mNotes;
        self.mNotes
    }

    // Java: void setNotesProperty(int notes)
    pub fn setNotesProperty(&mut self, notes: i32) {
        // Java: this.mNotes = notes;
        self.mNotes = Some(notes);
    }

    // Java: Integer getPagesProperty()
    pub fn getPagesProperty(&self) -> Option<i32> {
        // Java: return this.mPages;
        self.mPages
    }

    // Java: void setPagesProperty(int pages)
    pub fn setPagesProperty(&mut self, pages: i32) {
        // Java: this.mPages = pages;
        self.mPages = Some(pages);
    }

    // Java: Integer getParagraphsProperty()
    pub fn getParagraphsProperty(&self) -> Option<i32> {
        // Java: return this.mParagraphs;
        self.mParagraphs
    }

    // Java: void setParagraphsProperty(int paragraphs)
    pub fn setParagraphsProperty(&mut self, paragraphs: i32) {
        // Java: this.mParagraphs = paragraphs;
        self.mParagraphs = Some(paragraphs);
    }

    // Java: String getPresentationFormatProperty()
    pub fn getPresentationFormatProperty(&self) -> Option<&str> {
        // Java: return this.mPresentationFormat;
        self.mPresentationFormat.as_deref()
    }

    // Java: void setPresentationFormatProperty(String presentationFormat)
    pub fn setPresentationFormatProperty(&mut self, presentation_format: &str) {
        // Java: this.mPresentationFormat = presentationFormat;
        self.mPresentationFormat = Some(presentation_format.to_string());
    }

    // Java: Boolean getScaleCropProperty()
    pub fn getScaleCropProperty(&self) -> Option<bool> {
        // Java: return this.mScaleCrop;
        self.mScaleCrop
    }

    // Java: void setScaleCropProperty(boolean scaleCrop)
    pub fn setScaleCropProperty(&mut self, scale_crop: bool) {
        // Java: this.mScaleCrop = scaleCrop;
        self.mScaleCrop = Some(scale_crop);
    }

    // Java: Boolean getSharedDocProperty()
    pub fn getSharedDocProperty(&self) -> Option<bool> {
        // Java: return this.mSharedDoc;
        self.mSharedDoc
    }

    // Java: void setSharedDocProperty(boolean sharedDoc)
    pub fn setSharedDocProperty(&mut self, shared_doc: bool) {
        // Java: this.mSharedDoc = sharedDoc;
        self.mSharedDoc = Some(shared_doc);
    }

    // Java: Integer getSlidesProperty()
    pub fn getSlidesProperty(&self) -> Option<i32> {
        // Java: return this.mSlides;
        self.mSlides
    }

    // Java: void setSlidesProperty(int slides)
    pub fn setSlidesProperty(&mut self, slides: i32) {
        // Java: this.mSlides = slides;
        self.mSlides = Some(slides);
    }

    // Java: String getTemplateProperty()
    pub fn getTemplateProperty(&self) -> Option<&str> {
        // Java: return this.mTemplate;
        self.mTemplate.as_deref()
    }

    // Java: void setTemplateProperty(String template)
    pub fn setTemplateProperty(&mut self, template: &str) {
        // Java: this.mTemplate = template;
        self.mTemplate = Some(template.to_string());
    }

    // Java: String getTitlesOfPartsProperty()
    pub fn getTitlesOfPartsProperty(&self) -> Option<&str> {
        // Java: return null;
        None
    }

    // Java: void setTitlesOfPartsProperty(String titlesOfParts)
    pub fn setTitlesOfPartsProperty(&mut self, _titles_of_parts: &str) {
    }

    // Java: Integer getTotalTimeProperty()
    pub fn getTotalTimeProperty(&self) -> Option<i32> {
        // Java: return this.mTotalTime;
        self.mTotalTime
    }

    // Java: void setTotalTimeProperty(int totalTime)
    pub fn setTotalTimeProperty(&mut self, total_time: i32) {
        // Java: this.mTotalTime = totalTime;
        self.mTotalTime = Some(total_time);
    }

    // Java: Integer getWordCountProperty()
    pub fn getWordCountProperty(&self) -> Option<i32> {
        // Java: return this.mWords;
        self.mWords
    }

    // Java: Integer getNumberCharactersProperty()
    pub fn getNumberCharactersProperty(&self) -> Option<i32> {
        // Java: return this.mCharactersWithSpaces;
        self.mCharactersWithSpaces
    }
}
