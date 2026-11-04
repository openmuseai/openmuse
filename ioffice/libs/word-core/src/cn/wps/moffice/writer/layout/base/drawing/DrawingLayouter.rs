#![allow(dead_code, non_snake_case, unused_variables, unused_mut, unused_imports)]
// Java: import cn.wps.collections.KList
// CARGO_OFF_SURFACE use util::collections::KList::KList;
// Java: import cn.wps.collections.KPList
// CARGO_OFF_SURFACE use util::collections::KPList::KPList;
// Java: import cn.wps.core.runtime.Assert
// CARGO_OFF_SURFACE use util::runtime::Assert::Assert;
// Java: import cn.wps.graphics.Rect
// CARGO_OFF_SURFACE use util::graphics::Rect::Rect;
// Java: import cn.wps.graphics.RectF
// CARGO_OFF_SURFACE use util::graphics::RectF::RectF;
// Java: import cn.wps.moffice.drawing.GroupShape
// CARGO_OFF_SURFACE use drawing::cn::wps::moffice::drawing::GroupShape::GroupShape;
// Java: import cn.wps.moffice.drawing.IDrawingCache
// CARGO_OFF_SURFACE use drawing::cn::wps::moffice::drawing::IDrawingCache::IDrawingCache;
// Java: import cn.wps.moffice.drawing.IPos
// CARGO_OFF_SURFACE use drawing::cn::wps::moffice::drawing::IPos::IPos;
// Java: import cn.wps.moffice.drawing.Shape
// CARGO_OFF_SURFACE use drawing::cn::wps::moffice::drawing::Shape::Shape;
// Java: import cn.wps.moffice.drawing.ShapeHelper
// CARGO_OFF_SURFACE use drawing::cn::wps::moffice::drawing::ShapeHelper::ShapeHelper;
// Java: import cn.wps.moffice.property.PropertySet
// CARGO_OFF_SURFACE use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.moffice.writer.cache.ITypoRect
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::ITypoRect::ITypoRect;
// Java: import cn.wps.moffice.writer.cache.LocateTool
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::LocateTool::LocateTool;
// Java: import cn.wps.moffice.writer.cache.TypoColumnPage
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoColumnPage::TypoColumnPage;
// Java: import cn.wps.moffice.writer.cache.TypoColumns
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoColumns::TypoColumns;
// Java: import cn.wps.moffice.writer.cache.TypoDrawing
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
// Java: import cn.wps.moffice.writer.cache.TypoDrawingLayers
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoDrawingLayers::TypoDrawingLayers;
// Java: import cn.wps.moffice.writer.cache.TypoFootEndNote
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoFootEndNote::TypoFootEndNote;
// Java: import cn.wps.moffice.writer.cache.TypoHeaderFooterPage
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoHeaderFooterPage::TypoHeaderFooterPage;
// Java: import cn.wps.moffice.writer.cache.TypoLayoutPage
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoLayoutPage::TypoLayoutPage;
// Java: import cn.wps.moffice.writer.cache.TypoList
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoList::TypoList;
// Java: import cn.wps.moffice.writer.cache.TypoMemoryObj
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoMemoryObj::TypoMemoryObj;
// Java: import cn.wps.moffice.writer.cache.TypoObjBase
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
// Java: import cn.wps.moffice.writer.cache.TypoPage
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage;
// Java: import cn.wps.moffice.writer.cache.TypoRect
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoRect::TypoRect;
// Java: import cn.wps.moffice.writer.cache.TypoRectCoreImpl
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoRectCoreImpl::TypoRectCoreImpl;
// Java: import cn.wps.moffice.writer.cache.TypoRectMemoryImpl
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoRectMemoryImpl::TypoRectMemoryImpl;
// Java: import cn.wps.moffice.writer.cache.TypoSnapshot
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
// Java: import cn.wps.moffice.writer.cache.TypoTextBox
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoTextBox::TypoTextBox;
// Java: import cn.wps.moffice.writer.cache.TypoTextLine
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine;
// Java: import cn.wps.moffice.writer.cache.TypoToolsManager
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoToolsManager::TypoToolsManager;
// Java: import cn.wps.moffice.writer.cache.style.TypoSHDAndBRCs
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::style::TypoSHDAndBRCs::TypoSHDAndBRCs;
// Java: import cn.wps.moffice.writer.core.Document
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.core.shape.ShapeMetrics
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::core::shape::ShapeMetrics::ShapeMetrics;
// Java: import cn.wps.moffice.writer.data.DefaultSectionProperties
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::DefaultSectionProperties::DefaultSectionProperties;
// Java: import cn.wps.moffice.writer.data.DefaultValue
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::DefaultValue::DefaultValue;
// Java: import cn.wps.moffice.writer.data.IPapxTable
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::IPapxTable::IPapxTable;
// Java: import cn.wps.moffice.writer.data.PLCSection
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::PLCSection::PLCSection;
// Java: import cn.wps.moffice.writer.data.RANGEFactory
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::RANGEFactory::RANGEFactory;
// Java: import cn.wps.moffice.writer.data.table.CellInfo
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::table::CellInfo::CellInfo;
// Java: import cn.wps.moffice.writer.data.table.RowInfo
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::table::RowInfo::RowInfo;
// Java: import cn.wps.moffice.writer.data.table.TableInfo
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::table::TableInfo::TableInfo;
// Java: import cn.wps.moffice.writer.data.table.TablePos
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::table::TablePos::TablePos;
// Java: import cn.wps.moffice.writer.data.value.BRC
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::value::BRC::BRC;
// Java: import cn.wps.moffice.writer.data.value.DropCap
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::value::DropCap::DropCap;
// Java: import cn.wps.moffice.writer.data.value.PageSetup
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::value::PageSetup::PageSetup;
// Java: import cn.wps.moffice.writer.data.value.TextFlow
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::value::TextFlow::TextFlow;
// Java: import cn.wps.moffice.writer.drawing.DrawingContainer
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::drawing::DrawingContainer::DrawingContainer;
// Java: import cn.wps.moffice.writer.drawing.TextFrame
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::drawing::TextFrame::TextFrame;
// Java: import cn.wps.moffice.writer.layout.base.LayouterBase
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::LayouterBase::LayouterBase;
// Java: import cn.wps.moffice.writer.layout.base.SectionTool
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::SectionTool::SectionTool;
// Java: import cn.wps.moffice.writer.layout.base.drawing.textbox.TextboxLayoutState
pub use crate::cn::wps::moffice::writer::layout::base::drawing::textbox::TextboxLayoutState::TextboxLayoutState;
// Java: import cn.wps.moffice.writer.layout.base.drawing.textbox.TextboxLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::drawing::textbox::TextboxLayouter::TextboxLayouter;
// Java: import cn.wps.moffice.writer.layout.base.env.DocumentEnv
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.DrawingCacheItem
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::DrawingCacheItem::DrawingCacheItem;
// Java: import cn.wps.moffice.writer.layout.base.env.MainState
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
// Java: import cn.wps.moffice.writer.layout.base.env.PageEnv
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::PageEnv::PageEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.ParaEnv
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::ParaEnv::ParaEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.Pools
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::Pools::Pools;
// Java: import cn.wps.moffice.writer.layout.base.env.ViewEnv
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
// Java: import cn.wps.moffice.writer.layout.base.graph.DrawingNode
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::graph::DrawingNode::DrawingNode;
// Java: import cn.wps.moffice.writer.layout.base.line.IGetTextWrapLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::line::IGetTextWrapLayouter::IGetTextWrapLayouter;
// Java: import cn.wps.moffice.writer.layout.base.tools.BRCBorderTool
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::tools::BRCBorderTool::BRCBorderTool;
// Java: import cn.wps.moffice.writer.layout.pageview.headerfooter.HeaderFooterPageTextboxLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::pageview::headerfooter::HeaderFooterPageTextboxLayouter::HeaderFooterPageTextboxLayouter;
// Java: import cn.wps.moffice.writer.layout.base.drawing.ShapeCollision
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::drawing::ShapeCollision::ShapeCollision;
// Java: import java.lang.Object
// CARGO_OFF_SURFACE use stubs::java::lang::Object::Object;
// Java: import cn.wps.moffice.writer.layout.base.drawing.TextWrapLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::drawing::TextWrapLayouter::TextWrapLayouter;
// Java: import org.apache.poi.poifs.filesystem.Entry
// CARGO_OFF_SURFACE use io_poi_compact::org::apache::poi::poifs::filesystem::Entry::Entry;
// Java: import java.lang.CloneNotSupportedException
// CARGO_OFF_SURFACE use stubs::java::lang::CloneNotSupportedException::CloneNotSupportedException;

use core::marker::PhantomData as _;

// CARGO_OFF_SURFACE_TYPES — drawing/cache neighbors not on crate surface this function
#[derive(Default, Clone)]
pub struct KList<T> {
    pub items: Vec<T>,
}
impl<T> KList<T> {
    pub fn new() -> Self { Self { items: Vec::new() } }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
    pub fn firstElement(&self) -> Option<&T> { self.items.first() }
}
#[derive(Default, Clone)]
pub struct KPList<T> {
    pub items: Vec<T>,
}
#[derive(Default, Clone)]
pub struct DrawingCacheItem;
#[derive(Default, Clone)]
pub struct DrawingNode {
    pub shape: Shape,
    pub cp: i32,
}
#[derive(Default, Clone)]
pub struct Shape;
#[derive(Default, Clone)]
pub struct PageSetup;
#[derive(Default, Clone)]
pub struct TextboxLayouter;
#[derive(Default, Clone)]
pub struct IGetTextWrapLayouter;
#[derive(Default, Clone)]
pub struct RectFWrapImp {
    pub left: i32,
    pub right: i32,
    pub wrap_type: i32,
}
impl RectFWrapImp {
    pub fn getLeft(&self) -> i32 { self.left }
    pub fn getRight(&self) -> i32 { self.right }
    pub fn getWrapType(&self) -> i32 { self.wrap_type }
}
#[derive(Default, Clone)]
pub struct NormalCollosion;
impl NormalCollosion {
    pub fn new(_tools: TypoToolsManager) -> Self { Self }
    pub fn adjust(&mut self, _drawing: &mut TypoDrawing, _stateEnvoy: &mut dyn IStateEnvoy, _main: &crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState) {}
}
#[derive(Default, Clone)]
pub struct TypoToolsManager;
impl TypoToolsManager {
    pub fn obtainRect(&self) -> TypoRect { TypoRect::default() }
    pub fn recycle(&self, _r: TypoRect) {}
    pub fn recycle_mem(&self, _m: &TypoMemoryObj) {}
}
#[derive(Default, Clone)]
pub struct TypoMemoryObj;
#[derive(Default, Clone, PartialEq)]
pub struct TypoRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
impl TypoRect {
    pub fn set(&mut self, drawing: &TypoDrawing) {
        self.left = drawing.left;
        self.top = drawing.top;
        self.right = drawing.left + drawing.width;
        self.bottom = drawing.top + drawing.height;
    }
    pub fn width(&self) -> i32 { self.right - self.left }
    pub fn getLeft(&self) -> i32 { self.left }
    pub fn setLeft(&mut self, v: i32) { self.left = v; }
    pub fn setWidth(&mut self, w: i32) { self.right = self.left + w; }
    pub fn recycle(&mut self) {}
}
/// Java ShapeCollision.ClipRect remaining text strips after wrap subtract.
#[derive(Default, Clone)]
pub struct ClipRect {
    pub layout_left: i32,
    pub layout_right: i32,
    pub top: i32,
    pub bottom: i32,
    pub remaining: Vec<(i32, i32)>,
}
impl ClipRect {
    pub fn init(&mut self, layoutLeft: i32, top: i32, layoutRight: i32, bottom: i32) {
        self.layout_left = layoutLeft;
        self.layout_right = layoutRight;
        self.top = top;
        self.bottom = bottom;
        self.remaining = vec![(layoutLeft, layoutRight)];
    }
    /// Punch [wrap_left, wrap_right) out of remaining strips (Java ClipRect subtract).
    pub fn subtract_wrap(&mut self, wrap_left: i32, wrap_right: i32) {
        let mut next = Vec::new();
        for (l, r) in self.remaining.drain(..) {
            if wrap_right <= l || wrap_left >= r {
                next.push((l, r));
                continue;
            }
            if l < wrap_left {
                next.push((l, wrap_left.min(r)));
            }
            if r > wrap_right {
                next.push((wrap_right.max(l), r));
            }
        }
        self.remaining = next;
    }
    pub fn getRects(&self) -> Vec<RectFWrapImp> {
        self.remaining
            .iter()
            .map(|&(l, r)| RectFWrapImp { left: l, right: r, wrap_type: 0 })
            .collect()
    }
    pub fn max_width(&self) -> i32 {
        self.remaining.iter().map(|&(l, r)| r - l).max().unwrap_or(0)
    }
}
#[derive(Default, Clone)]
pub struct IPos {
    pub alignH: i32,
    pub alignV: i32,
    pub posHPercent: f32,
    pub posVPercent: f32,
}
impl IPos {
    pub fn getAlignmentH(&self) -> i32 { self.alignH }
    pub fn getAlignmentV(&self) -> i32 { self.alignV }
    pub fn getPosHPercent(&self) -> f32 { self.posHPercent }
    pub fn getPosVPercent(&self) -> f32 { self.posVPercent }
}
#[derive(Default, Clone, PartialEq)]
pub struct ITypoRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
impl ITypoRect {
    pub fn offsetTo(&mut self, x: i32, y: i32) {
        let w = self.right - self.left;
        let h = self.bottom - self.top;
        self.left = x;
        self.top = y;
        self.right = x + w;
        self.bottom = y + h;
    }
    pub fn setWidth(&mut self, w: i32) { self.right = self.left + w; }
    pub fn setHeight(&mut self, h: i32) { self.bottom = self.top + h; }
    pub fn setLeft(&mut self, v: i32) { self.left = v; }
    pub fn setRight(&mut self, v: i32) { self.right = v; }
    pub fn setTop(&mut self, v: i32) { self.top = v; }
    pub fn setBottom(&mut self, v: i32) { self.bottom = v; }
    pub fn getLeft(&self) -> i32 { self.left }
    pub fn getTop(&self) -> i32 { self.top }
    pub fn getRight(&self) -> i32 { self.right }
    pub fn getBottom(&self) -> i32 { self.bottom }
    pub fn width(&self) -> i32 { self.right - self.left }
    pub fn height(&self) -> i32 { self.bottom - self.top }
    pub fn offset(&mut self, dx: i32, dy: i32) {
        self.left += dx;
        self.right += dx;
        self.top += dy;
        self.bottom += dy;
    }
    pub fn set__ityporect(&mut self, page: &TypoPage) {
        self.left = 0;
        self.top = 0;
        self.right = page.width;
        self.bottom = page.height;
    }
    pub fn recycle(&mut self) {}
    pub fn from_typo(r: &TypoRect) -> Self {
        Self { left: r.left, top: r.top, right: r.right, bottom: r.bottom }
    }
}
#[derive(Default, Clone)]
pub struct TypoRectCoreImpl {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
impl TypoRectCoreImpl {
    pub fn obtain() -> Self { Self::default() }
    pub fn recycle(&mut self) {}
    pub fn offset(&mut self, dx: i32, dy: i32) {
        self.left += dx;
        self.right += dx;
        self.top += dy;
        self.bottom += dy;
    }
    pub fn offsetTo(&mut self, x: i32, y: i32) {
        let w = self.right - self.left;
        let h = self.bottom - self.top;
        self.left = x;
        self.top = y;
        self.right = x + w;
        self.bottom = y + h;
    }
    pub fn isEmpty(&self) -> bool { self.left >= self.right || self.top >= self.bottom }
    pub fn set(&mut self, other: &TypoRectCoreImpl) {
        self.left = other.left;
        self.top = other.top;
        self.right = other.right;
        self.bottom = other.bottom;
    }
    pub fn union(&mut self, other: &TypoRectCoreImpl) {
        if self.isEmpty() {
            self.set(other);
            return;
        }
        self.left = self.left.min(other.left);
        self.top = self.top.min(other.top);
        self.right = self.right.max(other.right);
        self.bottom = self.bottom.max(other.bottom);
    }
    pub fn intersects(&self, l: i32, t: i32, r: i32, b: i32) -> bool {
        self.left < r && l < self.right && self.top < b && t < self.bottom
    }
}
#[derive(Default, Clone)]
pub struct TypoPage {
    pub type_: i32,
    pub pointer: i32,
    pub marginLeft: i32,
    pub marginTop: i32,
    pub width: i32,
    pub height: i32,
    pub coreWidth: i32,
    pub coreHeight: i32,
    pub visualTextflow: i32,
    pub lines: i32,
    pub parent: i32,
    pub documentType: i32,
}
impl TypoPage {
    pub fn getType(&self) -> i32 { self.type_ }
    pub fn getPointer(&self) -> i32 { self.pointer }
    pub fn getMarginLeft(&self) -> i32 { self.marginLeft }
    pub fn getMarginTop(&self) -> i32 { self.marginTop }
    pub fn width(&self) -> i32 { self.width }
    pub fn getCoreWidth(&self) -> i32 { self.coreWidth }
    pub fn getCoreHeight(&self) -> i32 { self.coreHeight }
    pub fn getVisualTextflow(&self) -> i32 { self.visualTextflow }
    pub fn getLines(&self) -> i32 { self.lines }
    pub fn getParent(&self) -> i32 { self.parent }
    pub fn getDocumentType(&self) -> i32 { self.documentType }
}
#[derive(Default, Clone)]
pub struct TypoDrawing {
    pub left: i32,
    pub top: i32,
    pub width: i32,
    pub height: i32,
    pub wrap: bool,
    pub table: bool,
    pub cp: i32,
    pub pointer: i32,
    pub pos: Option<IPos>,
    pub intersected: bool,
    pub layout_padding: i32,
    pub has_shape: bool,
    pub shape_type: i32,
}
impl TypoDrawing {
    pub fn getPos(&self) -> Option<IPos> { self.pos.clone() }
    pub fn isWrap(&self) -> bool { self.wrap }
    pub fn isTable(&self) -> bool { self.table }
    pub fn getLeft(&self) -> i32 { self.left }
    pub fn getTop(&self) -> i32 { self.top }
    pub fn getCP(&self) -> i32 { self.cp }
    pub fn getPointer(&self) -> i32 { self.pointer }
    pub fn getLayoutPadding(&self) -> i32 { self.layout_padding }
    pub fn offsetTo(&mut self, x: i32, y: i32) { self.left = x; self.top = y; }
    pub fn offset(&mut self, dx: i32, dy: i32) { self.left += dx; self.top += dy; }
    pub fn setIntersectedWithText__boolean(&mut self, v: bool) { self.intersected = v; }
    pub fn layoutRectForPageCore(&self, out: &mut TypoRectCoreImpl) {
        out.left = self.left;
        out.top = self.top;
        out.right = self.left + self.width;
        out.bottom = self.top + self.height;
    }
}
#[derive(Default, Clone)]
pub struct TypoLayoutPage;
impl TypoLayoutPage {
    pub fn getFooterPage(&self) -> i32 { 0 }
    pub fn getFootnotes(&self) -> i32 { 0 }
}
#[derive(Default, Clone)]
pub struct TypoColumnPage;
impl TypoColumnPage {
    pub fn getFootnotes(&self) -> i32 { 0 }
}
#[derive(Default, Clone)]
pub struct TypoHeaderFooterPage;
impl TypoHeaderFooterPage {
    pub fn getTop(_p: i32, _s: &crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) -> i32 { 0 }
}
#[derive(Default, Clone)]
pub struct TypoFootEndNote;
impl TypoFootEndNote {
    pub fn getTop(_p: i32, _s: &crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) -> i32 { 0 }
}
pub struct DrawingPositionTool;
impl DrawingPositionTool {
    pub fn shouldCorrectDrawing(d: &TypoDrawing) -> bool { d.isWrap() }
    pub fn correctWrapDrawing(d: &mut TypoDrawing, limit: &TypoRectCoreImpl, _a: bool, _b: bool) {
        if d.left < limit.left { d.left = limit.left; }
        if d.top < limit.top { d.top = limit.top; }
        let right = d.left + d.width;
        let bottom = d.top + d.height;
        if right > limit.right { d.left = (limit.right - d.width).max(limit.left); }
        if bottom > limit.bottom { d.top = (limit.bottom - d.height).max(limit.top); }
    }
    pub fn normalizeRelX__int_int(rel_x: i32, global_page_idx: i32) -> i32 {
        let even = global_page_idx % 2 == 0;
        if rel_x == 6 { if even { 4 } else { 5 } }
        else if rel_x == 7 { if even { 5 } else { 4 } }
        else { rel_x }
    }
    pub fn normalizeRelY__int_int_boolean(rel_y: i32, global_page_idx: i32, before_2013: bool) -> i32 {
        let even = global_page_idx % 2 == 0;
        if before_2013 {
            if rel_y == 6 || rel_y == 7 { return if even { 4 } else { 5 }; }
        } else if rel_y == 6 {
            return if even { 4 } else { 5 };
        } else if rel_y == 7 {
            return if even { 5 } else { 4 };
        }
        rel_y
    }
    pub fn isHoriPosOfLineContainerPage__int(relX: i32) -> bool { relX == 3 }
    pub fn isVertPosOfLineContainerPage__int(relY: i32) -> bool { relY == 3 || relY == 2 }
    pub fn isWrap__int(text_wrap: i32) -> bool {
        matches!(text_wrap, 0 | 1 | 2 | 4 | 5)
    }
    pub fn shouldOffsetDrawingUsingRenderRect__typodrawing_int_int(d: &TypoDrawing, align_h: i32, align_v: i32) -> bool {
        if !d.has_shape {
            return false;
        }
        if align_h == 0 && align_v == 0 {
            return false;
        }
        if d.shape_type == 202 {
            return true;
        }
        match d.getPos() {
            Some(pos) => pos.getPosHPercent() != 0.0 || pos.getPosVPercent() != 0.0,
            None => false,
        }
    }
    pub fn shouldOffsetDrawingUsingRenderRect__typodrawing(d: &TypoDrawing) -> bool {
        match d.getPos() {
            Some(pos) => Self::shouldOffsetDrawingUsingRenderRect__typodrawing_int_int(d, pos.getAlignmentH(), pos.getAlignmentV()),
            None => false,
        }
    }
    pub fn shouldReverseShapeRectLeft(d: &TypoDrawing, rel_x: i32, page: &TypoPage, snapshot: &crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) -> bool {
        rel_x == 2 && Self::isWrapTableOrTextFrame(d.getPointer(), snapshot) && page.getVisualTextflow() == 1
    }
    pub fn isWrapTableOrTextFrame(drawing: i32, snapshot: &crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) -> bool {
        if drawing == 0 {
            false
        } else {
            let drawing_type = crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::getDrawingType__int_typosnapshot(drawing, snapshot);
            drawing_type == 7 || drawing_type == 8
        }
    }
    pub fn normalizeAlignH(align_h: i32, global_page_idx: i32) -> i32 {
        let even = global_page_idx % 2 == 0;
        if align_h == 4 { if even { 1 } else { 3 } }
        else if align_h == 5 { if even { 3 } else { 1 } }
        else { align_h }
    }
    fn normalize_align_v(rel_y: i32, align_v: i32, global_page_idx: i32) -> i32 {
        let even = global_page_idx % 2 == 0;
        let align_v = if align_v == 4 { if even { 1 } else { 3 } }
            else if align_v == 5 { if even { 3 } else { 1 } }
            else { align_v };
        if rel_y == 2 && align_v != 0 { 1 } else { align_v }
    }
    pub fn getDrawingLeftOfPageCore(align_h: i32, page_idx: i32, shape: &ITypoRect, align: &ITypoRect) -> i32 {
        match Self::normalizeAlignH(align_h, page_idx) {
            0 => align.getLeft() + shape.getLeft(),
            1 => align.getLeft(),
            2 => (align.getLeft() + align.getRight() - shape.width()) / 2,
            3 => align.getRight() - shape.width(),
            _ => align.getLeft(),
        }
    }
    pub fn getDrawingTopOfPageCore(rel_y: i32, align_v: i32, page_idx: i32, shape: &ITypoRect, align: &ITypoRect) -> i32 {
        match Self::normalize_align_v(rel_y, align_v, page_idx) {
            0 => align.getTop() + shape.getTop(),
            1 => align.getTop(),
            2 => (align.getTop() + align.getBottom() - shape.height()) / 2,
            3 => align.getBottom() - shape.height(),
            _ => align.getTop(),
        }
    }
    pub fn getColPageDrawingOffsetResult(a: &TypoPage, b: &TypoPage, snapshot: &crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) -> i64 {
        crate::cn::wps::moffice::writer::layout::base::drawing::DrawingCoordTool::colPageDrawingOffset(a.getPointer(), b.getPointer(), snapshot)
    }
    pub fn getColPageDrawingOffsetX(r: i64) -> i32 {
        crate::cn::wps::moffice::writer::layout::base::drawing::DrawingCoordTool::unpack_first_int(r)
    }
    pub fn getColPageDrawingOffsetY(r: i64) -> i32 {
        crate::cn::wps::moffice::writer::layout::base::drawing::DrawingCoordTool::unpack_second_int(r)
    }
}
pub struct DrawingCoordTool;
impl DrawingCoordTool {
    pub fn renderRectForPageCore(_d: &TypoDrawing, _r: &mut TypoRectCoreImpl) {}
    pub fn isVertTextflow(textflow: i32) -> bool { textflow != 0 }
}
pub struct TypoRectMemoryImpl;
impl TypoRectMemoryImpl {
    pub fn getTop__int_typosnapshot(pointer: i32, snapshot: &crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) -> i32 {
        snapshot.get__int(pointer + 1)
    }
}
pub struct DrawingAffectTool;
impl DrawingAffectTool {
    pub fn unionAffectRect(affectRect: &mut TypoRectCoreImpl, layoutRect: &TypoRectCoreImpl) {
        if affectRect.isEmpty() {
            affectRect.set(layoutRect);
        } else {
            affectRect.union(layoutRect);
        }
    }
    /// Java getAffectedPage for non-column/textbox pages: intersect core then first rollback line.
    pub fn getAffectedPage(curPage: &TypoPage, layoutRect: &TypoRectCoreImpl, _snapshot: &crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) -> i32 {
        if 14 == curPage.getType() || 6 == curPage.getType() {
            return if curPage.lines != 0 { curPage.pointer } else { 0 };
        }
        if !Self::isIntersectedWithPageCore(curPage, layoutRect) {
            return 0;
        }
        if curPage.lines == 0 {
            return 0;
        }
        curPage.pointer
    }
    fn isIntersectedWithPageCore(linesPage: &TypoPage, affectRect: &TypoRectCoreImpl) -> bool {
        if affectRect.isEmpty() {
            return false;
        }
        let lastLine = linesPage.getLines();
        let lastLineBottom = if 0 == lastLine { 1 } else { lastLine };
        let pageCoreHeight = linesPage.getCoreHeight().max(lastLineBottom);
        affectRect.intersects(0, 0, linesPage.getCoreWidth(), pageCoreHeight)
    }
}
#[derive(Default, Clone)]
pub struct PageEnv {
    pub marginLeft: i32,
    pub marginTop: i32,
    pub coreWidth: i32,
    pub height: i32,
}
impl PageEnv {
    pub fn getCoreWidth(&self) -> i32 { self.coreWidth }
}
pub trait IStateEnvoy {
    fn setMaxWrapDrawingCP(&mut self, var1: i32);
    fn getMaxWrapDrawingCP(&self) -> i32;
    fn getCurPage(&self) -> TypoPage;
    fn getAffectRect(&self) -> TypoRectCoreImpl;
    fn getAffectRectMut(&mut self) -> &mut TypoRectCoreImpl;
    fn getPageEnv(&self) -> PageEnv;
    fn getDocument(&self) -> Document { Document }
    fn getTablePeriLayer(&self) -> i32 { 0 }
    fn getTextFrameLayer(&self) -> i32 { 0 }
    fn getCurLayoutHeight(&self) -> i32 { 0 }
    fn getRangeStart(&self) -> i32 { 0 }
    fn getRangeEnd(&self) -> i32 { 0 }
    fn setFinishPage(&mut self, _var1: bool) {}
    fn setAffectRect(&mut self, rect: TypoRectCoreImpl) {
        *self.getAffectRectMut() = rect;
    }
}
#[derive(Default, Clone)]
pub struct Document;
#[derive(Default, Clone)]
pub struct RectF;
#[derive(Default, Clone)]
pub struct TableInfo;
#[derive(Default, Clone)]
pub struct TypoTextBox;

// java public class DrawingLayouter extends LayouterBase<DrawingLayouter.IStateEnvoy>
#[derive(Default)]
pub struct DrawingLayouter {
    // java LayouterBase<DrawingLayouter.IStateEnvoy> super_
    // rust: no-stmt
    // java protected DocumentEnv mDocEnv;
    pub(crate) mDocEnv: DocumentEnv,
    // java protected ViewEnv mViewEnv;
    pub(crate) mViewEnv: ViewEnv,
    // java protected MainState mMainState;
    pub mMainState: MainState,
    // java protected TextboxLayouter mTextboxLayouter;
    pub(crate) mTextboxLayouter: Option<TextboxLayouter>,
    // java protected TextboxLayoutState mTextboxLayoutState;
    // Java owns one TextboxLayoutState object; Box preserves its address for
    // TextboxStateEnvoy's back-reference once flowTextBox is enabled.
    pub(crate) mTextboxLayoutState: Option<Box<TextboxLayoutState>>,
    // java protected IGetTextWrapLayouter mGetTextWrapLayouter;
    pub(crate) mGetTextWrapLayouter: Option<IGetTextWrapLayouter>,
    // java protected ShapeCollision.NormalCollosion mNormalCollosion;
    pub(crate) mNormalCollosion: NormalCollosion,
    // java protected KList<ShapeCollision.ClipRect.RectFWrapImp> mSubLineRects;
    pub(crate) mSubLineRects: KList<RectFWrapImp>,
    // java final KList mFilterFloatCache = new KList();
    pub(crate) mFilterFloatCache: Vec<crate::cn::wps::moffice::writer::layout::base::env::DrawingCacheItem::DrawingCacheItem>,
    // Java constructor keeps the parent env objects by reference.
    mLiveMainState: *mut MainState,
    mLiveDocEnv: *mut DocumentEnv,
}

// java public class DrawingLayouter extends LayouterBase<DrawingLayouter.IStateEnvoy>
impl DrawingLayouter {
    pub fn bind_live_env(&mut self, main: *mut MainState, doc: *mut DocumentEnv) {
        self.mLiveMainState = main;
        self.mLiveDocEnv = doc;
    }
    // java public  DrawingLayouter(MainState mainState, DocumentEnv docEnv, ViewEnv viewEnv, IGetTextWrapLayouter getTextWraLayouter)
    pub fn DrawingLayouter(mainState: MainState, docEnv: DocumentEnv, viewEnv: ViewEnv, getTextWraLayouter: Option<IGetTextWrapLayouter>) -> Self {
      // java this.mDocEnv = docEnv;
      let mDocEnv = docEnv;
      // java this.mViewEnv = viewEnv;
      let mViewEnv = viewEnv;
      // java this.mMainState = mainState;
      let mMainState = mainState;
      // java this.mGetTextWrapLayouter = getTextWraLayouter;
      let mGetTextWrapLayouter = getTextWraLayouter;
      // java this.mNormalCollosion = new ShapeCollision.NormalCollosion(this.mMainState.snapshot.getTypoTools());
      let mNormalCollosion = NormalCollosion::new(TypoToolsManager::default());
      // java this.mSubLineRects = new KList();
      let mSubLineRects = KList::new();
      Self {
        mDocEnv,
        mViewEnv,
        mMainState,
        mTextboxLayouter: None,
        mTextboxLayoutState: None,
        mGetTextWrapLayouter,
        mNormalCollosion,
        mSubLineRects,
        mFilterFloatCache: Vec::new(),
        mLiveMainState: std::ptr::null_mut(),
        mLiveDocEnv: std::ptr::null_mut(),
      }
    }

