// Java: import cn.wps.io.opc.model.POIXMLRelation
use crate::io::opc::model::POIXMLRelation::POIXMLRelation;

// Java: public interface IDmlRelation
pub trait IDmlRelation {
    // Java: abstract POIXMLRelation diagramDrawing()
    fn diagramDrawing(&self) -> POIXMLRelation;

    // Java: abstract POIXMLRelation diagramData()
    fn diagramData(&self) -> POIXMLRelation;

    // Java: abstract POIXMLRelation diagramQuickStyle()
    fn diagramQuickStyle(&self) -> POIXMLRelation;

    // Java: abstract POIXMLRelation chart()
    fn chart(&self) -> POIXMLRelation;

    // Java: abstract POIXMLRelation diagramLayout()
    fn diagramLayout(&self) -> POIXMLRelation;

    // Java: abstract POIXMLRelation diagramColors()
    fn diagramColors(&self) -> POIXMLRelation;

    // Java: abstract POIXMLRelation emf()
    fn emf(&self) -> POIXMLRelation;

    // Java: abstract POIXMLRelation wmf()
    fn wmf(&self) -> POIXMLRelation;

    // Java: abstract POIXMLRelation pict()
    fn pict(&self) -> POIXMLRelation;

    // Java: abstract POIXMLRelation jpeg()
    fn jpeg(&self) -> POIXMLRelation;

    // Java: abstract POIXMLRelation png()
    fn png(&self) -> POIXMLRelation;

    // Java: abstract POIXMLRelation dib()
    fn dib(&self) -> POIXMLRelation;

    // Java: abstract POIXMLRelation gif()
    fn gif(&self) -> POIXMLRelation;

    // Java: abstract POIXMLRelation pcx()
    fn pcx(&self) -> POIXMLRelation;

    // Java: abstract POIXMLRelation psd()
    fn psd(&self) -> POIXMLRelation;

    // Java: abstract POIXMLRelation tga()
    fn tga(&self) -> POIXMLRelation;

    // Java: abstract POIXMLRelation ink()
    fn ink(&self) -> POIXMLRelation;

    // Java: abstract POIXMLRelation tiff()
    fn tiff(&self) -> POIXMLRelation;

    // Java: abstract POIXMLRelation eps()
    fn eps(&self) -> POIXMLRelation;

    // Java: abstract POIXMLRelation bmp()
    fn bmp(&self) -> POIXMLRelation;

    // Java: abstract POIXMLRelation wpg()
    fn wpg(&self) -> POIXMLRelation;

    // Java: abstract POIXMLRelation wdp()
    fn wdp(&self) -> POIXMLRelation;

    // Java: abstract POIXMLRelation svg()
    fn svg(&self) -> POIXMLRelation;

}
