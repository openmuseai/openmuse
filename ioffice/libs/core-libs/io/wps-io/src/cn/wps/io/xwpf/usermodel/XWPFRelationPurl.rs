#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.opc.model.POIXMLRelation
// Java: import java.util.HashMap
// Java: import java.util.Map

use std::collections::HashMap;
use std::sync::{OnceLock, Mutex};
use crate::io::opc::model::POIXMLRelation::POIXMLRelation;

// Java field: Map<String, XWPFRelationPurl> _table
static _TABLE: OnceLock<Mutex<HashMap<String, XWPFRelationPurl>>> = OnceLock::new();

fn get_table() -> &'static Mutex<HashMap<String, XWPFRelationPurl>> {
    _TABLE.get_or_init(|| Mutex::new(HashMap::new()))
}

// Java field: XWPFRelationPurl STYLES
static STYLES: OnceLock<XWPFRelationPurl> = OnceLock::new();
// Java field: XWPFRelationPurl FONT_TABLE
static FONT_TABLE: OnceLock<XWPFRelationPurl> = OnceLock::new();
// Java field: XWPFRelationPurl SETTINGS
static SETTINGS: OnceLock<XWPFRelationPurl> = OnceLock::new();
// Java field: XWPFRelationPurl NUMBERING
static NUMBERING: OnceLock<XWPFRelationPurl> = OnceLock::new();
// Java field: XWPFRelationPurl COMMENT
static COMMENT: OnceLock<XWPFRelationPurl> = OnceLock::new();
// Java field: XWPFRelationPurl ENDNOTE
static ENDNOTE: OnceLock<XWPFRelationPurl> = OnceLock::new();
// Java field: XWPFRelationPurl FOOTNOTE
static FOOTNOTE: OnceLock<XWPFRelationPurl> = OnceLock::new();
// Java field: XWPFRelationPurl HEADER
static HEADER: OnceLock<XWPFRelationPurl> = OnceLock::new();
// Java field: XWPFRelationPurl FOOTER
static FOOTER: OnceLock<XWPFRelationPurl> = OnceLock::new();
// Java field: XWPFRelationPurl IMAGES
static IMAGES: OnceLock<XWPFRelationPurl> = OnceLock::new();
// Java field: XWPFRelationPurl CHART
static CHART: OnceLock<XWPFRelationPurl> = OnceLock::new();
// Java field: XWPFRelationPurl CHART_SRC
static CHART_SRC: OnceLock<XWPFRelationPurl> = OnceLock::new();
// Java field: XWPFRelationPurl CHART_EXTERNAL_SRC
static CHART_EXTERNAL_SRC: OnceLock<XWPFRelationPurl> = OnceLock::new();
// Java field: XWPFRelationPurl CHART_EXTERNAL_XML
static CHART_EXTERNAL_XML: OnceLock<XWPFRelationPurl> = OnceLock::new();
// Java field: XWPFRelationPurl DIAGRAM_COLORS
static DIAGRAM_COLORS: OnceLock<XWPFRelationPurl> = OnceLock::new();
// Java field: XWPFRelationPurl DIAGRAM_DATA
static DIAGRAM_DATA: OnceLock<XWPFRelationPurl> = OnceLock::new();
// Java field: XWPFRelationPurl DIAGRAM_LAYOUT
static DIAGRAM_LAYOUT: OnceLock<XWPFRelationPurl> = OnceLock::new();
// Java field: XWPFRelationPurl DIAGRAM_QUICKSTYLE
static DIAGRAM_QUICKSTYLE: OnceLock<XWPFRelationPurl> = OnceLock::new();
// Java field: XWPFRelationPurl DIAGRAM_DRAWING
static DIAGRAM_DRAWING: OnceLock<XWPFRelationPurl> = OnceLock::new();
// Java field: XWPFRelationPurl THEME
static THEME: OnceLock<XWPFRelationPurl> = OnceLock::new();
// Java field: XWPFRelationPurl NOT_IMPLEMENTED
static NOT_IMPLEMENTED: OnceLock<XWPFRelationPurl> = OnceLock::new();

// Java: public class XWPFRelationPurl extends POIXMLRelation
#[derive(Clone)]
pub struct XWPFRelationPurl {
    pub(crate) content_type: Option<String>,
    pub(crate) relation: Option<String>,
    pub(crate) default_name: Option<String>,
    pub(crate) base: POIXMLRelation,
}

