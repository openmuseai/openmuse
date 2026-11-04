use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{Cursor, Read};
use std::path::Path;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
use quick_xml::events::Event;
use quick_xml::Reader;
use zip::ZipArchive;

use crate::cn::wps::drawing::dmlr::handler::shape::sppr::fill::BlipFillHandler::BlipFillHandler;
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::fill::FillNotify::FillNotify;
use crate::cn::wps::drawing::data::shape::ShapePr::ShapePr as DataShapePr;
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::ShapePropertiesHandler::{
    GroupShapePropertiesHandler, ShapePropertiesHandler,
};
use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::text_body_surf::{
    TextBodyHandler, TextBodySurf,
};
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
use crate::cn::wps::drawing::dmlr::util::DGMap::DGMap;
use crate::cn::wps::kfc::ooxfr::OxfrPackage::OxfrPackage;
use crate::cn::wps::show::pptxr::util::SWReaderMap::SWReaderMap;
use crate::cn::wps::show::util::layout_shape::{
    GroupShape, KmoShapeTree, KmoShowShape, KmoSlide, KmoSlideLayout, KmoSlideMaster, Placeholder,
    ShapePr,
};
use crate::cn::wps::show::util::layout_textbox::KmoTextOperator;
use crate::cn::wps::drawing::app::DgStore::DgStore;
use crate::cn::wps::drawing::dmlr::model::HandlerCallback::{
    HandlerCallback, HandlerCallbackState,
};
// rust: ★③-5 去假面（形态 F）—— `OpenCb` 的 `ShapePr`/`ShapeStyle` 关联类型原先绑定到
//   `sw_shape_hosts` 的**同名空壳**（`ShapePr;` / `ShapeStyle;` 单元结构，无字段无方法），
//   导致真 handler（`SwShapeHandler`/`SwPictureHandler`/…）无法以 `OpenCb` 实例化（E0271）。
//   现改为**真身** `drawing::data::{shape::ShapePr, style::ShapeStyle}`，与各 handler 的
//   `HandlerCallback<ShapePr = …, ShapeStyle = …>` 约束一致。
//   owner: ppt.show.pptxr.pptx_mvp    wave: M-PPT-SHAPE-DISPATCH
use crate::cn::wps::drawing::data::shape::ShapePr::ShapePr as HostShapePr;
use crate::cn::wps::drawing::data::style::ShapeStyle::ShapeStyle as HostShapeStyle;
use crate::cn::wps::show::app::slide::ISlide::{
    Background, ColorMapping, ISlide, KmoPresentation, KmoSlideEventListeners, KmoTheme, KsoWMInfo,
};
use crate::cn::wps::show::app::slide::KmoFilm::KmoFilm;
use crate::cn::wps::show::pptxr::handler::slides::CommonSlideDataHandler::{
    snapshot_kmo, CommonSlideDataHandler,
};
use crate::cn::wps::show::pptxr::handler::shape::SwGroupShapeHandler::SwGroupShapeHandler;

pub fn element_id(local: &str) -> i32 {
    match local {
        "xfrm" => 3211589,
        "off" => 1114200,
        "ext" => 1114197,
        "chOff" => 1114204,
        "chExt" => 1114205,
        "ln" => 1114148,
        "noFill" => 1114370,
        "solidFill" => 1114371,
        "gradFill" => 1114372,
        "pattFill" => 1114373,
        "grpFill" => 1114374,
        "blipFill" => 3211586,
        "prstGeom" => 1114429,
        "custGeom" => 1114428,
        "srgbClr" => 1114216,
        "schemeClr" => 1114219,
        "blip" => 1114365,
        "bodyPr" => 1114448,
        "lstStyle" => 1114449,
        "p" => 1114562,
        "r" => 1114193,
        "t" => 1114214,
        "rPr" => 1114603,
        _ => -1,
    }
}

pub fn attr_id(local: &str) -> i32 {
    match local {
        "bwMode" => 1114443,
        "flipH" => 1114202,
        "flipV" => 1114203,
        "rot" => 1114201,
        "x" => 1114161,
        "y" => 1114162,
        "cx" => 1114163,
        "cy" => 1114164,
        "val" => 1114158,
        "sz" => 1114589,
        "b" => 1114195,
        "embed" => 4259842,
        "idx" => 3211459,
        "type" => 3211302,
        "prst" => 1114293,
        "vert" => 1114480,
        "w" => 1114304,
        "showMasterSp" => 3211600,
        _ => -1,
    }
}

fn local_name(q: &[u8]) -> String {
    let s = String::from_utf8_lossy(q);
    match s.rsplit_once(':') {
        Some((_, l)) => l.to_string(),
        None => s.into_owned(),
    }
}

fn first_element_attrs(xml: &str, local: &str) -> XmlRoAttrList {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                if local_name(e.name().as_ref()) == local {
                    return attrs_from(&e);
                }
            }
            Ok(Event::Empty(e)) => {
                if local_name(e.name().as_ref()) == local {
                    return attrs_from(&e);
                }
            }
            Ok(Event::Eof) | Err(_) => break,
            _ => {}
        }
        buf.clear();
    }
    XmlRoAttrList::new()
}

fn apply_show_master_sp_attrs(attrs: &XmlRoAttrList) -> Option<bool> {
    let count = attrs.count();
    let mut i = 0;
    while i < count {
        // Java SlideHandler/SlideLayoutHandler.startElement:
        // if ((tmp = attrsNotCopy.getAttrAt(i)) != null) switch (attrName) case 3211600
        let tmp = attrs.get_attr_at(i as usize);
        if tmp.get_name() == 3211600 {
            // Java: this.slideData.putShowMasterSp(tmp.toBoolean());
            return Some(tmp.to_boolean());
        }
        i += 1;
    }
    None
}

/// Java `SlideHandler.startElement` case 3211600 + `Slide.isShowMasterSp` (缺省 true).
fn parse_slide_show_master_sp(xml: &str) -> bool {
    let attrs = first_element_attrs(xml, "sld");
    // Java: if (!this.map.has(3)) return this.parent == null ? true : this.parent.isShowMasterSp();
    apply_show_master_sp_attrs(&attrs).unwrap_or(true)
}

/// Java `SlideLayoutHandler.startElement` case 3211600 + `SlideLayout.isShowMasterSp` (缺省 true).
fn parse_layout_show_master_sp(xml: &str) -> bool {
    let attrs = first_element_attrs(xml, "sldLayout");
    apply_show_master_sp_attrs(&attrs).unwrap_or(true)
}

fn attrs_from(e: &quick_xml::events::BytesStart<'_>) -> XmlRoAttrList {
    let mut list = XmlRoAttrList::new();
    for a in e.attributes().flatten() {
        let key = local_name(a.key.as_ref());
        let id = attr_id(&key);
        if id >= 0 {
            let val = a.unescape_value().unwrap_or_default();
            list.add_attr(id, &val);
        }
    }
    list
}

pub fn walk_xml(xml: &str, root: &mut dyn ElementHandler) {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(false);
    let mut buf = Vec::new();
    let mut stack: Vec<Box<dyn ElementHandler>> = Vec::new();
    let mut first = true;
    let mut skip = 0u32;
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Empty(e)) => {
                if skip > 0 {
                    buf.clear();
                    continue;
                }
                let name = local_name(e.name().as_ref());
                let id = element_id(&name);
                let attrs = attrs_from(&e);
                if first {
                    first = false;
                    root.startElement(id, &attrs);
                    root.endElement(id);
                } else {
                    let parent: &mut dyn ElementHandler = match stack.last_mut() {
                        Some(h) => &mut **h,
                        None => root,
                    };
                    if let Some(mut child) = parent.enterSubElement(id) {
                        child.startElement(id, &attrs);
                        child.endElement(id);
                    }
                }
            }
            Ok(Event::Start(e)) => {
                if skip > 0 {
                    skip += 1;
                    buf.clear();
                    continue;
                }
                let name = local_name(e.name().as_ref());
                let id = element_id(&name);
                let attrs = attrs_from(&e);
                if first {
                    first = false;
                    root.startElement(id, &attrs);
                } else {
                    let parent: &mut dyn ElementHandler = match stack.last_mut() {
                        Some(h) => &mut **h,
                        None => root,
                    };
                    if let Some(mut child) = parent.enterSubElement(id) {
                        child.startElement(id, &attrs);
                        stack.push(child);
                    } else {
                        skip = 1;
                    }
                }
            }
            Ok(Event::Text(t)) => {
                if skip == 0 {
                    let s = t.unescape().unwrap_or_default();
                    if let Some(h) = stack.last_mut() {
                        h.characters(&s);
                    }
                }
            }
            Ok(Event::End(_)) => {
                if skip > 0 {
                    skip -= 1;
                } else if let Some(mut h) = stack.pop() {
                    h.endElement(0);
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }
    root.endElement(0);
}

