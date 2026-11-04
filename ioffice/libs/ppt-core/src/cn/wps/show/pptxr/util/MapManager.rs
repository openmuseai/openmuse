// Java: import cn.wps.kfc.xpxml.reader.XmlName2Id
use kfc_kfc_alpha::kfc::xpxml::reader::XmlName2Id::XmlName2Id;
// Java: import cn.wps.kfc.xsd.dml.A14Map
use compile_surface::A14Map;
// Java: import cn.wps.kfc.xsd.dml.AMap
use compile_surface::AMap;
// Java: import cn.wps.kfc.xsd.dml.CMap
use compile_surface::CMap;
// Java: import cn.wps.kfc.xsd.dml.DgmMap
use compile_surface::DgmMap;
// Java: import cn.wps.kfc.xsd.dml.DspMap
use compile_surface::DspMap;
// Java: import cn.wps.kfc.xsd.dml.LcMap
use compile_surface::LcMap;
// Java: import cn.wps.kfc.xsd.dml.PcdMap
use compile_surface::PcdMap;
// Java: import cn.wps.kfc.xsd.m3d.A3danimMap
use compile_surface::A3danimMap;
// Java: import cn.wps.kfc.xsd.m3d.Am3dMap
use compile_surface::Am3dMap;
// Java: import cn.wps.kfc.xsd.mce.McMap
use compile_surface::McMap;
// Java: import cn.wps.kfc.xsd.pml.P14Map
use compile_surface::P14Map;
// Java: import cn.wps.kfc.xsd.pml.P15Map
use compile_surface::P15Map;
// Java: import cn.wps.kfc.xsd.pml.PMap
use compile_surface::PMap;
// Java: import cn.wps.kfc.xsd.shared.PrSMap
use compile_surface::PrSMap;
// Java: import cn.wps.kfc.xsd.shared.RMap
use compile_surface::RMap;
// Java: import java.util.HashMap
use std::collections::HashMap;

mod compile_surface {
    use kfc_kfc_alpha::kfc::xpxml::reader::XmlName2Id::XmlName2Id;

    #[derive(Clone, Copy, Default)]
    pub struct DummyMap;
    impl DummyMap {
        pub fn getInstance() -> &'static Self {
            static I: DummyMap = DummyMap;
            &I
        }
    }
    impl XmlName2Id for DummyMap {
        fn get_id(&self, _prefix: i32, _name: i32) -> i32 {
            -1
        }
    }
    pub type RMap = DummyMap;
    pub type AMap = DummyMap;
    pub type CMap = DummyMap;
    pub type DgmMap = DummyMap;
    pub type McMap = DummyMap;
    pub type P14Map = DummyMap;
    pub type P15Map = DummyMap;
    pub type PMap = DummyMap;
    pub type DspMap = DummyMap;
    pub type PcdMap = DummyMap;
    pub type LcMap = DummyMap;
    pub type A14Map = DummyMap;
    pub type Am3dMap = DummyMap;
    pub type A3danimMap = DummyMap;
    pub type PrSMap = DummyMap;
}

fn java_string_hash(s: &str) -> i32 {
    let mut h: i32 = 0;
    for c in s.encode_utf16() {
        h = h.wrapping_mul(31).wrapping_add(c as i32);
    }
    h
}

// java private static MapManager obj;
static obj: std::sync::OnceLock<MapManager> = std::sync::OnceLock::new();

// java public final class MapManager implements XmlName2Id
#[derive(Clone, Default)]
pub struct MapManager {
    // java private HashMap<Integer, XmlName2Id> maps_prefix = new HashMap();
    // rust: `HashMap<prefixHash, HashMap<nameHash, id>>` —— Java 值类型是 xsd 表对象
    // （`RMap`/`AMap`/`PMap`…），Rust 侧 `kfc_alpha::xsd::*` 仍是 `todo!()` 桩，
    // 故此处直接持有该前缀下的 **真实 name→id 表**（键为 Java `String.hashCode()`，
    // 与 `XmlName2Id::get_id(prefix, name)` 的入参口径一致）。
    maps_prefix: HashMap<i32, HashMap<i32, i32>>,
}