    // java protected void onFlowWrapDrawing(IStateEnvoy stateEnvoy, int anchorCP)
    pub fn onFlowWrapDrawing(&self, stateEnvoy: &mut dyn IStateEnvoy, anchorCP: i32) {
      // java stateEnvoy.setMaxWrapDrawingCP(Math.max(stateEnvoy.getMaxWrapDrawingCP(), anchorCP));
      stateEnvoy.setMaxWrapDrawingCP((stateEnvoy.getMaxWrapDrawingCP()).max(anchorCP));
    }

    // java public void adjustTextFrame(TypoDrawing drawing, int paraTop, IStateEnvoy stateEnvoy)
    pub fn adjustTextFrame(&mut self, drawing: &mut TypoDrawing, paraTop: i32, stateEnvoy: &mut dyn IStateEnvoy) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java TypoSnapshot snapshot = this.mMainState.snapshot;
      let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
      // java IPos pos = drawing.getPos();
      let mut pos: Box<dyn IPos> = drawing.getPos();
      // java if (pos != null)
      if true {
        // java RectF shapeRect = pos.getShapeRect();
        let mut shapeRect: RectF = pos.getShapeRect();
        // java drawing.setLeft(ShapeMetrics.transToLayout(shapeRect.left));
        drawing.setLeft(ShapeMetrics::transToLayout__float(shapeRect.left));
        // java drawing.setTop(ShapeMetrics.transToLayout(shapeRect.top));
        drawing.setTop(ShapeMetrics::transToLayout__float(shapeRect.top));
        // java int pTextbox = drawing.getTextbox();
        let mut pTextbox: i32 = drawing.getTextbox();
        // java drawing.setWidth(TypoObjBase.width(pTextbox, snapshot));
        drawing.setWidth(TypoObjBase::width__int_typosnapshot(pTextbox, snapshot));
        // java drawing.setHeight(TypoObjBase.height(pTextbox, snapshot));
        drawing.setHeight(TypoObjBase::height__int_typosnapshot(pTextbox, snapshot));
        // java if (shapeRect.isEmpty())
        if shapeRect.is_empty() {
          // java shapeRect.right = shapeRect.left + ShapeMetrics.transFromLayout((float)drawing.width());
          shapeRect.right = shapeRect.left + ShapeMetrics::transFromLayout__float(((drawing.width()) as f32));
          // java shapeRect.bottom = shapeRect.top + ShapeMetrics.transFromLayout((float)drawing.height());
          shapeRect.bottom = shapeRect.top + ShapeMetrics::transFromLayout__float(((drawing.height()) as f32));
        }
        // java int globalPageIdx = this.mMainState.curPageIndex;
        let mut globalPageIdx: i32 = self.mMainState.curPageIndex;
        // java TypoPage floatsPage = this.getFloatContainerPage(stateEnvoy);
        let mut floatsPage: TypoPage = self.getFloatContainerPage({ let __x: &dyn IStateEnvoy = todo!("no align strategy matched"); __x });
        // java int relX = DrawingPositionTool.normalizeRelX(pos, floatsPage, globalPageIdx);
        let mut relX: i32 = DrawingPositionTool::normalizeRelX(pos, floatsPage, globalPageIdx);
        // java int relY = DrawingPositionTool.normalizeRelY(pos, floatsPage, globalPageIdx, this.mDocEnv.textDocument.getCompatibilityMode().isBefore2013Mode());
        let mut relY: i32 = DrawingPositionTool::normalizeRelY(pos, floatsPage, globalPageIdx, self.mDocEnv.textDocument.getCompatibilityMode().isBefore2013Mode());
        // java DropCap dropCap = (DropCap)stateEnvoy.getParaEnv().curParaPropEnv.paraProperty.getObject(256, (Object)null);
        let mut dropCap: DropCap = (stateEnvoy.getParaEnv().curParaPropEnv.paraProperty.getObject(256, None)).downcast_ref::<DropCap>().unwrap();
        // java boolean isDropCap = dropCap != null && dropCap.getType() != 0;
        let mut isDropCap: bool = true && dropCap.getType() != 0;
        // java drawing.setIsDropCap(isDropCap);
        drawing.setIsDropCap(isDropCap);
        // java TypoRectCoreImpl alignRect = TypoRectCoreImpl.obtain();
        let mut alignRect: TypoRectCoreImpl = TypoRectCoreImpl::obtain();
        // java this.calcAlignRect(drawing, alignRect, 0, 0, paraTop, relX, relY, true, true, stateEnvoy);
        self.calcAlignRect({ let __x: &TypoDrawing = todo!("no align strategy matched"); __x }, { let __x: &mut ITypoRect = todo!("no align strategy matched"); __x }, 0, 0, paraTop, relX, relY, true, true, stateEnvoy);
        // java TypoRectCoreImpl shapeRectBak = TypoRectCoreImpl.obtain();
        let mut shapeRectBak: TypoRectCoreImpl = TypoRectCoreImpl::obtain();
        // java ShapeMetrics.transToLayout(shapeRect, (Rect)shapeRectBak);
        ShapeMetrics::transToLayout__rectf_rect(shapeRect, (shapeRectBak).downcast_ref::<Rect>().unwrap());
        // java this.doOffsetDrawing(drawing, relX, relY, 0, shapeRectBak, alignRect, stateEnvoy);
        self.doOffsetDrawing(drawing, relX, relY, 0, { let __x: &mut ITypoRect = todo!("no align strategy matched"); __x }, { let __x: &ITypoRect = todo!("no align strategy matched"); __x }, { let __x: &dyn IStateEnvoy = todo!("no align strategy matched"); __x });
        // java DrawingResizeTool.setDrawingLayoutRectPaddingsWithIPos(drawing, pos);
        DrawingResizeTool::setDrawingLayoutRectPaddingsWithIPos(drawing, pos);
        // java if (isDropCap)
        if isDropCap {
          // java this.adjustDropCap(drawing, stateEnvoy, shapeRect, alignRect, relX, relY, pos.getAlignmentH(), pos.getAlignmentV());
          self.adjustDropCap(drawing, stateEnvoy, shapeRect, { let __x: &mut ITypoRect = todo!("no align strategy matched"); __x }, relX, relY, pos.getAlignmentH(), pos.getAlignmentV());
        }
        // java if (!isDropCap)
        if !isDropCap {
          // java this.correctIfWrapShapeOutOfPage(drawing, pos, stateEnvoy);
          self.correctIfWrapShapeOutOfPage(drawing, pos.as_ref(), stateEnvoy);
        }
        // java this.mNormalCollosion.adjust(drawing, stateEnvoy, this.mMainState);
        self.mNormalCollosion.adjust(drawing, stateEnvoy, self.mMainState);
        // java this.calcAffectRect(drawing, 0, stateEnvoy);
        self.calcAffectRect(drawing, 0, stateEnvoy);
        // java alignRect.recycle();
        alignRect.recycle();
        // java shapeRectBak.recycle();
        shapeRectBak.recycle();
      }
      */
    }

    // java private void adjustDropCap(TypoDrawing drawing, IStateEnvoy stateEnvoy, RectF shapeRect, ITypoRect alignRect, int relX, int relY, int alignX, int alignY)
    pub fn adjustDropCap(&mut self, drawing: &mut TypoDrawing, stateEnvoy: &mut dyn IStateEnvoy, shapeRect: RectF, alignRect: &mut ITypoRect, relX: i32, relY: i32, alignX: i32, alignY: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java PLCSection.SectionNode section = stateEnvoy.getDocument().getMainDocument().getPLCSection().getSection(drawing.getCP());
      let mut section: PLCSection::SectionNode = stateEnvoy.getDocument().getMainDocument().getPLCSection().getSection(drawing.getCP());
      // java PropertySet sectionProp = SectionTool.calcSectionPropertySet(this.mViewEnv, section);
      let mut sectionProp: PropertySet = SectionTool::calcSectionPropertySet(self.mViewEnv, section);
      // java int textFlow = sectionProp.getInt(714, DefaultValue.section_TextFlow);
      let mut textFlow: i32 = sectionProp.getInt(714, DefaultValue::section_TextFlow);
      // java if (TextFlow.isVertical(textFlow))
      if TextFlow::isVertical(textFlow) {
        // java this.adjustDropCapVertical(drawing, stateEnvoy, shapeRect, alignRect, relX, relY, alignX, alignY);
        self.adjustDropCapVertical(drawing, stateEnvoy, shapeRect, alignRect, relX, relY, alignX, alignY);
      }
      // java else
      else {
        // java this.adjustDropCapHorizontal(drawing, stateEnvoy, shapeRect, alignRect, relX, relY, alignX, alignY);
        self.adjustDropCapHorizontal(drawing, stateEnvoy, shapeRect, alignRect, relX, relY, alignX, alignY);
      }
      // java TypoSnapshot snapshot = this.mMainState.snapshot;
      let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
      // java int pDrawingList = this.mMainState.getDrawingLayers(stateEnvoy.getDocument()).getDrawins();
      let mut pDrawingList: i32 = self.mMainState.getDrawingLayers__document(stateEnvoy.getDocument()).getDrawins();
      // java if (pDrawingList != 0)
      if pDrawingList != 0 {
        // java TypoList typoList = snapshot.getTypoTools().obtainTypoList(pDrawingList);
        let mut typoList: TypoList = snapshot.getTypoTools().obtainTypoList(pDrawingList);
        // java int parentPage = this.getFloatContainerPage(stateEnvoy).getPointer();
        let mut parentPage: i32 = self.getFloatContainerPage({ let __x: &dyn IStateEnvoy = todo!("no align strategy matched"); __x }).getPointer();
        // java int pSelf = drawing.getPointer();
        let mut pSelf: i32 = drawing.getPointer();
        // java TypoRectCoreImpl pos1 = null;
        let mut pos1: TypoRectCoreImpl = Default::default();
        // java TypoRectCoreImpl pos2 = null;
        let mut pos2: TypoRectCoreImpl = Default::default();
        // java int size = typoList.size();
        let mut size: i32 = (typoList.len() as i32);
        let mut i: i32 = size - 1;
        // java for(int i = size - 1; i >= 0 --i)
        while i >= 0 {
          // java int pItem = typoList.quickGet(i);
          let mut pItem: i32 = typoList.quickGet(i);
          // java if (pItem != pSelf)
          if pItem != pSelf {
            // java if (TypoDrawing.getPage(pItem, snapshot) != parentPage)
            if TypoDrawing::getPage(pItem, snapshot) != parentPage {
              // java break;
              break;
            }
            // java if (pos1 == null)
            if false {
              // java pos1 = TypoRectCoreImpl.obtain();
              pos1 = TypoRectCoreImpl::obtain();
              // java pos1.set(drawing.getLeft(), drawing.getTop(), drawing.getRight(), drawing.getBottom());
              pos1.set(drawing.getLeft(), drawing.getTop(), drawing.getRight(), drawing.getBottom());
            }
            // java if (pos2 == null)
            if false {
              // java pos2 = TypoRectCoreImpl.obtain();
              pos2 = TypoRectCoreImpl::obtain();
            }
            // java pos2.set(TypoObjBase.getLeft(pItem, snapshot), TypoObjBase.getTop(pItem, snapshot), TypoObjBase.getRight(pItem, snapshot), TypoObjBase.getBottom(pItem, snapshot));
            pos2.set(TypoObjBase::getLeft__int_typosnapshot(pItem, snapshot), TypoObjBase::getTop__int_typosnapshot(pItem, snapshot), TypoObjBase::getRight__int_typosnapshot(pItem, snapshot), TypoObjBase::getBottom__int_typosnapshot(pItem, snapshot));
            // java if (!Rect.intersects(pos1, pos2))
            if !Rect::intersects__rect_rect(pos1, pos2) {
              // java break;
              break;
            }
            // java if (TypoDrawing.isDropCap(pItem, snapshot))
            if TypoDrawing::isDropCap(pItem, snapshot) {
              // java if (drawing.getTop() < TypoDrawing.getBottom(pItem, snapshot))
              if drawing.getTop() < TypoDrawing::getBottom(pItem, snapshot) {
                // java drawing.offsetYTo(TypoDrawing.getBottom(pItem, snapshot));
                drawing.offsetYTo(TypoDrawing::getBottom(pItem, snapshot));
              }
              // java break;
              break;
            }
          }
        }
        // java if (pos1 != null)
        if true {
          // java pos1.recycle();
          pos1.recycle();
        }
        // java if (pos2 != null)
        if true {
          // java pos2.recycle();
          pos2.recycle();
        }
        // java snapshot.getTypoTools().recycle((TypoMemoryObj)typoList);
        snapshot.getTypoTools().recycle((typoList).downcast_ref::<TypoMemoryObj>().unwrap());
      }
      */
    }

    // java private void adjustDropCapHorizontal(TypoDrawing drawing, IStateEnvoy stateEnvoy, RectF shapeRect, ITypoRect alignRect, int relX, int relY, int alignX, int alignY)
    pub fn adjustDropCapHorizontal(&mut self, drawing: &mut TypoDrawing, stateEnvoy: &mut dyn IStateEnvoy, shapeRect: RectF, alignRect: &mut ITypoRect, relX: i32, relY: i32, alignX: i32, alignY: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java DropCap dropCap = (DropCap)stateEnvoy.getParaEnv().curParaPropEnv.paraProperty.getObject(256, (Object)null);
      let mut dropCap: DropCap = (stateEnvoy.getParaEnv().curParaPropEnv.paraProperty.getObject(256, None)).downcast_ref::<DropCap>().unwrap();
      // java if (dropCap != null && shapeRect.left == 0.0F)
      if true && shapeRect.left == 0 {
        // java boolean isAdjustmentIgnored = relY == 2 && alignY != 0;
        let mut isAdjustmentIgnored: bool = relY == 2 && alignY != 0;
        // java if (dropCap.getType() != 2 || relX != 4 && relX != 5)
        if dropCap.getType() != 2 || relX != 4 && relX != 5 {
          // java if (dropCap.getType() == 1)
          if dropCap.getType() == 1 {
            // java int page = this.getLineContainerPage(drawing, 0, stateEnvoy);
            let mut page: i32 = self.getLineContainerPage({ let __x: &TypoDrawing = todo!("no align strategy matched"); __x }, 0, { let __x: &dyn IStateEnvoy = todo!("no align strategy matched"); __x });
            // java if (isAdjustmentIgnored || TypoMemoryObj.getType(page, this.mMainState.snapshot) != 14)
            if isAdjustmentIgnored || TypoMemoryObj::getType__int_typosnapshot(page, self.mMainState.snapshot) != 14 {
              // java return;
              return;
            }
            // java int columnsPointer = TypoPage.getParent(page, this.mMainState.snapshot);
            let mut columnsPointer: i32 = TypoPage::getParent(page, self.mMainState.snapshot);
            // java int columnIndex = TypoColumns.indexOf(page, columnsPointer, this.mMainState.snapshot);
            let mut columnIndex: i32 = TypoColumns::indexOf(page, columnsPointer, self.mMainState.snapshot);
            // java if (columnIndex <= 0)
            if columnIndex <= 0 {
              // java return;
              return;
            }
            // java TypoRectMemoryImpl paddingRect = this.mMainState.snapshot.getTypoTools().obtainMemoryRect(drawing.getLayoutPadding());
            let mut paddingRect: TypoRectMemoryImpl = self.mMainState.snapshot.getTypoTools().obtainMemoryRect(drawing.getLayoutPadding());
            // java int paddingHorizontal = paddingRect.getLeft();
            let mut paddingHorizontal: i32 = paddingRect.getLeft();
            // java int prevColumnPointer = TypoColumns.getColumnPage(columnIndex - 1, columnsPointer, this.mMainState.snapshot);
            let mut prevColumnPointer: i32 = TypoColumns::getColumnPage__int_int_typosnapshot(columnIndex - 1, columnsPointer, self.mMainState.snapshot);
            // java int maxSpace = TypoColumnPage.getMarginLeft(page, this.mMainState.snapshot) + TypoColumnPage.getMarginRight(prevColumnPointer, this.mMainState.snapshot);
            let mut maxSpace: i32 = TypoColumnPage::getMarginLeft(page, self.mMainState.snapshot) + TypoColumnPage::getMarginRight(prevColumnPointer, self.mMainState.snapshot);
            // java if (paddingHorizontal > maxSpace)
            if paddingHorizontal > maxSpace {
              // java drawing.offsetX(paddingHorizontal - maxSpace);
              drawing.offsetX(paddingHorizontal - maxSpace);
            }
            // java paddingRect.recycle();
            paddingRect.recycle();
          }
        }
        // java else
        else {
          // java TypoRectMemoryImpl paddingRect = this.mMainState.snapshot.getTypoTools().obtainMemoryRect(drawing.getLayoutPadding());
          let mut paddingRect: TypoRectMemoryImpl = self.mMainState.snapshot.getTypoTools().obtainMemoryRect(drawing.getLayoutPadding());
          // java int paddingHorizontal = paddingRect.getLeft();
          let mut paddingHorizontal: i32 = paddingRect.getLeft();
          // java int maxOffset = alignRect.width() - drawing.width();
          let mut maxOffset: i32 = alignRect.width() - drawing.width();
          // java boolean alignRight = relX == 4;
          let mut alignRight: bool = relX == 4;
          // java boolean offsetRight = !alignRight;
          let mut offsetRight: bool = !alignRight;
          // java if (paddingHorizontal < maxOffset)
          if paddingHorizontal < maxOffset {
            // java if (offsetRight)
            if offsetRight {
              // java drawing.offset(paddingHorizontal, 0);
              drawing.offset(paddingHorizontal, 0);
            }
            // java else
            else {
              // java drawing.offset(-paddingHorizontal, 0);
              drawing.offset(-paddingHorizontal, 0);
            }
            // java TypoRectMemoryImpl layoutRectPaddings = this.mMainState.snapshot.getTypoTools().obtainMemoryRect(drawing.getLayoutPadding());
            let mut layoutRectPaddings: TypoRectMemoryImpl = self.mMainState.snapshot.getTypoTools().obtainMemoryRect(drawing.getLayoutPadding());
            // java layoutRectPaddings.setLeft(0);
            layoutRectPaddings.setLeft(0);
            // java layoutRectPaddings.setRight(0);
            layoutRectPaddings.setRight(0);
            // java layoutRectPaddings.recycle();
            layoutRectPaddings.recycle();
          }
          // java else
          else {
            // java int page = this.getLineContainerPage(drawing, 0, stateEnvoy);
            let mut page: i32 = self.getLineContainerPage({ let __x: &TypoDrawing = todo!("no align strategy matched"); __x }, 0, { let __x: &dyn IStateEnvoy = todo!("no align strategy matched"); __x });
            // java if (isAdjustmentIgnored || TypoMemoryObj.getType(page, this.mMainState.snapshot) != 14)
            if isAdjustmentIgnored || TypoMemoryObj::getType__int_typosnapshot(page, self.mMainState.snapshot) != 14 {
              // java this.offsetMarginToEdge(drawing, alignRect, maxOffset, offsetRight, true);
              self.offsetMarginToEdge(drawing, { let __x: &ITypoRect = todo!("no align strategy matched"); __x }, maxOffset, offsetRight, true);
            }
            // java int columnsPointer = TypoPage.getParent(page, this.mMainState.snapshot);
            let mut columnsPointer: i32 = TypoPage::getParent(page, self.mMainState.snapshot);
            // java int columnIndex = TypoColumns.indexOf(page, columnsPointer, this.mMainState.snapshot);
            let mut columnIndex: i32 = TypoColumns::indexOf(page, columnsPointer, self.mMainState.snapshot);
            // java if (columnIndex == 0)
            if columnIndex == 0 {
              // java this.offsetMarginToEdge(drawing, alignRect, maxOffset, offsetRight, true);
              self.offsetMarginToEdge(drawing, { let __x: &ITypoRect = todo!("no align strategy matched"); __x }, maxOffset, offsetRight, true);
            }
            // java else if (columnIndex > 0)
            else if columnIndex > 0 {
              // java TypoToolsManager typoToolsManager = this.mMainState.snapshot.getTypoTools();
              let mut typoToolsManager: TypoToolsManager = self.mMainState.snapshot.getTypoTools();
              // java TypoColumnPage columnPage = typoToolsManager.obtainTypoColumnPage(page);
              let mut columnPage: TypoColumnPage = typoToolsManager.obtainTypoColumnPage(page);
              // java TypoRect columnPageRect = typoToolsManager.obtainRect();
              let mut columnPageRect: TypoRect = typoToolsManager.obtainRect();
              // java columnPage.rectForPageCore(columnPageRect);
              columnPage.rectForPageCore(columnPageRect);
              // java drawing.offsetXTo(columnPageRect.getLeft());
              drawing.offsetXTo(columnPageRect.getLeft());
              // java int prevColumnPointer = TypoColumns.getColumnPage(columnIndex - 1, columnsPointer, this.mMainState.snapshot);
              let mut prevColumnPointer: i32 = TypoColumns::getColumnPage__int_int_typosnapshot(columnIndex - 1, columnsPointer, self.mMainState.snapshot);
              // java int maxSpace = TypoColumnPage.getMarginRight(prevColumnPointer, this.mMainState.snapshot);
              let mut maxSpace: i32 = TypoColumnPage::getMarginRight(prevColumnPointer, self.mMainState.snapshot);
              // java int restSpace = maxSpace - paddingHorizontal;
              let mut restSpace: i32 = maxSpace - paddingHorizontal;
              // java if (restSpace < 0)
              if restSpace < 0 {
                // java drawing.offsetX(-restSpace);
                drawing.offsetX(-restSpace);
              }
              // java typoToolsManager.recycle(columnPageRect);
              typoToolsManager.recycle(columnPageRect);
              // java typoToolsManager.recycle((TypoMemoryObj)columnPage);
              typoToolsManager.recycle((columnPage).downcast_ref::<TypoMemoryObj>().unwrap());
            }
          }
        }
      }
      */
    }

    // java private void adjustDropCapVertical(TypoDrawing drawing, IStateEnvoy stateEnvoy, RectF shapeRect, ITypoRect alignRect, int relX, int relY, int alignX, int alignY)
    pub fn adjustDropCapVertical(&mut self, drawing: &mut TypoDrawing, stateEnvoy: &mut dyn IStateEnvoy, shapeRect: RectF, alignRect: &mut ITypoRect, relX: i32, relY: i32, alignX: i32, alignY: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java DropCap dropCap = (DropCap)stateEnvoy.getParaEnv().curParaPropEnv.paraProperty.getObject(256, (Object)null);
      let mut dropCap: DropCap = (stateEnvoy.getParaEnv().curParaPropEnv.paraProperty.getObject(256, None)).downcast_ref::<DropCap>().unwrap();
      // java if (dropCap != null && shapeRect.top == 0.0F && dropCap.getType() == 2 && (relY == 4 || relY == 5))
      if true && shapeRect.top == 0 && dropCap.getType() == 2 && relY == 4 || relY == 5 {
        // java TypoRectMemoryImpl paddingRect = this.mMainState.snapshot.getTypoTools().obtainMemoryRect(drawing.getLayoutPadding());
        let mut paddingRect: TypoRectMemoryImpl = self.mMainState.snapshot.getTypoTools().obtainMemoryRect(drawing.getLayoutPadding());
        // java int paddingVertical = paddingRect.getTop();
        let mut paddingVertical: i32 = paddingRect.getTop();
        // java int maxOffset = alignRect.height() - drawing.height();
        let mut maxOffset: i32 = alignRect.height() - drawing.height();
        // java boolean alignBottom = relY == 4;
        let mut alignBottom: bool = relY == 4;
        // java boolean offsetBottom = !alignBottom;
        let mut offsetBottom: bool = !alignBottom;
        // java if (paddingVertical < maxOffset)
        if paddingVertical < maxOffset {
          // java if (offsetBottom)
          if offsetBottom {
            // java drawing.offset(0, paddingVertical);
            drawing.offset(0, paddingVertical);
          }
          // java else
          else {
            // java drawing.offset(0, -paddingVertical);
            drawing.offset(0, -paddingVertical);
          }
          // java TypoRectMemoryImpl layoutRectPaddings = this.mMainState.snapshot.getTypoTools().obtainMemoryRect(drawing.getLayoutPadding());
          let mut layoutRectPaddings: TypoRectMemoryImpl = self.mMainState.snapshot.getTypoTools().obtainMemoryRect(drawing.getLayoutPadding());
          // java layoutRectPaddings.setLeft(0);
          layoutRectPaddings.setLeft(0);
          // java layoutRectPaddings.setRight(0);
          layoutRectPaddings.setRight(0);
          // java layoutRectPaddings.recycle();
          layoutRectPaddings.recycle();
        }
        // java else
        else {
          // java this.offsetMarginToEdge(drawing, alignRect, maxOffset, offsetBottom, false);
          self.offsetMarginToEdge(drawing, { let __x: &ITypoRect = todo!("no align strategy matched"); __x }, maxOffset, offsetBottom, false);
        }
      }
      */
    }