fn parse_quoted_i32(xml: &str, key: &str) -> Option<i32> {
    let pat = format!("{key}=\"");
    let i = xml.find(&pat)?;
    let rest = &xml[i + pat.len()..];
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    digits.parse().ok()
}

fn utf8_window(xml: &str, start: usize, max_bytes: usize) -> &str {
    if start >= xml.len() || !xml.is_char_boundary(start) {
        return "";
    }
    let mut end = (start + max_bytes).min(xml.len());
    while end > start && !xml.is_char_boundary(end) {
        end -= 1;
    }
    &xml[start..end]
}

fn parse_sld_sz(xml: &str) -> (i32, i32) {
    let mut w = 12_192_000;
    let mut h = 6_858_000;
    if let Some(i) = xml.find("sldSz") {
        let slice = utf8_window(xml, i, 400);
        if let Some(v) = parse_quoted_i32(slice, "cx") {
            w = v;
        }
        if let Some(v) = parse_quoted_i32(slice, "cy") {
            h = v;
        }
    }
    (w, h)
}

fn zip_read(z: &mut ZipArchive<Cursor<Vec<u8>>>, name: &str) -> Option<String> {
    let mut f = z.by_name(name).ok()?;
    let mut raw = Vec::new();
    f.read_to_end(&mut raw).ok()?;
    if let Ok(s) = String::from_utf8(raw.clone()) {
        return Some(s);
    }
    Some(String::from_utf8_lossy(&raw).into_owned())
}

fn zip_read_bytes(z: &mut ZipArchive<Cursor<Vec<u8>>>, name: &str) -> Option<Vec<u8>> {
    let mut f = z.by_name(name).ok()?;
    let mut raw = Vec::new();
    f.read_to_end(&mut raw).ok()?;
    Some(raw)
}

fn slide_xml_names(names: &[String]) -> Vec<String> {
    let mut slides: Vec<(u32, String)> = names
        .iter()
        .filter(|n| {
            n.starts_with("ppt/slides/slide")
                && n.ends_with(".xml")
                && !n.contains("/_rels/")
        })
        .filter_map(|n| {
            let stem = n.rsplit('/').next()?;
            let num = stem
                .strip_prefix("slide")?
                .strip_suffix(".xml")?
                .parse::<u32>()
                .ok()?;
            Some((num, n.clone()))
        })
        .collect();
    slides.sort_by_key(|(n, _)| *n);
    slides.into_iter().map(|(_, n)| n).collect()
}