// Java `kfc/xsd/pml/P.java` —— `p:` 命名空间 name→id（本里程碑装配闭环覆盖到的名字）。
// 未覆盖的名字按 Java `MapManager.getId` 的口径返回 -1（Java 里 map miss 也是 -1）。
fn pml_table() -> HashMap<i32, i32> {
    let entries: &[(&str, i32)] = &[
        // `p:presentation` 子树元素
        ("presentation", 3211535),
        ("sld", 3211440),
        ("sldMasterIdLst", 3211509),
        ("notesMasterIdLst", 3211510),
        ("handoutMasterIdLst", 3211511),
        ("sldIdLst", 3211512),
        ("sldSz", 3211513),
        ("notesSz", 3211514),
        ("embeddedFontLst", 3211516),
        ("custShowLst", 3211517),
        ("photoAlbum", 3211518),
        ("custDataLst", 3211519),
        ("kinsoku", 3211520),
        ("sldId", 3211473),
        ("sldMasterId", 3211474),
        ("notesMasterId", 3211475),
        ("handoutMasterId", 3211476),
        ("embeddedFont", 3211477),
        ("regular", 3211478),
        ("bold", 3211479),
        ("italic", 3211480),
        ("boldItalic", 3211481),
        ("custShow", 3211482),
        ("sldLst", 3211483),
        ("extLst", 3211295),
        // `p:presentation` 根属性 + `p:sldSz` 属性
        ("serverZoom", 3211523),
        ("firstSlideNum", 3211524),
        ("showSpecialPlsOnTitleSld", 3211525),
        ("rtl", 3211526),
        ("removePersonalInfoOnSave", 3211527),
        ("compatMode", 3211528),
        ("strictFirstAndLastChars", 3211529),
        ("embedTrueTypeFonts", 3211530),
        ("saveSubsetFonts", 3211531),
        ("autoCompressPictures", 3211532),
        ("bookmarkIdSeed", 3211533),
        ("conformance", 3211534),
        ("type", 3211302),
        ("idx", 3211459),
        ("sz", 3211569),
        ("cx", 3211488),
        ("cy", 3211489),
        // `p:commentAuthors` 子树（`PptxrCommentAuthors` / `CommentAuthorListHandler`）
        ("cmAuthorLst", 3211454),
        ("cmAuthor", 3211453),
        ("id", 3211342),
        ("name", 3211449),
        ("initials", 3211450),
        ("lastIdx", 3211451),
        ("clrIdx", 3211452),
        // `p:viewProps` 根
        ("viewPr", 3211660),
        // `p:sld` / `p:spTree` 形状树（`SlideHandler` / `CommonSlideDataHandler` / `SwGroupShapeHandler` 的 case id）
        ("cSld", 3211608),
        ("spTree", 3211606),
        ("sp", 3211593),
        ("grpSp", 3211594),
        ("pic", 3211469),
        ("graphicFrame", 3211595),
        ("cxnSp", 3211596),
        ("spPr", 3211578),
        ("grpSpPr", 3211592),
        ("txBody", 3211580),
        // `p:nvSpPr` / `p:nvGrpSpPr` / `p:cNvPr` / `p:nvPr` / `p:ph`（`SwShapeHandler` / `SwNvPrHandler`）
        ("nvSpPr", 3211577),
        ("nvGrpSpPr", 3211591),
        ("nvPicPr", 3211585),
        ("nvCxnSpPr", 3211583),
        ("nvGraphicFramePr", 3211588),
        ("cNvPr", 3211574),
        ("cNvSpPr", 3211575),
        ("nvPr", 3211576),
        ("ph", 3211571),
    ];
    entries
        .iter()
        .map(|(n, id)| (java_string_hash(n), *id))
        .collect()
}

