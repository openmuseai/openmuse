#![allow(dead_code, non_snake_case, non_upper_case_globals)]
use std::collections::HashMap;
use std::sync::OnceLock;
// Java: import cn.wps.kfc.xpxml.reader.XmlName2Id
// Java: import cn.wps.kfc.xsd.StringIntMap

// Java field: String URI
pub(crate) const URI_str: &str = "http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing";
// Java field: String PREFIX
pub(crate) const PREFIX_str: &str = "xdr";
// Java field: String from
pub(crate) const from_str: &str = "from";
// Java field: String to
pub(crate) const to_str: &str = "to";
// Java field: String fLocksWithSheet
pub(crate) const fLocksWithSheet_str: &str = "fLocksWithSheet";
// Java field: String fPrintsWithSheet
pub(crate) const fPrintsWithSheet_str: &str = "fPrintsWithSheet";
// Java field: String cNvPr
pub(crate) const cNvPr_str: &str = "cNvPr";
// Java field: String cNvSpPr
pub(crate) const cNvSpPr_str: &str = "cNvSpPr";
// Java field: String nvSpPr
pub(crate) const nvSpPr_str: &str = "nvSpPr";
// Java field: String spPr
pub(crate) const spPr_str: &str = "spPr";
// Java field: String style
pub(crate) const style_str: &str = "style";
// Java field: String txBody
pub(crate) const txBody_str: &str = "txBody";
// Java field: String macro
pub(crate) const r#macro_str: &str = "macro";
// Java field: String textlink
pub(crate) const textlink_str: &str = "textlink";
// Java field: String fLocksText
pub(crate) const fLocksText_str: &str = "fLocksText";
// Java field: String fPublished
pub(crate) const fPublished_str: &str = "fPublished";
// Java field: String cNvCxnSpPr
pub(crate) const cNvCxnSpPr_str: &str = "cNvCxnSpPr";
// Java field: String nvCxnSpPr
pub(crate) const nvCxnSpPr_str: &str = "nvCxnSpPr";
// Java field: String cNvPicPr
pub(crate) const cNvPicPr_str: &str = "cNvPicPr";
// Java field: String nvPicPr
pub(crate) const nvPicPr_str: &str = "nvPicPr";
// Java field: String blipFill
pub(crate) const blipFill_str: &str = "blipFill";
// Java field: String cNvGraphicFramePr
pub(crate) const cNvGraphicFramePr_str: &str = "cNvGraphicFramePr";
// Java field: String nvGraphicFramePr
pub(crate) const nvGraphicFramePr_str: &str = "nvGraphicFramePr";
// Java field: String xfrm
pub(crate) const xfrm_str: &str = "xfrm";
// Java field: String cNvGrpSpPr
pub(crate) const cNvGrpSpPr_str: &str = "cNvGrpSpPr";
// Java field: String nvGrpSpPr
pub(crate) const nvGrpSpPr_str: &str = "nvGrpSpPr";
// Java field: String grpSpPr
pub(crate) const grpSpPr_str: &str = "grpSpPr";
// Java field: String sp
pub(crate) const sp_str: &str = "sp";
// Java field: String grpSp
pub(crate) const grpSp_str: &str = "grpSp";
// Java field: String graphicFrame
pub(crate) const graphicFrame_str: &str = "graphicFrame";
// Java field: String cxnSp
pub(crate) const cxnSp_str: &str = "cxnSp";
// Java field: String pic
pub(crate) const pic_str: &str = "pic";
// Java field: String contentPart
pub(crate) const contentPart_str: &str = "contentPart";
// Java field: String col
pub(crate) const col_str: &str = "col";
// Java field: String colOff
pub(crate) const colOff_str: &str = "colOff";
// Java field: String row
pub(crate) const row_str: &str = "row";
// Java field: String rowOff
pub(crate) const rowOff_str: &str = "rowOff";
// Java field: String clientData
pub(crate) const clientData_str: &str = "clientData";
// Java field: String editAs
pub(crate) const editAs_str: &str = "editAs";
// Java field: String ext
pub(crate) const ext_str: &str = "ext";
// Java field: String pos
pub(crate) const pos_str: &str = "pos";
// Java field: String twoCellAnchor
pub(crate) const twoCellAnchor_str: &str = "twoCellAnchor";
// Java field: String oneCellAnchor
pub(crate) const oneCellAnchor_str: &str = "oneCellAnchor";
// Java field: String absoluteAnchor
pub(crate) const absoluteAnchor_str: &str = "absoluteAnchor";
// Java field: String wsDr
pub(crate) const wsDr_str: &str = "wsDr";
// Java field: String cx
pub(crate) const cx_str: &str = "cx";
// Java field: String cy
pub(crate) const cy_str: &str = "cy";
// Java field: String name
pub(crate) const name_str: &str = "name";
// Java field: String id
pub(crate) const id_str: &str = "id";
// Java field: String txBox
pub(crate) const txBox_str: &str = "txBox";
// Java field: String x
pub(crate) const x_str: &str = "x";
// Java field: String y
pub(crate) const y_str: &str = "y";
// Java field: String anchor
pub(crate) const anchor_str: &str = "anchor";
// Java field: String object
pub(crate) const object_str: &str = "object";

