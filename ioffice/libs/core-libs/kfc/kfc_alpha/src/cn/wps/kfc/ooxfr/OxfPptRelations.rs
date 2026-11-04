// Rust rewrite of Java `cn.wps.kfc.ooxfr.OxfPptRelations` + `OxfPptRelationsImpl`.
// Java: the instance table lives behind the `sRelations` strategy (setPptRelation/setClipRelation);
// the part/relationship dictionary below is the Ppt strategy (`OxfPptRelationsImpl`) verbatim.
use std::sync::OnceLock;

use super::super::opcr::OpcRelation::OpcRelation;

// java public class OxfPptRelations
#[derive(Clone, Copy, Debug, Default)]
pub struct OxfPptRelations;

fn rel(content_type: &str, rels_type: &str, part_name: &str) -> OpcRelation {
    OpcRelation::OpcRelation(content_type, rels_type, part_name)
}

// java public static final OpcRelation ITEM = new OpcRelation("application/vnd.openxmlformats-officedocument.customXml+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml", "/customXml/item#.xml");
pub fn ITEM() -> OpcRelation {
    static V: OnceLock<OpcRelation> = OnceLock::new();
    V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.customXml+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml", "/customXml/item#.xml")).clone()
}

// java public static final OpcRelation ITEMPROPS = new OpcRelation("application/vnd.openxmlformats-officedocument.customXmlProperties+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXmlProps", "/customXml/itemProps#.xml");
pub fn ITEMPROPS() -> OpcRelation {
    static V: OnceLock<OpcRelation> = OnceLock::new();
    V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.customXmlProperties+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXmlProps", "/customXml/itemProps#.xml")).clone()
}

impl OxfPptRelations {
    // java public static void setPptRelation()
    pub fn setPptRelation() {}

    // java public static void setClipRelation()
    pub fn setClipRelation() {}