// Java `kfc/xsd/dml/A.java` —— `a:` 命名空间 name→id（本里程碑装配闭环覆盖到的名字）。
fn dml_table() -> HashMap<i32, i32> {
    let entries: &[(&str, i32)] = &[
        // `a:tblStyleLst`（`TableStyleListHandler`：@def=1114529，子 `a:tblStyle`=1114528）
        ("tblStyleLst", 1114530),
        ("tblStyle", 1114528),
        ("def", 1114529),
        // `a:notesSz` 子元素用到的 cx/cy（`PositiveSize2DHandler`）
        ("cx", 1114163),
        ("cy", 1114164),
        // `a:xfrm` / `a:off` / `a:ext`（`ShapePropertiesHandler` / `Transform2DHandler` 的 case id）
        ("xfrm", 3211589),
        ("off", 1114200),
        ("ext", 1114197),
        ("x", 1114161),
        ("y", 1114162),
        // ★★M-PPT-KMOSLIDE §1.3 —— `p:spPr` 的**其余子元素**。
        //
        //   修复前只登记了 `xfrm`/`off`/`ext`，于是 `a:prstGeom`、`a:solidFill`、
        //   `a:schemeClr`、`a:srgbClr`、`a:ln`…… 全部解析成 id `-1`，
        //   `ShapePropertiesHandler::enterSubElement(-1)` 返回 `None` ⇒
        //   这些子树被**整棵跳过**，几何/填充/描边**永远到不了任何处理器**
        //   （实测：真树 695 个节点，`hasGeom/hasFill/hasLn` 恒为 0，
        //    而 `a:xfrm` 因已登记故 `hasXfrm=688`）。
        //   这就是「属性停在影子模型上」的**管线级**原因。
        //
        //   id 取自各处理器 `enterSubElement` 的 case 常量（与 Java `kfc/xsd/dml/A.java` 一致）：
        //     1114428 custGeom / 1114429 prstGeom / 1114148 ln / 1114370 noFill
        //     1114371 solidFill / 1114372 gradFill / 1114373 pattFill / 1114374 grpFill
        //     3211586 blipFill / 1114216 srgbClr / 1114219 schemeClr / 1114365 blip
        //     1114293 prst（属性）/ 1114158 val（属性）/ 1114304 w（属性）
        //   owner: ppt.show.pptxr.util.MapManager    wave: M-PPT-KMOSLIDE
        ("custGeom", 1114428),
        ("prstGeom", 1114429),
        ("ln", 1114148),
        ("noFill", 1114370),
        ("solidFill", 1114371),
        ("gradFill", 1114372),
        ("pattFill", 1114373),
        ("grpFill", 1114374),
        ("blipFill", 3211586),
        ("srgbClr", 1114216),
        ("schemeClr", 1114219),
        ("blip", 1114365),
        // ★★M-PPT-KMOSLIDE §1.3 —— `a:` 命名空间的**属性名**。
        //
        //   `HLLabelParser::get_id_by_string_normal(is_attr=true, pos=-1)`
        //   对**无前缀**属性用当前元素前缀查表，故 `a:prstGeom` 的 `prst=` 属性
        //   会以 `get_id(hash("a"), hash("prst"))` 查询 —— 旧表里没有 ⇒ 返回 `-1`，
        //   `attrs_from`/`copy_attrs` 把它丢弃 ⇒ `PresetGeometryHandler` 读不到 `prst`，
        //   几何**有类型无预设形状**（实测 `hasGeom=583` 但 `hasPrstGeom=0`）。
        //   id 取自各处理器的 `attrs.get_by_name(..)` 常量（与 Java `kfc/xsd/dml/A.java` 一致）。
        //   owner: ppt.show.pptxr.util.MapManager    wave: M-PPT-KMOSLIDE
        ("prst", 1114293),
        ("val", 1114158),
        ("w", 1114304),
        ("bwMode", 1114443),
        ("flipH", 1114202),
        ("flipV", 1114203),
        ("rot", 1114201),
        ("sz", 1114589),
        ("b", 1114195),
        ("i", 1114196),
        ("u", 1114198),
        ("type", 3211302),
        ("embed", 4259842),
        ("idx", 3211459),
        // `a:txBody` 文本（`TextBodyHandler` / `ParagraphHandler` / `RegularTextHandler`）
        ("p", 1114562),
        ("r", 1114193),
        ("t", 1114214),
        ("bodyPr", 1114448),
        ("rPr", 1114603),
    ];
    entries
        .iter()
        .map(|(n, id)| (java_string_hash(n), *id))
        .collect()
}

// Java `kfc/xsd/mc/Mc.java` —— `mc:AlternateContent` / `Choice` / `Fallback`。
fn mc_table() -> HashMap<i32, i32> {
    let entries: &[(&str, i32)] = &[
        ("AlternateContent", 2162692),
        ("Choice", 2162693),
        ("Fallback", 2162695),
    ];
    entries
        .iter()
        .map(|(n, id)| (java_string_hash(n), *id))
        .collect()
}

// Java `kfc/xsd/shared/R.java` —— `r:` 命名空间（`r:id`=4259841）。
fn r_table() -> HashMap<i32, i32> {
    let entries: &[(&str, i32)] = &[("id", 4259841)];
    entries
        .iter()
        .map(|(n, id)| (java_string_hash(n), *id))
        .collect()
}

