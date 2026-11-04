#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.opc.model.POIXMLRelation
// Java: import cn.wps.io.xwpf.IDmlRelation
// Java: import java.util.HashMap
// Java: import java.util.Map

use std::collections::HashMap;
use std::sync::{OnceLock, Mutex};
use crate::io::opc::model::POIXMLRelation::POIXMLRelation;
use super::PictureType::PictureType;

// Java field: Map<String, XWPFRelation> _table
static TABLE: OnceLock<Mutex<HashMap<String, XWPFRelation>>> = OnceLock::new();

fn get_table() -> &'static Mutex<HashMap<String, XWPFRelation>> {
    TABLE.get_or_init(|| Mutex::new(HashMap::new()))
}

// Java field: XWPFRelation DOCUMENT
static DOCUMENT: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation TEMPLATE
static TEMPLATE: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation MACRO_DOCUMENT
static MACRO_DOCUMENT: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation MACRO_TEMPLATE_DOCUMENT
static MACRO_TEMPLATE_DOCUMENT: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation NUMBERING
static NUMBERING: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation FONT_TABLE
static FONT_TABLE: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation SETTINGS
static SETTINGS: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation COMMENTS_EX
static COMMENTS_EX: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation STYLES
static STYLES: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation THEME
static THEME: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation THEME_OVERRIDE
static THEME_OVERRIDE: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation WEB_SETTINGS
static WEB_SETTINGS: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation HEADER
static HEADER: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation FOOTER
static FOOTER: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation HYPERLINK
static HYPERLINK: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation COMMENT
static COMMENT: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation FOOTNOTE
static FOOTNOTE: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation ENDNOTE
static ENDNOTE: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation RECIPIENTDATA
static RECIPIENTDATA: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation IMAGE_EMF
static IMAGE_EMF: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation IMAGE_WMF
static IMAGE_WMF: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation IMAGE_PICT
static IMAGE_PICT: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation IMAGE_JPEG
static IMAGE_JPEG: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation IMAGE_PNG
static IMAGE_PNG: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation IMAGE_DIB
static IMAGE_DIB: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation IMAGE_GIF
static IMAGE_GIF: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation IMAGE_PCX
static IMAGE_PCX: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation IMAGE_PSD
static IMAGE_PSD: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation IMAGE_TGA
static IMAGE_TGA: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation IMAGE_TIFF
static IMAGE_TIFF: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation IMAGE_EPS
static IMAGE_EPS: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation IMAGE_BMP
static IMAGE_BMP: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation IMAGE_WPG
static IMAGE_WPG: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation IMAGE_SVG
static IMAGE_SVG: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation IMAGES
static IMAGES: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation HDPHOTO_WDP
static HDPHOTO_WDP: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation DIAGRAM_DATA
static DIAGRAM_DATA: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation DIAGRAM_LAYOUT
static DIAGRAM_LAYOUT: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation DIAGRAM_QUICKSTYLE
static DIAGRAM_QUICKSTYLE: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation DIAGRAM_COLORS
static DIAGRAM_COLORS: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation DIAGRAM_DRAWING
static DIAGRAM_DRAWING: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation CHART
static CHART: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation CHART_EX
static CHART_EX: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation CHART_SRC
static CHART_SRC: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation DOCX_SRC
static DOCX_SRC: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation DOCM_SRC
static DOCM_SRC: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation DOC_SRC
static DOC_SRC: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation SLIDEX_SRC
static SLIDEX_SRC: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation PPTX_SRC
static PPTX_SRC: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation PPTM_SRC
static PPTM_SRC: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation SLDM_SRC
static SLDM_SRC: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation PPT_SRC
static PPT_SRC: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation XLSM_SRC
static XLSM_SRC: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation XLSX_SRC
static XLSX_SRC: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation XLSB_SRC
static XLSB_SRC: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation XLS_SRC
static XLS_SRC: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation CHART_EXTERNAL_XML
static CHART_EXTERNAL_XML: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation OLE_OBJECT
static OLE_OBJECT: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation ACTIVE_X_XML
static ACTIVE_X_XML: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation ACTIVE_X_BIN
static ACTIVE_X_BIN: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation VBA_PROJECT
static VBA_PROJECT: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation CHART_COLOR
static CHART_COLOR: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation CHART_STYLE
static CHART_STYLE: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation INK
static INK: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation CUSTOMXML_ITEM_PROPS
static CUSTOMXML_ITEM_PROPS: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation CUSTOMXML_ITEM
static CUSTOMXML_ITEM: OnceLock<XWPFRelation> = OnceLock::new();
// Java field: XWPFRelation NOT_IMPLEMENTED
static NOT_IMPLEMENTED: OnceLock<XWPFRelation> = OnceLock::new();

