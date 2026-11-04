use std::{collections::BTreeMap,sync::{LazyLock,Mutex,MutexGuard}};
use crate::kfc::{opcr::OpcRelation::OpcRelation,opcw::native::RelationRef,clipr::XlsClipRelationsImpl::XlsClipRelationsImpl};
use super::{IOxfXlsRelations::IOxfXlsRelations,OxfXlsRelationsImpl::OxfXlsRelationsImpl};
// java public final class OxfXlRelations {
pub struct OxfXlRelations;
// java private static IOxfXlsRelations sRelations;
static sRelations:Mutex<Option<Box<dyn IOxfXlsRelations>>>=Mutex::new(None);
// Native field access guard; Java decisions remain in their original methods.
fn strategy()->MutexGuard<'static,Option<Box<dyn IOxfXlsRelations>>>{sRelations.lock().unwrap_or_else(std::sync::PoisonError::into_inner)}
// Native decoder of the 73 exact static constructor literals, including null.
fn literal(name:&str)->OpcRelation{
    static DATA:LazyLock<BTreeMap<String,[Option<String>;3]>>=LazyLock::new(||serde_json::from_str(include_str!("resources/xl-relations.json")).expect("original relation literals"));
    let v=&DATA[name];OpcRelation::OpcRelation(v[0].clone(),v[1].clone(),v[2].clone())
}
macro_rules! relations{($($name:ident),*$(,)?)=>{
    $(pub static $name:LazyLock<OpcRelation>=LazyLock::new(||literal(stringify!($name)));)*
    // Native reflection lookup; ET methods below access their original static fields.
    pub fn constant(name:&str)->&'static OpcRelation{match name{$(stringify!($name)=>&*$name,)*_=>panic!("NoSuchFieldException")}}
};}
// Exact original literal static fields, in original declaration order.
relations!{
    // java public static final OpcRelation WORKBOOK = new OpcRelation("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument", "/xl/workbook.xml");
    WORKBOOK,
    // java public static final OpcRelation MACROS_WORKBOOK = new OpcRelation("application/vnd.ms-excel.sheet.macroEnabled.main+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument", "/xl/workbook.xml");
    MACROS_WORKBOOK,
    // java public static final OpcRelation TEMPLATE_WORKBOOK = new OpcRelation("application/vnd.openxmlformats-officedocument.spreadsheetml.template.main+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument", "/xl/workbook.xml");
    TEMPLATE_WORKBOOK,
    // java public static final OpcRelation MACRO_TEMPLATE_WORKBOOK = new OpcRelation("application/vnd.ms-excel.template.macroEnabled.main+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument", "/xl/workbook.xml");
    MACRO_TEMPLATE_WORKBOOK,
    // java public static final OpcRelation MACRO_ADDIN_WORKBOOK = new OpcRelation("application/vnd.ms-excel.addin.macroEnabled.main+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument", "/xl/workbook.xml");
    MACRO_ADDIN_WORKBOOK,
    // java public static final OpcRelation WORKSHEET = new OpcRelation("application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet", "/xl/worksheets/sheet#.xml");
    WORKSHEET,
    // java public static final OpcRelation MACROSHEET = new OpcRelation("application/vnd.openxmlformats-officedocument.spreadsheetml.macrosheet+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/macrosheet", "/xl/chartsheets/sheet#.xml");
    MACROSHEET,
    // java public static final OpcRelation DIALOGSHEET = new OpcRelation("application/vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/dialogsheet", "/xl/chartsheets/sheet#.xml");
    DIALOGSHEET,
    // java public static final OpcRelation CHARTSHEET = new OpcRelation("application/vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartsheet", "/xl/chartsheets/sheet#.xml");
    CHARTSHEET,
    // java public static final OpcRelation CHART = new OpcRelation("application/vnd.openxmlformats-officedocument.drawingml.chart+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart", "/xl/charts/chart#.xml");
    CHART,
    // java public static final OpcRelation DIAGRAMSQUICKSTYLE = new OpcRelation("application/vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramQuickStyle", "/xl/diagrams/quickStyle#.xml");
    DIAGRAMSQUICKSTYLE,
    // java public static final OpcRelation DIAGRAMSLAYOUT = new OpcRelation("application/vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramLayout", "/xl/diagrams/layout#.xml");
    DIAGRAMSLAYOUT,
    // java public static final OpcRelation DIAGRAMSDATA = new OpcRelation("application/vnd.openxmlformats-officedocument.drawingml.diagramData+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramData", "/xl/diagrams/data#.xml");
    DIAGRAMSDATA,
    // java public static final OpcRelation DIAGRAMSDRAWING = new OpcRelation("application/vnd.ms-office.drawingml.diagramDrawing+xml", "http://schemas.microsoft.com/office/2007/relationships/diagramDrawing", "/xl/diagrams/drawing#.xml");
    DIAGRAMSDRAWING,
    // java public static final OpcRelation DIAGRAMSCOLORS = new OpcRelation("application/vnd.openxmlformats-officedocument.drawingml.diagramColors+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramColors", "/xl/diagrams/colors#.xml");
    DIAGRAMSCOLORS,
    // java public static final OpcRelation SHARED_STRINGS = new OpcRelation("application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings", "/xl/sharedStrings.xml");
    SHARED_STRINGS,
    // java public static final OpcRelation STYLES = new OpcRelation("application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles", "/xl/styles.xml");
    STYLES,
    // java public static final OpcRelation CELL_IMAGES = new OpcRelation("application/vnd.wps-officedocument.cellimage+xml", "http://www.wps.cn/officeDocument/2020/cellImage", "/xl/cellimages.xml");
    CELL_IMAGES,
    // java public static final OpcRelation DRAWINGS = new OpcRelation("application/vnd.openxmlformats-officedocument.drawing+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing", "/xl/drawings/drawing#.xml");
    DRAWINGS,
    // java public static final OpcRelation VML_DRAWINGS = new OpcRelation("application/vnd.openxmlformats-officedocument.vmlDrawing", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing", "/xl/drawings/vmlDrawing#.vml");
    VML_DRAWINGS,
    // java public static final OpcRelation CUSTOM_XML_MAPPINGS = new OpcRelation("application/xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/xmlMaps", "/xl/xmlMaps.xml");
    CUSTOM_XML_MAPPINGS,
    // java public static final OpcRelation SINGLE_XML_CELLS = new OpcRelation("application/vnd.openxmlformats-officedocument.spreadsheetml.tableSingleCells+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableSingleCells", "/tables/tableSingleCells#.xml");
    SINGLE_XML_CELLS,
    // java public static final OpcRelation TABLE = new OpcRelation("application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/table", "/xl/tables/table#.xml");
    TABLE,
    // java public static final OpcRelation IMAGES = new OpcRelation((String)null, "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image", (String)null);
    IMAGES,
    // java public static final OpcRelation IMAGE_EMF = new OpcRelation("image/x-emf", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image", "/xl/media/image#.emf");
    IMAGE_EMF,
    // java public static final OpcRelation IMAGE_WMF = new OpcRelation("image/x-wmf", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image", "/xl/media/image#.wmf");
    IMAGE_WMF,
    // java public static final OpcRelation IMAGE_PICT = new OpcRelation("image/pict", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image", "/xl/media/image#.pict");
    IMAGE_PICT,
    // java public static final OpcRelation IMAGE_JPEG = new OpcRelation("image/jpeg", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image", "/xl/media/image#.jpeg");
    IMAGE_JPEG,
    // java public static final OpcRelation IMAGE_PNG = new OpcRelation("image/png", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image", "/xl/media/image#.png");
    IMAGE_PNG,
    // java public static final OpcRelation IMAGE_DIB = new OpcRelation("image/dib", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image", "/xl/media/image#.dib");
    IMAGE_DIB,
    // java public static final OpcRelation IMAGE_GIF = new OpcRelation("image/gif", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image", "/xl/media/image#.gif");
    IMAGE_GIF,
    // java public static final OpcRelation IMAGE_BMP = new OpcRelation("image/bmp", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image", "/xl/media/image#.bmp");
    IMAGE_BMP,
    // java public static final OpcRelation HDPHOTO = new OpcRelation("hdphoto/wdp", "http://schemas.microsoft.com/office/2007/relationships/hdphoto", "/xl/media/hdphoto#.wdp");
    HDPHOTO,
    // java public static final OpcRelation SHEET_COMMENTS = new OpcRelation("application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments", "/xl/comments#.xml");
    SHEET_COMMENTS,
    // java public static final OpcRelation SHEET_HYPERLINKS = new OpcRelation((String)null, "http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink", (String)null);
    SHEET_HYPERLINKS,
    // java public static final OpcRelation VBA_MACROS = new OpcRelation("application/vnd.ms-office.vbaProject", "http://schemas.microsoft.com/office/2006/relationships/vbaProject", "/xl/vbaProject.bin");
    VBA_MACROS,
    // java public static final OpcRelation ACTIVEX_CONTROLS = new OpcRelation("application/vnd.ms-office.activeX+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/control", "/xl/activeX/activeX#.xml");
    ACTIVEX_CONTROLS,
    // java public static final OpcRelation ACTIVEX_BINS = new OpcRelation("application/vnd.ms-office.activeX", "http://schemas.microsoft.com/office/2006/relationships/activeXControlBinary", "/xl/activeX/activeX#.bin");
    ACTIVEX_BINS,
    // java public static final OpcRelation THEME = new OpcRelation("application/vnd.openxmlformats-officedocument.theme+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme", "/xl/theme/theme#.xml");
    THEME,
    // java public static final OpcRelation THEMEOVERRIDE = new OpcRelation("application/vnd.openxmlformats-officedocument.themeOverride+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/themeOverride", "/xl/theme/themeOverride#.xml");
    THEMEOVERRIDE,
    // java public static final OpcRelation CALC_CHAIN = new OpcRelation("application/vnd.openxmlformats-officedocument.spreadsheetml.calcChain+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/calcChain", "/xl/calcChain.xml");
    CALC_CHAIN,
    // java public static final OpcRelation EXTERNAL_LINK = new OpcRelation("application/vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/externalLink", "/xl/externalLinks/externalLink#.xml");
    EXTERNAL_LINK,
    // java public static final OpcRelation EXTERNAL_LINK_PATH = new OpcRelation((String)null, "http://schemas.openxmlformats.org/officeDocument/2006/relationships/externalLinkPath", (String)null);
    EXTERNAL_LINK_PATH,
    // java public static final OpcRelation HYPERLINK = new OpcRelation((String)null, "http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink", (String)null);
    HYPERLINK,
    // java public static final OpcRelation INK = new OpcRelation("application/inkml+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml", "/xl/ink/ink#.xml");
    INK,
    // java public static final OpcRelation PICTURE_XML = new OpcRelation("application/vnd.ms-office.DrsPicture+xml", "http://schemas.microsoft.com/office/2006/relationships/pictureXml", "/drs/picturexml.xml");
    PICTURE_XML,
    // java public static final OpcRelation INK_XML = new OpcRelation("application/inkml+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml", "/drs/ink/ink#.xml");
    INK_XML,
    // java public static final OpcRelation DOWNREV_XML = new OpcRelation("application/vnd.ms-office.DrsDownRev+xml", "http://schemas.microsoft.com/office/2006/relationships/downRev", "/drs/downrev.xml");
    DOWNREV_XML,
    // java public static final OpcRelation SHAPE_XML = new OpcRelation("application/vnd.ms-office.DrsShape+xml", "http://schemas.microsoft.com/office/2006/relationships/shapeXml", "/drs/shapexml.xml");
    SHAPE_XML,
    // java public static final OpcRelation ITEM = new OpcRelation("application/vnd.openxmlformats-officedocument.customXml+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml", "/customXml/item#.xml");
    ITEM,
    // java public static final OpcRelation ITEMPROPS = new OpcRelation("application/vnd.openxmlformats-officedocument.customXmlProperties+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXmlProps", "/customXml/itemProps#.xml");
    ITEMPROPS,
    // java public static final OpcRelation WEBEXTENSION = new OpcRelation("application/vnd.wps-officedocument.webExtension+xml", "http://www.wps.cn/officeDocument/2018/webExtension", "/xl/webExtensions/webExtension#.xml");
    WEBEXTENSION,
    // java public static final OpcRelation CHART_COLOR = new OpcRelation("application/vnd.ms-office.chartcolorstyle+xml", "http://schemas.microsoft.com/office/2011/relationships/chartColorStyle", "/xl/charts/colors#.xml");
    CHART_COLOR,
    // java public static final OpcRelation CHART_STYLE = new OpcRelation("application/vnd.ms-office.chartstyle+xml", "http://schemas.microsoft.com/office/2011/relationships/chartStyle", "/xl/charts/style#.xml");
    CHART_STYLE,
    // java public static final OpcRelation BIN_SRC = new OpcRelation("application/vnd.openxmlformats-officedocument.oleObject", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject", "/xl/embeddings/oleObject#.bin");
    BIN_SRC,
    // java public static final OpcRelation DOCX_SRC = new OpcRelation("application/vnd.openxmlformats-officedocument.wordprocessingml.document", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package", "/xl/embeddings/Microsoft_Word_Document#.docx");
    DOCX_SRC,
    // java public static final OpcRelation DOCM_SRC = new OpcRelation("application/vnd.ms-word.document.macroEnabled.12", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package", "/xl/embeddings/Microsoft_Word_Macro-Enabled_Document#.docm");
    DOCM_SRC,
    // java public static final OpcRelation DOC_SRC = new OpcRelation("application/msword", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject", "/xl/embeddings/Microsoft_Word_97_-_2003_Document#.doc");
    DOC_SRC,
    // java public static final OpcRelation SLIDEX_SRC = new OpcRelation("application/vnd.openxmlformats-officedocument.presentationml.slide", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package", "/xl/embeddings/Microsoft_PowerPoint_Slide#.sldx");
    SLIDEX_SRC,
    // java public static final OpcRelation PPTX_SRC = new OpcRelation("application/vnd.openxmlformats-officedocument.presentationml.presentation", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package", "/xl/embeddings/Microsoft_PowerPoint_Presentation#.pptx");
    PPTX_SRC,
    // java public static final OpcRelation SLDM_SRC = new OpcRelation("application/vnd.ms-powerpoint.slide.macroEnabled.12", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package", "/xl/embeddings/Microsoft_PowerPoint_Macro-Enabled_Slide#.sldm");
    SLDM_SRC,
    // java public static final OpcRelation PPTM_SRC = new OpcRelation("application/vnd.ms-powerpoint.presentation.macroEnabled.12", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package", "/xl/embeddings/Microsoft_PowerPoint_Macro-Enabled_Presentation#.pptm");
    PPTM_SRC,
    // java public static final OpcRelation PPT_SRC = new OpcRelation("application/vnd.ms-powerpoint", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject", "/xl/embeddings/Microsoft_PowerPoint_97-2003_Presentation#.ppt");
    PPT_SRC,
    // java public static final OpcRelation XLSM_SRC = new OpcRelation("application/vnd.ms-excel.sheet.macroEnabled.12", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package", "/xl/embeddings/Microsoft_Excel_Macro-Enabled_Worksheet#.xlsm");
    XLSM_SRC,
    // java public static final OpcRelation XLSX_SRC = new OpcRelation("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package", "/xl/embeddings/Microsoft_Excel_Worksheet#.xlsx");
    XLSX_SRC,
    // java public static final OpcRelation XLSB_SRC = new OpcRelation("application/vnd.ms-excel.sheet.binary.macroEnabled.12", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package", "/xl/embeddings/Microsoft_Excel_Binary_Worksheet#.xlsb");
    XLSB_SRC,
    // java public static final OpcRelation XLS_SRC = new OpcRelation("application/vnd.ms-excel", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject", "/xl/embeddings/Microsoft_Excel_97-2003_Worksheet#.xls");
    XLS_SRC,
    // java public static final OpcRelation TIMINGINFO = new OpcRelation("application/vnd.openxmlformats-officedocument.presentationml.timingInfo+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/timingInfo", "/drs/timingInfo.xml");
    TIMINGINFO,
    // java public static final OpcRelation EXTERNALPATH = new OpcRelation((String)null, "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject", (String)null);
    EXTERNALPATH,
    // java public static final OpcRelation CONNECTIONS = new OpcRelation("application/vnd.openxmlformats-officedocument.spreadsheetml.connections+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/connections", "/xl/connections.xml");
    CONNECTIONS,
    // java public static final OpcRelation PIVOT_TABLE = new OpcRelation("application/vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotTable", "/xl/pivotTables/pivotTable#.xml");
    PIVOT_TABLE,
    // java public static final OpcRelation PIVOT_CACHE_DEFINITION = new OpcRelation("application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheDefinition", "/xl/pivotCache/pivotCacheDefinition#.xml");
    PIVOT_CACHE_DEFINITION,
    // java public static final OpcRelation PIVOT_CACHE_RECORDS = new OpcRelation("application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheRecords", "/xl/pivotCache/pivotCacheRecords#.xml");
    PIVOT_CACHE_RECORDS,
}