fn make(content_type: Option<&str>, rel: Option<&str>, default_name: Option<&str>, cls: Option<&str>) -> XWPFRelationPurl {
    let relation = XWPFRelationPurl {
        content_type: content_type.map(|s| s.to_string()),
        relation: rel.map(|s| s.to_string()),
        default_name: default_name.map(|s| s.to_string()),
        base: POIXMLRelation::new_with_cls(
            content_type.unwrap_or("").to_string(),
            rel.unwrap_or("").to_string(),
            default_name.unwrap_or("").to_string(),
            cls.map(|s| s.to_string()),
        ),
    };
    if cls.is_some() {
        if let Some(r) = rel {
            let mut table = get_table().lock().unwrap();
            if !table.contains_key(r) {
                table.insert(r.to_string(), relation.clone());
            }
        }
    }
    relation
}

fn ensure_statics() {
    STYLES.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"), Some("http://purl.oclc.org/ooxml/officeDocument/relationships/styles"), Some("/word/styles.xml"), Some("XWPFStyles")));
    FONT_TABLE.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml"), Some("http://purl.oclc.org/ooxml/officeDocument/relationships/fontTable"), Some("/word/fontTable.xml"), Some("XWPFFontTable")));
    SETTINGS.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml"), Some("http://purl.oclc.org/ooxml/officeDocument/relationships/settings"), Some("/word/settings.xml"), Some("XWPFSettings")));
    NUMBERING.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml"), Some("http://purl.oclc.org/ooxml/officeDocument/relationships/numbering"), Some("/word/numbering.xml"), Some("XWPFNumbering")));
    COMMENT.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml"), Some("http://purl.oclc.org/ooxml/officeDocument/relationships/comments"), Some("/word/comments.xml"), Some("XWPFComment")));
    ENDNOTE.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.wordprocessingml.endnotes+xml"), Some("http://purl.oclc.org/ooxml/officeDocument/relationships/endnotes"), Some("/word/endnotes.xml"), Some("XWPFEndnote")));
    FOOTNOTE.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml"), Some("http://purl.oclc.org/ooxml/officeDocument/relationships/footnotes"), Some("/word/footnotes.xml"), Some("XWPFFootnote")));
    HEADER.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.wordprocessingml.header+xml"), Some("http://purl.oclc.org/ooxml/officeDocument/relationships/header"), Some("/word/header#.xml"), Some("XWPFHeader")));
    FOOTER.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml"), Some("http://purl.oclc.org/ooxml/officeDocument/relationships/footer"), Some("/word/footer#.xml"), Some("XWPFFooter")));
    IMAGES.get_or_init(|| make(None, Some("http://purl.oclc.org/ooxml/officeDocument/relationships/image"), None, Some("XWPFPictureData")));
    CHART.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.drawingml.chart+xml"), Some("http://purl.oclc.org/ooxml/officeDocument/relationships/chart"), Some("/word/charts/chart#.xml"), Some("XWPFChartData")));
    CHART_SRC.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"), Some("http://purl.oclc.org/ooxml/officeDocument/relationships/package"), Some("/word/embeddings/Microsoft_Excel____#.xlsx"), Some("XWPFChartSrc")));
    CHART_EXTERNAL_SRC.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.oleObject"), Some("http://purl.oclc.org/ooxml/officeDocument/relationships/oleObject"), Some("/word/embeddings/oleObject#.bin"), Some("XWPFChartSrc")));
    CHART_EXTERNAL_XML.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.drawingml.chartshapes+xml"), Some("http://purl.oclc.org/ooxml/officeDocument/relationships/chartUserShapes"), Some("/word/drawings/drawing#.xml"), Some("XWPFChartSrc")));
    DIAGRAM_COLORS.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.drawingml.diagramColors+xml"), Some("http://purl.oclc.org/ooxml/officeDocument/relationships/diagramColors"), Some("/word/diagrams/colors#.xml"), Some("XWPFDiagramColors")));
    DIAGRAM_DATA.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.drawingml.diagramData+xml"), Some("http://purl.oclc.org/ooxml/officeDocument/relationships/diagramData"), Some("/word/diagrams/data#.xml"), Some("XWPFDiagramData")));
    DIAGRAM_LAYOUT.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml"), Some("http://purl.oclc.org/ooxml/officeDocument/relationships/diagramLayout"), Some("/word/diagrams/layout#.xml"), Some("XWPFDiagramLayout")));
    DIAGRAM_QUICKSTYLE.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml"), Some("http://purl.oclc.org/ooxml/officeDocument/relationships/diagramQuickStyle"), Some("/word/diagrams/quickStyle#.xml"), Some("XWPFDiagramStyle")));
    DIAGRAM_DRAWING.get_or_init(|| make(Some("application/vnd.ms-office.drawingml.diagramDrawing+xml"), Some("http://schemas.microsoft.com/office/2007/relationships/diagramDrawing"), Some("/word/diagrams/drawing#.xml"), Some("XWPFDiagramDrawing")));
    THEME.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.theme+xml"), Some("http://purl.oclc.org/ooxml/officeDocument/relationships/theme"), Some("/word/theme/theme1.xml"), Some("XWPFTheme")));
    NOT_IMPLEMENTED.get_or_init(|| make(None, None, None, Some("XWPFNotImplemented")));
}