fn presentation_slide_rids(xml: &str) -> Vec<String> {
    let mut reader = Reader::from_str(xml);
    let mut buffer = Vec::new();
    let mut result = Vec::new();
    loop {
        match reader.read_event_into(&mut buffer) {
            Ok(Event::Start(element)) | Ok(Event::Empty(element))
                if local_name(element.name().as_ref()) == "sldId" =>
            {
                for attribute in element.attributes().flatten() {
                    let key = String::from_utf8_lossy(attribute.key.as_ref());
                    if key == "r:id" || key.ends_with(":id") {
                        if let Ok(value) = attribute.unescape_value() {
                            result.push(value.into_owned());
                        }
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buffer.clear();
    }
    result
}

fn collapse_zip_path(path: &str) -> String {
    let mut components: Vec<&str> = Vec::new();
    for component in path.split('/') {
        match component {
            "" | "." => {}
            ".." => {
                components.pop();
            }
            value => components.push(value),
        }
    }
    components.join("/")
}

/// Java OPC: Relationship Target is relative to the source part, not always `ppt/`.
pub fn resolve_relationship_target(part_name: &str, target: &str) -> String {
    if target.starts_with('/') {
        return collapse_zip_path(target.trim_start_matches('/'));
    }
    let dir = part_name.rsplit_once('/').map(|(d, _)| d).unwrap_or("");
    if dir.is_empty() {
        collapse_zip_path(target)
    } else {
        collapse_zip_path(&format!("{dir}/{target}"))
    }
}

#[derive(Clone, Debug)]
pub struct PackageRelationship {
    pub id: String,
    pub rel_type: String,
    pub target: String,
}

pub fn parse_package_relationships(xml: &str) -> Vec<PackageRelationship> {
    let mut reader = Reader::from_str(xml);
    let mut buffer = Vec::new();
    let mut result = Vec::new();
    loop {
        match reader.read_event_into(&mut buffer) {
            Ok(Event::Start(element)) | Ok(Event::Empty(element))
                if local_name(element.name().as_ref()) == "Relationship" =>
            {
                let mut id = None;
                let mut relationship_type = None;
                let mut target = None;
                for attribute in element.attributes().flatten() {
                    let key = local_name(attribute.key.as_ref());
                    let value = attribute
                        .unescape_value()
                        .map(|value| value.into_owned())
                        .unwrap_or_default();
                    match key.as_str() {
                        "Id" => id = Some(value),
                        "Type" => relationship_type = Some(value),
                        "Target" => target = Some(value),
                        _ => {}
                    }
                }
                if let (Some(id), Some(relationship_type), Some(target)) =
                    (id, relationship_type, target)
                {
                    result.push(PackageRelationship {
                        id,
                        rel_type: relationship_type,
                        target,
                    });
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buffer.clear();
    }
    result
}

pub fn part_rels_name(part_name: &str) -> String {
    let (dir, file) = part_name.rsplit_once('/').unwrap_or(("", part_name));
    if dir.is_empty() {
        format!("_rels/{file}.rels")
    } else {
        format!("{dir}/_rels/{file}.rels")
    }
}

// Java: PptxrSlide.getRels(OxfPptRelations.slideLayout()) / getLayoutRels()
pub fn first_rel_target_by_type_suffix(
    part_name: &str,
    rels_xml: &str,
    type_suffix: &str,
) -> Option<String> {
    parse_package_relationships(rels_xml)
        .into_iter()
        .find(|rel| rel.rel_type.ends_with(type_suffix))
        .map(|rel| resolve_relationship_target(part_name, &rel.target))
}

pub fn slide_layout_part_from_rels(slide_part: &str, rels_xml: &str) -> Option<String> {
    first_rel_target_by_type_suffix(slide_part, rels_xml, "/slideLayout")
}

pub fn slide_master_part_from_rels(layout_part: &str, rels_xml: &str) -> Option<String> {
    first_rel_target_by_type_suffix(layout_part, rels_xml, "/slideMaster")
}

pub fn theme_part_from_rels(part_name: &str, rels_xml: &str) -> Option<String> {
    first_rel_target_by_type_suffix(part_name, rels_xml, "/theme")
}

pub fn slide_image_part_from_rels(slide_part: &str, rels_xml: &str, r_id: &str) -> Option<String> {
    parse_package_relationships(rels_xml)
        .into_iter()
        .find(|rel| rel.id == r_id && rel.rel_type.ends_with("/image"))
        .map(|rel| resolve_relationship_target(slide_part, &rel.target))
}

fn presentation_slide_targets(xml: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    for rel in parse_package_relationships(xml) {
        if rel.rel_type.ends_with("/slide") {
            result.insert(
                rel.id,
                resolve_relationship_target("ppt/presentation.xml", &rel.target),
            );
        }
    }
    result
}

fn ordered_slide_xml_names(
    archive: &mut ZipArchive<Cursor<Vec<u8>>>,
    presentation_xml: &str,
    fallback_names: &[String],
) -> Vec<String> {
    let Some(rels_xml) = zip_read(archive, "ppt/_rels/presentation.xml.rels") else {
        return slide_xml_names(fallback_names);
    };
    let targets = presentation_slide_targets(&rels_xml);
    let ordered = presentation_slide_rids(presentation_xml)
        .into_iter()
        .filter_map(|relationship_id| targets.get(&relationship_id).cloned())
        .collect::<Vec<_>>();
    if ordered.is_empty() {
        slide_xml_names(fallback_names)
    } else {
        ordered
    }
}

fn extract_tag(xml: &str, tag: &str) -> Option<String> {
    let patterns = [format!("<p:{}", tag), format!("<a:{}", tag), format!("<{}", tag)];
    for open in &patterns {
        if let Some(start) = xml.find(open.as_str()) {
            let close = format!("</{}{}>", &open[1..2], tag).replace(":/", "/");
            let closes = [
                format!("</p:{}>", tag),
                format!("</a:{}>", tag),
                format!("</{}>", tag),
            ];
            for c in &closes {
                if let Some(end) = xml[start..].find(c.as_str()) {
                    return Some(xml[start..start + end + c.len()].to_string());
                }
            }
            // self-closing: <a:srgbClr val="112233"/>
            if let Some(gt) = xml[start..].find('>') {
                let slice = &xml[start..=start + gt];
                if slice.ends_with("/>") && tagged_open(&xml[start..], open) {
                    return Some(slice.to_string());
                }
            }
            let _ = close;
        }
    }
    None
}

fn is_shape_open(at: &str) -> bool {
    let after = if let Some(r) = at.strip_prefix("<p:sp") {
        r
    } else if let Some(r) = at.strip_prefix("<sp") {
        r
    } else {
        return false;
    };
    matches!(after.as_bytes().first(), Some(b'>' | b' ' | b'/'))
}

fn tagged_open(at: &str, open: &str) -> bool {
    let Some(rest) = at.strip_prefix(open) else {
        return false;
    };
    matches!(rest.as_bytes().first(), Some(b'>' | b' ' | b'/'))
}

fn next_sp_start(xml: &str) -> Option<usize> {
    let mut search = 0;
    while search < xml.len() {
        let rel = xml[search..]
            .find("<p:sp")
            .or_else(|| xml[search..].find("<sp"))?;
        let i = search + rel;
        if is_shape_open(&xml[i..]) {
            return Some(i);
        }
        search = i + 1;
    }
    None
}

fn next_tree_start(xml: &str) -> Option<(usize, &'static str)> {
    let mut best: Option<(usize, &'static str)> = None;
    let push = |best: &mut Option<(usize, &'static str)>, i: usize, kind: &'static str| {
        if best.map(|(b, _)| i < b).unwrap_or(true) {
            *best = Some((i, kind));
        }
    };
    if let Some(i) = next_sp_start(xml) {
        push(&mut best, i, "sp");
    }
    for (needle, kind) in [
        ("<p:pic", "pic"),
        ("<p:cxnSp", "cxnSp"),
        ("<p:grpSp", "grpSp"),
        ("<p:graphicFrame", "graphicFrame"),
    ] {
        if let Some(i) = xml.find(needle) {
            if tagged_open(&xml[i..], needle) {
                push(&mut best, i, kind);
            }
        }
    }
    best
}

fn parse_quoted_attr(xml: &str, key: &str) -> Option<String> {
    let pat = format!("{key}=\"");
    let i = xml.find(&pat)?;
    let rest = &xml[i + pat.len()..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

fn hex_rgb(s: &str) -> Option<u32> {
    let h = s.trim().trim_start_matches('#');
    if h.is_empty() {
        return None;
    }
    u32::from_str_radix(h, 16).ok()
}

fn extract_prefixed_open(xml: &str, open: &str, close: &str) -> Option<String> {
    let mut offset = 0usize;
    while let Some(i) = xml[offset..].find(open) {
        let abs = offset + i;
        if tagged_open(&xml[abs..], open) {
            if let Some(end) = xml[abs..].find(close) {
                return Some(xml[abs..abs + end + close.len()].to_string());
            }
            return None;
        }
        offset = abs + open.len();
    }
    None
}

fn solid_fill_argb(xml: &str, theme: &HashMap<i32, u32>) -> Option<u32> {
    if let Some(srgb) = extract_tag(xml, "srgbClr") {
        if let Some(val) = parse_quoted_attr(&srgb, "val") {
            return hex_rgb(&val).map(|c| 0xFF000000 | c);
        }
    }
    if let Some(sch) = extract_tag(xml, "schemeClr") {
        if let Some(val) = parse_quoted_attr(&sch, "val") {
            if let Some(idx) = DGMap::schemeColorMap().get(val.as_str()) {
                if let Some(rgb) = theme.get(idx) {
                    return Some(*rgb);
                }
                if let Some(rgb) = theme.get(&match *idx {
                    0 => 15, // dk1 → tx1 alias already in table
                    1 => 12,
                    _ => *idx,
                }) {
                    return Some(*rgb);
                }
            }
        }
    }
    None
}

/// `p:bgPr` solidFill only (pp-17:01). bgRef is ignored here.
pub fn parse_bg_pr_solid_fill(slide_xml: &str, theme: &HashMap<i32, u32>) -> Option<u32> {
    let bg = extract_prefixed_open(slide_xml, "<p:bg", "</p:bg>")?;
    let bgpr = extract_prefixed_open(&bg, "<p:bgPr", "</p:bgPr>")
        .or_else(|| extract_tag(&bg, "bgPr"))?;
    let solid = extract_tag(&bgpr, "solidFill")?;
    solid_fill_argb(&solid, theme)
}

/// `p:bgRef` when there is no bgPr (pp-17:02).
pub fn parse_bg_ref_fill(slide_xml: &str, theme: &HashMap<i32, u32>) -> Option<u32> {
    let bg = extract_prefixed_open(slide_xml, "<p:bg", "</p:bg>")?;
    if extract_prefixed_open(&bg, "<p:bgPr", "</p:bgPr>").is_some() {
        return None;
    }
    let bgref = extract_prefixed_open(&bg, "<p:bgRef", "</p:bgRef>")
        .or_else(|| extract_tag(&bg, "bgRef"))?;
    if let Some(c) = solid_fill_argb(&bgref, theme) {
        return Some(c);
    }
    let idx = parse_quoted_attr(&bgref, "idx")?.parse::<i32>().ok()?;
    if idx == 1 || idx == 1001 {
        return theme.get(&12).copied().or_else(|| theme.get(&1).copied());
    }
    None
}

fn apply_slide_background(
    slide_xml: &str,
    theme: &HashMap<i32, u32>,
) -> (bool, Option<u32>) {
    if let Some(c) = parse_bg_pr_solid_fill(slide_xml, theme) {
        return (true, Some(c));
    }
    if let Some(c) = parse_bg_ref_fill(slide_xml, theme) {
        return (true, Some(c));
    }
    (false, None)
}

fn extract_scheme_rgb(theme_xml: &str, slot: &str) -> Option<u32> {
    let open = format!("<a:{slot}");
    let start = theme_xml.find(&open).or_else(|| {
        theme_xml.find(&format!("<{slot}"))
    })?;
    let slice = utf8_window(theme_xml, start, 800);
    if let Some(val) = slice.find("srgbClr") {
        let rest = &slice[val..];
        if let Some(h) = parse_quoted_attr(rest, "val") {
            return hex_rgb(&h);
        }
    }
    if let Some(val) = slice.find("sysClr") {
        let rest = &slice[val..];
        if let Some(h) = parse_quoted_attr(rest, "lastClr").or_else(|| parse_quoted_attr(rest, "val")) {
            return hex_rgb(&h);
        }
    }
    None
}

// Java: ThemeHandler / clrScheme slots via DGMap.schemeColorMap
pub fn parse_theme_scheme_table(theme_xml: &str) -> HashMap<i32, u32> {
    let mut table = HashMap::new();
    for (name, idx) in DGMap::schemeColorMap() {
        if let Some(rgb) = extract_scheme_rgb(theme_xml, name) {
            table.insert(*idx, 0xFF000000 | rgb);
        }
    }
    let alias = [
        ("dk1", 15i32), // tx1
        ("lt1", 12),    // bg1
        ("dk2", 16),    // tx2
        ("lt2", 13),    // bg2
    ];
    for (name, dest) in alias {
        if table.contains_key(&dest) {
            continue;
        }
        if let Some(src) = DGMap::schemeColorMap().get(name).and_then(|i| table.get(i).copied()) {
            table.insert(dest, src);
        }
    }
    table
}

pub fn pic_blip_embed(xml: &str) -> Option<String> {
    let blip = extract_tag(xml, "blipFill")?;
    parse_quoted_attr(&blip, "embed").or_else(|| parse_quoted_attr(&blip, "r:embed"))
}

// Java: SwNvPrHandler.PlaceholderShapeHandler + SWReaderMap.placeholderTypeMap
pub fn parse_placeholder_from_shape_xml(xml: &str) -> Option<Placeholder> {
    let ph_at = xml.find("<p:ph").or_else(|| xml.find("<ph "))?;
    let slice = utf8_window(xml, ph_at, 500);
    let mut ph = Placeholder::default();
    if let Some(t) = parse_quoted_attr(slice, "type") {
        if let Some(v) = SWReaderMap::get_placeholderTypeMap().get(t.as_str()) {
            ph.putType(*v);
        }
    }
    if let Some(idx_s) = parse_quoted_attr(slice, "idx") {
        if idx_s == "4294967295" {
            ph.putIdx(-1);
        } else if let Ok(n) = idx_s.parse::<i32>() {
            ph.putIdx(n);
        }
    }
    Some(ph)
}

fn apply_scheme_fill(pr: &mut ShapePr, theme: &HashMap<i32, u32>) {
    if pr.has_fill {
        return;
    }
    if let Some(srgb) = pr.fill_obj.solid {
        pr.has_fill = true;
        pr.fill = srgb;
        return;
    }
    if let Some(sch) = pr.fill_obj.color.scheme {
        if let Some(rgb) = theme.get(&sch) {
            pr.has_fill = true;
            pr.fill = *rgb as i32;
            pr.fill_obj.solid = Some(*rgb as i32);
        }
    }
}

fn resolve_run_colors(op: &mut KmoTextOperator, theme: &HashMap<i32, u32>) {
    for para in &mut op.paras {
        for run in &mut para.runs {
            if run.ch_prop.font_color != 0 && run.ch_prop.font_color != 0xFF000000u32 as i32 {
                continue;
            }
            if let Some(sch) = run.ch_prop.scheme_slot {
                if let Some(rgb) = theme.get(&sch) {
                    run.ch_prop.font_color = *rgb as i32;
                }
            }
        }
    }
}

struct ParseCtx<'a> {
    theme: &'a HashMap<i32, u32>,
    images: &'a HashMap<String, Vec<u8>>,
    next_media_id: &'a mut i32,
}

fn parse_slide_xml(xml: &str, ctx: &mut ParseCtx<'_>) -> Vec<KmoShowShape> {
    parse_tree_children(xml, ctx)
}

/// 把一页 `slideN.xml` 的形状树写进已经装配好的 `KmoSlide`。
///
/// `PptxrSlides.open()` 在读到真实 part 字节后调用。主题色表为空时，显式 srgb 与文字仍然落盘。
pub fn fill_kmo_slide_from_xml(slide: &mut KmoSlide, xml: &str) {
    let theme = HashMap::new();
    let images = HashMap::new();
    let mut next_media_id = 0;
    let mut ctx = ParseCtx {
        theme: &theme,
        images: &images,
        next_media_id: &mut next_media_id,
    };
    slide.sp_tree.children = parse_slide_xml(xml, &mut ctx);
    let (has_background, bg_fill) = apply_slide_background(xml, &theme);
    slide.has_background = has_background;
    slide.bg_fill = bg_fill;
    slide.show_master_shape = parse_slide_show_master_sp(xml);
}

fn parse_tree_children(xml: &str, ctx: &mut ParseCtx<'_>) -> Vec<KmoShowShape> {
    let mut shapes = Vec::new();
    let mut rest = xml;
    while let Some((i, kind)) = next_tree_start(rest) {
        let from = &rest[i..];
        let close = match kind {
            "pic" => "</p:pic>",
            "cxnSp" => "</p:cxnSp>",
            "grpSp" => "</p:grpSp>",
            "graphicFrame" => "</p:graphicFrame>",
            _ if from.starts_with("<p:sp") => "</p:sp>",
            _ => "</sp>",
        };
        if let Some(end) = from.find(close) {
            let chunk = &from[..end + close.len()];
            if kind == "grpSp" {
                if let Some(sh) = parse_one_grp(chunk, ctx) {
                    shapes.push(sh);
                }
            } else if let Some(mut sh) = parse_one_sp(chunk, ctx) {
                sh.shape_type = match kind {
                    "pic" => 2,
                    "cxnSp" => 3,
                    "graphicFrame" => 4,
                    _ => 1,
                };
                shapes.push(sh);
            }
            rest = &from[end + close.len()..];
        } else {
            break;
        }
    }
    shapes
}

fn parse_one_grp(xml: &str, ctx: &mut ParseCtx<'_>) -> Option<KmoShowShape> {
    let mut group = parse_one_sp(xml, ctx)?;
    group.shape_type = 0;
    let dg = Rc::new(RefCell::new(GroupShape {
        sp_pr: group.sp_pr.clone(),
        ..Default::default()
    }));
    if let Some(sppr) = extract_tag(xml, "grpSpPr") {
        let mut gph = GroupShapePropertiesHandler::GroupShapePropertiesHandler(dg.clone());
        walk_xml(&sppr, &mut gph);
    }
    {
        let g = dg.borrow();
        group.has_xfrm = extract_tag(xml, "xfrm").is_some() || group.has_xfrm;
        group.transform2DX = g.sp_pr.xfrm.x;
        group.transform2DY = g.sp_pr.xfrm.y;
        group.transform2DW = g.sp_pr.xfrm.cx;
        group.transform2DH = g.sp_pr.xfrm.cy;
        group.sp_pr = g.sp_pr.clone();
        group.child_off = if g.has_child_point {
            Some((g.child_x, g.child_y))
        } else {
            None
        };
        group.child_ext = if g.has_child_extents {
            Some((g.child_cx, g.child_cy))
        } else {
            None
        };
    }
    let inner_start = xml.find('>').map(|i| i + 1).unwrap_or(0);
    let inner_end = xml.rfind("</p:grpSp>").unwrap_or(xml.len());
    group.children = parse_tree_children(&xml[inner_start..inner_end], ctx);
    Some(group)
}

fn parse_one_sp(xml: &str, ctx: &mut ParseCtx<'_>) -> Option<KmoShowShape> {
    let data_pr = Rc::new(RefCell::new(DataShapePr::create()));
    let mut sph = ShapePropertiesHandler::ShapePropertiesHandler(data_pr.clone());
    if let Some(sppr) = extract_tag(xml, "spPr")
        .or_else(|| extract_tag(xml, "grpSpPr"))
        .or_else(|| extract_tag(xml, "xfrm"))
    {
        walk_xml(&sppr, &mut sph);
    }
    let mut pr = ShapePr::default();
    // rust: ★2B-③-2 —— `layout_*_obj()` 现返回**真身**快照；本处 `ShapePr` 是
    //   `layout_shape` 影子域的展示结构（产物 B 的 MVP 渲染模型），
    //   故把真身快照按字段投影回影子表示（保持产物 B 行为不变）。
    // rust: ★2B-③-2 —— `layout_*` 现返回**真身共享句柄**（见其 owner 注记：
    //   `saveBin`/`open` 值快照有损），故借出后逐字段投影到影子表示。
    {
        let h = sph.layout_geom_obj();
        pr.geom_obj = sprint_geom_to_obj(&mut h.borrow_mut());
    }
    {
        let h = sph.layout_fill_obj();
        pr.fill_obj = sprint_fill_to_obj(&mut h.borrow_mut());
    }
    {
        let h = sph.layout_xfrm();
        pr.xfrm = sprint_xfrm_to_obj(&mut h.borrow_mut());
    }
    {
        let h = sph.layout_ln_obj();
        pr.outline_obj = sprint_outline_to_obj(&mut h.borrow_mut());
    }
    if data_pr.borrow().hasLn() {
        pr.has_ln = true;
        pr.ln = data_pr.borrow_mut().ln().get_width();
    }
    if data_pr.borrow().hasGeom() {
        pr.geom_type = Some(data_pr.borrow_mut().geom().get_type());
    }
    if let Some(blip) = extract_tag(xml, "blipFill") {
        let notify = Rc::new(RefCell::new(FillNotify::default()));
        // rust: ★2B-③-2 —— 处理器现持真 `Fill`。**必须从 `layout_fill_obj()` 的真身快照出发**
        //   （而非新建空 `Fill`），否则会丢掉 `FillPropertiesHandler` 已解析的全部填充属性 ——
        //   旧影子实现是 `pr.fill_obj.clone()` 后追加，此处保持同一语义。
        // rust: 真 `Fill` 无 `Clone`，经 `saveBin`/`open` 取得等价副本。
        // rust: 直接用真身句柄（`layout_fill_obj()` 已返回 `Shared<Fill>`）。
        let real_fill = sph.layout_fill_obj();
        let mut blip_h = BlipFillHandler::BlipFillHandler(real_fill.clone(), Some(notify.clone()));
        walk_xml(&blip, &mut blip_h);
        {
            let mut rf = real_fill.borrow_mut();
            pr.fill_obj = sprint_fill_to_obj(&mut rf);
        }
        // rust: ★2B-③-2 —— `blip_embed`/`blip_link` 是**媒体引用**的真实通道
        //   （下游 `shape.sp_pr.fill_obj.blip_embed` 决定 `media_r_id`）。
        //   真 `Blip` 位于 form-D gate 态，故从**已保留的真实采集通道**
        //   （`FillNotify` 回调列表）回填，语义与旧影子实现一致（后者最后一个胜出）。
        //   owner: ppt.show.pptxr.pptx_mvp    wave: M-PPT-SHAPE-DISPATCH
        let nb = notify.borrow();
        if let Some(e) = nb.blip_embed.last() {
            pr.fill_obj.blip_embed = Some(e.clone());
        }
        if let Some(l) = nb.blip_link.last() {
            pr.fill_obj.blip_link = Some(l.clone());
        }
    }
    let body = Rc::new(RefCell::new(TextBodySurf::default()));
    let mut tbh = TextBodyHandler::TextBodyHandler(body.clone());
    let mut text_op: Option<KmoTextOperator> = None;
    if let Some(tx) = extract_tag(xml, "txBody") {
        walk_xml(&tx, &mut tbh);
        let mut op = TextBodyHandler::to_operator(&body.borrow());
        resolve_run_colors(&mut op, ctx.theme);
        if !op.paras.is_empty() {
            text_op = Some(op);
        }
    }
    apply_scheme_fill(&mut pr, ctx.theme);
    let saw_xfrm = extract_tag(xml, "xfrm").is_some();
    let mut shape = KmoShowShape {
        has_xfrm: saw_xfrm,
        transform2DX: pr.xfrm.x,
        transform2DY: pr.xfrm.y,
        transform2DW: pr.xfrm.cx,
        transform2DH: pr.xfrm.cy,
        is_text_box: text_op.is_some(),
        has_text_body: text_op.is_some(),
        text_op,
        sp_pr: pr,
        ..KmoShowShape::default()
    };
    if let Some(ph) = parse_placeholder_from_shape_xml(xml) {
        shape.writePh(&ph);
    }
    let embed = shape
        .sp_pr
        .fill_obj
        .blip_embed
        .clone()
        .or_else(|| pic_blip_embed(xml));
    if let Some(r_id) = embed {
        shape.media_r_id = Some(r_id.clone());
        if let Some(bytes) = ctx.images.get(&r_id) {
            *ctx.next_media_id += 1;
            shape.media_id = *ctx.next_media_id;
            shape.media_bytes = Some(bytes.clone());
        }
    }
    Some(shape)
}

fn attach_slide_parent(slide: &mut KmoSlide) {
    let host = Rc::new(KmoSlide {
        layout: slide.layout.clone(),
        slide_width: slide.slide_width,
        slide_height: slide.slide_height,
        ..Default::default()
    });
    fn walk(shapes: &mut [KmoShowShape], host: &Rc<KmoSlide>) {
        for shape in shapes {
            shape.has_parent = true;
            shape.parent_type = 0;
            shape.parent_slide = Some(host.clone());
            walk(&mut shape.children, host);
        }
    }
    walk(&mut slide.sp_tree.children, &host);
}

fn bind_layout_master(
    archive: &mut ZipArchive<Cursor<Vec<u8>>>,
    slide_name: &str,
    slide: &mut KmoSlide,
    ctx: &mut ParseCtx<'_>,
) {
    let Some(rels_xml) = zip_read(archive, &part_rels_name(slide_name)) else {
        return;
    };
    let Some(layout_part) = slide_layout_part_from_rels(slide_name, &rels_xml) else {
        return;
    };
    let Some(layout_xml) = zip_read(archive, &layout_part) else {
        return;
    };
    let layout_children = parse_tree_children(&layout_xml, ctx);
    let layout_show_master = parse_layout_show_master_sp(&layout_xml);
    let mut master = KmoSlideMaster {
        sp_tree: Rc::new(KmoShapeTree::default()),
        ..Default::default()
    };
    if let Some(layout_rels) = zip_read(archive, &part_rels_name(&layout_part)) {
        if let Some(master_part) = slide_master_part_from_rels(&layout_part, &layout_rels) {
            if let Some(master_xml) = zip_read(archive, &master_part) {
                master.sp_tree = Rc::new(KmoShapeTree {
                    children: parse_tree_children(&master_xml, ctx),
                    ..Default::default()
                });
            }
        }
    }
    slide.layout = Some(Rc::new(KmoSlideLayout {
        master: Some(Rc::new(master)),
        sp_tree: Some(Rc::new(KmoShapeTree {
            children: layout_children,
            ..Default::default()
        })),
        show_master_shape: layout_show_master,
        ..Default::default()
    }));
}

fn load_theme_table(
    archive: &mut ZipArchive<Cursor<Vec<u8>>>,
    names: &[String],
) -> HashMap<i32, u32> {
    if let Some(pres_rels) = zip_read(archive, "ppt/_rels/presentation.xml.rels") {
        if let Some(theme_part) = theme_part_from_rels("ppt/presentation.xml", &pres_rels) {
            if let Some(xml) = zip_read(archive, &theme_part) {
                return parse_theme_scheme_table(&xml);
            }
        }
    }
    for name in names {
        if name.starts_with("ppt/theme/theme") && name.ends_with(".xml") && !name.contains("_rels") {
            if let Some(xml) = zip_read(archive, name) {
                return parse_theme_scheme_table(&xml);
            }
        }
    }
    HashMap::new()
}

fn slide_image_map(
    archive: &mut ZipArchive<Cursor<Vec<u8>>>,
    slide_name: &str,
) -> HashMap<String, Vec<u8>> {
    let mut images = HashMap::new();
    let Some(rels_xml) = zip_read(archive, &part_rels_name(slide_name)) else {
        return images;
    };
    for rel in parse_package_relationships(&rels_xml) {
        if !rel.rel_type.ends_with("/image") {
            continue;
        }
        let part = resolve_relationship_target(slide_name, &rel.target);
        if let Some(bytes) = zip_read_bytes(archive, &part) {
            images.insert(rel.id, bytes);
        }
    }
    images
}

fn tree_kind_element_id(kind: &str) -> i32 {
    match kind {
        "sp" => 3211593,
        "grpSp" => 3211594,
        "pic" => 3211469,
        "graphicFrame" => 3211595,
        "cxnSp" => 3211596,
        _ => 0,
    }
}

fn extract_sptree_inner(slide_xml: &str) -> &str {
    let Some(i) = slide_xml.find("<p:spTree") else {
        return "";
    };
    let Some(gt) = slide_xml[i..].find('>') else {
        return "";
    };
    let start = i + gt + 1;
    let Some(end) = slide_xml.rfind("</p:spTree>") else {
        return "";
    };
    if end < start {
        return "";
    }
    &slide_xml[start..end]
}

fn feed_open_sp_pr(chunk: &str, handler: &mut SwGroupShapeHandler<OpenCb>) {
    let Some(sh) = handler.shapeHandler.clone() else {
        return;
    };
    let pr = Rc::new(RefCell::new(DataShapePr::create()));
    if let Some(sppr) = extract_tag(chunk, "spPr") {
        let mut sph = ShapePropertiesHandler::ShapePropertiesHandler(pr.clone());
        walk_xml(&sppr, &mut sph);
    }
    sh.borrow_mut().commit_data_sp_pr(pr);
}

// rust: ★W6c-9 本函数无对应 Java 成员 —— **产品喂入端补齐 `p:spPr` 派发**。
//   Java 的四类子形状 handler 在 `enterSubElement(3211578)` 都建真身
//   `ShapePropertiesHandler`；引擎读到 `p:spPr` 时调用该分支。本 port 的喂入端
//   原先只在 `kind == "sp"` 时经 `feed_open_sp_pr` 覆盖 `sw`，其余三类从未收到
//   `p:spPr`（门禁 ⑰.3 实测 "写入调用点 1（目标 4）"）。
//   owner: show.pptxr.pptx_mvp.feed_group_handler    wave: M-PPT-KMOSLIDE-W6c-9
fn feed_child_sp_pr(kind: &str, chunk: &str, handler: &mut SwGroupShapeHandler<OpenCb>) {
    let Some(sppr) = extract_tag(chunk, "spPr") else {
        return;
    };
    let sub: Option<Box<dyn crate::cn::wps::drawing::dmlr::handler::ElementHandler>> = match kind {
        "pic" => handler
            .pictureHandler
            .clone()
            .and_then(|h| h.borrow_mut().enterSubElement(3211578)),
        "cxnSp" => handler
            .connectionShapeHandler
            .clone()
            .and_then(|h| h.borrow_mut().enterSubElement(3211578)),
        // rust: ★W6c-9 —— 四类**统一**用 `3211578`（`p:spPr` 的 Java 元素号，
        //   实测 `SwShapeHandler.java:66`/`SwConnectorShapeHandler.java:42`/
        //   `SwPictureHandler.java:47` 均为 `case 3211578`）。
        //   ★图文框侧原先写作 `3211587` 属**转写错误**（3211587 在 Java 里是
        //   `p:cNvGraphicFramePr`），已在本波一并纠错为 3211578
        //   （见 `SwGraphicFrameHandler.rs` 该分支的 owner 注）⇒ 此处不再需要特例。
        //   owner: show.pptxr.pptx_mvp.feed_group_handler    wave: M-PPT-KMOSLIDE-W6c-9
        "graphicFrame" => handler
            .graphicFrameHandler
            .clone()
            .and_then(|h| h.borrow_mut().enterSubElement(3211578)),
        _ => None,
    };
    if let Some(mut sub) = sub {
        walk_xml(&sppr, sub.as_mut());
    }
}

fn feed_group_handler(xml: &str, handler: &mut SwGroupShapeHandler<OpenCb>) {
    let mut rest = xml;
    loop {
        let Some((i, kind)) = next_tree_start(rest) else {
            break;
        };
        let from = &rest[i..];
        let close = match kind {
            "pic" => "</p:pic>",
            "cxnSp" => "</p:cxnSp>",
            "grpSp" => "</p:grpSp>",
            "graphicFrame" => "</p:graphicFrame>",
            _ if from.starts_with("<p:sp") => "</p:sp>",
            _ => "</sp>",
        };
        if let Some(end) = from.find(close) {
            let chunk = &from[..end + close.len()];
            let id = tree_kind_element_id(kind);
            if id != 0 {
                let _ = handler.enterSubElement(id);
                if id == 3211593 {
                    feed_open_sp_pr(chunk, handler);
                } else {
                    // rust: ★W6c-9 —— 其余三类（pic/cxnSp/graphicFrame）同样要收 `p:spPr`。
                    //   owner: show.pptxr.pptx_mvp.feed_group_handler  wave: M-PPT-KMOSLIDE-W6c-9
                    feed_child_sp_pr(kind, chunk, handler);
                }
                if kind == "grpSp" {
                    if let Some(nested) = handler.groupShapeHandler.clone() {
                        let inner_start = chunk.find('>').map(|j| j + 1).unwrap_or(0);
                        let inner_end = chunk.rfind("</p:grpSp>").unwrap_or(chunk.len());
                        if inner_end > inner_start {
                            feed_group_handler(
                                &chunk[inner_start..inner_end],
                                &mut nested.borrow_mut(),
                            );
                        }
                        let snap = snapshot_kmo(&nested.borrow().kmoGroup.borrow());
                        let last = handler.kmoGroup.borrow().childCount() - 1;
                        if last >= 0 {
                            handler.kmoGroup.borrow_mut().replaceChildAt(last, snap);
                        }
                    }
                }
            }
            rest = &from[end + close.len()..];
        } else {
            break;
        }
    }
}

fn write_app_film(
    slide_xml: &str,
    store: *mut DgStore,
    host: &dyn ISlide,
) -> Rc<RefCell<KmoFilm>> {
    let film = Rc::new(RefCell::new(KmoFilm::KmoFilm(host)));
    let mut csld = CommonSlideDataHandler::<OpenCb>::CommonSlideDataHandler(film.clone(), store, None);
    // Java: enterSubElement(3211606) → new SwGroupShapeHandler(this.shapeTree, callback)
    let mut group = SwGroupShapeHandler::<OpenCb>::SwGroupShapeHandler(
        csld.shapeTree.clone(),
        None,
    );
    let _ = csld.enterSubElement(3211606);
    feed_group_handler(extract_sptree_inner(slide_xml), &mut group);
    group.endElement(3211606);
    csld.endElement(0);
    film
}

struct OpenSlideHost {
    pres: KmoPresentation,
    theme: KmoTheme,
    events: KmoSlideEventListeners,
    bg: Background,
    wm: KsoWMInfo,
    cm: ColorMapping,
}

impl OpenSlideHost {
    fn with_store(store: *mut DgStore) -> Self {
        OpenSlideHost {
            pres: KmoPresentation {
                drawing_store: store,
                // rust: ★③-5 —— 打开期注入**真实**媒体表，使 `SwPictureHandler` 的
                //   `parent().getPresentation().mediaLib().getMediaData(id)` 与 Java 同形可达
                //   （原先该宿主无 `mediaLib()`，该调用链断裂）。
                //   owner: ppt.show.pptxr.pptx_mvp    wave: M-PPT-SHAPE-DISPATCH
                media_lib: std::rc::Rc::new(std::cell::RefCell::new(
                    crate::cn::wps::show::app::media::KmoMediaLib::KmoMediaLib::new(),
                )),
            },
            theme: KmoTheme,
            events: KmoSlideEventListeners::new(),
            // rust: ★W6c-4 —— 三个真身类型不再是单元结构体 ⇒ 必须构造实例。
            //   owner: ppt.show.pptxr.pptx_mvp    wave: M-PPT-KMOSLIDE-W6c-4
            bg: Background::new(),
            wm: KsoWMInfo::new(),
            cm: ColorMapping::create(),
        }
    }
}

impl ISlide for OpenSlideHost {
    fn r#type(&self) -> i32 {
        0
    }
    fn id(&self) -> i64 {
        0
    }
    fn getPresentation(&self) -> &KmoPresentation {
        &self.pres
    }
    fn theme(&self) -> &KmoTheme {
        &self.theme
    }
    fn eventListeners(&self) -> &KmoSlideEventListeners {
        &self.events
    }
    fn spTree(&self) -> &crate::cn::wps::show::app::shape::KmoShapeTree::KmoShapeTree {
        panic!("unused")
    }
    fn replaceSpid(&self, _var1: i32, _var2: i32) {}
    fn background(&self) -> &Background {
        &self.bg
    }
    fn ksoWMInfo(&self) -> &KsoWMInfo {
        &self.wm
    }
    fn colorMapping(&self) -> &ColorMapping {
        &self.cm
    }
    fn isIOReading(&self) -> bool {
        false
    }
}

struct OpenCb {
    state: HandlerCallbackState,
}
impl HandlerCallback for OpenCb {
    type DiagramPair = ();
    type Blip = ();
    type Color = ();
    // rust: ★③-5 去假面（形态 F）—— 真 handler（`SwPictureHandler`/`SwGraphicFrameHandler`/…）的
    //   `HandlerCallback<Hyperlink = Hyperlink>` 约束要求**真身**（原为 `()` ⇒ E0271）。
    //   owner: ppt.show.pptxr.pptx_mvp    wave: M-PPT-SHAPE-DISPATCH
    type Hyperlink = crate::cn::wps::drawing::data::hyperlink::Hyperlink::Hyperlink;
    type Media = ();
    type AudioFile = ();
    type QuickTimeFile = ();
    type VideoFile = ();
    type EmbeddedWAVAudioFile = ();
    type Model3D = ();
    type Shape = ();
    type ShapePr = HostShapePr;
    type ShapeStyle = HostShapeStyle;
    type TextRun = ();
    type TagLst = ();
    type FileManager = util::cn::wps::util::manager::FileManager::FileManager;
    fn callback_state(&self) -> &HandlerCallbackState {
        &self.state
    }
}

pub struct PptxDocument {
    pub slide_width: i32,
    pub slide_height: i32,
    pub slide_names: Vec<String>,
    pub slides: Vec<KmoSlide>,
    pub films: Vec<Rc<RefCell<KmoFilm>>>,
    open_store: Option<Box<DgStore>>,
    open_hosts: Vec<Box<OpenSlideHost>>,
}

// rust: 本方法无对应 Java 方法 —— `PptxDocument` 是**本 port 的文档句柄**（Java 侧没有这个类：
//   Java 的 `PptxReader.open()` 直接把结果装配进 `KmoPresentation`，`DgStore` 由
//   `presentation.drawingStore()` 取得）。本 port 尚无完整的 `KmoPresentation` 装配
//   （见 M-PPT-PRESENTATION），故在此**暴露已真实持有的 `DgStore`**，
//   使「`commit_drawing` 的 spPr 落到了真树」这件事**可被客观断言**（tests/render/08_function_*）。
//
//   ★这是**只读观察口**，不是新假面：它返回的是 `open_pptx_document_bytes` 里
//   真实创建、并被 `commit_drawing`/`SwGroupShapeHandler` 实际写入的**同一** `DgStore`
//   （`store_ptr` 即指向它）。它不产生任何新的行为分支，也不改变产品路径。
//   按 SP1b 先例（rust 扩展须登记 owner/wave）登记：
//   owner: ppt.show.pptxr.pptx_mvp    wave: M-PPT-KMOSLIDE
impl PptxDocument {
    /// 打开期真实使用的 `DgStore`（`commit_drawing` 的写入目标）。
    pub fn store(&self) -> Option<&DgStore> {
        self.open_store.as_deref()
    }
}

pub fn open_pptx_document_bytes(bytes: &[u8]) -> Result<PptxDocument, String> {
    // Java: PptxReader.open() 先经 `OxfrPackage`/`OpcrPackage`/`ZipFile`（S0 特征面）解析包结构，
    //   再由 `getPresPart(pack)` 取出 `ppt/presentation.xml` 对应的 part。
    // 产品接线：本函数的**包结构入口**即 S0 管线（`OxfrPackage::OxfrPackage__raf` → `OpcrPackage`
    //   → `ZipFile::from_reader`），presentation 部件名由 `part.getPartName()` 解析得出，
    //   不再硬编码 `"ppt/presentation.xml"`。
    let (pres_part_name, pres_xml) = {
        let mut cursor = Cursor::new(bytes.to_vec());
        let mut pack = OxfrPackage::OxfrPackage__raf(&mut cursor)
            .map_err(|e| format!("FileDamagedException: {e}"))?;
        let part = crate::cn::wps::show::pptxr::PptxReader::getPresPart(&mut pack)
            .map_err(|e| format!("{e:?}"))?;
        let part_name = part.getPartName();
        let mut stream = part
            .getInputStream(false)
            .map_err(|e| format!("IOException: {e}"))?;
        let mut raw = Vec::new();
        stream
            .read_to_end(&mut raw)
            .map_err(|e| format!("IOException: {e}"))?;
        (
            part_name,
            String::from_utf8(raw).map_err(|e| format!("IOException: {e}"))?,
        )
    };
    let mut z = ZipArchive::new(Cursor::new(bytes.to_vec())).map_err(|e| e.to_string())?;
    let names: Vec<String> = z.file_names().map(|s| s.to_string()).collect();
    // 部件名来自 S0 解析链（`OpcrPackagePart::getPartName()`）；字节直接取自该 part 的取流面。
    // 一致性：`zip_read` 走同一 entry 复核，两侧必须等价（不等 ⇒ S0 取流面与 zip crate 分歧）。
    let pres_xml_via_zip = zip_read(&mut z, &pres_part_name)
        .ok_or_else(|| format!("missing {pres_part_name}"))?;
    let pres_xml = if pres_xml_via_zip == pres_xml {
        pres_xml
    } else {
        return Err(format!(
            "S0 part stream mismatch for {pres_part_name}: part={} bytes, zip={} bytes",
            pres_xml.len(),
            pres_xml_via_zip.len()
        ));
    };
    let (w, h) = parse_sld_sz(&pres_xml);
    // Java: PresentationHandlerCallback.onSlideIds(rids)
    let slide_names = ordered_slide_xml_names(&mut z, &pres_xml, &names);
    if slide_names.is_empty() {
        return Err("SuffixError: Not pptx file".to_string());
    }
    let theme = load_theme_table(&mut z, &names);
    let mut next_media_id = 0;
    // Java: PptxrSlides.open() walks the unread slide list in presentation order.
    let mut slides = Vec::with_capacity(slide_names.len());
    let mut open_store = Box::new(DgStore::DgStore_new());
    let store_ptr = open_store.as_mut() as *mut DgStore;
    let mut open_hosts: Vec<Box<OpenSlideHost>> = Vec::new();
    let mut films: Vec<Rc<RefCell<KmoFilm>>> = Vec::new();
    for slide_name in &slide_names {
        let slide_xml = zip_read(&mut z, slide_name)
            .ok_or_else(|| format!("missing slide: {slide_name}"))?;
        let images = slide_image_map(&mut z, slide_name);
        let mut ctx = ParseCtx {
            theme: &theme,
            images: &images,
            next_media_id: &mut next_media_id,
        };
        let children = parse_slide_xml(&slide_xml, &mut ctx);
        let (has_background, bg_fill) = apply_slide_background(&slide_xml, &theme);
        let mut slide = KmoSlide {
            slide_width: w,
            slide_height: h,
            show_master_shape: parse_slide_show_master_sp(&slide_xml),
            has_background,
            bg_fill,
            sp_tree: KmoShapeTree {
                children,
                ..Default::default()
            },
            ..Default::default()
        };
        bind_layout_master(&mut z, slide_name, &mut slide, &mut ctx);
        attach_slide_parent(&mut slide);
        slides.push(slide);

        let host = Box::new(OpenSlideHost::with_store(store_ptr));
        let host_ptr: *const dyn ISlide = host.as_ref();
        open_hosts.push(host);
        let film = write_app_film(&slide_xml, store_ptr, unsafe { &*host_ptr });
        films.push(film);
    }
    crate::cn::wps::show::layout::page_inheritance::resolve_pages(&mut slides);
    for sl in &slides { for ch in &sl.sp_tree.children {
    }}
    Ok(PptxDocument {
        slide_width: w,
        slide_height: h,
        slide_names,
        slides,
        films,
        open_store: Some(open_store),
        open_hosts,
    })
}

pub fn open_pptx_bytes(bytes: &[u8]) -> Result<KmoSlide, String> {
    open_pptx_document_bytes(bytes)?
        .slides
        .into_iter()
        .next()
        .ok_or_else(|| "SuffixError: Not pptx file".to_string())
}

pub fn open_pptx_path(path: impl AsRef<Path>) -> Result<KmoSlide, String> {
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    open_pptx_bytes(&bytes)
}

pub fn open_pptx_document_path(path: impl AsRef<Path>) -> Result<PptxDocument, String> {
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    open_pptx_document_bytes(&bytes)
}

// rust: ★2B-③-2 —— 真身快照 → `layout_shape` 影子表示的投影辅助。
//   产物 B（`pptx_mvp` 的 MVP 渲染模型）消费影子 `ShapePr`；本波把 `sppr/**`
//   处理器改为写**真身**后，需要在边界处**逐字段**投影，以保证产物 B 行为不变
//   （任何字段遗漏都会改变渲染命令数/指纹，R15 会立刻暴露）。
//   owner: ppt.show.pptxr.pptx_mvp    wave: M-PPT-SHAPE-DISPATCH
fn sprint_geom_to_obj(g: &mut crate::cn::wps::drawing::data::geometry::Geometry2D::Geometry2D) -> crate::cn::wps::show::util::layout_shape::GeomObj {
    // rust: ★2B-③-2 —— 保留存在性：`prst` 仅在真身确有 `prstGeom` 时为 `Some`。
    //   owner: ppt.show.pptxr.pptx_mvp    wave: M-PPT-SHAPE-DISPATCH
    // rust: ★2B-③-2 —— `avLst`/`gdLst`/`rect`/`paths` 属 `custGeom`；其真身
    //   `CustomGeometry` 模块仍是 form-B gate，数据暂存在承载类型的**具体镜像**上，
    //   此处读回，避免投影丢字段（R15 已实测）。
    //   owner: ppt.show.pptxr.pptx_mvp    wave: M-PPT-SHAPE-DISPATCH
    let mut out = crate::cn::wps::show::util::layout_shape::GeomObj {
        prst: if g.hasPrstGeom() { Some(g.prstGeom().prst()) } else { None },
        ..Default::default()
    };
    {
        let cg = g.custGeom();
        out.av_lst = cg
            .av_lst_concrete_pairs
            .clone()
            .into_iter()
            .map(|(name, fmla)| crate::cn::wps::show::util::layout_shape::GeomGuideObj {
                name: Some(name),
                fmla: Some(fmla),
            })
            .collect();
        out.gd_lst = cg
            .gd_lst_concrete
            .iter()
            .map(|(name, fmla)| crate::cn::wps::show::util::layout_shape::GeomGuideObj {
                name: Some(name.clone()),
                fmla: Some(fmla.clone()),
            })
            .collect();
        out.rect = cg.rect_concrete.clone();
        out.paths = cg.paths_concrete.clone();
    }
    out
}

fn sprint_fill_to_obj(f: &mut crate::cn::wps::drawing::data::fill::Fill::Fill) -> crate::cn::wps::show::util::layout_shape::FillObj {
    // rust: ★2B-③-2 —— 同 `sprint_outline_to_obj`，**保留存在性**（`hasType` 决定
    //   `fill_type` 是否为 `Some`）。
    //   owner: ppt.show.pptxr.pptx_mvp    wave: M-PPT-SHAPE-DISPATCH
    let mut out = crate::cn::wps::show::util::layout_shape::FillObj::default();
    if !f.hasType() {
        return out;
    }
    out.fill_type = Some(f.fill_type());
    match f.fill_type() {
        // solidFill
        0 => {
            let c = f.solidFill_mut();
            if c.hasMode() { out.color.mode = Some(c.mode()); }
            if c.hasSrgb() {
                let rgb = c.srgb();
                let rgb = c.srgb();
                out.solid = Some(
                    (0xFF000000u32
                        | ((rgb.r() as u32) << 16)
                        | ((rgb.g() as u32) << 8)
                        | (rgb.b() as u32)) as i32,
                );
            }
            if c.hasScheme() {
                out.color.scheme = Some(c.scheme());
            }
        }
        // gradFill
        4 => {
            let g = f.gradFill_mut();
            if g.hasRotWithShape() { out.rot_with_shape = Some(g.isRotWithShape()); }
            if g.hasFlip() { out.flip = Some(g.flip()); }
            let mut bins = Vec::new();
            g.gsLst().read(&mut bins);
            out.gs_lst = bins
                .iter()
                .map(|b| {
                    let mut gs = crate::cn::wps::drawing::data::fill::GradientFill::GradientStop::open(b.as_ref());
                    let mut o = crate::cn::wps::show::util::layout_shape::GradientStopObj::default();
                    o.pos = gs.pos();
                    // rust: 停靠点颜色 -> 影子 `ColorSlot`（真身 `GradientStop::color()`）。
                    if gs.hasColor() {
                        let pr = gs.color();
                        if pr.hasMode() { o.color.mode = Some(pr.mode()); }
                        if pr.hasSrgb() {
                            let rgb = pr.srgb();
                            o.color.srgb = Some(
                                (0xFF000000u32
                                    | ((rgb.r() as u32) << 16)
                                    | ((rgb.g() as u32) << 8)
                                    | (rgb.b() as u32)) as i32,
                            );
                        }
                        if pr.hasScheme() {
                            o.color.scheme = Some(pr.scheme());
                        }
                    }
                    o
                })
                .collect();
            let lin = g.shade().lin_mut();
            if lin.hasScaled() { out.lin_scaled = Some(lin.isScaled()); }
            if lin.hasAng() { out.lin_ang = Some(lin.ang()); }
        }
        // pattFill
        1 => {
            let pf = f.pattFill_mut();
            if pf.hasPrst() { out.patt_prst = Some(pf.prst()); }
        }
        // blipFill
        2 => {
            let b = f.blipFill_mut();
            if b.hasDpi() { out.dpi = Some(b.dpi()); }
            if b.hasSrcRect() {
                // rust: `srcRect` 为 KProto 值；经真 `RelativeRect` 读回四边。
                let sr = b.srcRect();
                let rr = crate::cn::wps::drawing::data::basic::RelativeRect::RelativeRect::open(sr);
                out.src_rect = Some(crate::cn::wps::show::util::layout_shape::SrcRectObj {
                    l: rr.left() as f64 / 100000.0,
                    t: rr.top() as f64 / 100000.0,
                    r: rr.right() as f64 / 100000.0,
                    b: rr.bottom() as f64 / 100000.0,
                });
                out.src_rect_written = true;
            }
        }
        _ => {}
    }
    out
}

fn sprint_xfrm_to_obj(t: &mut crate::cn::wps::drawing::data::basic::Transform2D::Transform2D) -> crate::cn::wps::show::util::layout_shape::Transform2D {
    crate::cn::wps::show::util::layout_shape::Transform2D {
        x: t.point().x(),
        y: t.point().y(),
        cx: t.extents().cx(),
        cy: t.extents().cy(),
        rotation: t.rotation(),
        flipH: t.isFlipHorizontal(),
        flipV: t.isFlipVertical(),
    }
}

fn sprint_outline_to_obj(o: &mut crate::cn::wps::drawing::data::outline::Outline::Outline) -> crate::cn::wps::show::util::layout_shape::OutlineObj {
    // rust: ★2B-③-2 —— **保留"存在性"**：影子 `OutlineObj` 的 `Option` 字段表示
    //   "该属性在 XML 中出现过"。若把真身默认值一律投影为 `Some(..)`，
    //   `convert_kmo_harness` 的 `hasWidth()`/`hasFill()` 等分支会由 false 变 true，
    //   进而改变 `LineProperty` 与渲染命令（R15 已实测）。
    //   owner: ppt.show.pptxr.pptx_mvp    wave: M-PPT-SHAPE-DISPATCH
    let mut out = crate::cn::wps::show::util::layout_shape::OutlineObj::default();
    if o.hasWidth() { out.width = Some(o.get_width()); }
    if o.hasCompoundType() { out.compound = Some(o.compoundType()); }
    if o.hasLineCap() { out.cap = Some(o.lineCap()); }
    if o.hasStrokeAlignment() { out.algn = Some(o.get_strokeAlignment()); }
    if o.hasLineJoin() { out.join_type = Some(o.lineJoin().get_type()); }
    if o.lineJoin().hasMiterJoinLimit() { out.miter_limit = Some(o.lineJoin().get_miterJoinLimit()); }
    if o.hasPresetDash() { out.preset_dash = Some(o.get_presetDash()); }
    if o.hasFill() { out.fill = sprint_fill_to_obj_outline(o); }
    if o.hasCustomDashLst() {
        // rust: ★W6c-6 —— **转写纠错**。Java 的 `customDashLst` 是 `List<Outline.DashStop>`，
        //   每停靠点一个 bin（内部 KpMap 持 `dashLength`/`spaceLength`，**已是百分比小数**）。
        //   原实现把 `readCustomDashLst_bins()` 的返回值当「拍平的 int 序列」按 2 个一组
        //   组装，并额外除以 100000 ⇒ 与真身编码不等价（原编码本就是拍平近似）。
        //   现经真身 `DashStop::open(bin)` 读回 `dashLength`/`spaceLength`
        //   （`putDashLength` 收的就是 `toPercentage()` 的结果，单位一致，不再除）。
        //   owner: ppt.show.pptxr.pptx_mvp    wave: M-PPT-KMOSLIDE-W6c-6
        let bins = o.readCustomDashLst_bins();
        out.dash_stops = bins
            .iter()
            .map(|b| {
                let d = crate::cn::wps::drawing::data::outline::Outline::DashStop::open(b.as_ref());
                crate::cn::wps::show::util::layout_shape::DashStopObj {
                    dash: d.get_dashLength(),
                    space: d.get_spaceLength(),
                }
            })
            .collect();
    }
    if o.hasHeadEnd() {
        // rust: 逐字段保留存在性（`<a:headEnd type="arrow"/>` 可能没有 `w`/`len`）。
        let he = o.headEnd();
        out.head_end = Some(crate::cn::wps::show::util::layout_shape::LineEndObj {
            end_type: if he.hasType() { Some(he.get_type()) } else { None },
            width: if he.hasWidth() { Some(he.get_width()) } else { None },
            len: if he.hasLen() { Some(he.get_len()) } else { None },
        });
        out.head_written = true;
    }
    if o.hasTailEnd() {
        let te = o.tailEnd();
        out.tail_end = Some(crate::cn::wps::show::util::layout_shape::LineEndObj {
            end_type: if te.hasType() { Some(te.get_type()) } else { None },
            width: if te.hasWidth() { Some(te.get_width()) } else { None },
            len: if te.hasLen() { Some(te.get_len()) } else { None },
        });
        out.tail_written = true;
    }
    out
}

fn sprint_fill_to_obj_outline(o: &mut crate::cn::wps::drawing::data::outline::Outline::Outline) -> crate::cn::wps::show::util::layout_shape::FillObj {
    let mut out = crate::cn::wps::show::util::layout_shape::FillObj::default();
    {
        let of = o.fill();
        if !of.hasType() {
            return out;
        }
        out.fill_type = Some(of.r#type());
    }
    let c = o.fill().solidFill_mut();
    if c.hasMode() { out.color.mode = Some(c.mode()); }
    if c.hasSrgb() {
        let rgb = c.srgb();
        out.solid = Some(
            (0xFF000000u32 | ((rgb.r() as u32) << 16) | ((rgb.g() as u32) << 8) | (rgb.b() as u32))
                as i32,
        );
    }
    if c.hasScheme() {
        out.color.scheme = Some(c.scheme());
    }
    out
}
