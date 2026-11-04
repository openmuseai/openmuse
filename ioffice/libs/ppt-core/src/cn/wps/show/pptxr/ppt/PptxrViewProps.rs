#![allow(dead_code, non_snake_case, unused_variables, unused_imports)]

use std::io;

// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.kfc.opcr.OpcrPackagePart
use crate::cn::wps::show::pptxr::util::OpcrPackagePartRef::OpcrPackagePartRef;
// Java: import cn.wps.kfc.opcr.OpcrRelationships
use kfc_kfc_alpha::cn::wps::kfc::opcr::OpcrRelationships::OpcrRelationships;
// Java: import cn.wps.show.PptResourceFactory
// Java: import cn.wps.show.data.viewproperty.ViewProperty
// Java: import cn.wps.show.pptxr.handler.ViewPropsHandler
// Java: import cn.wps.show.pptxr.util.UnifiedXmlReader
// Java: import java.io.FileNotFoundException
// Java: import java.io.IOException
// Java: import java.io.InputStream

/// `java cn.wps.show.data.viewproperty.ViewProperty` 的 Rust 端口。
///
/// `rust:` `cn.wps.show.data.viewproperty` 在 default feature 下不可达
/// （`data/viewproperty/mod.rs` 被注释掉），故端口在本文件承载 `ViewProperty`
/// 被 `ViewPropsHandler` 写入的字段面（`p:viewPr` 根属性 + `p:normalViewPr` 子属性）。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ViewProperty {
    // java private boolean showComments;
    pub showComments: Option<bool>,
    // java private boolean showCommentsByNoAuthor;
    pub showCommentsByNoAuthor: Option<bool>,
    // java private boolean snapToGrid;
    pub snapToGrid: Option<bool>,
    // java private boolean snapToObjects;
    pub snapToObjects: Option<bool>,
    // java private boolean gridVisible;
    pub gridVisible: Option<bool>,
    // java private int gridSpacingCx;
    pub gridSpacingCx: Option<i32>,
    // java private int gridSpacingCy;
    pub gridSpacingCy: Option<i32>,
    // java boolean hasBeenSaved
    pub saved: bool,
}

impl ViewProperty {
    // java void save()
    pub fn save(&mut self) {
        self.saved = true;
    }
}

/// `java cn.wps.show.pptxr.handler.ViewPropertyHandlerCallback` 的 Rust 端口
/// （把 `r:id` 解析成 media id；`OpcrRelationships` 可为 null —— Java 允许传 null）。
#[derive(Default)]
pub struct ViewPropertyHandlerCallback {
    // java private OpcrRelationships mRelationships;
    pub mRelationships: Option<OpcrRelationships>,
}

impl ViewPropertyHandlerCallback {
    // java ViewPropertyHandlerCallback(OpcrRelationships relationships)
    pub fn ViewPropertyHandlerCallback(relationships: Option<OpcrRelationships>) -> Self {
        Self {
            mRelationships: relationships,
        }
    }
}

// java public class PptxrViewProps
pub struct PptxrViewProps {
    // java private ViewProperty mViewProperty;
    mViewProperty: ViewProperty,
    // java private OpcrPackagePart mPart;
    mPart: Option<OpcrPackagePartRef>,
    // rust: 真实读取到的 part 字节数（可观测面）
    mBytesRead: usize,
}

// java public class PptxrViewProps
impl PptxrViewProps {
    // java public PptxrViewProps(ViewProperty viewProperty, OpcrPackagePart part)
    pub fn PptxrViewProps(viewProperty: ViewProperty, part: Option<OpcrPackagePartRef>) -> Self {
        // java this.mViewProperty = viewProperty;
        // java this.mPart = part;
        Self {
            mViewProperty: viewProperty,
            mPart: part,
            mBytesRead: 0,
        }
    }