impl OxfXlRelations{
    // java public OxfXlRelations() { super(); }
    pub fn OxfXlRelations()->Self{Self}
    // java public static void setXlsRelation() { sRelations = new OxfXlsRelationsImpl(); }
    pub fn setXlsRelation(){*strategy()=Some(Box::new(OxfXlsRelationsImpl::OxfXlsRelationsImpl()));}
    // java public static void setClipRelation() { sRelations = new XlsClipRelationsImpl(); }
    pub fn setClipRelation(){*strategy()=Some(Box::new(XlsClipRelationsImpl::XlsClipRelationsImpl()));}
    // java public static OpcRelation chart() { return sRelations.chart(); }
    pub fn chart()->RelationRef{strategy().as_mut().expect("NullPointerException").chart()}
    // java public static OpcRelation chartColor() { return sRelations.chartColor(); }
    pub fn chartColor()->RelationRef{strategy().as_mut().expect("NullPointerException").chartColor()}
    // java public static OpcRelation chartStyle() { return sRelations.chartStyle(); }
    pub fn chartStyle()->RelationRef{strategy().as_mut().expect("NullPointerException").chartStyle()}
    // java public static OpcRelation chartExternalSrc() { return sRelations.chartExternalSrc(); }
    pub fn chartExternalSrc()->RelationRef{strategy().as_mut().expect("NullPointerException").chartExternalSrc()}
    // java public static OpcRelation diagramQuickStyle() { return sRelations.diagramQuickStyle(); }
    pub fn diagramQuickStyle()->RelationRef{strategy().as_mut().expect("NullPointerException").diagramQuickStyle()}
    // java public static OpcRelation diagramData() { return sRelations.diagramData(); }
    pub fn diagramData()->RelationRef{strategy().as_mut().expect("NullPointerException").diagramData()}
    // java public static OpcRelation diagramLayout() { return sRelations.diagramLayout(); }
    pub fn diagramLayout()->RelationRef{strategy().as_mut().expect("NullPointerException").diagramLayout()}
    // java public static OpcRelation diagramColors() { return sRelations.diagramColors(); }
    pub fn diagramColors()->RelationRef{strategy().as_mut().expect("NullPointerException").diagramColors()}
    // java public static OpcRelation diagramDrawing() { return sRelations.diagramDrawing(); }
    pub fn diagramDrawing()->RelationRef{strategy().as_mut().expect("NullPointerException").diagramDrawing()}
    // java public static OpcRelation oleObjectDocx() { return sRelations.oleObjectDocx(); }
    pub fn oleObjectDocx()->RelationRef{strategy().as_mut().expect("NullPointerException").oleObjectDocx()}
    // java public static OpcRelation oleObjectDoc() { return sRelations.oleObjectDoc(); }
    pub fn oleObjectDoc()->RelationRef{strategy().as_mut().expect("NullPointerException").oleObjectDoc()}
    // java public static OpcRelation oleObjectDocm() { return sRelations.oleObjectDocm(); }
    pub fn oleObjectDocm()->RelationRef{strategy().as_mut().expect("NullPointerException").oleObjectDocm()}
    // java public static OpcRelation imageEmf() { return sRelations.imageEmf(); }
    pub fn imageEmf()->RelationRef{strategy().as_mut().expect("NullPointerException").imageEmf()}
    // java public static OpcRelation imageWmf() { return sRelations.imageWmf(); }
    pub fn imageWmf()->RelationRef{strategy().as_mut().expect("NullPointerException").imageWmf()}
    // java public static OpcRelation imagePict() { return sRelations.imagePict(); }
    pub fn imagePict()->RelationRef{strategy().as_mut().expect("NullPointerException").imagePict()}
    // java public static OpcRelation imageJpeg() { return sRelations.imageJpeg(); }
    pub fn imageJpeg()->RelationRef{strategy().as_mut().expect("NullPointerException").imageJpeg()}
    // java public static OpcRelation imagePng() { return sRelations.imagePng(); }
    pub fn imagePng()->RelationRef{strategy().as_mut().expect("NullPointerException").imagePng()}
    // java public static OpcRelation imageDib() { return sRelations.imageDib(); }
    pub fn imageDib()->RelationRef{strategy().as_mut().expect("NullPointerException").imageDib()}
    // java public static OpcRelation imageGif() { return sRelations.imageGif(); }
    pub fn imageGif()->RelationRef{strategy().as_mut().expect("NullPointerException").imageGif()}
    // java public static OpcRelation imageBmp() { return sRelations.imageBmp(); }
    pub fn imageBmp()->RelationRef{strategy().as_mut().expect("NullPointerException").imageBmp()}
    // java public static OpcRelation wdp() { return sRelations.wdp(); }
    pub fn wdp()->RelationRef{strategy().as_mut().expect("NullPointerException").wdp()}
    // java public static OpcRelation theme() { return sRelations.theme(); }
    pub fn theme()->RelationRef{strategy().as_mut().expect("NullPointerException").theme()}
    // java public static OpcRelation themeOverride() { return sRelations.themeOverride(); }
    pub fn themeOverride()->RelationRef{strategy().as_mut().expect("NullPointerException").themeOverride()}
    // java public static OpcRelation ink() { return sRelations.ink(); }
    pub fn ink()->RelationRef{strategy().as_mut().expect("NullPointerException").ink()}
    // java public static OpcRelation oleObjectBin() { return sRelations.oleObjectBin(); }
    pub fn oleObjectBin()->RelationRef{strategy().as_mut().expect("NullPointerException").oleObjectBin()}
}
