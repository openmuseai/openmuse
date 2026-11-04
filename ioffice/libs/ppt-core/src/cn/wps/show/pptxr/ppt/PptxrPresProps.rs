#![allow(dead_code, non_snake_case, unused_variables, unused_imports)]

use std::cell::RefCell;
use std::io;
use std::rc::Rc;

// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
// Java: import cn.wps.kfc.opcr.OpcrPackagePart
use crate::cn::wps::show::pptxr::util::OpcrPackagePartRef::OpcrPackagePartRef;
// Java: import cn.wps.show.app.KmoPresentation
use crate::cn::wps::show::util::layout_shape::KmoPresentation;
// Java: import cn.wps.show.data.Presentation
// Java: import cn.wps.show.pptxr.handler.PresPropsHandler
// Java: import cn.wps.show.pptxr.util.UnifiedXmlReader
use crate::cn::wps::show::pptxr::util::UnifiedXmlReader::UnifiedXmlReader;
// Java: import java.io.IOException
// Java: import java.io.InputStream

/// `java cn.wps.show.data.presproperty.PresProperty` 的 Rust 端口。
///
/// `rust:` `cn.wps.show.data.presproperty` 在 default feature 下不可达
/// （`data/presproperty/mod.rs` 为空），故端口在本文件承载 `PresProperty`
/// 被 `PresPropsHandler` 写入的字段面。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PresProperty {
    // java private int serverZoom;
    pub serverZoom: Option<i32>,
    // java private boolean showSpecialPlsOnTitleSld;
    pub showSpecialPlsOnTitleSld: Option<bool>,
    // java private boolean rtl;
    pub rtl: Option<bool>,
}

// java public final class PptxrPresProps
pub struct PptxrPresProps {
    // java private KmoPresentation mPres;
    mPres: Rc<RefCell<KmoPresentation>>,
    // java private OpcrPackagePart mPart;
    mPart: Option<OpcrPackagePartRef>,
    // rust: 真实解析出的 presProperty（可观测面）
    mPresProperty: PresProperty,
    // rust: 真实读取到的 part 字节数（可观测面）
    mBytesRead: usize,
}

// java public final class PptxrPresProps
impl PptxrPresProps {
    // java public PptxrPresProps(KmoPresentation pres, OpcrPackagePart part)
    pub fn PptxrPresProps(pres: Rc<RefCell<KmoPresentation>>, part: Option<OpcrPackagePartRef>) -> Self {
        // java this.mPres = pres;
        // java this.mPart = part;
        Self {
            mPres: pres,
            mPart: part,
            mPresProperty: PresProperty::default(),
            mBytesRead: 0,
        }
    }

    // java public void open() throws IOException
    pub fn open(&mut self) -> io::Result<()> {
        // java if (this.mPart != null)
        if let Some(part) = self.mPart.clone() {
            // java InputStream stream = this.mPart.getInputStream();
            let mut stream = part.getInputStream_default()?;
            // java Presentation presentationData = Presentation.open(this.mPres.protocol());
            // java PresPropsHandler handler = new PresPropsHandler(presentationData.presProperty(), (HandlerCallback)null);
            /* DEFERRED_NON_FUNCTION: `PresPropsHandler` 依赖 `kfc/xsd/pml/P.java` 的
             * `p:presentationPr` 子树 id↔name 表（3211548/3211549/…/3211564），
             * 该表在 Rust 侧尚未建立（`kfc_alpha::xsd::pml::PMap` 是 `todo!()` 桩），
             * 且 `cn.wps.show.data.presproperty.PresProperty` 在 default feature 下不可达。
             * `presProps` 不参与幻灯片内容/尺寸/主题/顺序，不影响渲染管线推进。
             * owner: pp-03 presProps / wave: S4。 */
            // java try { UnifiedXmlReader.read((InputStream)stream, handler); }
            let mut raw = Vec::new();
            {
                use std::io::Read;
                stream.read_to_end(&mut raw)?;
            }
            self.mBytesRead = raw.len();
            self.mPresProperty = parse_pres_property(&raw);
            // java catch (Throwable e) { IOLog.d(TAG, "Throwable", e); }
            // java this.mPres.updateProtocol(presentationData.save());
        }
        Ok(())
    }

    // rust: 可观测面（Java 无此方法）
    pub fn presProperty(&self) -> PresProperty {
        self.mPresProperty.clone()
    }
    pub fn bytesRead(&self) -> usize {
        self.mBytesRead
    }
}

/// 真实 `ppt/presProps.xml` 根元素属性解析。
///
/// `rust:` Java 由 `PresPropsHandler` 按 id 驱动（见上方 DEFERRED 说明）；
/// 端口在此按 **XML 本地名** 解析同一批根属性（`p:presentationPr/@serverZoom` 等），
/// 语义与 `PresPropsHandler.startElement` 的写入分支一致。
fn parse_pres_property(raw: &[u8]) -> PresProperty {
    let xml = String::from_utf8_lossy(raw);
    let mut out = PresProperty::default();
    let mut reader = quick_xml::Reader::from_str(&xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(quick_xml::events::Event::Start(ref e))
            | Ok(quick_xml::events::Event::Empty(ref e)) => {
                for a in e.attributes().flatten() {
                    let key = String::from_utf8_lossy(a.key.as_ref()).into_owned();
                    let local = key.rsplit(':').next().unwrap_or(&key).to_string();
                    let val = a.unescape_value().unwrap_or_default().into_owned();
                    match local.as_str() {
                        "serverZoom" => out.serverZoom = val.parse::<i32>().ok(),
                        "showSpecialPlsOnTitleSld" => {
                            out.showSpecialPlsOnTitleSld = Some(val == "1" || val == "true")
                        }
                        "rtl" => out.rtl = Some(val == "1" || val == "true"),
                        _ => {}
                    }
                }
            }
            Ok(quick_xml::events::Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }
    out
}
