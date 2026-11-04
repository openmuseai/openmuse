// Java: cn.wps.moffice.writer.io.reader.dmlReader.LineImporter
use std::{any::Any, cell::RefCell, ops::{Deref, DerefMut}, rc::Rc, sync::{Arc, Mutex}};
use drawing::cn::wps::moffice::drawing::graphics::SolidFill::SolidFill;
use drawing::cn::wps::moffice::drawing::line::{Arrow::Arrow, LineProperty::LineProperty};
use io_wps_io::cn::wps::io::dom::io::WMap::WMap;
use io_wps_io::cn::wps::io::xwpf::{IImporterBase::IImporterBase, SaxAttributes::SaxAttributes};
use util::base::io::{MetricUtility::MetricUtility, TypeConversion::TypeConversion};
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::Xml07Context;
use super::{
    DmlImporterBase::DmlImporterBase,
    GradFillImporter::GradFillImporter,
    PatternFillImporter::PatternFillImporter,
    SolidFillImporter::SolidFillImporter,
    context::{DmlShapeCtxt::DmlShapeCtxt, DmlSubDocCtxt::DmlSubDocCtxt, ThemeContext::ThemeContext},
    helper::LineImporterHelper::LineImporterHelper,
};

fn main_string(attrs: &dyn SaxAttributes, name: &str) -> Option<String> {
    if attrs.getLength() == 0 { None } else { attrs.getValue_by_uri(WMap::MAIN_NAMESPACING(), name) }
}

// Java: public class LineImporter extends DmlImporterBase
pub struct LineImporter<ShapeImporter, DmlCommonImporter: IImporterBase> {
    pub super_: DmlImporterBase<ShapeImporter, DmlCommonImporter>,
    // Java: private LineProperty mTarget;
    mTarget: Option<Arc<Mutex<LineProperty>>>,
    // Java: public SolidFillImporter mSolidFillImporter;
    pub mSolidFillImporter: SolidFillImporter<ShapeImporter, DmlCommonImporter>,
    // Java: public GradFillImporter mGradFillImporter;
    pub mGradFillImporter: GradFillImporter<ShapeImporter, DmlCommonImporter>,
    // Java: public PatternFillImporter mPatternFillImporter;
    pub mPatternFillImporter: PatternFillImporter<ShapeImporter, DmlCommonImporter>,
    // Java: private boolean mIsText;
    mIsText: bool,
}

impl<S, D: IImporterBase> Deref for LineImporter<S, D> {
    type Target = DmlImporterBase<S, D>;
    fn deref(&self) -> &Self::Target { &self.super_ }
}
impl<S, D: IImporterBase> DerefMut for LineImporter<S, D> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.super_ }
}

impl<S, D: IImporterBase> LineImporter<S, D> {
    // Java: LineImporter(..., boolean isText)
    pub fn new(subDocCtxt: Rc<RefCell<DmlSubDocCtxt<S>>>, ctxt: Rc<RefCell<DmlShapeCtxt>>,
        themeCtxt: Rc<RefCell<ThemeContext>>, xml07Context: Option<Xml07Context>,
        dmlImporter: Rc<RefCell<D>>, isText: bool) -> Self {
        Self {
            mSolidFillImporter: SolidFillImporter::new(subDocCtxt.clone(), ctxt.clone(),
                themeCtxt.clone(), xml07Context.clone(), dmlImporter.clone(), isText),
            mGradFillImporter: GradFillImporter::new(subDocCtxt.clone(), ctxt.clone(),
                themeCtxt.clone(), xml07Context.clone(), dmlImporter.clone(), isText),
            mPatternFillImporter: PatternFillImporter::new(subDocCtxt.clone(), ctxt.clone(),
                themeCtxt.clone(), xml07Context.clone(), dmlImporter.clone()),
            super_: DmlImporterBase::new(subDocCtxt, ctxt, themeCtxt, xml07Context, dmlImporter),
            mTarget: None,
            mIsText: isText,
        }
    }

    // Java: public void excute()
    pub fn excute(&mut self) {
        self.initTarget();
        let (dashing, join, shape_type, start, end, width, cap, style, line_on) = {
            let ctxt = self.mCtxt.borrow();
            (ctxt.mDashing, ctxt.mJoinStyle, ctxt.mShapeType,
                ctxt.mStartArrow.clone(), ctxt.mEndArrow.clone(), ctxt.mLineWidth, ctxt.mCap,
                ctxt.mLineStyle, ctxt.mLineFillOn)
        };
        let mut line = self.mTarget.as_ref().expect("mTarget").lock().expect("line");
        if let Some(dashing) = dashing { line.setDashing(dashing); }
        if let Some(join) = join { line.setJoinStyle(join); }
        // Java: shape type 43 swaps start and end arrows.
        if shape_type == Some(43) {
            if let Some(start) = start { line.setEndArrow_element(Some(Arc::new(start) as Arc<dyn Any + Send + Sync>)); }
            if let Some(end) = end { line.setStartArrow_element(Some(Arc::new(end) as Arc<dyn Any + Send + Sync>)); }
        } else {
            if let Some(start) = start { line.setStartArrow_element(Some(Arc::new(start) as Arc<dyn Any + Send + Sync>)); }
            if let Some(end) = end { line.setEndArrow_element(Some(Arc::new(end) as Arc<dyn Any + Send + Sync>)); }
        }
        if let Some(width) = width { line.setWidth(width); }
        if let Some(cap) = cap { line.setEndCapStyle(cap); }
        if let Some(style) = style { line.setLineStyle(style); }
        if !line_on { line.setfLine(false); }
    }