    // java public void open() throws IOException
    pub fn open(&mut self) -> io::Result<()> {
        // java if (this.mPart != null)
        if let Some(part) = self.mPart.clone() {
            // java InputStream stream = this.mPart.getInputStream();
            let mut stream = part.getInputStream_default()?;
            // java ViewPropertyHandlerCallback callback = new ViewPropertyHandlerCallback(this.mPart.getRelationships());
            let _callback = ViewPropertyHandlerCallback::ViewPropertyHandlerCallback(
                Some(part.getRelationships()?),
            );
            // java ViewPropsHandler handler = new ViewPropsHandler(this.mViewProperty, callback);
            /* DEFERRED_NON_FUNCTION: `ViewPropsHandler` 依赖 `kfc/xsd/pml/P.java` 的
             * `p:viewPr` 子树 id↔name 表（3211651…3211659、3211633…3211649），
             * 该表在 Rust 侧尚未建立（`kfc_alpha::xsd::pml::PMap` 是 `todo!()` 桩）。
             * `viewProps` 不参与幻灯片内容/尺寸/主题/顺序，不影响渲染管线推进。
             * owner: pp-03 viewProps / wave: S4。 */
            // java try { UnifiedXmlReader.read((InputStream)stream, handler); }
            let mut raw = Vec::new();
            {
                use std::io::Read;
                stream.read_to_end(&mut raw)?;
            }
            self.mBytesRead = raw.len();
            self.mViewProperty = parse_view_property(&raw);
            // java catch (Throwable e) { IOLog.d(TAG, "Throwable", e); }
        }
        Ok(())
    }

    // java public static ViewProperty buildDefaultViewProperty() throws IOException
    pub fn buildDefaultViewProperty() -> io::Result<ViewProperty> {
        // java ViewProperty defaultViewProperty = new ViewProperty();
        let mut defaultViewProperty = ViewProperty::default();
        // java InputStream stream = PptResourceFactory.getResPrivider().getViewProps();
        // java if (stream == null) { throw new FileNotFoundException("The resources is not exist."); }
        /* DEFERRED_NON_FUNCTION: Java 从打包资源读默认 viewProps
         * （`cn.wps.show.PptResourceFactory.getResPrivider().getViewProps()`）；
         * Rust 侧 `PptResourceFactory`（`show/mod.rs`）在 default feature 下不可达，
         * 没有 viewProps 资源提供者。端口在此按 Java `ViewPropsHandler` 的默认值
         * 构造同一份默认 `ViewProperty`（不抛 FileNotFoundException —— 该异常在
         * Rust 端口无对应资源可缺）。owner: pp-03 viewProps / wave: S4。 */
        defaultViewProperty.showComments = Some(false);
        defaultViewProperty.showCommentsByNoAuthor = Some(false);
        defaultViewProperty.snapToGrid = Some(true);
        defaultViewProperty.snapToObjects = Some(true);
        defaultViewProperty.gridVisible = Some(true);
        // java return defaultViewProperty;
        Ok(defaultViewProperty)
    }

    // rust: 可观测面（Java 无此方法）
    pub fn viewProperty(&self) -> ViewProperty {
        self.mViewProperty.clone()
    }
    pub fn bytesRead(&self) -> usize {
        self.mBytesRead
    }
}

/// 真实 `ppt/viewProps.xml` 解析（按 XML 本地名，语义与 `ViewPropsHandler` 的写入分支一致）。
fn parse_view_property(raw: &[u8]) -> ViewProperty {
    let xml = String::from_utf8_lossy(raw);
    let mut out = ViewProperty::default();
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
                    let as_bool = val == "1" || val == "true";
                    match local.as_str() {
                        "showComments" => out.showComments = Some(as_bool),
                        "showCommentsByNoAuthor" => out.showCommentsByNoAuthor = Some(as_bool),
                        "snapToGrid" => out.snapToGrid = Some(as_bool),
                        "snapToObjects" => out.snapToObjects = Some(as_bool),
                        "gridVisible" => out.gridVisible = Some(as_bool),
                        "cx" => out.gridSpacingCx = val.parse::<i32>().ok(),
                        "cy" => out.gridSpacingCy = val.parse::<i32>().ok(),
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