    // java public OpcRelation presentation()
    pub fn presentation() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument", "/ppt/presentation.xml")).clone()
    }

    // java public OpcRelation macrosPresentation()
    pub fn macrosPresentation() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.ms-powerpoint.presentation.macroEnabled.main+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument", "/ppt/presentation.xml")).clone()
    }

    // java public OpcRelation macrosSlideShowPresentation()
    pub fn macrosSlideShowPresentation() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.ms-powerpoint.slideshow.macroEnabled.main+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument", "/ppt/presentation.xml")).clone()
    }

    // java public OpcRelation templatePresentation()
    pub fn templatePresentation() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.presentationml.template.main+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument", "/ppt/presentation.xml")).clone()
    }

    // java public OpcRelation macroTemplatePresentation()
    pub fn macroTemplatePresentation() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.ms-powerpoint.template.macroEnabled.main+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument", "/ppt/presentation.xml")).clone()
    }

    // java public OpcRelation slideShowPresentation()
    pub fn slideShowPresentation() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.presentationml.slideshow.main+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument", "/ppt/presentation.xml")).clone()
    }

    // java public OpcRelation presProps()
    pub fn presProps() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.presentationml.presProps+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/presProps", "/ppt/presProps.xml")).clone()
    }

    // java public OpcRelation tableStyles()
    pub fn tableStyles() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableStyles", "/ppt/tableStyles.xml")).clone()
    }

    // java public OpcRelation theme()
    pub fn theme() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.theme+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme", "/ppt/theme/theme#.xml")).clone()
    }

    // java public OpcRelation themeOverride()
    pub fn themeOverride() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.themeOverride+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/themeOverride", "/ppt/theme/themeOverride#.xml")).clone()
    }

    // java public OpcRelation viewProps()
    pub fn viewProps() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/viewProps", "/ppt/viewProps.xml")).clone()
    }

    // java public OpcRelation comment()
    pub fn comment() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.presentationml.comments+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments", "/ppt/comments/comment#.xml")).clone()
    }

    // java public OpcRelation commentAuthors()
    pub fn commentAuthors() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.presentationml.commentAuthors+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/commentAuthors", "/ppt/commentAuthors.xml")).clone()
    }

    // java public OpcRelation chart()
    pub fn chart() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.drawingml.chart+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart", "/ppt/charts/chart#.xml")).clone()
    }

    // java public OpcRelation chartColor()
    pub fn chartColor() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.ms-office.chartcolorstyle+xml", "http://schemas.microsoft.com/office/2011/relationships/chartColorStyle", "/ppt/charts/colors#.xml")).clone()
    }

    // java public OpcRelation chartStyle()
    pub fn chartStyle() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.ms-office.chartstyle+xml", "http://schemas.microsoft.com/office/2011/relationships/chartStyle", "/ppt/charts/style#.xml")).clone()
    }

    // java public OpcRelation chartSrc()
    pub fn chartSrc() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package", "/ppt/embeddings/Microsoft_Excel____#.xlsx")).clone()
    }

    // java public OpcRelation chartExternalSrc()
    pub fn chartExternalSrc() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.oleObject", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject", "/ppt/embeddings/oleObject#.bin")).clone()
    }

    // java public OpcRelation slide()
    pub fn slide() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.presentationml.slide+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide", "/ppt/slides/slide#.xml")).clone()
    }

    // java public OpcRelation slideLayout()
    pub fn slideLayout() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout", "/ppt/slideLayouts/slideLayout#.xml")).clone()
    }

    // java public OpcRelation slideMaster()
    pub fn slideMaster() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster", "/ppt/slideMasters/slideMaster#.xml")).clone()
    }

    // java public OpcRelation handoutMaster()
    pub fn handoutMaster() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/handoutMaster", "/ppt/handoutMasters/handoutMaster#.xml")).clone()
    }

    // java public OpcRelation notesMaster()
    pub fn notesMaster() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesMaster", "/ppt/notesMasters/notesMaster#.xml")).clone()
    }

    // java public OpcRelation notesSlide()
    pub fn notesSlide() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesSlide", "/ppt/notesSlides/notesSlide#.xml")).clone()
    }

    // java public OpcRelation diagramData()
    pub fn diagramData() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.drawingml.diagramData+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramData", "/ppt/diagrams/data#.xml")).clone()
    }

    // java public OpcRelation diagramLayout()
    pub fn diagramLayout() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramLayout", "/ppt/diagrams/layout#.xml")).clone()
    }

    // java public OpcRelation diagramQuickStyle()
    pub fn diagramQuickStyle() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramQuickStyle", "/ppt/diagrams/quickStyle#.xml")).clone()
    }

    // java public OpcRelation diagramColors()
    pub fn diagramColors() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.drawingml.diagramColors+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramColors", "/ppt/diagrams/colors#.xml")).clone()
    }

    // java public OpcRelation diagramDrawing()
    pub fn diagramDrawing() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.ms-office.drawingml.diagramDrawing+xml", "http://schemas.microsoft.com/office/2007/relationships/diagramDrawing", "/ppt/diagrams/drawing#.xml")).clone()
    }

    // java public OpcRelation vmlDrawings()
    pub fn vmlDrawings() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.vmlDrawing", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing", "/ppt/drawings/vmlDrawing#.vml")).clone()
    }

    // java public OpcRelation model3DGlb()
    pub fn model3DGlb() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("model/gltf.binary", "http://schemas.microsoft.com/office/2017/06/relationships/model3d", "/ppt/media/model3d#.glb")).clone()
    }

    // java public OpcRelation model3DObj()
    pub fn model3DObj() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("model/gltf.binary", "http://schemas.microsoft.com/office/2017/06/relationships/model3d", "/ppt/media/model3d#.obj")).clone()
    }

    // java public OpcRelation audioBin()
    pub fn audioBin() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("audio/unknown", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio", "/ppt/media/audio#.bin")).clone()
    }

    // java public OpcRelation imageEmf()
    pub fn imageEmf() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("image/x-emf", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image", "/ppt/media/image#.emf")).clone()
    }

    // java public OpcRelation imageWmf()
    pub fn imageWmf() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("image/x-wmf", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image", "/ppt/media/image#.wmf")).clone()
    }

    // java public OpcRelation imagePict()
    pub fn imagePict() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("image/pict", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image", "/ppt/media/image#.pict")).clone()
    }

    // java public OpcRelation imageJpeg()
    pub fn imageJpeg() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("image/jpeg", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image", "/ppt/media/image#.jpeg")).clone()
    }

    // java public OpcRelation imagePng()
    pub fn imagePng() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("image/png", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image", "/ppt/media/image#.png")).clone()
    }

    // java public OpcRelation imageBmp()
    pub fn imageBmp() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("image/bmp", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image", "/ppt/media/image#.bmp")).clone()
    }

    // java public OpcRelation imageDib()
    pub fn imageDib() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("image/dib", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image", "/ppt/media/image#.dib")).clone()
    }

    // java public OpcRelation imageGif()
    pub fn imageGif() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("image/gif", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image", "/ppt/media/image#.gif")).clone()
    }

    // java public OpcRelation imageTiff()
    pub fn imageTiff() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("image/tiff", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image", "/ppt/media/image#.tiff")).clone()
    }

    // java public OpcRelation imageWebp()
    pub fn imageWebp() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("image/webp", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image", "/ppt/media/image#.webp")).clone()
    }

    // java public OpcRelation wdp()
    pub fn wdp() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("image/vnd.ms-photo", "http://schemas.microsoft.com/office/2007/relationships/hdphoto", "/ppt/media/hdphoto#.wdp")).clone()
    }

    // java public OpcRelation svg()
    pub fn svg() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("image/svg", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image", "/ppt/media/image#.svg")).clone()
    }

    // java public OpcRelation audioMp3()
    pub fn audioMp3() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("audio/mpeg", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio", "/ppt/media/media#.mp3")).clone()
    }

    // java public OpcRelation audioMidi()
    pub fn audioMidi() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("audio/midi", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio", "/ppt/media/media#.mid")).clone()
    }

    // java public OpcRelation audioMp4()
    pub fn audioMp4() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("audio/mp4", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio", "/ppt/media/media#.m4a")).clone()
    }

    // java public OpcRelation audioWma()
    pub fn audioWma() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("audio/x-ms-wma", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio", "/ppt/media/media#.wma")).clone()
    }

    // java public OpcRelation audioWav()
    pub fn audioWav() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("audio/x-wav", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio", "/ppt/media/media#.wav")).clone()
    }

    // java public OpcRelation audioAac()
    pub fn audioAac() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("audio/aac", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio", "/ppt/media/media#.aac")).clone()
    }

    // java public OpcRelation audioOgg()
    pub fn audioOgg() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("audio/ogg", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio", "/ppt/media/media#.ogg")).clone()
    }

    // java public OpcRelation audioAu()
    pub fn audioAu() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("audio/au", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio", "/ppt/media/media#.au")).clone()
    }

    // java public OpcRelation audioAmr()
    pub fn audioAmr() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("audio/amr", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio", "/ppt/media/media#.amr")).clone()
    }

    // java public OpcRelation audioApe()
    pub fn audioApe() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("audio/ape", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio", "/ppt/media/media#.ape")).clone()
    }

    // java public OpcRelation audioAiff()
    pub fn audioAiff() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("audio/aiff", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio", "/ppt/media/media#.aiff")).clone()
    }

    // java public OpcRelation audio3gpp()
    pub fn audio3gpp() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("audio/3gpp", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio", "/ppt/media/media#.3gpp")).clone()
    }

    // java public OpcRelation audioMmf()
    pub fn audioMmf() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("audio/mmf", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio", "/ppt/media/media#.mmf")).clone()
    }

    // java public OpcRelation audioM4r()
    pub fn audioM4r() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("audio/m4r", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio", "/ppt/media/media#.m4r")).clone()
    }

    // java public OpcRelation audioFlac()
    pub fn audioFlac() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("audio/flac", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio", "/ppt/media/media#.flac")).clone()
    }

    // java public OpcRelation videoMp4()
    pub fn videoMp4() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("video/mp4", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/video", "/ppt/media/media#.mp4")).clone()
    }

    // java public OpcRelation videoAvi()
    pub fn videoAvi() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("video/x-msvideo", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/video", "/ppt/media/media#.avi")).clone()
    }

    // java public OpcRelation video3Gp()
    pub fn video3Gp() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("video/unknown", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/video", "/ppt/media/media#.3gp")).clone()
    }

    // java public OpcRelation video3gpp2()
    pub fn video3gpp2() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("video/3gpp2", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/video", "/ppt/media/media#.3g2")).clone()
    }

    // java public OpcRelation videoAsf()
    pub fn videoAsf() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("video/unknown", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/video", "/ppt/media/media#.asf")).clone()
    }

    // java public OpcRelation videoMpg()
    pub fn videoMpg() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("video/unknown", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/video", "/ppt/media/media#.mpg")).clone()
    }

    // java public OpcRelation videoM2ts()
    pub fn videoM2ts() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("video/unknown", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/video", "/ppt/media/media#.m2ts")).clone()
    }

    // java public OpcRelation videoFlv()
    pub fn videoFlv() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("video/unknown", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/video", "/ppt/media/media#.flv")).clone()
    }

    // java public OpcRelation videoWmv()
    pub fn videoWmv() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("video/x-ms-wmv", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/video", "/ppt/media/media#.wmv")).clone()
    }

    // java public OpcRelation videoSwf()
    pub fn videoSwf() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("video/unknown", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/video", "/ppt/media/media#.swf")).clone()
    }

    // java public OpcRelation videoMov()
    pub fn videoMov() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("video/quicktime", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/video", "/ppt/media/media#.mov")).clone()
    }

    // java public OpcRelation videoRm()
    pub fn videoRm() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("video/unknown", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/video", "/ppt/media/media#.rm")).clone()
    }

    // java public OpcRelation videoRmvb()
    pub fn videoRmvb() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("video/unknown", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/video", "/ppt/media/media#.rmvb")).clone()
    }

    // java public OpcRelation videoMkv()
    pub fn videoMkv() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("video/unknown", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/video", "/ppt/media/media#.mkv")).clone()
    }

    // java public OpcRelation mso07MediaMp3()
    pub fn mso07MediaMp3() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("(String)null", "http://schemas.microsoft.com/office/2007/relationships/media", "/ppt/media/media#.mp3")).clone()
    }

    // java public OpcRelation mso07MediaWma()
    pub fn mso07MediaWma() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("(String)null", "http://schemas.microsoft.com/office/2007/relationships/media", "/ppt/media/media#.wma")).clone()
    }

    // java public OpcRelation mso07MediaWav()
    pub fn mso07MediaWav() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("(String)null", "http://schemas.microsoft.com/office/2007/relationships/media", "/ppt/media/media#.wav")).clone()
    }

    // java public OpcRelation mso07MediaMp4()
    pub fn mso07MediaMp4() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("(String)null", "http://schemas.microsoft.com/office/2007/relationships/media", "/ppt/media/media#.mp4")).clone()
    }

    // java public OpcRelation mso07MediaAvi()
    pub fn mso07MediaAvi() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("(String)null", "http://schemas.microsoft.com/office/2007/relationships/media", "/ppt/media/media#.avi")).clone()
    }

    // java public OpcRelation mso07Media3gp()
    pub fn mso07Media3gp() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("(String)null", "http://schemas.microsoft.com/office/2007/relationships/media", "/ppt/media/media#.3gp")).clone()
    }

    // java public OpcRelation mso07Media3gpp2()
    pub fn mso07Media3gpp2() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("(String)null", "http://schemas.microsoft.com/office/2007/relationships/media", "/ppt/media/media#.3g2")).clone()
    }

    // java public OpcRelation mso07MediaAsf()
    pub fn mso07MediaAsf() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("(String)null", "http://schemas.microsoft.com/office/2007/relationships/media", "/ppt/media/media#.asf")).clone()
    }

    // java public OpcRelation mso07MediaMpg()
    pub fn mso07MediaMpg() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("(String)null", "http://schemas.microsoft.com/office/2007/relationships/media", "/ppt/media/media#.mpg")).clone()
    }

    // java public OpcRelation mso07MediaM2ts()
    pub fn mso07MediaM2ts() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("(String)null", "http://schemas.microsoft.com/office/2007/relationships/media", "/ppt/media/media#.m2ts")).clone()
    }

    // java public OpcRelation mso07MediaFlv()
    pub fn mso07MediaFlv() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("(String)null", "http://schemas.microsoft.com/office/2007/relationships/media", "/ppt/media/media#.flv")).clone()
    }

    // java public OpcRelation mso07MediaWmv()
    pub fn mso07MediaWmv() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("(String)null", "http://schemas.microsoft.com/office/2007/relationships/media", "/ppt/media/media#.wmv")).clone()
    }

    // java public OpcRelation mso07MediaSwf()
    pub fn mso07MediaSwf() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("(String)null", "http://schemas.microsoft.com/office/2007/relationships/media", "/ppt/media/media#.swf")).clone()
    }

    // java public OpcRelation mso07MediaMov()
    pub fn mso07MediaMov() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("(String)null", "http://schemas.microsoft.com/office/2007/relationships/media", "/ppt/media/media#.mov")).clone()
    }

    // java public OpcRelation mso07MediaRm()
    pub fn mso07MediaRm() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("(String)null", "http://schemas.microsoft.com/office/2007/relationships/media", "/ppt/media/media#.rm")).clone()
    }

    // java public OpcRelation mso07MediaRmvb()
    pub fn mso07MediaRmvb() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("(String)null", "http://schemas.microsoft.com/office/2007/relationships/media", "/ppt/media/media#.rmvb")).clone()
    }

    // java public OpcRelation mso07MediaMkv()
    pub fn mso07MediaMkv() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("(String)null", "http://schemas.microsoft.com/office/2007/relationships/media", "/ppt/media/media#.mkv")).clone()
    }

    // java public OpcRelation fontData()
    pub fn fontData() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/x-fontdata", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/font", "/ppt/fonts/font#.fntdata")).clone()
    }

    // java public OpcRelation oleObjectBin()
    pub fn oleObjectBin() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.oleObject", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject", "/ppt/embeddings/oleObject#.bin")).clone()
    }

    // java public OpcRelation oleObjectPpt()
    pub fn oleObjectPpt() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.ms-powerpoint", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject", "/ppt/embeddings/Microsoft_PowerPoint_97-2003_____#.ppt")).clone()
    }

    // java public OpcRelation oleObjectPptx()
    pub fn oleObjectPptx() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.presentationml.presentation", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package", "/ppt/embeddings/Microsoft_PowerPoint_____#.pptx")).clone()
    }

    // java public OpcRelation oleObjectPptm()
    pub fn oleObjectPptm() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.ms-powerpoint.presentation.macroEnabled.12", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package", "/ppt/embeddingsMicrosoft_PowerPoint_Macro-Enabled_Presentation_____#.pptm")).clone()
    }

    // java public OpcRelation oleObjectSldx()
    pub fn oleObjectSldx() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.presentationml.slide", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject", "/ppt/embeddings/Microsoft_PowerPoint____#.sldx")).clone()
    }

    // java public OpcRelation oleObjectXlsx()
    pub fn oleObjectXlsx() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package", "/ppt/embeddings/Microsoft_Office_Excel_2007_Workbook#.xlsx")).clone()
    }

    // java public OpcRelation oleObjectXlsm()
    pub fn oleObjectXlsm() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.ms-excel.sheet.macroEnabled.12", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package", "/ppt/embeddings/Microsoft_Excel_Macro-Enabled_Worksheet#.xlsm")).clone()
    }

    // java public OpcRelation oleObjectXlsb()
    pub fn oleObjectXlsb() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.ms-excel.sheet.binary.macroEnabled.12", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package", "/ppt/embeddings/Microsoft_Office_Excel_Binary_Workbook#.xlsb")).clone()
    }

    // java public OpcRelation oleObjectXls()
    pub fn oleObjectXls() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.ms-excel", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject", "/ppt/embeddings/Microsoft_Excel_97-2003____#.xls")).clone()
    }

    // java public OpcRelation oleObjectDocx()
    pub fn oleObjectDocx() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.wordprocessingml.document", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package", "/ppt/embeddings/Microsoft_Word_Document#.docx")).clone()
    }

    // java public OpcRelation oleObjectDoc()
    pub fn oleObjectDoc() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.wordprocessingml.document", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject", "/ppt/embeddings/Microsoft_Word_Document#.doc")).clone()
    }

    // java public OpcRelation oleObjectDocm()
    pub fn oleObjectDocm() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.ms-word.document.macroEnabled.12", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package", "/ppt/embeddings/Microsoft_Word_Macro-Enabled_Document#.docm")).clone()
    }

    // java public OpcRelation oleObjectDoc97()
    pub fn oleObjectDoc97() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/msword", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject", "/ppt/embeddings/Microsoft_Word_97_-_2003___#.doc")).clone()
    }

    // java public OpcRelation ink()
    pub fn ink() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/inkml+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml", "/ppt/ink/ink#.xml")).clone()
    }

    // java public OpcRelation tags()
    pub fn tags() -> OpcRelation {
        static V: OnceLock<OpcRelation> = OnceLock::new();
        V.get_or_init(|| rel("application/vnd.openxmlformats-officedocument.presentationml.tags+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tags", "/ppt/tags/tag#.xml")).clone()
    }
}