// Java: public class XWPFRelation extends POIXMLRelation implements IDmlRelation
#[derive(Clone)]
pub struct XWPFRelation {
    pub(crate) base: POIXMLRelation,
}

fn register(rel: &str, relation: &XWPFRelation, cls: &Option<String>) {
    if cls.is_some() {
        let mut table = get_table().lock().unwrap();
        if !table.contains_key(rel) {
            table.insert(rel.to_string(), relation.clone());
        }
    }
}

fn make(r#type: Option<&str>, rel: Option<&str>, default_name: Option<&str>, cls: Option<&str>) -> XWPFRelation {
    let rel_s = rel.unwrap_or("").to_string();
    let cls_s = cls.map(|s| s.to_string());
    let relation = XWPFRelation {
        base: POIXMLRelation::new_with_cls(
            r#type.unwrap_or("").to_string(),
            rel_s.clone(),
            default_name.unwrap_or("").to_string(),
            cls_s.clone(),
        ),
    };
    register(&rel_s, &relation, &cls_s);
    relation
}

fn ensure_statics() {
    DOCUMENT.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument"), Some("/word/document.xml"), None));
    TEMPLATE.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.wordprocessingml.template.main+xml"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument"), Some("/word/document.xml"), None));
    MACRO_DOCUMENT.get_or_init(|| make(Some("application/vnd.ms-word.document.macroEnabled.main+xml"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument"), Some("/word/document.xml"), None));
    MACRO_TEMPLATE_DOCUMENT.get_or_init(|| make(Some("application/vnd.ms-word.template.macroEnabledTemplate.main+xml"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument"), Some("/word/document.xml"), None));
    NUMBERING.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering"), Some("/word/numbering.xml"), Some("XWPFNumbering")));
    FONT_TABLE.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable"), Some("/word/fontTable.xml"), Some("XWPFFontTable")));
    SETTINGS.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings"), Some("/word/settings.xml"), Some("XWPFSettings")));
    COMMENTS_EX.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.wordprocessingml.commentsExtended+xml"), Some("http://schemas.microsoft.com/office/2011/relationships/commentsExtended"), Some("/word/commentsExtended.xml"), Some("XWPFCommentsEx")));
    STYLES.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles"), Some("/word/styles.xml"), Some("XWPFStyles")));
    THEME.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.theme+xml"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme"), Some("/word/theme/theme1.xml"), Some("XWPFTheme")));
    THEME_OVERRIDE.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.themeOverride+xml"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/themeOverride"), Some("/word/theme/themeOverride#.xml"), Some("XWPFTheme")));
    WEB_SETTINGS.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.wordprocessingml.webSettings+xml"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/webSettings"), Some("/word/webSettings.xml"), None));
    HEADER.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.wordprocessingml.header+xml"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/header"), Some("/word/header#.xml"), Some("XWPFHeader")));
    FOOTER.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer"), Some("/word/footer#.xml"), Some("XWPFFooter")));
    HYPERLINK.get_or_init(|| make(None, Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink"), None, None));
    COMMENT.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments"), Some("/word/comments.xml"), Some("XWPFComment")));
    FOOTNOTE.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/footnotes"), Some("/word/footnotes.xml"), Some("XWPFFootnote")));
    ENDNOTE.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.wordprocessingml.endnotes+xml"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/endnotes"), Some("/word/endnotes.xml"), Some("XWPFEndnote")));
    RECIPIENTDATA.get_or_init(|| make(Some("application/vnd.ms-word.mailMergeRecipientData+xml"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/recipientData"), Some("/word/recipientData.xml"), Some("XWPFRecipientData")));

    let img = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";
    IMAGE_EMF.get_or_init(|| make(Some(PictureType::EMF.content_type()), Some(img), Some("/word/media/image#.emf"), Some("XWPFPictureData")));
    IMAGE_WMF.get_or_init(|| make(Some(PictureType::WMF.content_type()), Some(img), Some("/word/media/image#.wmf"), Some("XWPFPictureData")));
    IMAGE_PICT.get_or_init(|| make(Some(PictureType::PICT.content_type()), Some(img), Some("/word/media/image#.pict"), Some("XWPFPictureData")));
    IMAGE_JPEG.get_or_init(|| make(Some(PictureType::JPEG.content_type()), Some(img), Some("/word/media/image#.jpeg"), Some("XWPFPictureData")));
    IMAGE_PNG.get_or_init(|| make(Some(PictureType::PNG.content_type()), Some(img), Some("/word/media/image#.png"), Some("XWPFPictureData")));
    IMAGE_DIB.get_or_init(|| make(Some(PictureType::DIB.content_type()), Some(img), Some("/word/media/image#.dib"), Some("XWPFPictureData")));
    IMAGE_GIF.get_or_init(|| make(Some(PictureType::GIF.content_type()), Some(img), Some("/word/media/image#.gif"), Some("XWPFPictureData")));
    IMAGE_PCX.get_or_init(|| make(Some(PictureType::PCX.content_type()), Some(img), Some("/word/media/image#.pcx"), Some("XWPFPictureData")));
    IMAGE_PSD.get_or_init(|| make(Some(PictureType::PSD.content_type()), Some(img), Some("/word/media/image#.psd"), Some("XWPFPictureData")));
    IMAGE_TGA.get_or_init(|| make(Some(PictureType::TGA.content_type()), Some(img), Some("/word/media/image#.tga"), Some("XWPFPictureData")));
    IMAGE_TIFF.get_or_init(|| make(Some(PictureType::TIFF.content_type()), Some(img), Some("/word/media/image#.tiff"), Some("XWPFPictureData")));
    IMAGE_EPS.get_or_init(|| make(Some(PictureType::EPS.content_type()), Some(img), Some("/word/media/image#.eps"), Some("XWPFPictureData")));
    IMAGE_BMP.get_or_init(|| make(Some(PictureType::BMP.content_type()), Some(img), Some("/word/media/image#.bmp"), Some("XWPFPictureData")));
    IMAGE_WPG.get_or_init(|| make(Some(PictureType::WPG.content_type()), Some(img), Some("/word/media/image#.wpg"), Some("XWPFPictureData")));
    IMAGE_SVG.get_or_init(|| make(Some(PictureType::SVG.content_type()), Some(img), Some("/word/media/image#.svg"), Some("XWPFPictureData")));
    IMAGES.get_or_init(|| make(None, Some(img), None, None));
    HDPHOTO_WDP.get_or_init(|| make(Some(PictureType::WDP.content_type()), Some("http://schemas.microsoft.com/office/2007/relationships/hdphoto"), Some("/word/media/hdphoto#.wdp"), Some("XWPFPictureData")));
    DIAGRAM_DATA.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.drawingml.diagramData+xml"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramData"), Some("/word/diagrams/data#.xml"), Some("XWPFDiagramData")));
    DIAGRAM_LAYOUT.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramLayout"), Some("/word/diagrams/layout#.xml"), Some("XWPFDiagramLayout")));
    DIAGRAM_QUICKSTYLE.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramQuickStyle"), Some("/word/diagrams/quickStyle#.xml"), Some("XWPFDiagramStyle")));
    DIAGRAM_COLORS.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.drawingml.diagramColors+xml"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramColors"), Some("/word/diagrams/colors#.xml"), Some("XWPFDiagramColors")));
    DIAGRAM_DRAWING.get_or_init(|| make(Some("application/vnd.ms-office.drawingml.diagramDrawing+xml"), Some("http://schemas.microsoft.com/office/2007/relationships/diagramDrawing"), Some("/word/diagrams/drawing#.xml"), Some("XWPFDiagramDrawing")));
    CHART.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.drawingml.chart+xml"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart"), Some("/word/charts/chart#.xml"), Some("XWPFChartData")));
    CHART_EX.get_or_init(|| make(Some("application/vnd.ms-office.chartex+xml"), Some("http://schemas.microsoft.com/office/2014/relationships/chartEx"), Some("/word/charts/chartEx#.xml"), Some("XWPFChartData")));
    CHART_SRC.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/package"), Some("/word/embeddings/Microsoft_Excel____#.xlsx"), Some("XWPFChartSrc")));
    DOCX_SRC.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.wordprocessingml.document"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/package"), Some("/word/embeddings/Microsoft_Word_Document#.docx"), Some("POIXMLDocumentPart")));
    DOCM_SRC.get_or_init(|| make(Some("application/vnd.ms-word.document.macroEnabled.12"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/package"), Some("/word/embeddings/Microsoft_Word_Macro-Enabled_Document#.docm"), Some("POIXMLDocumentPart")));
    DOC_SRC.get_or_init(|| make(Some("application/msword"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject"), Some("/word/embeddings/Microsoft_Word_97_-_2003_Document#.doc"), Some("POIXMLDocumentPart")));
    SLIDEX_SRC.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.presentationml.slide"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/package"), Some("/word/embeddings/Microsoft_PowerPoint_Slide#.sldx"), Some("POIXMLDocumentPart")));
    PPTX_SRC.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.presentationml.presentation"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/package"), Some("/word/embeddings/Microsoft_PowerPoint_Presentation#.pptx"), Some("POIXMLDocumentPart")));
    PPTM_SRC.get_or_init(|| make(Some("application/vnd.ms-powerpoint.presentation.macroEnabled.12"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/package"), Some("/word/embeddings/Microsoft_PowerPoint_Macro-Enabled_Presentation#.pptm"), Some("POIXMLDocumentPart")));
    SLDM_SRC.get_or_init(|| make(Some("application/vnd.ms-powerpoint.slide.macroEnabled.12"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/package"), Some("/word/embeddings/Microsoft_Office_PowerPoint_2007#.sldm"), Some("POIXMLDocumentPart")));
    PPT_SRC.get_or_init(|| make(Some("application/vnd.ms-powerpoint"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject"), Some("/word/embeddings/Microsoft_PowerPoint_97-2003_Presentation#.ppt"), Some("POIXMLDocumentPart")));
    XLSM_SRC.get_or_init(|| make(Some("application/vnd.ms-excel.sheet.macroEnabled.12"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/package"), Some("/word/embeddings/Microsoft_Excel_Macro-Enabled_Worksheet#.xlsm"), Some("POIXMLDocumentPart")));
    XLSX_SRC.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/package"), Some("/word/embeddings/Microsoft_Excel_Worksheet#.xlsx"), Some("POIXMLDocumentPart")));
    XLSB_SRC.get_or_init(|| make(Some("application/vnd.ms-excel.sheet.binary.macroEnabled.12"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/package"), Some("/word/embeddings/Microsoft_Excel_Binary_Worksheet#.xlsb"), Some("POIXMLDocumentPart")));
    XLS_SRC.get_or_init(|| make(Some("application/vnd.ms-excel"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject"), Some("/word/embeddings/Microsoft_Excel_97-2003_Worksheet#.xls"), Some("POIXMLDocumentPart")));
    CHART_EXTERNAL_XML.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.drawingml.chartshapes+xml"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartUserShapes"), Some("/word/drawings/drawing#.xml"), Some("XWPFChartSrc")));
    OLE_OBJECT.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.oleObject"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject"), Some("/word/embeddings/oleObject#.bin"), Some("XWPFOleObject")));
    ACTIVE_X_XML.get_or_init(|| make(Some("application/vnd.ms-office.activeX+xml"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/control"), Some("/word/activeX/activeX#.xml"), Some("POIXMLDocumentPart")));
    ACTIVE_X_BIN.get_or_init(|| make(Some("application/vnd.ms-office.activeX"), Some("http://schemas.microsoft.com/office/2006/relationships/activeXControlBinary"), Some("/word/activeX/activeX#.bin"), Some("POIXMLDocumentPart")));
    VBA_PROJECT.get_or_init(|| make(Some("application/vnd.ms-office.vbaProject"), None, Some("/word/vbaProject.bin"), Some("POIXMLDocumentPart")));
    CHART_COLOR.get_or_init(|| make(Some("application/vnd.ms-office.chartcolorstyle+xml"), Some("http://schemas.microsoft.com/office/2011/relationships/chartColorStyle"), Some("/word/charts/colors#.xml"), Some("XWPFChartSrc")));
    CHART_STYLE.get_or_init(|| make(Some("application/vnd.ms-office.chartstyle+xml"), Some("http://schemas.microsoft.com/office/2011/relationships/chartStyle"), Some("/word/charts/style#.xml"), Some("XWPFChartSrc")));
    INK.get_or_init(|| make(Some("application/inkml+xml"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml"), Some("/word/ink/ink#.xml"), Some("XWPFInkData")));
    CUSTOMXML_ITEM_PROPS.get_or_init(|| make(Some("application/vnd.openxmlformats-officedocument.customXmlProperties+xml"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXmlProps"), Some("/customXml/itemProps#.xml"), Some("XWPFCustomXmlProps")));
    CUSTOMXML_ITEM.get_or_init(|| make(Some("application/xml"), Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml"), Some("/customXml/item#.xml"), Some("XWPFCustomXml")));
    NOT_IMPLEMENTED.get_or_init(|| make(None, None, None, Some("XWPFNotImplemented")));
}

// Java: public class XWPFRelation extends POIXMLRelation implements IDmlRelation
impl XWPFRelation {
    // Java: inherited POIXMLRelation._cls used by XWPFFactory reflection.
    pub fn getRelationClass(&self) -> Option<&str> { self.base.getRelationClass() }
    // Java: XWPFRelation()
    // Rust: 命名为 **Java 构造函数同名**（`XWPFRelation`）以满足 §3.9 门面
    //       （无参构造函数是 Java 的 public 成员，门禁要求 Rust 侧有同名对应）。
    //       ⚠️ 不得另加一个额外的 `XWPFRelation()` fn：§3.8 按 **`// Java:` 注释** 的
    //       出现次序把 Java 的第 n 个同名成员与 Rust 的第 n 个同名注释区配对，
    //       多一个区会把 `XWPFRelation(String,…)` 的语句错配到无参构造函数上。
    #[allow(non_snake_case)]
    pub fn XWPFRelation() -> Self {
        // Java: super();
        XWPFRelation {
            base: POIXMLRelation::new(),
        }
    }

    // Java: XWPFRelation(String type, String rel, String defaultName, Class<? extends POIXMLDocumentPart> cls)
    pub fn new_with_cls(r#type: Option<String>, rel: String, default_name: Option<String>, cls: Option<String>) -> Self {
        // Java: super(type, rel, defaultName, cls);
        let relation = XWPFRelation {
            base: POIXMLRelation::new_with_cls(
                r#type.unwrap_or_default(),
                rel.clone(),
                default_name.unwrap_or_default(),
                cls.clone(),
            ),
        };
        // Java: if (cls != null && !_table.containsKey(rel))
        // Java: _table.put(rel, this);
        register(&rel, &relation, &cls);
        relation
    }

    // Java: static XWPFRelation getInstance(String rel)
    pub fn getInstance(rel: &str) -> XWPFRelation {
        ensure_statics();
        // Java: XWPFRelation ret = (XWPFRelation)_table.get(rel);
        let table = get_table().lock().unwrap();
        let mut ret = table.get(rel).cloned();
        // Java: if (null == ret)
        if ret.is_none() {
        // Java: ret = NOT_IMPLEMENTED;
            ret = Some(NOT_IMPLEMENTED.get().unwrap().clone());
        }
        // Java: return ret;
        ret.unwrap_or_else(|| NOT_IMPLEMENTED.get().unwrap().clone())
    }

    // Java: POIXMLRelation diagramDrawing()
    pub fn diagramDrawing() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return DIAGRAM_DRAWING;
        DIAGRAM_DRAWING.get().unwrap()
    }

    // Java: POIXMLRelation diagramData()
    pub fn diagramData() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return DIAGRAM_DATA;
        DIAGRAM_DATA.get().unwrap()
    }

    // Java: POIXMLRelation diagramQuickStyle()
    pub fn diagramQuickStyle() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return DIAGRAM_QUICKSTYLE;
        DIAGRAM_QUICKSTYLE.get().unwrap()
    }

    // Java: POIXMLRelation chart()
    pub fn chart() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return CHART;
        CHART.get().unwrap()
    }

    // Java: POIXMLRelation diagramLayout()
    pub fn diagramLayout() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return DIAGRAM_LAYOUT;
        DIAGRAM_LAYOUT.get().unwrap()
    }

    // Java: POIXMLRelation diagramColors()
    pub fn diagramColors() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return DIAGRAM_COLORS;
        DIAGRAM_COLORS.get().unwrap()
    }

    // Java: POIXMLRelation emf()
    pub fn emf() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return IMAGE_EMF;
        IMAGE_EMF.get().unwrap()
    }

    // Java: POIXMLRelation wmf()
    pub fn wmf() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return IMAGE_WMF;
        IMAGE_WMF.get().unwrap()
    }

    // Java: POIXMLRelation pict()
    pub fn pict() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return IMAGE_PICT;
        IMAGE_PICT.get().unwrap()
    }

    // Java: POIXMLRelation jpeg()
    pub fn jpeg() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return IMAGE_JPEG;
        IMAGE_JPEG.get().unwrap()
    }

    // Java: POIXMLRelation png()
    pub fn png() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return IMAGE_PNG;
        IMAGE_PNG.get().unwrap()
    }

    // Java: POIXMLRelation dib()
    pub fn dib() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return IMAGE_DIB;
        IMAGE_DIB.get().unwrap()
    }

    // Java: POIXMLRelation gif()
    pub fn gif() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return IMAGE_GIF;
        IMAGE_GIF.get().unwrap()
    }

    // Java: POIXMLRelation pcx()
    pub fn pcx() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return IMAGE_PCX;
        IMAGE_PCX.get().unwrap()
    }

    // Java: POIXMLRelation psd()
    pub fn psd() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return IMAGE_PSD;
        IMAGE_PSD.get().unwrap()
    }

    // Java: POIXMLRelation tga()
    pub fn tga() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return IMAGE_TGA;
        IMAGE_TGA.get().unwrap()
    }

    // Java: POIXMLRelation ink()
    pub fn ink() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return INK;
        INK.get().unwrap()
    }

    // Java field: XWPFRelation CUSTOMXML_ITEM_PROPS
    pub fn CUSTOMXML_ITEM_PROPS() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return CUSTOMXML_ITEM_PROPS;
        CUSTOMXML_ITEM_PROPS.get().unwrap()
    }

    // Java field: XWPFRelation CUSTOMXML_ITEM
    pub fn CUSTOMXML_ITEM() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return CUSTOMXML_ITEM;
        CUSTOMXML_ITEM.get().unwrap()
    }

    // Java field: XWPFRelation STYLES（静态量访问器，供 onDocumentRead 双侧关系串比对）
    pub fn STYLES() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return STYLES;
        STYLES.get().unwrap()
    }

    // Java field: XWPFRelation THEME
    pub fn THEME() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return THEME;
        THEME.get().unwrap()
    }

    // Java field: XWPFRelation NUMBERING
    pub fn NUMBERING() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return NUMBERING;
        NUMBERING.get().unwrap()
    }

    // Java field: XWPFRelation COMMENT
    pub fn COMMENT() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return COMMENT;
        COMMENT.get().unwrap()
    }

    // Java field: XWPFRelation ENDNOTE
    pub fn ENDNOTE() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return ENDNOTE;
        ENDNOTE.get().unwrap()
    }

    // Java field: XWPFRelation FOOTNOTE
    pub fn FOOTNOTE() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return FOOTNOTE;
        FOOTNOTE.get().unwrap()
    }

    // Java field: XWPFRelation SETTINGS
    pub fn SETTINGS() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return SETTINGS;
        SETTINGS.get().unwrap()
    }

    // Java field: XWPFRelation COMMENTS_EX
    pub fn COMMENTS_EX() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return COMMENTS_EX;
        COMMENTS_EX.get().unwrap()
    }

    // Java field: XWPFRelation FONT_TABLE
    pub fn FONT_TABLE() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return FONT_TABLE;
        FONT_TABLE.get().unwrap()
    }

    // Java field: XWPFRelation IMAGES
    pub fn IMAGES() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return IMAGES;
        IMAGES.get().unwrap()
    }

    // Java: POIXMLRelation tiff()
    pub fn tiff() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return IMAGE_TIFF;
        IMAGE_TIFF.get().unwrap()
    }

    // Java: POIXMLRelation eps()
    pub fn eps() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return IMAGE_EPS;
        IMAGE_EPS.get().unwrap()
    }

    // Java: POIXMLRelation bmp()
    pub fn bmp() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return IMAGE_BMP;
        IMAGE_BMP.get().unwrap()
    }

    // Java: POIXMLRelation wpg()
    pub fn wpg() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return IMAGE_WPG;
        IMAGE_WPG.get().unwrap()
    }

    // Java: POIXMLRelation wdp()
    pub fn wdp() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return HDPHOTO_WDP;
        HDPHOTO_WDP.get().unwrap()
    }

    // Java: POIXMLRelation svg()
    pub fn svg() -> &'static XWPFRelation {
        ensure_statics();
        // Java: return IMAGE_SVG;
        IMAGE_SVG.get().unwrap()
    }
}
