#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unreachable_code, unused_unsafe)]

// Java: import android.graphics.RectF
use stubs::android::graphics::RectF::RectF;
// Java: import cn.wps.base.assertion.KSAssert
use super::kmo_show_shape_hosts::KSAssert;
// Java: import cn.wps.drawing.app.DgStore
use crate::cn::wps::drawing::app::DgStoreData::DgStoreData;
// Java: import cn.wps.drawing.app.KmoProtocol
use super::kmo_show_shape_hosts::KmoProtocol;
// Java: import cn.wps.drawing.app.chart.KmoOleChart
use super::kmo_show_shape_hosts::KmoOleChart;
// Java: import cn.wps.drawing.app.format.color.DgColor
use super::kmo_show_shape_hosts::DgColor;
// Java: import cn.wps.drawing.app.format.color.DgColorFormat
use super::kmo_show_shape_hosts::DgColorFormat;
// Java: import cn.wps.drawing.app.format.fill.DgFillFormat
use super::kmo_show_shape_hosts::DgFillFormat;
// Java: import cn.wps.drawing.app.format.outline.DgOutlineFormat
use super::kmo_show_shape_hosts::DgOutlineFormat;
// Java: import cn.wps.drawing.app.graphic.GraphicObject
// rust: ★W6c-10 —— 真身接口（`drawing/app/graphic/GraphicObject.rs`，Java `GraphicObject.java` 9 行）。
//   原先从 `kmo_show_shape_hosts` 取单元空壳（`r#type()` 恒 0，无 `copy()`/`query` 落点）⇒ 形态 F。
//   owner: show.app.shape.KmoShowShape    wave: M-PPT-KMOSLIDE-W6c-10
use crate::cn::wps::drawing::app::graphic::GraphicObject::GraphicObject;
// Java: import cn.wps.drawing.app.shape.DgConnector
use super::kmo_show_shape_hosts::DgConnector;
// Java: import cn.wps.drawing.app.shape.DgGroup
use crate::cn::wps::drawing::app::shape::DgGroup::DgGroup;
// Java: import cn.wps.drawing.app.shape.DgPicture
use super::kmo_show_shape_hosts::DgPicture;
// Java: import cn.wps.drawing.app.shape.DgShape
use super::kmo_show_shape_hosts::DgShape;
// Java: import cn.wps.drawing.app.shape.DgShapeStyle
use super::kmo_show_shape_hosts::DgShapeStyle;
// Java: import cn.wps.drawing.app.table.KmoTable
use super::kmo_show_shape_hosts::KmoTable;
// Java: import cn.wps.drawing.app.util.UnitsConverter
use super::kmo_show_shape_hosts::UnitsConverter;
// Java: import cn.wps.drawing.data.basic.Transform2D
use super::kmo_show_shape_hosts::Transform2D;
// Java: import cn.wps.drawing.data.blip.Blip
use super::kmo_show_shape_hosts::Blip;
// Java: import cn.wps.drawing.data.color.Color
use super::kmo_show_shape_hosts::Color;
// Java: import cn.wps.drawing.data.fill.Fill
use super::kmo_show_shape_hosts::Fill;
// Java: import cn.wps.drawing.data.geometry.Geometry2D
use super::kmo_show_shape_hosts::Geometry2D;
// Java: import cn.wps.drawing.data.geometry.PresetGeometry
use super::kmo_show_shape_hosts::PresetGeometry;
// Java: import cn.wps.drawing.data.hyperlink.Hyperlink
use super::kmo_show_shape_hosts::Hyperlink;
// Java: import cn.wps.drawing.data.media.Media
use super::kmo_show_shape_hosts::Media;
// Java: import cn.wps.drawing.data.outline.Outline
use super::kmo_show_shape_hosts::Outline;
// Java: import cn.wps.drawing.data.shape.Shape
use super::kmo_show_shape_hosts::Shape;
// Java: import cn.wps.drawing.data.shape.ShapeExt
use super::kmo_show_shape_hosts::ShapeExt;
// Java: import cn.wps.drawing.data.shape.ShapePr
use super::kmo_show_shape_hosts::ShapePr;
// Java: import cn.wps.drawing.data.style.ShapeStyle
use super::kmo_show_shape_hosts::ShapeStyle;
// Java: import cn.wps.graphics.PictureInfo
use super::kmo_show_shape_hosts::PictureInfo;
// Java: import cn.wps.graphics.PictureParser
use super::kmo_show_shape_hosts::PictureParser;
// Java: import cn.wps.kfc.api.list.IIntList
use super::kmo_show_shape_hosts::IIntList;
// Java: import cn.wps.kfc.impl.arrayList.WIntArrayList
use super::kmo_show_shape_hosts::WIntArrayList;
// Java: import cn.wps.kfc.memor4j.util.KmoUndoableList
use super::kmo_show_shape_hosts::KmoUndoableList;
// Java: import cn.wps.kso.kproto.KoBin
use super::kmo_show_shape_hosts::KoBin;
// Java: import cn.wps.kso.kproto.util.KoIntList
use super::kmo_show_shape_hosts::KoIntList;
// Java: import cn.wps.memor4j.KmoAtom
use kfc::cn::wps::memor4j::KmoAtom::KmoAtom;
// Java: import cn.wps.moffice.drawing.graphics.GraphicsPath
use super::kmo_show_shape_hosts::GraphicsPath;
// Java: import cn.wps.moffice.drawing.media.IMediaLib
use super::kmo_show_shape_hosts::IMediaLib;
// Java: import cn.wps.moffice.kfs.File
use super::kmo_show_shape_hosts::File;
// Java: import cn.wps.moffice.util.FileDataStorage
use super::kmo_show_shape_hosts::FileDataStorage;
// Java: import cn.wps.show.app.ISlideCache
use super::kmo_show_shape_hosts::ISlideCache;
// Java: import cn.wps.show.app.KmoPresentation
use super::kmo_show_shape_hosts::KmoPresentation;
// Java: import cn.wps.show.app.KsoWMInfo
// rust: ★③-5 去假面（形态 F）—— `kmo_show_shape_hosts::KsoWMInfo` 是**单元空壳**
//   （无 `create`/`get`，`shapeTagMap()` 因此无法实现）。真身在 `show::app::KsoWMInfo`
//   （`create(CustDataLst)` / `get(&str)`），Java 同包同名。
//   owner: show.app.shape.KmoShowShape    wave: M-PPT-SHAPE-DISPATCH
use crate::cn::wps::show::app::KsoWMInfo::KsoWMInfo;
// Java: import cn.wps.show.app.Ink.KmoInkLib
use super::kmo_show_shape_hosts::KmoInkLib;
// Java: import cn.wps.show.app.chart.KmoChartOperator
use super::kmo_show_shape_hosts::KmoChartOperator;
// Java: import cn.wps.show.app.format.KmoFont
use super::kmo_show_shape_hosts::KmoFont;
// Java: import cn.wps.show.app.gradient.FillBean
use super::kmo_show_shape_hosts::FillBean;
// Java: import cn.wps.show.app.media.KmoMediaData
use super::kmo_show_shape_hosts::KmoMediaData;
// Java: import cn.wps.show.app.media.KmoMediaLib
use super::kmo_show_shape_hosts::KmoMediaLib;
// Java: import cn.wps.show.app.media.KmoMediaUtil
use super::kmo_show_shape_hosts::KmoMediaUtil;
// Java: import cn.wps.show.app.model3d.KmoModel3D
use super::kmo_show_shape_hosts::KmoModel3D;
// Java: import cn.wps.show.app.ole.KmoOleManager
use super::kmo_show_shape_hosts::KmoOleManager;
// Java: import cn.wps.show.app.ole.KmoOleObject
use super::kmo_show_shape_hosts::KmoOleObject;
// Java: import cn.wps.show.app.selection.KmoSelection
use super::kmo_show_shape_hosts::KmoSelection;
// Java: import cn.wps.show.app.selection.KmoTextRange
use super::kmo_show_shape_hosts::KmoTextRange;
// Java: import cn.wps.show.app.slide.ISlide
use crate::cn::wps::show::app::slide::ISlide::ISlide;
// Java: import cn.wps.show.app.slide.KmoFilm
use super::kmo_show_shape_hosts::KmoFilm;
// Java: import cn.wps.show.app.table.KmoTableOperator
use super::kmo_show_shape_hosts::KmoTableOperator;
// Java: import cn.wps.show.app.text.KmoCellTextOperator
use super::kmo_show_shape_hosts::KmoCellTextOperator;
// Java: import cn.wps.show.app.text.KmoContext
use super::kmo_show_shape_hosts::KmoContext;
// Java: import cn.wps.show.app.text.KmoTextOperator
use super::kmo_show_shape_hosts::KmoTextOperator;
// Java: import cn.wps.show.app.theme.KmoTheme
use super::kmo_show_shape_hosts::KmoTheme;
// Java: import cn.wps.show.app.util.ColorInterpretCtx
use super::kmo_show_shape_hosts::ColorInterpretCtx;
// Java: import cn.wps.show.app.util.ColorInterpreter
use super::kmo_show_shape_hosts::ColorInterpreter;
// Java: import cn.wps.show.app.util.KmoShapeUtil
use super::kmo_show_shape_hosts::KmoShapeUtil;
// Java: import cn.wps.show.app.util.ModularGroupUtil
use super::kmo_show_shape_hosts::ModularGroupUtil;
// Java: import cn.wps.show.data.custdatalst.CustDataLst
// rust: ★③-5 去假面（形态 F）—— `kmo_show_shape_hosts::CustDataLst` 是**单元空壳**
//   （`:200 pub struct CustDataLst;`，无 `tags`/`save`）。真身是 SP1.0 交付的 728 行
//   `show::data::custdatalst::CustDataLst`（Java 同包同名）。
//   owner: show.app.shape.KmoShowShape    wave: M-PPT-SHAPE-DISPATCH
use crate::cn::wps::show::data::custdatalst::CustDataLst::CustDataLst;
// Java: import cn.wps.show.data.custdatalst.Tag
use super::kmo_show_shape_hosts::Tag;
// Java: import cn.wps.show.data.ole.OleObject
use super::kmo_show_shape_hosts::OleObject;
// Java: import cn.wps.show.data.shape.ShowShape
use super::kmo_show_shape_hosts::ShowShape;
// Java: import java.util.ArrayList
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import java.util.Map
// Java: import org.apache.poi.hwpf.ole.stream.PackageStreamParser
use super::kmo_show_shape_hosts::PackageStreamParser;
// Java: import cn.wps.show.app.shape.ShowShapeAtomData
use super::ShowShapeAtomData::ShowShapeAtomData;
// Java: import java.lang.ThreadLocal
use stubs::java::lang::ThreadLocal::ThreadLocal;
// Java: import java.lang.Object
use stubs::java::lang::Object::Object;
// Java: import org.apache.poi.poifs.filesystem.Entry
use super::kmo_show_shape_hosts::Entry;
// Java: import android.graphics
// rust: no-stmt
use std::collections::{HashMap};
use super::kmo_show_shape_hosts::{Placeholder, AdjustValueList, cn_wps_graphics_RectF};

// java public static final int INVILAD_EXOBJID = -1;
pub(crate) const INVILAD_EXOBJID: i32 = -1;
// java public static final String MODULAR_GROUP_EXT_URI = "{D589DF8D-B311-4D76-A726-2CF4D000D303}";
pub(crate) const MODULAR_GROUP_EXT_URI: &str = "{D589DF8D-B311-4D76-A726-2CF4D000D303}";
// java public static final int NO_FILL_DEFAULT_COLOR = 0;
pub(crate) const NO_FILL_DEFAULT_COLOR: i32 = 0;

// java public class KmoShowShape extends KmoAtom<ShowShapeAtomData> implements KmoProtocol
pub struct KmoShowShape {
    // java KmoAtom<ShowShapeAtomData> super_
    pub(crate) super_: KmoAtom<ShowShapeAtomData>,
    // java private ISlide parent;
    parent: Option<*const dyn ISlide>,
    // java private int dgShapeId;
    dgShapeId: i32,
    // java private KmoUndoableList<KmoShowShape> children;
    children: Option<Vec<KmoShowShape>>,
    // java private ThreadLocal<DgShape> dgShapeThreadLocal = new ThreadLocal();
    dgShapeThreadLocal: Option<DgShape>,
    // java private ThreadLocal<DgGroup> dgGroupThreadLocal = new ThreadLocal();
    dgGroupThreadLocal: Option<DgGroup>,
    // java private ThreadLocal<DgPicture> dgPicThreadLocal = new ThreadLocal();
    dgPicThreadLocal: Option<DgPicture>,
    // java private ThreadLocal<DgConnector> dgConnThreadLocal = new ThreadLocal();
    dgConnThreadLocal: Option<DgConnector>,
    // java private RectF textRect;
    textRect: Option<RectF>,
    // java private int exObjId = -1;
    exObjId: i32,
    // java private int beautifyOrder = 0;
    beautifyOrder: i32,
    // java private boolean isLocked = false;
    isLocked: bool,
    // java private boolean isEditState = false;
    isEditState: bool,
    // java private int originalAutoFit = -1;
    originalAutoFit: i32,
    // java private int originalAnchor = -1;
    originalAnchor: i32,
    // 导入器写入的正文。`getTextContent` 在 dg 文本仓接通前先读这里。
    plain_text: String,
    // 导入器见到 `p:ph`。`isPlaceHolder` 会读它。
    saw_ph: bool,
}

impl Clone for KmoShowShape {
    fn clone(&self) -> Self {
        KmoShowShape {
            // rust: Java 侧 `clone()` 走 `super.clone()`（浅拷贝对象头）；Rust 无对象头，
            //       按 `ShowShapeAtomData.copy()` 语义复制原子数据（shape 深拷贝、parentGroup 共享）。
            super_: KmoAtom::new(self.super_.rawData().copy()),
            parent: self.parent,
            dgShapeId: self.dgShapeId,
            children: self.children.clone(),
            dgShapeThreadLocal: None,
            dgGroupThreadLocal: None,
            dgPicThreadLocal: None,
            dgConnThreadLocal: None,
            textRect: None,
            exObjId: self.exObjId,
            beautifyOrder: self.beautifyOrder,
            isLocked: self.isLocked,
            isEditState: self.isEditState,
            originalAutoFit: self.originalAutoFit,
            originalAnchor: self.originalAnchor,
            plain_text: self.plain_text.clone(),
            saw_ph: self.saw_ph,
        }
    }
}

impl std::fmt::Debug for KmoShowShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KmoShowShape")
            .field("dgShapeId", &self.dgShapeId)
            .field("childCount", &self.childCount())
            .finish()
    }
}

// java public interface KmoProtocol
impl KmoProtocol for KmoShowShape {
    // java public KoBin protocol()
    fn protocol(&self) -> Box<dyn crate::cn::wps::kso::kproto::KoBin::KoBin> {
      // java return ((ShowShapeAtomData)this.rawData()).shape.save();
      self.super_.rawData().shape.borrow_mut().save()
    }

    // java public void updateProtocol(KoBin data)
    fn updateProtocol(&mut self, data: &dyn crate::cn::wps::kso::kproto::KoBin::KoBin) {
      // java ((ShowShapeAtomData)this.rawData()).shape.load(data);
      self.super_.rawData().shape.borrow_mut().load(data);
    }

}

// java public class KmoShowShape extends KmoAtom<ShowShapeAtomData> implements KmoProtocol
impl KmoShowShape {
    // java public boolean isEditState()
    pub fn isEditState(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.isEditState;
      return self.isEditState;
      */
    }

    // java public void setEditState(boolean editState)
    pub fn setEditState(&mut self, editState: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.isEditState = editState;
      self.isEditState = editState;
      */
    }

    // java public  KmoShowShape(ISlide parent, int type)
    pub fn KmoShowShape__islide_int(parent: &dyn ISlide, r#type: i32) -> Self {
      // java super(new ShowShapeAtomData());
      // java int spid = parent.getPresentation().canvas().addEmptyShape(type);
      let spid: i32 = parent.getPresentation().canvas().addEmptyShape(r#type);
      // java this.init(parent, type, spid);
      Self::KmoShowShape__islide_int_int(parent, r#type, spid)
    }

    // java public  KmoShowShape(ISlide parent, int type, int spId)
    pub fn KmoShowShape__islide_int_int(parent: &dyn ISlide, r#type: i32, spId: i32) -> Self {
      // java super(new ShowShapeAtomData());
      let mut this = KmoShowShape {
        super_: KmoAtom::new(ShowShapeAtomData::new()),
        parent: None,
        dgShapeId: 0,
        children: None,
        dgShapeThreadLocal: None,
        dgGroupThreadLocal: None,
        dgPicThreadLocal: None,
        dgConnThreadLocal: None,
        textRect: None,
        exObjId: -1,
        beautifyOrder: 0,
        isLocked: false,
        isEditState: false,
        originalAutoFit: -1,
        originalAnchor: -1,
        plain_text: String::new(),
        saw_ph: false,
      };
      // java this.init(parent, type, spId);
      this.init(parent, r#type, spId);
      this
    }

    // java private void init(ISlide parent, int type, int spId)
    pub fn init(&mut self, parent: &dyn ISlide, r#type: i32, spId: i32) {
      // java this.dgShapeId = spId;
      self.dgShapeId = spId;
      // java this.parent = parent;
      self.parent = Some(unsafe {
        std::mem::transmute::<&dyn ISlide, *const dyn ISlide>(parent)
      });
      // java if (type == 0)
      if r#type == 0 {
        // java this.children = new KmoUndoableList();
        self.children = Some(Vec::new());
      }
    }

    // java public int hashCode()
    pub fn hashCode(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.id();
      return self.id();
      */
    }

    // java public boolean equals(Object o)
    pub fn equals__object(&self, o: &dyn Object) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.equals(o, true);
      return self.equals__object_boolean(Some(o), true);
      */
    }

    // java public boolean equalsLax(KmoShowShape o)
    pub fn equalsLax(&self, o: KmoShowShape) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.equals(o, false);
      return self.equals__object_boolean(Some(o), false);
      */
    }