// java name casing aliases kept for existing Rust call sites (Java declares `audio3gpp, macrosPresentation, macrosSlideShowPresentation, mso07Media3gp, mso07Media3gpp2, mso07MediaM2ts, video3gpp2, videoM2ts, vmlDrawings`)
impl OxfPptRelations {
    // java public OpcRelation audio3gpp()
    pub fn audio3Gpp() -> OpcRelation { OxfPptRelations::audio3gpp() }
}
impl OxfPptRelations {
    // java public OpcRelation macrosPresentation()
    pub fn macroPresentation() -> OpcRelation { OxfPptRelations::macrosPresentation() }
}
impl OxfPptRelations {
    // java public OpcRelation macrosSlideShowPresentation()
    pub fn macroSlideShowPresentation() -> OpcRelation { OxfPptRelations::macrosSlideShowPresentation() }
}
impl OxfPptRelations {
    // java public OpcRelation mso07Media3gp()
    pub fn mso07Media3Gp() -> OpcRelation { OxfPptRelations::mso07Media3gp() }
}
impl OxfPptRelations {
    // java public OpcRelation mso07Media3gpp2()
    pub fn mso07Media3Gpp2() -> OpcRelation { OxfPptRelations::mso07Media3gpp2() }
}
impl OxfPptRelations {
    // java public OpcRelation mso07MediaM2ts()
    pub fn mso07MediaM2Ts() -> OpcRelation { OxfPptRelations::mso07MediaM2ts() }
}
impl OxfPptRelations {
    // java public OpcRelation video3gpp2()
    pub fn video3Gpp2() -> OpcRelation { OxfPptRelations::video3gpp2() }
}
impl OxfPptRelations {
    // java public OpcRelation videoM2ts()
    pub fn videoM2Ts() -> OpcRelation { OxfPptRelations::videoM2ts() }
}
impl OxfPptRelations {
    // java public OpcRelation vmlDrawings()
    pub fn vmlDrawing() -> OpcRelation { OxfPptRelations::vmlDrawings() }
}
