//! 真实语料 **C1**（`samples/ppt-render/flutter/assets/Sample_ppt.pptx`）测试夹具。
//!
//! 设计原则：**整个测试矩阵只从这一份真实 `.pptx` 出发**。
//! - 全量用例直接跑 C1 原始字节；
//! - 分支用例用**真实 zip 字节手术**从 C1 派生变体（删条目 / 改 `ppt/presentation.xml` /
//!   改 `ppt/_rels/presentation.xml.rels` / 注入真实属性），派生物**仍是真实 OPC 包**，
//!   而不是另造一个合成最小包。
use std::cell::RefCell;
use std::io::{Cursor, Read, Write};
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::ooxfr::OxfrPackage::OxfrPackage;
use ppt_core::cn::wps::show::app::event::KmoPresOpenEventListener::KmoPresOpenEventListener;
use ppt_core::cn::wps::show::pptxr::ppt::media::PptxrMedia::PptxrMedia;
use ppt_core::cn::wps::show::pptxr::ppt::PptxrPresentation::PptxrPresentation;
use ppt_core::cn::wps::show::pptxr::util::OpcrPackagePartRef::OpcrPackagePartRef;
use ppt_core::cn::wps::show::pptxr::PptxReader::getPresPart;
use ppt_core::cn::wps::show::util::layout_shape::KmoPresentation;

/// 真实语料 C1（相对 `libs/ppt-core` 的 cargo test CWD）。
pub const C1: &str = "../../samples/ppt-render/flutter/assets/Sample_ppt.pptx";
/// CORPUS 冻结：C1 的字节长度 / 条目数（防语料被替换而不自知）。
pub const C1_LEN: u64 = 5_550_804;
pub const C1_ENTRIES: usize = 159;
/// C1 `ppt/presentation.xml` 的真实 `p:sldId` 条数。
pub const C1_SLIDES: i32 = 23;
/// java `PptxrSlides.UnReadSlides.getCreateSlideCount(pres, 23)` = `min(23, 10)` = 10：
/// `open()` 内 `docDidFinishInitializing` 回调时**热装配**的页数，其余由 `slidesReader.open()` 补齐。
pub const C1_EAGER_SLIDES: i32 = 10;
/// C1 真实的 slide size（`p:sldSz`）。
pub const C1_SLD_CX: i32 = 12_192_000;
pub const C1_SLD_CY: i32 = 6_858_000;
/// C1 真实的 notes size（`p:notesSz`）。
pub const C1_NOTES_CX: i32 = 6_858_000;
pub const C1_NOTES_CY: i32 = 9_144_000;
/// C1 真实的 `ppt/tableStyles.xml` 的 `@def`（`a:tblStyleLst` 内**没有** style 条目 ⇒ count = 0）。
pub const C1_TBL_DEF: &str = "{5C22544A-7EE6-4342-B048-85BDC9FD1C3A}";
/// C1 真实 `ppt/presentation.xml` 上唯一的 presentation 级属性。
pub const C1_PRES_ATTR: (&str, &str) = ("saveSubsetFonts", "1");

const PRES: &str = "ppt/presentation.xml";
const PRES_RELS: &str = "ppt/_rels/presentation.xml.rels";
const TBL: &str = "ppt/tableStyles.xml";
const PRES_PROPS: &str = "ppt/presProps.xml";
const VIEW_PROPS: &str = "ppt/viewProps.xml";

/// FNV-1a 64：仅用于「同一份语料」自证，不引入新依赖。
pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in bytes {
        h ^= *b as u64;
        h = h.wrapping_mul(0x1000_0000_01b3);
    }
    h
}

/// C1 的原始字节 + 语料自证（长度 / 条目数）。
pub fn c1_bytes() -> Vec<u8> {
    let bytes = std::fs::read(C1).expect("read C1 Sample_ppt.pptx");
    assert_eq!(bytes.len() as u64, C1_LEN, "C1 字节长度与 CORPUS 冻结值不一致");
    let mut z = zip::ZipArchive::new(Cursor::new(bytes.clone())).expect("C1 is a real zip");
    assert_eq!(z.len(), C1_ENTRIES, "C1 条目数与 CORPUS 冻结值不一致");
    let _ = (&mut z, fnv1a64(&bytes));
    bytes
}

