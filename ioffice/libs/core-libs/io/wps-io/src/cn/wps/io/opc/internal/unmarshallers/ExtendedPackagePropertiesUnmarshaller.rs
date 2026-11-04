#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.io.TypeConversion
// Java: import cn.wps.io.dom.Document
// Java: import cn.wps.io.dom.DocumentException
// Java: import cn.wps.io.dom.Element
// Java: import cn.wps.io.dom.io.SAXReader
// Java: import cn.wps.io.opc.OPCPackage
// Java: import cn.wps.io.opc.PackagePart
// Java: import cn.wps.io.opc.PackagePartName
// Java: import cn.wps.io.opc.PackagingURIHelper
// Java: import cn.wps.io.opc.ZipPackagePart
// Java: import cn.wps.io.opc.internal.ExtendedPackagePropertiesPart
// Java: import cn.wps.io.opc.internal.PartUnmarshaller
// Java: import cn.wps.io.opc.internal.ZipHelper
// Java: import cn.wps.io.xwpf.XWPFPackage
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidFormatException
// Java: import cn.wps.moffice.kfs.zip.ZipEntry
// Java: import java.io.IOException
// Java: import java.io.InputStream

use crate::io::dom::tree::DefaultElement::DefaultElement;
use crate::io::opc::internal::ExtendedPackagePropertiesPart::ExtendedPackagePropertiesPart;
use crate::io::xwpf::XWPFPackage::XWPFPackage;
use crate::io::xwpf::util::ZipFileZipEntrySource::ZipFileZipEntrySource;
use super::UnmarshallContext::UnmarshallContext;

// Java field: String KEYWORD_WORDS
pub(crate) const KEYWORD_WORDS: &str = "Words";
// Java field: String KEYWORD_CHARACTERS_WITH_SPACES
pub(crate) const KEYWORD_CHARACTERS_WITH_SPACES: &str = "CharactersWithSpaces";
// Java field: String KEYWORD_APP_VERSION
pub(crate) const KEYWORD_APP_VERSION: &str = "AppVersion";
// Java field: String KEYWORD_CHARACTERS
pub(crate) const KEYWORD_CHARACTERS: &str = "Characters";
// Java field: String KEYWORD_APPLICATION
pub(crate) const KEYWORD_APPLICATION: &str = "Application";
// Java field: String KEYWORD_COMPANY
pub(crate) const KEYWORD_COMPANY: &str = "Company";
// Java field: String KEYWORD_DIG_SIG
pub(crate) const KEYWORD_DIG_SIG: &str = "DigSig";
// Java field: String KEYWORD_DOC_SECURITY
pub(crate) const KEYWORD_DOC_SECURITY: &str = "DocSecurity";
// Java field: String KEYWORD_HEADING_PAIRS
pub(crate) const KEYWORD_HEADING_PAIRS: &str = "HeadingPairs";
// Java field: String KEYWORD_HIDDEN_SLIDES
pub(crate) const KEYWORD_HIDDEN_SLIDES: &str = "HiddenSlides";
// Java field: String KEYWORD_HLINKS
pub(crate) const KEYWORD_HLINKS: &str = "HLinks";
// Java field: String KEYWORD_HYPER_LINK_BASE
pub(crate) const KEYWORD_HYPER_LINK_BASE: &str = "HyperlinkBase";
// Java field: String KEYWORD_HYPER_LINK_CHANGED
pub(crate) const KEYWORD_HYPER_LINK_CHANGED: &str = "HyperlinksChanged";
// Java field: String KEYWORD_LINES
pub(crate) const KEYWORD_LINES: &str = "Lines";
// Java field: String KEYWORD_LINKS_UP_TO_DATE
pub(crate) const KEYWORD_LINKS_UP_TO_DATE: &str = "LinksUpToDate";
// Java field: String KEYWORD_MANAGER
pub(crate) const KEYWORD_MANAGER: &str = "Manager";
// Java field: String KEYWORD_MM_CLIPS
pub(crate) const KEYWORD_MM_CLIPS: &str = "MMClips";
// Java field: String KEYWORD_NOTES
pub(crate) const KEYWORD_NOTES: &str = "Notes";
// Java field: String KEYWORD_PAGES
pub(crate) const KEYWORD_PAGES: &str = "Pages";
// Java field: String KEYWORD_PARAGRAPHS
pub(crate) const KEYWORD_PARAGRAPHS: &str = "Paragraphs";
// Java field: String KEYWORD_PRESENTATION_FORMAT
pub(crate) const KEYWORD_PRESENTATION_FORMAT: &str = "PresentationFormat";
// Java field: String KEYWORD_SCALE_CROP
pub(crate) const KEYWORD_SCALE_CROP: &str = "ScaleCrop";
// Java field: String KEYWORD_SHARE_DOC
pub(crate) const KEYWORD_SHARE_DOC: &str = "SharedDoc";
// Java field: String KEYWORD_SLIDES
pub(crate) const KEYWORD_SLIDES: &str = "Slides";
// Java field: String KEYWORD_TEMPLEATE
pub(crate) const KEYWORD_TEMPLEATE: &str = "Template";
// Java field: String KEYWORD_TITLES_OF_PARTS
pub(crate) const KEYWORD_TITLES_OF_PARTS: &str = "TitlesOfParts";
// Java field: String KEYWORD_TATAL_TIME
pub(crate) const KEYWORD_TATAL_TIME: &str = "TotalTime";