// Java: public class XWPFRelationPurl extends POIXMLRelation
impl XWPFRelationPurl {
    // Java: inherited POIXMLRelation._cls used by XWPFFactory reflection.
    pub fn getRelationClass(&self) -> Option<&str> { self.base.getRelationClass() }
    // Java: XWPFRelationPurl(String type, String rel, String defaultName, Class<? extends POIXMLDocumentPart> cls)
    pub fn new(content_type: Option<&str>, rel: Option<&str>, default_name: Option<&str>) -> Self {
        // Java: super(type, rel, defaultName, cls);
        // Java: if (cls != null && !_table.containsKey(rel))
        // Java: _table.put(rel, this);
        make(content_type, rel, default_name, Some("POIXMLDocumentPart"))
    }

    // Java: public static XWPFRelationPurl STYLES（静态量访问器，供 onDocumentRead 双侧关系串比对）
    pub fn STYLES() -> &'static XWPFRelationPurl {
        ensure_statics();
        // Java: return STYLES;
        STYLES.get().unwrap()
    }

    // Java: public static XWPFRelationPurl THEME
    pub fn THEME() -> &'static XWPFRelationPurl {
        ensure_statics();
        // Java: return THEME;
        THEME.get().unwrap()
    }

    // Java: public static XWPFRelationPurl NUMBERING
    pub fn NUMBERING() -> &'static XWPFRelationPurl {
        ensure_statics();
        // Java: return NUMBERING;
        NUMBERING.get().unwrap()
    }

    // Java: public static XWPFRelationPurl COMMENT
    pub fn COMMENT() -> &'static XWPFRelationPurl {
        ensure_statics();
        // Java: return COMMENT;
        COMMENT.get().unwrap()
    }

    // Java: public static XWPFRelationPurl ENDNOTE
    pub fn ENDNOTE() -> &'static XWPFRelationPurl {
        ensure_statics();
        // Java: return ENDNOTE;
        ENDNOTE.get().unwrap()
    }

    // Java: public static XWPFRelationPurl FOOTNOTE
    pub fn FOOTNOTE() -> &'static XWPFRelationPurl {
        ensure_statics();
        // Java: return FOOTNOTE;
        FOOTNOTE.get().unwrap()
    }

    // Java: public static XWPFRelationPurl SETTINGS
    pub fn SETTINGS() -> &'static XWPFRelationPurl {
        ensure_statics();
        // Java: return SETTINGS;
        SETTINGS.get().unwrap()
    }

    // Java: public static XWPFRelationPurl FONT_TABLE
    pub fn FONT_TABLE() -> &'static XWPFRelationPurl {
        ensure_statics();
        // Java: return FONT_TABLE;
        FONT_TABLE.get().unwrap()
    }

    // Java: public static XWPFRelationPurl IMAGES
    pub fn IMAGES() -> &'static XWPFRelationPurl {
        ensure_statics();
        // Java: return IMAGES;
        IMAGES.get().unwrap()
    }

    pub fn getRelation(&self) -> Option<&str> {
        self.relation.as_deref()
    }

    // Java: static XWPFRelationPurl getInstance(String rel)
    pub fn getInstance(rel: &str) -> XWPFRelationPurl {
        ensure_statics();
        // Java: XWPFRelationPurl ret = (XWPFRelationPurl)_table.get(rel);
        let table = get_table().lock().unwrap();
        let mut ret = table.get(rel).cloned();
        // Java: if (null == ret)
        if ret.is_none() {
        // Java: ret = NOT_IMPLEMENTED;
            ret = Some(NOT_IMPLEMENTED.get().unwrap().clone());
        }
        // Java: return ret;
        ret.unwrap()
    }
}
