use std::{collections::BTreeMap,sync::{Arc,LazyLock}};
use crate::kfc::{opcr::OpcRelation::OpcRelation,opcw::native::RelationRef,ooxfr::IOxfRelationsBase::IOxfRelationsBase};
// java class ClipRelationsBase implements IOxfRelationsBase {
pub struct ClipRelationsBase{
    // java private OpcRelation mTheme;
    mTheme:Option<Arc<OpcRelation>>,
    // java private OpcRelation mThemeOverride;
    mThemeOverride:Option<Arc<OpcRelation>>,
    // java private OpcRelation mChart;
    mChart:Option<Arc<OpcRelation>>,
    // java private OpcRelation mChartColor;
    mChartColor:Option<Arc<OpcRelation>>,
    // java private OpcRelation mChartStyle;
    mChartStyle:Option<Arc<OpcRelation>>,
    // java private OpcRelation mChartExternalSrc;
    mChartExternalSrc:Option<Arc<OpcRelation>>,
    // java private OpcRelation mDiagramData;
    mDiagramData:Option<Arc<OpcRelation>>,
    // java private OpcRelation mDiagramLayout;
    mDiagramLayout:Option<Arc<OpcRelation>>,
    // java private OpcRelation mDiagramQuickStyle;
    mDiagramQuickStyle:Option<Arc<OpcRelation>>,
    // java private OpcRelation mDiagramColors;
    mDiagramColors:Option<Arc<OpcRelation>>,
    // java private OpcRelation mDiagramDrawing;
    mDiagramDrawing:Option<Arc<OpcRelation>>,
    // java private OpcRelation mImageEmf;
    mImageEmf:Option<Arc<OpcRelation>>,
    // java private OpcRelation mImageWmf;
    mImageWmf:Option<Arc<OpcRelation>>,
    // java private OpcRelation mImagePict;
    mImagePict:Option<Arc<OpcRelation>>,
    // java private OpcRelation mImageJpeg;
    mImageJpeg:Option<Arc<OpcRelation>>,
    // java private OpcRelation mImagePng;
    mImagePng:Option<Arc<OpcRelation>>,
    // java private OpcRelation mImageDib;
    mImageDib:Option<Arc<OpcRelation>>,
    // java private OpcRelation mImageGif;
    mImageGif:Option<Arc<OpcRelation>>,
    // java private OpcRelation mImageBmp;
    mImageBmp:Option<Arc<OpcRelation>>,
    // java private OpcRelation mWdp;
    mWdp:Option<Arc<OpcRelation>>,
    // java private OpcRelation mOleObjectBin;
    mOleObjectBin:Option<Arc<OpcRelation>>,
    // java private OpcRelation mOleObjectDocx;
    mOleObjectDocx:Option<Arc<OpcRelation>>,
    // java private OpcRelation mOleObjectDoc;
    mOleObjectDoc:Option<Arc<OpcRelation>>,
    // java private OpcRelation mOleObjectDocm;
    mOleObjectDocm:Option<Arc<OpcRelation>>,
    // java private OpcRelation mInk;
    mInk:Option<Arc<OpcRelation>>,
    // java private OpcRelation mDrawing;
    mDrawing:Option<Arc<OpcRelation>>,
}
// Native decoding of the original pure constructor literals.
fn literal(name:&str)->OpcRelation{
    static DATA:LazyLock<BTreeMap<String,[Option<String>;3]>>=LazyLock::new(||serde_json::from_str(include_str!("../ooxfr/resources/clip-relations.json")).expect("original clipboard relation literals"));
    let v=&DATA[name];OpcRelation::OpcRelation(v[0].clone(),v[1].clone(),v[2].clone())
}
impl ClipRelationsBase{
    // java ClipRelationsBase() { super(); }
    pub(crate) fn ClipRelationsBase()->Self{Self{mTheme:None,mThemeOverride:None,mChart:None,mChartColor:None,mChartStyle:None,mChartExternalSrc:None,mDiagramData:None,mDiagramLayout:None,mDiagramQuickStyle:None,mDiagramColors:None,mDiagramDrawing:None,mImageEmf:None,mImageWmf:None,mImagePict:None,mImageJpeg:None,mImagePng:None,mImageDib:None,mImageGif:None,mImageBmp:None,mWdp:None,mOleObjectBin:None,mOleObjectDocx:None,mOleObjectDoc:None,mOleObjectDocm:None,mInk:None,mDrawing:None}}
}
impl IOxfRelationsBase for ClipRelationsBase{
    // java public OpcRelation theme() {
    fn theme(&mut self)->RelationRef{
        // java if (null == this.mTheme) { this.mTheme = new OpcRelation("application/vnd.openxmlformats-officedocument.theme+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme", "/clipboard/theme/theme#.xml"); }
        if self.mTheme.is_none(){self.mTheme=Some(Arc::new(literal("mTheme")));}
        // java return this.mTheme;
        RelationRef::Shared(self.mTheme.as_ref().unwrap().clone())
    }
    // java public OpcRelation themeOverride() {
    fn themeOverride(&mut self)->RelationRef{
        // java if (null == this.mThemeOverride) { this.mThemeOverride = new OpcRelation("application/vnd.openxmlformats-officedocument.themeOverride+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/themeOverride", "/clipboard/theme/themeOverride#.xml"); }
        if self.mThemeOverride.is_none(){self.mThemeOverride=Some(Arc::new(literal("mThemeOverride")));}
        // java return this.mThemeOverride;
        RelationRef::Shared(self.mThemeOverride.as_ref().unwrap().clone())
    }
    // java public OpcRelation chart() {
    fn chart(&mut self)->RelationRef{
        // java if (null == this.mChart) { this.mChart = new OpcRelation("application/vnd.openxmlformats-officedocument.drawingml.chart+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart", "/clipboard/charts/chart#.xml"); }
        if self.mChart.is_none(){self.mChart=Some(Arc::new(literal("mChart")));}
        // java return this.mChart;
        RelationRef::Shared(self.mChart.as_ref().unwrap().clone())
    }
    // java public OpcRelation chartColor() {
    fn chartColor(&mut self)->RelationRef{
        // java if (null == this.mChartColor) { this.mChartColor = new OpcRelation("application/vnd.ms-office.chartcolorstyle+xml", "http://schemas.microsoft.com/office/2011/relationships/chartColorStyle", "/clipboard/charts/colors#.xml"); }
        if self.mChartColor.is_none(){self.mChartColor=Some(Arc::new(literal("mChartColor")));}
        // java return this.mChartColor;
        RelationRef::Shared(self.mChartColor.as_ref().unwrap().clone())
    }
    // java public OpcRelation chartStyle() {
    fn chartStyle(&mut self)->RelationRef{
        // java if (null == this.mChartStyle) { this.mChartStyle = new OpcRelation("application/vnd.ms-office.chartstyle+xml", "http://schemas.microsoft.com/office/2011/relationships/chartStyle", "/clipboard/charts/style#.xml"); }
        if self.mChartStyle.is_none(){self.mChartStyle=Some(Arc::new(literal("mChartStyle")));}
        // java return this.mChartStyle;
        RelationRef::Shared(self.mChartStyle.as_ref().unwrap().clone())
    }
    // java public OpcRelation chartExternalSrc() {
    fn chartExternalSrc(&mut self)->RelationRef{
        // java if (null == this.mChartExternalSrc) { this.mChartExternalSrc = new OpcRelation("application/vnd.openxmlformats-officedocument.oleObject", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject", "/clipboard/embeddings/oleObject#.bin"); }
        if self.mChartExternalSrc.is_none(){self.mChartExternalSrc=Some(Arc::new(literal("mChartExternalSrc")));}
        // java return this.mChartExternalSrc;
        RelationRef::Shared(self.mChartExternalSrc.as_ref().unwrap().clone())
    }
    // java public OpcRelation diagramData() {
    fn diagramData(&mut self)->RelationRef{
        // java if (null == this.mDiagramData) { this.mDiagramData = new OpcRelation("application/vnd.openxmlformats-officedocument.drawingml.diagramData+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramData", "/clipboard/diagrams/data#.xml"); }
        if self.mDiagramData.is_none(){self.mDiagramData=Some(Arc::new(literal("mDiagramData")));}
        // java return this.mDiagramData;
        RelationRef::Shared(self.mDiagramData.as_ref().unwrap().clone())
    }
    // java public OpcRelation diagramLayout() {
    fn diagramLayout(&mut self)->RelationRef{
        // java if (null == this.mDiagramLayout) { this.mDiagramLayout = new OpcRelation("application/vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramLayout", "/clipboard/diagrams/layout#.xml"); }
        if self.mDiagramLayout.is_none(){self.mDiagramLayout=Some(Arc::new(literal("mDiagramLayout")));}
        // java return this.mDiagramLayout;
        RelationRef::Shared(self.mDiagramLayout.as_ref().unwrap().clone())
    }
    // java public OpcRelation diagramQuickStyle() {
    fn diagramQuickStyle(&mut self)->RelationRef{
        // java if (null == this.mDiagramQuickStyle) { this.mDiagramQuickStyle = new OpcRelation("application/vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramQuickStyle", "/clipboard/diagrams/quickStyle#.xml"); }
        if self.mDiagramQuickStyle.is_none(){self.mDiagramQuickStyle=Some(Arc::new(literal("mDiagramQuickStyle")));}
        // java return this.mDiagramQuickStyle;
        RelationRef::Shared(self.mDiagramQuickStyle.as_ref().unwrap().clone())
    }
    // java public OpcRelation diagramColors() {
    fn diagramColors(&mut self)->RelationRef{
        // java if (null == this.mDiagramColors) { this.mDiagramColors = new OpcRelation("application/vnd.openxmlformats-officedocument.drawingml.diagramColors+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramColors", "/clipboard/diagrams/colors#.xml"); }
        if self.mDiagramColors.is_none(){self.mDiagramColors=Some(Arc::new(literal("mDiagramColors")));}
        // java return this.mDiagramColors;
        RelationRef::Shared(self.mDiagramColors.as_ref().unwrap().clone())
    }
    // java public OpcRelation diagramDrawing() {
    fn diagramDrawing(&mut self)->RelationRef{
        // java if (null == this.mDiagramDrawing) { this.mDiagramDrawing = new OpcRelation("application/vnd.ms-office.drawingml.diagramDrawing+xml", "http://schemas.microsoft.com/office/2007/relationships/diagramDrawing", "/clipboard/diagrams/drawing#.xml"); }
        if self.mDiagramDrawing.is_none(){self.mDiagramDrawing=Some(Arc::new(literal("mDiagramDrawing")));}
        // java return this.mDiagramDrawing;
        RelationRef::Shared(self.mDiagramDrawing.as_ref().unwrap().clone())
    }
    // java public OpcRelation imageEmf() {
    fn imageEmf(&mut self)->RelationRef{
        // java if (null == this.mImageEmf) { this.mImageEmf = new OpcRelation("image/x-emf", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image", "/clipboard/media/image#.emf"); }
        if self.mImageEmf.is_none(){self.mImageEmf=Some(Arc::new(literal("mImageEmf")));}
        // java return this.mImageEmf;
        RelationRef::Shared(self.mImageEmf.as_ref().unwrap().clone())
    }
    // java public OpcRelation imageWmf() {
    fn imageWmf(&mut self)->RelationRef{
        // java if (null == this.mImageWmf) { this.mImageWmf = new OpcRelation("image/x-wmf", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image", "/clipboard/media/image#.wmf"); }
        if self.mImageWmf.is_none(){self.mImageWmf=Some(Arc::new(literal("mImageWmf")));}
        // java return this.mImageWmf;
        RelationRef::Shared(self.mImageWmf.as_ref().unwrap().clone())
    }
    // java public OpcRelation imagePict() {
    fn imagePict(&mut self)->RelationRef{
        // java if (null == this.mImagePict) { this.mImagePict = new OpcRelation("image/pict", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image", "/clipboard/media/image#.pict"); }
        if self.mImagePict.is_none(){self.mImagePict=Some(Arc::new(literal("mImagePict")));}
        // java return this.mImagePict;
        RelationRef::Shared(self.mImagePict.as_ref().unwrap().clone())
    }
    // java public OpcRelation imageJpeg() {
    fn imageJpeg(&mut self)->RelationRef{
        // java if (null == this.mImageJpeg) { this.mImageJpeg = new OpcRelation("image/jpeg", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image", "/clipboard/media/image#.jpeg"); }
        if self.mImageJpeg.is_none(){self.mImageJpeg=Some(Arc::new(literal("mImageJpeg")));}
        // java return this.mImageJpeg;
        RelationRef::Shared(self.mImageJpeg.as_ref().unwrap().clone())
    }
    // java public OpcRelation imagePng() {
    fn imagePng(&mut self)->RelationRef{
        // java if (null == this.mImagePng) { this.mImagePng = new OpcRelation("image/png", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image", "/clipboard/media/image#.png"); }
        if self.mImagePng.is_none(){self.mImagePng=Some(Arc::new(literal("mImagePng")));}
        // java return this.mImagePng;
        RelationRef::Shared(self.mImagePng.as_ref().unwrap().clone())
    }
    // java public OpcRelation imageDib() {
    fn imageDib(&mut self)->RelationRef{
        // java if (null == this.mImageDib) { this.mImageDib = new OpcRelation("image/dib", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image", "/clipboard/media/image#.dib"); }
        if self.mImageDib.is_none(){self.mImageDib=Some(Arc::new(literal("mImageDib")));}
        // java return this.mImageDib;
        RelationRef::Shared(self.mImageDib.as_ref().unwrap().clone())
    }
    // java public OpcRelation imageGif() {
    fn imageGif(&mut self)->RelationRef{
        // java if (null == this.mImageGif) { this.mImageGif = new OpcRelation("image/gif", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image", "/clipboard/media/image#.gif"); }
        if self.mImageGif.is_none(){self.mImageGif=Some(Arc::new(literal("mImageGif")));}
        // java return this.mImageGif;
        RelationRef::Shared(self.mImageGif.as_ref().unwrap().clone())
    }
    // java public OpcRelation imageBmp() {
    fn imageBmp(&mut self)->RelationRef{
        // java if (null == this.mImageBmp) { this.mImageBmp = new OpcRelation("image/bmp", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image", "/clipboard/media/image#.bmp"); }
        if self.mImageBmp.is_none(){self.mImageBmp=Some(Arc::new(literal("mImageBmp")));}
        // java return this.mImageBmp;
        RelationRef::Shared(self.mImageBmp.as_ref().unwrap().clone())
    }
    // java public OpcRelation wdp() {
    fn wdp(&mut self)->RelationRef{
        // java if (null == this.mWdp) { this.mWdp = new OpcRelation("image/vnd.ms-photo", "http://schemas.microsoft.com/office/2007/relationships/hdphoto", "/clipboard/media/hdphoto#.wdp"); }
        if self.mWdp.is_none(){self.mWdp=Some(Arc::new(literal("mWdp")));}
        // java return this.mWdp;
        RelationRef::Shared(self.mWdp.as_ref().unwrap().clone())
    }
    // java public OpcRelation oleObjectBin() {
    fn oleObjectBin(&mut self)->RelationRef{
        // java if (null == this.mOleObjectBin) { this.mOleObjectBin = new OpcRelation("application/vnd.openxmlformats-officedocument.oleObject", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject", "/clipboard/embeddings/oleObject#.bin"); }
        if self.mOleObjectBin.is_none(){self.mOleObjectBin=Some(Arc::new(literal("mOleObjectBin")));}
        // java return this.mOleObjectBin;
        RelationRef::Shared(self.mOleObjectBin.as_ref().unwrap().clone())
    }
    // java public OpcRelation oleObjectDocx() {
    fn oleObjectDocx(&mut self)->RelationRef{
        // java if (null == this.mOleObjectDocx) { this.mOleObjectDocx = new OpcRelation("application/vnd.openxmlformats-officedocument.wordprocessingml.document", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package", "/clipboard/embeddings/Microsoft_Word_Document#.docx"); }
        if self.mOleObjectDocx.is_none(){self.mOleObjectDocx=Some(Arc::new(literal("mOleObjectDocx")));}
        // java return this.mOleObjectDocx;
        RelationRef::Shared(self.mOleObjectDocx.as_ref().unwrap().clone())
    }
    // java public OpcRelation oleObjectDoc() {
    fn oleObjectDoc(&mut self)->RelationRef{
        // java if (null == this.mOleObjectDoc) { this.mOleObjectDoc = new OpcRelation("application/vnd.openxmlformats-officedocument.wordprocessingml.document", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject", "/clipboard/embeddings/Microsoft_Word_Document#.doc"); }
        if self.mOleObjectDoc.is_none(){self.mOleObjectDoc=Some(Arc::new(literal("mOleObjectDoc")));}
        // java return this.mOleObjectDoc;
        RelationRef::Shared(self.mOleObjectDoc.as_ref().unwrap().clone())
    }
    // java public OpcRelation oleObjectDocm() {
    fn oleObjectDocm(&mut self)->RelationRef{
        // java if (null == this.mOleObjectDocm) { this.mOleObjectDocm = new OpcRelation("application/vnd.ms-word.document.macroEnabled.12", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package", "/clipboard/embeddings/Microsoft_Word_Macro-Enabled_Document#.docm"); }
        if self.mOleObjectDocm.is_none(){self.mOleObjectDocm=Some(Arc::new(literal("mOleObjectDocm")));}
        // java return this.mOleObjectDocm;
        RelationRef::Shared(self.mOleObjectDocm.as_ref().unwrap().clone())
    }
    // java public OpcRelation ink() {
    fn ink(&mut self)->RelationRef{
        // java if (null == this.mInk) { this.mInk = new OpcRelation("application/inkml+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml", "/clipboard/ink/ink#.xml"); }
        if self.mInk.is_none(){self.mInk=Some(Arc::new(literal("mInk")));}
        // java return this.mInk;
        RelationRef::Shared(self.mInk.as_ref().unwrap().clone())
    }
}
impl ClipRelationsBase{
    // java public OpcRelation drawing() {
    pub fn drawing(&mut self)->RelationRef{
        // java if (null == this.mDrawing) { this.mDrawing = new OpcRelation("application/vnd.openxmlformats-officedocument.drawing+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing", "/clipboard/drawings/drawing1.xml"); }
        if self.mDrawing.is_none(){self.mDrawing=Some(Arc::new(literal("mDrawing")));}
        // java return this.mDrawing;
        RelationRef::Shared(self.mDrawing.as_ref().unwrap().clone())
    }
}