// Java: cn.wps.base.io.TypeConversion.string2Integer(String str)
fn string2_integer(value: &str) -> Option<i32> {
    // Java: return TypeConversion.string2Integer(str);
    // Rust: 同一实现已落在 util crate（`cn/wps/base/io/TypeConversion.rs:47`）⇒ 委派，避免二次分叉
    core_libs_util::cn::wps::base::io::TypeConversion::TypeConversion::string2Integer(value)
}

fn string2_boolean(value: &str) -> bool {
    value.eq_ignore_ascii_case("true")
}

// Java: public class ExtendedPackagePropertiesUnmarshaller implements PartUnmarshaller
pub struct ExtendedPackagePropertiesUnmarshaller;

// Java: public class ExtendedPackagePropertiesUnmarshaller implements PartUnmarshaller
impl ExtendedPackagePropertiesUnmarshaller {
    // Java: ExtendedPackagePropertiesUnmarshaller()
    pub fn ExtendedPackagePropertiesUnmarshaller() -> Self {
        // Java: super();
        ExtendedPackagePropertiesUnmarshaller
    }

    // rust: no-java（Rust 惯用别名，crate 内沿用 `new`）
    pub fn new() -> Self {
        Self::ExtendedPackagePropertiesUnmarshaller()
    }

    // Java: PackagePart unmarshall(UnmarshallContext context, InputStream in)
    pub fn unmarshall(
        &self,
        context: &UnmarshallContext,
        input: Option<&mut dyn std::io::Read>,
    ) -> Result<Option<ExtendedPackagePropertiesPart>, String> {
        // Java: ExtendedPackagePropertiesPart extendedProps = new ExtendedPackagePropertiesPart(context.getPackage(), context.getPartName());
        // rust: 同名构造器 `ExtendedPackagePropertiesPart(OPCPackage pack, PackagePartName partName)`；本 crate 的包参数
        //       由 `UnmarshallContext` 承载，构造器 `pack` 传 `None`（登记 §1.7.1 ④）
        let partName = context
            .getPartName()
            .as_ref()
            .ok_or_else(|| "partName".to_string())?
            .clone();
                // rust: ★2026-09-21 修正（M-STAGE1 P1-c1）：照 Java 传 `context.getPackage()`；
        //       `PackagePart::loadRelationships()` 已改为真解析 `_rels`，container 为 null 会 panic。
let mut extendedProps = ExtendedPackagePropertiesPart::new(*context.getPackage(), partName);
        let mut ownedStream: Option<Box<dyn std::io::Read>> = None;
        // Java: if (in == null) {
        // rust: Java 的 `in` ↔ 本 crate 的 `input`（「调用方未给流」同为 `None` 语义，登记 §1.7.1 ④）
        if input.is_none() {
            // Java: ZipEntry zipEntry = context.getZipEntry();
            let zipEntry = context.getZipEntry();
            // Java: String zipEntryName = zipEntry.getName();
            // Java: if (zipEntryName != null && zipEntryName.contains("../")) {
            // Java: return null;
            // ★2026-09-20 落地（原为 §1.7.1 ⑧ 登记的未实现项）：`UnmarshallContext::getZipEntry()`
            //   现在返回**真实类型** `&Option<ZipEntryRef>`（不再是 `&dyn Any` 壳）⇒ `zipEntry.getName()`
            //   可用，Java 的 `../` 越界（zip-slip）校验得以在**此处**按原文实现，
            //   不再依赖 `ZipFileZipEntrySource::getInputStream` 内部兜底。
            let zipEntryName: Option<String> = zipEntry.as_ref().map(|ze| ze.getName().to_string());
            if zipEntryName.as_deref().map_or(false, |n| n.contains("../")) {
                return Ok(None);
            }
            // Java: OPCPackage opcPackage = context.getPackage();
            let opcPackage = context.getPackage();
            // Java: if (zipEntry != null) {
            if zipEntry.is_some() {
                // ★2026-09-20 **分支归位**（对齐 `ExtendedPackagePropertiesUnmarshaller.java:65-66`）：
                //   Java 的 `if (zipEntry != null)` 分支体**只有一句**
                //   `in = ((XWPFPackage)opcPackage).getZipArchive().getInputStream(zipEntry);`；
                //   而 `getPart` / `zipPackagePart` 那几行属于 **`else` 分支**。旧 Rust 把它们
                //   错放进了 `if` 分支 ⇒ 本次按 Java 原文归位（这也是 §3.8 报
                //   "Java field 'zipPackagePart' has no adjacent Rust identifier" 的根因）。
                if let Some(pkg_ptr) = opcPackage {
                    // ★`UnmarshallContext._package` 已是真实类型 `*const OPCPackage`（不再是 `dyn Any`）
                    //   ⇒ Java 的 `(XWPFPackage)opcPackage` 向上强转不再需要：`getZipArchive()`
                    //   声明在**基类** `OPCPackage`（`OPCPackage.java:77`）上，直接在真实基类引用上调用。
                    let opc_pkg: &crate::io::opc::OPCPackage::OPCPackage = unsafe { &**pkg_ptr };
                    // Java: in = ((XWPFPackage)opcPackage).getZipArchive().getInputStream(context.getZipEntry());
                    // ★去掉 `archive.as_any().downcast_ref::<ZipFileZipEntrySource>()`：Java 是在
                    //   `ZipEntrySource` **接口**上直接调 `getInputStream` ⇒ Rust 直接在 trait 对象上调同名方法。
                    if let Some(ze) = zipEntry.as_ref() {
                        if let Some(archive) = opc_pkg.getZipArchive().as_ref() {
                            // ★`ze` 就是 `&ZipEntryRef` ⇒ 原样透传
                            if let Ok(stream) = archive.getInputStream_with_flag(ze, false) {
                                ownedStream = Some(stream);
                            }
                        }
                    }
                }
            // Java: else
            } else {
                // Java: if (opcPackage == null) {
                if opcPackage.is_none() {
                    // Java: throw new IOException("Error while trying to get the part input stream.");
                    return Err("Error while trying to get the part input stream.".to_string());
                }
                // ★2026-09-20：以下 `else` 分支体本次**按 Java 原文实现**（旧实现是注释 + no-stmt）。
                let opc_pkg: &mut crate::io::opc::OPCPackage::OPCPackage =
                    unsafe { &mut *(*opcPackage.as_ref().unwrap() as *mut crate::io::opc::OPCPackage::OPCPackage) };
                // Java: PackagePartName partName = PackagingURIHelper.createPartName(ZipHelper.getExtendedPropertiesURI((XWPFPackage)context.getPackage()));
                let partName = crate::io::opc::internal::ZipHelper::ZipHelper::getExtendedPropertiesURI(opc_pkg)
                    .and_then(|uri| crate::io::opc::PackagingURIHelper::PackagingURIHelper::createPartName_uri(&uri).ok());
                // Java: PackagePart part = opcPackage.getPart(partName);
                let part = partName.as_ref().and_then(|pn| opc_pkg.getPart(pn));
                // Java: if (part != null && part instanceof ZipPackagePart) {
                //   ★Rust 组合式建模：Java 的 `instanceof ZipPackagePart` ⇔ 该 part 带 zip entry
                if part.map_or(false, |p| p.zipEntry.is_some()) {
                    // Java: ZipPackagePart zipPackagePart = (ZipPackagePart)part;
                    let zipPackagePart = part.expect("part != null");
                    // Java: zipEntry = zipPackagePart.getZipArchive();
                    let zipEntry = zipPackagePart.zipEntry.clone();
                    // Java: in = ((XWPFPackage)opcPackage).getZipArchive().getInputStream(zipEntry);
                    if let (Some(ze), Some(archive)) =
                        (zipEntry.as_ref(), opc_pkg.getZipArchive().as_ref())
                    {
                        if let Ok(stream) = archive.getInputStream_with_flag(ze, false) {
                            ownedStream = Some(stream);
                        }
                    }
                }
            }
        }
        // Java: SAXReader xmlReader = new SAXReader();
        let mut xmlReader = crate::io::dom::io::SAXReader::SAXReader::new();
        // Java: try {
        // Java: root = xmlDoc.getRootElement();
        // Java: } catch (DocumentException e) {
        // Java: throw new IOException(e.getMessage());
        // rust: Java 的 `in` 在本 crate 由 `stream` 承载（`input` 或包内 zip 流，登记 §1.7.1 ④）
        let stream: &mut dyn std::io::Read = if let Some(ref mut s) = ownedStream {
            s.as_mut()
        } else if let Some(s) = input {
            s
        } else {
            return Err("Error while trying to get the part input stream.".to_string());
        };
        // Java: Document xmlDoc = xmlReader.read(in);
        let xmlDoc = match xmlReader.read_stream(stream) {
            Ok(doc) => doc,
            Err(e) => return Err(e.to_string()),
        };
        // Java: Element root = null;
        let root_any: &dyn std::any::Any = xmlDoc.as_ref();
        let root = if let Some(dd) =
            root_any.downcast_ref::<crate::io::dom::tree::DefaultDocument::DefaultDocument>()
        {
            dd.getRootElement()
                .map(|r| r.as_ref() as &dyn std::any::Any)
        } else {
            None
        };
        let root_any = root.unwrap_or(root_any);
        // Java: Integer words = this.loadWordCount(root);
        // Java: if (null != words) {
        // Java: extendedProps.setWordCountProperty(words);
        // rust: Java 的 `Integer/Boolean/String` 可空包装 ↔ 本 crate 的 `Option` ⇒ 前两条合并为 `if let Some(..)`（登记 §1.7.1 ⑥）
        if let Some(words) = self.loadWordCount(root_any) {
            extendedProps.setWordCountProperty(words);
        }
        // Java: Integer character = this.loadNumberCharacters(root);
        // Java: if (null != character) {
        // Java: extendedProps.setCharactersWithSpacesProperty(character);
        // rust: Java 的 `Integer/Boolean/String` 可空包装 ↔ 本 crate 的 `Option` ⇒ 前两条合并为 `if let Some(..)`（登记 §1.7.1 ⑥）
        if let Some(character) = self.loadNumberCharacters(root_any) {
            extendedProps.setCharactersWithSpacesProperty(character);
        }
        // Java: String appVersion = this.loadAppVersion(root);
        // Java: if (null != appVersion) {
        // Java: extendedProps.setAppVersionProperty(appVersion);
        // rust: Java 的 `Integer/Boolean/String` 可空包装 ↔ 本 crate 的 `Option` ⇒ 前两条合并为 `if let Some(..)`（登记 §1.7.1 ⑥）
        if let Some(ref app_version) = self.loadAppVersion(root_any) {
            extendedProps.setAppVersionProperty(app_version);
        }
        // Java: String application = this.loadApplication(root);
        // Java: if (null != application) {
        // Java: extendedProps.setApplicationProperty(application);
        // rust: Java 的 `Integer/Boolean/String` 可空包装 ↔ 本 crate 的 `Option` ⇒ 前两条合并为 `if let Some(..)`（登记 §1.7.1 ⑥）
        if let Some(ref application) = self.loadApplication(root_any) {
            extendedProps.setApplicationProperty(application);
        }
        // Java: String company = this.loadCompany(root);
        // Java: if (null != company) {
        // Java: extendedProps.setCompanyProperty(company);
        // rust: Java 的 `Integer/Boolean/String` 可空包装 ↔ 本 crate 的 `Option` ⇒ 前两条合并为 `if let Some(..)`（登记 §1.7.1 ⑥）
        if let Some(ref company) = self.loadCompany(root_any) {
            extendedProps.setCompanyProperty(company);
        }
        // Java: Integer docSec = this.loadDocSecurity(root);
        // Java: if (null != docSec) {
        // Java: extendedProps.setDocSecurityProperty(docSec);
        // rust: Java 的 `Integer/Boolean/String` 可空包装 ↔ 本 crate 的 `Option` ⇒ 前两条合并为 `if let Some(..)`（登记 §1.7.1 ⑥）
        if let Some(doc_sec) = self.loadDocSecurity(root_any) {
            extendedProps.setDocSecurityProperty(doc_sec);
        }
        // Java: Integer hiddenSlides = this.loadHiddenSlides(root);
        // Java: if (null != hiddenSlides) {
        // Java: extendedProps.setHiddenSlidesProperty(hiddenSlides);
        // rust: Java 的 `Integer/Boolean/String` 可空包装 ↔ 本 crate 的 `Option` ⇒ 前两条合并为 `if let Some(..)`（登记 §1.7.1 ⑥）
        if let Some(hidden_slides) = self.loadHiddenSlides(root_any) {
            extendedProps.setHiddenSlidesProperty(hidden_slides);
        }
        // Java: String hyperlinkBase = this.loadHyperlinkBase(root);
        // Java: if (null != hyperlinkBase) {
        // Java: extendedProps.setHyperlinkBaseProperty(hyperlinkBase);
        // rust: Java 的 `Integer/Boolean/String` 可空包装 ↔ 本 crate 的 `Option` ⇒ 前两条合并为 `if let Some(..)`（登记 §1.7.1 ⑥）
        if let Some(ref hyperlink_base) = self.loadHyperlinkBase(root_any) {
            extendedProps.setHyperlinkBaseProperty(hyperlink_base);
        }
        // Java: Boolean hyperlinksChanged = this.loadHyperlinksChanged(root);
        // Java: if (null != hyperlinksChanged) {
        // Java: extendedProps.setHyperlinksChangedProperty(hyperlinksChanged);
        // rust: Java 的 `Integer/Boolean/String` 可空包装 ↔ 本 crate 的 `Option` ⇒ 前两条合并为 `if let Some(..)`（登记 §1.7.1 ⑥）
        if let Some(hyperlinks_changed) = self.loadHyperlinksChanged(root_any) {
            extendedProps.setHyperlinksChangedProperty(hyperlinks_changed);
        }
        // Java: Boolean linksUpToDate = this.loadLinksUpToDate(root);
        // Java: if (null != linksUpToDate) {
        // Java: extendedProps.setLinksUpToDateProperty(linksUpToDate);
        // rust: Java 的 `Integer/Boolean/String` 可空包装 ↔ 本 crate 的 `Option` ⇒ 前两条合并为 `if let Some(..)`（登记 §1.7.1 ⑥）
        if let Some(links_up_to_date) = self.loadLinksUpToDate(root_any) {
            extendedProps.setLinksUpToDateProperty(links_up_to_date);
        }
        // Java: String manager = this.loadManager(root);
        // Java: if (null != manager) {
        // Java: extendedProps.setManagerProperty(manager);
        // rust: Java 的 `Integer/Boolean/String` 可空包装 ↔ 本 crate 的 `Option` ⇒ 前两条合并为 `if let Some(..)`（登记 §1.7.1 ⑥）
        if let Some(ref manager) = self.loadManager(root_any) {
            extendedProps.setManagerProperty(manager);
        }
        // Java: Integer MMClips = this.loadMMClips(root);
        // Java: if (null != MMClips) {
        // Java: extendedProps.setMMClipsProperty(MMClips);
        // rust: Java 的 `Integer/Boolean/String` 可空包装 ↔ 本 crate 的 `Option` ⇒ 前两条合并为 `if let Some(..)`（登记 §1.7.1 ⑥）
        if let Some(mm_clips) = self.loadMMClips(root_any) {
            extendedProps.setMMClipsProperty(mm_clips);
        }
        // Java: String presentationFormat = this.loadPresentationFormat(root);
        // Java: if (null != presentationFormat) {
        // Java: extendedProps.setPresentationFormatProperty(presentationFormat);
        // rust: Java 的 `Integer/Boolean/String` 可空包装 ↔ 本 crate 的 `Option` ⇒ 前两条合并为 `if let Some(..)`（登记 §1.7.1 ⑥）
        if let Some(ref presentation_format) = self.loadPresentationFormat(root_any) {
            extendedProps.setPresentationFormatProperty(presentation_format);
        }
        // Java: Boolean scaleCrop = this.loadScaleCrop(root);
        // Java: if (null != scaleCrop) {
        // Java: extendedProps.setScaleCropProperty(scaleCrop);
        // rust: Java 的 `Integer/Boolean/String` 可空包装 ↔ 本 crate 的 `Option` ⇒ 前两条合并为 `if let Some(..)`（登记 §1.7.1 ⑥）
        if let Some(scale_crop) = self.loadScaleCrop(root_any) {
            extendedProps.setScaleCropProperty(scale_crop);
        }
        // Java: Boolean shareDoc = this.loadSharedDoc(root);
        // Java: if (null != shareDoc) {
        // Java: extendedProps.setSharedDocProperty(shareDoc);
        // rust: Java 的 `Integer/Boolean/String` 可空包装 ↔ 本 crate 的 `Option` ⇒ 前两条合并为 `if let Some(..)`（登记 §1.7.1 ⑥）
        if let Some(share_doc) = self.loadSharedDoc(root_any) {
            extendedProps.setSharedDocProperty(share_doc);
        }
        // Java: Integer slides = this.loadSlides(root);
        // Java: if (null != slides) {
        // Java: extendedProps.setSlidesProperty(slides);
        // rust: Java 的 `Integer/Boolean/String` 可空包装 ↔ 本 crate 的 `Option` ⇒ 前两条合并为 `if let Some(..)`（登记 §1.7.1 ⑥）
        if let Some(slides) = self.loadSlides(root_any) {
            extendedProps.setSlidesProperty(slides);
        }
        // Java: String template = this.loadTemplate(root);
        // Java: if (null != template) {
        // Java: extendedProps.setTemplateProperty(template);
        // rust: Java 的 `Integer/Boolean/String` 可空包装 ↔ 本 crate 的 `Option` ⇒ 前两条合并为 `if let Some(..)`（登记 §1.7.1 ⑥）
        if let Some(ref template) = self.loadTemplate(root_any) {
            extendedProps.setTemplateProperty(template);
        }
        // Java: Integer totalTime = this.loadTotalTime(root);
        // Java: if (null != totalTime) {
        // Java: extendedProps.setTotalTimeProperty(totalTime);
        // rust: Java 的 `Integer/Boolean/String` 可空包装 ↔ 本 crate 的 `Option` ⇒ 前两条合并为 `if let Some(..)`（登记 §1.7.1 ⑥）
        if let Some(total_time) = self.loadTotalTime(root_any) {
            extendedProps.setTotalTimeProperty(total_time);
        }
        // Java: Integer pages = this.loadPages(root);
        // Java: if (null != pages) {
        // Java: extendedProps.setPagesProperty(pages);
        // rust: Java 的 `Integer/Boolean/String` 可空包装 ↔ 本 crate 的 `Option` ⇒ 前两条合并为 `if let Some(..)`（登记 §1.7.1 ⑥）
        if let Some(pages) = self.loadPages(root_any) {
            extendedProps.setPagesProperty(pages);
        }
        // Java: root.detachRecursively();
        // rust: no-stmt —— `Element.detachRecursively()` 在本 crate 未接线（DOM 层停车，登记 §1.7.1 ⑩）
        // Java: return extendedProps;
        Ok(Some(extendedProps))
    }