    // java private void offsetMarginToEdge(TypoDrawing drawing, ITypoRect alignRect, int maxOffset, boolean offsetOpposite, boolean isHorizontal)
    pub fn offsetMarginToEdge(&self, drawing: &mut TypoDrawing, alignRect: &ITypoRect, maxOffset: i32, offsetOpposite: bool, isHorizontal: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (offsetOpposite)
      if offsetOpposite {
        // java if (isHorizontal)
        if isHorizontal {
          // java drawing.offsetXTo(alignRect.getLeft() + maxOffset);
          drawing.offsetXTo(alignRect.getLeft() + maxOffset);
        }
        // java else
        else {
          // java drawing.offsetYTo(alignRect.getTop() + maxOffset);
          drawing.offsetYTo(alignRect.getTop() + maxOffset);
        }
      }
      // java else if (isHorizontal)
      else if isHorizontal {
        // java drawing.offsetXTo(alignRect.getLeft());
        drawing.offsetXTo(alignRect.getLeft());
      }
      // java else
      else {
        // java drawing.offsetYTo(alignRect.getTop());
        drawing.offsetYTo(alignRect.getTop());
      }
      */
    }

    // java public void adjustWrapTable(TypoDrawing drawing, TableInfo tableInfo, int paraTop, boolean needCutTable, IStateEnvoy stateEnvoy)
    // The live page-view adapter uses the same committed cache drawing and
    // TableInfo as TableLayouter. The observed relX=0 / relY=1 / center branch
    // follows Java calcAlignRect + doOffsetDrawing exactly; other branches
    // remain at their prior position until their dependency methods are live.
    pub fn adjustWrapTable(
        &mut self,
        drawing: &mut crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing,
        tableInfo: &crate::cn::wps::moffice::writer::layout::base::table::TableLayouter::TableInfo,
        _paraTop: i32,
        _needCutTable: bool,
        stateEnvoy: &mut crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState,
    ) {
        let Some(pos) = tableInfo.getPos() else { return; };
        let main = if self.mLiveMainState.is_null() { &self.mMainState } else { unsafe { &*self.mLiveMainState } };
        let doc = if self.mLiveDocEnv.is_null() { &mut self.mDocEnv } else { unsafe { &mut *self.mLiveDocEnv } };
        let page_index = main.curPageIndex;
        let rel_x = DrawingPositionTool::normalizeRelX__int_int(pos.getRelativeX(), page_index);
        let rel_y = DrawingPositionTool::normalizeRelY__int_int_boolean(
            pos.getRelativeY(), page_index,
            doc.textDocument_mut().map(|doc| doc.getCompatibilityMode().isBefore2013Mode()).unwrap_or(true),
        );
        // Java stores IPos before horizontal/vertical placement. Keep it on
        // the snapshot even when a placement branch has not been ported yet.
        if let Some(table_pos) = tableInfo.pos.as_ref() {
            drawing.setPos(table_pos.clone());
        }
        if (rel_x, rel_y, pos.getAlignmentH(), pos.getAlignmentV()) != (0, 1, 2, 0) { return; }
        let shape_rect = pos.getShapeRect();
        let shape_x = crate::cn::wps::moffice::writer::core::shape::ShapeMetrics::ShapeMetrics::transToLayout__float(shape_rect.left);
        let shape_y = crate::cn::wps::moffice::writer::core::shape::ShapeMetrics::ShapeMetrics::transToLayout__float(shape_rect.top);
        drawing.offsetTo(shape_x, shape_y);
        let shape = ITypoRect { left: shape_x, top: shape_y, right: shape_x + drawing.width(), bottom: shape_y + drawing.height() };
        let align = ITypoRect { left: 0, top: -stateEnvoy.pageEnv.marginTop, right: stateEnvoy.pageEnv.getCoreWidth(), bottom: stateEnvoy.pageEnv.height - stateEnvoy.pageEnv.marginTop };
        let x = DrawingPositionTool::getDrawingLeftOfPageCore(pos.getAlignmentH(), page_index, &shape, &align);
        let y = DrawingPositionTool::getDrawingTopOfPageCore(rel_y, pos.getAlignmentV(), page_index, &shape, &align);
        drawing.offsetTo(x, y);
        if std::env::var_os("WORD_PARITY_TABLE_DRAW_TRACE").is_some() {
            eprintln!("table_pos after cp={} box={},{},{},{} basis={},{},{},{} rel={},{}", tableInfo.getStartCP(),
                drawing.getLeft(), drawing.getTop(), drawing.getRight(), drawing.getBottom(),
                align.left, align.top, align.right, align.bottom, rel_x, rel_y);
        }
    }