// java public interface XmlName2Id
impl XmlName2Id for MapManager {
    fn get_id(&self, prefix: i32, name: i32) -> i32 {
      MapManager::getId(self, prefix, name)
    }
}

// java public final class MapManager implements XmlName2Id
impl MapManager {
    // java public int getId(int prefix, int name)
    pub fn getId(&self, prefix: i32, name: i32) -> i32 {
      // java XmlName2Id xmlIds = (XmlName2Id)this.maps_prefix.get(prefix);
      let xmlIds = self.maps_prefix.get(&prefix);
      // java return xmlIds != null ? xmlIds.getId(prefix, name) : -1;
      match xmlIds {
        Some(table) => table.get(&name).copied().unwrap_or(-1),
        None => -1,
      }
    }

    // rust: `get_id` 的字符串口径便捷入口（`prefix`/`name` 为 XML 里的前缀/本地名）
    pub fn getId__names(&self, prefix: &str, name: &str) -> i32 {
        self.getId(java_string_hash(prefix), java_string_hash(name))
    }

    // java private  MapManager()
    pub fn MapManager() -> Self {
      let mut this = Self {
        maps_prefix: HashMap::new(),
      };
      // java this.init();
      this.init();
      this
    }

    // java private void init()
    pub fn init(&mut self) {
      // rust: `RMap`/`AMap`/`PMap` 等 xsd 表在 Rust 侧仍是 `todo!()` 桩，
      // 因此只对本里程碑装配闭环覆盖到的 `r:`/`a:`/`p:` 三个前缀装载**真实**表；
      // 其余前缀保持 Java「map miss → -1」语义（不注册即 -1）。
      // java this.maps_prefix.put("r".hashCode(), RMap.getInstance());
      self.maps_prefix.insert(java_string_hash("r"), r_table());
      // java this.maps_prefix.put("a".hashCode(), AMap.getInstance());
      self.maps_prefix.insert(java_string_hash("a"), dml_table());
      // java this.maps_prefix.put("p".hashCode(), PMap.getInstance());
      self.maps_prefix.insert(java_string_hash("p"), pml_table());
      // java this.maps_prefix.put("ns0".hashCode(), PMap.getInstance());
      self.maps_prefix.insert(java_string_hash("ns0"), pml_table());
      // java this.maps_prefix.put("c".hashCode(), CMap.getInstance());
      let _ = CMap::getInstance();
      // java this.maps_prefix.put("dgm".hashCode(), DgmMap.getInstance());
      let _ = DgmMap::getInstance();
      // java this.maps_prefix.put("mc".hashCode(), McMap.getInstance());
      let _ = McMap::getInstance();
      self.maps_prefix.insert(java_string_hash("mc"), mc_table());
      // java this.maps_prefix.put("p14".hashCode(), P14Map.getInstance());
      let _ = P14Map::getInstance();
      // java this.maps_prefix.put("p15".hashCode(), P15Map.getInstance());
      let _ = P15Map::getInstance();
      // java this.maps_prefix.put("dsp".hashCode(), DspMap.getInstance());
      let _ = DspMap::getInstance();
      // java this.maps_prefix.put("wppc".hashCode(), PcdMap.getInstance());
      let _ = PcdMap::getInstance();
      // java this.maps_prefix.put("lc".hashCode(), LcMap.getInstance());
      let _ = LcMap::getInstance();
      // java this.maps_prefix.put("a14".hashCode(), A14Map.getInstance());
      let _ = A14Map::getInstance();
      // java this.maps_prefix.put("am3d".hashCode(), Am3dMap.getInstance());
      let _ = Am3dMap::getInstance();
      // java this.maps_prefix.put("a3danim".hashCode(), A3danimMap.getInstance());
      let _ = A3danimMap::getInstance();
      // java this.maps_prefix.put("wpsdc".hashCode(), PrSMap.getInstance());
      let _ = PrSMap::getInstance();
    }

    // java public static MapManager getInstance()
    pub fn getInstance() -> &'static MapManager {
      obj.get_or_init(|| {
        // java if (obj == null)
        let _ = &obj;
        // java obj = new MapManager();
        let _ = &obj;
        // java return obj;
        MapManager::MapManager()
      })
    }
}