    fn load_string_elem(&self, root: &dyn std::any::Any, name: &str) -> Option<String> {
        let el = root.downcast_ref::<DefaultElement>()?;
        let child = el.element(name)?;
        let child_el = child.downcast_ref::<DefaultElement>()?;
        Some(child_el.getStringValue())
    }

    fn load_string_elem_nonempty(&self, root: &dyn std::any::Any, name: &str) -> Option<String> {
        let el = root.downcast_ref::<DefaultElement>()?;
        let child = el.element(name)?;
        let child_el = child.downcast_ref::<DefaultElement>()?;
        let value = child_el.getStringValue();
        if value.len() > 0 {
            Some(value)
        } else {
            None
        }
    }

    fn load_int_elem(&self, root: &dyn std::any::Any, name: &str) -> Option<i32> {
        let el = root.downcast_ref::<DefaultElement>()?;
        let child = el.element(name)?;
        let child_el = child.downcast_ref::<DefaultElement>()?;
        let value = child_el.getStringValue();
        if value.len() > 0 {
            string2_integer(&value)
        } else {
            None
        }
    }

    // Java: String loadAppVersion(Element root)
    fn loadAppVersion(&self, root: &dyn std::any::Any) -> Option<String> {
        // Java: Element el = root.element("AppVersion");
        // Java: return null == el ? null : el.getStringValue();
        // rust: 上述 koffice 语句由本 crate 的 `load_*` 辅助/下方实现一步完成（登记 §1.7.1 ⑤）
        self.load_string_elem(root, "AppVersion")
    }
    // Java: String loadApplication(Element root)
    fn loadApplication(&self, root: &dyn std::any::Any) -> Option<String> {
        // Java: Element el = root.element("Application");
        // Java: return null == el ? null : el.getStringValue();
        // rust: 上述 koffice 语句由本 crate 的 `load_*` 辅助/下方实现一步完成（登记 §1.7.1 ⑤）
        self.load_string_elem(root, "Application")
    }
    // Java: String loadCompany(Element root)
    fn loadCompany(&self, root: &dyn std::any::Any) -> Option<String> {
        // Java: Element el = root.element("Company");
        // Java: return null == el ? null : el.getStringValue();
        // rust: 上述 koffice 语句由本 crate 的 `load_*` 辅助/下方实现一步完成（登记 §1.7.1 ⑤）
        self.load_string_elem(root, "Company")
    }
    // Java: Integer loadDocSecurity(Element root)
    fn loadDocSecurity(&self, root: &dyn std::any::Any) -> Option<i32> {
        // Java: Element el = root.element("DocSecurity");
        // Java: if (null == el) {
        // Java: return null;
        // Java: String value = el.getStringValue();
        // Java: return null != value && value.length() > 0 ? TypeConversion.string2Integer(value) : null;
        // rust: 上述 koffice 语句由本 crate 的 `load_*` 辅助/下方实现一步完成（登记 §1.7.1 ⑤）
        self.load_int_elem(root, "DocSecurity")
    }
    // Java: Integer loadHiddenSlides(Element root)
    fn loadHiddenSlides(&self, root: &dyn std::any::Any) -> Option<i32> {
        // Java: Element el = root.element("HiddenSlides");
        // Java: if (null == el) {
        // Java: return null;
        // Java: String value = el.getStringValue();
        // Java: return null != value && value.length() > 0 ? TypeConversion.string2Integer(value) : null;
        // rust: 上述 koffice 语句由本 crate 的 `load_*` 辅助/下方实现一步完成（登记 §1.7.1 ⑤）
        self.load_int_elem(root, "HiddenSlides")
    }
    // Java: String loadHyperlinkBase(Element root)
    fn loadHyperlinkBase(&self, root: &dyn std::any::Any) -> Option<String> {
        // Java: Element el = root.element("HyperlinkBase");
        // Java: if (null == el) {
        // Java: return null;
        // Java: String value = el.getStringValue();
        // Java: return null != value && value.length() > 0 ? value : null;
        // rust: 上述 koffice 语句由本 crate 的 `load_*` 辅助/下方实现一步完成（登记 §1.7.1 ⑤）
        self.load_string_elem_nonempty(root, "HyperlinkBase")
    }
    // Java: Boolean loadHyperlinksChanged(Element root)
    fn loadHyperlinksChanged(&self, root: &dyn std::any::Any) -> Option<bool> {
        // Java: Element el = root.element("HyperlinksChanged");
        // Java: return null == el ? null : new Boolean(el.getStringValue());
        // rust: 上述 koffice 语句由本 crate 的 `load_*` 辅助/下方实现一步完成（登记 §1.7.1 ⑤）
        let el = root.downcast_ref::<DefaultElement>()?;
        let child = el.element("HyperlinksChanged")?;
        let child_el = child.downcast_ref::<DefaultElement>()?;
        Some(string2_boolean(&child_el.getStringValue()))
    }
    // Java: Integer loadLines(Element root)
    fn loadLines(&self, root: &dyn std::any::Any) -> Option<i32> {
        // Java: Element el = root.element("Lines");
        // Java: if (null == el) {
        // Java: return null;
        // Java: String value = el.getStringValue();
        // Java: return null != value && value.length() > 0 ? TypeConversion.string2Integer(value) : null;
        // rust: 上述 koffice 语句由本 crate 的 `load_*` 辅助/下方实现一步完成（登记 §1.7.1 ⑤）
        self.load_int_elem(root, "Lines")
    }
    // Java: Boolean loadLinksUpToDate(Element root)
    fn loadLinksUpToDate(&self, root: &dyn std::any::Any) -> Option<bool> {
        // Java: Element el = root.element("LinksUpToDate");
        // Java: if (null == el) {
        // Java: return null;
        // Java: String value = el.getStringValue();
        // Java: return null != value && value.length() > 0 ? new Boolean(el.getStringValue()) : null;
        // rust: 上述 koffice 语句由本 crate 的 `load_*` 辅助/下方实现一步完成（登记 §1.7.1 ⑤）
        let el = root.downcast_ref::<DefaultElement>()?;
        let child = el.element("LinksUpToDate")?;
        let child_el = child.downcast_ref::<DefaultElement>()?;
        let value = child_el.getStringValue();
        if value.len() > 0 {
            Some(string2_boolean(&value))
        } else {
            None
        }
    }
    // Java: String loadManager(Element root)
    fn loadManager(&self, root: &dyn std::any::Any) -> Option<String> {
        // Java: Element el = root.element("Manager");
        // Java: if (null == el) {
        // Java: return null;
        // Java: String value = el.getStringValue();
        // Java: return null != value && value.length() > 0 ? el.getStringValue() : null;
        // rust: 上述 koffice 语句由本 crate 的 `load_*` 辅助/下方实现一步完成（登记 §1.7.1 ⑤）
        self.load_string_elem_nonempty(root, "Manager")
    }
    // Java: Integer loadMMClips(Element root)
    fn loadMMClips(&self, root: &dyn std::any::Any) -> Option<i32> {
        // Java: Element el = root.element("MMClips");
        // Java: if (null == el) {
        // Java: return null;
        // Java: String value = el.getStringValue();
        // Java: return null != value && value.length() > 0 ? TypeConversion.string2Integer(value) : null;
        // rust: 上述 koffice 语句由本 crate 的 `load_*` 辅助/下方实现一步完成（登记 §1.7.1 ⑤）
        self.load_int_elem(root, "MMClips")
    }
    // Java: Integer loadWordCount(Element root)
    fn loadWordCount(&self, root: &dyn std::any::Any) -> Option<i32> {
        // Java: Element el = root.element("Words");
        // Java: if (null == el) {
        // Java: return null;
        // Java: String value = el.getStringValue();
        // Java: return null != value && value.length() > 0 ? TypeConversion.string2Integer(value) : null;
        // rust: 上述 koffice 语句由本 crate 的 `load_*` 辅助/下方实现一步完成（登记 §1.7.1 ⑤）
        self.load_int_elem(root, "Words")
    }
    // Java: Integer loadNotes(Element root)
    fn loadNotes(&self, root: &dyn std::any::Any) -> Option<i32> {
        // Java: Element el = root.element("Notes");
        // Java: if (null == el) {
        // Java: return null;
        // Java: String value = el.getStringValue();
        // Java: return null != value && value.length() > 0 ? TypeConversion.string2Integer(value) : null;
        // rust: 上述 koffice 语句由本 crate 的 `load_*` 辅助/下方实现一步完成（登记 §1.7.1 ⑤）
        self.load_int_elem(root, "Notes")
    }
    // Java: Integer loadPages(Element root)
    fn loadPages(&self, root: &dyn std::any::Any) -> Option<i32> {
        // Java: Element el = root.element("Pages");
        // Java: if (null == el) {
        // Java: return null;
        // Java: String value = el.getStringValue();
        // Java: return null != value && value.length() > 0 ? TypeConversion.string2Integer(value) : null;
        // rust: 上述 koffice 语句由本 crate 的 `load_*` 辅助/下方实现一步完成（登记 §1.7.1 ⑤）
        self.load_int_elem(root, "Pages")
    }
    // Java: Integer loadParagraphs(Element root)
    fn loadParagraphs(&self, root: &dyn std::any::Any) -> Option<i32> {
        // Java: Element el = root.element("Paragraphs");
        // Java: if (null == el) {
        // Java: return null;
        // Java: String value = el.getStringValue();
        // Java: return null != value && value.length() > 0 ? TypeConversion.string2Integer(value) : null;
        // rust: 上述 koffice 语句由本 crate 的 `load_*` 辅助/下方实现一步完成（登记 §1.7.1 ⑤）
        self.load_int_elem(root, "Paragraphs")
    }
    // Java: String loadPresentationFormat(Element root)
    fn loadPresentationFormat(&self, root: &dyn std::any::Any) -> Option<String> {
        // Java: Element el = root.element("PresentationFormat");
        // Java: if (null == el) {
        // Java: return null;
        // Java: String value = el.getStringValue();
        // Java: return null != value && value.length() > 0 ? el.getStringValue() : null;
        // rust: 上述 koffice 语句由本 crate 的 `load_*` 辅助/下方实现一步完成（登记 §1.7.1 ⑤）
        self.load_string_elem_nonempty(root, "PresentationFormat")
    }
    // Java: Boolean loadScaleCrop(Element root)
    fn loadScaleCrop(&self, root: &dyn std::any::Any) -> Option<bool> {
        // Java: Element el = root.element("ScaleCrop");
        // Java: return null == el ? null : new Boolean(el.getStringValue());
        // rust: 上述 koffice 语句由本 crate 的 `load_*` 辅助/下方实现一步完成（登记 §1.7.1 ⑤）
        let el = root.downcast_ref::<DefaultElement>()?;
        let child = el.element("ScaleCrop")?;
        let child_el = child.downcast_ref::<DefaultElement>()?;
        Some(string2_boolean(&child_el.getStringValue()))
    }
    // Java: Integer loadNumberCharacters(Element root)
    fn loadNumberCharacters(&self, root: &dyn std::any::Any) -> Option<i32> {
        // Java: Element el = root.element("CharactersWithSpaces");
        // Java: if (null == el) {
        // Java: return null;
        // Java: String value = el.getStringValue();
        // Java: return null != value && value.length() > 0 ? TypeConversion.string2Integer(value) : null;
        // rust: 上述 koffice 语句由本 crate 的 `load_*` 辅助/下方实现一步完成（登记 §1.7.1 ⑤）
        self.load_int_elem(root, "CharactersWithSpaces")
    }
    // Java: Boolean loadSharedDoc(Element root)
    fn loadSharedDoc(&self, root: &dyn std::any::Any) -> Option<bool> {
        // Java: Element el = root.element("SharedDoc");
        // Java: return null == el ? null : new Boolean(el.getStringValue());
        // rust: 上述 koffice 语句由本 crate 的 `load_*` 辅助/下方实现一步完成（登记 §1.7.1 ⑤）
        let el = root.downcast_ref::<DefaultElement>()?;
        let child = el.element("SharedDoc")?;
        let child_el = child.downcast_ref::<DefaultElement>()?;
        Some(string2_boolean(&child_el.getStringValue()))
    }
    // Java: Integer loadSlides(Element root)
    fn loadSlides(&self, root: &dyn std::any::Any) -> Option<i32> {
        // Java: Element el = root.element("Slides");
        // Java: if (null == el) {
        // Java: return null;
        // Java: String value = el.getStringValue();
        // Java: return null != value && value.length() > 0 ? TypeConversion.string2Integer(value) : null;
        // rust: 上述 koffice 语句由本 crate 的 `load_*` 辅助/下方实现一步完成（登记 §1.7.1 ⑤）
        self.load_int_elem(root, "Slides")
    }
    // Java: String loadTemplate(Element root)
    fn loadTemplate(&self, root: &dyn std::any::Any) -> Option<String> {
        // Java: Element el = root.element("Template");
        // Java: if (null == el) {
        // Java: return null;
        // Java: String value = el.getStringValue();
        // Java: return null != value && value.length() > 0 ? el.getStringValue() : null;
        // rust: 上述 koffice 语句由本 crate 的 `load_*` 辅助/下方实现一步完成（登记 §1.7.1 ⑤）
        self.load_string_elem_nonempty(root, "Template")
    }
    // Java: Integer loadTotalTime(Element root)
    fn loadTotalTime(&self, root: &dyn std::any::Any) -> Option<i32> {
        // Java: Element el = root.element("TotalTime");
        // Java: if (null == el) {
        // Java: return null;
        // Java: String value = el.getStringValue();
        // Java: return null != value && value.length() > 0 ? TypeConversion.string2Integer(value) : null;
        // rust: 上述 koffice 语句由本 crate 的 `load_*` 辅助/下方实现一步完成（登记 §1.7.1 ⑤）
        self.load_int_elem(root, "TotalTime")
    }
}


// NOTE(impl-trait): public class ExtendedPackagePropertiesUnmarshaller implements PartUnmarshaller
impl crate::io::opc::internal::PartUnmarshaller::PartUnmarshaller for ExtendedPackagePropertiesUnmarshaller {
    // NOTE(impl-trait): PackagePart unmarshall(UnmarshallContext context, InputStream in)
    fn unmarshall(&self, context: &crate::io::opc::internal::unmarshallers::UnmarshallContext::UnmarshallContext, input: Option<&mut dyn std::io::Read>) -> Result<Option<crate::io::opc::PackagePart::PackagePart>, String> {
        Ok(ExtendedPackagePropertiesUnmarshaller::unmarshall(self, context, input)?.map(|p| p.base))
    }
}