/// 对 C1 真实字节的一次手术。
pub enum Edit {
    /// 删除一个真实条目
    DropEntry(&'static str),
    /// 用新内容替换一个真实条目
    PutEntry(&'static str, Vec<u8>),
    /// 在真实条目内容上做文本变换（如删 `p:sldSz`、清空 `p:sldIdLst`、注入真实属性）
    Text(&'static str, fn(String) -> String),
    /// 从真实 `ppt/_rels/presentation.xml.rels` 中删除指定 `Type` 结尾的关系
    DropRelType(&'static str),
}

/// 真实 zip 字节手术：读 C1 → 逐条目应用 Edit → 写真实 zip 到临时文件，返回其路径。
///
/// 每个变体一个唯一文件名（同 target 内测试并行执行）。
pub fn variant(tag: &str, edits: &[Edit]) -> String {
    let bytes = c1_bytes();
    let mut zin = zip::ZipArchive::new(Cursor::new(bytes)).expect("C1 zip");
    let dir = std::env::temp_dir().join("m_ppt_pres_c1_variants");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(format!("{}-{}.pptx", std::process::id(), tag));
    let file = std::fs::File::create(&path).unwrap();
    let mut zout = zip::ZipWriter::new(file);

    let drop_entry = |name: &str| edits.iter().any(|e| matches!(e, Edit::DropEntry(n) if *n == name));
    let put = |name: &str| {
        edits.iter().find_map(|e| match e {
            Edit::PutEntry(n, v) if *n == name => Some(v.clone()),
            _ => None,
        })
    };
    let text = |name: &str, body: String| {
        let mut out = body;
        for e in edits {
            match e {
                Edit::Text(n, f) if *n == name => out = f(out),
                Edit::DropRelType(ty) if name == PRES_RELS => out = drop_rel_type(&out, ty),
                _ => {}
            }
        }
        out
    };

    for i in 0..zin.len() {
        let (name, mut buf, method) = {
            let mut e = zin.by_index(i).unwrap();
            let name = e.name().to_string();
            let method = e.compression();
            let mut buf = Vec::new();
            e.read_to_end(&mut buf).unwrap();
            (name, buf, method)
        };
        if drop_entry(&name) {
            continue;
        }
        if let Some(v) = put(&name) {
            buf = v;
        } else if name.ends_with(".xml") || name.ends_with(".rels") {
            let s = String::from_utf8_lossy(&buf).into_owned();
            buf = text(&name, s).into_bytes();
        }
        let opts: zip::write::FileOptions<()> =
            zip::write::FileOptions::default().compression_method(method);
        zout.start_file(name, opts).unwrap();
        zout.write_all(&buf).unwrap();
    }
    zout.finish().unwrap();
    path.to_string_lossy().into_owned()
}

/// 从真实 `presentation.xml.rels` 里按 `Type` 末段删除关系（保留其余真实关系）。
fn drop_rel_type(rels: &str, ty: &str) -> String {
    let needle = format!("/{ty}\"");
    let mut out = String::with_capacity(rels.len());
    let mut rest = rels;
    while let Some(i) = rest.find("<Relationship ") {
        out.push_str(&rest[..i]);
        let after = &rest[i..];
        let end = after.find("/>").map(|e| e + 2).unwrap_or(after.len());
        let node = &after[..end];
        if !node.contains(&needle) {
            out.push_str(node);
        }
        rest = &after[end..];
    }
    out.push_str(rest);
    out
}

/// 删除 `p:presentation` 下的一个真实空元素（如 `<p:sldSz .../>`、`<p:notesSz .../>`）。
pub fn drop_self_closing(xml: String, tag: &str) -> String {
    let open = format!("<{tag}");
    let mut out = String::with_capacity(xml.len());
    let mut rest = xml.as_str();
    while let Some(i) = rest.find(&open) {
        out.push_str(&rest[..i]);
        let after = &rest[i..];
        let end = after.find("/>").map(|e| e + 2).unwrap_or(after.len());
        rest = &after[end..];
    }
    out.push_str(rest);
    out
}

/// 把第一个自闭合元素 `<tag .../>` 替换成给定文本（用于把 C1 的真实尺寸改成非默认值）。
pub fn set_self_closing(xml: String, tag: &str, replacement: &str) -> String {
    let open = format!("<{tag}");
    match xml.find(&open) {
        Some(i) => {
            let after = &xml[i..];
            let end = after.find("/>").map(|e| e + 2).unwrap_or(after.len());
            format!("{}{}{}", &xml[..i], replacement, &after[end..])
        }
        None => xml,
    }
}

/// 覆盖/追加一个元素的属性（保留该元素原有的其它属性与子结构）。
pub fn set_attrs(xml: String, tag: &str, attrs: &str) -> String {
    let open = format!("<{tag}");
    let Some(i) = xml.find(&open) else { return xml };
    let after = &xml[i + open.len()..];
    let Some(gt) = after.find('>') else { return xml };
    let in_tag = &after[..gt];
    let self_closing = in_tag.trim_end().ends_with('/');
    let mut merged = String::new();
    for pair in attrs.split_whitespace() {
        let name = match pair.split('=').next() {
            Some(n) => n,
            None => continue,
        };
        merged.push(' ');
        merged.push_str(pair);
        let _ = name;
    }
    // 去掉同名旧属性（后出现的覆盖前面的语义）
    let mut head = in_tag.to_string();
    for pair in attrs.split_whitespace() {
        let name = pair.split('=').next().unwrap_or("");
        let key = format!("{name}=\"");
        while let Some(p) = head.find(&key) {
            let start = head[..p].rfind(char::is_whitespace).map(|s| s).unwrap_or(p);
            let rest = &head[p..];
            let end = rest.find('"').and_then(|q| rest[q + 1..].find('"')).map(|q| q + q + 2).unwrap_or(rest.len());
            head = format!("{}{}", &head[..start], &rest[end..]);
        }
    }
    let tail = &after[gt..];
    let closing = if self_closing { "" } else { "" };
    format!(
        "{}{}{}{}{}",
        &xml[..i],
        open,
        head.trim_end_matches('/'),
        merged,
        format!("{closing}{tail}")
    )
}

/// 替换 `p:sldIdLst` 的全部内容（保留 `<p:sldIdLst>` 外壳）。
pub fn set_sld_id_lst(xml: String, inner: &str) -> String {
    let start = xml.find("<p:sldIdLst>").expect("C1 has p:sldIdLst");
    let body_start = start + "<p:sldIdLst>".len();
    let end = xml[body_start..].find("</p:sldIdLst>").expect("close") + body_start;
    format!("{}{}{}", &xml[..body_start], inner, &xml[end..])
}

/// `p:presentationPr` / `p:viewPr` 的属性注入（真实 part 内容上手术）。
pub fn inject_attrs(xml: String, tag: &str, attrs: &str) -> String {
    let open = format!("<{tag}");
    match xml.find(&open) {
        Some(i) => {
            let insert_at = i + open.len();
            format!("{}{}{}", &xml[..insert_at], attrs, &xml[insert_at..])
        }
        None => xml,
    }
}

/// 记录真实回调的 listener（Java `KmoPresOpenEventListener` 三个方法）。
#[derive(Default)]
pub struct RecordingListener {
    pub calls: Vec<(bool, i32, i32, i32)>,
    pub slide_done: Vec<i32>,
    pub doc_done: i32,
}

impl KmoPresOpenEventListener for RecordingListener {
    fn docDidFinishInitializing(&mut self, pres: &mut KmoPresentation, hasBlankSlide: bool) {
        self.calls.push((
            hasBlankSlide,
            pres.slideCount(),
            pres.slideWidth(),
            pres.slideHeight(),
        ));
    }
    fn slideDidFinishReadingEverything(&mut self, slideIndex: i32) {
        self.slide_done.push(slideIndex);
    }
    fn docDidFinishReadingEverything(&mut self) {
        self.doc_done += 1;
    }
}

/// 真实装配一次：真实 part → 真实 `PptxrPresentation`。
pub struct Opened {
    pub reader: PptxrPresentation,
    pub pres: Rc<RefCell<KmoPresentation>>,
    pub listener: Rc<RefCell<RecordingListener>>,
}

pub fn open_path(path: &str) -> Opened {
    let mut pack = OxfrPackage::OxfrPackage(path).expect("open real OPC package");
    let part = getPresPart(&mut pack).expect("locate real pres part");
    let pres_part = OpcrPackagePartRef::of(part).expect("real part handle");
    let pres = Rc::new(RefCell::new(KmoPresentation::default()));
    let listener = Rc::new(RefCell::new(RecordingListener::default()));
    let mut reader = PptxrPresentation::PptxrPresentation(
        pres.clone(),
        Some(pres_part),
        Some(listener.clone()),
        None,
        PptxrMedia::PptxrMedia(None, 0),
        0,
    );
    reader.open().expect("real PptxrPresentation.open()");
    Opened {
        reader,
        pres,
        listener,
    }
}

/// 真实 C1 全量装配。
pub fn open_c1() -> Opened {
    open_path(C1)
}

/// 变体装配（真实 zip 手术产物）。
pub fn open_variant(tag: &str, edits: &[Edit]) -> Opened {
    open_path(&variant(tag, edits))
}