    // Java: public LineProperty result()
    pub fn result(&self) -> Option<Arc<Mutex<LineProperty>>> { self.mTarget.clone() }

    // Java: protected void initTarget()
    pub fn initTarget(&mut self) {
        if self.mTarget.is_none() {
            self.mTarget = Some(Arc::new(Mutex::new(LineProperty::new())));
        }
    }

    // Java: public void clearContext()
    pub fn clearContext(&mut self) {
        self.mTarget = None;
        let mut ctxt = self.mCtxt.borrow_mut();
        ctxt.mStartArrow = None;
        ctxt.mEndArrow = None;
        ctxt.mDashing = None;
        ctxt.mJoinStyle = None;
        ctxt.mLineWidth = None;
        ctxt.mCap = None;
        ctxt.mLineStyle = None;
    }

    // Java: public void importSolidFill()
    pub fn importSolidFill(&mut self) {
        self.initTarget();
        self.mSolidFillImporter.excute();
        let fill = self.mSolidFillImporter.result();
        self.mSolidFillImporter.clearContext();
        let fill = fill.expect("solid");
        let (color, transparency) = {
            let fill = fill.borrow();
            (fill.base.getColor(), fill.base.getTransparency())
        };
        let stored = SolidFill { base: fill.borrow().base.clone() };
        let mut line = self.mTarget.as_ref().expect("mTarget").lock().expect("line");
        line.setColor(color);
        line.setTransparency(transparency);
        line.setType(0);
        line.setFill(Some(Arc::new(stored) as Arc<dyn Any + Send + Sync>));
        drop(line);
        self.mCtxt.borrow_mut().mSolidFill = None;
    }

    // Java: public void importGradFill()
    pub fn importGradFill(&mut self) {
        self.initTarget();
        self.mGradFillImporter.excute();
        let fill = self.mGradFillImporter.result();
        self.mGradFillImporter.clearContext();
        self.mTarget.as_ref().expect("mTarget").lock().expect("line")
            .setFill(fill.map(|fill| fill as Arc<dyn Any + Send + Sync>));
    }

    // Java: public void importPatternFill()
    pub fn importPatternFill(&mut self) {
        self.initTarget();
        self.mPatternFillImporter.excute();
        let fill = self.mPatternFillImporter.result();
        let (color, back) = fill.as_ref().map(|fill| {
            let fill = fill.lock().expect("pattern");
            (fill.getColorObj(), fill.getBackColorObj())
        }).unwrap_or((None, None));
        {
            let mut line = self.mTarget.as_ref().expect("mTarget").lock().expect("line");
            if let Some(color) = color { line.setColor(color); }
            if let Some(back) = back { line.setBackColor(back); }
            line.setType(1);
            line.setFill(fill.map(|fill| fill as Arc<dyn Any + Send + Sync>));
        }
        self.mPatternFillImporter.clearContext();
    }

    // Java: public void setDashing(Attributes attrs)
    pub fn setDashing(&mut self, attrs: &dyn SaxAttributes) {
        if let Some(dashing) = main_string(attrs, "val") {
            self.mCtxt.borrow_mut().mDashing = Some(LineImporterHelper::toLineDashing(&dashing));
        }
    }

    // Java: public void setJoinStyle(int elementId)
    pub fn setJoinStyle(&mut self, elementId: i32) {
        let style = match elementId {
            93630586 => Some(0),
            103906565 => Some(1),
            108704142 => Some(2),
            _ => None,
        };
        if let Some(style) = style {
            self.mCtxt.borrow_mut().mJoinStyle = Some(style);
        }
    }

    // Java: public void setTailEnd(Attributes attrs)
    pub fn setTailEnd(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mEndArrow = Some(Self::arrow_from(attrs));
    }

    // Java: public void setHeadEnd(Attributes attrs)
    pub fn setHeadEnd(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mStartArrow = Some(Self::arrow_from(attrs));
    }

    // Java: public void setLn(Attributes attrs)
    pub fn setLn(&mut self, attrs: &dyn SaxAttributes) {
        let width = main_string(attrs, "w");
        let cmpd = main_string(attrs, "cmpd");
        let cap = main_string(attrs, "cap");
        let mut ctxt = self.mCtxt.borrow_mut();
        if let Some(width) = width {
            if let Some(width) = TypeConversion::string2Integer(&width) {
                ctxt.mLineWidth = Some(MetricUtility::EMU2Points(width as f32));
            }
        }
        if let Some(cmpd) = cmpd {
            ctxt.mLineStyle = Some(LineImporterHelper::string2LineStyle(&cmpd));
        }
        if let Some(cap) = cap {
            ctxt.mCap = Some(LineImporterHelper::string2LineEndCapStyle(&cap));
        }
    }

    // Java: public void setNoFill()
    pub fn setNoFill(&mut self) {
        self.mCtxt.borrow_mut().mLineFillOn = false;
    }

    fn arrow_from(attrs: &dyn SaxAttributes) -> Arrow {
        let mut arrow = Arrow::new();
        if let Some(type_name) = main_string(attrs, "type") {
            arrow.setHead(LineImporterHelper::string2ArrowStyle(&type_name));
        }
        if let Some(width) = main_string(attrs, "w") {
            arrow.setWidth(LineImporterHelper::string2ArrowWidth(&width));
        }
        if let Some(len) = main_string(attrs, "len") {
            arrow.setLength(LineImporterHelper::string2ArrowLength(&len));
        }
        arrow
    }
}