    // java protected IPos getTablePos(TableInfo tableInfo)
    pub fn getTablePos(&self, tableInfo: &TableInfo) -> Option<IPos> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java IPos pos = tableInfo.getPos();
      let mut pos: Box<dyn IPos> = tableInfo.getPos();
      // java if (null == pos)
      if false {
        // java pos = new TablePos();
        pos = TablePos::new();
        // java pos.setShapeRect(new RectF());
        pos.setShapeRect(RectF::new());
        // java pos.setAlignmentH(0);
        pos.setAlignmentH(0);
        // java pos.setAlignmentV(0);
        pos.setAlignmentV(0);
        // java pos.setRelativeH(2);
        pos.setRelativeH(2);
        // java pos.setRelativeV(2);
        pos.setRelativeV(2);
        // java pos.setTextWrap(1);
        pos.setTextWrap(1);
        // java pos.setAllowOverlap(false);
        pos.setAllowOverlap(false);
        // java pos.setWrapDistBottom(0.0F);
        pos.setWrapDistBottom(0);
        // java pos.setWrapDistLeft(0.0F);
        pos.setWrapDistLeft(0);
        // java pos.setWrapDistRight(0.0F);
        pos.setWrapDistRight(0);
        // java pos.setWrapDistTop(0.0F);
        pos.setWrapDistTop(0);
      }
      // java return pos;
      return { let __x: Option<IPos> = todo!("no align strategy matched"); __x };
      */
    }

    // java public void correctForCellMargin(TypoDrawing drawing, TableInfo tableInfo, IStateEnvoy stateEnvoy)
    pub fn correctForCellMargin(&self, drawing: &mut TypoDrawing, tableInfo: &TableInfo, stateEnvoy: &dyn IStateEnvoy) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (stateEnvoy.getDocument().getTextDocument().getCompatibilityMode().isTableIndentedByOldStyle())
      if stateEnvoy.getDocument().getTextDocument().getCompatibilityMode().isTableIndentedByOldStyle() {
        // java RowInfo firstRow = tableInfo.getRowByIndex(0);
        let mut firstRow: RowInfo = tableInfo.getRowByIndex(0);
        // java if (null != firstRow)
        if true {
          // java CellInfo firstCell = firstRow.getCellByIndex(0);
          let mut firstCell: CellInfo = firstRow.getCellByIndex(0);
          // java if (null != firstCell)
          if true {
            // java int marginLeft = firstCell.getMarginLeft();
            let mut marginLeft: i32 = firstCell.getMarginLeft();
            // java int marginRight = firstCell.getMarginRight();
            let mut marginRight: i32 = firstCell.getMarginRight();
            // java int cellCount = firstRow.size();
            let mut cellCount: i32 = (firstRow.len() as i32);
            // java if (cellCount > 1)
            if cellCount > 1 {
              // java CellInfo lastCell = firstRow.getCellByIndex(cellCount - 1);
              let mut lastCell: CellInfo = firstRow.getCellByIndex(cellCount - 1);
              // java marginRight = lastCell.getMarginRight();
              marginRight = lastCell.getMarginRight();
            }
            // java IPos pos = drawing.getPos();
            let mut pos: Box<dyn IPos> = drawing.getPos();
            // java if (pos != null)
            if true {
              // java switch (pos.getAlignmentH())
              match pos.getAlignmentH() {
                // java case 0: case 1: case 4: default:
                0 | 1 | 4 | _ => {
                  // java drawing.offset(-marginLeft, 0);
                  drawing.offset(-marginLeft, 0);
                }
                // java case 2:
                2 => {
                  // java drawing.offset((-marginLeft + marginRight) / 2, 0);
                  drawing.offset(-marginLeft + marginRight / 2, 0);
                }
                // java case 3: case 5:
                3 | 5 => {
                  // java drawing.offset(marginRight, 0);
                  drawing.offset(marginRight, 0);
                }
              }
            }
          }
        }
      }
      */
    }

    // java protected void onStart(IStateEnvoy stateEnvoy)
    pub fn onStart(&self, stateEnvoy: &dyn IStateEnvoy) {
      // rust: no-stmt
    }

    // java protected void onEnd(IStateEnvoy stateEnvoy)
    pub fn onEnd(&self, stateEnvoy: &dyn IStateEnvoy) {
      // rust: no-stmt
    }

    // java protected boolean correctIfWrapTableOutOfPage(TypoDrawing drawing, IPos pos, boolean needCutTable, IStateEnvoy stateEnvoy)
    pub fn correctIfWrapTableOutOfPage(&mut self, drawing: &mut TypoDrawing, pos: Option<&IPos>, needCutTable: bool, stateEnvoy: &mut dyn IStateEnvoy) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java boolean hasCorrected = false;
      let mut hasCorrected: bool = false;
      // java if (!DrawingPositionTool.isWrap(pos))
      if !DrawingPositionTool::isWrap(pos) {
        // java return hasCorrected;
        return hasCorrected;
      }
      // java else
      else {
        // java TypoRectCoreImpl limitRect = TypoRectCoreImpl.obtain();
        let mut limitRect: TypoRectCoreImpl = TypoRectCoreImpl::obtain();
        // java this.calcLimitRect(pos, limitRect, stateEnvoy);
        self.calcLimitRect(pos, { let __x: &mut ITypoRect = todo!("no align strategy matched"); __x }, { let __x: &dyn IStateEnvoy = todo!("no align strategy matched"); __x });
        // java if (drawing.getTop() < limitRect.top)
        if drawing.getTop() < limitRect.top {
          // java int orgDrawingLeft = drawing.getLeft();
          let mut orgDrawingLeft: i32 = drawing.getLeft();
          // java drawing.offsetTo(orgDrawingLeft, limitRect.top);
          drawing.offsetTo(orgDrawingLeft, limitRect.top);
          // java hasCorrected = true;
          hasCorrected = true;
        }
        // java if (needCutTable)
        if needCutTable {
          // java int firstLine = TypoList.first(drawing.getTextBoxLines(), this.mMainState.snapshot);
          let mut firstLine: i32 = TypoList::first__int_typosnapshot(drawing.getTextBoxLines(), self.mMainState.snapshot);
          // java if (firstLine != 0)
          if firstLine != 0 {
            // java int minHeight = TypoObjBase.height(firstLine, this.mMainState.snapshot);
            let mut minHeight: i32 = TypoObjBase::height__int_typosnapshot(firstLine, self.mMainState.snapshot);
            // java int minTop = limitRect.bottom - minHeight;
            let mut minTop: i32 = limitRect.bottom - minHeight;
            // java if (drawing.getTop() > minTop)
            if drawing.getTop() > minTop {
              // java int orgDrawingLeft = drawing.getLeft();
              let mut orgDrawingLeft: i32 = drawing.getLeft();
              // java drawing.offsetTo(orgDrawingLeft, minTop);
              drawing.offsetTo(orgDrawingLeft, minTop);
              // java hasCorrected = true;
              hasCorrected = true;
            }
          }
        }
        // java else
        else {
          // java boolean needCorrectHori = stateEnvoy.getTablePeriLayer() > 1;
          let mut needCorrectHori: bool = stateEnvoy.getTablePeriLayer() > 1;
          // java hasCorrected |= DrawingPositionTool.correctWrapDrawing(drawing, limitRect, needCorrectHori, true);
          hasCorrected = hasCorrected | DrawingPositionTool::correctWrapDrawing(drawing, limitRect, needCorrectHori, true);
        }
        // java limitRect.recycle();
        limitRect.recycle();
        // java return hasCorrected;
        return hasCorrected;
      }
      */
    }

    // java protected void correctIfWrapShapeOutOfPage(TypoDrawing drawing, IPos pos, IStateEnvoy stateEnvoy)
    pub fn correctIfWrapShapeOutOfPage(&mut self, drawing: &mut TypoDrawing, pos: Option<&IPos>, stateEnvoy: &mut dyn IStateEnvoy) {
      // java if (DrawingPositionTool.shouldCorrectDrawing(drawing))
      if DrawingPositionTool::shouldCorrectDrawing(drawing) {
        // java TypoRectCoreImpl limitRect = TypoRectCoreImpl.obtain();
        let mut limitRect: TypoRectCoreImpl = TypoRectCoreImpl::obtain();
        // java this.calcLimitRect(pos, limitRect, stateEnvoy);
        let mut limit_as_typo = ITypoRect {
          left: limitRect.left,
          top: limitRect.top,
          right: limitRect.right,
          bottom: limitRect.bottom,
        };
        self.calcLimitRect(pos, &mut limit_as_typo, stateEnvoy);
        limitRect.left = limit_as_typo.left;
        limitRect.top = limit_as_typo.top;
        limitRect.right = limit_as_typo.right;
        limitRect.bottom = limit_as_typo.bottom;
        // java DrawingPositionTool.correctWrapDrawing(drawing, limitRect, true, true);
        DrawingPositionTool::correctWrapDrawing(drawing, &limitRect, true, true);
        // java limitRect.recycle();
        limitRect.recycle();
      }
    }

    // java protected void adjustFloatRect(TypoDrawing drawing, int anchorLine, int paraLine, int relX, int relY, IStateEnvoy stateEnvoy)
    pub fn adjustFloatRect(&mut self, drawing: &mut TypoDrawing, anchorLine: i32, paraLine: i32, relX: i32, relY: i32, stateEnvoy: &mut dyn IStateEnvoy) {
      // java TypoSnapshot snapshot = this.mMainState.snapshot;
      let _snapshot = &self.mMainState.snapshot;
      // java TypoToolsManager typoToolsManager = snapshot.getTypoTools();
      let mut typoToolsManager: TypoToolsManager = TypoToolsManager::default();
      // java IPos pos = drawing.getPos();
      let pos: Option<IPos> = drawing.getPos();
      // java TypoRect shapeRt = typoToolsManager.obtainRect();
      let mut shapeRt: TypoRect = typoToolsManager.obtainRect();
      // java shapeRt.set(drawing);
      shapeRt.set(drawing);
      // java TypoRect alignRect = typoToolsManager.obtainRect();
      let mut alignRect: TypoRect = typoToolsManager.obtainRect();
      // java this.calcAlignRect(drawing, alignRect, anchorLine, paraLine, 0, relX, relY, true, true, stateEnvoy);
      let mut align_i = ITypoRect::from_typo(&alignRect);
      self.calcAlignRect(drawing, &mut align_i, anchorLine, paraLine, 0, relX, relY, true, true, stateEnvoy);
      alignRect.left = align_i.left;
      alignRect.top = align_i.top;
      alignRect.right = align_i.right;
      alignRect.bottom = align_i.bottom;
      // java this.doOffsetDrawing(drawing, relX, relY, anchorLine, shapeRt, alignRect, stateEnvoy);
      let mut shape_i = ITypoRect::from_typo(&shapeRt);
      self.doOffsetDrawing(drawing, relX, relY, anchorLine, &mut shape_i, &align_i, stateEnvoy);
      // java this.correctIfWrapShapeOutOfPage(drawing, pos, stateEnvoy);
      self.correctIfWrapShapeOutOfPage(drawing, pos.as_ref(), stateEnvoy);
      // java this.mNormalCollosion.adjust(drawing, stateEnvoy, this.mMainState);
      self.mNormalCollosion.adjust(drawing, stateEnvoy, &self.mMainState);
      // java if (drawing.isWrap())
      if drawing.isWrap() {
        // java this.calcAffectRect(drawing, anchorLine, stateEnvoy);
        self.calcAffectRect(drawing, anchorLine, stateEnvoy);
      }
      // java typoToolsManager.recycle(shapeRt);
      typoToolsManager.recycle(shapeRt);
      // java typoToolsManager.recycle(alignRect);
      typoToolsManager.recycle(alignRect);
    }

    // java protected void doOffsetDrawing(TypoDrawing drawing, int relX, int relY, int anchorLine, ITypoRect shapeRect, ITypoRect alignRect, IStateEnvoy stateEnvoy)
    pub fn doOffsetDrawing(&self, drawing: &mut TypoDrawing, relX: i32, relY: i32, anchorLine: i32, shapeRect: &mut ITypoRect, alignRect: &ITypoRect, stateEnvoy: &dyn IStateEnvoy) {
      // java IPos pos = drawing.getPos();
      let pos: Option<IPos> = drawing.getPos();
      // java int alignH = 0;
      let mut alignH: i32 = 0;
      // java int globalPageIdx = this.mMainState.curPageIndex;
      let mut globalPageIdx: i32 = self.mMainState.curPageIndex;
      // java TypoPage floatsPage = this.getFloatContainerPage(stateEnvoy);
      let mut floatsPage: TypoPage = self.getFloatContainerPage(stateEnvoy);
      // java int alignV = 0;
      let mut alignV: i32 = 0;
      // java if (pos != null)
      if let Some(ref pos) = pos {
        // java alignV = pos.getAlignmentV();
        alignV = pos.getAlignmentV();
        // java alignH = pos.getAlignmentH();
        alignH = pos.getAlignmentH();
      }
      // java if (6 == floatsPage.getType() && drawing.isTable())
      if 6 == floatsPage.getType() && drawing.isTable() {
        // java alignV = 0;
        alignV = 0;
      }
      // java ITypoRect shapeRectArg = shapeRect;
      let mut shapeRectArg: ITypoRect = shapeRect.clone();
      let mut used_render = false;
      // java if (DrawingPositionTool.shouldOffsetDrawingUsingRenderRect(drawing, alignH, alignV))
      if DrawingPositionTool::shouldOffsetDrawingUsingRenderRect__typodrawing_int_int(drawing, alignH, alignV) {
        // java TypoRectCoreImpl renderRect = TypoRectCoreImpl.obtain();
        let mut renderRect: TypoRectCoreImpl = TypoRectCoreImpl::obtain();
        // java DrawingCoordTool.renderRectForPageCore(drawing, renderRect);
        DrawingCoordTool::renderRectForPageCore(drawing, &mut renderRect);
        // java shapeRectArg = renderRect;
        shapeRectArg = ITypoRect { left: renderRect.left, top: renderRect.top, right: renderRect.right, bottom: renderRect.bottom };
        used_render = true;
        // java int shapeRectLeft = drawing.getLeft();
        let mut shapeRectLeft: i32 = drawing.getLeft();
        // java int shapeRectTop = drawing.getTop();
        let mut shapeRectTop: i32 = drawing.getTop();
        // java int leftPadding = shapeRectLeft - renderRect.left;
        let mut leftPadding: i32 = shapeRectLeft - renderRect.left;
        // java int topPadding = shapeRectTop - renderRect.top;
        let mut topPadding: i32 = shapeRectTop - renderRect.top;
        // java drawing.offsetTo(leftPadding, topPadding);
        drawing.offsetTo(leftPadding, topPadding);
        // java renderRect.offsetTo(shapeRectLeft, shapeRectTop);
        renderRect.offsetTo(shapeRectLeft, shapeRectTop);
      }
      // java else
      else {
        // java drawing.offsetTo(0, 0);
        drawing.offsetTo(0, 0);
      }
      // java if (DrawingPositionTool.shouldReverseShapeRectLeft(drawing, relX, floatsPage))
      if DrawingPositionTool::shouldReverseShapeRectLeft(drawing, relX, &floatsPage, &self.mMainState.snapshot) {
        // java if (shapeRectArg != shapeRect)
        if shapeRectArg != *shapeRect {
          // java shapeRectArg = TypoRectCoreImpl.obtain();
          shapeRectArg = shapeRect.clone();
          // java shapeRectArg.set(shapeRect);
          shapeRectArg = shapeRect.clone();
        }
        // java int orgWidth = shapeRect.width();
        let mut orgWidth: i32 = shapeRect.width();
        // java shapeRectArg.setLeft(-shapeRect.getLeft());
        shapeRectArg.setLeft(-shapeRect.getLeft());
        // java shapeRectArg.setWidth(orgWidth);
        shapeRectArg.setWidth(orgWidth);
      }
      // java int left = DrawingPositionTool.getDrawingLeftOfPageCore(alignH, globalPageIdx, shapeRectArg, alignRect);
      let mut left: i32 = DrawingPositionTool::getDrawingLeftOfPageCore(alignH, globalPageIdx, &shapeRectArg, alignRect);
      // java int top = DrawingPositionTool.getDrawingTopOfPageCore(relY, alignV, globalPageIdx, shapeRectArg, alignRect);
      let mut top: i32 = DrawingPositionTool::getDrawingTopOfPageCore(relY, alignV, globalPageIdx, &shapeRectArg, alignRect);
      // java drawing.offset(left, top);
      drawing.offset(left, top);
      // java if (drawing.isWrap() && (2 == relY || 3 == relY))
      if drawing.isWrap() && (2 == relY || 3 == relY) {
        // java int layoutPadingTop = TypoRectMemoryImpl.getTop(drawing.getLayoutPadding(), this.mMainState.snapshot);
        let mut layoutPadingTop: i32 = TypoRectMemoryImpl::getTop__int_typosnapshot(drawing.getLayoutPadding(), &self.mMainState.snapshot);
        // java if (layoutPadingTop > 0)
        if layoutPadingTop > 0 {
          // java drawing.offset(0, layoutPadingTop);
          drawing.offset(0, layoutPadingTop);
        }
      }
      // java if (shapeRectArg != shapeRect)
      if used_render || shapeRectArg != *shapeRect {
        // java shapeRectArg.recycle();
        shapeRectArg.recycle();
      }
    }

    // java private void calcAffectRect(TypoDrawing drawing, int anchorLine, IStateEnvoy stateEnvoy)
    pub fn calcAffectRect(&mut self, drawing: &mut TypoDrawing, anchorLine: i32, stateEnvoy: &mut dyn IStateEnvoy) {
      // java TypoRectCoreImpl layoutRect = TypoRectCoreImpl.obtain();
      let mut layoutRect: TypoRectCoreImpl = TypoRectCoreImpl::obtain();
      // java drawing.layoutRectForPageCore(layoutRect);
      drawing.layoutRectForPageCore(&mut layoutRect);
      // java TypoPage curPage = stateEnvoy.getCurPage();
      let mut curPage: TypoPage = stateEnvoy.getCurPage();
      // java TypoPage floatsPage = this.getFloatContainerPage(stateEnvoy);
      let mut floatsPage: TypoPage = self.getFloatContainerPage(stateEnvoy);
      // java long offsetResult = DrawingPositionTool.getColPageDrawingOffsetResult(floatsPage, curPage);
      let mut offsetResult: i64 = DrawingPositionTool::getColPageDrawingOffsetResult(&floatsPage, &curPage, &self.mMainState.snapshot);
      // java int offsetX = DrawingPositionTool.getColPageDrawingOffsetX(offsetResult);
      let mut offsetX: i32 = DrawingPositionTool::getColPageDrawingOffsetX(offsetResult);
      // java int offsetY = DrawingPositionTool.getColPageDrawingOffsetY(offsetResult);
      let mut offsetY: i32 = DrawingPositionTool::getColPageDrawingOffsetY(offsetResult);
      // java if (0 != offsetX || 0 != offsetY)
      if 0 != offsetX || 0 != offsetY {
        // java layoutRect.offset(offsetX, offsetY);
        layoutRect.offset(offsetX, offsetY);
      }
      // java boolean isIntersected = this.doCalcAffectRect(drawing, anchorLine, layoutRect, stateEnvoy.getAffectRect(), stateEnvoy);
      let mut affect_copy = stateEnvoy.getAffectRect();
      let mut isIntersected: bool = self.doCalcAffectRect(drawing, anchorLine, &layoutRect, &mut affect_copy, stateEnvoy);
      stateEnvoy.setAffectRect(affect_copy);
      // java if (isIntersected)
      if isIntersected {
        // java drawing.setIntersectedWithText(true);
        drawing.setIntersectedWithText__boolean(true);
      }
      // java layoutRect.recycle();
      layoutRect.recycle();
      // java int anchorCP = drawing.getCP();
      let mut anchorCP: i32 = drawing.getCP();
      // java this.onFlowWrapDrawing(stateEnvoy, anchorCP);
      self.onFlowWrapDrawing(stateEnvoy, anchorCP);
    }

    // java protected boolean doCalcAffectRect(TypoDrawing drawing, int anchorLine, TypoRectCoreImpl layoutRect, TypoRectCoreImpl affectRect, IStateEnvoy stateEnvoy)
    pub fn doCalcAffectRect(&self, drawing: &TypoDrawing, anchorLine: i32, layoutRect: &TypoRectCoreImpl, affectRect: &mut TypoRectCoreImpl, stateEnvoy: &dyn IStateEnvoy) -> bool {
      // java boolean isIntersected = false;
      let mut isIntersected: bool = false;
      // java if (0 != DrawingAffectTool.getAffectedPage(stateEnvoy.getCurPage(), layoutRect, this.mMainState.snapshot))
      if 0 != DrawingAffectTool::getAffectedPage(&stateEnvoy.getCurPage(), layoutRect, &self.mMainState.snapshot) {
        // java DrawingAffectTool.unionAffectRect(affectRect, layoutRect);
        DrawingAffectTool::unionAffectRect(affectRect, layoutRect);
        // java isIntersected = true;
        isIntersected = true;
      }
      // java return isIntersected;
      return isIntersected;
    }

    // java protected int getAlignRectRelColumnHoriOffset(TypoDrawing drawing, int anchorLine, int paraLine, IStateEnvoy stateEnvoy)
    pub fn getAlignRectRelColumnHoriOffset(&mut self, drawing: &TypoDrawing, anchorLine: i32, paraLine: i32, stateEnvoy: &dyn IStateEnvoy) -> i32 {
      let _ = (drawing, paraLine, stateEnvoy);
      if self.mGetTextWrapLayouter.is_none() || anchorLine == 0 {
        return 0;
      }
      0
      /* DEFERRED_NON_FUNCTION
      // java TextWrapLayouter textWrapLayouter;
      let mut textWrapLayouter: TextWrapLayouter = TextWrapLayouter::default();
      // java if (null != this.mGetTextWrapLayouter && null != (textWrapLayouter = this.mGetTextWrapLayouter.getTextWrapLayouter()))
      if let Some(mGetTextWrapLayouter) = self.mGetTextWrapLayouter.as_mut() {
        if { textWrapLayouter = self.mGetTextWrapLayouter.getTextWrapLayouter(); textWrapLayouter }.is_some() {
          // java if (0 == anchorLine)
          if 0 == anchorLine {
            // java return 0;
            return 0;
          }
          // java else
          else {
            // java int excludeStartCP = Integer.MIN_VALUE;
            let mut excludeStartCP: i32 = i32::MIN;
            // java int excludeEndCP = Integer.MAX_VALUE;
            let mut excludeEndCP: i32 = i32::MAX;
            // java if (drawing.isWrap())
            if drawing.isWrap() {
              // java Document doc = stateEnvoy.getDocument();
              let mut doc: Box<dyn Document> = stateEnvoy.getDocument();
              // java int anchorCP = drawing.getCP();
              let mut anchorCP: i32 = drawing.getCP();
              // java IPapxTable.Entry para = doc.getPapxTable().seek(anchorCP);
              let mut para: Box<dyn Entry> = doc.getPapxTable().seek(anchorCP);
              // java excludeStartCP = para.position();
              excludeStartCP = para.position();
              // java excludeEndCP = para.endCp();
              excludeEndCP = para.endCp();
            }
            // java KList<ShapeCollision.ClipRect.RectFWrapImp> subLineRects = this.mSubLineRects;
            let mut subLineRects: KList = { let __x: KList = todo!("no align strategy matched"); __x };
            // java textWrapLayouter.getSubLineRects(subLineRects, TypoObjBase.getTop(anchorLine, this.mMainState.snapshot), TypoObjBase.getBottom(anchorLine, this.mMainState.snapshot), excludeStartCP, excludeEndCP, stateEnvoy);
            textWrapLayouter.getSubLineRects(subLineRects, TypoObjBase::getTop__int_typosnapshot(anchorLine, self.mMainState.snapshot), TypoObjBase::getBottom__int_typosnapshot(anchorLine, self.mMainState.snapshot), excludeStartCP, excludeEndCP, stateEnvoy);
            // java if (subLineRects.isEmpty())
            if subLineRects.is_empty() {
              // java return 0;
              return 0;
            }
            // java else
            else {
              // java int pageMarginLeft = stateEnvoy.getCurPage().getMarginLeft();
              let mut pageMarginLeft: i32 = stateEnvoy.getCurPage().getMarginLeft();
              // java int layoutLeft = stateEnvoy.getPageEnv().marginLeft - pageMarginLeft;
              let mut layoutLeft: i32 = stateEnvoy.getPageEnv().marginLeft - pageMarginLeft;
              // java int ret = ((ShapeCollision.ClipRect.RectFWrapImp)subLineRects.firstElement()).getLeft() - layoutLeft;
              let mut ret: i32 = (subLineRects.firstElement()).downcast_ref::<ClipRect::RectFWrapImp>().unwrap().getLeft() - layoutLeft;
              // java ShapeCollision.ClipRect.recycleRectList(subLineRects);
              ShapeCollision::ClipRect.recycleRectList(subLineRects);
              // java return ret;
              return ret;
            }
          }
        }
      }
      // java else
      else {
        // java return 0;
        return 0;
      }
      */
    }

    // java protected void calcLimitRect(IPos pos, ITypoRect outRect, IStateEnvoy stateEnvoy)
    pub fn calcLimitRect(&self, pos: Option<&IPos>, outRect: &mut ITypoRect, stateEnvoy: &dyn IStateEnvoy) {
      // java TypoPage floatsPage = this.getFloatContainerPage(stateEnvoy);
      let mut floatsPage: TypoPage = self.getFloatContainerPage(stateEnvoy);
      // java if (!this.mDocEnv.textDocument.getCompatibilityMode().isBefore2013Mode())
      if self.mDocEnv.textDocument.is_none() {
        // java outRect.offsetTo(-floatsPage.getMarginLeft(), 0);
        outRect.offsetTo(-floatsPage.getMarginLeft(), 0);
        // java outRect.setWidth(floatsPage.width());
        outRect.setWidth(floatsPage.width());
        // java int height = floatsPage.getCoreHeight();
        let mut height: i32 = floatsPage.getCoreHeight();
        // java switch (floatsPage.getType())
        match floatsPage.getType() {
          // java case 2:
          2 => {
            // java TypoLayoutPage layoutPage = (TypoLayoutPage)floatsPage;
            let mut layoutPage: TypoLayoutPage = TypoLayoutPage;
            // java int pFooterPage = layoutPage.getFooterPage();
            let mut pFooterPage: i32 = layoutPage.getFooterPage();
            // java if (0 != pFooterPage)
            if 0 != pFooterPage {
              // java int footerTop = TypoHeaderFooterPage.getTop(pFooterPage, this.mMainState.snapshot) - stateEnvoy.getPageEnv().marginTop;
              let mut footerTop: i32 = TypoHeaderFooterPage::getTop(pFooterPage, &self.mMainState.snapshot) - stateEnvoy.getPageEnv().marginTop;
              // java height = Math.min(height, footerTop);
              height = (height).min(footerTop);
            }
            // java int pFootnotes = layoutPage.getFootnotes();
            let mut pFootnotes: i32 = layoutPage.getFootnotes();
            // java if (0 != pFootnotes)
            if 0 != pFootnotes {
              // java height = Math.min(height, TypoFootEndNote.getTop(pFootnotes, this.mMainState.snapshot));
              height = (height).min(TypoFootEndNote::getTop(pFootnotes, &self.mMainState.snapshot));
            }
          }
          // java case 14:
          14 => {
            // java TypoColumnPage columnPage = (TypoColumnPage)floatsPage;
            let mut columnPage: TypoColumnPage = TypoColumnPage;
            // java if (0 != columnPage.getFootnotes())
            if 0 != columnPage.getFootnotes() {
              // java height = Math.min(height, TypoFootEndNote.getTop(columnPage.getFootnotes(), this.mMainState.snapshot));
              height = (height).min(TypoFootEndNote::getTop(columnPage.getFootnotes(), &self.mMainState.snapshot));
            }
          }
          // java default:
          _ => {
          }
        }
        // java outRect.setHeight(height);
        outRect.setHeight(height);
      }
      // java else
      else {
        // java outRect.set(floatsPage);
        outRect.set__ityporect(&floatsPage);
        // java outRect.offsetTo(-floatsPage.getMarginLeft(), -floatsPage.getMarginTop());
        outRect.offsetTo(-floatsPage.getMarginLeft(), -floatsPage.getMarginTop());
      }
    }

    // java protected void calcAlignRect(TypoDrawing drawing, ITypoRect outRect, int anchorLine, int paraLine, int paraTop, int relX, int relY, boolean needCalcLeftAndRight, boolean needCalcTopAndBottom, IStateEnvoy stateEnvoy)
    pub fn calcAlignRect(&mut self, drawing: &TypoDrawing, outRect: &mut ITypoRect, anchorLine: i32, paraLine: i32, paraTop: i32, relX: i32, relY: i32, needCalcLeftAndRight: bool, needCalcTopAndBottom: bool, stateEnvoy: &mut dyn IStateEnvoy) {
      // java if (needCalcLeftAndRight)
      if needCalcLeftAndRight {
        // java this.doCalcAlignRectLeftAndRight(drawing, relX, anchorLine, paraLine, paraTop, outRect, stateEnvoy);
        self.doCalcAlignRectLeftAndRight(drawing, relX, anchorLine, paraLine, paraTop, outRect, stateEnvoy);
      }
      // java if (needCalcTopAndBottom)
      if needCalcTopAndBottom {
        // java this.doCalcAlignRectTopAndBottom(drawing, relY, anchorLine, paraLine, paraTop, outRect, stateEnvoy);
        self.doCalcAlignRectTopAndBottom(drawing, relY, anchorLine, paraLine, paraTop, outRect, stateEnvoy);
      }
    }

    fn write_align_left_right(&self, rel_x: i32, align_page: i32, floats_page: i32, out: &mut ITypoRect) {
      use crate::cn::wps::moffice::writer::layout::base::drawing::DrawingCoordTool as Coord;
      let mut rect = Coord::CoordRect { left: out.left, top: out.top, right: out.right, bottom: out.bottom };
      Coord::calcAlignRectLeftAndRightWithPage(rel_x, align_page, floats_page, &mut rect, &self.mMainState.snapshot);
      out.left = rect.left;
      out.top = rect.top;
      out.right = rect.right;
      out.bottom = rect.bottom;
    }

    fn anchor_hori_pos(&self, drawing: &TypoDrawing, anchor_line: i32, p_floats: i32, need_rel_floats: bool, state_envoy: &dyn IStateEnvoy) -> i32 {
      use crate::cn::wps::moffice::writer::layout::base::drawing::DrawingCoordTool as Coord;
      let line_type = if anchor_line != 0 { self.mMainState.snapshot.get__int(anchor_line) } else { -1 };
      if line_type != 3 || !need_rel_floats {
        return 0;
      }
      let p_lines = self.getLineContainerPage(drawing, anchor_line, state_envoy);
      Coord::unpack_first_int(Coord::colPageDrawingOffset(p_lines, p_floats, &self.mMainState.snapshot))
    }

    // java private void doCalcAlignRectLeftAndRight(TypoDrawing drawing, int relX, int anchorLine, int paraLine, int paraTop, ITypoRect outRect, IStateEnvoy stateEnvoy)
    pub fn doCalcAlignRectLeftAndRight(&mut self, drawing: &TypoDrawing, relX: i32, anchorLine: i32, paraLine: i32, paraTop: i32, outRect: &mut ITypoRect, stateEnvoy: &mut dyn IStateEnvoy) {
      use crate::cn::wps::moffice::writer::layout::base::drawing::DrawingCoordTool as Coord;
      let floats_page = self.getFloatContainerPage(stateEnvoy);
      let p_floats = floats_page.getPointer();
      let textflow = Coord::visualTextflow(p_floats, &self.mMainState.snapshot);
      let is_rel_char = relX == 3;
      let is_rel_column = relX == 2;
      let is_wrap_tbl = DrawingPositionTool::isWrapTableOrTextFrame(drawing.getPointer(), &self.mMainState.snapshot);
      let not_in_cell = crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::isLayoutNotInCell__int_typosnapshot(drawing.getPointer(), &self.mMainState.snapshot);
      if is_wrap_tbl && textflow == 1 && relX == 0 && !not_in_cell {
        let p_lines = self.getLineContainerPage(drawing, anchorLine, stateEnvoy);
        let p_align = self.getHoriAlignPage(drawing, anchorLine, relX, stateEnvoy);
        self.write_align_left_right(1, p_align, p_floats, outRect);
        let offset_x = Coord::getLineXForFloatsPageCore(p_lines, p_floats, paraTop, &self.mMainState.snapshot);
        self.mMainState.snapshot.extend__int_int(p_align + 3, -offset_x);
        self.mMainState.snapshot.extend__int_int(p_align + 5, -offset_x);
      } else if Coord::isVertTextflow__int(textflow) && (is_rel_column || is_rel_char) && !not_in_cell {
        let mut extend = 0;
        if is_rel_char || is_wrap_tbl {
          let width = drawing.width;
          extend = if textflow == 1 { -width } else { width };
        }
        let p_lines = self.getLineContainerPage(drawing, anchorLine, stateEnvoy);
        self.write_align_left_right(0, p_lines, p_floats, outRect);
        let line = if is_rel_char { anchorLine } else { paraLine };
        let line_left = if is_wrap_tbl {
          Coord::getLineXForFloatsPageCore(p_lines, p_floats, paraTop, &self.mMainState.snapshot)
        } else {
          Coord::getPrevLineBottom(p_floats, line, &self.mMainState.snapshot)
        };
        outRect.setLeft(line_left + extend);
      } else if relX == 3 {
        let char_pos = self.anchor_hori_pos(drawing, anchorLine, p_floats, true, stateEnvoy);
        outRect.setLeft(char_pos);
        outRect.setRight(char_pos);
      } else {
        self.calcAlignRectLeftAndRightWithPage(drawing, anchorLine, relX, &floats_page, outRect, stateEnvoy);
        if is_rel_column {
          let offset_x = self.getAlignRectRelColumnHoriOffset(drawing, anchorLine, paraLine, stateEnvoy);
          outRect.offset(offset_x, 0);
        }
      }
      /* DEFERRED_NON_FUNCTION
      // java TypoSnapshot snapshot = this.mMainState.snapshot;
      let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
      // java TypoPage floatsPage = this.getFloatContainerPage(stateEnvoy);
      let mut floatsPage: TypoPage = self.getFloatContainerPage({ let __x: &dyn IStateEnvoy = todo!("no align strategy matched"); __x });
      // java int textflow = floatsPage.getVisualTextflow();
      let mut textflow: i32 = floatsPage.getVisualTextflow();
      // java boolean isRelChar = 3 == relX;
      let mut isRelChar: bool = 3 == relX;
      // java boolean isRelColumn = 2 == relX;
      let mut isRelColumn: bool = 2 == relX;
      // java boolean isWrapTblOrTextFrame = DrawingPositionTool.isWrapTableOrTextFrame(drawing.getPointer(), snapshot);
      let mut isWrapTblOrTextFrame: bool = DrawingPositionTool::isWrapTableOrTextFrame(drawing.getPointer(), snapshot);
      // java boolean isLayoutNotInCell = drawing.isLayoutNotInCell();
      let mut isLayoutNotInCell: bool = drawing.isLayoutNotInCell();
      // java if (isWrapTblOrTextFrame && 1 == textflow && 0 == relX && !isLayoutNotInCell)
      if isWrapTblOrTextFrame && 1 == textflow && 0 == relX && !isLayoutNotInCell {
        // java int pLinesPage = this.getLineContainerPage(drawing, anchorLine, stateEnvoy);
        let mut pLinesPage: i32 = self.getLineContainerPage(drawing, anchorLine, { let __x: &dyn IStateEnvoy = todo!("no align strategy matched"); __x });
        // java int pAlignPage = this.getHoriAlignPage(drawing, anchorLine, relX, stateEnvoy);
        let mut pAlignPage: i32 = self.getHoriAlignPage(drawing, anchorLine, relX, { let __x: &dyn IStateEnvoy = todo!("no align strategy matched"); __x });
        // java TypoPage linesPage = (TypoPage)snapshot.getTypoTools().obtain(pLinesPage);
        let mut linesPage: TypoPage = (snapshot.getTypoTools().obtain(pLinesPage)).downcast_ref::<TypoPage>().unwrap();
        // java TypoPage alignPage = (TypoPage)snapshot.getTypoTools().obtain(pAlignPage);
        let mut alignPage: TypoPage = (snapshot.getTypoTools().obtain(pAlignPage)).downcast_ref::<TypoPage>().unwrap();
        // java DrawingAlignRectTool.calcAlignRectLeftAndRightWithPage(1, alignPage, floatsPage, outRect);
        DrawingAlignRectTool::calcAlignRectLeftAndRightWithPage({ let __x: &TypoDrawing = todo!("no align strategy matched"); __x }, { let __x: i32 = todo!("no align strategy matched"); __x }, { let __x: i32 = todo!("no align strategy matched"); __x }, { let __x: &TypoPage = todo!("no align strategy matched"); __x });
        // java int offsetX = DrawingCoordTool.getLineXForFloatsPageCore(linesPage, floatsPage, paraTop);
        let mut offsetX: i32 = DrawingCoordTool::getLineXForFloatsPageCore(linesPage, floatsPage, paraTop);
        // java alignPage.offsetX(-offsetX);
        alignPage.offsetX(-offsetX);
        // java snapshot.getTypoTools().recycle((TypoMemoryObj)linesPage);
        snapshot.getTypoTools().recycle((linesPage).downcast_ref::<TypoMemoryObj>().unwrap());
        // java snapshot.getTypoTools().recycle((TypoMemoryObj)alignPage);
        snapshot.getTypoTools().recycle((alignPage).downcast_ref::<TypoMemoryObj>().unwrap());
      }
      // java else if (DrawingCoordTool.isVertTextflow(textflow) && (isRelColumn || isRelChar) && !isLayoutNotInCell)
      else if DrawingCoordTool::isVertTextflow__typopage(textflow) && isRelColumn || isRelChar && !isLayoutNotInCell {
        // java int extend = 0;
        let mut extend: i32 = 0;
        // java if (isRelChar || isWrapTblOrTextFrame)
        if isRelChar || isWrapTblOrTextFrame {
          // java int width = 0;
          let mut width: i32 = 0;
          // java if (DrawingPositionTool.shouldOffsetDrawingUsingRenderRect(drawing))
          if DrawingPositionTool::shouldOffsetDrawingUsingRenderRect__typodrawing(drawing) {
            // java width = DrawingCoordTool.getDrawingRenderWidth(drawing);
            width = DrawingCoordTool::getDrawingRenderWidth(drawing);
          }
          // java else
          else {
            // java width = DrawingCoordTool.getDrawingWidth(drawing);
            width = DrawingCoordTool::getDrawingWidth(drawing);
          }
          // java extend = 1 == textflow ? -width : width;
          extend = (if 1 == textflow { -width } else { width });
        }
        // java int pLinesPage = this.getLineContainerPage(drawing, anchorLine, stateEnvoy);
        let mut pLinesPage: i32 = self.getLineContainerPage(drawing, anchorLine, { let __x: &dyn IStateEnvoy = todo!("no align strategy matched"); __x });
        // java TypoPage linesPage = (TypoPage)snapshot.getTypoTools().obtain(pLinesPage);
        let mut linesPage: TypoPage = (snapshot.getTypoTools().obtain(pLinesPage)).downcast_ref::<TypoPage>().unwrap();
        // java DrawingAlignRectTool.calcAlignRectLeftAndRightWithPage(0, linesPage, floatsPage, outRect);
        DrawingAlignRectTool::calcAlignRectLeftAndRightWithPage({ let __x: &TypoDrawing = todo!("no align strategy matched"); __x }, { let __x: i32 = todo!("no align strategy matched"); __x }, { let __x: i32 = todo!("no align strategy matched"); __x }, { let __x: &TypoPage = todo!("no align strategy matched"); __x });
        // java int lineLeft = 0;
        let mut lineLeft: i32 = 0;
        // java int line = isRelChar ? anchorLine : paraLine;
        let mut line: i32 = (if isRelChar { anchorLine } else { paraLine });
        // java if (isWrapTblOrTextFrame)
        if isWrapTblOrTextFrame {
          // java lineLeft = DrawingCoordTool.getLineXForFloatsPageCore(linesPage, floatsPage, paraTop);
          lineLeft = DrawingCoordTool::getLineXForFloatsPageCore(linesPage, floatsPage, paraTop);
        }
        // java else
        else {
          // java lineLeft = DrawingCoordTool.getPrevLineBottom(floatsPage, line, snapshot);
          lineLeft = DrawingCoordTool::getPrevLineBottom(floatsPage, line, snapshot);
        }
        // java outRect.setLeft(lineLeft + extend);
        outRect.setLeft(lineLeft + extend);
        // java snapshot.getTypoTools().recycle((TypoMemoryObj)linesPage);
        snapshot.getTypoTools().recycle((linesPage).downcast_ref::<TypoMemoryObj>().unwrap());
      }
      // java else if (3 == relX)
      else if 3 == relX {
        // java int charPos = this.getAnchorHoriPos(drawing, anchorLine, true, stateEnvoy);
        let mut charPos: i32 = self.getAnchorHoriPos(drawing, anchorLine, true, { let __x: &dyn IStateEnvoy = todo!("no align strategy matched"); __x });
        // java outRect.setLeft(charPos);
        outRect.setLeft(charPos);
        // java outRect.setRight(charPos);
        outRect.setRight(charPos);
      }
      // java else
      else {
        // java this.calcAlignRectLeftAndRightWithPage(drawing, anchorLine, relX, floatsPage, outRect, stateEnvoy);
        self.calcAlignRectLeftAndRightWithPage(drawing, anchorLine, relX, &floatsPage, outRect, { let __x: &dyn IStateEnvoy = todo!("no align strategy matched"); __x });
        // java if (isRelColumn)
        if isRelColumn {
          // java int offsetX = this.getAlignRectRelColumnHoriOffset(drawing, anchorLine, paraLine, stateEnvoy);
          let mut offsetX: i32 = self.getAlignRectRelColumnHoriOffset(drawing, anchorLine, paraLine, { let __x: &dyn IStateEnvoy = todo!("no align strategy matched"); __x });
          // java outRect.offset(offsetX, 0);
          outRect.offset(offsetX, 0);
        }
      }
      */
    }

    // java protected void calcAlignRectLeftAndRightWithPage(TypoDrawing drawing, int anchorLine, int relX, TypoPage floatsPage, ITypoRect outRect, IStateEnvoy stateEnvoy)
    pub fn calcAlignRectLeftAndRightWithPage(&self, drawing: &TypoDrawing, anchorLine: i32, relX: i32, floatsPage: &TypoPage, outRect: &mut ITypoRect, stateEnvoy: &dyn IStateEnvoy) {
      let p_align = self.getHoriAlignPage(drawing, anchorLine, relX, stateEnvoy);
      self.write_align_left_right(relX, p_align, floatsPage.getPointer(), outRect);
      /* DEFERRED_NON_FUNCTION
      // java TypoSnapshot snapshot = this.mMainState.snapshot;
      let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
      // java int pHoriPage = this.getHoriAlignPage(drawing, anchorLine, relX, stateEnvoy);
      let mut pHoriPage: i32 = self.getHoriAlignPage(drawing, anchorLine, relX, stateEnvoy);
      // java TypoPage horiPage = (TypoPage)snapshot.getTypoTools().obtain(pHoriPage);
      let mut horiPage: TypoPage = (snapshot.getTypoTools().obtain(pHoriPage)).downcast_ref::<TypoPage>().unwrap();
      // java DrawingAlignRectTool.calcAlignRectLeftAndRightWithPage(relX, horiPage, floatsPage, outRect);
      DrawingAlignRectTool::calcAlignRectLeftAndRightWithPage({ let __x: &TypoDrawing = todo!("no align strategy matched"); __x }, { let __x: i32 = todo!("no align strategy matched"); __x }, { let __x: i32 = todo!("no align strategy matched"); __x }, { let __x: &TypoPage = todo!("no align strategy matched"); __x });
      // java snapshot.getTypoTools().recycle((TypoMemoryObj)horiPage);
      snapshot.getTypoTools().recycle((horiPage).downcast_ref::<TypoMemoryObj>().unwrap());
      */
    }

    // java protected void calcAlignRectTopAndBottomWithPage(TypoDrawing drawing, int anchorLine, int relY, TypoPage floatsPage, ITypoRect outRect, IStateEnvoy stateEnvoy)
    pub fn calcAlignRectTopAndBottomWithPage(&self, drawing: &TypoDrawing, anchorLine: i32, relY: i32, floatsPage: &TypoPage, outRect: &mut ITypoRect, stateEnvoy: &dyn IStateEnvoy) {
      use crate::cn::wps::moffice::writer::layout::base::drawing::DrawingCoordTool as Coord;
      let p_align = self.getVertAlignPage(drawing, anchorLine, relY, stateEnvoy);
      let mut rect = Coord::CoordRect { left: outRect.left, top: outRect.top, right: outRect.right, bottom: outRect.bottom };
      Coord::calcAlignRectTopAndBottomWithPage(relY, drawing.getPointer(), p_align, floatsPage.getPointer(), &mut rect, &self.mMainState.snapshot);
      outRect.left = rect.left;
      outRect.top = rect.top;
      outRect.right = rect.right;
      outRect.bottom = rect.bottom;
      let _ = stateEnvoy;
      /* DEFERRED_NON_FUNCTION
      // java TypoSnapshot snapshot = this.mMainState.snapshot;
      let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
      // java int pVertPage = this.getVertAlignPage(drawing, anchorLine, relY, stateEnvoy);
      let mut pVertPage: i32 = self.getVertAlignPage(drawing, anchorLine, relY, stateEnvoy);
      // java TypoPage vertPage = (TypoPage)snapshot.getTypoTools().obtain(pVertPage);
      let mut vertPage: TypoPage = (snapshot.getTypoTools().obtain(pVertPage)).downcast_ref::<TypoPage>().unwrap();
      // java DrawingAlignRectTool.calcAlignRectTopAndBottomWithPage(relY, drawing, vertPage, floatsPage, outRect, snapshot);
      DrawingAlignRectTool::calcAlignRectTopAndBottomWithPage({ let __x: &TypoDrawing = todo!("no align strategy matched"); __x }, { let __x: i32 = todo!("no align strategy matched"); __x }, { let __x: i32 = todo!("no align strategy matched"); __x }, floatsPage, outRect, { let __x: &dyn IStateEnvoy = todo!("no align strategy matched"); __x });
      // java snapshot.getTypoTools().recycle((TypoMemoryObj)vertPage);
      snapshot.getTypoTools().recycle((vertPage).downcast_ref::<TypoMemoryObj>().unwrap());
      */
    }

    // java private void doCalcAlignRectTopAndBottom(TypoDrawing drawing, int relY, int anchorLine, int paraLine, int paraTop, ITypoRect outRect, IStateEnvoy stateEnvoy)
    pub fn doCalcAlignRectTopAndBottom(&self, drawing: &TypoDrawing, relY: i32, anchorLine: i32, paraLine: i32, paraTop: i32, outRect: &mut ITypoRect, stateEnvoy: &dyn IStateEnvoy) {
      use crate::cn::wps::moffice::writer::layout::base::drawing::DrawingCoordTool as Coord;
      let floats_page = self.getFloatContainerPage(stateEnvoy);
      let p_floats = floats_page.getPointer();
      let textflow = Coord::visualTextflow(p_floats, &self.mMainState.snapshot);
      let is_vert = Coord::isVertTextflow__int(textflow);
      let is_rel_para = relY == 2;
      let is_rel_line = relY == 3;
      let is_rel_margin_or_page = !is_rel_para && !is_rel_line;
      let is_wrap_tbl = DrawingPositionTool::isWrapTableOrTextFrame(drawing.getPointer(), &self.mMainState.snapshot);
      if is_vert && is_wrap_tbl && relY == 2 {
        let p_lines = self.getLineContainerPage(drawing, anchorLine, stateEnvoy);
        let mut rect = Coord::CoordRect { left: outRect.left, top: outRect.top, right: outRect.right, bottom: outRect.bottom };
        Coord::calcAlignRectTopAndBottomWithPage(0, drawing.getPointer(), p_lines, p_floats, &mut rect, &self.mMainState.snapshot);
        outRect.left = rect.left;
        outRect.top = rect.top;
        outRect.right = rect.right;
        outRect.bottom = rect.bottom;
      } else if is_rel_margin_or_page {
        self.calcAlignRectTopAndBottomWithPage(drawing, anchorLine, relY, &floats_page, outRect, stateEnvoy);
      } else if is_vert {
        if anchorLine == 0 {
          outRect.setTop(0);
          outRect.setBottom(0);
          return;
        }
        let org_left = outRect.getLeft();
        let org_right = outRect.getRight();
        let align_top = if is_rel_line {
          let line_left = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getLeft__int_typosnapshot(anchorLine, &self.mMainState.snapshot);
          let anchor_pos = self.anchor_hori_pos(drawing, anchorLine, p_floats, false, stateEnvoy);
          let mut rect = Coord::CoordRect { left: outRect.left, top: outRect.top, right: outRect.right, bottom: outRect.bottom };
          Coord::rectForPageCoreRel(anchorLine, p_floats, &mut rect, &self.mMainState.snapshot);
          outRect.left = rect.left;
          outRect.top = rect.top;
          outRect.right = rect.right;
          outRect.bottom = rect.bottom;
          rect.top + (anchor_pos - line_left)
        } else {
          let p_lines = self.getLineContainerPage(drawing, anchorLine, stateEnvoy);
          let mut rect = Coord::CoordRect { left: outRect.left, top: outRect.top, right: outRect.right, bottom: outRect.bottom };
          Coord::rectForPageCoreRel(p_lines, p_floats, &mut rect, &self.mMainState.snapshot);
          outRect.left = rect.left;
          outRect.top = rect.top;
          outRect.right = rect.right;
          outRect.bottom = rect.bottom;
          rect.top + Coord::getMarginTop(p_lines, textflow, &self.mMainState.snapshot)
        };
        outRect.left = org_left;
        outRect.top = align_top;
        outRect.right = org_right;
        outRect.bottom = align_top;
      } else if anchorLine != 0 {
        let line = if is_rel_para { paraLine } else { anchorLine };
        let res = Coord::getPrevLineBottom(p_floats, line, &self.mMainState.snapshot);
        outRect.setTop(res);
        outRect.setBottom(res);
      } else {
        let p_lines = self.getLineContainerPage(drawing, anchorLine, stateEnvoy);
        let offset = Coord::colPageDrawingOffset(p_lines, p_floats, &self.mMainState.snapshot);
        let res = paraTop + Coord::unpack_second_int(offset);
        outRect.setTop(res);
        outRect.setBottom(res);
      }
      let _ = paraLine;
      /* DEFERRED_NON_FUNCTION
      // java TypoSnapshot snapshot = this.mMainState.snapshot;
      let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
      // java TypoPage floatsPage = this.getFloatContainerPage(stateEnvoy);
      let mut floatsPage: TypoPage = self.getFloatContainerPage(stateEnvoy);
      // java int textflow = floatsPage.getVisualTextflow();
      let mut textflow: i32 = floatsPage.getVisualTextflow();
      // java boolean isVertTextFlow = DrawingCoordTool.isVertTextflow(textflow);
      let mut isVertTextFlow: bool = DrawingCoordTool::isVertTextflow__typopage(textflow);
      // java boolean isRelPara = 2 == relY;
      let mut isRelPara: bool = 2 == relY;
      // java boolean isRelLine = 3 == relY;
      let mut isRelLine: bool = 3 == relY;
      // java boolean isRelMarginOrPage = !isRelPara && !isRelLine;
      let mut isRelMarginOrPage: bool = !isRelPara && !isRelLine;
      // java boolean isWrapTblOrTextFrame = DrawingPositionTool.isWrapTableOrTextFrame(drawing.getPointer(), snapshot);
      let mut isWrapTblOrTextFrame: bool = DrawingPositionTool::isWrapTableOrTextFrame(drawing.getPointer(), snapshot);
      // java if (isVertTextFlow && isWrapTblOrTextFrame && 2 == relY)
      if isVertTextFlow && isWrapTblOrTextFrame && 2 == relY {
        // java int pLinesPage = this.getLineContainerPage(drawing, anchorLine, stateEnvoy);
        let mut pLinesPage: i32 = self.getLineContainerPage(drawing, anchorLine, stateEnvoy);
        // java TypoPage linesPage = (TypoPage)snapshot.getTypoTools().obtain(pLinesPage);
        let mut linesPage: TypoPage = (snapshot.getTypoTools().obtain(pLinesPage)).downcast_ref::<TypoPage>().unwrap();
        // java DrawingAlignRectTool.calcAlignRectTopAndBottomWithPage(0, drawing, linesPage, floatsPage, outRect, snapshot);
        DrawingAlignRectTool::calcAlignRectTopAndBottomWithPage({ let __x: &TypoDrawing = todo!("no align strategy matched"); __x }, { let __x: i32 = todo!("no align strategy matched"); __x }, { let __x: i32 = todo!("no align strategy matched"); __x }, &floatsPage, outRect, { let __x: &dyn IStateEnvoy = todo!("no align strategy matched"); __x });
        // java snapshot.getTypoTools().recycle((TypoMemoryObj)linesPage);
        snapshot.getTypoTools().recycle((linesPage).downcast_ref::<TypoMemoryObj>().unwrap());
      }
      // java else if (isRelMarginOrPage)
      else if isRelMarginOrPage {
        // java this.calcAlignRectTopAndBottomWithPage(drawing, anchorLine, relY, floatsPage, outRect, stateEnvoy);
        self.calcAlignRectTopAndBottomWithPage(drawing, anchorLine, relY, &floatsPage, outRect, stateEnvoy);
      }
      // java else if (isVertTextFlow)
      else if isVertTextFlow {
        // java if (0 == anchorLine)
        if 0 == anchorLine {
          // java outRect.setTop(0);
          outRect.setTop(0);
          // java outRect.setBottom(0);
          outRect.setBottom(0);
          // java return;
          return;
        }
        // java int orgLeft = outRect.getLeft();
        let mut orgLeft: i32 = outRect.getLeft();
        // java int orgRight = outRect.getRight();
        let mut orgRight: i32 = outRect.getRight();
        // java int alignRectTop = 0;
        let mut alignRectTop: i32 = 0;
        // java if (isRelLine)
        if isRelLine {
          // java int lineLeft = TypoObjBase.getLeft(anchorLine, snapshot);
          let mut lineLeft: i32 = TypoObjBase::getLeft__int_typosnapshot(anchorLine, snapshot);
          // java int anchorPos = this.getAnchorHoriPos(drawing, anchorLine, false, stateEnvoy);
          let mut anchorPos: i32 = self.getAnchorHoriPos(drawing, anchorLine, false, stateEnvoy);
          // java DrawingCoordTool.rectForPageCore(anchorLine, floatsPage, outRect);
          DrawingCoordTool::rectForPageCore__int_typopage_ityporect(anchorLine, floatsPage, outRect);
          // java alignRectTop = outRect.getTop() + (anchorPos - lineLeft);
          alignRectTop = outRect.getTop() + anchorPos - lineLeft;
        }
        // java else
        else {
          // java int pLinesPage = this.getLineContainerPage(drawing, anchorLine, stateEnvoy);
          let mut pLinesPage: i32 = self.getLineContainerPage(drawing, anchorLine, stateEnvoy);
          // java DrawingCoordTool.rectForPageCore(pLinesPage, floatsPage, outRect);
          DrawingCoordTool::rectForPageCore__int_typopage_ityporect(pLinesPage, floatsPage, outRect);
          // java TypoPage linesPage = (TypoPage)snapshot.getTypoTools().obtain(pLinesPage);
          let mut linesPage: TypoPage = (snapshot.getTypoTools().obtain(pLinesPage)).downcast_ref::<TypoPage>().unwrap();
          // java alignRectTop = outRect.getTop() + DrawingCoordTool.getMarginTop(linesPage, textflow);
          alignRectTop = outRect.getTop() + DrawingCoordTool::getMarginTop(linesPage, textflow);
          // java snapshot.getTypoTools().recycle((TypoMemoryObj)linesPage);
          snapshot.getTypoTools().recycle((linesPage).downcast_ref::<TypoMemoryObj>().unwrap());
        }
        // java outRect.set(orgLeft, alignRectTop, orgRight, alignRectTop);
        outRect.set__int_int_int_int(orgLeft, alignRectTop, orgRight, alignRectTop);
      }
      // java else if (0 != anchorLine)
      else if 0 != anchorLine {
        // java int line = isRelPara ? paraLine : anchorLine;
        let mut line: i32 = (if isRelPara { paraLine } else { anchorLine });
        // java int res = DrawingCoordTool.getPrevLineBottom(floatsPage, line, snapshot);
        let mut res: i32 = DrawingCoordTool::getPrevLineBottom(floatsPage, line, snapshot);
        // java outRect.setTop(res);
        outRect.setTop(res);
        // java outRect.setBottom(res);
        outRect.setBottom(res);
      }
      // java else
      else {
        // java int pLinesPage = this.getLineContainerPage(drawing, anchorLine, stateEnvoy);
        let mut pLinesPage: i32 = self.getLineContainerPage(drawing, anchorLine, stateEnvoy);
        // java TypoPage linesPage = (TypoPage)snapshot.getTypoTools().obtain(pLinesPage);
        let mut linesPage: TypoPage = (snapshot.getTypoTools().obtain(pLinesPage)).downcast_ref::<TypoPage>().unwrap();
        // java long offsetResult = DrawingPositionTool.getColPageDrawingOffsetResult(linesPage, floatsPage);
        let mut offsetResult: i64 = DrawingPositionTool::getColPageDrawingOffsetResult(linesPage, floatsPage);
        // java snapshot.getTypoTools().recycle((TypoMemoryObj)linesPage);
        snapshot.getTypoTools().recycle((linesPage).downcast_ref::<TypoMemoryObj>().unwrap());
        // java int res = paraTop + DrawingPositionTool.getColPageDrawingOffsetY(offsetResult);
        let mut res: i32 = paraTop + DrawingPositionTool::getColPageDrawingOffsetY(offsetResult);
        // java outRect.setTop(res);
        outRect.setTop(res);
        // java outRect.setBottom(res);
        outRect.setBottom(res);
      }
      */
    }

    // java protected void calcTextBoxFitRect(ITypoRect outRect, IStateEnvoy stateEnvoy)
    pub fn calcTextBoxFitRect(&self, outRect: &mut ITypoRect, stateEnvoy: &dyn IStateEnvoy) {
      let floats_page = self.getFloatContainerPage(stateEnvoy);
      outRect.left = 0;
      outRect.top = 0;
      outRect.right = 0;
      outRect.bottom = 0;
      let width = crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage::getCoreWidth__int_typosnapshot(floats_page.getPointer(), &self.mMainState.snapshot);
      outRect.setWidth(width);
      outRect.setHeight(stateEnvoy.getPageEnv().height);
      /* DEFERRED_NON_FUNCTION
      // java TypoPage floatsPage = this.getFloatContainerPage(stateEnvoy);
      let mut floatsPage: TypoPage = self.getFloatContainerPage(stateEnvoy);
      // java outRect.setEmpty();
      outRect.setEmpty();
      // java outRect.setWidth(floatsPage.getCoreWidth());
      outRect.setWidth(floatsPage.getCoreWidth());
      // java outRect.setHeight(stateEnvoy.getPageEnv().height);
      outRect.setHeight(stateEnvoy.getPageEnv().height);
      */
    }

    // java protected void flowChildShapes(TypoDrawing drawing, int parentLeft, int parentTop, float dw, float dh, float parentRot, RectF rtParentCoord, ITypoRect parentRenderRect, IStateEnvoy stateEnvoy)
    pub fn flowChildShapes(&mut self, drawing: &mut TypoDrawing, parentLeft: i32, parentTop: i32, dw: f32, dh: f32, parentRot: f32, rtParentCoord: RectF, parentRenderRect: &ITypoRect, stateEnvoy: &mut dyn IStateEnvoy) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java TypoSnapshot snapshot = this.mMainState.snapshot;
      let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
      // java Shape shape = drawing.getShape();
      let mut shape: Shape = drawing.getShape();
      // java TypoRectCoreImpl renderRect = TypoRectCoreImpl.obtain();
      let mut renderRect: TypoRectCoreImpl = TypoRectCoreImpl::obtain();
      // java if (shape.isChildShape())
      if shape.isChildShape() {
        // java IPos pos = drawing.getPos();
        let mut pos: Box<dyn IPos> = drawing.getPos();
        // java if (pos != null)
        if true {
          // java RectF shapeRect = pos.getShapeRect();
          let mut shapeRect: RectF = pos.getShapeRect();
          // java if (ShapeHelper.isChangeRect((int)pos.getRotation()))
          if ShapeHelper::isChangeRect(((pos.getRotation()) as i32)) {
            // java int cx = (int)(((shapeRect.left + shapeRect.right) / 2.0F - rtParentCoord.left) * dw);
            let mut cx: i32 = ((shapeRect.left + shapeRect.right / 2 - rtParentCoord.left * dw) as i32);
            // java int cy = (int)(((shapeRect.top + shapeRect.bottom) / 2.0F - rtParentCoord.top) * dh);
            let mut cy: i32 = ((shapeRect.top + shapeRect.bottom / 2 - rtParentCoord.top * dh) as i32);
            // java int hw = (int)((shapeRect.right - shapeRect.left) * dh / 2.0F);
            let mut hw: i32 = ((shapeRect.right - shapeRect.left * dh / 2) as i32);
            // java int hh = (int)((shapeRect.bottom - shapeRect.top) * dw / 2.0F);
            let mut hh: i32 = ((shapeRect.bottom - shapeRect.top * dw / 2) as i32);
            // java drawing.set(cx - hw, cy - hh, cx + hw, cy + hh);
            drawing.set(cx - hw, cy - hh, cx + hw, cy + hh);
          }
          // java else
          else {
            // java drawing.setLeft((int)((shapeRect.left - rtParentCoord.left) * dw));
            drawing.setLeft(((shapeRect.left - rtParentCoord.left * dw) as i32));
            // java drawing.setRight((int)((shapeRect.right - rtParentCoord.left) * dw));
            drawing.setRight(((shapeRect.right - rtParentCoord.left * dw) as i32));
            // java drawing.setTop((int)((shapeRect.top - rtParentCoord.top) * dh));
            drawing.setTop(((shapeRect.top - rtParentCoord.top * dh) as i32));
            // java drawing.setBottom((int)((shapeRect.bottom - rtParentCoord.top) * dh));
            drawing.setBottom(((shapeRect.bottom - rtParentCoord.top * dh) as i32));
          }
          // java DrawingShapeRectTool.scaleShapeRect(pos, drawing);
          DrawingShapeRectTool::scaleShapeRect(pos, drawing);
        }
        // java this.flowTextBox(drawing, shape, stateEnvoy);
        self.flowTextBox(drawing, &shape, stateEnvoy);
        // java ITypoRect outRect = TypoRectCoreImpl.obtain();
        let mut outRect: Box<dyn ITypoRect> = TypoRectCoreImpl::obtain();
        // java DrawingShapeRectTool.calcDrawingOutRect(drawing, outRect);
        DrawingShapeRectTool::calcDrawingOutRect(drawing, outRect);
        // java renderRect.set(outRect);
        renderRect.set(outRect);
        // java DrawingResizeTool.setDrawingRenderRect(drawing, outRect);
        DrawingResizeTool::setDrawingRenderRect(drawing, outRect.as_ref());
        // java DrawingResizeTool.setDrawingLayoutRect(drawing, outRect);
        DrawingResizeTool::setDrawingLayoutRect(drawing, outRect.as_ref());
        // java outRect.recycle();
        outRect.recycle();
      }
      // java if (shape instanceof GroupShape)
      if shape.downcast_ref::<GroupShape>().is_some() {
        // java TypoRectCoreImpl unionedChildRect = TypoRectCoreImpl.obtain();
        let mut unionedChildRect: TypoRectCoreImpl = TypoRectCoreImpl::obtain();
        // java GroupShape group = (GroupShape)shape;
        let mut group: GroupShape = (shape).downcast_ref::<GroupShape>().unwrap();
        // java float nextRot = drawing.getPos() == null ? 0.0F : drawing.getPos().getRotation();
        let mut nextRot: f32 = (if drawing.getPos().is_none() { 0 } else { drawing.getPos().getRotation() });
        // java int nextLeft = drawing.getLeft();
        let mut nextLeft: i32 = drawing.getLeft();
        // java int nextTop = drawing.getTop();
        let mut nextTop: i32 = drawing.getTop();
        // java rtParentCoord = group.getCoordBounds();
        rtParentCoord = group.getCoordBounds();
        // java float nextDw = (float)drawing.width() / rtParentCoord.width();
        let mut nextDw: f32 = ((drawing.width()) as f32) / rtParentCoord.width();
        // java float nextDh = (float)drawing.height() / rtParentCoord.height();
        let mut nextDh: f32 = ((drawing.height()) as f32) / rtParentCoord.height();
        // java int i = 0;
        let mut i: i32 = 0;
        let mut count: i32 = group.getChildCount();
        // java for(int count = group.getChildCount(); i < count; ++i)
        while i < count {
          // java Shape sonShape = group.getChildShape(i);
          let mut sonShape: Shape = group.getChildShape(i);
          // java TypoDrawing childDrawing = snapshot.getTypoTools().obtainTypoDrawing();
          let mut childDrawing: TypoDrawing = snapshot.getTypoTools().obtainTypoDrawing();
          // java childDrawing.alloc(snapshot);
          childDrawing.alloc(snapshot);
          // java drawing.addDrawing(childDrawing.getPointer());
          drawing.addDrawing(childDrawing.getPointer());
          // java childDrawing.setIsChildDrawing(true);
          childDrawing.setIsChildDrawing(true);
          // java childDrawing.setParent(drawing.getPointer());
          childDrawing.setParent(drawing.getPointer());
              // java childDrawing.setCloneShape(sonShape.cloneLightWeight());
              childDrawing.setCloneShape(sonShape.cloneLightWeight());
              // java CloneNotSupportedException e
              todo!();
                // java e.printStackTrace();
                e.printStackTrace();
          // java this.flowChildShapes(childDrawing, nextLeft, nextTop, nextDw, nextDh, nextRot, rtParentCoord, unionedChildRect, stateEnvoy);
          self.flowChildShapes(&mut childDrawing, nextLeft, nextTop, nextDw, nextDh, nextRot, rtParentCoord, { let __x: &ITypoRect = todo!("no align strategy matched"); __x }, stateEnvoy);
        }
        // java DrawingResizeTool.setDrawingRenderRect(drawing, unionedChildRect);
        DrawingResizeTool::setDrawingRenderRect(drawing, { let __x: &Shape = todo!("no align strategy matched"); __x });
        // java renderRect.set(unionedChildRect);
        renderRect.set(unionedChildRect);
        // java unionedChildRect.recycle();
        unionedChildRect.recycle();
      }
      // java int pParentDrawing = drawing.getParentDrawing();
      let mut pParentDrawing: i32 = drawing.getParentDrawing();
      // java if (0 != pParentDrawing && null != parentRenderRect)
      if 0 != pParentDrawing && true {
        // java TypoDrawing parentDrawing = snapshot.getTypoTools().obtainTypoDrawing(pParentDrawing);
        let mut parentDrawing: TypoDrawing = snapshot.getTypoTools().obtainTypoDrawing(pParentDrawing);
        // java int centerX = (parentDrawing.getLeft() + parentDrawing.getRight()) / 2;
        let mut centerX: i32 = parentDrawing.getLeft() + parentDrawing.getRight() / 2;
        // java int centerY = (parentDrawing.getTop() + parentDrawing.getBottom()) / 2;
        let mut centerY: i32 = parentDrawing.getTop() + parentDrawing.getBottom() / 2;
        // java snapshot.getTypoTools().recycle((TypoMemoryObj)parentDrawing);
        snapshot.getTypoTools().recycle((parentDrawing).downcast_ref::<TypoMemoryObj>().unwrap());
        // java renderRect.offset(parentLeft, parentTop);
        renderRect.offset(parentLeft, parentTop);
        // java DrawingShapeRectTool.calcRotatedRect(renderRect, centerX, centerY, parentRot);
        DrawingShapeRectTool::calcRotatedRect__ityporect_int_int_float(renderRect, centerX, centerY, parentRot);
        // java parentRenderRect.union(renderRect);
        parentRenderRect.union__ityporect(renderRect);
      }
      // java renderRect.recycle();
      renderRect.recycle();
      */
    }

    // java protected void createTextboxLayouter(IStateEnvoy stateEnvoy)
    pub fn createTextboxLayouter(&mut self, stateEnvoy: &dyn IStateEnvoy) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mTextboxLayouter)
      if self.mTextboxLayouter.is_none() {
        // java if (stateEnvoy.getDocument().getType() == 2)
        if stateEnvoy.getDocument().getType() == 2 {
          // java this.mTextboxLayouter = new HeaderFooterPageTextboxLayouter(this.mMainState, this.mDocEnv, this.mViewEnv);
          self.mTextboxLayouter = HeaderFooterPageTextboxLayouter::new(self.mMainState, self.mDocEnv, self.mViewEnv);
        }
        // java else
        else {
          // java this.mTextboxLayouter = new TextboxLayouter(this.mMainState, this.mDocEnv, this.mViewEnv);
          self.mTextboxLayouter = TextboxLayouter::new(self.mMainState, self.mDocEnv, self.mViewEnv);
        }
      }
      */
    }

    // java private TextboxLayoutState getTextBoxLayoutState()
    pub fn getTextBoxLayoutState(&mut self) -> &mut TextboxLayoutState {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mTextboxLayoutState == null)
      if self.mTextboxLayoutState.is_none() {
        // java this.mTextboxLayoutState = (TextboxLayoutState)this.mTextboxLayouter.createLayoutState();
        self.mTextboxLayoutState = (self.mTextboxLayouter.createLayoutState()).downcast_ref::<TextboxLayoutState>().unwrap();
      }
      // java return this.mTextboxLayoutState;
      return self.mTextboxLayoutState.unwrap();
      */
    }

    // java protected void flowTextBox(TypoDrawing drawing, Shape shape, IStateEnvoy stateEnvoy)
    pub fn flowTextBox(&mut self, drawing: &mut TypoDrawing, shape: &Shape, stateEnvoy: &mut dyn IStateEnvoy) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java TextFrame textbox = (TextFrame)shape.getTextObj();
      let mut textbox: TextFrame = (shape.getTextObj()).downcast_ref::<TextFrame>().unwrap();
      // java if (null != textbox && textbox.hasText())
      if true && textbox.hasText() {
        // java TypoSnapshot snapshot = this.mMainState.snapshot;
        let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
        // java this.createTextboxLayouter(stateEnvoy);
        self.createTextboxLayouter({ let __x: &dyn IStateEnvoy = todo!("no align strategy matched"); __x });
        // java TextboxLayoutState textboxEnv = this.getTextBoxLayoutState();
        let mut textboxEnv: TextboxLayoutState = self.getTextBoxLayoutState().clone();
        // java PageEnv textboxPageEnv = textboxEnv.pageEnv;
        let mut textboxPageEnv: PageEnv = textboxEnv.pageEnv;
        // java PageEnv pageEnv = stateEnvoy.getPageEnv();
        let mut pageEnv: PageEnv = stateEnvoy.getPageEnv();
        // java textboxPageEnv.gridHeight = pageEnv.gridHeight;
        textboxPageEnv.gridHeight = pageEnv.gridHeight;
        // java textboxPageEnv.gridWidth = pageEnv.gridWidth;
        textboxPageEnv.gridWidth = pageEnv.gridWidth;
        // java textboxPageEnv.gridModel = pageEnv.gridModel;
        textboxPageEnv.gridModel = pageEnv.gridModel;
        // java textboxPageEnv.charSpaceForGrid = pageEnv.charSpaceForGrid;
        textboxPageEnv.charSpaceForGrid = pageEnv.charSpaceForGrid;
        // java this.mDocEnv.pools.recycleTableCellLayoutStates(textboxEnv.cellStateList);
        self.mDocEnv.pools.recycleTableCellLayoutStates(textboxEnv.cellStateList);
        // java long textRange = textbox.getTextContentRange();
        let mut textRange: i64 = textbox.getTextContentRange();
        // java textboxEnv.setRange(RANGEFactory.getStart(textRange), RANGEFactory.getEnd(textRange));
        textboxEnv.setRange(RANGEFactory::getStart(textRange), RANGEFactory::getEnd(textRange));
        // java textboxEnv.setRangeEnd(Math.max(textboxEnv.getRangeStart(), textboxEnv.getRangeEnd() - 1));
        textboxEnv.setRangeEnd((textboxEnv.getRangeStart()).max(textboxEnv.getRangeEnd() - 1));
        // java TypoToolsManager toolsManager = snapshot.getTypoTools();
        let mut toolsManager: TypoToolsManager = snapshot.getTypoTools();
        // java TypoTextBox typoTextbox = toolsManager.obtainTypoTextBox();
        let mut typoTextbox: TypoTextBox = toolsManager.obtainTypoTextBox();
        // java typoTextbox.alloc(snapshot);
        typoTextbox.alloc(snapshot);
        // java drawing.setTextbox(typoTextbox.getPointer());
        drawing.setTextbox__int(typoTextbox.getPointer());
        // java textboxEnv.curPage = typoTextbox;
        textboxEnv.curPage = typoTextbox;
        // java RectF textboxRect = textbox.getTextRect((float)drawing.width(), (float)drawing.height());
        let mut textboxRect: RectF = textbox.getTextRect(((drawing.width()) as f32), ((drawing.height()) as f32));
        // java if (null != textboxRect)
        if true {
          // java typoTextbox.set((int)textboxRect.left, (int)textboxRect.top, (int)textboxRect.right, (int)textboxRect.bottom);
          typoTextbox.set(((textboxRect.left) as i32), ((textboxRect.top) as i32), ((textboxRect.right) as i32), ((textboxRect.bottom) as i32));
        }
        // java else
        else {
          // java typoTextbox.set(0, 0, drawing.width(), drawing.height());
          typoTextbox.set(0, 0, drawing.width(), drawing.height());
        }
        // java DrawingShapeRectTool.resizeTextBoxForShapeBorder(shape, typoTextbox);
        DrawingShapeRectTool::resizeTextBoxForShapeBorder(shape, typoTextbox);
        // java TypoRect oldTxtBoxRt = toolsManager.obtainRect();
        let mut oldTxtBoxRt: TypoRect = toolsManager.obtainRect();
        // java oldTxtBoxRt.set(typoTextbox);
        oldTxtBoxRt.set(typoTextbox);
        // java int textFlow = 0;
        let mut textFlow: i32 = 0;
        // java switch (textbox.getTextFlow())
        match textbox.getTextFlow() {
          // java case 1:
          1 => {
            // java textFlow = 1;
            textFlow = 1;
          }
          // java case 2:
          2 => {
            // java textFlow = 3;
            textFlow = 3;
          }
          // java case 3: case 5:
          3 | 5 => {
            // java textFlow = 5;
            textFlow = 5;
          }
          // java case 4:
          4 => {
            // java textFlow = 4;
            textFlow = 4;
          }
          // java default:
          _ => {
          }
        }
        // java textboxPageEnv.width = typoTextbox.width();
        textboxPageEnv.width = typoTextbox.width();
        // java textboxPageEnv.height = typoTextbox.height();
        textboxPageEnv.height = typoTextbox.height();
        // java boolean wrapText = textbox.getWrapTextInShape();
        let mut wrapText: bool = textbox.getWrapTextInShape();
        // java int textWrapType = textbox.getWrapText();
        let mut textWrapType: i32 = textbox.getWrapText();
        // java boolean wrapNone = textWrapType == 2 && !shape.isChildShape();
        let mut wrapNone: bool = textWrapType == 2 && !shape.isChildShape();
        // java boolean fitShape2Text = textbox.getFitShape2Text();
        let mut fitShape2Text: bool = textbox.getFitShape2Text();
        // java boolean fitText2Shape = textbox.getFitText2Shape();
        let mut fitText2Shape: bool = textbox.getFitText2Shape();
        // java textboxEnv.needCollectExcessInfo = true;
        textboxEnv.needCollectExcessInfo = true;
        // java textboxEnv.needAdjustLastParaBrc = fitShape2Text;
        textboxEnv.needAdjustLastParaBrc = fitShape2Text;
        // java textboxEnv.pageEnv.textFlow = textFlow;
        textboxEnv.pageEnv.textFlow = textFlow;
        // java boolean adjustTextbox2Text = fitShape2Text || (null == textboxRect || wrapNone) && !fitText2Shape && !wrapText;
        let mut adjustTextbox2Text: bool = fitShape2Text || false || wrapNone && !fitText2Shape && !wrapText;
        // java textboxPageEnv.marginTop = ShapeMetrics.transToLayout(textbox.getMarginTop());
        textboxPageEnv.marginTop = ShapeMetrics::transToLayout__float(textbox.getMarginTop());
        // java textboxPageEnv.marginLeft = ShapeMetrics.transToLayout(textbox.getMarginLeft());
        textboxPageEnv.marginLeft = ShapeMetrics::transToLayout__float(textbox.getMarginLeft());
        // java textboxPageEnv.marginRight = ShapeMetrics.transToLayout(textbox.getMarginRight());
        textboxPageEnv.marginRight = ShapeMetrics::transToLayout__float(textbox.getMarginRight());
        // java textboxPageEnv.marginBottom = ShapeMetrics.transToLayout(textbox.getMarginBottom());
        textboxPageEnv.marginBottom = ShapeMetrics::transToLayout__float(textbox.getMarginBottom());
        // java DrawingShapeRectTool.resizeTextBoxMarginsForShapeBorder(shape, textboxPageEnv);
        DrawingShapeRectTool::resizeTextBoxMarginsForShapeBorder(shape, textboxPageEnv);
        // java boolean needSwitchWidthAndHeight = false;
        let mut needSwitchWidthAndHeight: bool = false;
        // java switch (textFlow)
        match textFlow {
          // java case 1:
          1 => {
            // java typoTextbox.setRotationAsia(true);
            typoTextbox.setRotationAsia(true);
          }
          // java case 5:
          5 => {
            // java typoTextbox.setTextFlow(1);
            typoTextbox.setTextFlow(1);
            // java typoTextbox.setMargin(textboxPageEnv.marginRight, textboxPageEnv.marginTop, textboxPageEnv.marginLeft, textboxPageEnv.marginBottom);
            typoTextbox.setMargin(textboxPageEnv.marginRight, textboxPageEnv.marginTop, textboxPageEnv.marginLeft, textboxPageEnv.marginBottom);
            // java needSwitchWidthAndHeight = true;
            needSwitchWidthAndHeight = true;
          }
          // java case 3:
          3 => {
            // java typoTextbox.setTextFlow(3);
            typoTextbox.setTextFlow(3);
            // java typoTextbox.setMargin(textboxPageEnv.marginLeft, textboxPageEnv.marginBottom, textboxPageEnv.marginRight, textboxPageEnv.marginTop);
            typoTextbox.setMargin(textboxPageEnv.marginLeft, textboxPageEnv.marginBottom, textboxPageEnv.marginRight, textboxPageEnv.marginTop);
            // java needSwitchWidthAndHeight = true;
            needSwitchWidthAndHeight = true;
          }
          // java case 4:
          4 => {
            // java typoTextbox.setRotationAsia(true);
            typoTextbox.setRotationAsia(true);
          }
          // java case 2: default:
          2 | _ => {
            // java typoTextbox.setMargin(textboxPageEnv.marginTop, textboxPageEnv.marginLeft, textboxPageEnv.marginBottom, textboxPageEnv.marginRight);
            typoTextbox.setMargin(textboxPageEnv.marginTop, textboxPageEnv.marginLeft, textboxPageEnv.marginBottom, textboxPageEnv.marginRight);
          }
        }
        // java if (adjustTextbox2Text)
        if adjustTextbox2Text {
          // java TypoRectCoreImpl fitRect = TypoRectCoreImpl.obtain();
          let mut fitRect: TypoRectCoreImpl = TypoRectCoreImpl::obtain();
          // java this.calcTextBoxFitRect(fitRect, stateEnvoy);
          self.calcTextBoxFitRect({ let __x: &mut ITypoRect = todo!("no align strategy matched"); __x }, { let __x: &dyn IStateEnvoy = todo!("no align strategy matched"); __x });
          // java if (!wrapText || wrapNone)
          if !wrapText || wrapNone {
            // java int maxTextBoxWidth = fitRect.width();
            let mut maxTextBoxWidth: i32 = fitRect.width();
            // java if (needSwitchWidthAndHeight)
            if needSwitchWidthAndHeight {
              // java textboxPageEnv.height = maxTextBoxWidth;
              textboxPageEnv.height = maxTextBoxWidth;
              // java typoTextbox.setHeight(maxTextBoxWidth);
              typoTextbox.setHeight(maxTextBoxWidth);
            }
            // java else
            else {
              // java textboxPageEnv.width = maxTextBoxWidth;
              textboxPageEnv.width = maxTextBoxWidth;
              // java typoTextbox.setWidth(maxTextBoxWidth);
              typoTextbox.setWidth(maxTextBoxWidth);
            }
          }
          // java if (fitShape2Text)
          if fitShape2Text {
            // java int coreHeight = fitRect.height();
            let mut coreHeight: i32 = fitRect.height();
            // java if (needSwitchWidthAndHeight)
            if needSwitchWidthAndHeight {
              // java textboxPageEnv.width = coreHeight;
              textboxPageEnv.width = coreHeight;
              // java typoTextbox.setWidth(coreHeight);
              typoTextbox.setWidth(coreHeight);
            }
            // java else
            else {
              // java textboxPageEnv.height = coreHeight;
              textboxPageEnv.height = coreHeight;
              // java typoTextbox.setHeight(coreHeight);
              typoTextbox.setHeight(coreHeight);
            }
          }
          // java fitRect.recycle();
          fitRect.recycle();
        }
        // java textboxEnv.cp = textboxEnv.getRangeStart();
        textboxEnv.cp = textboxEnv.getRangeStart();
        // java int oldFieldSize = this.mMainState.fieldRanges.size();
        let mut oldFieldSize: i32 = (self.mMainState.fieldRanges.len() as i32);
        // java this.mTextboxLayouter.flow(textboxEnv);
        self.mTextboxLayouter.flow(textboxEnv);
        // java if (!adjustTextbox2Text)
        if !adjustTextbox2Text {
          // java DrawingResizeTool.resizeTextBoxAsClipRect(typoTextbox, textboxEnv);
          DrawingResizeTool::resizeTextBoxAsClipRect(typoTextbox, textboxEnv);
          // java Integer vObj = textbox.getTextAnchorVObj();
          let mut vObj: i32 = textbox.getTextAnchorVObj();
          // java if (null != vObj && vObj != 0)
          if true && vObj != 0 {
            // java this.vAlignTextBox(typoTextbox, vObj);
            self.vAlignTextBox(&mut typoTextbox, vObj);
          }
          // java snapshot.getTypoTools().recycle(oldTxtBoxRt);
          snapshot.getTypoTools().recycle(oldTxtBoxRt);
        }
        // java else
        else {
          // java boolean needReflowTextbox = !wrapText || wrapNone;
          let mut needReflowTextbox: bool = !wrapText || wrapNone;
          // java if (needReflowTextbox)
          if needReflowTextbox {
            // java int maxCoreWidth = textboxEnv.maxLineCoreWidth;
            let mut maxCoreWidth: i32 = textboxEnv.maxLineCoreWidth;
            // java if (maxCoreWidth < textboxPageEnv.getCoreWidth())
            if maxCoreWidth < textboxPageEnv.getCoreWidth() {
              // java if (needSwitchWidthAndHeight)
              if needSwitchWidthAndHeight {
                // java typoTextbox.setHeight(maxCoreWidth + typoTextbox.getMarginLeft() + typoTextbox.getMarginRight());
                typoTextbox.setHeight(maxCoreWidth + typoTextbox.getMarginLeft() + typoTextbox.getMarginRight());
                // java textboxPageEnv.height = typoTextbox.height();
                textboxPageEnv.height = typoTextbox.height();
              }
              // java else
              else {
                // java typoTextbox.setWidth(maxCoreWidth + typoTextbox.getMarginLeft() + typoTextbox.getMarginRight());
                typoTextbox.setWidth(maxCoreWidth + typoTextbox.getMarginLeft() + typoTextbox.getMarginRight());
                // java textboxPageEnv.width = typoTextbox.width();
                textboxPageEnv.width = typoTextbox.width();
              }
              // java textboxEnv.needCollectExcessInfo = true;
              textboxEnv.needCollectExcessInfo = true;
              // java textboxEnv.needAdjustLastParaBrc = fitShape2Text;
              textboxEnv.needAdjustLastParaBrc = fitShape2Text;
              // java if (fitShape2Text)
              if fitShape2Text {
                // java int coreHeight = 0;
                let mut coreHeight: i32 = 0;
                // java int lastLine = TypoList.last(typoTextbox.getLines(), snapshot);
                let mut lastLine: i32 = TypoList::last__int_typosnapshot(typoTextbox.getLines(), snapshot);
                // java if (0 != lastLine)
                if 0 != lastLine {
                  // java coreHeight = TypoObjBase.getBottom(lastLine, snapshot);
                  coreHeight = TypoObjBase::getBottom__int_typosnapshot(lastLine, snapshot);
                }
                // java if (needSwitchWidthAndHeight)
                if needSwitchWidthAndHeight {
                  // java typoTextbox.setWidth(typoTextbox.getMarginTop() + typoTextbox.getMarginBottom() + coreHeight);
                  typoTextbox.setWidth(typoTextbox.getMarginTop() + typoTextbox.getMarginBottom() + coreHeight);
                  // java textboxPageEnv.width = typoTextbox.width();
                  textboxPageEnv.width = typoTextbox.width();
                }
                // java else
                else {
                  // java typoTextbox.setHeight(typoTextbox.getMarginTop() + typoTextbox.getMarginBottom() + coreHeight);
                  typoTextbox.setHeight(typoTextbox.getMarginTop() + typoTextbox.getMarginBottom() + coreHeight);
                  // java textboxPageEnv.height = typoTextbox.height();
                  textboxPageEnv.height = typoTextbox.height();
                }
              }
              // java int needClearLength = this.mMainState.fieldRanges.size() - oldFieldSize;
              let mut needClearLength: i32 = (self.mMainState.fieldRanges.len() as i32) - oldFieldSize;
              // java this.mMainState.fieldRanges.remove(oldFieldSize, needClearLength);
              self.mMainState.fieldRanges.remove(oldFieldSize, needClearLength);
              // java this.mDocEnv.pools.recycleTableCellLayoutStates(textboxEnv.cellStateList);
              self.mDocEnv.pools.recycleTableCellLayoutStates(textboxEnv.cellStateList);
              // java this.mDocEnv.cacheCleaner.recyclePageContent(textboxEnv.curPage.getPointer());
              self.mDocEnv.cacheCleaner.recyclePageContent(textboxEnv.curPage.getPointer());
              // java textboxEnv.cp = textboxEnv.getRangeStart();
              textboxEnv.cp = textboxEnv.getRangeStart();
              // java this.mTextboxLayouter.flow(textboxEnv);
              self.mTextboxLayouter.flow(textboxEnv);
            }
          }
          // java boolean needScaleDrawing = false;
          let mut needScaleDrawing: bool = false;
          // java if (fitShape2Text)
          if fitShape2Text {
            // java int coreHeight = 0;
            let mut coreHeight: i32 = 0;
            // java int lastLine = TypoList.last(typoTextbox.getLines(), snapshot);
            let mut lastLine: i32 = TypoList::last__int_typosnapshot(typoTextbox.getLines(), snapshot);
            // java if (0 != lastLine)
            if 0 != lastLine {
              // java coreHeight = TypoObjBase.getBottom(lastLine, snapshot);
              coreHeight = TypoObjBase::getBottom__int_typosnapshot(lastLine, snapshot);
              // java int type = TypoObjBase.getType(lastLine, snapshot);
              let mut r#type: i32 = TypoObjBase::getType(lastLine, snapshot);
              // java if (type == 3)
              if r#type == 3 {
                // java int pLastSHDAndBRCs = TypoTextLine.getSHDAndBRCs(lastLine, snapshot);
                let mut pLastSHDAndBRCs: i32 = TypoTextLine::getSHDAndBRCs__int_typosnapshot(lastLine, snapshot);
                // java if (0 != pLastSHDAndBRCs)
                if 0 != pLastSHDAndBRCs {
                  // java TypoSHDAndBRCs typoSHDAndBRCs = toolsManager.obtainTypoSHDAndBRCs(pLastSHDAndBRCs);
                  let mut typoSHDAndBRCs: TypoSHDAndBRCs = toolsManager.obtainTypoSHDAndBRCs(pLastSHDAndBRCs);
                  // java if (typoSHDAndBRCs.size() > 0)
                  if (typoSHDAndBRCs.len() as i32) > 0 {
                    // java int lastIndex = typoSHDAndBRCs.size() - 1;
                    let mut lastIndex: i32 = (typoSHDAndBRCs.len() as i32) - 1;
                    // java int bottom = typoSHDAndBRCs.getBottom(lastIndex);
                    let mut bottom: i32 = typoSHDAndBRCs.getBottom(lastIndex);
                    // java if (bottom >= coreHeight)
                    if bottom >= coreHeight {
                      // java coreHeight = bottom;
                      coreHeight = bottom;
                      // java BRC brc = (BRC)snapshot.getObj(typoSHDAndBRCs.getBrcBottom(lastIndex));
                      let mut brc: BRC = (snapshot.getObj(typoSHDAndBRCs.getBrcBottom(lastIndex))).downcast_ref::<BRC>().unwrap();
                      // java if (null != brc)
                      if true {
                        // java coreHeight = (int)((float)bottom + BRCBorderTool.getActualStrokeWidth(brc.getBorderType(), brc.getLineWidth()) / 2.0F);
                        coreHeight = ((((bottom) as f32) + BRCBorderTool::getActualStrokeWidth(brc.getBorderType(), brc.getLineWidth()) / 2) as i32);
                      }
                    }
                  }
                  // java toolsManager.recycle((TypoMemoryObj)typoSHDAndBRCs);
                  toolsManager.recycle((typoSHDAndBRCs).downcast_ref::<TypoMemoryObj>().unwrap());
                }
              }
            }
            // java if (needSwitchWidthAndHeight)
            if needSwitchWidthAndHeight {
              // java typoTextbox.setWidth(typoTextbox.getMarginTop() + typoTextbox.getMarginBottom() + coreHeight);
              typoTextbox.setWidth(typoTextbox.getMarginTop() + typoTextbox.getMarginBottom() + coreHeight);
            }
            // java else
            else {
              // java typoTextbox.setHeight(typoTextbox.getMarginTop() + typoTextbox.getMarginBottom() + coreHeight);
              typoTextbox.setHeight(typoTextbox.getMarginTop() + typoTextbox.getMarginBottom() + coreHeight);
            }
            // java needScaleDrawing = true;
            needScaleDrawing = true;
          }
          // java else if (!fitText2Shape && !wrapText)
          else if !fitText2Shape && !wrapText {
            // java needScaleDrawing = true;
            needScaleDrawing = true;
          }
          // java else if (wrapNone)
          else if wrapNone {
            // java needScaleDrawing = true;
            needScaleDrawing = true;
          }
          // java if (needScaleDrawing)
          if needScaleDrawing {
            // java float xScale = (float)typoTextbox.width() / (float)oldTxtBoxRt.width();
            let mut xScale: f32 = ((typoTextbox.width()) as f32) / ((oldTxtBoxRt.width()) as f32);
            // java float yScale = (float)typoTextbox.height() / (float)oldTxtBoxRt.height();
            let mut yScale: f32 = ((typoTextbox.height()) as f32) / ((oldTxtBoxRt.height()) as f32);
            // java drawing.setWidth((int)((float)drawing.width() * xScale));
            drawing.setWidth(((((drawing.width()) as f32) * xScale) as i32));
            // java drawing.setHeight((int)((float)drawing.height() * yScale));
            drawing.setHeight(((((drawing.height()) as f32) * yScale) as i32));
            // java typoTextbox.offsetTo((int)((float)oldTxtBoxRt.getLeft() * xScale), (int)((float)oldTxtBoxRt.getTop() * yScale));
            typoTextbox.offsetTo(((((oldTxtBoxRt.getLeft()) as f32) * xScale) as i32), ((((oldTxtBoxRt.getTop()) as f32) * yScale) as i32));
          }
          // java else
          else {
            // java if (typoTextbox.width() > drawing.width())
            if typoTextbox.width() > drawing.width() {
              // java typoTextbox.setWidth(drawing.width());
              typoTextbox.setWidth(drawing.width());
            }
            // java if (typoTextbox.height() > drawing.height())
            if typoTextbox.height() > drawing.height() {
              // java typoTextbox.setHeight(drawing.height());
              typoTextbox.setHeight(drawing.height());
            }
          }
          // java DrawingResizeTool.resizeTextBoxAsClipRect(typoTextbox, textboxEnv);
          DrawingResizeTool::resizeTextBoxAsClipRect(typoTextbox, textboxEnv);
          // java toolsManager.recycle(oldTxtBoxRt);
          toolsManager.recycle(oldTxtBoxRt);
          // java toolsManager.recycle((TypoMemoryObj)textboxEnv.curPage);
          toolsManager.recycle((textboxEnv.curPage).downcast_ref::<TypoMemoryObj>().unwrap());
          // java textboxEnv.curPage = null;
          textboxEnv.curPage = None;
        }
      }
      // java else
      else {
        // java drawing.setTextbox(0);
        drawing.setTextbox__int(0);
      }
      */
    }

    // java private void vAlignTextBox(TypoTextBox typoTextbox, int vTextAnchor)
    pub fn vAlignTextBox(&self, typoTextbox: &mut TypoTextBox, vTextAnchor: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (typoTextbox.hasLines())
      if typoTextbox.hasLines() {
        // java TypoSnapshot snapshot = this.mMainState.snapshot;
        let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
        // java int maxBottom = 0;
        let mut maxBottom: i32 = 0;
        // java int lastLine = TypoList.last(typoTextbox.getLines(), snapshot);
        let mut lastLine: i32 = TypoList::last__int_typosnapshot(typoTextbox.getLines(), snapshot);
        // java maxBottom = TypoObjBase.getBottom(lastLine, snapshot);
        maxBottom = TypoObjBase::getBottom__int_typosnapshot(lastLine, snapshot);
        // java if (TypoMemoryObj.getType(lastLine, snapshot) == 3)
        if TypoMemoryObj::getType__int_typosnapshot(lastLine, snapshot) == 3 {
          // java TypoSHDAndBRCs lastSHDAndBRC = snapshot.getTypoTools().obtainTypoSHDAndBRCs(TypoTextLine.getSHDAndBRCs(lastLine, snapshot));
          let mut lastSHDAndBRC: TypoSHDAndBRCs = snapshot.getTypoTools().obtainTypoSHDAndBRCs(TypoTextLine::getSHDAndBRCs__int_typosnapshot(lastLine, snapshot));
          // java if (lastSHDAndBRC.size() > 0)
          if (lastSHDAndBRC.len() as i32) > 0 {
            // java maxBottom = Math.max(maxBottom, lastSHDAndBRC.getBottom(0));
            maxBottom = (maxBottom).max(lastSHDAndBRC.getBottom(0));
          }
          // java snapshot.getTypoTools().recycle((TypoMemoryObj)lastSHDAndBRC);
          snapshot.getTypoTools().recycle((lastSHDAndBRC).downcast_ref::<TypoMemoryObj>().unwrap());
        }
        // java int spaceAfterLastLine = typoTextbox.getCoreHeight() - maxBottom;
        let mut spaceAfterLastLine: i32 = typoTextbox.getCoreHeight() - maxBottom;
        // java if (spaceAfterLastLine > 0)
        if spaceAfterLastLine > 0 {
          // java int vOffset = 0;
          let mut vOffset: i32 = 0;
          // java switch (vTextAnchor)
          match vTextAnchor {
            // java case 1: case 4:
            1 | 4 => {
              // java vOffset = spaceAfterLastLine / 2;
              vOffset = spaceAfterLastLine / 2;
            }
            // java case 2: case 5:
            2 | 5 => {
              // java vOffset = spaceAfterLastLine;
              vOffset = spaceAfterLastLine;
            }
            // java case 3:
            3 => {
            }
            // java default:
            _ => {
            }
          }
          // java int pLines = typoTextbox.getLines();
          let mut pLines: i32 = typoTextbox.getLines();
          // java if (pLines != 0)
          if pLines != 0 {
            let mut i: i32 = 0;
            // java for(int i = 0; i < TypoList.size(pLines, snapshot); ++i)
            while i < TypoList::size__int_typosnapshot(pLines, snapshot) {
              // java int line = TypoList.quickGet(i, pLines, snapshot);
              let mut line: i32 = TypoList::quickGet__int_int_typosnapshot(i, pLines, snapshot);
              // java if (line != 0)
              if line != 0 {
                // java TypoObjBase.offset(0, vOffset, line, snapshot);
                TypoObjBase::offset__int_int_int_typosnapshot(0, vOffset, line, snapshot);
              }
            }
          }
        }
      }
      */
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mGetTextWrapLayouter = null;
      self.mGetTextWrapLayouter = None;
      // java if (null != this.mTextboxLayouter)
      if let Some(mTextboxLayouter) = self.mTextboxLayouter.as_mut() {
        // java this.mTextboxLayouter.dispose();
        mTextboxLayouter.dispose();
        // java this.mTextboxLayouter = null;
        mTextboxLayouter = None;
      }
      // java if (null != this.mNormalCollosion)
      if let Some(mNormalCollosion) = self.mNormalCollosion.as_mut() {
        // java this.mNormalCollosion.dispose();
        mNormalCollosion.dispose();
        // java this.mNormalCollosion = null;
        mNormalCollosion = None;
      }
      // java if (null != this.mSubLineRects)
      if let Some(mSubLineRects) = self.mSubLineRects.as_mut() {
        // java this.mSubLineRects.dispose();
        mSubLineRects.dispose();
        // java this.mSubLineRects = null;
        mSubLineRects = None;
      }
      // java super.dispose();
      self.super_.dispose();
      */
    }

    // java private int getAnchorHoriPos(TypoDrawing drawing, int anchorLine, boolean needRelFloatsPage, IStateEnvoy stateEnvoy)
    pub fn getAnchorHoriPos(&self, drawing: &TypoDrawing, anchorLine: i32, needRelFloatsPage: bool, stateEnvoy: &dyn IStateEnvoy) -> i32 {
      let floats_page = self.getFloatContainerPage(stateEnvoy);
      self.anchor_hori_pos(drawing, anchorLine, floats_page.getPointer(), needRelFloatsPage, stateEnvoy)
      /* DEFERRED_NON_FUNCTION
      // java if (3 != TypoMemoryObj.getType(anchorLine, this.mMainState.snapshot))
      if 3 != TypoMemoryObj::getType__int_typosnapshot(anchorLine, self.mMainState.snapshot) {
        // java return 0;
        return 0;
      }
      // java else
      else {
        // java int pos = LocateTool.getAnchorPos(anchorLine, stateEnvoy.getDocument(), drawing.getCP(), this.mMainState.snapshot);
        let mut pos: i32 = LocateTool::getAnchorPos(anchorLine, stateEnvoy.getDocument(), drawing.getCP(), self.mMainState.snapshot);
        // java int offsetX = 0;
        let mut offsetX: i32 = 0;
        // java if (needRelFloatsPage)
        if needRelFloatsPage {
          // java int pLinesPage = this.getLineContainerPage(drawing, anchorLine, stateEnvoy);
          let mut pLinesPage: i32 = self.getLineContainerPage(drawing, anchorLine, stateEnvoy);
          // java TypoPage linesPage = (TypoPage)this.mMainState.snapshot.getTypoTools().obtain(pLinesPage);
          let mut linesPage: TypoPage = (self.mMainState.snapshot.getTypoTools().obtain(pLinesPage)).downcast_ref::<TypoPage>().unwrap();
          // java TypoPage floatsPage = this.getFloatContainerPage(stateEnvoy);
          let mut floatsPage: TypoPage = self.getFloatContainerPage(stateEnvoy);
          // java long ret = DrawingPositionTool.getColPageDrawingOffsetResult(linesPage, floatsPage);
          let mut ret: i64 = DrawingPositionTool::getColPageDrawingOffsetResult(linesPage, floatsPage);
          // java offsetX = DrawingPositionTool.getColPageDrawingOffsetX(ret);
          offsetX = DrawingPositionTool::getColPageDrawingOffsetX(ret);
          // java this.mMainState.snapshot.getTypoTools().recycle((TypoMemoryObj)linesPage);
          self.mMainState.snapshot.getTypoTools().recycle((linesPage).downcast_ref::<TypoMemoryObj>().unwrap());
        }
        // java return offsetX + pos;
        return offsetX + pos;
      }
      */
    }

    // java protected TypoPage getEmbedContainerPage(IStateEnvoy stateEnvoy)
    pub fn getEmbedContainerPage(&self, stateEnvoy: &dyn IStateEnvoy) -> TypoPage {
      // java return stateEnvoy.getCurPage();
      stateEnvoy.getCurPage()
    }

    // java protected TypoPage getFloatContainerPage(IStateEnvoy stateEnvoy)
    pub fn getFloatContainerPage(&self, stateEnvoy: &dyn IStateEnvoy) -> TypoPage {
      // java return stateEnvoy.getCurPage();
      return stateEnvoy.getCurPage();
    }

    // java protected int getLineContainerPage(TypoDrawing drawing, int anchorLine, IStateEnvoy stateEnvoy)
    pub fn getLineContainerPage(&self, drawing: &TypoDrawing, anchorLine: i32, stateEnvoy: &dyn IStateEnvoy) -> i32 {
      let _ = drawing;
      if anchorLine != 0 {
        crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getPage__int_typosnapshot(anchorLine, &self.mMainState.snapshot)
      } else {
        stateEnvoy.getCurPage().getPointer()
      }
    }

    // java protected int getHoriAlignPage(TypoDrawing drawing, int anchorLine, int relX, IStateEnvoy stateEnvoy)
    pub fn getHoriAlignPage(&self, drawing: &TypoDrawing, anchorLine: i32, relX: i32, stateEnvoy: &dyn IStateEnvoy) -> i32 {
      if DrawingPositionTool::isHoriPosOfLineContainerPage__int(relX) {
        self.getLineContainerPage(drawing, anchorLine, stateEnvoy)
      } else {
        self.getFloatContainerPage(stateEnvoy).getPointer()
      }
    }

    // java protected int getVertAlignPage(TypoDrawing drawing, int anchorLine, int relY, IStateEnvoy stateEnvoy)
    pub fn getVertAlignPage(&self, drawing: &TypoDrawing, anchorLine: i32, relY: i32, stateEnvoy: &dyn IStateEnvoy) -> i32 {
      if DrawingPositionTool::isVertPosOfLineContainerPage__int(relY) {
        self.getLineContainerPage(drawing, anchorLine, stateEnvoy)
      } else {
        self.getFloatContainerPage(stateEnvoy).getPointer()
      }
    }

    // java public void addDrawings(KPList<DrawingNode> nodeList, IStateEnvoy stateEnvoy)
    pub fn addDrawings(&mut self, nodeList: &mut KPList<DrawingNode>, stateEnvoy: &mut dyn IStateEnvoy) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (!nodeList.isEmpty())
      if !nodeList.is_empty() {
        // java KList<DrawingCacheItem> floatsCache = this.mMainState.floatsCache;
        let mut floatsCache: KList = self.mMainState.floatsCache;
        // java TypoPage floatsPage = this.getFloatContainerPage(stateEnvoy);
        let mut floatsPage: TypoPage = self.getFloatContainerPage({ let __x: &dyn IStateEnvoy = todo!("no align strategy matched"); __x });
        // java KPList.Iterator<DrawingNode> it = nodeList.iterator().seekBegin();
        let mut it: Vec<DrawingNode> = nodeList.iterator().seekBegin();
        // java DrawingNode node = null;
        let mut node: DrawingNode = Default::default();
        // java while (null != (node = (DrawingNode)it.next()))
        while { node = (it.next()).downcast_ref::<DrawingNode>().unwrap(); node }.is_some() {
          // java this.doAddFloatCache(floatsCache, floatsPage, node.shape, node.cp, stateEnvoy);
          self.doAddFloatCache({ let __x: &mut KList<DrawingCacheItem> = todo!("no align strategy matched"); __x }, &floatsPage, &node.shape, node.cp, stateEnvoy);
        }
        // java nodeList.clean();
        nodeList.clean();
        // java it.recycle();
        it.recycle();
      }
      */
    }

    // java DrawingLayouter.getEmbed body: alloc, parent, type, clone, embed, preflow, horz rule, recycle pointer.
    pub fn preflow_embed_pointer(
        snapshot: &mut crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot,
        shape: &std::sync::Arc<std::sync::Mutex<drawing::cn::wps::moffice::drawing::Shape::Shape>>,
        parent_pointer: i32,
        global_page_idx: i32,
        core_width: i32,
        left_indent: i32,
        right_indent: i32,
        visual_textflow: i32,
    ) -> i32 {
      let mut drawing = snapshot.getTypoTools().obtainTypoDrawing();
      // java drawing.alloc(snapshot); setParent; setDrawingType(0); setCloneShape; setEmbed(true);
      drawing.alloc(snapshot);
      drawing.setParent(parent_pointer);
      drawing.setDrawingType(0);
      drawing.setCloneShape(shape.clone());
      drawing.setEmbed(true);
      let mut layouter = DrawingLayouter::default();
      layouter.mMainState.curPageIndex = global_page_idx;
      if let Ok(mut guard) = shape.lock() {
        layouter.doPreFlowEmbedDrawing(&mut drawing, &mut guard, global_page_idx, visual_textflow);
        crate::cn::wps::moffice::writer::layout::base::drawing::DrawingShapeRectTool::DrawingShapeRectTool::apply_horz_rule(&mut drawing, &mut guard, core_width, left_indent, right_indent);
      }
      let ret = drawing.getPointer();
      snapshot.getTypoTools().recycle_drawing(&mut drawing);
      ret
    }

    // java public int getEmbed(Shape shape, IStateEnvoy stateEnvoy)
    // parent/core/indent/textflow are the envoy reads Java performs inside this method.
    pub fn getEmbed(
        &mut self,
        shape: &std::sync::Arc<std::sync::Mutex<drawing::cn::wps::moffice::drawing::Shape::Shape>>,
        parent_pointer: i32,
        core_width: i32,
        left_indent: i32,
        right_indent: i32,
        visual_textflow: i32,
    ) -> i32 {
      let page_index = self.mMainState.curPageIndex;
      let snapshot = &mut self.mMainState.snapshot;
      Self::preflow_embed_pointer(snapshot, shape, parent_pointer, page_index, core_width, left_indent, right_indent, visual_textflow)
      /* DEFERRED_NON_FUNCTION
      // java if (null == shape)
      if false {
        // java return 0;
        return 0;
      }
      // java else
      else {
        // java TypoSnapshot snapshot = this.mMainState.snapshot;
        let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
        // java TypoPage parentPage = this.getEmbedContainerPage(stateEnvoy);
        let mut parentPage: TypoPage = self.getEmbedContainerPage(stateEnvoy);
        // java TypoDrawing drawing = snapshot.getTypoTools().obtainTypoDrawing();
        let mut drawing: TypoDrawing = snapshot.getTypoTools().obtainTypoDrawing();
        // java drawing.alloc(snapshot);
        drawing.alloc(snapshot);
        // java drawing.setParent(parentPage.getPointer());
        drawing.setParent(parentPage.getPointer());
        // java drawing.setDrawingType(0);
        drawing.setDrawingType(0);
        // java drawing.setCloneShape(shape);
        drawing.setCloneShape(shape);
        // java drawing.setEmbed(true);
        drawing.setEmbed(true);
        // java this.doPreFlowEmbedDrawing(drawing, shape, stateEnvoy);
        self.doPreFlowEmbedDrawing(&mut drawing, shape, &mut stateEnvoy);
        // java if (shape.getHorzRule())
        if shape.getHorzRule() {
          // java PageEnv pageEnv = stateEnvoy.getPageEnv();
          let mut pageEnv: PageEnv = stateEnvoy.getPageEnv();
          // java ParaEnv paraEnv = stateEnvoy.getParaEnv();
          let mut paraEnv: ParaEnv = stateEnvoy.getParaEnv();
          // java int maxWidth = Math.max(pageEnv.getCoreWidth() - paraEnv.curParaPropEnv.leftIndent - paraEnv.curParaPropEnv.rightIndent, 0);
          let mut maxWidth: i32 = (pageEnv.getCoreWidth() - paraEnv.curParaPropEnv.leftIndent - paraEnv.curParaPropEnv.rightIndent).max(0);
          // java float hrScale = shape.getHorzRuleScale();
          let mut hrScale: f32 = shape.getHorzRuleScale();
          // java if (hrScale > 0.0F)
          if hrScale > 0 {
            // java drawing.setWidth((int)(hrScale * (float)pageEnv.getCoreWidth()));
            drawing.setWidth(((hrScale * ((pageEnv.getCoreWidth()) as f32)) as i32));
          }
          // java if (drawing.width() > maxWidth)
          if drawing.width() > maxWidth {
            // java drawing.setWidth(maxWidth);
            drawing.setWidth(maxWidth);
          }
          // java if (drawing.height() <= 0)
          if drawing.height() <= 0 {
            // java drawing.setHeight(20);
            drawing.setHeight(20);
          }
          // java int layoutRectPadding = drawing.getLayoutPadding();
          let mut layoutRectPadding: i32 = drawing.getLayoutPadding();
          // java switch (shape.getHorzRuleAlignment())
          match shape.getHorzRuleAlignment() {
            // java case 1:
            1 => {
              // java int halfOfPadding = (maxWidth - drawing.width()) / 2;
              let mut halfOfPadding: i32 = maxWidth - drawing.width() / 2;
              // java if (halfOfPadding > 0)
              if halfOfPadding > 0 {
                // java TypoRectMemoryImpl.setLeft(halfOfPadding, layoutRectPadding, snapshot);
                TypoRectMemoryImpl::setLeft__int_int_typosnapshot(halfOfPadding, layoutRectPadding, snapshot);
                // java TypoRectMemoryImpl.setRight(halfOfPadding, layoutRectPadding, snapshot);
                TypoRectMemoryImpl::setRight__int_int_typosnapshot(halfOfPadding, layoutRectPadding, snapshot);
              }
            }
            // java case 2:
            2 => {
              // java TypoRectMemoryImpl.setLeft(maxWidth - drawing.width(), layoutRectPadding, snapshot);
              TypoRectMemoryImpl::setLeft__int_int_typosnapshot(maxWidth - drawing.width(), layoutRectPadding, snapshot);
            }
            // java default:
            _ => {
              // java TypoRectMemoryImpl.setRight(maxWidth - drawing.width(), layoutRectPadding, snapshot);
              TypoRectMemoryImpl::setRight__int_int_typosnapshot(maxWidth - drawing.width(), layoutRectPadding, snapshot);
            }
          }
        }
        // java int ret = drawing.getPointer();
        let mut ret: i32 = drawing.getPointer();
        // java snapshot.getTypoTools().recycle((TypoMemoryObj)drawing);
        snapshot.getTypoTools().recycle((drawing).downcast_ref::<TypoMemoryObj>().unwrap());
        // java return ret;
        return ret;
      }
      */
    }

    // java protected TypoDrawing getFloatDrawing(TypoPage floatsPage, Shape shape, int cp, IStateEnvoy stateEnvoy)
    pub fn getFloatDrawing(
        &mut self,
        parent: i32,
        shape: &std::sync::Arc<std::sync::Mutex<drawing::cn::wps::moffice::drawing::Shape::Shape>>,
        cp: i32,
        doc: Option<&dyn crate::cn::wps::moffice::writer::core::Document::Document>,
    ) -> i32 {
      let mut drawing = self.mMainState.snapshot.getTypoTools().obtainTypoDrawing();
      drawing.alloc(&mut self.mMainState.snapshot);
      drawing.setParent(parent);
      drawing.setDrawingType(0);
      let cloned = shape.lock().ok().and_then(|mut guard| guard.cloneLightWeight_default().ok());
      if let Some(clone_shape) = cloned {
        let arc = std::sync::Arc::new(std::sync::Mutex::new(clone_shape));
        let text_wrap = arc.lock().ok().and_then(|mut guard| {
          guard.getShapePos().and_then(|pos| {
            pos.downcast_ref::<drawing::cn::wps::moffice::drawing::ShapePos::ShapePos>().map(|p| p.getTextWrap())
          })
        });
        drawing.setCloneShape(arc);
        if text_wrap.is_some_and(|wrap| DrawingPositionTool::isWrap__int(wrap)) {
          drawing.setWrap(true);
        }
      }
      if let Some(document) = doc {
        drawing.setCP(cp, document);
      }
      let ret = drawing.getPointer();
      self.mMainState.snapshot.getTypoTools().recycle_drawing(&mut drawing);
      ret
      /* DEFERRED_NON_FUNCTION
      // java TypoDrawing drawing = this.mMainState.snapshot.getTypoTools().obtainTypoDrawing();
      let mut drawing: TypoDrawing = self.mMainState.snapshot.getTypoTools().obtainTypoDrawing();
      // java drawing.alloc(this.mMainState.snapshot);
      drawing.alloc(self.mMainState.snapshot);
      // java drawing.setParent(floatsPage.getPointer());
      drawing.setParent(floatsPage.getPointer());
      // java drawing.setDrawingType(0);
      drawing.setDrawingType(0);
      // java Shape cloneShape = null;
      let mut cloneShape: Shape = Default::default();
          // java cloneShape = shape.cloneLightWeight();
          cloneShape = shape.cloneLightWeight();
          // java drawing.setCloneShape(cloneShape);
          drawing.setCloneShape(cloneShape);
          // java CloneNotSupportedException e
          todo!();
            // java e.printStackTrace();
            e.printStackTrace();
      // java drawing.setCP(cp, stateEnvoy.getDocument());
      drawing.setCP(cp, stateEnvoy.getDocument());
      // java if (cloneShape != null)
      if true {
        // java this.changeShapePos(cloneShape);
        self.changeShapePos(&mut cloneShape);
        // java if (DrawingPositionTool.isWrap(cloneShape.getShapePos()))
        if DrawingPositionTool::isWrap(cloneShape.getShapePos()) {
          // java drawing.setWrap(true);
          drawing.setWrap(true);
        }
      }
      // java return drawing;
      return Some(drawing);
      */
    }

    // java protected void doAddFloatCache(KList<DrawingCacheItem> floatsCache, TypoPage floatsPage, Shape shape, int anchorCP, IStateEnvoy stateEnvoy)
    pub fn doAddFloatCache(
        &mut self,
        shape: &std::sync::Arc<std::sync::Mutex<drawing::cn::wps::moffice::drawing::Shape::Shape>>,
        anchor_cp: i32,
        doc: Option<&dyn crate::cn::wps::moffice::writer::core::Document::Document>,
        parent: i32,
        doc_type: i32,
    ) {
      let flows = shape.lock().ok().map(|mut guard| self.needFlow(&mut guard)).unwrap_or(false);
      if !flows { return; }
      let Some(document) = doc else { return; };
      let layers_ok = self.mMainState.getDrawingLayers(document).is_some();
      if !layers_ok { return; }
      let valid = self.mMainState.getDrawingLayers(document).map(|layers| layers.isValid()).unwrap_or(false);
      let mut p_drawing = if valid {
        self.mMainState.getDrawingLayers(document).map(|layers| layers.getFloat(anchor_cp, parent)).unwrap_or(0)
      } else {
        0
      };
      if p_drawing == 0 {
        if let Some(idx) = self.mMainState.floatsCache.iter().position(|item| item.cp == anchor_cp) {
          let drawing_ptr = self.mMainState.floatsCache[idx].drawing;
          let mut drawing = self.mMainState.snapshot.getTypoTools().obtainTypoDrawing__int(drawing_ptr);
          if drawing.getShapePosElement().is_some() {
            p_drawing = drawing_ptr;
            let before_2013 = self.before_2013_mode();
            let page_idx = self.mMainState.curPageIndex;
            if let Some(pos) = drawing.getShapePosElement() {
              if let Some(shape_pos) = pos.downcast_ref::<drawing::cn::wps::moffice::drawing::ShapePos::ShapePos>() {
                let rel_x = DrawingPositionTool::normalizeRelX__int_int(shape_pos.getRelativeX(), page_idx);
                let rel_y = DrawingPositionTool::normalizeRelY__int_int_boolean(shape_pos.getRelativeY(), page_idx, before_2013);
                self.mMainState.floatsCache[idx].relX = rel_x;
                self.mMainState.floatsCache[idx].relY = rel_y;
              }
            }
            if let Some(item_shape) = self.mMainState.floatsCache[idx].shape.clone() {
              if let Ok(mut guard) = item_shape.lock() {
                let rel_x = self.mMainState.floatsCache[idx].relX;
                let rel_y = self.mMainState.floatsCache[idx].relY;
                self.setDrawingShapeRect(&mut drawing, &mut guard, rel_x, rel_y, false, false, page_idx);
              }
            }
          }
          self.mMainState.snapshot.getTypoTools().recycle_drawing(&mut drawing);
        }
      }
      if p_drawing == 0 {
        let mut new_item = self.mDocEnv.pools.obtainDrawingCacheItem();
        new_item.cp = anchor_cp;
        new_item.docType = doc_type;
        new_item.shape = Some(shape.clone());
        let pointer = self.getFloatDrawing(parent, shape, anchor_cp, Some(document));
        new_item.drawing = pointer;
        let mut drawing = self.mMainState.snapshot.getTypoTools().obtainTypoDrawing__int(pointer);
        if drawing.getShapePosElement().is_some() {
          let before_2013 = self.before_2013_mode();
          let page_idx = self.mMainState.curPageIndex;
          let (rel_x, rel_y) = if let Some(pos) = drawing.getShapePosElement() {
            if let Some(shape_pos) = pos.downcast_ref::<drawing::cn::wps::moffice::drawing::ShapePos::ShapePos>() {
              (
                DrawingPositionTool::normalizeRelX__int_int(shape_pos.getRelativeX(), page_idx),
                DrawingPositionTool::normalizeRelY__int_int_boolean(shape_pos.getRelativeY(), page_idx, before_2013),
              )
            } else { (0, 0) }
          } else { (0, 0) };
          new_item.relX = rel_x;
          new_item.relY = rel_y;
          if let Ok(mut guard) = shape.lock() {
            self.doPreFlowDrawing(&mut drawing, &mut guard, rel_x, rel_y, false, page_idx);
          }
          self.add_sorted_float(new_item);
        }
        self.mMainState.snapshot.getTypoTools().recycle_drawing(&mut drawing);
      }
    }

    fn before_2013_mode(&mut self) -> bool {
      self.mDocEnv.textDocument_mut()
        .map(|doc| doc.getCompatibilityMode().isBefore2013Mode())
        .unwrap_or(true)
    }

    fn add_sorted_float(&mut self, node: crate::cn::wps::moffice::writer::layout::base::env::DrawingCacheItem::DrawingCacheItem) {
      let cp = node.cp;
      let pos = self.mMainState.floatsCache.iter().position(|item| item.cp >= cp).unwrap_or(self.mMainState.floatsCache.len());
      // Java walks while iter.cp < node.cp, so the insert point is the first item with cp >= node.cp.
      // If that item's cp is equal, Java still inserts before it because the loop stops when cp < nodeCP is false.
      let pos = if pos < self.mMainState.floatsCache.len() && self.mMainState.floatsCache[pos].cp >= cp {
        pos
      } else {
        self.mMainState.floatsCache.len()
      };
      // The last-element fast path appends when last.cp < node.cp. position() already does that.
      let _ = pos;
      let insert_at = self.mMainState.floatsCache.iter().position(|item| item.cp >= cp).unwrap_or(self.mMainState.floatsCache.len());
      self.mMainState.floatsCache.insert(insert_at, node);
    }
      /* DEFERRED_NON_FUNCTION
      // java if (this.needFlow(shape))
      if self.needFlow(shape) {
        // java TypoDrawingLayers drawingLayers = this.mMainState.getDrawingLayers(stateEnvoy.getDocument());
        let mut drawingLayers: TypoDrawingLayers = self.mMainState.getDrawingLayers__document(stateEnvoy.getDocument());
        // java if (null != drawingLayers)
        if true {
          // java int pDrawing = drawingLayers.isValid() ? drawingLayers.getFloat(anchorCP, floatsPage.getPointer()) : 0;
          let mut pDrawing: i32 = (if drawingLayers.isValid() { drawingLayers.getFloat(anchorCP, floatsPage.getPointer()) } else { 0 });
          // java if (0 == pDrawing)
          if 0 == pDrawing {
            // java DrawingCacheItem item = DrawingCacheTool.getCacheItemByCP(floatsCache, anchorCP);
            let mut item: DrawingCacheItem = DrawingCacheTool::getCacheItemByCP(floatsCache, anchorCP);
            // java if (null != item)
            if true {
              // java IPos pos = TypoDrawing.getPos(item.drawing, this.mMainState.snapshot);
              let mut pos: Box<dyn IPos> = TypoDrawing::getPos__int_typosnapshot(item.drawing, self.mMainState.snapshot);
              // java if (pos != null)
              if true {
                // java pDrawing = item.drawing;
                pDrawing = item.drawing;
                // java TypoDrawing drawing = this.mMainState.snapshot.getTypoTools().obtainTypoDrawing(pDrawing);
                let mut drawing: TypoDrawing = self.mMainState.snapshot.getTypoTools().obtainTypoDrawing(pDrawing);
                // java if (drawing.getPos() != null)
                if drawing.getPos().is_some() {
                  // java DrawingCacheTool.calcDrawingRelXAndRelY(drawing.getPos(), this.getFloatContainerPage(stateEnvoy), this.mMainState.curPageIndex, item, this.mDocEnv.textDocument.getCompatibilityMode().isBefore2013Mode());
                  DrawingCacheTool::calcDrawingRelXAndRelY(drawing.getPos(), self.getFloatContainerPage({ let __x: &dyn IStateEnvoy = todo!("no align strategy matched"); __x }), self.mMainState.curPageIndex, item, self.mDocEnv.textDocument.getCompatibilityMode().isBefore2013Mode());
                }
                // java this.setDrawingShapeRect(drawing, item.shape, item.relX, item.relY, false, false, stateEnvoy);
                self.setDrawingShapeRect(&mut drawing, &item.shape, item.relX, item.relY, false, false, { let __x: &dyn IStateEnvoy = todo!("no align strategy matched"); __x });
                // java this.mMainState.snapshot.getTypoTools().recycle((TypoMemoryObj)drawing);
                self.mMainState.snapshot.getTypoTools().recycle((drawing).downcast_ref::<TypoMemoryObj>().unwrap());
              }
            }
          }
          // java if (0 == pDrawing)
          if 0 == pDrawing {
            // java DrawingCacheItem newItem = this.mDocEnv.pools.obtainDrawingCacheItem();
            let mut newItem: DrawingCacheItem = self.mDocEnv.pools.obtainDrawingCacheItem();
            // java newItem.cp = anchorCP;
            newItem.cp = anchorCP;
            // java newItem.docType = floatsPage.getDocumentType();
            newItem.docType = floatsPage.getDocumentType();
            // java newItem.shape = shape;
            newItem.shape = shape;
            // java TypoDrawing drawing = this.getFloatDrawing(floatsPage, shape, anchorCP, stateEnvoy);
            let mut drawing: TypoDrawing = self.getFloatDrawing(floatsPage, shape, anchorCP, { let __x: &dyn IStateEnvoy = todo!("no align strategy matched"); __x }).unwrap();
            // java newItem.drawing = drawing.getPointer();
            newItem.drawing = drawing.getPointer();
            // java IPos pos = drawing.getPos();
            let mut pos: Box<dyn IPos> = drawing.getPos();
            // java if (pos != null)
            if true {
              // java int globalPageIdx = this.mMainState.curPageIndex;
              let mut globalPageIdx: i32 = self.mMainState.curPageIndex;
              // java int relX = DrawingPositionTool.normalizeRelX(pos, floatsPage, globalPageIdx);
              let mut relX: i32 = DrawingPositionTool::normalizeRelX(pos, floatsPage, globalPageIdx);
              // java int relY = DrawingPositionTool.normalizeRelY(pos, floatsPage, globalPageIdx, this.mDocEnv.textDocument.getCompatibilityMode().isBefore2013Mode());
              let mut relY: i32 = DrawingPositionTool::normalizeRelY(pos, floatsPage, globalPageIdx, self.mDocEnv.textDocument.getCompatibilityMode().isBefore2013Mode());
              // java newItem.relX = relX;
              newItem.relX = relX;
              // java newItem.relY = relY;
              newItem.relY = relY;
              // java this.doPreFlowDrawing(drawing, shape, relX, relY, false, stateEnvoy);
              self.doPreFlowDrawing(&mut drawing, shape, relX, relY, false, stateEnvoy);
              // java DrawingCacheTool.addToSortedCacheItems(floatsCache, newItem);
              DrawingCacheTool::addToSortedCacheItems__klistdrawingcacheitem_klistdrawingcacheitem(floatsCache, newItem);
            }
            // java this.mMainState.snapshot.getTypoTools().recycle((TypoMemoryObj)drawing);
            self.mMainState.snapshot.getTypoTools().recycle((drawing).downcast_ref::<TypoMemoryObj>().unwrap());
          }
        }
      }
      */

    // java protected boolean needFlow(Shape shape)
    pub fn needFlow(&self, shape: &mut drawing::cn::wps::moffice::drawing::Shape::Shape) -> bool {
      if shape.getHidden() {
        return false;
      }
      if shape.getShapePos().is_none() {
        return false;
      }
      true
    }

    // java protected void changeShapePos(Shape cloneShape)
    pub fn changeShapePos(&self, cloneShape: &mut Shape) {
      let _ = cloneShape;
    }

    // java protected void doPreFlowDrawing(TypoDrawing drawing, Shape shape, int relX, int relY, boolean isEmbed, IStateEnvoy stateEnvoy)
    pub fn doPreFlowDrawing(
        &mut self,
        drawing: &mut crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing,
        shape: &mut drawing::cn::wps::moffice::drawing::Shape::Shape,
        _rel_x: i32,
        _rel_y: i32,
        is_embed: bool,
        global_page_idx: i32,
    ) {
      // java this.setDrawingShapeRect(...); textbox; setDrawingRenderRect; setDrawingLayoutRect;
      // java setDrawingShapeRect; setDrawingRenderRect; embed setDrawingLayoutRect copies render padding.
      crate::cn::wps::moffice::writer::layout::base::drawing::DrawingShapeRectTool::DrawingShapeRectTool::calc_shape_rect(drawing, shape, true, is_embed, global_page_idx, 0, 0);
      self.setDrawingRenderRect(drawing, shape);
      self.setDrawingLayoutRect(drawing);
      /* DEFERRED_NON_FUNCTION
      // java this.setDrawingShapeRect(drawing, shape, relX, relY, true, isEmbed, stateEnvoy);
      self.setDrawingShapeRect(drawing, shape, relX, relY, true, isEmbed, { let __x: &dyn IStateEnvoy = todo!("no align strategy matched"); __x });
      // java TextFrame textbox = (TextFrame)shape.getTextObj();
      let mut textbox: TextFrame = (shape.getTextObj()).downcast_ref::<TextFrame>().unwrap();
      // java if (null != textbox && textbox.hasText())
      if true && textbox.hasText() {
        // java this.flowTextBox(drawing, shape, stateEnvoy);
        self.flowTextBox(drawing, shape, stateEnvoy);
      }
      // java TypoRectCoreImpl renderRect = TypoRectCoreImpl.obtain();
      let mut renderRect: TypoRectCoreImpl = TypoRectCoreImpl::obtain();
      // java this.setDrawingRenderRect(drawing, shape, renderRect, stateEnvoy);
      self.setDrawingRenderRect(drawing, shape, { let __x: &mut ITypoRect = todo!("no align strategy matched"); __x }, { let __x: &dyn IStateEnvoy = todo!("no align strategy matched"); __x });
      // java this.setDrawingLayoutRect(drawing, renderRect);
      self.setDrawingLayoutRect(drawing, { let __x: &ITypoRect = todo!("no align strategy matched"); __x });
      // java renderRect.recycle();
      renderRect.recycle();
      */
    }

    // java private void doPreFlowEmbedDrawing(TypoDrawing drawing, Shape shape, IStateEnvoy stateEnvoy)
    pub fn doPreFlowEmbedDrawing(
        &mut self,
        drawing: &mut crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing,
        shape: &mut drawing::cn::wps::moffice::drawing::Shape::Shape,
        global_page_idx: i32,
        visual_textflow: i32,
    ) {
      // java this.doPreFlowDrawing(drawing, shape, 0, 0, true, stateEnvoy);
      self.doPreFlowDrawing(drawing, shape, 0, 0, true, global_page_idx);
      // java if (!drawing.isRotatedWithPage() && DrawingCoordTool.isVertTextflow(embedPage))
      if !drawing.isRotatedWithPage() && visual_textflow != 0 {
        crate::cn::wps::moffice::writer::layout::base::drawing::DrawingResizeTool::DrawingResizeTool::reverseRotateEmbedDrawing(drawing, visual_textflow);
      }
      /* DEFERRED_NON_FUNCTION
      // java this.doPreFlowDrawing(drawing, shape, 0, 0, true, stateEnvoy);
      self.doPreFlowDrawing(drawing, shape, 0, 0, true, stateEnvoy);
      // java if (!drawing.isRotatedWithPage())
      if !drawing.isRotatedWithPage() {
        // java TypoPage embedPage = this.getEmbedContainerPage(stateEnvoy);
        let mut embedPage: TypoPage = self.getEmbedContainerPage({ let __x: &dyn IStateEnvoy = todo!("no align strategy matched"); __x });
        // java if (DrawingCoordTool.isVertTextflow(embedPage))
        if DrawingCoordTool::isVertTextflow__typopage(embedPage) {
          // java DrawingResizeTool.reverseRotateEmbedDrawing(embedPage, drawing);
          DrawingResizeTool::reverseRotateEmbedDrawing(embedPage, drawing);
        }
      }
      */
    }

    // java protected void setDrawingShapeRect(TypoDrawing drawing, Shape shape, int relX, int relY, boolean needCalcSize, boolean isEmbed, IStateEnvoy stateEnvoy)
    pub fn setDrawingShapeRect(
        &self,
        drawing: &mut crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing,
        shape: &mut drawing::cn::wps::moffice::drawing::Shape::Shape,
        _rel_x: i32,
        _rel_y: i32,
        need_calc_size: bool,
        is_embed: bool,
        global_page_idx: i32,
    ) {
      crate::cn::wps::moffice::writer::layout::base::drawing::DrawingShapeRectTool::DrawingShapeRectTool::calc_shape_rect(drawing, shape, need_calc_size, is_embed, global_page_idx, 0, 0);
      /* DEFERRED_NON_FUNCTION
      // java IPos pos = drawing.getPos();
      let mut pos: Box<dyn IPos> = drawing.getPos();
      // java if (pos != null)
      if true {
        // java TypoRectCoreImpl alignRect = TypoRectCoreImpl.obtain();
        let mut alignRect: TypoRectCoreImpl = TypoRectCoreImpl::obtain();
        // java if (!isEmbed)
        if !isEmbed {
          // java boolean needCalcHori = 0.0F != pos.getPosHPercent();
          let mut needCalcHori: bool = 0 != pos.getPosHPercent();
          // java boolean needCalcVert = 0.0F != pos.getPosVPercent();
          let mut needCalcVert: bool = 0 != pos.getPosVPercent();
          // java this.calcAlignRect(drawing, alignRect, 0, 0, 0, relX, relY, needCalcHori, needCalcVert, stateEnvoy);
          self.calcAlignRect({ let __x: &TypoDrawing = todo!("no align strategy matched"); __x }, { let __x: &mut ITypoRect = todo!("no align strategy matched"); __x }, 0, 0, 0, relX, relY, needCalcHori, needCalcVert, &mut stateEnvoy);
        }
        // java DrawingShapeRectTool.calcDrawingShapeRect(shape, pos, drawing.isEmbed(), needCalcSize, this.mMainState.curPageIndex, this.getPageSetup(stateEnvoy), alignRect, drawing);
        DrawingShapeRectTool::calcDrawingShapeRect(shape, pos, drawing.isEmbed(), needCalcSize, self.mMainState.curPageIndex, self.getPageSetup(stateEnvoy), alignRect, drawing);
        // java alignRect.recycle();
        alignRect.recycle();
      }
      */
    }

    // java private void setDrawingRenderRect(TypoDrawing drawing, Shape shape, ITypoRect outRect, IStateEnvoy stateEnvoy)
    pub fn setDrawingRenderRect(
        &self,
        drawing: &mut crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing,
        shape: &mut drawing::cn::wps::moffice::drawing::Shape::Shape,
    ) {
      crate::cn::wps::moffice::writer::layout::base::drawing::DrawingShapeRectTool::DrawingShapeRectTool::apply_render_padding(drawing, shape);
      /* DEFERRED_NON_FUNCTION
      // java ITypoRect unionedRenderRect = TypoRectCoreImpl.obtain();
      let mut unionedRenderRect: Box<dyn ITypoRect> = TypoRectCoreImpl::obtain();
      // java DrawingShapeRectTool.calcDrawingOutRect(drawing, unionedRenderRect);
      DrawingShapeRectTool::calcDrawingOutRect(drawing, unionedRenderRect);
      // java if (shape.isGroupShape())
      if shape.isGroupShape() {
        // java this.flowChildShapes(drawing, 0, 0, 1.0F, 1.0F, 0.0F, (RectF)null, (ITypoRect)null, stateEnvoy);
        self.flowChildShapes(drawing, 0, 0, 1, 1, 0, Default::default(), Default::default(), &mut stateEnvoy);
        // java TypoRectCoreImpl tmpRect = TypoRectCoreImpl.obtain();
        let mut tmpRect: TypoRectCoreImpl = TypoRectCoreImpl::obtain();
        // java DrawingCoordTool.renderRectForPageCore(drawing, tmpRect);
        DrawingCoordTool::renderRectForPageCore(drawing, tmpRect);
        // java unionedRenderRect.union(tmpRect);
        unionedRenderRect.union(tmpRect);
        // java tmpRect.recycle();
        tmpRect.recycle();
      }
      // java outRect.set(unionedRenderRect);
      outRect.set__ityporect(unionedRenderRect);
      // java DrawingResizeTool.setDrawingRenderRect(drawing, unionedRenderRect);
      DrawingResizeTool::setDrawingRenderRect(drawing, unionedRenderRect.as_ref());
      // java unionedRenderRect.recycle();
      unionedRenderRect.recycle();
      */
    }

    // java private void setDrawingLayoutRect(TypoDrawing drawing, ITypoRect renderRect)
    pub fn setDrawingLayoutRect(&self, drawing: &mut crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing) {
      if drawing.isEmbed() {
        drawing.copyRenderPaddingToLayout();
        return;
      }
      // java else: expand the render rect by IPos wrap distance, then store layout padding.
      let mut wrap_l = 0;
      let mut wrap_t = 0;
      let mut wrap_r = 0;
      let mut wrap_b = 0;
      let mut shape_rect_h: Option<f32> = None;
      let mut has_pos = false;
      if let Some(pos_el) = drawing.getShapePosElement() {
        if let Some(pos) = pos_el.downcast_ref::<drawing::cn::wps::moffice::drawing::ShapePos::ShapePos>() {
          has_pos = true;
          let to_layout = |value: f32| {
            let scaled = value * 20.0;
            if scaled >= 0.0 { (scaled + 0.5) as i32 } else { (scaled - 0.5) as i32 }
          };
          wrap_l = to_layout(pos.getWrapDistLeft());
          wrap_t = to_layout(pos.getWrapDistTop());
          wrap_r = to_layout(pos.getWrapDistRight());
          wrap_b = to_layout(pos.getWrapDistBottom());
          if let Some(rect_el) = pos.getShapeRect() {
            if let Some(rect) = rect_el.downcast_ref::<util::cn::wps::graphics::RectF::RectF>() {
              shape_rect_h = Some(rect.bottom - rect.top);
            }
          }
        }
      }
      let mut shape_type = 0;
      if let Some(shape) = drawing.shape_arc() {
        if let Ok(guard) = shape.lock() {
          shape_type = guard.getShapeType();
        }
      }
      let (rl, rt, rr, rb) = drawing.render_padding_tuple();
      let mut pl = rl + wrap_l;
      let mut pt = rt + wrap_t;
      let mut pr = rr + wrap_r;
      let mut pb = rb + wrap_b;
      let ww8_zero = self.mDocEnv.isShapeLayoutLikeWW8
        && has_pos
        && shape_type == 20
        && shape_rect_h == Some(0.0);
      if ww8_zero {
        let top = drawing.getTop() - pt;
        let bottom = drawing.getBottom() + pb;
        let center = (top + bottom) / 2;
        pt = drawing.getTop() - center;
        pb = center - drawing.getBottom();
      } else {
        if drawing.height() + pt + pb == 0 {
          pt += 1;
          pb += 1;
        }
        if drawing.width() + pl + pr == 0 {
          pt += 1;
          pr += 1;
        }
      }
      drawing.setLayoutPadding(pl, pt, pr, pb);
    }
      /* DEFERRED_NON_FUNCTION
      // java if (drawing.isEmbed())
      if drawing.isEmbed() {
        // java TypoToolsManager typoToolsManager = this.mMainState.snapshot.getTypoTools();
        let mut typoToolsManager: TypoToolsManager = self.mMainState.snapshot.getTypoTools();
        // java TypoRectMemoryImpl renderPadding = typoToolsManager.obtainMemoryRect(drawing.getRenderPadding());
        let mut renderPadding: TypoRectMemoryImpl = typoToolsManager.obtainMemoryRect(drawing.getRenderPadding());
        // java TypoRectMemoryImpl layoutPadding = typoToolsManager.obtainMemoryRect(drawing.getLayoutPadding());
        let mut layoutPadding: TypoRectMemoryImpl = typoToolsManager.obtainMemoryRect(drawing.getLayoutPadding());
        // java layoutPadding.set(renderPadding);
        layoutPadding.set(renderPadding);
        // java renderPadding.recycle();
        renderPadding.recycle();
        // java layoutPadding.recycle();
        layoutPadding.recycle();
      }
      // java else
      else {
        // java TypoRectCoreImpl layoutRect = TypoRectCoreImpl.obtain();
        let mut layoutRect: TypoRectCoreImpl = TypoRectCoreImpl::obtain();
        // java IPos pos = drawing.getPos();
        let mut pos: Box<dyn IPos> = drawing.getPos();
        // java if (pos != null)
        if true {
          // java DrawingResizeTool.expandRect(renderRect, (IPos)drawing.getPos(), layoutRect);
          DrawingResizeTool::expandRect__ityporect_ipos_ityporect(renderRect, todo!("cast → Box<dyn IPos>"), layoutRect);
        }
        // java Shape shape = drawing.getShape();
        let mut shape: Shape = drawing.getShape();
        // java if (this.mDocEnv.isShapeLayoutLikeWW8 && shape != null && shape.getShapeType() == 20 && pos != null && 0.0F == pos.getShapeRect().height())
        if self.mDocEnv.isShapeLayoutLikeWW8 && true && shape.getShapeType() == 20 && true && 0 == pos.getShapeRect().height() {
          // java int centerY = layoutRect.centerY();
          let mut centerY: i32 = layoutRect.centerY();
          // java layoutRect.top = centerY;
          layoutRect.top = centerY;
          // java layoutRect.bottom = centerY;
          layoutRect.bottom = centerY;
        }
        // java else
        else {
          // java if (0.0F == (float)layoutRect.height())
          if 0 == ((layoutRect.height()) as f32) {
            // java --layoutRect.top;
            { let _tmp = layoutRect.top; layoutRect.top -= 1; _tmp };
            // java ++layoutRect.bottom;
            { let _tmp = layoutRect.bottom; layoutRect.bottom += 1; _tmp };
          }
          // java if (0.0F == (float)layoutRect.width())
          if 0 == ((layoutRect.width()) as f32) {
            // java --layoutRect.top;
            { let _tmp = layoutRect.top; layoutRect.top -= 1; _tmp };
            // java ++layoutRect.right;
            { let _tmp = layoutRect.right; layoutRect.right += 1; _tmp };
          }
        }
        // java DrawingResizeTool.setDrawingLayoutRect(drawing, layoutRect);
        DrawingResizeTool::setDrawingLayoutRect(drawing, { let __x: &ITypoRect = todo!("no align strategy matched"); __x });
        // java layoutRect.recycle();
        layoutRect.recycle();
      }
      */
    // java private void addDrawingToPage(TypoDrawing drawing, IStateEnvoy stateEnvoy)
    pub fn addDrawingToPage(
        &mut self,
        drawing: &mut crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing,
        parent: i32,
        doc: &dyn crate::cn::wps::moffice::writer::core::Document::Document,
    ) {
      drawing.setParent(parent);
      let before_2013 = self.before_2013_mode();
      if let Some(layers) = self.mMainState.getDrawingLayers(doc) {
        layers.before_2013 = before_2013;
        layers.appendDrawingObject(drawing.getPointer(), Some(doc));
      } else {
        panic!("guest current state in textbox!!");
      }
    }
      /* DEFERRED_NON_FUNCTION
      // java drawing.setParent(this.getFloatContainerPage(stateEnvoy).getPointer());
      drawing.setParent(self.getFloatContainerPage({ let __x: &dyn IStateEnvoy = todo!("no align strategy matched"); __x }).getPointer());
      // java Document doc = stateEnvoy.getDocument();
      let mut doc: Box<dyn Document> = stateEnvoy.getDocument();
      // java TypoDrawingLayers layers = this.mMainState.getDrawingLayers(doc);
      let mut layers: TypoDrawingLayers = self.mMainState.getDrawingLayers__document(doc);
      // java if (layers != null)
      if true {
        // java layers.appendDrawingObject(drawing.getPointer(), doc);
        layers.appendDrawingObject(drawing.getPointer(), doc);
      }
      // java else
      else {
        // java throw new RuntimeException("guest current state in textbox!!");
        panic!("{}", RuntimeException::new("guest current state in textbox!!"));
      }
      */

    // java protected boolean filterFlowDrawing(DrawingCacheItem drawingItem, IStateEnvoy stateEnvoy)
    pub fn filterFlowDrawing(&self, drawingItem: &crate::cn::wps::moffice::writer::layout::base::env::DrawingCacheItem::DrawingCacheItem, stateEnvoy: &dyn IStateEnvoy) -> bool {
      drawingItem.docType != stateEnvoy.getCurPage().getDocumentType()
        || drawingItem.cp < stateEnvoy.getRangeStart()
        || drawingItem.cp >= stateEnvoy.getRangeEnd()
    }

    // java private KList getFilterFloatCache(int row, int maxAnchorCP, IStateEnvoy stateEnvoy)
    pub fn getFilterFloatCache(&mut self, row: i32, maxAnchorCP: i32, stateEnvoy: &dyn IStateEnvoy) {
      self.mFilterFloatCache.clear();
      let pending = std::mem::take(&mut self.mMainState.floatsCache);
      let mut keep = Vec::new();
      for item in pending {
        let selected = !self.filterFlowDrawing(&item, stateEnvoy)
          && Self::should_flow_after_para_line(row, maxAnchorCP, item.cp, &self.mMainState.snapshot);
        if selected {
          self.mFilterFloatCache.push(item);
        } else {
          keep.push(item);
        }
      }
      self.mMainState.floatsCache = keep;
    }

    // java DrawingLineTool.shouldFlowDrawingAfterFlowParaLine
    fn should_flow_after_para_line(row: i32, max_anchor_cp: i32, cp: i32, snapshot: &crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) -> bool {
      if max_anchor_cp >= 0 {
        return cp < max_anchor_cp;
      }
      if row == 0 {
        return false;
      }
      Self::is_in_table_row(row, cp, snapshot)
    }

    // java TableCellRangesTool.isInTableRow(row, cp, null, true, snapshot)
    fn is_in_table_row(row: i32, cp: i32, snapshot: &crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) -> bool {
      let in_row = crate::cn::wps::moffice::writer::cache::TypoRangeBase::TypoRangeBase::contains__int_int_typosnapshot(cp, row, snapshot);
      let wrap = crate::cn::wps::moffice::util::BoolBitSet::BoolBitSet::new(0).get(snapshot.get__int(row + 10));
      if wrap {
        return in_row;
      }
      let size = snapshot.get__int(row + 12);
      let mut i = 0;
      while i < size {
        let mut p_cell = snapshot.get__int(row + 16 + i);
        i += 1;
        if p_cell == 0 {
          continue;
        }
        let flag = snapshot.get__int(p_cell + 11);
        if crate::cn::wps::moffice::util::BoolBitSet::BoolBitSet::new(6).get(flag) {
          p_cell = Self::first_vert_merge_cell(p_cell, snapshot);
        }
        if crate::cn::wps::moffice::writer::cache::TypoRangeBase::TypoRangeBase::contains__int_int_typosnapshot(cp, p_cell, snapshot) {
          return true;
        }
      }
      false
    }

    // java TypoTableCell.getFirstVertMergeCell
    fn first_vert_merge_cell(cell: i32, snapshot: &crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) -> i32 {
      let mut first = cell;
      let flag = snapshot.get__int(cell + 11);
      let vert = crate::cn::wps::moffice::util::BoolBitSet::BoolBitSet::new(6).get(flag);
      let is_first = crate::cn::wps::moffice::util::BoolBitSet::BoolBitSet::new(7).get(flag);
      if vert && !is_first {
        loop {
          let pre = Self::pre_vert_merge_cell(first, snapshot);
          if pre == 0 {
            break;
          }
          first = pre;
        }
      }
      first
    }

    // java TypoTableCell.getPreVertMergeCell
    fn pre_vert_merge_cell(cell: i32, snapshot: &crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) -> i32 {
      let flag = snapshot.get__int(cell + 11);
      let vert = crate::cn::wps::moffice::util::BoolBitSet::BoolBitSet::new(6).get(flag);
      let is_first = crate::cn::wps::moffice::util::BoolBitSet::BoolBitSet::new(7).get(flag);
      if !(vert && !is_first) {
        return 0;
      }
      let p_page = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getPage__int_typosnapshot(cell, snapshot);
      if p_page == 0 {
        return 0;
      }
      let p_lines = snapshot.get__int(p_page + 17);
      let parent = snapshot.get__int(cell + 2);
      let mut row_index = Self::list_index_of(parent, p_lines, snapshot);
      if row_index == -1 {
        return 0;
      }
      let rgdxa_left = snapshot.get__int(cell + 3);
      let rgdxa_right = snapshot.get__int(cell + 5);
      let mut ret = 0;
      let mut last_left = 0;
      let mut last_right = 0;
      while row_index > 0 {
        row_index -= 1;
        let p_row = Self::list_get(row_index, p_lines, snapshot);
        let row_type = if p_row == 0 { -1 } else { snapshot.get__int(p_row) };
        if row_type != 5 {
          break;
        }
        let size = snapshot.get__int(p_row + 12);
        let mut cell_index = 0;
        while cell_index < size {
          let temp_cell = snapshot.get__int(p_row + 16 + cell_index);
          cell_index += 1;
          let temp_flag = if temp_cell == 0 { 0 } else { snapshot.get__int(temp_cell + 11) };
          let temp_vert = crate::cn::wps::moffice::util::BoolBitSet::BoolBitSet::new(6).get(temp_flag);
          let temp_first = crate::cn::wps::moffice::util::BoolBitSet::BoolBitSet::new(7).get(temp_flag);
          if !temp_vert && !temp_first {
            if ret != 0 {
              break;
            }
          } else {
            let left = snapshot.get__int(temp_cell + 3);
            let right = snapshot.get__int(temp_cell + 5);
            let left_diff = (left - rgdxa_left).abs();
            let right_diff = (right - rgdxa_right).abs();
            if left_diff < 6 && right_diff < 6 {
              if ret == 0 || (left_diff < (last_left - rgdxa_left).abs() && right_diff <= (last_right - rgdxa_right).abs()) {
                ret = temp_cell;
                last_left = left;
                last_right = right;
              }
            } else if ret != 0 {
              break;
            }
          }
        }
        if ret != 0 {
          break;
        }
      }
      ret
    }

    fn list_index_of(item: i32, p_list: i32, snapshot: &crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) -> i32 {
      let size = if p_list == 0 { 0 } else { snapshot.get__int(p_list + 2) };
      let mut i = 0;
      while i < size {
        if snapshot.get__int(p_list + 3 + i) == item {
          return i;
        }
        i += 1;
      }
      -1
    }

    fn list_get(index: i32, p_list: i32, snapshot: &crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) -> i32 {
      let size = if p_list == 0 { 0 } else { snapshot.get__int(p_list + 2) };
      if index >= 0 && index < size { snapshot.get__int(p_list + 3 + index) } else { 0 }
    }
      /* DEFERRED_NON_FUNCTION
      // java this.mFilterFloatCache.clear();
      self.mFilterFloatCache.clear();
      // java KList<DrawingCacheItem> floatsCache = this.mMainState.floatsCache;
      let mut floatsCache: KList = self.mMainState.floatsCache;
      // java KList.Iterator<DrawingCacheItem> it = floatsCache.iterator();
      let mut it: Vec<DrawingCacheItem> = floatsCache.iterator();
      // java DrawingCacheItem item = null;
      let mut item: DrawingCacheItem = Default::default();
      // java while (null != (item = (DrawingCacheItem)it.next()))
      while { item = (it.next()).downcast_ref::<DrawingCacheItem>().unwrap(); item }.is_some() {
        // java if (!this.filterFlowDrawing(item, stateEnvoy) && DrawingLineTool.shouldFlowDrawingAfterFlowParaLine(row, maxAnchorCP, item.cp, this.mMainState.snapshot))
        if !self.filterFlowDrawing(&item, stateEnvoy) && DrawingLineTool::shouldFlowDrawingAfterFlowParaLine(row, maxAnchorCP, item.cp, self.mMainState.snapshot) {
          // java floatsCache.remove(item);
          floatsCache.remove(item);
          // java this.mFilterFloatCache.add(item);
          self.mFilterFloatCache.add__t(item);
        }
      }
      // java return this.mFilterFloatCache;
      return self.mFilterFloatCache;
      */

    // java protected int doPostFlowDrawings(int row, int maxAnchorCP, boolean needAddToPage, boolean needFindAnchorLines, IStateEnvoy stateEnvoy)
    pub fn doPostFlowDrawings(&mut self, row: i32, maxAnchorCP: i32, needAddToPage: bool, needFindAnchorLines: bool, stateEnvoy: &mut dyn IStateEnvoy) -> i32 {
      self.getFilterFloatCache(row, maxAnchorCP, stateEnvoy);
      let floats_page = self.getFloatContainerPage(stateEnvoy);
      let cur_page = stateEnvoy.getCurPage();
      let para_line = if needFindAnchorLines {
        crate::cn::wps::moffice::writer::layout::base::drawing::DrawingLineTool::DrawingLineTool::findAnchorLines(
          floats_page.getPointer(),
          cur_page.getPointer(),
          &self.mMainState.snapshot,
          &mut self.mFilterFloatCache,
          row,
          maxAnchorCP,
        )
      } else {
        0
      };
      let mut max_drawing_cp = i32::MIN;
      let items = std::mem::take(&mut self.mFilterFloatCache);
      for mut item in items {
        let p_drawing = item.drawing;
        let item_cp = item.cp;
        let anchor_line = item.anchor;
        let rel_x = item.relX;
        let rel_y = item.relY;
        let doc_type = item.docType;
        item.drawing = 0;
        self.mDocEnv.pools.recycleDrawingCacheItem(item);
        let mut drawing = self.mMainState.snapshot.getTypoTools().obtainTypoDrawing__int(p_drawing);
        if needAddToPage {
          let parent = self.getFloatContainerPage(stateEnvoy).getPointer();
          drawing.setParent(parent);
          let before_2013 = self.before_2013_mode();
          let doc_ptr = self.mDocEnv.textDocument_mut().and_then(|text| text.getSubdocument(doc_type)).map(|doc| doc as *const dyn crate::cn::wps::moffice::writer::core::Document::Document);
          if let Some(doc_ptr) = doc_ptr {
            let doc = unsafe { &*doc_ptr };
            if let Some(layers) = self.mMainState.getDrawingLayers(doc) {
              layers.before_2013 = before_2013;
              layers.appendDrawingObject(drawing.getPointer(), Some(doc));
            } else {
              panic!("guest current state in textbox!!");
            }
          }
        }
        let mut shadow = TypoDrawing::default();
        shadow.pointer = drawing.getPointer();
        shadow.width = drawing.width();
        shadow.height = drawing.height();
        shadow.left = drawing.getLeft();
        shadow.top = drawing.getTop();
        shadow.wrap = drawing.isWrap();
        shadow.table = crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::isTable__int_typosnapshot(drawing.getPointer(), &self.mMainState.snapshot);
        shadow.layout_padding = drawing.getLayoutPadding();
        shadow.cp = crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::getCP__int_typosnapshot(drawing.getPointer(), &self.mMainState.snapshot);
        if let Some(shape) = drawing.shape_arc() {
          if let Ok(guard) = shape.lock() {
            shadow.has_shape = true;
            shadow.shape_type = guard.getShapeType();
          }
        }
        if let Some(pos) = drawing.getShapePosElement() {
          if let Some(shape_pos) = pos.downcast_ref::<drawing::cn::wps::moffice::drawing::ShapePos::ShapePos>() {
            shadow.pos = Some(IPos {
              alignH: shape_pos.getAlignmentH(),
              alignV: shape_pos.getAlignmentV(),
              posHPercent: shape_pos.getPosHPercent(),
              posVPercent: shape_pos.getPosVPercent(),
            });
          }
        }
        self.adjustFloatRect(&mut shadow, anchor_line, para_line, rel_x, rel_y, stateEnvoy);
        drawing.offsetTo(shadow.left, shadow.top);
        if let Some(shape) = drawing.shape_arc() {
          if let Ok(guard) = shape.lock() {
            let shape_id = guard.getShapeID();
            if let Some(container) = guard.getDrawingContainer() {
              if let Some(dc) = container.downcast_ref::<crate::cn::wps::moffice::writer::drawing::DrawingContainer::DrawingContainer>() {
                let _ = (dc, shape_id);
              }
            }
          }
        }
        max_drawing_cp = max_drawing_cp.max(item_cp);
        self.mMainState.snapshot.getTypoTools().recycle_drawing(&mut drawing);
      }
      max_drawing_cp
      /* DEFERRED_NON_FUNCTION
      // java Pools pools = this.mDocEnv.pools;
      let mut pools: Pools = self.mDocEnv.pools;
      // java KList<DrawingCacheItem> floatsCache = this.getFilterFloatCache(row, maxAnchorCP, stateEnvoy);
      let mut floatsCache: KList = { let __x: KList = todo!("no align strategy matched"); __x };
      // java KList.Iterator<DrawingCacheItem> it = floatsCache.iterator();
      let mut it: Vec<DrawingCacheItem> = floatsCache.iterator();
      // java int maxDrawingCP = Integer.MIN_VALUE;
      let mut maxDrawingCP: i32 = i32::MIN;
      // java int paraLine = 0;
      let mut paraLine: i32 = 0;
      // java DrawingCacheItem item = null;
      let mut item: DrawingCacheItem = Default::default();
      // java TypoDrawing drawing;
      let mut drawing: TypoDrawing = TypoDrawing::default();
      let mut curPage: TypoPage = stateEnvoy.getCurPage();
      // java for(TypoPage curPage = stateEnvoy.getCurPage(); null != (item = (DrawingCacheItem)it.next()) this.mMainState.snapshot.getTypoTools().recycle((TypoMemoryObj)drawing))
      while { item = (it.next()).downcast_ref::<DrawingCacheItem>().unwrap(); item }.is_some() {
        // java int pDrawing = item.drawing;
        let mut pDrawing: i32 = item.drawing;
        // java int itemCP = item.cp;
        let mut itemCP: i32 = item.cp;
        // java if (needFindAnchorLines && 0 == paraLine)
        if needFindAnchorLines && 0 == paraLine {
          // java paraLine = DrawingLineTool.findAnchorLines(this.getFloatContainerPage(stateEnvoy), stateEnvoy.getCurPage(), this.mMainState.snapshot, floatsCache, row, maxAnchorCP);
          paraLine = DrawingLineTool::findAnchorLines(self.getFloatContainerPage({ let __x: &dyn IStateEnvoy = todo!("no align strategy matched"); __x }), stateEnvoy.getCurPage(), self.mMainState.snapshot, floatsCache, row, maxAnchorCP);
        }
        // java int anchorLine = item.anchor;
        let mut anchorLine: i32 = item.anchor;
        // java int relX = item.relX;
        let mut relX: i32 = item.relX;
        // java int relY = item.relY;
        let mut relY: i32 = item.relY;
        // java item.drawing = 0;
        item.drawing = 0;
        // java floatsCache.remove(item);
        floatsCache.remove(item);
        // java pools.recycleDrawingCacheItem(item);
        pools.recycleDrawingCacheItem(item);
        // java drawing = this.mMainState.snapshot.getTypoTools().obtainTypoDrawing(pDrawing);
        drawing = self.mMainState.snapshot.getTypoTools().obtainTypoDrawing(pDrawing);
        // java if (needAddToPage)
        if needAddToPage {
          // java this.addDrawingToPage(drawing, stateEnvoy);
          self.addDrawingToPage(&mut drawing, stateEnvoy);
        }
        // java this.adjustFloatRect(drawing, anchorLine, paraLine, relX, relY, stateEnvoy);
        self.adjustFloatRect(&mut drawing, anchorLine, paraLine, relX, relY, stateEnvoy);
        // java maxDrawingCP = Math.max(maxDrawingCP, itemCP);
        maxDrawingCP = (maxDrawingCP).max(itemCP);
        // java Shape shape = drawing.getShape();
        let mut shape: Shape = drawing.getShape();
        // java if (shape != null)
        if true {
          // java DrawingContainer drawingContainer = (DrawingContainer)shape.getDrawingContainer();
          let mut drawingContainer: DrawingContainer = (shape.getDrawingContainer()).downcast_ref::<DrawingContainer>().unwrap();
          // java IDrawingCache cache = drawingContainer.getDrawingCache();
          let mut cache: Box<dyn IDrawingCache> = drawingContainer.getDrawingCache();
          // java if (cache != null)
          if true {
            // java cache.addToCache(shape.getShapeID());
            cache.addToCache(shape.getShapeID());
          }
        }
      }
      // java it.recycle();
      it.recycle();
      // java return maxDrawingCP;
      return maxDrawingCP;
      */
    }

    // java public boolean flowBeforeCP(int maxAnchorCP, IStateEnvoy stateEnvoy)
    pub fn flowBeforeCP(&mut self, maxAnchorCP: i32, stateEnvoy: &mut dyn IStateEnvoy) -> bool {
      self.doFlow(0, maxAnchorCP, stateEnvoy)
    }

    // java public boolean flowForTableRow(int row, IStateEnvoy stateEnvoy)
    pub fn flowForTableRow(&mut self, row: i32, stateEnvoy: &mut dyn IStateEnvoy) -> bool {
      self.doFlow(row, i32::MIN, stateEnvoy)
    }

    // java private boolean doFlow(int row, int maxAnchorCP, IStateEnvoy stateEnvoy)
    pub fn doFlow(&mut self, row: i32, maxAnchorCP: i32, stateEnvoy: &mut dyn IStateEnvoy) -> bool {
      if self.mMainState.floatsCache.is_empty() {
        return false;
      }
      self.doPostFlowDrawings(row, maxAnchorCP, true, true, stateEnvoy);
      !stateEnvoy.getAffectRect().isEmpty()
    }

    // java protected PageSetup getPageSetup(IStateEnvoy stateEnvoy)
    pub fn getPageSetup(&self, stateEnvoy: &dyn IStateEnvoy) -> Option<crate::cn::wps::moffice::writer::data::value::PageSetup::PageSetup> {
      let _ = stateEnvoy;
      let read = |sec: &util::moffice::property::PropertySet::PropertySet| {
        sec.getValue(676).and_then(|value| {
          value.as_ref().as_any().downcast_ref::<crate::cn::wps::moffice::writer::data::value::PageSetup::PageSetup>().copied()
        })
      };
      if let Some(sec) = self.mMainState.secPropSet.as_ref() {
        if let Some(setup) = read(sec) {
          return Some(setup);
        }
      }
      read(&crate::cn::wps::moffice::writer::data::DefaultSectionProperties::DefaultSectionProperties::getProperties())
    }

}