    // java private boolean equals(Object o, boolean isStrict)
    pub fn equals__object_boolean(&mut self, o: Option<&dyn Object>, isStrict: bool) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null != o && o instanceof KmoShowShape)
      if let Some(o) = o {
        if o.downcast_ref::<KmoShowShape>().is_some() {
          // java KmoShowShape dst = (KmoShowShape)o;
          let mut dst: KmoShowShape = (o).downcast_ref::<KmoShowShape>().unwrap();
          // java if (this == dst)
          if self == dst {
            // java return true;
            return true;
          }
          // java else if (this.type() != dst.type())
          else if self.r#type() != dst.r#type() {
            // java return false;
            return false;
          }
          // java else if (!this.compareDgShape(dst, isStrict))
          else if !self.compareDgShape(dst, isStrict) {
            // java return false;
            return false;
          }
          // java else if (isStrict && !((ShowShapeAtomData)this.rawData()).shape.equals(((ShowShapeAtomData)dst.rawData()).shape))
          else if isStrict && !(self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.equals((dst.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape) {
            // java return false;
            return false;
          }
          // java else if (!isStrict && !((ShowShapeAtomData)this.rawData()).shape.equalsLax(((ShowShapeAtomData)dst.rawData()).shape))
          else if !isStrict && !(self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.equalsLax((dst.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape) {
            // java return false;
            return false;
          }
          // java else
          else {
            // java if (this.type() == 0)
            if self.r#type() == 0 {
              // java int count = this.childCount();
              let mut count: i32 = self.childCount();
              // java if (count != dst.childCount())
              if count != dst.childCount() {
                // java return false;
                return false;
              }
              let mut i: i32 = 0;
              // java for (int i = 0; i < count ++i)
              while i < count {
                // java KmoShowShape srcSp = this.childAt(i);
                let mut srcSp: KmoShowShape = self.childAt(i).unwrap();
                // java KmoShowShape dstSp = dst.childAt(i);
                let mut dstSp: KmoShowShape = dst.childAt(i).unwrap();
                // java if (isStrict && !srcSp.equals(dstSp))
                if isStrict && !srcSp.equals(dstSp) {
                  // java return false;
                  return false;
                }
                // java if (!isStrict && !srcSp.equalsLax(dstSp))
                if !isStrict && !srcSp.equalsLax(dstSp) {
                  // java return false;
                  return false;
                }
              }
            }
            // java return true;
            return true;
          }
        }
      }
      // java else
      else {
        // java return false;
        return false;
      }
      */
    }

    // java private boolean compareDgShape(KmoShowShape dst, boolean isStrict)
    pub fn compareDgShape(&self, dst: KmoShowShape, isStrict: bool) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java DgStore srcStore = this.parent.getPresentation().drawingStore();
      let mut srcStore: DgStore = self.parent.getPresentation().drawingStore();
      // java Shape srcSp = Shape.open(srcStore.query(this.dgShapeId));
      let mut srcSp: Shape = Shape::open(srcStore.query(self.dgShapeId));
      // java DgStore dstStore = dst.parent.getPresentation().drawingStore();
      let mut dstStore: DgStore = dst.parent.getPresentation().drawingStore();
      // java Shape dstSp = Shape.open(dstStore.query(dst.dgShapeId));
      let mut dstSp: Shape = Shape::open(dstStore.query(dst.dgShapeId));
      // java if (isStrict && !srcSp.equals(dstSp))
      if isStrict && !srcSp.equals(dstSp) {
        // java return false;
        return false;
      }
      // java else if (!isStrict && !srcSp.equalsLax(dstSp))
      else if !isStrict && !srcSp.equalsLax(dstSp) {
        // java return false;
        return false;
      }
      // java else
      else {
        // java return isStrict || this.compareMedia(dstSp, srcSp, dst.parent.getPresentation().mediaLib(), this.parent.getPresentation().mediaLib());
        return isStrict || self.compareMedia(dstSp, srcSp, dst.parent.getPresentation().mediaLib(), self.parent.getPresentation().mediaLib());
      }
      */
    }

    // java private boolean compareMedia(Shape dst, Shape src, KmoMediaLib dstMedialib, KmoMediaLib srcMedialib)
    pub fn compareMedia(&self, dst: Shape, src: Shape, dstMedialib: KmoMediaLib, srcMedialib: KmoMediaLib) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Blip dstBlip = dst.blipFill().blipFill().blip();
      let mut dstBlip: Blip = dst.blipFill().blipFill().blip();
      // java Blip srcBlip = src.blipFill().blipFill().blip();
      let mut srcBlip: Blip = src.blipFill().blipFill().blip();
      // java return KmoFilm.compareMedia(dstBlip, srcBlip, dstMedialib, srcMedialib);
      return KmoFilm::compareMedia__blip_blip_kmomedialib_kmomedialib({ let __x: Shape = todo!("no align strategy matched"); __x }, { let __x: Shape = todo!("no align strategy matched"); __x }, dstMedialib, srcMedialib);
      */
    }

    // java public void copy(KmoShowShape srcSp)
    pub fn copy(&mut self, srcSp: KmoShowShape) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.doCopy(srcSp, false);
      self.doCopy(srcSp, false);
      */
    }

    // java public void copyEx(KmoShowShape srcSp)
    pub fn copyEx(&mut self, srcSp: KmoShowShape) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.doCopy(srcSp, true);
      self.doCopy(srcSp, true);
      */
    }

    // java private void doCopy(KmoShowShape srcSp, boolean copyMedia)
    pub fn doCopy(&mut self, srcSp: KmoShowShape, copyMedia: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.updateProtocol(srcSp.protocol());
      self.updateProtocol(srcSp.protocol());
      // java this.dgShapeId = DgShape.copyShape(this.parent.getPresentation().drawingStore(), srcSp.drawingShape(), srcSp.graphic(), srcSp.graphicChoice());
      self.dgShapeId = DgShape::copyShape__dgstore_kobin_graphicobject_graphicobject(self.parent.getPresentation().drawingStore(), srcSp.drawingShape(), srcSp.graphic(), srcSp.graphicChoice());
      // java this.beautifyOrder = srcSp.beautifyOrder;
      self.beautifyOrder = srcSp.beautifyOrder;
      // java if (srcSp.hasInk())
      if srcSp.hasInk() {
        // java KmoInkLib inkLib = this.parent.getPresentation().inkLib();
        let mut inkLib: KmoInkLib = self.parent.getPresentation().inkLib();
        // java this.putInkId(inkLib.addInk(inkLib.getInk(srcSp.inkId())));
        self.putInkId(inkLib.addInk(inkLib.getInk(srcSp.inkId())));
      }
      // java if (copyMedia && DgStore.queryType(this.id()) == 2)
      if copyMedia && DgStore::queryType(self.id()) == 2 {
        // java this.updatePicRef4Copy(srcSp);
        self.updatePicRef4Copy(srcSp);
      }
      // java if (copyMedia && -1 != srcSp.getDgShape().fill().picId())
      if copyMedia && -1 != srcSp.getDgShape().fill().picId() {
        // java this.updateFillPicRef4Copy(srcSp);
        self.updateFillPicRef4Copy(srcSp);
      }
      // java if (copyMedia && srcSp.hasVideo())
      if copyMedia && srcSp.hasVideo() {
        // java this.updateVideoRef4Copy(srcSp);
        self.updateVideoRef4Copy(srcSp);
      }
      // java if (copyMedia && srcSp.hasAudio())
      if copyMedia && srcSp.hasAudio() {
        // java this.updateAudioRef4Copy(srcSp);
        self.updateAudioRef4Copy(srcSp);
      }
      // java if (srcSp.isOle())
      if srcSp.isOle() {
        // java this.updateOleRef4Copy(srcSp, copyMedia);
        self.updateOleRef4Copy(srcSp, copyMedia);
      }
      // java if (copyMedia && srcSp.isModel3D())
      if copyMedia && srcSp.isModel3D() {
        // java this.updateModel3DRef4Copy(srcSp);
        self.updateModel3DRef4Copy(srcSp);
      }
      // java if (copyMedia && srcSp.isChart())
      if copyMedia && srcSp.isChart() {
        // java this.updateChart4Copy(srcSp);
        self.updateChart4Copy(srcSp);
      }
      // java if (srcSp.type() == 0)
      if srcSp.r#type() == 0 {
        // java this.getDgGroup().removeAllChild();
        self.getDgGroup().removeAllChild();
        // java ArrayList<KmoShowShape> newChilds = new ArrayList();
        let mut newChilds: Vec<KmoShowShape> = { let __x: Vec<KmoShowShape> = todo!("no align strategy matched"); __x };
        // java int i = 0;
        let mut i: i32 = 0;
        let mut count: i32 = srcSp.childCount();
        // java for (int count = srcSp.childCount(); i < count ++i)
        while i < count {
          // java KmoShowShape newChild = new KmoShowShape(this.parent, srcSp.childAt(i).type());
          let mut newChild: KmoShowShape = KmoShowShape::new(self.parent, srcSp.childAt(i).r#type());
          // java newChild.doCopy(srcSp.childAt(i), copyMedia);
          newChild.doCopy(srcSp.childAt(i), copyMedia);
          // java newChilds.add(newChild);
          newChilds.add(newChild);
        }
        // java this.addChilds(newChilds);
        self.addChilds(newChilds);
      }
      */
    }

    // java private void updateModel3DRef4Copy(KmoShowShape srcSp)
    pub fn updateModel3DRef4Copy(&mut self, srcSp: KmoShowShape) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java KmoMediaLib srcMediaLib = srcSp.parent().getPresentation().mediaLib();
      let mut srcMediaLib: KmoMediaLib = srcSp.parent().getPresentation().mediaLib();
      // java KmoModel3D model3D = (KmoModel3D)srcSp.graphic();
      let mut model3D: KmoModel3D = (srcSp.graphic()).downcast_ref::<KmoModel3D>().unwrap();
      // java int mediaId = model3D.mediaId();
      let mut mediaId: i32 = model3D.mediaId();
      // java KmoMediaData srcMediaData = srcMediaLib.getMediaData(mediaId);
      let mut srcMediaData: KmoMediaData = srcMediaLib.getMediaData(mediaId);
      // java if (null != srcMediaData)
      if true {
        // java FileDataStorage srcFds = srcMediaData.getData();
        let mut srcFds: FileDataStorage = srcMediaData.getData();
        // java if (null != srcFds)
        if true {
          // java KmoMediaLib dstMediaLib = this.parent().getPresentation().mediaLib();
          let mut dstMediaLib: KmoMediaLib = self.parent().getPresentation().mediaLib();
          // java int newMediaId = dstMediaLib.addMediaData(srcFds, srcMediaData.getMediaDataType());
          let mut newMediaId: i32 = dstMediaLib.addMediaData(srcFds, srcMediaData.getMediaDataType());
          // java ((KmoModel3D)this.graphic()).setMedia(newMediaId);
          (self.graphic()).downcast_ref::<KmoModel3D>().unwrap().setMedia(newMediaId);
        }
      }
      */
    }

    // java private void updateVideoRef4Copy(KmoShowShape srcSp)
    pub fn updateVideoRef4Copy(&mut self, srcSp: KmoShowShape) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java KmoMediaLib srcMediaLib = srcSp.parent().getPresentation().mediaLib();
      let mut srcMediaLib: KmoMediaLib = srcSp.parent().getPresentation().mediaLib();
      // java int videoId = srcSp.videoMediaId();
      let mut videoId: i32 = srcSp.videoMediaId();
      // java KmoMediaData srcMediaData = srcMediaLib.getMediaData(videoId);
      let mut srcMediaData: KmoMediaData = srcMediaLib.getMediaData(videoId);
      // java if (null != srcMediaData)
      if true {
        // java FileDataStorage srcFds = srcMediaData.getData();
        let mut srcFds: FileDataStorage = srcMediaData.getData();
        // java if (null != srcFds)
        if true {
          // java KmoMediaLib dstMediaLib = this.parent().getPresentation().mediaLib();
          let mut dstMediaLib: KmoMediaLib = self.parent().getPresentation().mediaLib();
          // java int newVideoId = dstMediaLib.addMediaData(srcFds, srcMediaData.getMediaDataType());
          let mut newVideoId: i32 = dstMediaLib.addMediaData(srcFds, srcMediaData.getMediaDataType());
          // java ((ShowShapeAtomData)this.rawData()).shape.media().videoFile().putMediaID(newVideoId);
          (self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.media().videoFile().putMediaID(newVideoId);
          // java ((ShowShapeAtomData)this.rawData()).shape.save();
          (self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.save();
        }
      }
      */
    }

    // java private void updateAudioRef4Copy(KmoShowShape srcSp)
    pub fn updateAudioRef4Copy(&mut self, srcSp: KmoShowShape) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java KmoMediaLib srcMediaLib = srcSp.parent().getPresentation().mediaLib();
      let mut srcMediaLib: KmoMediaLib = srcSp.parent().getPresentation().mediaLib();
      // java int audioId = srcSp.audioMediaId();
      let mut audioId: i32 = srcSp.audioMediaId();
      // java KmoMediaData srcMediaData = srcMediaLib.getMediaData(audioId);
      let mut srcMediaData: KmoMediaData = srcMediaLib.getMediaData(audioId);
      // java if (null != srcMediaData)
      if true {
        // java FileDataStorage srcFds = srcMediaData.getData();
        let mut srcFds: FileDataStorage = srcMediaData.getData();
        // java if (null != srcFds)
        if true {
          // java KmoMediaLib dstMediaLib = this.parent().getPresentation().mediaLib();
          let mut dstMediaLib: KmoMediaLib = self.parent().getPresentation().mediaLib();
          // java int newAudioId = dstMediaLib.addMediaData(srcFds, srcMediaData.getMediaDataType());
          let mut newAudioId: i32 = dstMediaLib.addMediaData(srcFds, srcMediaData.getMediaDataType());
          // java ((ShowShapeAtomData)this.rawData()).shape.media().audioFile().putMediaID(newAudioId);
          (self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.media().audioFile().putMediaID(newAudioId);
          // java ((ShowShapeAtomData)this.rawData()).shape.save();
          (self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.save();
        }
      }
      */
    }

    // java private void updateFillPicRef4Copy(KmoShowShape srcSp)
    pub fn updateFillPicRef4Copy(&mut self, srcSp: KmoShowShape) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java KmoMediaLib srcMediaLib = srcSp.parent().getPresentation().mediaLib();
      let mut srcMediaLib: KmoMediaLib = srcSp.parent().getPresentation().mediaLib();
      // java int srcPicId = srcSp.getDgShape().fill().picId();
      let mut srcPicId: i32 = srcSp.getDgShape().fill().picId();
      // java KmoMediaData srcMediaData = srcMediaLib.getMediaData(srcPicId);
      let mut srcMediaData: KmoMediaData = srcMediaLib.getMediaData(srcPicId);
      // java if (null != srcMediaData)
      if true {
        // java FileDataStorage srcFds = srcMediaData.getData();
        let mut srcFds: FileDataStorage = srcMediaData.getData();
        // java if (null != srcFds)
        if true {
          // java KmoMediaLib dstMediaLib = this.parent().getPresentation().mediaLib();
          let mut dstMediaLib: KmoMediaLib = self.parent().getPresentation().mediaLib();
          // java DgShape dgShape = this.getDgShape();
          let mut dgShape: DgShape = self.getDgShape().unwrap();
          // java DgFillFormat fill = dgShape.fill();
          let mut fill: DgFillFormat = { let __x: DgFillFormat = todo!("no align strategy matched"); __x };
          // java int newPicId = dstMediaLib.addMediaData(srcFds, srcMediaData.getMediaDataType());
          let mut newPicId: i32 = dstMediaLib.addMediaData(srcFds, srcMediaData.getMediaDataType());
          // java fill.setPicId(newPicId);
          fill.setPicId(newPicId);
          // java dgShape.updateFill();
          dgShape.updateFill();
        }
      }
      */
    }

    // java private void updatePicRef4Copy(KmoShowShape srcSp)
    pub fn updatePicRef4Copy(&mut self, srcSp: KmoShowShape) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java KmoMediaLib srcMediaLib = srcSp.parent().getPresentation().mediaLib();
      let mut srcMediaLib: KmoMediaLib = srcSp.parent().getPresentation().mediaLib();
      // java int srcPicId = srcSp.getPicId();
      let mut srcPicId: i32 = srcSp.getPicId();
      // java KmoMediaData srcMediaData = srcMediaLib.getMediaData(srcPicId);
      let mut srcMediaData: KmoMediaData = srcMediaLib.getMediaData(srcPicId);
      // java if (null != srcMediaData)
      if true {
        // java FileDataStorage srcFds = srcMediaData.getData();
        let mut srcFds: FileDataStorage = srcMediaData.getData();
        // java if (null != srcFds)
        if true {
          // java KmoMediaLib dstMediaLib = this.parent().getPresentation().mediaLib();
          let mut dstMediaLib: KmoMediaLib = self.parent().getPresentation().mediaLib();
          // java int newPicId = dstMediaLib.addMediaData(srcFds, srcMediaData.getMediaDataType());
          let mut newPicId: i32 = dstMediaLib.addMediaData(srcFds, srcMediaData.getMediaDataType());
          // java this.getDgPicture().changePicture(newPicId);
          self.getDgPicture().changePicture(newPicId);
        }
      }
      */
    }

    // java private void updateOleRef4Copy(KmoShowShape srcSp, boolean copyMedia)
    pub fn updateOleRef4Copy(&mut self, srcSp: KmoShowShape, copyMedia: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java KmoMediaLib srcMediaLib = srcSp.parent().getPresentation().mediaLib();
      let mut srcMediaLib: KmoMediaLib = srcSp.parent().getPresentation().mediaLib();
      // java int mediaId = copyMedia ? ((KmoOleObject)srcSp.graphic()).mediaId() : ((KmoOleObject)this.graphic()).mediaId();
      let mut mediaId: i32 = (if copyMedia { (srcSp.graphic()).downcast_ref::<KmoOleObject>().unwrap().mediaId() } else { (self.graphic()).downcast_ref::<KmoOleObject>().unwrap().mediaId() });
      // java KmoMediaData srcMediaData = srcMediaLib.getMediaData(mediaId);
      let mut srcMediaData: KmoMediaData = srcMediaLib.getMediaData(mediaId);
      // java if (srcMediaData != null && srcMediaData.getData() != null)
      if true && srcMediaData.getData().is_some() {
        // java FileDataStorage dataStorage = srcMediaData.getData();
        let mut dataStorage: FileDataStorage = srcMediaData.getData();
        // java if (dataStorage.getFile() == null || dataStorage.getFile().length() <= 0L)
        if dataStorage.getFile().is_none() || (dataStorage.getFile().len() as i32) <= 0 {
          // java srcSp.parent().getPresentation().getOleManager().dumpOleBinFileIfNeed(mediaId);
          srcSp.parent().getPresentation().getOleManager().dumpOleBinFileIfNeed(mediaId);
        }
        // java if (copyMedia || dataStorage.getFile().length() <= 0L)
        if copyMedia || (dataStorage.getFile().len() as i32) <= 0 {
          // java int id = this.parent().getPresentation().mediaLib().addMediaData(dataStorage, srcMediaData.getMediaDataType());
          let mut id: i32 = self.parent().getPresentation().mediaLib().addMediaData(dataStorage, srcMediaData.getMediaDataType());
          // java ((KmoOleObject)this.graphic()).setMediaId(id);
          (self.graphic()).downcast_ref::<KmoOleObject>().unwrap().setMediaId(id);
          // java if (copyMedia && ((KmoOleObject)srcSp.graphic()).picture() != null)
          if copyMedia && (srcSp.graphic()).downcast_ref::<KmoOleObject>().unwrap().picture().is_some() {
            // java KmoShowShape picture = new KmoShowShape(this.parent(), ((KmoOleObject)srcSp.graphic()).picture().type());
            let mut picture: KmoShowShape = KmoShowShape::new(self.parent(), (srcSp.graphic()).downcast_ref::<KmoOleObject>().unwrap().picture().r#type());
            // java picture.copyOlePicture(((KmoOleObject)srcSp.graphic()).picture(), srcMediaLib);
            picture.copyOlePicture((srcSp.graphic()).downcast_ref::<KmoOleObject>().unwrap().picture(), srcMediaLib);
            // java ((KmoOleObject)this.graphic()).setPicture(picture);
            (self.graphic()).downcast_ref::<KmoOleObject>().unwrap().setPicture(picture);
          }
        }
      }
      */
    }

    // java private void copyOlePicture(KmoShowShape picShape, KmoMediaLib picMediaLib)
    pub fn copyOlePicture(&mut self, picMediaLib: KmoMediaLib) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.updateProtocol(picShape.protocol());
      self.updateProtocol(picShape.protocol());
      // java this.dgShapeId = DgShape.copyShape(this.parent().getPresentation().drawingStore(), picShape.drawingShape(), (GraphicObject)null, (GraphicObject)null);
      self.dgShapeId = DgShape::copyShape__dgstore_kobin_graphicobject_graphicobject(self.parent().getPresentation().drawingStore(), picShape.drawingShape(), None, None);
      // java int srcPicId = picShape.getPicId();
      let mut srcPicId: i32 = picShape.getPicId();
      // java KmoMediaData srcMediaData = picMediaLib.getMediaData(srcPicId);
      let mut srcMediaData: KmoMediaData = picMediaLib.getMediaData(srcPicId);
      // java if (null != srcMediaData)
      if true {
        // java FileDataStorage srcFds = srcMediaData.getData();
        let mut srcFds: FileDataStorage = srcMediaData.getData();
        // java if (null != srcFds)
        if true {
          // java KmoMediaLib dstMediaLib = this.parent().getPresentation().mediaLib();
          let mut dstMediaLib: KmoMediaLib = self.parent().getPresentation().mediaLib();
          // java int newPicId = dstMediaLib.addMediaData(srcFds, srcMediaData.getMediaDataType());
          let mut newPicId: i32 = dstMediaLib.addMediaData(srcFds, srcMediaData.getMediaDataType());
          // java this.getDgPicture().changePicture(newPicId);
          self.getDgPicture().changePicture(newPicId);
        }
      }
      */
    }

    // java private void updateChart4Copy(KmoShowShape srcSp)
    pub fn updateChart4Copy(&mut self, srcSp: KmoShowShape) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java KmoMediaLib srcMediaLib = srcSp.parent().getPresentation().mediaLib();
      let mut srcMediaLib: KmoMediaLib = srcSp.parent().getPresentation().mediaLib();
      // java KmoOleChart oleChart = (KmoOleChart)srcSp.graphic();
      let mut oleChart: KmoOleChart = (srcSp.graphic()).downcast_ref::<KmoOleChart>().unwrap();
      // java Map<String, Integer> mediaMap = oleChart.getChartMediaRidMap();
      let mut mediaMap: HashMap<String, i32> = oleChart.getChartMediaRidMap();
      // java if (mediaMap != null && mediaMap.size() != 0)
      if true && (mediaMap.len() as i32) != 0 {
        // java for (Map.Entry<String, Integer> item : mediaMap.entrySet())
        for item in mediaMap.entrySet().iter() {
          // java String rid = (String)item.getKey();
          let mut rid: String = { let __x: String = todo!("no align strategy matched"); __x };
          // java if (item.getValue() != null)
          if item.getValue().is_some() {
            // java KmoMediaData srcMediaData = srcMediaLib.getMediaData((Integer)item.getValue());
            let mut srcMediaData: KmoMediaData = srcMediaLib.getMediaData(((item.getValue()) as i32));
            // java if (srcMediaData != null)
            if true {
              // java FileDataStorage srcFds = srcMediaData.getData();
              let mut srcFds: FileDataStorage = srcMediaData.getData();
              // java if (srcFds != null)
              if true {
                // java KmoMediaLib dstMediaLib = this.parent().getPresentation().mediaLib();
                let mut dstMediaLib: KmoMediaLib = self.parent().getPresentation().mediaLib();
                // java int newId = dstMediaLib.addMediaData(srcFds, srcMediaData.getMediaDataType());
                let mut newId: i32 = dstMediaLib.addMediaData(srcFds, srcMediaData.getMediaDataType());
                // java oleChart.addMeidaItem(rid, newId);
                oleChart.addMeidaItem(rid, newId);
              }
            }
          }
        }
      }
      */
    }

    // java public KoBin drawingShape()
    pub fn drawingShape(&self) -> Box<dyn crate::cn::wps::kso::kproto::KoBin::KoBin> {
      // java return this.parent.getPresentation().drawingStore().query(this.dgShapeId);
      match self.parent {
        Some(p) => {
          let store = unsafe { (*p).getPresentation().drawingStore() };
          if store.is_null() {
            crate::cn::wps::kso::kproto::KoBin::Null::value1()
          } else {
            unsafe { (*store).query(self.dgShapeId) }
          }
        }
        None => crate::cn::wps::kso::kproto::KoBin::Null::value1(),
      }
    }

    // java public ISlide parent()
    pub fn parent(&self) -> Option<*const dyn ISlide> {
      // java return this.parent;
      self.parent
    }

    // java public ISlideCache getSlideCache()
    pub fn getSlideCache(&self) -> Option<ISlideCache> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.parent.getPresentation().getSlideCache();
      return self.parent.getPresentation().getSlideCache();
      */
    }

    // java public IMediaLib getMediaLib()
    pub fn getMediaLib(&self) -> Option<IMediaLib> {
      // java return this.parent.getPresentation().getMediaLib();
      None
    }

    // java public KmoShowShape parentGroup()
    pub fn parentGroup(&self) -> Option<KmoShowShape> {
      // java if (((ShowShapeAtomData)this.rawData()).parentGroup == null)
      if true {
        // java return null;
        return None;
      }
      // java else
      else {
        // java return ((ShowShapeAtomData)((ShowShapeAtomData)this.rawData()).parentGroup.rawData()).parentGroup == null ? null : ((ShowShapeAtomData)this.rawData()).parentGroup;
        return None;
      }
    }

    // java public KmoShowShape ancestor()
    pub fn ancestor(&self) -> Option<KmoShowShape> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (!this.isChildOfGroup())
      if !self.isChildOfGroup() {
        // java return this;
        return Some(self);
      }
      // java else
      else {
        // java KmoShowShape ancestor;
        let mut ancestor: KmoShowShape = KmoShowShape::default();
        ancestor = self.parentGroup();
        // java for (ancestor = this.parentGroup() ancestor.isChildOfGroup() ancestor = ancestor.parentGroup())
        while ancestor.isChildOfGroup() {
        }
        // java return ancestor;
        return Some(ancestor);
      }
      */
    }

    // java public boolean isChildOfGroup()
    pub fn isChildOfGroup(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.parentGroup() != null;
      return self.parentGroup().is_some();
      */
    }

    // java public void setParentGroup(KmoShowShape parentGroup)
    pub fn setParentGroup(&mut self, parentGroup: KmoShowShape) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.beforeWrite();
      self.beforeWrite();
      // java ((ShowShapeAtomData)this.rawData()).parentGroup = parentGroup;
      (self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().parentGroup = parentGroup;
      */
    }

    pub(crate) fn setPlainText(&mut self, text: String) {
      self.plain_text = text;
    }

    pub(crate) fn markPlaceholder(&mut self) {
      self.saw_ph = true;
    }

    // java public void addChild(KmoShowShape child)
    pub fn addChild(&mut self, child: KmoShowShape) {
      // java child.beforeWrite();
      // rust: 真实实现（`KmoAtom::beforeWrite` ⇒ `AtomStoreImpl::needCopy` 的写前 COW），
      //       原先登记为 `no-stmt`；与 §27 的 `removeChildAt` 同一族，一并接线。
      let mut child = child;
      child.super_.beforeWrite();
      // java ((ShowShapeAtomData)child.rawData()).parentGroup = this;
      // rust: 偏差登记 —— Java 写 `parentGroup = this`（指向父对象的引用）。
      //       Rust 的 `children: Vec<KmoShowShape>` 是**值语义**，父对象在 `&mut self` 下
      //       不持有 `Rc<RefCell<KmoShowShape>>`，无法构造指向自身的 `Weak`。
      //       故本句保持 `no-stmt`；对应的反向引用由调用方在具备 `Rc` 时经
      //       `setParentGroup` 建立。owner: show.app.shape    wave: M-PPT-KMOSLIDE-W6c
      let child_id = child.id();
      // java this.children.add(child);
      self.children.as_mut().expect("NullPointerException").push(child);
      // java this.getDgGroup().add(child.id());
      self.getDgGroup().add(child_id);
    }

    pub(crate) fn replaceChildAt(&mut self, index: i32, child: KmoShowShape) {
      self.children.as_mut().expect("NullPointerException")[index as usize] = child;
    }

    // java public void addChilds(List<KmoShowShape> childs)
    pub fn addChilds(&mut self, childs: Vec<KmoShowShape>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java IIntList childIds = new WIntArrayList();
      let mut childIds: Box<dyn IIntList> = { let __x: Box<Box<dyn IIntList>> = todo!("no align strategy matched"); __x };
      // java for (KmoShowShape child : childs)
      for child in childs.iter() {
        // java child.beforeWrite();
        child.beforeWrite();
        // java ((ShowShapeAtomData)child.rawData()).parentGroup = this;
        (child.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().parentGroup = self;
        // java this.children.add(child);
        self.children.add(child);
        // java childIds.add(child.dgShapeId);
        childIds.add(child.dgShapeId);
      }
      // java this.getDgGroup().addAll(childIds);
      self.getDgGroup().addAll(childIds);
      */
    }

    // java public void insertChilds(int index, List<KmoShowShape> childs)
    pub fn insertChilds(&mut self, childs: Vec<KmoShowShape>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java KoIntList childIds = new KoIntList();
      let mut childIds: KoIntList = KoIntList::new();
      // java int i = 0;
      let mut i: i32 = 0;
      let mut count: i32 = (childs.len() as i32);
      // java for (int count = childs.size(); i < count ++i)
      while i < count {
        // java KmoShowShape child = (KmoShowShape)childs.get(i);
        let mut child: KmoShowShape = (childs.get(i)).downcast_ref::<KmoShowShape>().unwrap();
        // java child.beforeWrite();
        child.beforeWrite();
        // java ((ShowShapeAtomData)child.rawData()).parentGroup = this;
        (child.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().parentGroup = self;
        // java this.children.add(index + i, child);
        self.children.add(index + i, child);
        // java childIds.add(child.dgShapeId);
        childIds.add(child.dgShapeId);
      }
      // java this.getDgGroup().insertAll(childIds, index);
      self.getDgGroup().insertAll(childIds, index);
      */
    }

    // java public void insertChild(int index, KmoShowShape child)
    pub fn insertChild(&mut self, child: KmoShowShape) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java child.beforeWrite();
      child.beforeWrite();
      // java ((ShowShapeAtomData)child.rawData()).parentGroup = this;
      (child.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().parentGroup = self;
      // java this.children.add(index, child);
      self.children.add(index, child);
      // java this.getDgGroup().insert(index, child.id());
      self.getDgGroup().insert(index, child.id());
      */
    }

    // java public boolean removeChildAt(int index)
    pub fn removeChildAt(&mut self, index: i32) -> bool {
      // java if (index < 0)
      if index < 0 {
        // java return false;
        return false;
      }
      // java else
      else {
        // java ((KmoShowShape)this.children.get(index)).beforeWrite();
        // rust: `KmoAtom::beforeWrite()` 是真实实现（`AtomStoreImpl::needCopy` 的写前 COW），
        //       经 `super_` 转发，与 Java 的继承调用等价。
        self.children.as_mut().expect("NullPointerException")[index as usize].super_.beforeWrite();
        // java ((ShowShapeAtomData)((KmoShowShape)this.children.get(index)).rawData()).parentGroup = null;
        // rust: Java 置 null ⇒ Rust 后向引用置 `None`（与 `ShowShapeAtomData.parentGroup: Option<Weak<..>>` 同构）。
        self.children.as_mut().expect("NullPointerException")[index as usize]
          .super_
          .rawData_mut()
          .parentGroup = None;
        // java this.children.remove(index);
        self.children.as_mut().expect("NullPointerException").remove(index as usize);
        // java this.getDgGroup().removeAt(index);
        self.getDgGroup().removeAt(index);
        // java return true;
        return true;
      }
    }

    // java public void removeAllChild()
    pub fn removeAllChild(&mut self) {
      // java for (int sz = this.childCount(); sz > 0; --sz)
      let mut sz: i32 = self.childCount();
      while sz > 0 {
        // java this.removeChildAt(0);
        self.removeChildAt(0);
        sz -= 1;
      }
    }

    // java public int indexOfChild(KmoShowShape child)
    pub fn indexOfChild(&self, child: &KmoShowShape) -> i32 {
      // java return this.children.indexOf(child);
      match self.children.as_ref().expect("NullPointerException").iter().position(|c| c.dgShapeId == child.dgShapeId) {
        Some(i) => i as i32,
        None => -1,
      }
    }

    // java public KmoShowShape childAt(int index)
    pub fn childAt(&self, index: i32) -> &KmoShowShape {
      // java return (KmoShowShape)this.children.get(index);
      &self.children.as_ref().expect("NullPointerException")[index as usize]
    }

    // java public int childCount()
    pub fn childCount(&self) -> i32 {
      // java return this.children == null ? 0 : this.children.size();
      return (if self.children.is_none() { 0 } else { (self.children.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) });
    }

    // java public GraphicObject graphic()
    // rust: ★W6c-10 —— 真实读取（替代恒 `return None` 的静默丢弃型假面）。Java 原文：
    //   `if (this.type() != 4) return null; else { DgStore store = this.parent.getPresentation().
    //   drawingStore(); Shape sp = Shape.open(store.query(this.id())); return
    //   store.graphicStore().query(sp.graphic().id()); }`
    //   三个依赖件均已真身化：① `Shape::graphic()`（`Shape.rs:1591`，经 `map[12]`/父级跟随）；
    //   ② `Shape::open(store.query(id))`；③ `store.graphicStore().query(gid)`（本波：真身注册表
    //   `drawing/app/graphic/GraphicStore.rs`，Java 54 行）。
    //   `parent == None` / 仓为空 ⇒ Java 的 NPE/`query(id)` 越界语义 ⇒ 返回 `None`（Java `null`）。
    //   ⚠ 返回 `&'static`：仓是**泄漏的**（`ImportHost::leak_with_store`/`Box::leak`），
    //   `GraphicObject` 由仓持有，引用生命周期与调用方无关 —— 用 `&'static` 表达"引用不借自
    //   `&self`"（否则调用方在持有该引用时无法 `borrow_mut` 同一个 `KmoShowShape`/组）。
    //   owner: show.app.shape.KmoShowShape    wave: M-PPT-KMOSLIDE-W6c-10
    pub fn graphic(&self) -> Option<&'static dyn GraphicObject> {
      // java if (this.type() != 4)
      if self.r#type() != 4 {
        // java return null;
        return None;
      }
      // java else
      else {
        // java DgStore store = this.parent.getPresentation().drawingStore();
        let Some(p) = self.parent else {
          // java: null-parent ⇒ NPE；按本仓 active-path 约定返回 None（下游不经它写入）。
          return None;
        };
        let store = unsafe { (*p).getPresentation().drawingStore() };
        if store.is_null() {
          return None;
        }
        // java Shape sp = Shape.open(store.query(this.id()));
        let gid = {
          let bin = unsafe { (*store).query(self.dgShapeId) };
          let sp = crate::cn::wps::drawing::data::shape::Shape::Shape::open(bin.as_ref());
          // java return store.graphicStore().query(sp.graphic().id());
          sp.graphic().id()
        };
        unsafe { &*store }.graphicStore().query(gid)
      }
    }

    // java public GraphicObject graphicChoice()
    pub fn graphicChoice(&self) -> Option<&'static dyn GraphicObject> {
      // rust: ★W6c-10 —— 与 `graphic()` 同族真实实现。Java 原文：
      //   `if (this.type() != 4) return null; else { … Shape sp = Shape.open(store.query(this.id()));
      //   return sp.graphic().hasChoiceId() ? store.graphicStore().query(sp.graphic().choiceId()) : null; }`
      //   返回 `&'static` 的理由同 `graphic()`（仓为泄漏单例）。
      //   owner: show.app.shape.KmoShowShape    wave: M-PPT-KMOSLIDE-W6c-10
      if self.r#type() != 4 {
        // java return null;
        return None;
      }
      else {
        let Some(p) = self.parent else {
          return None;
        };
        let store = unsafe { (*p).getPresentation().drawingStore() };
        if store.is_null() {
          return None;
        }
        let choice = {
          let bin = unsafe { (*store).query(self.dgShapeId) };
          let sp = crate::cn::wps::drawing::data::shape::Shape::Shape::open(bin.as_ref());
          if sp.graphic().hasChoiceId() {
            Some(sp.graphic().choiceId())
          } else {
            None
          }
        };
        // java return sp.graphic().hasChoiceId() ? store.graphicStore().query(sp.graphic().choiceId()) : null;
        match choice {
          Some(gid) => unsafe { &*store }.graphicStore().query(gid),
          None => None,
        }
      }
    }

    // java public int type()
    pub fn r#type(&self) -> i32 {
      // java return DgStore.queryType(this.dgShapeId);
      return DgStoreData::queryType(self.dgShapeId);
    }

    // java public int id()
    pub fn id(&self) -> i32 {
      // java return this.dgShapeId;
      return self.dgShapeId;
    }

    // java public boolean isPlaceHolder()
    pub fn isPlaceHolder(&self) -> bool {
      // java return ((ShowShapeAtomData)this.rawData()).shape.hasPh();
      self.saw_ph
        || self.super_.rawData().shape.borrow().hasPh()
        || super::kmo_show_shape_hosts::convert_attach(self.dgShapeId).is_placeholder
    }

    // java public void writePh(ShowShape.Placeholder ph)
    pub fn writePh(&mut self, ph: Placeholder) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.beforeWrite();
      self.beforeWrite();
      // java ((ShowShapeAtomData)this.rawData()).shape.writePh(ph);
      (self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.writePh(ph);
      */
    }

    // java public void deletePh()
    pub fn deletePh(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.beforeWrite();
      self.beforeWrite();
      // java ((ShowShapeAtomData)this.rawData()).shape.deletePh();
      (self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.deletePh();
      */
    }

    // java public ShowShape.Placeholder ph()
    pub fn ph(&self) -> Option<Placeholder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return ((ShowShapeAtomData)this.rawData()).shape.ph();
      return (self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.ph();
      */
    }

    // java public int phType()
    pub fn phType(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return PlaceHolderMerger.phType(this);
      return PlaceHolderMerger::phType(self);
      */
    }

    // java public CustDataLst custDataLst()
    // rust: ★③-5 真实实现（原为 `todo!()`）。Java 原文：
    //   return ((ShowShapeAtomData)this.rawData()).shape.custDataLst();
    //   `rawData().shape` 在本 port 是 `RefCell<ShowShape>`（见 `ShowShapeAtomData` 的登记偏差），
    //   真 `ShowShape::custDataLst()` 取 `&mut self` ⇒ `borrow_mut()`。
    //   Java 侧返回内部活引用；Rust 侧返回值拷贝（`CustDataLst` 无共享槽位，与本仓既有约定一致）。
    //   owner: show.app.shape.KmoShowShape    wave: M-PPT-SHAPE-DISPATCH
    pub fn custDataLst(&self) -> Option<CustDataLst> {
      // java return ((ShowShapeAtomData)this.rawData()).shape.custDataLst();
      // rust: 真 `CustDataLst` 无 `Clone`（KProto 容器模型）⇒ 按本仓既有约定用
      //   `save()` → `CustDataLst::open(bin)` 做**值拷贝**（等价于 Java 的活引用读取结果）。
      //   凡改动本值的调用方须写回原对象（`custDataLst_mut()`），见 owner 登记。
      let bin = self.super_.rawData().shape.borrow_mut().custDataLst().save();
      Some(CustDataLst::open(bin.as_ref()))
    }

    // java public boolean hasCustDataLst()
    // rust: ★③-5 真实实现（原为 `todo!()`）。Java 原文：
    //   return ((ShowShapeAtomData)this.rawData()).shape.hasCustDataLst();
    //   owner: show.app.shape.KmoShowShape    wave: M-PPT-SHAPE-DISPATCH
    pub fn hasCustDataLst(&self) -> bool {
      // java return ((ShowShapeAtomData)this.rawData()).shape.hasCustDataLst();
      self.super_.rawData().shape.borrow().hasCustDataLst()
    }

    // java public void setNoGrp(boolean isNogrp)
    pub fn setNoGrp(&mut self, isNogrp: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.getDgShape().setNoGrp(isNogrp);
      self.getDgShape().setNoGrp(isNogrp);
      */
    }

    // java private void findParents(IIntList parentIds)
    pub fn findParents(&mut self, parentIds: IIntList) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.isChildOfGroup())
      if self.isChildOfGroup() {
        // java KmoShowShape parentGroup = this.parentGroup();
        let mut parentGroup: KmoShowShape = self.parentGroup().unwrap();
        // java parentIds.add(parentGroup.dgShapeId);
        parentIds.add__int(parentGroup.dgShapeId);
        // java parentGroup.findParents(parentIds);
        parentGroup.findParents(parentIds);
      }
      */
    }

    // java public boolean hasInk()
    pub fn hasInk(&self) -> bool {
      // java return ((ShowShapeAtomData)this.rawData()).shape.hasInkId();
      false
    }

    // java public int inkId()
    pub fn inkId(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return ((ShowShapeAtomData)this.rawData()).shape.inkId();
      return (self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.inkId();
      */
    }

    // java public void putInkId(int inkId)
    pub fn putInkId(&mut self, inkId: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java ((ShowShapeAtomData)this.rawData()).shape.putInkId(inkId);
      (self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.putInkId(inkId);
      */
    }

    // java public boolean hasHlinkClick()
    pub fn hasHlinkClick(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.getDgShape().hasHlinkClick();
      return self.getDgShape().hasHlinkClick();
      */
    }

    // java public String hlinkClickAction()
    pub fn hlinkClickAction(&self) -> String {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.getDgShape().hlinkClickAction();
      return self.getDgShape().hlinkClickAction();
      */
    }

    // java public String hlinkClickTarget()
    pub fn hlinkClickTarget(&self) -> String {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.getDgShape().hlinkClickTarget();
      return self.getDgShape().hlinkClickTarget();
      */
    }

    // java public boolean hlinkClickTargetIsInternal()
    pub fn hlinkClickTargetIsInternal(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.getDgShape().hlinkClickIsInternal();
      return self.getDgShape().hlinkClickIsInternal();
      */
    }

    // java public void deleteHlinkClick()
    pub fn deleteHlinkClick(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.hasHlinkClick())
      if self.hasHlinkClick() {
        // java this.getDgShape().deleteHlinkClick();
        self.getDgShape().deleteHlinkClick();
        // java this.parent.getPresentation().setDirty();
        self.parent.getPresentation().setDirty();
        // java this.parent.eventListeners().onShapeContentsChanged(3, this);
        self.parent.eventListeners().onShapeContentsChanged(3, self);
        // java this.parent.eventListeners().onSlideContentsChanged();
        self.parent.eventListeners().onSlideContentsChanged();
      }
      */
    }

    // java public boolean setHyperlink(Hyperlink hyperlink)
    pub fn setHyperlink(&mut self, hyperlink: Hyperlink) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.getDgShape().setHlinkClick(hyperlink);
      self.getDgShape().setHlinkClick(hyperlink);
      // java this.parent.getPresentation().setDirty();
      self.parent.getPresentation().setDirty();
      // java this.parent.eventListeners().onShapeContentsChanged(3, this);
      self.parent.eventListeners().onShapeContentsChanged(3, self);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      // java return false;
      return false;
      */
    }

    // java public boolean hasHlinkHover()
    pub fn hasHlinkHover(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.getDgShape().hasHlinkOnHover();
      return self.getDgShape().hasHlinkOnHover();
      */
    }

    // java public String hlinkHoverAction()
    pub fn hlinkHoverAction(&self) -> String {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.getDgShape().hlinkHoverAction();
      return self.getDgShape().hlinkHoverAction();
      */
    }

    // java public String hlinkHoverTarget()
    pub fn hlinkHoverTarget(&self) -> String {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.getDgShape().hlinkHoverTarget();
      return self.getDgShape().hlinkHoverTarget();
      */
    }

    // java public boolean hlinkHoverTargetIsInternal()
    pub fn hlinkHoverTargetIsInternal(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.getDgShape().hlinkHoverIsInternal();
      return self.getDgShape().hlinkHoverIsInternal();
      */
    }

    // java public void deleteHlinkHover()
    pub fn deleteHlinkHover(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.hasHlinkHover())
      if self.hasHlinkHover() {
        // java this.getDgShape().deleteHlinkHover();
        self.getDgShape().deleteHlinkHover();
        // java this.parent.getPresentation().setDirty();
        self.parent.getPresentation().setDirty();
        // java this.parent.eventListeners().onShapeContentsChanged(3, this);
        self.parent.eventListeners().onShapeContentsChanged(3, self);
        // java this.parent.eventListeners().onSlideContentsChanged();
        self.parent.eventListeners().onSlideContentsChanged();
      }
      */
    }

    // java public void setHyperlinkOnHover(Hyperlink hyperlink)
    pub fn setHyperlinkOnHover(&mut self, hyperlink: Hyperlink) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.getDgShape().setHlinkHover(hyperlink);
      self.getDgShape().setHlinkHover(hyperlink);
      // java this.parent.getPresentation().setDirty();
      self.parent.getPresentation().setDirty();
      // java this.parent.eventListeners().onShapeContentsChanged(3, this);
      self.parent.eventListeners().onShapeContentsChanged(3, self);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      */
    }

    // java public void move(int offsetX, int offsetY)
    pub fn r#move(&mut self, offsetY: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.isPlaceHolder() && !this.getDgShape().hasTransform2D())
      if self.isPlaceHolder() && !self.getDgShape().hasTransform2D() {
        // java this.getDgShape().writeTransform2D(PlaceHolderMerger.mergeTransform(this));
        self.getDgShape().writeTransform2D(PlaceHolderMerger::mergeTransform(self));
      }
      // java if (this.isChildOfGroup())
      if self.isChildOfGroup() {
        // java IIntList parentIds = new WIntArrayList();
        let mut parentIds: Box<dyn IIntList> = { let __x: Box<Box<dyn IIntList>> = todo!("no align strategy matched"); __x };
        // java this.findParents(parentIds);
        self.findParents(parentIds);
        // java this.getDgShape().move(offsetX, offsetY, parentIds);
        self.getDgShape().r#move(offsetX, offsetY, parentIds);
        // java this.adjustParentTransform();
        self.adjustParentTransform();
      }
      // java else
      else {
        // java this.getDgShape().move(offsetX, offsetY);
        self.getDgShape().r#move(offsetX, offsetY);
      }
      // java this.parent.getPresentation().setDirty();
      self.parent.getPresentation().setDirty();
      // java KmoShowShape[] changedShapes = this.getAdjustTransformShapes(0);
      let mut changedShapes: Vec<KmoShowShape> = self.getAdjustTransformShapes(0);
      // java this.parent.eventListeners().onShapeContentsChanged(0, changedShapes);
      self.parent.eventListeners().onShapeContentsChanged(0, changedShapes);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      */
    }

    // java public void relocateModularGroup(int x, int y)
    pub fn relocateModularGroup(&mut self, x: i32, y: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.getDgShape().relocateModularGroup(x, y);
      self.getDgShape().relocateModularGroup(x, y);
      */
    }

    // java public void resize(int offsetLeft, int offsetTop, int offsetRight, int offsetBottom)
    pub fn resize__int_int_int_int(&mut self, offsetLeft: i32, offsetTop: i32, offsetRight: i32, offsetBottom: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.resize(offsetLeft, offsetTop, offsetRight, offsetBottom, true, true);
      self.resize__int_int_int_int_boolean_boolean(offsetLeft, offsetTop, offsetRight, offsetBottom, (true as i32), true);
      */
    }

    // java public void autoResize(int offsetLeft, int offsetTop, int offsetRight, int offsetBottom)
    pub fn autoResize(&mut self, offsetLeft: i32, offsetTop: i32, offsetRight: i32, offsetBottom: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.resize(offsetLeft, offsetTop, offsetRight, offsetBottom, false, true);
      self.resize__int_int_int_int_boolean_boolean(offsetLeft, offsetTop, offsetRight, offsetBottom, (false as i32), true);
      */
    }

    // java public void resizeAfterTableInsertRow()
    pub fn resizeAfterTableInsertRow(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.type() == 4 && this.getDgShape().graphicType() == 1)
      if self.r#type() == 4 && self.getDgShape().graphicType() == 1 {
        // java IIntList parentIds = new WIntArrayList();
        let mut parentIds: Box<dyn IIntList> = { let __x: Box<Box<dyn IIntList>> = todo!("no align strategy matched"); __x };
        // java this.findParents(parentIds);
        self.findParents(parentIds);
        // java KmoTable table = (KmoTable)this.parent.getPresentation().drawingStore().graphicStore().query(this.getDgShape().graphicId());
        let mut table: KmoTable = (self.parent.getPresentation().drawingStore().graphicStore().query(self.getDgShape().graphicId())).downcast_ref::<KmoTable>().unwrap();
        // java Transform2D transform2D = this.getDgShape().transform2D();
        let mut transform2D: Transform2D = self.getDgShape().transform2D().unwrap();
        // java transform2D.extents().putCx(table.tableWidth());
        transform2D.extents().putCx(table.tableWidth());
        // java transform2D.extents().putCy(table.tableHeight());
        transform2D.extents().putCy(table.tableHeight());
        // java transform2D.save();
        transform2D.save();
        // java this.getDgShape().writeTransform2D(transform2D);
        self.getDgShape().writeTransform2D(transform2D);
      }
      */
    }

    // java private void resize(int offsetLeft, int offsetTop, int offsetRight, int offsetBottom, boolean isSetDirty, boolean isReDraw)
    pub fn resize__int_int_int_int_boolean_boolean(&mut self, offsetLeft: i32, offsetTop: i32, offsetRight: i32, offsetBottom: i32, isSetDirty: bool, isReDraw: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.isPlaceHolder() && !this.getDgShape().hasTransform2D())
      if self.isPlaceHolder() && !self.getDgShape().hasTransform2D() {
        // java this.getDgShape().writeTransform2D(PlaceHolderMerger.mergeTransform(this));
        self.getDgShape().writeTransform2D(PlaceHolderMerger::mergeTransform(self));
      }
      // java IIntList parentIds = new WIntArrayList();
      let mut parentIds: Box<dyn IIntList> = { let __x: Box<Box<dyn IIntList>> = todo!("no align strategy matched"); __x };
      // java this.findParents(parentIds);
      self.findParents(parentIds);
      // java if (this.type() == 4 && this.getDgShape().graphicType() == 1)
      if self.r#type() == 4 && self.getDgShape().graphicType() == 1 {
        // java KmoTable table = (KmoTable)this.parent.getPresentation().drawingStore().graphicStore().query(this.getDgShape().graphicId());
        let mut table: KmoTable = (self.parent.getPresentation().drawingStore().graphicStore().query(self.getDgShape().graphicId())).downcast_ref::<KmoTable>().unwrap();
        // java Transform2D transform2D = this.getDgShape().transform2D();
        let mut transform2D: Transform2D = self.getDgShape().transform2D().unwrap();
        // java transform2D.extents().putCx(table.tableWidth());
        transform2D.extents().putCx(table.tableWidth());
        // java transform2D.extents().putCy(table.tableHeight());
        transform2D.extents().putCy(table.tableHeight());
        // java transform2D.save();
        transform2D.save();
        // java this.getDgShape().writeTransform2D(transform2D);
        self.getDgShape().writeTransform2D(transform2D);
        // java int oldCx = this.getDgShape().cx();
        let mut oldCx: i32 = self.getDgShape().cx();
        // java int oldCy = this.getDgShape().cy();
        let mut oldCy: i32 = self.getDgShape().cy();
        // java this.getDgShape().resizeWithoutFlip((float)offsetTop, (float)offsetLeft, (float)offsetBottom, (float)offsetRight, parentIds, (float)(table.colSize() * 208280), (float)(table.rowSize() * 135713));
        self.getDgShape().resizeWithoutFlip(((offsetTop) as f32), ((offsetLeft) as f32), ((offsetBottom) as f32), ((offsetRight) as f32), parentIds, ((table.colSize() * 208280) as f32), ((table.rowSize() * 135713) as f32));
        // java table.resizeHeight(oldCy, this.getDgShape().cy());
        table.resizeHeight(oldCy, self.getDgShape().cy());
        // java table.resizeWidth(oldCx, this.getDgShape().cx());
        table.resizeWidth(oldCx, self.getDgShape().cx());
      }
      // java else
      else {
        // java this.getDgShape().resize((float)offsetTop, (float)offsetLeft, (float)offsetBottom, (float)offsetRight, parentIds);
        self.getDgShape().resize(((offsetTop) as f32), ((offsetLeft) as f32), ((offsetBottom) as f32), ((offsetRight) as f32), parentIds);
      }
      // java if (this.isChildOfGroup())
      if self.isChildOfGroup() {
        // java this.adjustParentTransform();
        self.adjustParentTransform();
      }
      // java if (isSetDirty)
      if isSetDirty {
        // java this.parent.getPresentation().setDirty();
        self.parent.getPresentation().setDirty();
      }
      // java if (isReDraw)
      if isReDraw {
        // java KmoShowShape[] changedShapes = this.getAdjustTransformShapes(1);
        let mut changedShapes: Vec<KmoShowShape> = self.getAdjustTransformShapes(1);
        // java this.parent.eventListeners().onShapeContentsChanged(1, changedShapes);
        self.parent.eventListeners().onShapeContentsChanged(1, changedShapes);
        // java this.parent.eventListeners().onSlideContentsChanged();
        self.parent.eventListeners().onSlideContentsChanged();
      }
      */
    }

    // java private void resizeModularPicSelf(int offsetLeft, int offsetTop, int offsetRight, int offsetBottom, boolean isSetDirty, boolean isReDraw)
    pub fn resizeModularPicSelf(&mut self, offsetLeft: i32, offsetTop: i32, offsetRight: i32, offsetBottom: i32, isSetDirty: bool, isReDraw: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.getDgShape().resize(offsetTop, offsetLeft, offsetBottom, offsetRight);
      self.getDgShape().resize(offsetTop, offsetLeft, offsetBottom, offsetRight);
      // java if (this.isChildOfGroup())
      if self.isChildOfGroup() {
        // java this.adjustParentTransform();
        self.adjustParentTransform();
      }
      // java if (isSetDirty)
      if isSetDirty {
        // java this.parent.getPresentation().setDirty();
        self.parent.getPresentation().setDirty();
      }
      */
    }

    // java private void resizeCrop(int offsetLeft, int offsetTop, int offsetRight, int offsetBottom, boolean isSetDirty)
    pub fn resizeCrop(&mut self, offsetLeft: i32, offsetTop: i32, offsetRight: i32, offsetBottom: i32, isSetDirty: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.isPlaceHolder() && !this.getDgShape().hasTransform2D())
      if self.isPlaceHolder() && !self.getDgShape().hasTransform2D() {
        // java this.getDgShape().writeTransform2D(PlaceHolderMerger.mergeTransform(this));
        self.getDgShape().writeTransform2D(PlaceHolderMerger::mergeTransform(self));
      }
      // java if (this.type() == 2)
      if self.r#type() == 2 {
        // java this.getDgShape().resize(offsetTop, offsetLeft, offsetBottom, offsetRight);
        self.getDgShape().resize(offsetTop, offsetLeft, offsetBottom, offsetRight);
      }
      // java if (this.isChildOfGroup())
      if self.isChildOfGroup() {
        // java this.adjustParentTransform();
        self.adjustParentTransform();
      }
      // java if (isSetDirty)
      if isSetDirty {
        // java this.parent.getPresentation().setDirty();
        self.parent.getPresentation().setDirty();
      }
      */
    }

    // java public void groupUp(KmoShowShape[] childs)
    pub fn groupUp(&mut self, childs: Vec<KmoShowShape>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java assert this.type() == 0;
      todo!();
      // java List<Integer> childIds = new ArrayList();
      let mut childIds: Vec<i32> = { let __x: Vec<i32> = todo!("no align strategy matched"); __x };
      // java for (KmoShowShape child : childs)
      for child in childs.iter() {
        // java this.children.add(child);
        self.children.add(child);
        // java childIds.add(child.id());
        childIds.add(child.id());
        // java child.setParentGroup(this);
        child.setParentGroup(self);
      }
      // java this.getDgGroup().groupUp(childIds);
      self.getDgGroup().groupUp(childIds);
      */
    }

    // java public int[] ungroup()
    pub fn ungroup(&mut self) -> Vec<i32> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java assert this.type() == 0;
      todo!();
      // java Transform2D[] childrensTransform2D = new Transform2D[this.children.size()];
      let mut childrensTransform2D: Vec<Transform2D> = vec![Default::default(); (self.children.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) as usize];
      // java for (KmoShowShape child : this.children)
      for child in self.children.iter() {
        // java cn.wps.graphics.RectF rcShape = new cn.wps.graphics.RectF();
        let mut rcShape: RectF = RectF::new();
        // java KmoShapeUtil.calcShapeRect(child, rcShape);
        KmoShapeUtil::calcShapeRect(child, rcShape);
        // java int[] flip = new int[2];
        let mut flip: Vec<i32> = vec![Default::default(); 2usize];
        // java KmoShapeUtil.calcShapeFlip(child, flip, true);
        KmoShapeUtil::calcShapeFlip(child, flip, true);
        // java int res_rotation = KmoShapeUtil.calcShapeRotation(child, true);
        let mut res_rotation: i32 = KmoShapeUtil::calcShapeRotation(child, true);
        // java Transform2D transform2D = Transform2D.create();
        let mut transform2D: Transform2D = Transform2D::create();
        // java transform2D.point().putX(Math.round(rcShape.left));
        transform2D.point().putX(Math::round(rcShape.left));
        // java transform2D.point().putY(Math.round(rcShape.top));
        transform2D.point().putY(Math::round(rcShape.top));
        // java transform2D.extents().putCx(Math.round(rcShape.width()));
        transform2D.extents().putCx(Math::round(rcShape.width()));
        // java transform2D.extents().putCy(Math.round(rcShape.height()));
        transform2D.extents().putCy(Math::round(rcShape.height()));
        // java if (res_rotation != 0)
        if res_rotation != 0 {
          // java transform2D.putRotation(res_rotation);
          transform2D.putRotation(res_rotation);
        }
        // java if (flip[0] != 0)
        if flip[0usize] != 0 {
          // java transform2D.putFlipHorizontal(true);
          transform2D.putFlipHorizontal(true);
        }
        // java if (flip[1] != 0)
        if flip[1usize] != 0 {
          // java transform2D.putFlipVertical(true);
          transform2D.putFlipVertical(true);
        }
        // java childrensTransform2D[this.children.indexOf(child)] = transform2D;
        childrensTransform2D[(self.children.indexOf(child)) as usize] = transform2D;
        // java child.setParentGroup((KmoShowShape)null);
        child.setParentGroup(None);
        // java if (child.fillType() == 3 && this.fillType() != 3)
        if child.fillType() == 3 && self.fillType() != 3 {
          // java child.setFill(this.shapePr().fill());
          child.setFill(self.shapePr().fill());
        }
      }
      // java return this.getDgGroup().unGroup(childrensTransform2D);
      return self.getDgGroup().unGroup(childrensTransform2D);
      */
    }

    // java public void rotate(int ang)
    pub fn rotate(&mut self, ang: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (!this.getDgShape().hasTransform2D() && this.isPlaceHolder())
      if !self.getDgShape().hasTransform2D() && self.isPlaceHolder() {
        // java this.getDgShape().writeTransform2D(PlaceHolderMerger.mergeTransform(this));
        self.getDgShape().writeTransform2D(PlaceHolderMerger::mergeTransform(self));
      }
      // java this.getDgShape().rotate(ang);
      self.getDgShape().rotate(ang);
      // java if (this.isChildOfGroup())
      if self.isChildOfGroup() {
        // java this.adjustParentTransform();
        self.adjustParentTransform();
      }
      // java this.parent.getPresentation().setDirty();
      self.parent.getPresentation().setDirty();
      // java KmoShowShape[] changedShapes = this.getAdjustTransformShapes(2);
      let mut changedShapes: Vec<KmoShowShape> = self.getAdjustTransformShapes(2);
      // java this.parent.eventListeners().onShapeContentsChanged(2, changedShapes);
      self.parent.eventListeners().onShapeContentsChanged(2, changedShapes);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      */
    }

    // java private void adjustParentTransform()
    pub fn adjustParentTransform(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.parentGroup() != null)
      if self.parentGroup().is_some() {
        let mut currShape: KmoShowShape = self.parentGroup().unwrap();
        // java for (KmoShowShape currShape = this.parentGroup(); currShape != null currShape = currShape.parentGroup())
        while true {
          // java currShape.getDgGroup().adjustGroupXfrm();
          currShape.getDgGroup().adjustGroupXfrm();
        }
      }
      */
    }

    // java public void adjustModularGroupTransform()
    pub fn adjustModularGroupTransform(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.isModularGroup())
      if self.isModularGroup() {
        let mut currShape: KmoShowShape = self;
        // java for (KmoShowShape currShape = this; currShape != null currShape = currShape.parentGroup())
        while true {
          // java currShape.getDgGroup().adjustGroupXfrm();
          currShape.getDgGroup().adjustGroupXfrm();
        }
      }
      */
    }

    // java public void adjustGroupTransform()
    pub fn adjustGroupTransform(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.childCount() >= 1)
      if self.childCount() >= 1 {
        // java boolean hasGroup = false;
        let mut hasGroup: bool = false;
        // java int i = 0;
        let mut i: i32 = 0;
        let mut len: i32 = self.childCount() - 1;
        // java for (int len = this.childCount() - 1; i <= len ++i)
        while i <= len {
          // java KmoShowShape shape = this.childAt(i);
          let mut shape: KmoShowShape = self.childAt(i).unwrap();
          // java if (shape.childCount() > 0)
          if shape.childCount() > 0 {
            // java hasGroup = true;
            hasGroup = true;
            // java shape.adjustGroupTransform();
            shape.adjustGroupTransform();
          }
          // java if (i == len && shape.childCount() < 1 && !hasGroup)
          if i == len && shape.childCount() < 1 && !hasGroup {
            // java shape.adjustParentTransform();
            shape.adjustParentTransform();
          }
        }
      }
      */
    }

    // java private List<KmoShowShape> getAllChildrenShape(KmoShowShape shape)
    pub fn getAllChildrenShape(&self, shape: KmoShowShape) -> Vec<KmoShowShape> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (shape != null && shape.type() == 0)
      if true && shape.r#type() == 0 {
        // java List<KmoShowShape> children = new ArrayList(shape.childCount());
        let mut children: Vec<KmoShowShape> = { let __x: Vec<KmoShowShape> = todo!("no align strategy matched"); __x };
        // java int i = 0;
        let mut i: i32 = 0;
        let mut len: i32 = shape.childCount();
        // java for (int len = shape.childCount(); i < len ++i)
        while i < len {
          // java children.add(shape.childAt(i));
          children.add(shape.childAt(i));
          // java List<KmoShowShape> temp = shape.getAllChildrenShape(shape.childAt(i));
          let mut temp: Vec<KmoShowShape> = shape.getAllChildrenShape(shape.childAt(i).unwrap());
          // java if (temp != null && temp.size() != 0)
          if true && (temp.len() as i32) != 0 {
            // java children.addAll(temp);
            children.addAll(temp);
          }
        }
        // java return children;
        return children;
      }
      // java else
      else {
        // java return null;
        return Default::default();
      }
      */
    }

    // java private KmoShowShape[] getAdjustTransformShapes(int flag)
    pub fn getAdjustTransformShapes(&self, flag: i32) -> Vec<KmoShowShape> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java ArrayList<KmoShowShape> list = new ArrayList();
      let mut list: Vec<KmoShowShape> = { let __x: Vec<KmoShowShape> = todo!("no align strategy matched"); __x };
      // java if (this.parentGroup() == null)
      if self.parentGroup().is_none() {
        // java list.add(this);
        list.add(self);
      }
      // java else
      else {
        // java list.add(this);
        list.add(self);
        let mut currShape: KmoShowShape = self.parentGroup().unwrap();
        // java for (KmoShowShape currShape = this.parentGroup(); currShape != null currShape = currShape.parentGroup())
        while true {
          // java list.add(currShape);
          list.add(currShape);
        }
      }
      // java if (flag == 1 || flag == 2)
      if flag == 1 || flag == 2 {
        // java List<KmoShowShape> children = this.getAllChildrenShape(this);
        let mut children: Vec<KmoShowShape> = self.getAllChildrenShape(self);
        // java if (children != null && children.size() > 0)
        if let Some(children) = self.children.as_ref() {
          if (children.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) > 0 {
            // java list.addAll(children);
            list.addAll(children);
          }
        }
      }
      // java KmoShowShape[] array = new KmoShowShape[list.size()];
      let mut array: Vec<KmoShowShape> = vec![Default::default(); (list.len() as i32) as usize];
      // java return (KmoShowShape[])list.toArray(array);
      return { let __x: Vec<KmoShowShape> = todo!("no align strategy matched"); __x };
      */
    }

    // java public boolean isTextBox()
    pub fn isTextBox(&self) -> bool {
      // java return this.getDgShape().isTextBox();
      super::kmo_show_shape_hosts::convert_attach(self.dgShapeId).is_text_box
    }

    // java public boolean isNoTextEdit()
    pub fn isNoTextEdit(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.getDgShape().isNoTextEdit();
      return self.getDgShape().isNoTextEdit();
      */
    }

    // java public void layerOp(int opType)
    pub fn layerOp(&mut self, opType: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int newIndex = ((ShowShapeAtomData)this.rawData()).parentGroup.getDgGroup().layerOp(opType, this.id());
      let mut newIndex: i32 = (self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().parentGroup.getDgGroup().layerOp(opType, self.id());
      // java if (newIndex != -1)
      if newIndex != -1 {
        // java KmoShowShape pSp = ((ShowShapeAtomData)this.rawData()).parentGroup;
        let mut pSp: KmoShowShape = (self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().parentGroup;
        // java int childIndex = pSp.indexOfChild(this);
        let mut childIndex: i32 = pSp.indexOfChild(self);
        // java pSp.children.remove(childIndex);
        pSp.children.remove(childIndex);
        // java pSp.children.add(newIndex, this);
        pSp.children.add(newIndex, self);
        // java this.parent.getPresentation().setDirty();
        self.parent.getPresentation().setDirty();
        // java this.parent.eventListeners().onShapeContentsChanged(4, this);
        self.parent.eventListeners().onShapeContentsChanged(4, self);
        // java this.parent.eventListeners().onSlideContentsChanged();
        self.parent.eventListeners().onSlideContentsChanged();
      }
      */
    }

    // java public void clearTextContents()
    pub fn clearTextContents(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.type() == 1)
      if self.r#type() == 1 {
        // java this.getDgShape().clearTextContents();
        self.getDgShape().clearTextContents();
      }
      */
    }

    // java private void rerouteConnectors(int offsetX, int offsetY)
    pub fn rerouteConnectors(&mut self, offsetX: i32, offsetY: i32) {
    todo!("DEFERRED_NON_FUNCTION");
    /* DEFERRED_NON_FUNCTION

    */
    }

    // java public int borderColor()
    pub fn borderColor(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java DgShape dgShape = this.getDgShape();
      let mut dgShape: DgShape = self.getDgShape().unwrap();
      // java DgOutlineFormat outlineFormat = dgShape.outline();
      let mut outlineFormat: DgOutlineFormat = dgShape.outline();
      // java if (!outlineFormat.fillFormat().hasSolidColor())
      if !outlineFormat.fillFormat().hasSolidColor() {
        // java if (dgShape.hasStyle())
        if dgShape.hasStyle() {
          // java DgColorFormat colorFormat = dgShape.shapeStyle().outlineColor();
          let mut colorFormat: DgColorFormat = dgShape.shapeStyle().outlineColor();
          // java return this.getColor(colorFormat);
          return self.getColor__dgcolorformat(colorFormat);
        }
        // java Outline outline = PlaceHolderMerger.mergeShapePr(this).ln();
        let mut outline: Outline = PlaceHolderMerger::mergeShapePr(self).ln();
        // java if (outline.fill().hasSolidFill())
        if outline.fill().hasSolidFill() {
          // java return this.getColor(outline.fill().solidFill(), (Color)null);
          return self.getColor__color_color(outline.fill().solidFill(), None);
        }
        // java ShapeStyle shapeStyle = PlaceHolderMerger.mergeShapeStyle(this);
        let mut shapeStyle: ShapeStyle = PlaceHolderMerger::mergeShapeStyle(self);
        // java outline = this.getOutlineInShapeStyle(shapeStyle);
        outline = self.getOutlineInShapeStyle(shapeStyle);
        // java if (outline != null && outline.fill().hasSolidFill())
        if true && outline.fill().hasSolidFill() {
          // java return this.getColor(outline.fill().solidFill(), this.getOutlinePhColor(shapeStyle));
          return self.getColor__color_color(outline.fill().solidFill(), self.getOutlinePhColor(shapeStyle));
        }
      }
      // java DgColorFormat colorFormat = new DgColorFormat();
      let mut colorFormat: DgColorFormat = DgColorFormat::new();
      // java colorFormat.update(outlineFormat.fillFormat().solidColor().save());
      colorFormat.update(outlineFormat.fillFormat().solidColor().save());
      // java return this.getColor(colorFormat);
      return self.getColor__dgcolorformat(colorFormat);
      */
    }

    // java public FillBean borderFill()
    pub fn borderFill(&self) -> Option<FillBean> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java FillBean fillBean = new FillBean();
      let mut fillBean: FillBean = FillBean::new();
      // java DgShape dgShape = this.getDgShape();
      let mut dgShape: DgShape = self.getDgShape().unwrap();
      // java if (!dgShape.hasOutline() && dgShape.hasStyle())
      if !dgShape.hasOutline() && dgShape.hasStyle() {
        // java DgColorFormat colorFormat = dgShape.shapeStyle().outlineColor();
        let mut colorFormat: DgColorFormat = dgShape.shapeStyle().outlineColor();
        // java fillBean.fillMode = 0;
        fillBean.fillMode = 0;
        // java fillBean.solidColor = this.getColor(colorFormat);
        fillBean.solidColor = self.getColor__dgcolorformat(colorFormat);
        // java return fillBean;
        return Some(fillBean);
      }
      // java else
      else {
        // java Outline outline = PlaceHolderMerger.mergeShapePr(this).ln();
        let mut outline: Outline = PlaceHolderMerger::mergeShapePr(self).ln();
        // java if (outline != null)
        if true {
          // java switch (outline.fill().type())
          match outline.fill().r#type() {
            // java case 0:
            0 => {
              // java fillBean.fillMode = 0;
              fillBean.fillMode = 0;
              // java fillBean.solidColor = this.getColor(outline.fill().solidFill(), (Color)null);
              fillBean.solidColor = self.getColor__color_color(outline.fill().solidFill(), None);
              // java return fillBean;
              return Some(fillBean);
            }
            // java case 1:
            1 => {
              // java fillBean.fillMode = 1;
              fillBean.fillMode = 1;
              // java fillBean.pattenFill = outline.fill().pattFill();
              fillBean.pattenFill = outline.fill().pattFill();
              // java return fillBean;
              return Some(fillBean);
            }
            // java case 2:
            2 => {
              // java fillBean.fillMode = 2;
              fillBean.fillMode = 2;
              // java fillBean.blipFill = outline.fill().blipFill();
              fillBean.blipFill = outline.fill().blipFill();
              // java return fillBean;
              return Some(fillBean);
            }
            // java case 3:
            3 => {
              // java fillBean.fillMode = 3;
              fillBean.fillMode = 3;
              // java return fillBean;
              return Some(fillBean);
            }
            // java case 4:
            4 => {
              // java fillBean.fillMode = 4;
              fillBean.fillMode = 4;
              // java fillBean.gradientFill = outline.fill().gradFill();
              fillBean.gradientFill = outline.fill().gradFill();
              // java return fillBean;
              return Some(fillBean);
            }
            // java case 5:
            5 => {
              // java fillBean.fillMode = 5;
              fillBean.fillMode = 5;
              // java return fillBean;
              return Some(fillBean);
            }
            // java default:
            _ => {
            }
          }
        }
        // java ShapeStyle shapeStyle = PlaceHolderMerger.mergeShapeStyle(this);
        let mut shapeStyle: ShapeStyle = PlaceHolderMerger::mergeShapeStyle(self);
        // java outline = this.getOutlineInShapeStyle(shapeStyle);
        outline = self.getOutlineInShapeStyle(shapeStyle);
        // java if (outline != null && outline.fill().hasSolidFill())
        if true && outline.fill().hasSolidFill() {
          // java fillBean.fillMode = 0;
          fillBean.fillMode = 0;
          // java fillBean.solidColor = this.getColor(outline.fill().solidFill(), this.getOutlinePhColor(shapeStyle));
          fillBean.solidColor = self.getColor__color_color(outline.fill().solidFill(), self.getOutlinePhColor(shapeStyle));
          // java return fillBean;
          return Some(fillBean);
        }
        // java else
        else {
          // java return fillBean;
          return Some(fillBean);
        }
      }
      */
    }

    // java public void setBorderColor(int rgb)
    pub fn setBorderColor(&mut self, rgb: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java DgShape dgShape = this.getDgShape();
      let mut dgShape: DgShape = self.getDgShape().unwrap();
      // java dgShape.setBorderType(0);
      dgShape.setBorderType(0);
      // java dgShape.setBorderSolidFillColor(DgColor.rgb(rgb));
      dgShape.setBorderSolidFillColor(DgColor::rgb__int(rgb));
      // java this.parent.getPresentation().setDirty();
      self.parent.getPresentation().setDirty();
      // java this.parent.eventListeners().onShapeContentsChanged(3, this);
      self.parent.eventListeners().onShapeContentsChanged(3, self);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      */
    }

    // java public void setBorderGradientFillColor(FillBean fillBean)
    pub fn setBorderGradientFillColor(&mut self, fillBean: FillBean) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (fillBean != null && fillBean.gradientFill != null)
      if true && fillBean.gradientFill.is_some() {
        // java Fill fill = Fill.create();
        let mut fill: Fill = Fill::create();
        // java fill.putType(4);
        fill.putType(4);
        // java fill.writeGradFill(fillBean.gradientFill);
        fill.writeGradFill(fillBean.gradientFill);
        // java fill.save();
        fill.save();
        // java this.getDgShape().setBorderGradientFill(fill);
        self.getDgShape().setBorderGradientFill(fill);
        // java this.parent.getPresentation().setDirty();
        self.parent.getPresentation().setDirty();
        // java this.parent.eventListeners().onShapeContentsChanged(3, this);
        self.parent.eventListeners().onShapeContentsChanged(3, self);
        // java this.parent.eventListeners().onSlideContentsChanged();
        self.parent.eventListeners().onSlideContentsChanged();
      }
      */
    }

    // java private int getColor(DgColorFormat colorFormat)
    pub fn getColor__dgcolorformat(&self, colorFormat: &DgColorFormat) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.getARGBColor(colorFormat) & 16777215;
      return self.getARGBColor__dgcolorformat(colorFormat) & 16777215;
      */
    }

    // java private int getColor(Color color, Color phColor)
    pub fn getColor__color_color(&self, color: &Color, phColor: &Color) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.getARGBColor(color, phColor) & 16777215;
      return self.getARGBColor__color_color(color, phColor) & 16777215;
      */
    }

    // java private int getARGBColor(DgColorFormat colorFormat)
    pub fn getARGBColor__dgcolorformat(&self, colorFormat: &mut DgColorFormat) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java ColorInterpretCtx mColorContext = new ColorInterpretCtx();
      let mut mColorContext: ColorInterpretCtx = ColorInterpretCtx::new();
      // java mColorContext.setSlide(this.parent.getPresentation().selection().activedSlide());
      mColorContext.setSlide(self.parent.getPresentation().selection().activedSlide());
      // java int argb = ColorInterpreter.getHexARGBValue(Color.open(colorFormat.data()), mColorContext);
      let mut argb: i32 = ColorInterpreter::getHexARGBValue(Color::open(colorFormat.data()), mColorContext);
      // java return argb;
      return argb;
      */
    }

    // java private int getARGBColor(Color color, Color phColor)
    pub fn getARGBColor__color_color(&self, color: &Color, phColor: &Color) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java ColorInterpretCtx mColorContext = new ColorInterpretCtx();
      let mut mColorContext: ColorInterpretCtx = ColorInterpretCtx::new();
      // java mColorContext.withPhColor(phColor);
      mColorContext.withPhColor(phColor);
      // java mColorContext.setSlide(this.parent.getPresentation().selection().activedSlide());
      mColorContext.setSlide(self.parent.getPresentation().selection().activedSlide());
      // java int argb = ColorInterpreter.getHexARGBValue(color, mColorContext);
      let mut argb: i32 = ColorInterpreter::getHexARGBValue(color, mColorContext);
      // java return argb;
      return argb;
      */
    }

    // java public double borderWidth()
    pub fn borderWidth(&self) -> f64 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java DgShape dgShape = this.getDgShape();
      let mut dgShape: DgShape = self.getDgShape().unwrap();
      // java DgOutlineFormat outlineFormat = dgShape.outline();
      let mut outlineFormat: DgOutlineFormat = dgShape.outline();
      // java if (!outlineFormat.hasLineWidth())
      if !outlineFormat.hasLineWidth() {
        // java if (dgShape.hasStyle())
        if dgShape.hasStyle() {
          // java ShapeStyle shapeStyle = ShapeStyle.open(dgShape.shapeStyle().protocol());
          let mut shapeStyle: ShapeStyle = ShapeStyle::open(dgShape.shapeStyle().protocol());
          // java Outline outline = this.getOutlineInShapeStyle(shapeStyle);
          let mut outline: Outline = self.getOutlineInShapeStyle(shapeStyle).unwrap();
          // java if (outline != null && outline.hasWidth())
          if true && outline.hasWidth() {
            // java return UnitsConverter.emu2pt((double)outline.width());
            return UnitsConverter::emu2pt__double(((outline.width()) as f64));
          }
        }
        // java Outline outline = PlaceHolderMerger.mergeShapePr(this).ln();
        let mut outline: Outline = PlaceHolderMerger::mergeShapePr(self).ln();
        // java if (outline.hasWidth())
        if outline.hasWidth() {
          // java return UnitsConverter.emu2pt((double)outline.width());
          return UnitsConverter::emu2pt__double(((outline.width()) as f64));
        }
        // java ShapeStyle shapeStyle = PlaceHolderMerger.mergeShapeStyle(this);
        let mut shapeStyle: ShapeStyle = PlaceHolderMerger::mergeShapeStyle(self);
        // java outline = this.getOutlineInShapeStyle(shapeStyle);
        outline = self.getOutlineInShapeStyle(shapeStyle);
        // java if (outline != null && outline.hasWidth())
        if true && outline.hasWidth() {
          // java return UnitsConverter.emu2pt((double)outline.width());
          return UnitsConverter::emu2pt__double(((outline.width()) as f64));
        }
      }
      // java return dgShape.borderWidth();
      return dgShape.borderWidth();
      */
    }

    // java private Outline getOutlineInShapeStyle(ShapeStyle shapeStyle)
    pub fn getOutlineInShapeStyle(&self, shapeStyle: ShapeStyle) -> Option<Outline> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (shapeStyle == null)
      if false {
        // java return null;
        return None;
      }
      // java else
      else {
        // java int lnRef = shapeStyle.lnRef().idx();
        let mut lnRef: i32 = shapeStyle.lnRef().idx();
        // java KmoTheme theme = this.parent.theme();
        let mut theme: KmoTheme = self.parent.theme();
        // java Outline outlinePr = theme != null ? theme.outlineAt(lnRef) : null;
        let mut outlinePr: Outline = (if true { theme.outlineAt(lnRef) } else { Outline::default() });
        // java return outlinePr;
        return Some(outlinePr);
      }
      */
    }

    // java private Color getOutlinePhColor(ShapeStyle shapeStyle)
    pub fn getOutlinePhColor(&self, shapeStyle: ShapeStyle) -> Option<Color> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (shapeStyle == null)
      if false {
        // java return null;
        return None;
      }
      // java else
      else {
        // java return shapeStyle.lnRef().hasColor() ? shapeStyle.lnRef().color() : null;
        return Some((if shapeStyle.lnRef().hasColor() { Some(shapeStyle.lnRef().color()) } else { None }));
      }
      */
    }

    // java public void setBorderWidth(double width)
    pub fn setBorderWidth(&mut self, width: f64) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.getDgShape().setBorderWidth(width);
      self.getDgShape().setBorderWidth(width);
      // java this.parent.getPresentation().setDirty();
      self.parent.getPresentation().setDirty();
      // java this.parent.eventListeners().onShapeContentsChanged(3, this);
      self.parent.eventListeners().onShapeContentsChanged(3, self);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      */
    }

    // java public void setBorderWidthNoDirty(double width)
    pub fn setBorderWidthNoDirty(&mut self, width: f64) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.getDgShape().setBorderWidth(width);
      self.getDgShape().setBorderWidth(width);
      */
    }

    // java public void setBorderFillType(int type)
    pub fn setBorderFillType(&mut self, r#type: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java boolean isModify = this.setBorderFillTypeNoEvent(type);
      let mut isModify: bool = self.setBorderFillTypeNoEvent(r#type);
      // java if (isModify)
      if isModify {
        // java this.parent.getPresentation().setDirty();
        self.parent.getPresentation().setDirty();
        // java this.parent.eventListeners().onShapeContentsChanged(3, this);
        self.parent.eventListeners().onShapeContentsChanged(3, self);
        // java this.parent.eventListeners().onSlideContentsChanged();
        self.parent.eventListeners().onSlideContentsChanged();
      }
      */
    }

    // java private boolean setBorderFillTypeNoEvent(int type)
    pub fn setBorderFillTypeNoEvent(&mut self, r#type: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java DgShape dgShape = this.getDgShape();
      let mut dgShape: DgShape = self.getDgShape().unwrap();
      // java if (!this.canSetBorderType(type, dgShape))
      if !self.canSetBorderType(r#type, dgShape) {
        // java return false;
        return false;
      }
      // java else
      else {
        // java if (type == 5)
        if r#type == 5 {
          // java this.getDgShape().setBorderStyleType(0);
          self.getDgShape().setBorderStyleType(0);
          // java this.getDgShape().setBorderSolidFillColor(DgColor.rgb(0));
          self.getDgShape().setBorderSolidFillColor(DgColor::rgb__int(0));
        }
        // java this.getDgShape().setBorderType(type);
        self.getDgShape().setBorderType(r#type);
        // java return true;
        return true;
      }
      */
    }

    // java private boolean canSetBorderType(int newType, DgShape dgShape)
    pub fn canSetBorderType(&self, newType: i32, dgShape: DgShape) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java DgOutlineFormat outlineFormat = this.getDgShape().outline();
      let mut outlineFormat: DgOutlineFormat = self.getDgShape().outline();
      // java return !this.hasBorderFillInStyle() && outlineFormat.isEmpty() || this.borderFillType() != newType;
      return !self.hasBorderFillInStyle() && outlineFormat.is_empty() || self.borderFillType() != newType;
      */
    }

    // java public int borderStyleType()
    pub fn borderStyleType(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java DgShape dgShape = this.getDgShape();
      let mut dgShape: DgShape = self.getDgShape().unwrap();
      // java DgOutlineFormat outlineFormat = dgShape.outline();
      let mut outlineFormat: DgOutlineFormat = dgShape.outline();
      // java if (!outlineFormat.hasDashType())
      if !outlineFormat.hasDashType() {
        // java if (dgShape.hasStyle())
        if dgShape.hasStyle() {
          // java ShapeStyle shapeStyle = ShapeStyle.open(dgShape.shapeStyle().protocol());
          let mut shapeStyle: ShapeStyle = ShapeStyle::open(dgShape.shapeStyle().protocol());
          // java Outline outline = this.getOutlineInShapeStyle(shapeStyle);
          let mut outline: Outline = self.getOutlineInShapeStyle(shapeStyle).unwrap();
          // java if (outline != null && outline.hasPresetDash())
          if true && outline.hasPresetDash() {
            // java return outline.presetDash();
            return outline.presetDash();
          }
        }
        // java Outline outline = PlaceHolderMerger.mergeShapePr(this).ln();
        let mut outline: Outline = PlaceHolderMerger::mergeShapePr(self).ln();
        // java if (outline.hasPresetDash())
        if outline.hasPresetDash() {
          // java return outline.presetDash();
          return outline.presetDash();
        }
        // java ShapeStyle shapeStyle = PlaceHolderMerger.mergeShapeStyle(this);
        let mut shapeStyle: ShapeStyle = PlaceHolderMerger::mergeShapeStyle(self);
        // java outline = this.getOutlineInShapeStyle(shapeStyle);
        outline = self.getOutlineInShapeStyle(shapeStyle);
        // java if (outline != null && outline.hasPresetDash())
        if true && outline.hasPresetDash() {
          // java return outline.presetDash();
          return outline.presetDash();
        }
      }
      // java return this.getDgShape().borderStyleType();
      return self.getDgShape().borderStyleType();
      */
    }

    // java public int borderFillType()
    pub fn borderFillType(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.borderFillType(this.getDgShape());
      return self.borderFillType__dgshape(self.getDgShape());
      */
    }

    // java private int borderFillType(DgShape dgShape)
    pub fn borderFillType__dgshape(&self, dgShape: &mut DgShape) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java DgOutlineFormat outlineFormat = dgShape.outline();
      let mut outlineFormat: DgOutlineFormat = dgShape.outline();
      // java if (outlineFormat.isEmpty())
      if outlineFormat.is_empty() {
        // java if (dgShape.hasStyle())
        if dgShape.hasStyle() {
          // java ShapeStyle shapeStyle = ShapeStyle.open(dgShape.shapeStyle().protocol());
          let mut shapeStyle: ShapeStyle = ShapeStyle::open(dgShape.shapeStyle().protocol());
          // java Outline outline = this.getOutlineInShapeStyle(shapeStyle);
          let mut outline: Outline = self.getOutlineInShapeStyle(shapeStyle).unwrap();
          // java if (outline != null && outline.fill().hasType())
          if true && outline.fill().hasType() {
            // java return outline.fill().type();
            return outline.fill().r#type();
          }
        }
        // java Outline outline = PlaceHolderMerger.mergeShapePr(this).ln();
        let mut outline: Outline = PlaceHolderMerger::mergeShapePr(self).ln();
        // java if (outline.fill().hasType())
        if outline.fill().hasType() {
          // java return outline.fill().type();
          return outline.fill().r#type();
        }
        // java ShapeStyle shapeStyle = PlaceHolderMerger.mergeShapeStyle(this);
        let mut shapeStyle: ShapeStyle = PlaceHolderMerger::mergeShapeStyle(self);
        // java outline = this.getOutlineInShapeStyle(shapeStyle);
        outline = self.getOutlineInShapeStyle(shapeStyle);
        // java if (outline != null && outline.fill().hasType())
        if true && outline.fill().hasType() {
          // java return outline.fill().type();
          return outline.fill().r#type();
        }
      }
      // java return dgShape.outline().type();
      return dgShape.outline().r#type();
      */
    }

    // java private boolean hasBorderFillInStyle()
    pub fn hasBorderFillInStyle(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java DgShapeStyle shapeStyle1 = this.getDgShape().shapeStyle();
      let mut shapeStyle1: DgShapeStyle = self.getDgShape().shapeStyle().unwrap();
      // java int lnRef = shapeStyle1.outlineRefIndex();
      let mut lnRef: i32 = shapeStyle1.outlineRefIndex();
      // java KmoTheme theme = this.parent.theme();
      let mut theme: KmoTheme = self.parent.theme();
      // java Outline outlinePr = theme.outlineAt(lnRef);
      let mut outlinePr: Outline = theme.outlineAt(lnRef);
      // java return outlinePr != null ? outlinePr.fill().hasType() : false;
      return (if true { outlinePr.fill().hasType() } else { false });
      */
    }

    // java public void setBorderStyleType(int borderStyleType)
    pub fn setBorderStyleType(&mut self, borderStyleType: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.setBorderStyleTypeNoEvent(borderStyleType);
      self.setBorderStyleTypeNoEvent(borderStyleType);
      // java this.parent.getPresentation().setDirty();
      self.parent.getPresentation().setDirty();
      // java this.parent.eventListeners().onShapeContentsChanged(3, this);
      self.parent.eventListeners().onShapeContentsChanged(3, self);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      */
    }

    // java private void setBorderStyleTypeNoEvent(int borderStyleType)
    pub fn setBorderStyleTypeNoEvent(&mut self, borderStyleType: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java DgShape dgShape = this.getDgShape();
      let mut dgShape: DgShape = self.getDgShape().unwrap();
      // java DgOutlineFormat outlineFormat = dgShape.outline();
      let mut outlineFormat: DgOutlineFormat = dgShape.outline();
      // java if (this.borderFillType(dgShape) == 5)
      if self.borderFillType__dgshape(dgShape) == 5 {
        // java outlineFormat.setType(0);
        outlineFormat.setType(0);
      }
      // java dgShape.setBorderStyleType(borderStyleType);
      dgShape.setBorderStyleType(borderStyleType);
      */
    }

    // java public int fillType()
    pub fn fillType(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java DgShape dgShape = this.getDgShape();
      let mut dgShape: DgShape = self.getDgShape().unwrap();
      // java DgFillFormat fillFormat = dgShape.fill();
      let mut fillFormat: DgFillFormat = { let __x: DgFillFormat = todo!("no align strategy matched"); __x };
      // java if (fillFormat.isEmpty())
      if fillFormat.is_empty() {
        // java if (dgShape.hasStyle())
        if dgShape.hasStyle() {
          // java ShapeStyle shapeStyle = ShapeStyle.open(dgShape.shapeStyle().protocol());
          let mut shapeStyle: ShapeStyle = ShapeStyle::open(dgShape.shapeStyle().protocol());
          // java Fill fill = this.getFillInShapeStyle(shapeStyle);
          let mut fill: Fill = self.getFillInShapeStyle(shapeStyle).unwrap();
          // java if (fill != null && fill.hasType())
          if true && fill.hasType() {
            // java return fill.type();
            return fill.r#type();
          }
        }
        // java Fill fill = PlaceHolderMerger.mergeFill(this);
        let mut fill: Fill = PlaceHolderMerger::mergeFill(self);
        // java if (fill != null && fill.hasType())
        if true && fill.hasType() {
          // java return fill.type();
          return fill.r#type();
        }
        // java ShapeStyle shapeStyle = PlaceHolderMerger.mergeShapeStyle(this);
        let mut shapeStyle: ShapeStyle = PlaceHolderMerger::mergeShapeStyle(self);
        // java fill = this.getFillInShapeStyle(shapeStyle);
        fill = self.getFillInShapeStyle(shapeStyle);
        // java if (fill != null && fill.hasType())
        if true && fill.hasType() {
          // java return fill.type();
          return fill.r#type();
        }
      }
      // java return fillFormat.type();
      return fillFormat.r#type();
      */
    }

    // java public Fill getFill()
    pub fn getFill(&self) -> Option<Fill> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.isPicture())
      if self.isPicture() {
        // java return this.getDgPicture().getFill();
        return self.getDgPicture().getFill();
      }
      // java else
      else {
        // java DgShape dgShape = this.getDgShape();
        let mut dgShape: DgShape = self.getDgShape().unwrap();
        // java DgFillFormat fillFormat = dgShape.fill();
        let mut fillFormat: DgFillFormat = { let __x: DgFillFormat = todo!("no align strategy matched"); __x };
        // java if (fillFormat.isEmpty())
        if fillFormat.is_empty() {
          // java if (dgShape.hasStyle())
          if dgShape.hasStyle() {
            // java ShapeStyle shapeStyle = ShapeStyle.open(dgShape.shapeStyle().protocol());
            let mut shapeStyle: ShapeStyle = ShapeStyle::open(dgShape.shapeStyle().protocol());
            // java Fill fill = this.getFillInShapeStyle(shapeStyle);
            let mut fill: Fill = self.getFillInShapeStyle(shapeStyle).unwrap();
            // java if (fill != null && fill.hasType())
            if true && fill.hasType() {
              // java return fill;
              return Some(fill);
            }
          }
          // java Fill fill = PlaceHolderMerger.mergeFill(this);
          let mut fill: Fill = PlaceHolderMerger::mergeFill(self);
          // java if (fill != null && fill.hasType())
          if true && fill.hasType() {
            // java return fill;
            return Some(fill);
          }
          // java ShapeStyle shapeStyle = PlaceHolderMerger.mergeShapeStyle(this);
          let mut shapeStyle: ShapeStyle = PlaceHolderMerger::mergeShapeStyle(self);
          // java fill = this.getFillInShapeStyle(shapeStyle);
          fill = self.getFillInShapeStyle(shapeStyle);
          // java if (fill != null && fill.hasType())
          if true && fill.hasType() {
            // java return fill;
            return Some(fill);
          }
        }
        // java return fillFormat.getFill();
        return fillFormat.getFill();
      }
      */
    }

    // java private Fill getFillInShapeStyle(ShapeStyle shapeStyle)
    pub fn getFillInShapeStyle(&self, shapeStyle: ShapeStyle) -> Option<Fill> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (shapeStyle == null)
      if false {
        // java return null;
        return None;
      }
      // java else
      else {
        // java int fillRef = shapeStyle.fillRef().idx();
        let mut fillRef: i32 = shapeStyle.fillRef().idx();
        // java KmoTheme theme = this.parent.theme();
        let mut theme: KmoTheme = self.parent.theme();
        // java Fill fill = theme.fillAt(fillRef);
        let mut fill: Fill = theme.fillAt(fillRef);
        // java return fill;
        return Some(fill);
      }
      */
    }

    // java private Color getFillPhColor(ShapeStyle shapeStyle)
    pub fn getFillPhColor(&self, shapeStyle: ShapeStyle) -> Option<Color> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (shapeStyle == null)
      if false {
        // java return null;
        return None;
      }
      // java else
      else {
        // java return shapeStyle.fillRef().hasColor() ? shapeStyle.fillRef().color() : null;
        return Some((if shapeStyle.fillRef().hasColor() { Some(shapeStyle.fillRef().color()) } else { None }));
      }
      */
    }

    // java public void setFillType(int type)
    pub fn setFillType(&mut self, r#type: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.getDgShape().setFillType(type);
      self.getDgShape().setFillType(r#type);
      // java this.parent.getPresentation().setDirty();
      self.parent.getPresentation().setDirty();
      // java this.parent.eventListeners().onShapeContentsChanged(3, this);
      self.parent.eventListeners().onShapeContentsChanged(3, self);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      */
    }

    // java public void setFillColor(int rgb)
    pub fn setFillColor(&mut self, rgb: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.getDgShape().setFillColor(DgColor.rgb(rgb));
      self.getDgShape().setFillColor(DgColor::rgb__int(rgb));
      // java this.parent.getPresentation().setDirty();
      self.parent.getPresentation().setDirty();
      // java this.parent.eventListeners().onShapeContentsChanged(3, this);
      self.parent.eventListeners().onShapeContentsChanged(3, self);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      */
    }

    // java public void setGradientFill(FillBean fillBean)
    pub fn setGradientFill(&mut self, fillBean: FillBean) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (fillBean != null && fillBean.gradientFill != null)
      if true && fillBean.gradientFill.is_some() {
        // java Fill fill = Fill.create();
        let mut fill: Fill = Fill::create();
        // java fill.putType(4);
        fill.putType(4);
        // java fill.writeGradFill(fillBean.gradientFill);
        fill.writeGradFill(fillBean.gradientFill);
        // java fill.save();
        fill.save();
        // java this.setFill(fill);
        self.setFill(fill);
        // java this.parent.getPresentation().setDirty();
        self.parent.getPresentation().setDirty();
        // java this.parent.eventListeners().onShapeContentsChanged(3, this);
        self.parent.eventListeners().onShapeContentsChanged(3, self);
        // java this.parent.eventListeners().onSlideContentsChanged();
        self.parent.eventListeners().onSlideContentsChanged();
      }
      */
    }

    // java public void setFill(Fill fill)
    pub fn setFill(&mut self, fill: Fill) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.getDgShape().setFill(fill);
      self.getDgShape().setFill(fill);
      // java this.parent.getPresentation().setDirty();
      self.parent.getPresentation().setDirty();
      // java this.parent.eventListeners().onShapeContentsChanged(3, this);
      self.parent.eventListeners().onShapeContentsChanged(3, self);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      */
    }

    // java public boolean hasSelfFill()
    pub fn hasSelfFill(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.getDgShape().hasFill();
      return self.getDgShape().hasFill();
      */
    }

    // java public boolean hasFill()
    pub fn hasFill(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java DgShape dgShape = this.getDgShape();
      let mut dgShape: DgShape = self.getDgShape().unwrap();
      // java if (dgShape.hasFill())
      if dgShape.hasFill() {
        // java return true;
        return true;
      }
      // java else if (dgShape.hasStyle() && dgShape.shapeStyle().fillRefIndex() > 0)
      else if dgShape.hasStyle() && dgShape.shapeStyle().fillRefIndex() > 0 {
        // java return true;
        return true;
      }
      // java else
      else {
        // java Fill fill = PlaceHolderMerger.mergeFill(this);
        let mut fill: Fill = PlaceHolderMerger::mergeFill(self);
        // java if (fill != null)
        if true {
          // java return true;
          return true;
        }
        // java else
        else {
          // java ShapeStyle shapeStyle = PlaceHolderMerger.mergeShapeStyle(this);
          let mut shapeStyle: ShapeStyle = PlaceHolderMerger::mergeShapeStyle(self);
          // java fill = this.getFillInShapeStyle(shapeStyle);
          fill = self.getFillInShapeStyle(shapeStyle);
          // java return fill != null;
          return true;
        }
      }
      */
    }

    // java public boolean hasSelfOutline()
    pub fn hasSelfOutline(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.type() != 0 && this.type() != 4 ? this.getDgShape().hasOutline() : false;
      return (if self.r#type() != 0 && self.r#type() != 4 { self.getDgShape().hasOutline() } else { false });
      */
    }

    // java public boolean hasOutline()
    pub fn hasOutline(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.type() != 0 && this.type() != 4)
      if self.r#type() != 0 && self.r#type() != 4 {
        // java DgShape dgShape = this.getDgShape();
        let mut dgShape: DgShape = self.getDgShape().unwrap();
        // java if (dgShape.hasOutline())
        if dgShape.hasOutline() {
          // java return true;
          return true;
        }
        // java else if (dgShape.hasStyle() && dgShape.shapeStyle().outlineRefIndex() > 0)
        else if dgShape.hasStyle() && dgShape.shapeStyle().outlineRefIndex() > 0 {
          // java return true;
          return true;
        }
        // java else
        else {
          // java ShapePr shapePr = PlaceHolderMerger.mergeShapePr(this);
          let mut shapePr: ShapePr = PlaceHolderMerger::mergeShapePr(self);
          // java if (shapePr.hasLn())
          if shapePr.hasLn() {
            // java return true;
            return true;
          }
          // java else
          else {
            // java ShapeStyle shapeStyle = PlaceHolderMerger.mergeShapeStyle(this);
            let mut shapeStyle: ShapeStyle = PlaceHolderMerger::mergeShapeStyle(self);
            // java Outline outline = this.getOutlineInShapeStyle(shapeStyle);
            let mut outline: Outline = self.getOutlineInShapeStyle(shapeStyle).unwrap();
            // java return outline != null;
            return true;
          }
        }
      }
      // java else
      else {
        // java return false;
        return false;
      }
      */
    }

    // java public int fillColor()
    pub fn fillColor(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java DgShape dgShape = this.getDgShape();
      let mut dgShape: DgShape = self.getDgShape().unwrap();
      // java DgFillFormat fillFormat = dgShape.fill();
      let mut fillFormat: DgFillFormat = { let __x: DgFillFormat = todo!("no align strategy matched"); __x };
      // java if (fillFormat.type() == 3)
      if fillFormat.r#type() == 3 {
        // java return 0;
        return 0;
      }
      // java else if (!fillFormat.hasSolidColor() && dgShape.hasStyle())
      else if !fillFormat.hasSolidColor() && dgShape.hasStyle() {
        // java DgColorFormat colorFormat = this.getDgShape().shapeStyle().fillColor();
        let mut colorFormat: DgColorFormat = { let __x: DgColorFormat = todo!("no align strategy matched"); __x };
        // java return this.getColor(colorFormat);
        return self.getColor__dgcolorformat(colorFormat);
      }
      // java else
      else {
        // java Fill fill = PlaceHolderMerger.mergeFill(this);
        let mut fill: Fill = PlaceHolderMerger::mergeFill(self);
        // java if (fill != null && fill.hasSolidFill())
        if true && fill.hasSolidFill() {
          // java return this.getColor(fill.solidFill(), (Color)null);
          return self.getColor__color_color(fill.solidFill(), None);
        }
        // java else
        else {
          // java ShapeStyle shapeStyle = PlaceHolderMerger.mergeShapeStyle(this);
          let mut shapeStyle: ShapeStyle = PlaceHolderMerger::mergeShapeStyle(self);
          // java fill = this.getFillInShapeStyle(shapeStyle);
          fill = self.getFillInShapeStyle(shapeStyle);
          // java if (fill != null && fill.hasSolidFill())
          if true && fill.hasSolidFill() {
            // java return this.getColor(fill.solidFill(), this.getFillPhColor(shapeStyle));
            return self.getColor__color_color(fill.solidFill(), self.getFillPhColor(shapeStyle));
          }
          // java else if (fill != null && fill.type() == 5)
          else if true && fill.r#type() == 5 {
            // java return 0;
            return 0;
          }
          // java else
          else {
            // java DgColorFormat colorFormat = new DgColorFormat();
            let mut colorFormat: DgColorFormat = DgColorFormat::new();
            // java colorFormat.update(fillFormat.solidColor().save());
            colorFormat.update(fillFormat.solidColor().save());
            // java return this.getColor(colorFormat);
            return self.getColor__dgcolorformat(colorFormat);
          }
        }
      }
      */
    }

    // java public int fillARGBColor()
    pub fn fillARGBColor(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java DgShape dgShape = this.getDgShape();
      let mut dgShape: DgShape = self.getDgShape().unwrap();
      // java DgFillFormat fillFormat = dgShape.fill();
      let mut fillFormat: DgFillFormat = { let __x: DgFillFormat = todo!("no align strategy matched"); __x };
      // java if (fillFormat.type() == 3)
      if fillFormat.r#type() == 3 {
        // java return 0;
        return 0;
      }
      // java else if (!fillFormat.hasSolidColor() && dgShape.hasStyle())
      else if !fillFormat.hasSolidColor() && dgShape.hasStyle() {
        // java DgColorFormat colorFormat = this.getDgShape().shapeStyle().fillColor();
        let mut colorFormat: DgColorFormat = { let __x: DgColorFormat = todo!("no align strategy matched"); __x };
        // java return this.getARGBColor(colorFormat);
        return self.getARGBColor__dgcolorformat(colorFormat);
      }
      // java else
      else {
        // java Fill fill = PlaceHolderMerger.mergeFill(this);
        let mut fill: Fill = PlaceHolderMerger::mergeFill(self);
        // java if (fill != null && fill.hasSolidFill())
        if true && fill.hasSolidFill() {
          // java return this.getARGBColor(fill.solidFill(), (Color)null);
          return self.getARGBColor__color_color(fill.solidFill(), None);
        }
        // java else
        else {
          // java ShapeStyle shapeStyle = PlaceHolderMerger.mergeShapeStyle(this);
          let mut shapeStyle: ShapeStyle = PlaceHolderMerger::mergeShapeStyle(self);
          // java fill = this.getFillInShapeStyle(shapeStyle);
          fill = self.getFillInShapeStyle(shapeStyle);
          // java if (fill != null && fill.hasSolidFill())
          if true && fill.hasSolidFill() {
            // java return this.getARGBColor(fill.solidFill(), this.getFillPhColor(shapeStyle));
            return self.getARGBColor__color_color(fill.solidFill(), self.getFillPhColor(shapeStyle));
          }
          // java else if (fill != null && fill.type() == 5)
          else if true && fill.r#type() == 5 {
            // java return 0;
            return 0;
          }
          // java else
          else {
            // java DgColorFormat colorFormat = new DgColorFormat();
            let mut colorFormat: DgColorFormat = DgColorFormat::new();
            // java colorFormat.update(fillFormat.solidColor().save());
            colorFormat.update(fillFormat.solidColor().save());
            // java return this.getARGBColor(colorFormat);
            return self.getARGBColor__dgcolorformat(colorFormat);
          }
        }
      }
      */
    }

    // java public FillBean fill()
    pub fn fill(&self) -> Option<FillBean> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java FillBean fillBean = new FillBean();
      let mut fillBean: FillBean = FillBean::new();
      // java DgShape dgShape = this.getDgShape();
      let mut dgShape: DgShape = self.getDgShape().unwrap();
      // java if (!dgShape.hasFill() && dgShape.hasStyle())
      if !dgShape.hasFill() && dgShape.hasStyle() {
        // java DgColorFormat colorFormat = this.getDgShape().shapeStyle().fillColor();
        let mut colorFormat: DgColorFormat = { let __x: DgColorFormat = todo!("no align strategy matched"); __x };
        // java fillBean.fillMode = 0;
        fillBean.fillMode = 0;
        // java fillBean.solidColor = this.getColor(colorFormat);
        fillBean.solidColor = self.getColor__dgcolorformat(colorFormat);
        // java return fillBean;
        return Some(fillBean);
      }
      // java else
      else {
        // java Fill fill = PlaceHolderMerger.mergeFill(this);
        let mut fill: Fill = PlaceHolderMerger::mergeFill(self);
        // java if (fill != null)
        if true {
          // java switch (fill.type())
          match fill.r#type() {
            // java case 0:
            0 => {
              // java fillBean.fillMode = 0;
              fillBean.fillMode = 0;
              // java fillBean.solidColor = this.getColor(fill.solidFill(), (Color)null);
              fillBean.solidColor = self.getColor__color_color(fill.solidFill(), None);
              // java return fillBean;
              return Some(fillBean);
            }
            // java case 1:
            1 => {
              // java fillBean.fillMode = 1;
              fillBean.fillMode = 1;
              // java fillBean.pattenFill = fill.pattFill();
              fillBean.pattenFill = fill.pattFill();
              // java return fillBean;
              return Some(fillBean);
            }
            // java case 2:
            2 => {
              // java fillBean.fillMode = 2;
              fillBean.fillMode = 2;
              // java fillBean.blipFill = fill.blipFill();
              fillBean.blipFill = fill.blipFill();
              // java return fillBean;
              return Some(fillBean);
            }
            // java case 3:
            3 => {
              // java fillBean.fillMode = 3;
              fillBean.fillMode = 3;
              // java return fillBean;
              return Some(fillBean);
            }
            // java case 4:
            4 => {
              // java fillBean.fillMode = 4;
              fillBean.fillMode = 4;
              // java fillBean.gradientFill = fill.gradFill();
              fillBean.gradientFill = fill.gradFill();
              // java return fillBean;
              return Some(fillBean);
            }
            // java case 5:
            5 => {
              // java fillBean.fillMode = 5;
              fillBean.fillMode = 5;
              // java return fillBean;
              return Some(fillBean);
            }
            // java default:
            _ => {
            }
          }
        }
        // java ShapeStyle shapeStyle = PlaceHolderMerger.mergeShapeStyle(this);
        let mut shapeStyle: ShapeStyle = PlaceHolderMerger::mergeShapeStyle(self);
        // java fill = this.getFillInShapeStyle(shapeStyle);
        fill = self.getFillInShapeStyle(shapeStyle);
        // java if (fill != null && fill.hasSolidFill())
        if true && fill.hasSolidFill() {
          // java fillBean.fillMode = 0;
          fillBean.fillMode = 0;
          // java fillBean.solidColor = this.getColor(fill.solidFill(), this.getFillPhColor(shapeStyle));
          fillBean.solidColor = self.getColor__color_color(fill.solidFill(), self.getFillPhColor(shapeStyle));
          // java return fillBean;
          return Some(fillBean);
        }
        // java else if (fill != null && fill.type() == 5)
        else if true && fill.r#type() == 5 {
          // java fillBean.fillMode = 5;
          fillBean.fillMode = 5;
          // java return fillBean;
          return Some(fillBean);
        }
        // java else
        else {
          // java return fillBean;
          return Some(fillBean);
        }
      }
      */
    }

    // java public Fill parentShapeFill()
    pub fn parentShapeFill(&self) -> Option<Fill> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (!this.isChildOfGroup())
      if !self.isChildOfGroup() {
        // java return null;
        return None;
      }
      // java else
      else {
        let mut pSp: KmoShowShape = self.parentGroup().unwrap();
        // java for (KmoShowShape pSp = this.parentGroup(); null != pSp pSp = pSp.parentGroup())
        while true {
          // java if (pSp.fillType() != 3)
          if pSp.fillType() != 3 {
            // java return pSp.shapePr().fill();
            return { let __x: Option<Fill> = todo!("no align strategy matched"); __x };
          }
        }
        // java return null;
        return None;
      }
      */
    }

    // java public boolean hasStyle()
    pub fn hasStyle(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.getDgShape().hasStyle();
      return self.getDgShape().hasStyle();
      */
    }

    // java public void setQuickShapeStyle(int fillColor, int borderColor, int borderWidth, int borderType, int fontColor)
    pub fn setQuickShapeStyle(&mut self, fillColor: i32, borderColor: i32, borderWidth: i32, borderType: i32, fontColor: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java DgShape dgShape = this.getDgShape();
      let mut dgShape: DgShape = self.getDgShape().unwrap();
      // java if (borderType == -1)
      if borderType == -1 {
        // java this.setBorderFillTypeNoEvent(5);
        self.setBorderFillTypeNoEvent(5);
      }
      // java else
      else {
        // java this.setBorderStyleTypeNoEvent(borderType);
        self.setBorderStyleTypeNoEvent(borderType);
        // java dgShape.setBorderType(0);
        dgShape.setBorderType(0);
        // java dgShape.setBorderSolidFillColor(DgColor.rgb(borderColor));
        dgShape.setBorderSolidFillColor(DgColor::rgb__int(borderColor));
        // java dgShape.setBorderWidth((double)borderWidth);
        dgShape.setBorderWidth(((borderWidth) as f64));
      }
      // java dgShape.setFillColor(DgColor.rgb(fillColor));
      dgShape.setFillColor(DgColor::rgb__int(fillColor));
      // java this.getDgShape().setFontColor(fontColor);
      self.getDgShape().setFontColor(fontColor);
      // java this.parent.getPresentation().setDirty();
      self.parent.getPresentation().setDirty();
      // java this.parent.eventListeners().onShapeContentsChanged(3, this);
      self.parent.eventListeners().onShapeContentsChanged(3, self);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      */
    }

    // java private void setFontColor(int fontColor)
    pub fn setFontColor(&mut self, fontColor: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.getDgShape().setFontColor(fontColor);
      self.getDgShape().setFontColor(fontColor);
      // java this.parent.getPresentation().setDirty();
      self.parent.getPresentation().setDirty();
      // java this.parent.eventListeners().onShapeContentsChanged(3, this);
      self.parent.eventListeners().onShapeContentsChanged(3, self);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      */
    }

    // java public DgShapeStyle shapeStyle()
    pub fn shapeStyle(&self) -> Option<DgShapeStyle> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.getDgShape().shapeStyle();
      return self.getDgShape().shapeStyle();
      */
    }

    // java public void setShapeStyle(DgShapeStyle shapeStyle)
    pub fn setShapeStyle(&mut self, shapeStyle: DgShapeStyle) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.getDgShape().setShapeStyle(shapeStyle);
      self.getDgShape().setShapeStyle(shapeStyle);
      // java this.parent.getPresentation().setDirty();
      self.parent.getPresentation().setDirty();
      // java this.parent.eventListeners().onShapeContentsChanged(3, this);
      self.parent.eventListeners().onShapeContentsChanged(3, self);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      */
    }

    // java public KmoTextOperator getTextOperator()
    pub fn getTextOperator(&self) -> Option<KmoTextOperator> {
      // java switch (this.type())
      match self.r#type() {
        // java case 1:
        1 => {
          // java return new KmoTextOperator(this);
          return Some(KmoTextOperator);
        }
        // java case 4:
        4 => {
          // java GraphicObject graphic = this.graphic();
          // rust: no-stmt
          // java if (graphic != null && 1 == graphic.type())
          // rust: no-stmt
          // java return new KmoCellTextOperator(this);
          return None;
        }
        // java default:
        _ => {
          // java return null;
          return None;
        }
      }
    }

    // java public KmoTableOperator getTableOperator()
    pub fn getTableOperator(&self) -> Option<KmoTableOperator> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.type() == 4 && this.graphic().type() == 1 ? new KmoTableOperator(this) : null;
      return Some((if self.r#type() == 4 && self.graphic().r#type() == 1 { Some(KmoTableOperator::new(self)) } else { None }));
      */
    }

    // java public KmoChartOperator getChartOperator()
    pub fn getChartOperator(&self) -> Option<KmoChartOperator> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.type() == 4 && this.graphic().type() == 0 ? new KmoChartOperator(this) : null;
      return Some((if self.r#type() == 4 && self.graphic().r#type() == 0 { Some(KmoChartOperator::new(self)) } else { None }));
      */
    }

    // java public void setNoGroup(boolean type)
    pub fn setNoGroup(&mut self, r#type: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.getDgShape().setNoGroup(type);
      self.getDgShape().setNoGroup(r#type);
      */
    }

    // java public void setTextContent(String content)
    pub fn setTextContent(&mut self, content: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java String text = content.replace('\n', '\r');
      let mut text: String = content.replace('\n', '\r');
      // java this.getDgShape().setTextContent(text);
      self.getDgShape().setTextContent(text);
      */
    }

    // java public void replaceTextContentByPara(String content)
    pub fn replaceTextContentByPara(&mut self, content: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java String text = content.replace('\n', '\r');
      let mut text: String = content.replace('\n', '\r');
      // java this.getDgShape().replaceTextContentByPara(text);
      self.getDgShape().replaceTextContentByPara(text);
      */
    }

    // java public String getTextContent()
    pub fn getTextContent(&self) -> String {
      if !self.plain_text.is_empty() {
        return self.plain_text.clone();
      }
      String::new()
      /* DEFERRED_NON_FUNCTION
      // java String content = this.getDgShape().getTextContent();
      let mut content: String = self.getDgShape().getTextContent();
      // java int oldLen = content.length();
      let mut oldLen: i32 = (content.len() as i32);
      // java if (oldLen > 0 && content.charAt(oldLen - 1) == '\r')
      if oldLen > 0 && content.charAt(oldLen - 1) == '\r' {
        // java content = content.substring(0, oldLen - 1);
        content = content.substring(0, oldLen - 1);
      }
      // java return content.replace('\r', '\n');
      return content.replace('\r', '\n');
      */
    }

    // java public boolean hasTextBody()
    pub fn hasTextBody(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.getDgShape().hasTextBody();
      return self.getDgShape().hasTextBody();
      */
    }

    // java public boolean isTextEmpty()
    pub fn isTextEmpty(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.hasTextBody())
      if self.hasTextBody() {
        // java KmoTextOperator txOp = this.getTextOperator();
        let mut txOp: KmoTextOperator = self.getTextOperator().unwrap();
        // java if (null != txOp && !txOp.hasText())
        if true && !txOp.hasText() {
          // java return true;
          return true;
        }
      }
      // java return false;
      return false;
      */
    }

    // java public boolean hasTextContext()
    pub fn hasTextContext(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.getDgShape().hasTextContext();
      return self.getDgShape().hasTextContext();
      */
    }

    // java public void setName(String content)
    pub fn setName(&mut self, content: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.getDgShape().setName(content);
      self.getDgShape().setName(content);
      */
    }

    // java public String name()
    pub fn name(&self) -> String {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.getDgShape().name();
      return self.getDgShape().name();
      */
    }

    // java public void disconnect()
    pub fn disconnect(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.getDgConn().disconnect(true);
      self.getDgConn().disconnect(true);
      // java this.getDgConn().disconnect(false);
      self.getDgConn().disconnect(false);
      */
    }

    // java public boolean hasUseBgFill()
    pub fn hasUseBgFill(&self) -> bool {
      // java return ((ShowShapeAtomData)this.rawData()).shape.hasUseBgFill();
      false
    }

    // java public boolean isUseBgFill()
    pub fn isUseBgFill(&self) -> bool {
      // java return ((ShowShapeAtomData)this.rawData()).shape.isUseBgFill();
      false
    }

    // java public int anchor()
    pub fn anchor(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java KmoTextOperator textOperator = this.getTextOperator();
      let mut textOperator: KmoTextOperator = self.getTextOperator().unwrap();
      // java return null == textOperator ? 0 : textOperator.getAnchor(this);
      return (if false { 0 } else { textOperator.getAnchor(self) });
      */
    }

    // java public Boolean isAnchorCenter()
    pub fn isAnchorCenter(&self) -> Option<bool> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java KmoTextOperator textOperator = this.getTextOperator();
      let mut textOperator: KmoTextOperator = self.getTextOperator().unwrap();
      // java return null == textOperator ? false : textOperator.isAnchorCenter(this);
      return Some((if false { Some(false) } else { Some(textOperator.isAnchorCenter(self)) }));
      */
    }

    // java public void putAnchor(int anchor, boolean isAnchorCtr)
    pub fn putAnchor(&mut self, anchor: i32, isAnchorCtr: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (anchor >= 0 && 4 >= anchor)
      if anchor >= 0 && 4 >= anchor {
        // java KmoTextOperator textOperator = this.getTextOperator();
        let mut textOperator: KmoTextOperator = self.getTextOperator().unwrap();
        // java if (null != textOperator)
        if true {
          // java textOperator.putAnchor(anchor);
          textOperator.putAnchor(anchor);
          // java textOperator.putAnchorCenter(isAnchorCtr);
          textOperator.putAnchorCenter(isAnchorCtr);
          // java textOperator.save(this);
          textOperator.save(self);
        }
      }
      */
    }

    // java public int getPicId()
    pub fn getPicId(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.getDgPicture().getPicMediaId();
      return self.getDgPicture().getPicMediaId();
      */
    }

    // java public void changePicMediaId(int mediaId)
    pub fn changePicMediaId(&mut self, mediaId: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.getDgPicture().changePicture(mediaId);
      self.getDgPicture().changePicture(mediaId);
      // java this.parent.getPresentation().setDirty();
      self.parent.getPresentation().setDirty();
      // java this.parent.eventListeners().onShapeContentsChanged(3, this);
      self.parent.eventListeners().onShapeContentsChanged(3, self);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      */
    }

    // java public void changePic(String picPath, int width, int height)
    pub fn changePic__string_int_int(&mut self, picPath: &str, width: i32, height: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.changePic(picPath, width, height, (String)null);
      self.changePic__string_int_int_string(&picPath, width, height, String::new());
      */
    }

    // java public void changePic(String picPath, int width, int height, String descr)
    pub fn changePic__string_int_int_string(&mut self, picPath: &str, width: i32, height: i32, descr: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.changePic(picPath, width, height, descr, true);
      self.changePic__string_int_int_string_boolean(&picPath, width, height, descr, true);
      */
    }

    // java public void changePic(String picPath, int width, int height, String descr, boolean isFill)
    pub fn changePic__string_int_int_string_boolean(&mut self, picPath: &mut str, width: i32, height: i32, descr: &str, isFill: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java String fileExtension = picPath.substring(picPath.lastIndexOf(".") + 1);
      let mut fileExtension: String = picPath.substring(picPath.lastIndexOf(".") + 1);
      // java byte mediaType = KmoMediaUtil.getMediaType(fileExtension);
      let mut mediaType: u8 = KmoMediaUtil::getMediaType(fileExtension);
      // java int mediaId = this.parent.getPresentation().mediaLib().addMediaData((FileDataStorage)(new FileDataStorage(new File(picPath))), mediaType);
      let mut mediaId: i32 = self.parent.getPresentation().mediaLib().addMediaData((FileDataStorage::new(File::new(picPath))).downcast_ref::<FileDataStorage>().unwrap(), mediaType);
      // java if (width != 0 && height != 0)
      if width != 0 && height != 0 {
        // java Transform2D phTransform = this.transform2D();
        let mut phTransform: Transform2D = self.transform2D().unwrap();
        // java float widthScale = 0.0F;
        let mut widthScale: f32 = 0;
        // java float heightScale = 0.0F;
        let mut heightScale: f32 = 0;
        // java widthScale = (float)height / (float)width * ((float)phTransform.extents().cx() / (float)phTransform.extents().cy());
        widthScale = ((height) as f32) / ((width) as f32) * ((phTransform.extents().cx()) as f32) / ((phTransform.extents().cy()) as f32);
        // java widthScale = (1.0F - widthScale) * 0.5F;
        widthScale = 1 - widthScale * 0.5;
        // java heightScale = (float)width / (float)height * ((float)phTransform.extents().cy() / (float)phTransform.extents().cx());
        heightScale = ((width) as f32) / ((height) as f32) * ((phTransform.extents().cy()) as f32) / ((phTransform.extents().cx()) as f32);
        // java heightScale = (1.0F - heightScale) * 0.5F;
        heightScale = 1 - heightScale * 0.5;
        // java if (isFill)
        if isFill {
          // java if (heightScale <= widthScale)
          if heightScale <= widthScale {
            // java heightScale = 0.0F;
            heightScale = 0;
          }
          // java else
          else {
            // java widthScale = 0.0F;
            widthScale = 0;
          }
        }
        // java else if (heightScale <= widthScale)
        else if heightScale <= widthScale {
          // java widthScale = 0.0F;
          widthScale = 0;
        }
        // java else
        else {
          // java heightScale = 0.0F;
          heightScale = 0;
        }
        // java this.getDgPicture().changePicture(mediaId, widthScale, widthScale, heightScale, heightScale);
        self.getDgPicture().changePicture(mediaId, widthScale, widthScale, heightScale, heightScale);
        // java this.getDgShape().setDescr(descr);
        self.getDgShape().setDescr(descr);
        // java this.parent().getPresentation().setDirty();
        self.parent().getPresentation().setDirty();
        // java this.parent.eventListeners().onShapeContentsChanged(3, this);
        self.parent.eventListeners().onShapeContentsChanged(3, self);
        // java this.parent.eventListeners().onSlideContentsChanged();
        self.parent.eventListeners().onSlideContentsChanged();
      }
      // java else
      else {
        // java this.changePicMediaId(mediaId);
        self.changePicMediaId(mediaId);
      }
      */
    }

    // java public boolean clipPic(int type, RectF rectF, float rot, String descr)
    pub fn clipPic__int_rectf_float_string(&mut self, r#type: i32, rectF: Option<&mut RectF>, rot: f32, descr: &str) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (rectF == null)
      if rectF.is_none() {
        // java rectF = new RectF(0.0F, 0.0F, 1.0F, 1.0F);
        rectF = RectF::new(0, 0, 1, 1);
      }
      // java this.resizePic(rectF);
      self.resizePic(rectF);
      // java DgPicture pic = this.getDgPicture();
      let mut pic: DgPicture = self.getDgPicture().unwrap();
      // java pic.setPrstGeom(type);
      pic.setPrstGeom(r#type);
      // java pic.setSrcRect(new cn.wps.graphics.RectF(rectF.left, rectF.top, 1.0F - rectF.right, 1.0F - rectF.bottom));
      pic.setSrcRect(RectF::new(rectF.left, rectF.top, 1 - rectF.right, 1 - rectF.bottom));
      // java this.getDgShape().rotate((int)(rot * 60000.0F));
      self.getDgShape().rotate(((rot * 60000) as i32));
      // java this.getDgShape().setDescr(descr);
      self.getDgShape().setDescr(descr);
      // java this.parent().getPresentation().setDirty();
      self.parent().getPresentation().setDirty();
      // java this.parent.eventListeners().onShapeContentsChanged(3, this);
      self.parent.eventListeners().onShapeContentsChanged(3, self);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      // java return true;
      return true;
      */
    }

    // java public boolean clipModularGroupPic(int type, RectF rectF, float rot, String descr)
    pub fn clipModularGroupPic(&mut self, r#type: i32, rectF: RectF, rot: f32, descr: &str) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (rectF == null)
      if false {
        // java rectF = new RectF(0.0F, 0.0F, 1.0F, 1.0F);
        rectF = RectF::new(0, 0, 1, 1);
      }
      // java if (type != 74 && type != 3)
      if r#type != 74 && r#type != 3 {
        // java this.resizePic(rectF);
        self.resizePic(rectF);
      }
      // java else
      else {
        // java this.resizePicForSquare();
        self.resizePicForSquare();
      }
      // java DgPicture pic = this.getDgPicture();
      let mut pic: DgPicture = self.getDgPicture().unwrap();
      // java pic.setPrstGeom(type);
      pic.setPrstGeom(r#type);
      // java pic.setSrcRect(new cn.wps.graphics.RectF(rectF.left, rectF.top, 1.0F - rectF.right, 1.0F - rectF.bottom));
      pic.setSrcRect(RectF::new(rectF.left, rectF.top, 1 - rectF.right, 1 - rectF.bottom));
      // java this.getDgShape().rotate((int)(rot * 60000.0F));
      self.getDgShape().rotate(((rot * 60000) as i32));
      // java this.getDgShape().setDescr(descr);
      self.getDgShape().setDescr(descr);
      // java this.parent().getPresentation().setDirty();
      self.parent().getPresentation().setDirty();
      // java this.parent.eventListeners().onShapeContentsChanged(3, this);
      self.parent.eventListeners().onShapeContentsChanged(3, self);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      // java return true;
      return true;
      */
    }

    // java public boolean clipPic(float[] pts, boolean bSmooth, String descr)
    pub fn clipPic__floatarr_boolean_string(&mut self, pts: &Vec<f32>, bSmooth: bool, descr: &str) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java float[] ptsSmooth = bSmooth ? ShapeTool.getSmoothLine(pts) : pts;
      let mut ptsSmooth: Vec<f32> = (if bSmooth { ShapeTool::getSmoothLine(pts) } else { pts });
      // java if (ptsSmooth == null)
      if false {
        // java return false;
        return false;
      }
      // java else
      else {
        // java RectF rectF = bSmooth ? ShapeTool.getQuadsBounds(ptsSmooth) : ShapeTool.getLinesBounds(ptsSmooth);
        let mut rectF: RectF = (if bSmooth { ShapeTool::getQuadsBounds(ptsSmooth) } else { ShapeTool::getLinesBounds(ptsSmooth) });
        // java if (rectF == null)
        if false {
          // java return false;
          return false;
        }
        // java else
        else {
          // java this.resizePic(rectF);
          self.resizePic(rectF);
          // java DgPicture pic = this.getDgPicture();
          let mut pic: DgPicture = self.getDgPicture().unwrap();
          // java pic.setCustGeom(ptsSmooth, new cn.wps.graphics.RectF(rectF.left, rectF.top, rectF.right, rectF.bottom), bSmooth);
          pic.setCustGeom(ptsSmooth, RectF::new(rectF.left, rectF.top, rectF.right, rectF.bottom), bSmooth);
          // java pic.setSrcRect(new cn.wps.graphics.RectF(rectF.left, rectF.top, 1.0F - rectF.right, 1.0F - rectF.bottom));
          pic.setSrcRect(RectF::new(rectF.left, rectF.top, 1 - rectF.right, 1 - rectF.bottom));
          // java this.getDgShape().setDescr(descr);
          self.getDgShape().setDescr(descr);
          // java this.parent().getPresentation().setDirty();
          self.parent().getPresentation().setDirty();
          // java this.parent.eventListeners().onShapeContentsChanged(3, this);
          self.parent.eventListeners().onShapeContentsChanged(3, self);
          // java this.parent.eventListeners().onSlideContentsChanged();
          self.parent.eventListeners().onSlideContentsChanged();
          // java return true;
          return true;
        }
      }
      */
    }

    // java public boolean clipPic(GraphicsPath[] paths, String descr)
    pub fn clipPic__graphicspatharr_string(&mut self, paths: Option<&Vec<GraphicsPath>>, descr: &str) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (paths == null)
      if paths.is_none() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java cn.wps.graphics.RectF rectF = ShapeTool.getPathBounds(paths);
        let mut rectF: RectF = ShapeTool::getPathBounds(paths);
        // java this.resizePic(new RectF(rectF.left, rectF.top, rectF.right, rectF.bottom));
        self.resizePic(RectF::new(rectF.left, rectF.top, rectF.right, rectF.bottom));
        // java DgPicture pic = this.getDgPicture();
        let mut pic: DgPicture = self.getDgPicture().unwrap();
        // java pic.setCustGeom(paths, rectF);
        pic.setCustGeom(paths, rectF);
        // java pic.setSrcRect(new cn.wps.graphics.RectF(rectF.left, rectF.top, 1.0F - rectF.right, 1.0F - rectF.bottom));
        pic.setSrcRect(RectF::new(rectF.left, rectF.top, 1 - rectF.right, 1 - rectF.bottom));
        // java this.getDgShape().setDescr(descr);
        self.getDgShape().setDescr(descr);
        // java this.parent().getPresentation().setDirty();
        self.parent().getPresentation().setDirty();
        // java this.parent.eventListeners().onShapeContentsChanged(3, this);
        self.parent.eventListeners().onShapeContentsChanged(3, self);
        // java this.parent.eventListeners().onSlideContentsChanged();
        self.parent.eventListeners().onSlideContentsChanged();
        // java return true;
        return true;
      }
      */
    }

    // java public boolean clipResize(cn.wps.graphics.RectF rectF, String descr, boolean flipH, boolean flipV)
    pub fn clipResize(&mut self, rectF: cn_wps_graphics_RectF, descr: &str, flipH: bool, flipV: bool) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (rectF == null)
      if false {
        // java return false;
        return false;
      }
      // java else
      else {
        // java this.resizeCropPic(new cn.wps.graphics.RectF(rectF.left, rectF.top, rectF.right, rectF.bottom), flipH, flipV);
        self.resizeCropPic({ let __x: cn_wps_graphics_RectF = todo!("no align strategy matched"); __x }, flipH, flipV);
        // java DgPicture pic = this.getDgPicture();
        let mut pic: DgPicture = self.getDgPicture().unwrap();
        // java pic.setSrcRect(new cn.wps.graphics.RectF(rectF.left, rectF.top, 1.0F - rectF.right, 1.0F - rectF.bottom));
        pic.setSrcRect(RectF::new(rectF.left, rectF.top, 1 - rectF.right, 1 - rectF.bottom));
        // java this.getDgShape().setDescr(descr);
        self.getDgShape().setDescr(descr);
        // java this.parent().getPresentation().setDirty();
        self.parent().getPresentation().setDirty();
        // java this.parent.eventListeners().onShapeContentsChanged(3, this);
        self.parent.eventListeners().onShapeContentsChanged(3, self);
        // java this.parent.eventListeners().onSlideContentsChanged();
        self.parent.eventListeners().onSlideContentsChanged();
        // java return true;
        return true;
      }
      */
    }

    // java protected void resizePic(RectF rectF)
    pub fn resizePic(&mut self, rectF: RectF) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java KSAssert.assertNotNull(rectF);
      KSAssert::assertNotNull(rectF);
      // java DgPicture picture = this.getDgPicture();
      let mut picture: DgPicture = self.getDgPicture().unwrap();
      // java Transform2D phTransform = this.transform2D();
      let mut phTransform: Transform2D = self.transform2D().unwrap();
      // java float width = (float)phTransform.extents().cx();
      let mut width: f32 = ((phTransform.extents().cx()) as f32);
      // java float height = (float)phTransform.extents().cy();
      let mut height: f32 = ((phTransform.extents().cy()) as f32);
      // java float left = rectF.left;
      let mut left: f32 = rectF.left;
      // java float top = rectF.top;
      let mut top: f32 = rectF.top;
      // java float right = 1.0F - rectF.right;
      let mut right: f32 = 1 - rectF.right;
      // java float bottom = 1.0F - rectF.bottom;
      let mut bottom: f32 = 1 - rectF.bottom;
      // java cn.wps.graphics.RectF src = picture.getSrcRect();
      let mut src: RectF = picture.getSrcRect();
      // java if (src != null)
      if true {
        // java float rx = 1.0F - src.left - src.right;
        let mut rx: f32 = 1 - src.left - src.right;
        // java float ry = 1.0F - src.top - src.bottom;
        let mut ry: f32 = 1 - src.top - src.bottom;
        // java if ((double)Math.abs(rx) > 0.01 && (double)Math.abs(ry) > 0.01)
        if (((rx).abs()) as f64) > 0.01 && (((ry).abs()) as f64) > 0.01 {
          // java width /= rx;
          width = width / rx;
          // java height /= ry;
          height = height / ry;
          // java left -= src.left;
          left = left - src.left;
          // java top -= src.top;
          top = top - src.top;
          // java right -= src.right;
          right = right - src.right;
          // java bottom -= src.bottom;
          bottom = bottom - src.bottom;
        }
      }
      // java int offsetLeft = (int)(left * width);
      let mut offsetLeft: i32 = ((left * width) as i32);
      // java int offsetTop = (int)(top * height);
      let mut offsetTop: i32 = ((top * height) as i32);
      // java int offsetRight = (int)(-right * width);
      let mut offsetRight: i32 = ((-right * width) as i32);
      // java int offsetBottom = (int)(-bottom * height);
      let mut offsetBottom: i32 = ((-bottom * height) as i32);
      // java this.resize(offsetLeft, offsetTop, offsetRight, offsetBottom, true, false);
      self.resize__int_int_int_int_boolean_boolean(offsetLeft, offsetTop, offsetRight, offsetBottom, (true as i32), false);
      */
    }

    // java public void resizePicForSquare()
    pub fn resizePicForSquare(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Transform2D phTransform = this.transform2D();
      let mut phTransform: Transform2D = self.transform2D().unwrap();
      // java int width = phTransform.extents().cx();
      let mut width: i32 = phTransform.extents().cx();
      // java int height = phTransform.extents().cy();
      let mut height: i32 = phTransform.extents().cy();
      // java if (width > height)
      if width > height {
        // java int offset = (width - height) / 2;
        let mut offset: i32 = width - height / 2;
        // java this.getDgShape().putCxAndCY(height, height);
        self.getDgShape().putCxAndCY(height, height);
        // java this.resizeModularPicSelf(offset, 0, offset, 0, true, false);
        self.resizeModularPicSelf(offset, 0, offset, 0, true, false);
      }
      // java else
      else {
        // java int offset = (height - width) / 2;
        let mut offset: i32 = height - width / 2;
        // java this.getDgShape().putCxAndCY(width, width);
        self.getDgShape().putCxAndCY(width, width);
        // java this.resizeModularPicSelf(0, offset, 0, offset, true, false);
        self.resizeModularPicSelf(0, offset, 0, offset, true, false);
      }
      */
    }

    // java protected void resizeCropPic(cn.wps.graphics.RectF rectF, boolean flipH, boolean flipV)
    pub fn resizeCropPic(&mut self, rectF: cn_wps_graphics_RectF, flipH: bool, flipV: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java KSAssert.assertNotNull(rectF);
      KSAssert::assertNotNull(rectF);
      // java DgPicture picture = this.getDgPicture();
      let mut picture: DgPicture = self.getDgPicture().unwrap();
      // java Transform2D phTransform = this.transform2D();
      let mut phTransform: Transform2D = self.transform2D().unwrap();
      // java float width = (float)phTransform.extents().cx();
      let mut width: f32 = ((phTransform.extents().cx()) as f32);
      // java float height = (float)phTransform.extents().cy();
      let mut height: f32 = ((phTransform.extents().cy()) as f32);
      // java float left = rectF.left;
      let mut left: f32 = rectF.left;
      // java float top = rectF.top;
      let mut top: f32 = rectF.top;
      // java float right = 1.0F - rectF.right;
      let mut right: f32 = 1 - rectF.right;
      // java float bottom = 1.0F - rectF.bottom;
      let mut bottom: f32 = 1 - rectF.bottom;
      // java cn.wps.graphics.RectF src = picture.getSrcRect();
      let mut src: RectF = picture.getSrcRect();
      // java if (src != null)
      if true {
        // java float rx = 1.0F - src.left - src.right;
        let mut rx: f32 = 1 - src.left - src.right;
        // java float ry = 1.0F - src.top - src.bottom;
        let mut ry: f32 = 1 - src.top - src.bottom;
        // java if ((double)Math.abs(rx) > 0.01 && (double)Math.abs(ry) > 0.01)
        if (((rx).abs()) as f64) > 0.01 && (((ry).abs()) as f64) > 0.01 {
          // java width /= rx;
          width = width / rx;
          // java height /= ry;
          height = height / ry;
          // java left -= src.left;
          left = left - src.left;
          // java top -= src.top;
          top = top - src.top;
          // java right -= src.right;
          right = right - src.right;
          // java bottom -= src.bottom;
          bottom = bottom - src.bottom;
        }
      }
      // java float temp = left;
      let mut temp: f32 = left;
      // java if (flipH)
      if flipH {
        // java left = right;
        left = right;
        // java right = temp;
        right = temp;
      }
      // java temp = top;
      temp = top;
      // java if (flipV)
      if flipV {
        // java top = bottom;
        top = bottom;
        // java bottom = temp;
        bottom = temp;
      }
      // java int offsetLeft = (int)(left * width);
      let mut offsetLeft: i32 = ((left * width) as i32);
      // java int offsetTop = (int)(top * height);
      let mut offsetTop: i32 = ((top * height) as i32);
      // java int offsetRight = (int)(-right * width);
      let mut offsetRight: i32 = ((-right * width) as i32);
      // java int offsetBottom = (int)(-bottom * height);
      let mut offsetBottom: i32 = ((-bottom * height) as i32);
      // java this.resizeCrop(offsetLeft, offsetTop, offsetRight, offsetBottom, true);
      self.resizeCrop(offsetLeft, offsetTop, offsetRight, offsetBottom, true);
      */
    }

    // java public int getClipType()
    pub fn getClipType(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.shapePr().hasGeom())
      if self.shapePr().hasGeom() {
        // java Geometry2D geom = this.shapePr().geom();
        let mut geom: Geometry2D = self.shapePr().geom();
        // java if (geom != null && geom.type() == 0 && geom.hasPrstGeom())
        if true && geom.r#type() == 0 && geom.hasPrstGeom() {
          // java return geom.prstGeom().prst();
          return geom.prstGeom().prst();
        }
      }
      // java return 0;
      return 0;
      */
    }

    // java public boolean isPresetClip()
    pub fn isPresetClip(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.getDgPicture().getClipType() == 0;
      return self.getDgPicture().getClipType() == 0;
      */
    }

    // java public OleObject getOle()
    pub fn getOle(&self) -> Option<OleObject> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java GraphicObject graphic = this.graphic();
      let mut graphic: Box<dyn GraphicObject> = { let __x: Box<Box<dyn GraphicObject>> = todo!("no align strategy matched"); __x };
      // java if (graphic != null)
      if true {
        // java int type = graphic.type();
        let mut r#type: i32 = graphic.r#type();
        // java if (type == 3 && graphic instanceof KmoOleObject)
        if r#type == 3 && graphic.downcast_ref::<KmoOleObject>().is_some() {
          // java KmoOleObject oleObject = (KmoOleObject)graphic;
          let mut oleObject: KmoOleObject = (graphic).downcast_ref::<KmoOleObject>().unwrap();
          // java return OleObject.open(oleObject.protocol());
          return Some(OleObject::open(oleObject.protocol()));
        }
      }
      // java return null;
      return None;
      */
    }

    // java protected OleObject getPenkitOle()
    pub fn getPenkitOle(&self) -> Option<OleObject> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.getPenkitOleForSpecBrand((String)null);
      return Some(self.getPenkitOleForSpecBrand(Default::default()));
      */
    }

    // java protected OleObject getPenkitOleForSpecBrand(String brand)
    pub fn getPenkitOleForSpecBrand(&self, brand: &str) -> OleObject {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java OleObject ole = this.getOle();
      let mut ole: OleObject = self.getOle().unwrap();
      // java if (ole != null)
      if true {
        // java boolean isHuaweiOrHonorBrand = "huawei".equalsIgnoreCase(brand) || "honor".equalsIgnoreCase(brand);
        let mut isHuaweiOrHonorBrand: bool = "huawei".equalsIgnoreCase(brand) || "honor".equalsIgnoreCase(brand);
        // java if (ole.hasOle() && ole.ole().hasName())
        if ole.hasOle() && ole.ole().hasName() {
          // java String name = ole.ole().name();
          let mut name: String = ole.ole().name();
          // java if (brand == null && name.toLowerCase().endsWith("penkit"))
          if false && name.toLowerCase().endsWith("penkit") {
            // java return ole;
            return ole;
          }
          // java if (isHuaweiOrHonorBrand && (name.equalsIgnoreCase("penkit") || name.equalsIgnoreCase(ShapeTool.getPenKitName("huawei")) || name.equalsIgnoreCase(ShapeTool.getPenKitName("honor"))))
          if isHuaweiOrHonorBrand && name.equalsIgnoreCase("penkit") || name.equalsIgnoreCase(ShapeTool::getPenKitName("huawei")) || name.equalsIgnoreCase(ShapeTool::getPenKitName("honor")) {
            // java return ole;
            return ole;
          }
        }
        // java if (ole.hasProgID())
        if ole.hasProgID() {
          // java String progID = ole.progID();
          let mut progID: String = ole.progID();
          // java if (progID != null && progID.equalsIgnoreCase("Package"))
          if true && progID.equalsIgnoreCase("Package") {
            // java String oleFilename = ole.ole().name();
            let mut oleFilename: String = ole.ole().name();
            // java String oleType = (String)this.shapeTagMap().get("OLE_FILE_TYPE");
            let mut oleType: String = { let __x: String = todo!("no align strategy matched"); __x };
            // java String tagFilename = (String)this.shapeTagMap().get("OLE_FILE_NAME");
            let mut tagFilename: String = { let __x: String = todo!("no align strategy matched"); __x };
            // java boolean has3DTag = "3dobj".equals(oleType);
            let mut has3DTag: bool = "3dobj".equals(oleType);
            // java boolean has3DName = oleFilename != null && "3dobj".equals(oleFilename.toLowerCase());
            let mut has3DName: bool = true && "3dobj".equals(oleFilename.toLowerCase());
            // java boolean isTagFilename3D = false;
            let mut isTagFilename3D: bool = false;
            // java if (tagFilename != null)
            if true {
              // java isTagFilename3D = tagFilename.toLowerCase().endsWith("glb") || tagFilename.toLowerCase().endsWith("rsdz");
              isTagFilename3D = tagFilename.toLowerCase().endsWith("glb") || tagFilename.toLowerCase().endsWith("rsdz");
            }
            // java if (has3DTag || has3DName || isTagFilename3D)
            if has3DTag || has3DName || isTagFilename3D {
              // java return null;
              return Default::default();
            }
            // java String name = this.getPenKitFileName(ole);
            let mut name: String = self.getPenKitFileName(ole);
            // java if (name == null)
            if false {
              // java return null;
              return Default::default();
            }
            // java if (brand == null && name.toLowerCase().endsWith("penkit"))
            if false && name.toLowerCase().endsWith("penkit") {
              // java return ole;
              return ole;
            }
            // java if (isHuaweiOrHonorBrand && ShapeTool.isHuaweiOrHonorPenkitData(name))
            if isHuaweiOrHonorBrand && ShapeTool::isHuaweiOrHonorPenkitData(name) {
              // java return ole;
              return ole;
            }
          }
        }
      }
      // java return null;
      return Default::default();
      */
    }

    // java public String getPenKitFileName(OleObject ole)
    pub fn getPenKitFileName(&self, ole: OleObject) -> String {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java KmoOleManager oleManager = this.parent().getPresentation().getOleManager();
      let mut oleManager: KmoOleManager = self.parent().getPresentation().getOleManager();
      // java String binPath = oleManager.getOriginFile(ole.ole().mediaId());
      let mut binPath: String = oleManager.getOriginFile(ole.ole().mediaId());
      // java if (binPath != null)
      if true {
        // java new PackageStreamParser(oleManager.innerCache());
        PackageStreamParser::new(oleManager.innerCache());
        // java String name = PackageStreamParser.getFileName(binPath);
        let mut name: String = PackageStreamParser::getFileName(binPath);
        // java return name;
        return name;
      }
      // java else
      else {
        // java return null;
        return Default::default();
      }
      */
    }

    // java public boolean isPenKit()
    pub fn isPenKit(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.isPenKitForSpecBrand((String)null);
      return self.isPenKitForSpecBrand(Default::default());
      */
    }

    // java public boolean isPenKitForSpecBrand(String brand)
    pub fn isPenKitForSpecBrand(&self, brand: &str) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.getPenkitOleForSpecBrand(brand) != null;
      return self.getPenkitOleForSpecBrand(brand).is_some();
      */
    }

    // java public String getOleFile(String oleName)
    pub fn getOleFile(&self, oleName: &str) -> String {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java OleObject ole = this.getOle();
      let mut ole: OleObject = self.getOle().unwrap();
      // java if (ole != null && ole.hasOle() && ole.hasProgID())
      if true && ole.hasOle() && ole.hasProgID() {
        // java String name = ole.ole().hasName() ? ole.ole().name() : "";
        let mut name: String = (if ole.ole().hasName() { ole.ole().name() } else { "".to_string() });
        // java String progID = ole.progID();
        let mut progID: String = ole.progID();
        // java if (!name.equalsIgnoreCase(oleName) && progID == null)
        if !name.equalsIgnoreCase(oleName) && false {
          // java return null;
          return Default::default();
        }
        // java else
        else {
          // java KmoOleManager oleManager = this.parent().getPresentation().getOleManager();
          let mut oleManager: KmoOleManager = self.parent().getPresentation().getOleManager();
          // java String binPath = oleManager.getOriginFile(ole.ole().mediaId());
          let mut binPath: String = oleManager.getOriginFile(ole.ole().mediaId());
          // java if (binPath == null)
          if false {
            // java return null;
            return Default::default();
          }
          // java else
          else {
            // java String oriPath = (new PackageStreamParser(oleManager.innerCache())).getRawFilePath(binPath);
            let mut oriPath: String = PackageStreamParser::new(oleManager.innerCache()).getRawFilePath(binPath);
            // java return oriPath == null || !name.equals(oleName) && !oriPath.toLowerCase().endsWith(oleName) ? null : oriPath;
            return (if false || !name.equals(oleName) && !oriPath.toLowerCase().endsWith(oleName) { String::new() } else { oriPath });
          }
        }
      }
      // java else
      else {
        // java return null;
        return Default::default();
      }
      */
    }

    // java public boolean isPicture()
    pub fn isPicture(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.type() == 2 && !this.hasInk() && !this.hasAudio() && !this.hasVideo();
      return self.r#type() == 2 && !self.hasInk() && !self.hasAudio() && !self.hasVideo();
      */
    }

    // java public String getPenKit()
    pub fn getPenKit(&self) -> String {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java OleObject ole = this.getPenkitOle();
      let mut ole: OleObject = self.getPenkitOle().unwrap();
      // java if (ole != null && ole.ole().hasMediaId())
      if true && ole.ole().hasMediaId() {
        // java KmoOleManager oleManager = this.parent().getPresentation().getOleManager();
        let mut oleManager: KmoOleManager = self.parent().getPresentation().getOleManager();
        // java String binPath = oleManager.getOriginFile(ole.ole().mediaId());
        let mut binPath: String = oleManager.getOriginFile(ole.ole().mediaId());
        // java if (binPath != null)
        if true {
          // java return (new PackageStreamParser(oleManager.innerCache())).getRawFilePath(binPath);
          return PackageStreamParser::new(oleManager.innerCache()).getRawFilePath(binPath);
        }
      }
      // java return null;
      return Default::default();
      */
    }

    // java public boolean isEmptyPenkit()
    pub fn isEmptyPenkit(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int id = -1;
      let mut id: i32 = -1;
      // java KmoPresentation pres = this.parent().getPresentation();
      let mut pres: KmoPresentation = self.parent().getPresentation();
      // java Shape shape = Shape.open(pres.drawingStore().query(this.dgShapeId));
      let mut shape: Shape = Shape::open(pres.drawingStore().query(self.dgShapeId));
      // java if (shape != null)
      if true {
        // java GraphicObject obj = pres.drawingStore().graphicStore().query(shape.graphic().id());
        let mut obj: Box<dyn GraphicObject> = pres.drawingStore().graphicStore().query(shape.graphic().id());
        // java if (obj != null && obj instanceof KmoOleObject)
        if true && obj.downcast_ref::<KmoOleObject>().is_some() {
          // java KmoOleObject ole = (KmoOleObject)obj;
          let mut ole: KmoOleObject = (obj).downcast_ref::<KmoOleObject>().unwrap();
          // java KmoShowShape pic = ole.picture();
          let mut pic: KmoShowShape = ole.picture();
          // java if (pic != null)
          if true {
            // java Shape picShape = Shape.open(pres.drawingStore().query(pic.id()));
            let mut picShape: Shape = Shape::open(pres.drawingStore().query(pic.id()));
            // java if (picShape != null)
            if true {
              // java Blip blip = picShape.blipFill().blipFill().blip();
              let mut blip: Blip = picShape.blipFill().blipFill().blip();
              // java if (blip != null)
              if true {
                // java if (blip.hasLinkMediaID())
                if blip.hasLinkMediaID() {
                  // java id = blip.linkMediaID();
                  id = blip.linkMediaID();
                }
                // java else if (blip.hasEmbedMediaID())
                else if blip.hasEmbedMediaID() {
                  // java id = blip.embedMediaID();
                  id = blip.embedMediaID();
                }
              }
            }
          }
        }
      }
      // java if (id >= 0)
      if id >= 0 {
        // java String path = pres.mediaLib().getMediaPath(id);
        let mut path: String = pres.mediaLib().getMediaPath(id);
        // java if (path != null)
        if true {
          // java PictureInfo info = PictureParser.getPictureInfo(path);
          let mut info: PictureInfo = PictureParser::getPictureInfo__string(path);
          // java if (info != null && info.width == 1 && info.height == 1)
          if true && info.width == 1 && info.height == 1 {
            // java return true;
            return true;
          }
        }
      }
      // java return false;
      return false;
      */
    }

    // java public void editPenKit(String penkitPath, String picPath, String descr, String oleName)
    pub fn editPenKit(&mut self, penkitPath: &str, picPath: &str, descr: &str, oleName: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java ShapeTool.addOleObject(this.parent, penkitPath, picPath, this.dgShapeId, oleName, "Package", false);
      ShapeTool::addOleObject(self.parent, penkitPath, picPath, self.dgShapeId, oleName, "Package", false);
      // java if (this.isChildOfGroup())
      if self.isChildOfGroup() {
        // java this.adjustParentTransform();
        self.adjustParentTransform();
      }
      // java this.parent().getPresentation().setDirty();
      self.parent().getPresentation().setDirty();
      // java this.parent.eventListeners().onShapeContentsChanged(3, this);
      self.parent.eventListeners().onShapeContentsChanged(3, self);
      // java KmoShowShape[] changedShapes = this.getAdjustTransformShapes(1);
      let mut changedShapes: Vec<KmoShowShape> = self.getAdjustTransformShapes(1);
      // java this.parent.eventListeners().onShapeContentsChanged(1, changedShapes);
      self.parent.eventListeners().onShapeContentsChanged(1, changedShapes);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      */
    }

    // java public void changeOleBlipFill(String picPath, int width, int height)
    pub fn changeOleBlipFill(&mut self, picPath: &str, width: i32, height: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.isOle() && picPath != null && !picPath.isEmpty())
      if self.isOle() && true && !picPath.is_empty() {
        // java KmoShowShape picShape = ((KmoOleObject)this.graphic()).picture();
        let mut picShape: KmoShowShape = (self.graphic()).downcast_ref::<KmoOleObject>().unwrap().picture();
        // java if (picShape != null)
        if true {
          // java picShape.changePic(picPath, width, height);
          picShape.changePic(picPath, width, height);
          // java this.parent.eventListeners().onShapeContentsChanged(3, this);
          self.parent.eventListeners().onShapeContentsChanged(3, self);
          // java this.parent.eventListeners().onSlideContentsChanged();
          self.parent.eventListeners().onSlideContentsChanged();
        }
      }
      */
    }

    // java public void setFontSizeWidthScale(float scale, boolean sendEvent)
    pub fn setFontSizeWidthScale(&mut self, scale: f32, sendEvent: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java boolean dirty = false;
      let mut dirty: bool = false;
      // java if (this.hasTextBody())
      if self.hasTextBody() {
        // java KmoFont kmoFont = new KmoFont(this, new KmoContext(this.parent()));
        let mut kmoFont: KmoFont = KmoFont::new(self, KmoContext::new(self.parent()));
        // java kmoFont.putSizeWithScale(scale);
        kmoFont.putSizeWithScale(scale);
        // java dirty = true;
        dirty = true;
      }
      // java else if (this.type() == 0)
      else if self.r#type() == 0 {
        // java this.setFontSizeGroupScale(this, scale);
        self.setFontSizeGroupScale(self, scale);
        // java dirty = true;
        dirty = true;
      }
      // java else if (this.isTable())
      else if self.isTable() {
        // java KmoFont kmoFont = new KmoFont(this, new KmoContext(this.parent()));
        let mut kmoFont: KmoFont = KmoFont::new(self, KmoContext::new(self.parent()));
        // java kmoFont.putSizeWithScale(scale);
        kmoFont.putSizeWithScale(scale);
        // java dirty = true;
        dirty = true;
      }
      // java if (dirty)
      if dirty {
        // java this.parent().getPresentation().setDirty();
        self.parent().getPresentation().setDirty();
      }
      // java if (sendEvent && dirty)
      if sendEvent && dirty {
        // java this.parent.eventListeners().onShapeContentsChanged(3, this);
        self.parent.eventListeners().onShapeContentsChanged(3, self);
        // java this.parent.eventListeners().onSlideContentsChanged();
        self.parent.eventListeners().onSlideContentsChanged();
      }
      */
    }

    // java private void setFontSizeGroupScale(KmoShowShape showShape, float scale)
    pub fn setFontSizeGroupScale(&mut self, showShape: KmoShowShape, scale: f32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int i = 0;
      let mut i: i32 = 0;
      let mut size: i32 = showShape.childCount();
      // java for (int size = showShape.childCount(); i < size ++i)
      while i < size {
        // java KmoShowShape item = showShape.childAt(i);
        let mut item: KmoShowShape = showShape.childAt(i).unwrap();
        // java if (item.type() == 0)
        if item.r#type() == 0 {
          // java this.setFontSizeGroupScale(item, scale);
          self.setFontSizeGroupScale(item, scale);
        }
        // java if (item.hasTextBody())
        if item.hasTextBody() {
          // java KmoFont kmoFont = new KmoFont(item, new KmoContext(item.parent()));
          let mut kmoFont: KmoFont = KmoFont::new(item, KmoContext::new(item.parent()));
          // java kmoFont.putSizeWithScale(scale);
          kmoFont.putSizeWithScale(scale);
        }
      }
      */
    }

    // java public boolean setFontName(String name, boolean sendEvent)
    pub fn setFontName(&mut self, name: &str, sendEvent: bool) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java boolean dirty = false;
      let mut dirty: bool = false;
      // java if (this.hasTextBody())
      if self.hasTextBody() {
        // java KmoTextRange textRange = new KmoTextRange(this);
        let mut textRange: KmoTextRange = KmoTextRange::new(self);
        // java textRange.putStart(0);
        textRange.putStart(0);
        // java textRange.putEnd(Integer.MAX_VALUE);
        textRange.putEnd(i32::MAX);
        // java textRange.font().putNameAscii(name);
        textRange.font().putNameAscii(name);
        // java dirty = true;
        dirty = true;
      }
      // java else if (this.type() == 0)
      else if self.r#type() == 0 {
        // java if (this.setFontNameGroupScale(this, name))
        if self.setFontNameGroupScale(self, name) {
          // java dirty = true;
          dirty = true;
        }
      }
      // java else if (this.isTable())
      else if self.isTable() {
      }
      // java if (dirty)
      if dirty {
        // java this.parent().getPresentation().setDirty();
        self.parent().getPresentation().setDirty();
      }
      // java if (sendEvent && dirty)
      if sendEvent && dirty {
        // java this.parent.eventListeners().onShapeContentsChanged(3, this);
        self.parent.eventListeners().onShapeContentsChanged(3, self);
        // java this.parent.eventListeners().onSlideContentsChanged();
        self.parent.eventListeners().onSlideContentsChanged();
      }
      // java return dirty;
      return dirty;
      */
    }

    // java private boolean setFontNameGroupScale(KmoShowShape showShape, String name)
    pub fn setFontNameGroupScale(&mut self, showShape: KmoShowShape, name: &str) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java boolean dirty = false;
      let mut dirty: bool = false;
      // java int i = 0;
      let mut i: i32 = 0;
      let mut size: i32 = showShape.childCount();
      // java for (int size = showShape.childCount(); i < size ++i)
      while i < size {
        // java KmoShowShape item = showShape.childAt(i);
        let mut item: KmoShowShape = showShape.childAt(i).unwrap();
        // java if (item.type() == 0)
        if item.r#type() == 0 {
          // java dirty = this.setFontNameGroupScale(item, name);
          dirty = self.setFontNameGroupScale(item, name);
        }
        // java else if (item.hasTextBody())
        else if item.hasTextBody() {
          // java KmoTextRange textRange = new KmoTextRange(item);
          let mut textRange: KmoTextRange = KmoTextRange::new(item);
          // java textRange.putStart(0);
          textRange.putStart(0);
          // java textRange.putEnd(Integer.MAX_VALUE);
          textRange.putEnd(i32::MAX);
          // java textRange.font().putNameAscii(name);
          textRange.font().putNameAscii(name);
          // java dirty = true;
          dirty = true;
        }
      }
      // java return dirty;
      return dirty;
      */
    }

    // java public boolean isTable()
    pub fn isTable(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.type() == 4)
      if self.r#type() == 4 {
        // java GraphicObject graphic = this.graphic();
        let mut graphic: Box<dyn GraphicObject> = { let __x: Box<Box<dyn GraphicObject>> = todo!("no align strategy matched"); __x };
        // java if (graphic != null && graphic.type() == 1)
        if true && graphic.r#type() == 1 {
          // java return true;
          return true;
        }
      }
      // java return false;
      return false;
      */
    }

    // java public boolean isOle()
    pub fn isOle(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java GraphicObject graphic = this.graphic();
      let mut graphic: Box<dyn GraphicObject> = { let __x: Box<Box<dyn GraphicObject>> = todo!("no align strategy matched"); __x };
      // java return graphic != null && graphic.type() == 3;
      return true && graphic.r#type() == 3;
      */
    }

    // java public boolean isModel3D()
    pub fn isModel3D(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java GraphicObject graphic = this.graphic();
      let mut graphic: Box<dyn GraphicObject> = { let __x: Box<Box<dyn GraphicObject>> = todo!("no align strategy matched"); __x };
      // java return graphic != null && graphic.type() == 4;
      return true && graphic.r#type() == 4;
      */
    }

    // java public boolean isChart()
    pub fn isChart(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java GraphicObject graphic = this.graphic();
      let mut graphic: Box<dyn GraphicObject> = { let __x: Box<Box<dyn GraphicObject>> = todo!("no align strategy matched"); __x };
      // java return graphic != null && graphic.type() == 0;
      return true && graphic.r#type() == 0;
      */
    }

    // java public boolean setFontScale(float fontScale, double lnSpcReduction)
    pub fn setFontScale(&mut self, fontScale: f32, lnSpcReduction: f64) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java boolean isChangeAutoFitType = this.getDgShape().setFontScale(fontScale, lnSpcReduction);
      let mut isChangeAutoFitType: bool = self.getDgShape().setFontScale(fontScale, lnSpcReduction);
      // java if (isChangeAutoFitType)
      if isChangeAutoFitType {
        // java this.parent().eventListeners().onShapeContentsChanged(this);
        self.parent().eventListeners().onShapeContentsChanged(self);
        // java this.parent().eventListeners().onSlideContentsChanged();
        self.parent().eventListeners().onSlideContentsChanged();
      }
      // java else
      else {
        // java this.parent().getPresentation().eventListeners().onShapeFontScaleChanged();
        self.parent().getPresentation().eventListeners().onShapeFontScaleChanged();
      }
      // java return isChangeAutoFitType;
      return isChangeAutoFitType;
      */
    }

    // java public boolean hasFontSizeFit()
    pub fn hasFontSizeFit(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.getDgShape().hasFontSizeFit();
      return self.getDgShape().hasFontSizeFit();
      */
    }

    // java public void clearFontSizeFit()
    pub fn clearFontSizeFit(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.getDgShape().clearFontSizeFit();
      self.getDgShape().clearFontSizeFit();
      */
    }

    // java public boolean canRotate()
    pub fn canRotate(&self) -> bool {
      // java switch (this.type())
      match self.r#type() {
        // java case 0:
        0 => {
          // java int i = 0;
          let mut i: i32 = 0;
          let mut count: i32 = self.childCount();
          // java for (int count = this.childCount(); i < count ++i)
          while i < count {
            // java if (!this.childAt(i).canRotate())
            if !self.childAt(i).canRotate() {
              // java return false;
              return false;
            }
            i += 1;
          }
          // java return true;
          return true;
        }
        // java case 4:
        4 => {
          // java return false;
          return false;
        }
        // java default:
        _ => {
          // java return true;
          return true;
        }
      }
    }

    // java public int rotation()
    pub fn rotation(&self) -> i32 {
      // java return this.transform2D().rotation();
      self.transform2D().map(|t| t.rotation()).unwrap_or(0)
    }

    // java public boolean isHidden()
    pub fn isHidden(&self) -> bool {
      // java return this.getDgShape().isHidden();
      super::kmo_show_shape_hosts::convert_attach(self.dgShapeId).is_hidden
    }

    // java public Transform2D transform2D()
    pub fn transform2D(&self) -> Option<Transform2D> {
      // java return !this.getDgShape().hasTransform2D() && this.isPlaceHolder() ? PlaceHolderMerger.mergeTransform(this) : this.getDgShape().transform2D();
      let att = super::kmo_show_shape_hosts::convert_attach(self.dgShapeId);
      if let Some(x) = att.xfrm {
        return Some(x);
      }
      if let Some(pr) = att.shape_pr {
        // rust: ★③-0c —— `shape_pr.convert_xfrm_clone()` 现产出**真身** `basic::Transform2D`，
        //   而本文件（影子 `KmoShowShape`，产物 B 渲染路径使用）的 `Transform2D` 仍是影子类型。
        //   二者桥接按 KProto 往返：真身 `saveBin` → 影子 `open`。
        //   owner: ppt.show.app.shape.KmoShowShape    wave: M-PPT-SHAPE-DISPATCH
        if let Some(x) = pr.convert_xfrm_clone() {
          let mut real = x;
          let bin = real.saveBin(crate::cn::wps::kso::kproto::KoZone::KoZone::enter());
          return Some(Transform2D::Transform2D_from(bin.as_ref()));
        }
      }
      Some(Transform2D::default())
    }

    // java public ShapePr shapePr()
    pub fn shapePr(&self) -> Option<ShapePr> {
      // java return this.type() == 4 ? null : PlaceHolderMerger.mergeShapePr(this);
      if self.r#type() == 4 {
        None
      } else {
        Some(
          super::kmo_show_shape_hosts::convert_attach(self.dgShapeId)
            .shape_pr
            .unwrap_or_else(ShapePr::create),
        )
      }
    }

    // java public ShapeStyle style()
    pub fn style(&self) -> Option<ShapeStyle> {
      // java return this.type() != 0 && this.type() != 4 ? PlaceHolderMerger.mergeShapeStyle(this) : null;
      None
    }

    // java public int textDirection()
    pub fn textDirection(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java KmoTextOperator textOperator = this.getTextOperator();
      let mut textOperator: KmoTextOperator = self.getTextOperator().unwrap();
      // java return null == textOperator ? 0 : textOperator.textDirection(this);
      return (if false { 0 } else { textOperator.textDirection(self) });
      */
    }

    // java public void setTextDirection(int textDirection)
    pub fn setTextDirection(&mut self, textDirection: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (textDirection >= 0 && 6 >= textDirection)
      if textDirection >= 0 && 6 >= textDirection {
        // java KmoTextOperator textOperator = this.getTextOperator();
        let mut textOperator: KmoTextOperator = self.getTextOperator().unwrap();
        // java if (null != textOperator)
        if true {
          // java textOperator.putTextDirection(textDirection);
          textOperator.putTextDirection(textDirection);
          // java textOperator.save(this);
          textOperator.save(self);
        }
      }
      */
    }

    // java public boolean hasAudio()
    pub fn hasAudio(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return ((ShowShapeAtomData)this.rawData()).shape.hasMedia() && (((ShowShapeAtomData)this.rawData()).shape.media().hasAudioCD() || ((ShowShapeAtomData)this.rawData()).shape.media().hasAudioFile() || ((ShowShapeAtomData)this.rawData()).shape.media().hasEmbeddedWAVAudioFile());
      return (self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.hasMedia() && (self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.media().hasAudioCD() || (self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.media().hasAudioFile() || (self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.media().hasEmbeddedWAVAudioFile();
      */
    }

    // java public boolean hasVideo()
    pub fn hasVideo(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return ((ShowShapeAtomData)this.rawData()).shape.hasMedia() && ((ShowShapeAtomData)this.rawData()).shape.media().hasVideoFile();
      return (self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.hasMedia() && (self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.media().hasVideoFile();
      */
    }

    // java public int audioMediaId()
    pub fn audioMediaId(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (!((ShowShapeAtomData)this.rawData()).shape.hasMedia())
      if !(self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.hasMedia() {
        // java return -1;
        return -1;
      }
      // java else
      else {
        // java Media media = ((ShowShapeAtomData)this.rawData()).shape.media();
        let mut media: Media = (self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.media();
        // java if (media.hasAudioFile())
        if media.hasAudioFile() {
          // java return media.audioFile().mediaID();
          return media.audioFile().mediaID();
        }
        // java else
        else {
          // java return media.hasEmbeddedWAVAudioFile() ? media.embeddedWAVAudioFile().mediaID() : -1;
          return (if media.hasEmbeddedWAVAudioFile() { media.embeddedWAVAudioFile().mediaID() } else { -1 });
        }
      }
      */
    }

    // java public int mediaTrimStart()
    pub fn mediaTrimStart(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (!((ShowShapeAtomData)this.rawData()).shape.hasMedia())
      if !(self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.hasMedia() {
        // java return -1;
        return -1;
      }
      // java else
      else {
        // java Media media = ((ShowShapeAtomData)this.rawData()).shape.media();
        let mut media: Media = (self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.media();
        // java if (media.hasMediaExtAttr())
        if media.hasMediaExtAttr() {
          // java Media.MediaExtAttr mediaExtAttr = media.mediaExtAttr();
          let mut mediaExtAttr: Media::MediaExtAttr = media.mediaExtAttr();
          // java if (mediaExtAttr.hasTrimStart())
          if mediaExtAttr.hasTrimStart() {
            // java return mediaExtAttr.trimStart();
            return mediaExtAttr.trimStart();
          }
        }
        // java return 0;
        return 0;
      }
      */
    }

    // java public int mediaTrimEnd()
    pub fn mediaTrimEnd(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (!((ShowShapeAtomData)this.rawData()).shape.hasMedia())
      if !(self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.hasMedia() {
        // java return -1;
        return -1;
      }
      // java else
      else {
        // java Media media = ((ShowShapeAtomData)this.rawData()).shape.media();
        let mut media: Media = (self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.media();
        // java if (media.hasMediaExtAttr())
        if media.hasMediaExtAttr() {
          // java Media.MediaExtAttr mediaExtAttr = media.mediaExtAttr();
          let mut mediaExtAttr: Media::MediaExtAttr = media.mediaExtAttr();
          // java if (mediaExtAttr.hasTrimEnd())
          if mediaExtAttr.hasTrimEnd() {
            // java return mediaExtAttr.trimEnd();
            return mediaExtAttr.trimEnd();
          }
        }
        // java return 0;
        return 0;
      }
      */
    }

    // java public int videoMediaId()
    pub fn videoMediaId(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (!((ShowShapeAtomData)this.rawData()).shape.hasMedia())
      if !(self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.hasMedia() {
        // java return -1;
        return -1;
      }
      // java else
      else {
        // java Media media = ((ShowShapeAtomData)this.rawData()).shape.media();
        let mut media: Media = (self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.media();
        // java return media.hasVideoFile() ? media.videoFile().mediaID() : -1;
        return (if media.hasVideoFile() { media.videoFile().mediaID() } else { -1 });
      }
      */
    }

    // java public void delVideoFile()
    pub fn delVideoFile(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (((ShowShapeAtomData)this.rawData()).shape.hasMedia())
      if (self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.hasMedia() {
        // java ((ShowShapeAtomData)this.rawData()).shape.media().videoFile().deleteMediaID();
        (self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.media().videoFile().deleteMediaID();
        // java ((ShowShapeAtomData)this.rawData()).shape.save();
        (self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.save();
      }
      */
    }

    // java public void addVideoFile(String videoPath)
    pub fn addVideoFile(&mut self, videoPath: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.type() == 2)
      if self.r#type() == 2 {
        // java String fileExtension = videoPath.substring(videoPath.lastIndexOf(".") + 1);
        let mut fileExtension: String = videoPath.substring(videoPath.lastIndexOf(".") + 1);
        // java byte mediaType = KmoMediaUtil.getMediaType(fileExtension);
        let mut mediaType: u8 = KmoMediaUtil::getMediaType(fileExtension);
        // java FileDataStorage fileDataStorage = new FileDataStorage(new File(videoPath));
        let mut fileDataStorage: FileDataStorage = FileDataStorage::new(File::new(videoPath));
        // java int mediaId = this.parent.getPresentation().mediaLib().addMediaData((FileDataStorage)fileDataStorage, mediaType);
        let mut mediaId: i32 = self.parent.getPresentation().mediaLib().addMediaData((fileDataStorage).downcast_ref::<FileDataStorage>().unwrap(), mediaType);
        // java Media media = Media.create();
        let mut media: Media = Media::create();
        // java media.putMode(3);
        media.putMode(3);
        // java Media.VideoFile videoFile = Media.VideoFile.create();
        let mut videoFile: Media::VideoFile = Media::VideoFile.create();
        // java videoFile.putMediaID(mediaId);
        videoFile.putMediaID(mediaId);
        // java videoFile.save();
        videoFile.save();
        // java media.writeVideoFile(videoFile);
        media.writeVideoFile(videoFile);
        // java media.save();
        media.save();
        // java ((ShowShapeAtomData)this.rawData()).shape.writeMedia(media);
        (self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.writeMedia(media);
        // java ((ShowShapeAtomData)this.rawData()).shape.save();
        (self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.save();
      }
      */
    }

    // java public void delAudioFile()
    pub fn delAudioFile(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (((ShowShapeAtomData)this.rawData()).shape.hasMedia())
      if (self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.hasMedia() {
        // java ((ShowShapeAtomData)this.rawData()).shape.media().audioFile().deleteMediaID();
        (self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.media().audioFile().deleteMediaID();
        // java ((ShowShapeAtomData)this.rawData()).shape.save();
        (self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.save();
      }
      */
    }

    // java public void addAudioFile(String audioPath)
    pub fn addAudioFile(&mut self, audioPath: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.type() == 2)
      if self.r#type() == 2 {
        // java String fileExtension = audioPath.substring(audioPath.lastIndexOf(".") + 1);
        let mut fileExtension: String = audioPath.substring(audioPath.lastIndexOf(".") + 1);
        // java byte mediaType = KmoMediaUtil.getMediaType(fileExtension);
        let mut mediaType: u8 = KmoMediaUtil::getMediaType(fileExtension);
        // java FileDataStorage fileDataStorage = new FileDataStorage(new File(audioPath));
        let mut fileDataStorage: FileDataStorage = FileDataStorage::new(File::new(audioPath));
        // java int mediaId = this.parent.getPresentation().mediaLib().addMediaData((FileDataStorage)fileDataStorage, mediaType);
        let mut mediaId: i32 = self.parent.getPresentation().mediaLib().addMediaData((fileDataStorage).downcast_ref::<FileDataStorage>().unwrap(), mediaType);
        // java Media media = Media.create();
        let mut media: Media = Media::create();
        // java media.putMode(2);
        media.putMode(2);
        // java Media.AudioFile audioFile = Media.AudioFile.create();
        let mut audioFile: Media::AudioFile = Media::AudioFile.create();
        // java audioFile.putMediaID(mediaId);
        audioFile.putMediaID(mediaId);
        // java audioFile.save();
        audioFile.save();
        // java media.writeAudioFile(audioFile);
        media.writeAudioFile(audioFile);
        // java media.save();
        media.save();
        // java ((ShowShapeAtomData)this.rawData()).shape.writeMedia(media);
        (self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.writeMedia(media);
        // java ((ShowShapeAtomData)this.rawData()).shape.save();
        (self.rawData()).downcast_ref::<ShowShapeAtomData>().unwrap().shape.save();
      }
      */
    }

    // java public DgShape getDgShape()
    pub fn getDgShape(&self) -> Option<DgShape> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java DgShape dgShape = (DgShape)this.dgShapeThreadLocal.get();
      let mut dgShape: DgShape = (self.dgShapeThreadLocal.get()).downcast_ref::<DgShape>().unwrap();
      // java if (dgShape == null)
      if false {
        // java dgShape = (new DgShape(this.parent.getPresentation().drawingStore())).find(this.dgShapeId);
        dgShape = DgShape::new(self.parent.getPresentation().drawingStore()).find(self.dgShapeId);
        // java this.dgShapeThreadLocal.set(dgShape);
        self.dgShapeThreadLocal.set(dgShape);
      }
      // java dgShape.find(this.id());
      dgShape.find(self.id());
      // java return dgShape;
      return Some(dgShape);
      */
    }

    // java public DgGroup getDgGroup()
    pub fn getDgGroup(&mut self) -> &mut DgGroup {
      // java DgGroup dgGroup = (DgGroup)this.dgGroupThreadLocal.get();
      let mut dgGroup_missing = self.dgGroupThreadLocal.is_none();
      // java if (dgGroup == null)
      if dgGroup_missing {
        // java dgGroup = (new DgGroup(this.parent.getPresentation().drawingStore())).find(this.dgShapeId);
        let store = match self.parent {
          Some(p) => unsafe { (*p).getPresentation().drawingStore() },
          None => std::ptr::null_mut(),
        };
        let mut dgGroup = DgGroup::DgGroup(store);
        dgGroup.find(self.dgShapeId);
        // java this.dgGroupThreadLocal.set(dgGroup);
        self.dgGroupThreadLocal = Some(dgGroup);
      }
      // java dgGroup.find(this.id());
      let this_id = self.id();
      let dgGroup = self.dgGroupThreadLocal.as_mut().unwrap();
      dgGroup.find(this_id);
      // java return dgGroup;
      dgGroup
    }

    // java public DgConnector getDgConn()
    pub fn getDgConn(&self) -> Option<DgConnector> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java DgConnector dgConnector = (DgConnector)this.dgConnThreadLocal.get();
      let mut dgConnector: DgConnector = (self.dgConnThreadLocal.get()).downcast_ref::<DgConnector>().unwrap();
      // java if (dgConnector == null)
      if false {
        // java dgConnector = (new DgConnector(this.parent.getPresentation().drawingStore())).find(this.dgShapeId);
        dgConnector = DgConnector::new(self.parent.getPresentation().drawingStore()).find(self.dgShapeId);
        // java this.dgConnThreadLocal.set(dgConnector);
        self.dgConnThreadLocal.set(dgConnector);
      }
      // java dgConnector.find(this.id());
      dgConnector.find(self.id());
      // java return dgConnector;
      return Some(dgConnector);
      */
    }

    // java public DgPicture getDgPicture()
    pub fn getDgPicture(&self) -> Option<DgPicture> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java DgPicture dgPicture = (DgPicture)this.dgPicThreadLocal.get();
      let mut dgPicture: DgPicture = (self.dgPicThreadLocal.get()).downcast_ref::<DgPicture>().unwrap();
      // java if (dgPicture == null)
      if false {
        // java dgPicture = (new DgPicture(this.parent.getPresentation().drawingStore())).find(this.dgShapeId);
        dgPicture = DgPicture::new(self.parent.getPresentation().drawingStore()).find(self.dgShapeId);
        // java this.dgPicThreadLocal.set(dgPicture);
        self.dgPicThreadLocal.set(dgPicture);
      }
      // java dgPicture.find(this.id());
      dgPicture.find(self.id());
      // java return dgPicture;
      return Some(dgPicture);
      */
    }

    // java public void setTextRect(RectF textRect)
    pub fn setTextRect(&mut self, textRect: RectF) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.textRect = textRect;
      self.textRect = textRect;
      */
    }

    // java public RectF getTextRect()
    pub fn getTextRect(&self) -> Option<RectF> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.textRect;
      return self.textRect;
      */
    }

    // java public void setBeautifyOrder(int order)
    pub fn setBeautifyOrder(&mut self, order: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.beautifyOrder = order;
      self.beautifyOrder = order;
      */
    }

    // java public int beautifyOrder()
    pub fn beautifyOrder(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.beautifyOrder;
      return self.beautifyOrder;
      */
    }

    // java private void setLocked(boolean isLocked)
    pub fn setLocked(&mut self, isLocked: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.isLocked = isLocked;
      self.isLocked = isLocked;
      */
    }

    // java public boolean isLocked()
    pub fn isLocked(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.isLocked;
      return self.isLocked;
      */
    }

    // java public boolean parentLocked()
    pub fn parentLocked(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java boolean isLocked = false;
      let mut isLocked: bool = false;
      // java if (!this.isChildOfGroup())
      if !self.isChildOfGroup() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java KmoShowShape ancestor = this.parentGroup();
        let mut ancestor: KmoShowShape = self.parentGroup().unwrap();
        // java if (ancestor.isLocked())
        if ancestor.isLocked() {
          // java return true;
          return true;
        }
        // java else
        else {
          // java while (ancestor.isChildOfGroup())
          while ancestor.isChildOfGroup() {
            // java ancestor = ancestor.parentGroup();
            ancestor = ancestor.parentGroup();
            // java if (ancestor.isLocked())
            if ancestor.isLocked() {
              // java isLocked = true;
              isLocked = true;
              // java break;
              break;
            }
          }
          // java return isLocked;
          return isLocked;
        }
      }
      */
    }

    // java public void lock()
    pub fn lock(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (!this.isLocked)
      if !self.isLocked {
        // java this.setLocked(true);
        self.setLocked(true);
        // java this.lockParent();
        self.lockParent();
        // java this.lockChildrens(this);
        self.lockChildrens(self);
        // java this.parent.getPresentation().selection().unselectText();
        self.parent.getPresentation().selection().unselectText();
        // java this.parent.getPresentation().eventListeners().onSelectionChanged(2);
        self.parent.getPresentation().eventListeners().onSelectionChanged(2);
        // java this.parent.eventListeners().onSlideContentsChanged();
        self.parent.eventListeners().onSlideContentsChanged();
      }
      */
    }

    // java private void lockChildrens(KmoShowShape kmoShowShape)
    pub fn lockChildrens(&mut self, kmoShowShape: KmoShowShape) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (kmoShowShape.childCount() > 0)
      if kmoShowShape.childCount() > 0 {
        let mut childIndex: i32 = 0;
        // java for (int childIndex = 0; childIndex < kmoShowShape.childCount() ++childIndex)
        while childIndex < kmoShowShape.childCount() {
          // java KmoShowShape childShape = kmoShowShape.childAt(childIndex);
          let mut childShape: KmoShowShape = kmoShowShape.childAt(childIndex).unwrap();
          // java if (!childShape.isLocked())
          if !childShape.isLocked() {
            // java childShape.setLocked(true);
            childShape.setLocked(true);
            // java this.lockChildrens(childShape);
            self.lockChildrens(childShape);
          }
        }
      }
      */
    }

    // java private void lockParent()
    pub fn lockParent(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java KmoShowShape parentGroup = this.parentGroup();
      let mut parentGroup: KmoShowShape = self.parentGroup().unwrap();
      // java if (null != parentGroup)
      if true {
        // java int childCount = parentGroup.childCount();
        let mut childCount: i32 = parentGroup.childCount();
        // java if (childCount > 0)
        if childCount > 0 {
          let mut childIndex: i32 = 0;
          // java for (int childIndex = 0; childIndex < childCount ++childIndex)
          while childIndex < childCount {
            // java KmoShowShape childShape = parentGroup.childAt(childIndex);
            let mut childShape: KmoShowShape = parentGroup.childAt(childIndex).unwrap();
            // java if (!childShape.isLocked())
            if !childShape.isLocked() {
              // java return;
              return;
            }
          }
        }
        // java parentGroup.setLocked(true);
        parentGroup.setLocked(true);
        // java parentGroup.lockParent();
        parentGroup.lockParent();
      }
      */
    }

    // java public void unlock()
    pub fn unlock(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.isLocked())
      if self.isLocked() {
        // java this.setLocked(false);
        self.setLocked(false);
        // java this.unlockChildren(this);
        self.unlockChildren(self);
        // java this.unlockParent(this);
        self.unlockParent(self);
        // java this.parent.getPresentation().eventListeners().onSelectionChanged(2);
        self.parent.getPresentation().eventListeners().onSelectionChanged(2);
        // java this.parent.eventListeners().onSlideContentsChanged();
        self.parent.eventListeners().onSlideContentsChanged();
      }
      */
    }

    // java private void unlockChildren(KmoShowShape kmoShowShape)
    pub fn unlockChildren(&mut self, kmoShowShape: KmoShowShape) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (kmoShowShape.childCount() > 0)
      if kmoShowShape.childCount() > 0 {
        let mut childIndex: i32 = 0;
        // java for (int childIndex = 0; childIndex < kmoShowShape.childCount() ++childIndex)
        while childIndex < kmoShowShape.childCount() {
          // java KmoShowShape childShape = kmoShowShape.childAt(childIndex);
          let mut childShape: KmoShowShape = kmoShowShape.childAt(childIndex).unwrap();
          // java childShape.setLocked(false);
          childShape.setLocked(false);
          // java this.unlockChildren(childShape);
          self.unlockChildren(childShape);
        }
      }
      */
    }

    // java private void unlockParent(KmoShowShape kmoShowShape)
    pub fn unlockParent(&mut self, kmoShowShape: KmoShowShape) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (kmoShowShape.isChildOfGroup())
      if kmoShowShape.isChildOfGroup() {
        // java KmoShowShape parentGroup = this.getParentShapeLocked(kmoShowShape);
        let mut parentGroup: KmoShowShape = self.getParentShapeLocked(kmoShowShape);
        // java if (parentGroup.id() != kmoShowShape.id())
        if parentGroup.id() != kmoShowShape.id() {
          // java parentGroup.setLocked(false);
          parentGroup.setLocked(false);
          // java this.unlockChildren(parentGroup);
          self.unlockChildren(parentGroup);
        }
      }
      */
    }

    // java public KmoShowShape getParentShapeLocked(KmoShowShape shape)
    pub fn getParentShapeLocked(&self, shape: KmoShowShape) -> KmoShowShape {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (shape.isTable())
      if shape.isTable() {
        // java return shape;
        return shape;
      }
      // java else
      else {
        // java while (null != shape.parentGroup() && shape.parentGroup().isLocked())
        while shape.parentGroup().is_some() && shape.parentGroup().isLocked() {
          // java shape = shape.parentGroup();
          shape = shape.parentGroup();
        }
        // java return shape;
        return shape;
      }
      */
    }

    // java public void adjust(PresetGeometry.AdjustValueList adjustValueList)
    pub fn adjust(&mut self, adjustValueList: AdjustValueList) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.getDgShape().adjust(adjustValueList);
      self.getDgShape().adjust(adjustValueList);
      // java this.parent.getPresentation().setDirty();
      self.parent.getPresentation().setDirty();
      // java this.parent.eventListeners().onShapeContentsChanged(3, this);
      self.parent.eventListeners().onShapeContentsChanged(3, self);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      */
    }

    // java public int[] getPictureOriginSize()
    pub fn getPictureOriginSize(&self) -> Vec<i32> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int[] size = new int[]{0, 0};
      let mut size: Vec<i32> = vec![0, 0];
      // java if (this.type() != 2)
      if self.r#type() != 2 {
        // java return size;
        return size;
      }
      // java else
      else {
        // java String path = this.parent.getPresentation().mediaLib().getMediaPath(this.getPicId());
        let mut path: String = self.parent.getPresentation().mediaLib().getMediaPath(self.getPicId());
        // java if (!path.isEmpty())
        if !path.is_empty() {
          // java PictureInfo info = PictureParser.getPictureInfo(path);
          let mut info: PictureInfo = PictureParser::getPictureInfo__string(path);
          // java if (info != null)
          if true {
            // java size[0] = info.width;
            size[0usize] = info.width;
            // java size[1] = info.height;
            size[1usize] = info.height;
          }
        }
        // java return size;
        return size;
      }
      */
    }

    // java public void addCustTags(String tagKey, String tagValue)
    pub fn addCustTags(&mut self, tagKey: &str, tagValue: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (tagKey != null && !tagKey.isEmpty() && tagValue != null)
      if true && !tagKey.is_empty() && true {
        // java CustDataLst.TagLst tagLst = this.custDataLst().tags();
        let mut tagLst: CustDataLst::TagLst = self.custDataLst().tags();
        // java ArrayList<Tag> oriTags = new ArrayList();
        let mut oriTags: Vec<Tag> = { let __x: Vec<Tag> = todo!("no align strategy matched"); __x };
        // java tagLst.read(oriTags);
        tagLst.read(oriTags);
        // java Tag tag = Tag.create();
        let mut tag: Tag = Tag::create();
        // java tag.putName(tagKey);
        tag.putName(tagKey);
        // java tag.putVal(tagValue);
        tag.putVal(tagValue);
        // java tag.save();
        tag.save();
        // java oriTags.add(tag);
        oriTags.add(tag);
        // java this.custDataLst().tags().write(oriTags);
        self.custDataLst().tags().write(oriTags);
      }
      */
    }

    // java public void deleCustTags(String tagName)
    pub fn deleCustTags(&mut self, tagName: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (tagName != null && !tagName.isEmpty())
      if true && !tagName.is_empty() {
        // java CustDataLst.TagLst tagLst = this.custDataLst().tags();
        let mut tagLst: CustDataLst::TagLst = self.custDataLst().tags();
        // java ArrayList<Tag> oriTags = new ArrayList();
        let mut oriTags: Vec<Tag> = { let __x: Vec<Tag> = todo!("no align strategy matched"); __x };
        // java tagLst.read(oriTags);
        tagLst.read(oriTags);
        // java Iterator<Tag> iterator = oriTags.iterator();
        let mut iterator: Vec<Tag> = oriTags.iterator();
        // java while (iterator.hasNext())
        while iterator.hasNext() {
          // java Tag item = (Tag)iterator.next();
          let mut item: Tag = (iterator.next()).downcast_ref::<Tag>().unwrap();
          // java if (tagName.equals(item.name()))
          if tagName.equals(item.name()) {
            // java iterator.remove();
            iterator.remove();
            // java break;
            break;
          }
        }
        // java this.custDataLst().tags().write(oriTags);
        self.custDataLst().tags().write(oriTags);
      }
      */
    }

    // java public boolean isEditingText()
    pub fn isEditingText(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java KmoSelection selection = this.parent.getPresentation().selection();
      let mut selection: KmoSelection = self.parent.getPresentation().selection();
      // java return selection != null && selection.state() == 3 && selection.selectedShape().id() == this.id();
      return true && selection.state() == 3 && selection.selectedShape().id() == self.id();
      */
    }

    // java public void saveOriginalProperty()
    pub fn saveOriginalProperty(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.originalAutoFit = this.getDgShape().textAutoFit();
      self.originalAutoFit = self.getDgShape().textAutoFit();
      // java this.originalAnchor = 0;
      self.originalAnchor = 0;
      // java if (this.getTextOperator() != null)
      if self.getTextOperator().is_some() {
        // java this.originalAnchor = this.getTextOperator().getAnchor(this);
        self.originalAnchor = self.getTextOperator().getAnchor(self);
      }
      */
    }

    // java public int getOriginalAutoFit()
    pub fn getOriginalAutoFit(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.originalAutoFit;
      return self.originalAutoFit;
      */
    }

    // java public int getOriginalAnchor()
    pub fn getOriginalAnchor(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.originalAnchor;
      return self.originalAnchor;
      */
    }

    // java public void resetOriginalProperty()
    pub fn resetOriginalProperty(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.originalAutoFit = -1;
      self.originalAutoFit = -1;
      // java this.originalAnchor = -1;
      self.originalAnchor = -1;
      */
    }

    // java public KsoWMInfo shapeTagMap()
    // rust: ★③-5 真实实现（原为 `todo!("DEFERRED_NON_FUNCTION")` —— 因 `KsoWMInfo` 是单元空壳
    //   而无从实现；现改用真身 `show::app::KsoWMInfo`）。Java 原文：
    //     return KsoWMInfo.create(this.custDataLst());
    //   `custDataLst()` 因此一并真实化（见下）。
    //   owner: show.app.shape.KmoShowShape    wave: M-PPT-SHAPE-DISPATCH
    pub fn shapeTagMap(&self) -> Option<KsoWMInfo> {
      // java return KsoWMInfo.create(this.custDataLst());
      Some(KsoWMInfo::create(self.custDataLst()?))
    }

    // java public boolean is3DObject()
    pub fn is3DObject(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java OleObject ole = this.getOle();
      let mut ole: OleObject = self.getOle().unwrap();
      // java if (ole != null && ole.hasProgID())
      if true && ole.hasProgID() {
        // java String progID = ole.progID();
        let mut progID: String = ole.progID();
        // java String name = ole.ole().name();
        let mut name: String = ole.ole().name();
        // java String oleType = (String)this.shapeTagMap().get("OLE_FILE_TYPE");
        let mut oleType: String = { let __x: String = todo!("no align strategy matched"); __x };
        // java String tagFilename = (String)this.shapeTagMap().get("OLE_FILE_NAME");
        let mut tagFilename: String = { let __x: String = todo!("no align strategy matched"); __x };
        // java boolean has3DTag = "3dobj".equals(oleType);
        let mut has3DTag: bool = "3dobj".equals(oleType);
        // java boolean has3DName = name != null && "3dobj".equals(name.toLowerCase());
        let mut has3DName: bool = true && "3dobj".equals(name.toLowerCase());
        // java boolean isTagFilename3D = false;
        let mut isTagFilename3D: bool = false;
        // java if (tagFilename != null)
        if true {
          // java isTagFilename3D = tagFilename.toLowerCase().endsWith("glb") || tagFilename.toLowerCase().endsWith("rsdz");
          isTagFilename3D = tagFilename.toLowerCase().endsWith("glb") || tagFilename.toLowerCase().endsWith("rsdz");
        }
        // java if ((has3DName || has3DTag || isTagFilename3D) && progID != null && progID.equalsIgnoreCase("Package"))
        if has3DName || has3DTag || isTagFilename3D && true && progID.equalsIgnoreCase("Package") {
          // java return true;
          return true;
        }
      }
      // java return false;
      return false;
      */
    }

    // java public boolean isModularGroup()
    pub fn isModularGroup(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java DgStore srcStore = this.parent.getPresentation().drawingStore();
      let mut srcStore: DgStore = self.parent.getPresentation().drawingStore();
      // java Shape srcSp = Shape.open(srcStore.query(this.dgShapeId));
      let mut srcSp: Shape = Shape::open(srcStore.query(self.dgShapeId));
      // java return srcSp != null && srcSp.hasExtLst() && this.type() == 0 ? ModularGroupUtil.isValidModularGroup(this) : false;
      return (if true && srcSp.hasExtLst() && self.r#type() == 0 { ModularGroupUtil::isValidModularGroup(self) } else { false });
      */
    }

    // java public String modularGroupType()
    pub fn modularGroupType(&self) -> String {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java DgStore srcStore = this.parent.getPresentation().drawingStore();
      let mut srcStore: DgStore = self.parent.getPresentation().drawingStore();
      // java Shape srcSp = Shape.open(srcStore.query(this.dgShapeId));
      let mut srcSp: Shape = Shape::open(srcStore.query(self.dgShapeId));
      // java if (srcSp != null && srcSp.hasExtLst())
      if true && srcSp.hasExtLst() {
        // java List<ShapeExt> shapeExtList = new ArrayList();
        let mut shapeExtList: Vec<ShapeExt> = { let __x: Vec<ShapeExt> = todo!("no align strategy matched"); __x };
        // java srcSp.extLst().read(shapeExtList);
        srcSp.extLst().read(shapeExtList);
        // java for (ShapeExt ext : shapeExtList)
        for ext in shapeExtList.iter() {
          // java if (ext.hasUri() && "{D589DF8D-B311-4D76-A726-2CF4D000D303}".equals(ext.uri()) && ext.hasType())
          if ext.hasUri() && "{D589DF8D-B311-4D76-A726-2CF4D000D303}".equals(ext.uri()) && ext.hasType() {
            // java return ext.type();
            return { let __x: String = todo!("no align strategy matched"); __x };
          }
        }
        // java return "";
        return "".to_string();
      }
      // java else
      else {
        // java return "";
        return "".to_string();
      }
      */
    }

    // java public String modularGroupIndex()
    pub fn modularGroupIndex(&self) -> String {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java DgStore srcStore = this.parent.getPresentation().drawingStore();
      let mut srcStore: DgStore = self.parent.getPresentation().drawingStore();
      // java Shape srcSp = Shape.open(srcStore.query(this.dgShapeId));
      let mut srcSp: Shape = Shape::open(srcStore.query(self.dgShapeId));
      // java if (srcSp != null && srcSp.hasExtLst())
      if true && srcSp.hasExtLst() {
        // java List<ShapeExt> shapeExtList = new ArrayList();
        let mut shapeExtList: Vec<ShapeExt> = { let __x: Vec<ShapeExt> = todo!("no align strategy matched"); __x };
        // java srcSp.extLst().read(shapeExtList);
        srcSp.extLst().read(shapeExtList);
        // java for (ShapeExt ext : shapeExtList)
        for ext in shapeExtList.iter() {
          // java if (ext.hasUri() && "{D589DF8D-B311-4D76-A726-2CF4D000D303}".equals(ext.uri()) && ext.hasIndex())
          if ext.hasUri() && "{D589DF8D-B311-4D76-A726-2CF4D000D303}".equals(ext.uri()) && ext.hasIndex() {
            // java return ext.index();
            return ext.index();
          }
        }
        // java return "";
        return "".to_string();
      }
      // java else
      else {
        // java return "";
        return "".to_string();
      }
      */
    }

    // java public void putModularGroupExt(String type, String index, Boolean isDefault)
    pub fn putModularGroupExt__string_string_boolean(&self, r#type: &str, index: Option<&str>, isDefault: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java ShapeExt ext = ShapeExt.create();
      let mut ext: ShapeExt = ShapeExt::create();
      // java ext.putUri("{D589DF8D-B311-4D76-A726-2CF4D000D303}");
      ext.putUri("{D589DF8D-B311-4D76-A726-2CF4D000D303}");
      // java ext.putType(type);
      ext.putType(r#type);
      // java if (index != null && index.length() != 0)
      if let Some(index) = index {
        if (index.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) != 0 {
          // java ext.putIndex(index);
          ext.putIndex(index);
        }
      }
      // java if (isDefault != null)
      if true {
        // java ext.putIsDefault(isDefault);
        ext.putIsDefault(isDefault);
      }
      // java ext.save();
      ext.save();
      // java List<ShapeExt> tmp = this.getDgShape().getShapeExtLst();
      let mut tmp: Vec<ShapeExt> = self.getDgShape().getShapeExtLst();
      // java if (!tmp.contains(ext))
      if !tmp.contains(ext) {
        // java tmp.add(ext);
        tmp.add(ext);
      }
      // java this.getDgShape().writeShapeExtLst(tmp);
      self.getDgShape().writeShapeExtLst(tmp);
      */
    }

    // java public void putModularGroupExt(String type)
    pub fn putModularGroupExt__string(&mut self, r#type: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.putModularGroupExt(type, "", (Boolean)null);
      self.putModularGroupExt__string_string_boolean(&r#type, &"", None);
      */
    }

    // java public void putModularGroupExt(String type, String index)
    pub fn putModularGroupExt__string_string(&mut self, r#type: &str, index: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.putModularGroupExt(type, index, (Boolean)null);
      self.putModularGroupExt__string_string_boolean(&r#type, &index, None);
      */
    }

    // java public boolean isModularGroupPicDefault()
    pub fn isModularGroupPicDefault(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java DgStore srcStore = this.parent.getPresentation().drawingStore();
      let mut srcStore: DgStore = self.parent.getPresentation().drawingStore();
      // java Shape srcSp = Shape.open(srcStore.query(this.dgShapeId));
      let mut srcSp: Shape = Shape::open(srcStore.query(self.dgShapeId));
      // java if (srcSp != null && srcSp.hasExtLst())
      if true && srcSp.hasExtLst() {
        // java List<ShapeExt> shapeExtList = new ArrayList();
        let mut shapeExtList: Vec<ShapeExt> = { let __x: Vec<ShapeExt> = todo!("no align strategy matched"); __x };
        // java srcSp.extLst().read(shapeExtList);
        srcSp.extLst().read(shapeExtList);
        // java for (ShapeExt ext : shapeExtList)
        for ext in shapeExtList.iter() {
          // java if (ext.hasUri() && "{D589DF8D-B311-4D76-A726-2CF4D000D303}".equals(ext.uri()) && ext.hasIsDefault() && ext.type().equals("pic"))
          if ext.hasUri() && "{D589DF8D-B311-4D76-A726-2CF4D000D303}".equals(ext.uri()) && ext.hasIsDefault() && ext.r#type().equals("pic") {
            // java return ext.isIsDefault();
            return ext.isIsDefault();
          }
        }
        // java return false;
        return false;
      }
      // java else
      else {
        // java return false;
        return false;
      }
      */
    }

    // java public void putModularGroupPicture(int mediaId)
    pub fn putModularGroupPicture(&mut self, mediaId: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.getDgPicture().putModularGroupPicture(mediaId);
      self.getDgPicture().putModularGroupPicture(mediaId);
      */
    }

    // java public void copyModularDgShapeAttrFrom(KmoShowShape kmoShowShape)
    pub fn copyModularDgShapeAttrFrom(&mut self, kmoShowShape: KmoShowShape) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Shape shape = Shape.open(kmoShowShape.drawingShape());
      let mut shape: Shape = Shape::open(kmoShowShape.drawingShape());
      // java this.getDgShape().copySpprWithoutTransfromFrom(shape);
      self.getDgShape().copySpprWithoutTransfromFrom(shape);
      // java this.getDgShape().copyTextBodyPropertiesFrom(shape);
      self.getDgShape().copyTextBodyPropertiesFrom(shape);
      // java this.getDgShape().copyTextListStyleFrom(shape);
      self.getDgShape().copyTextListStyleFrom(shape);
      // java this.getDgShape().copyTextBodyFrom(shape);
      self.getDgShape().copyTextBodyFrom(shape);
      */
    }

    // java public void copyModularDgPictureAttrFrom(KmoShowShape kmoShowShape)
    pub fn copyModularDgPictureAttrFrom(&mut self, kmoShowShape: KmoShowShape) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Shape shape = Shape.open(kmoShowShape.drawingShape());
      let mut shape: Shape = Shape::open(kmoShowShape.drawingShape());
      // java this.getDgPicture().copyBlipEffectsFrom(shape);
      self.getDgPicture().copyBlipEffectsFrom(shape);
      // java this.getDgPicture().copySrcRectFrom(shape);
      self.getDgPicture().copySrcRectFrom(shape);
      */
    }

    // java public boolean isTitlePh()
    pub fn isTitlePh(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java ShowShape.Placeholder ph = this.ph();
      let mut ph: ShowShape::Placeholder = { let __x: ShowShape::Placeholder = todo!("no align strategy matched"); __x };
      // java return 3 == ph.type() || 2 == ph.type() || 0 == ph.type();
      return 3 == ph.r#type() || 2 == ph.r#type() || 0 == ph.r#type();
      */
    }

}

impl std::ops::Deref for KmoShowShape {
    type Target = KmoAtom<ShowShapeAtomData>;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for KmoShowShape {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