// Java: public class XdrMap implements XmlName2Id
pub struct XdrMap {
    // Java field: XdrMap obj
    // Java field: StringIntMap mapobj
    pub(crate) mapobj: HashMap<i32, i32>,
}
static OBJ: OnceLock<XdrMap> = OnceLock::new();

// Java: public class XdrMap implements XmlName2Id
impl XdrMap {
    // Java: void init()
    fn init(&mut self) {
      // Java: this.mapobj.put("from", 12289);
      self.mapobj.insert(hash_str("from"), 12289);
      // Java: this.mapobj.put("to", 12290);
      self.mapobj.insert(hash_str("to"), 12290);
      // Java: this.mapobj.put("fLocksWithSheet", 12291);
      self.mapobj.insert(hash_str("fLocksWithSheet"), 12291);
      // Java: this.mapobj.put("fPrintsWithSheet", 12292);
      self.mapobj.insert(hash_str("fPrintsWithSheet"), 12292);
      // Java: this.mapobj.put("cNvPr", 3211574);
      self.mapobj.insert(hash_str("cNvPr"), 3211574);
      // Java: this.mapobj.put("cNvSpPr", 3211575);
      self.mapobj.insert(hash_str("cNvSpPr"), 3211575);
      // Java: this.mapobj.put("nvSpPr", 3211577);
      self.mapobj.insert(hash_str("nvSpPr"), 3211577);
      // Java: this.mapobj.put("spPr", 3211578);
      self.mapobj.insert(hash_str("spPr"), 3211578);
      // Java: this.mapobj.put("style", 3211579);
      self.mapobj.insert(hash_str("style"), 3211579);
      // Java: this.mapobj.put("txBody", 3211580);
      self.mapobj.insert(hash_str("txBody"), 3211580);
      // Java: this.mapobj.put("macro", 12299);
      self.mapobj.insert(hash_str("macro"), 12299);
      // Java: this.mapobj.put("textlink", 12300);
      self.mapobj.insert(hash_str("textlink"), 12300);
      // Java: this.mapobj.put("fLocksText", 12301);
      self.mapobj.insert(hash_str("fLocksText"), 12301);
      // Java: this.mapobj.put("fPublished", 12302);
      self.mapobj.insert(hash_str("fPublished"), 12302);
      // Java: this.mapobj.put("cNvCxnSpPr", 3211582);
      self.mapobj.insert(hash_str("cNvCxnSpPr"), 3211582);
      // Java: this.mapobj.put("nvCxnSpPr", 3211583);
      self.mapobj.insert(hash_str("nvCxnSpPr"), 3211583);
      // Java: this.mapobj.put("cNvPicPr", 3211584);
      self.mapobj.insert(hash_str("cNvPicPr"), 3211584);
      // Java: this.mapobj.put("nvPicPr", 3211585);
      self.mapobj.insert(hash_str("nvPicPr"), 3211585);
      // Java: this.mapobj.put("blipFill", 3211586);
      self.mapobj.insert(hash_str("blipFill"), 3211586);
      // Java: this.mapobj.put("cNvGraphicFramePr", 3211587);
      self.mapobj.insert(hash_str("cNvGraphicFramePr"), 3211587);
      // Java: this.mapobj.put("nvGraphicFramePr", 3211588);
      self.mapobj.insert(hash_str("nvGraphicFramePr"), 3211588);
      // Java: this.mapobj.put("xfrm", 3211589);
      self.mapobj.insert(hash_str("xfrm"), 3211589);
      // Java: this.mapobj.put("cNvGrpSpPr", 3211590);
      self.mapobj.insert(hash_str("cNvGrpSpPr"), 3211590);
      // Java: this.mapobj.put("nvGrpSpPr", 3211591);
      self.mapobj.insert(hash_str("nvGrpSpPr"), 3211591);
      // Java: this.mapobj.put("grpSpPr", 3211592);
      self.mapobj.insert(hash_str("grpSpPr"), 3211592);
      // Java: this.mapobj.put("sp", 3211593);
      self.mapobj.insert(hash_str("sp"), 3211593);
      // Java: this.mapobj.put("grpSp", 3211594);
      self.mapobj.insert(hash_str("grpSp"), 3211594);
      // Java: this.mapobj.put("graphicFrame", 3211595);
      self.mapobj.insert(hash_str("graphicFrame"), 3211595);
      // Java: this.mapobj.put("cxnSp", 3211596);
      self.mapobj.insert(hash_str("cxnSp"), 3211596);
      // Java: this.mapobj.put("pic", 3211469);
      self.mapobj.insert(hash_str("pic"), 3211469);
      // Java: this.mapobj.put("contentPart", 3211597);
      self.mapobj.insert(hash_str("contentPart"), 3211597);
      // Java: this.mapobj.put("col", 12320);
      self.mapobj.insert(hash_str("col"), 12320);
      // Java: this.mapobj.put("colOff", 12321);
      self.mapobj.insert(hash_str("colOff"), 12321);
      // Java: this.mapobj.put("row", 12322);
      self.mapobj.insert(hash_str("row"), 12322);
      // Java: this.mapobj.put("rowOff", 12323);
      self.mapobj.insert(hash_str("rowOff"), 12323);
      // Java: this.mapobj.put("clientData", 12324);
      self.mapobj.insert(hash_str("clientData"), 12324);
      // Java: this.mapobj.put("editAs", 12325);
      self.mapobj.insert(hash_str("editAs"), 12325);
      // Java: this.mapobj.put("ext", 12326);
      self.mapobj.insert(hash_str("ext"), 12326);
      // Java: this.mapobj.put("pos", 12327);
      self.mapobj.insert(hash_str("pos"), 12327);
      // Java: this.mapobj.put("twoCellAnchor", 12328);
      self.mapobj.insert(hash_str("twoCellAnchor"), 12328);
      // Java: this.mapobj.put("oneCellAnchor", 12329);
      self.mapobj.insert(hash_str("oneCellAnchor"), 12329);
      // Java: this.mapobj.put("absoluteAnchor", 12330);
      self.mapobj.insert(hash_str("absoluteAnchor"), 12330);
      // Java: this.mapobj.put("wsDr", 12331);
      self.mapobj.insert(hash_str("wsDr"), 12331);
      // Java: this.mapobj.put("cx", 12332);
      self.mapobj.insert(hash_str("cx"), 12332);
      // Java: this.mapobj.put("cy", 12333);
      self.mapobj.insert(hash_str("cy"), 12333);
      // Java: this.mapobj.put("name", 12334);
      self.mapobj.insert(hash_str("name"), 12334);
      // Java: this.mapobj.put("id", 12335);
      self.mapobj.insert(hash_str("id"), 12335);
      // Java: this.mapobj.put("txBox", 3211661);
      self.mapobj.insert(hash_str("txBox"), 3211661);
      // Java: this.mapobj.put("x", 12337);
      self.mapobj.insert(hash_str("x"), 12337);
      // Java: this.mapobj.put("y", 12338);
      self.mapobj.insert(hash_str("y"), 12338);
      // Java: this.mapobj.put("anchor", 12339);
      self.mapobj.insert(hash_str("anchor"), 12339);
      // Java: this.mapobj.put("object", 12340);
      self.mapobj.insert(hash_str("object"), 12340);
    }

    // Java: XdrMap()
    fn new_internal() -> Self {
      // Java: super();
      let mut m = XdrMap {
          mapobj: HashMap::new(),
      };
      // Java: this.init();
      m.init();
      m
    }

    // Java: static XdrMap getInstance()
    pub fn getInstance() -> &'static XdrMap {
      // Java: if (obj == null)
      // Java: obj = new XdrMap();
      // Java: return obj;
      OBJ.get_or_init(|| XdrMap::new_internal())
    }

    // Java: int getId(int prefix, int name)
    pub fn getId(&self, prefix: i32, name: i32) -> i32 {
      // Java: int ret = this.mapobj.get(name);
      let ret = *self.mapobj.get(&name).unwrap_or(&-1);
      // Java: return ret != -1 ? ret : AMap.getInstance().getId(prefix, name);
      if ret != -1 {
          ret
      } else {
          crate::kfc::xsd::dml::AMap::AMap::getInstance().getId(prefix,name)
      }
    }

}

fn hash_str(s: &str) -> i32 {
    let mut h: i32 = 0;
    for c in s.chars() {
        h = h.wrapping_mul(31).wrapping_add(c as i32);
    }
    h
}
