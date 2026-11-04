#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unused_mut)]

// Java: package cn.wps.show.app.text
// Java: import cn.wps.drawing.data.text.TextBody
// Java: import cn.wps.drawing.data.text.TextListStyle
// rust: no-stmt —— `cn.wps.drawing.data.text.{TextBody,TextListStyle}` 真身在默认构建下被
//   `#[cfg(feature = "full")]` 关（`drawing/data/text/mod.rs`；`full` 因
//   `libs/core-libs/thirdparty/picparser` 10 个编译错误不可用），默认构建只剩 `textbody_hosts`
//   空宿主与 `util/layout_shape` 的 ZST 影子。本类**样式值面**（四个 ThreadLocal 缓存 +
//   `phStyleLayout()`/`phStyleMaster()` + 所有返回/接收 `TextListStyle` 的方法）因此整体 DEFERRED。
//   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
// Java: import cn.wps.show.app.shape.KmoShapeTree
// Java: import cn.wps.show.app.shape.KmoShowShape
use crate::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
// Java: import cn.wps.show.app.shape.PlaceHolderMerger
// rust: no-stmt —— Java 的 `cn.wps.show.app.shape.PlaceHolderMerger` 在 Rust 侧位于
//   `show/layout/PlaceHolderMerger.rs`，但其入参绑定 `util/layout_shape` 的 `KmoShowShape` 影子，
//   与本类使用的 `show/app/shape` 真身不是同一类型 ⇒ 本类内该调用随 DEFERRED 一并停车。
//   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
// Java: import cn.wps.show.app.slide.ISlide
use crate::cn::wps::show::app::slide::ISlide::ISlide;
// Java: import cn.wps.show.app.slide.KmoSlide
// Java: import cn.wps.show.app.slide.KmoSlideLayout
// Java: import cn.wps.show.app.slide.KmoSlideMaster
// rust: no-stmt —— `cn.wps.show.app.slide.{KmoSlide,KmoSlideLayout,KmoSlideMaster}` 在默认构建下
//   仅 `util/layout_shape.rs` 的 ZST 影子（且该文件归他波、禁改）⇒ `findMaster()` 及依其建立的
//   母版导航面整体 DEFERRED。
//   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
// Java: import java.util.HashMap
// Java: import java.util.Map
// rust: no-stmt —— 缓存 Map 的**值类型** `TextListStyle` 不可命名 ⇒ `phStyleLayout()`/
//   `phStyleMaster()` 两个 ThreadLocal<Map> 整体 DEFERRED（见方法注记）。
//   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

// Java: public class KmoTextStyleManager
//
// 本类是 `KmoSlide` 的成员 `textStyleManager`（`KmoSlide.java:55`，懒建于
// `KmoSlide.java:233-238`），职责是「模板继承（slide → layout → master → 默认）文本样式的缓存与解析」。
// 【SP3 落地范围】六个缓存有效位 + `isCleared` 状态机、`clear()`、`isCleared()`、`key()`、
//   `init()` 的**四级分派骨架**为真实实现；样式值面（返回/接收 `TextListStyle`）与母版导航面
//   （`KmoSlideMaster`）在上游 `TextListStyle`/`KmoSlide(Master|Layout)` 落到默认构建前无法命名，
//   逐项 DEFERRED（均带 owner/wave，供 SP3b 直接驱动）。
// owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3
pub struct KmoTextStyleManager {
    // Java field: private boolean isLayoutCacheValid = true
    pub(crate) isLayoutCacheValid: bool,
    // Java field: private boolean isMasterCacheValid = true
    pub(crate) isMasterCacheValid: bool,
    // Java field: private boolean isTitleStyleCacheValid = true
    pub(crate) isTitleStyleCacheValid: bool,
    // Java field: private boolean isBodyStyleCacheValid = true
    pub(crate) isBodyStyleCacheValid: bool,
    // Java field: private boolean isOtherStyleCacheValid = true
    pub(crate) isOtherStyleCacheValid: bool,
    // Java field: private boolean isDefaultStyleCacheValid = true
    pub(crate) isDefaultStyleCacheValid: bool,
    // Java field: private boolean isCleared = true
    pub(crate) isCleared: bool,
    // Java field: private ThreadLocal<Map<Integer, TextListStyle>> phStyleLayoutThreadLocal = new ThreadLocal()
    //
    // DEFERRED_NON_FUNCTION: `TextListStyle` 默认构建下不可命名 ⇒ 该 ThreadLocal 的**类型**无法表达。
    //   SP3b 落地真 `TextListStyle` 后按 Java 语义补 `thread_local!{ static PH_STYLE_LAYOUT:
    //   RefCell<HashMap<i32, TextListStyle>> }`，并把 `isLayoutCacheValid` 与 `phStyleLayout()`
    //   配套接上。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
    // Java field: private ThreadLocal<Map<Integer, TextListStyle>> phStyleMasterThreadLocal = new ThreadLocal()
    //
    // DEFERRED_NON_FUNCTION: 同上。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
    // Java field: private ThreadLocal<TextListStyle> titleStyleThreadLocal = new ThreadLocal()
    //
    // DEFERRED_NON_FUNCTION: 同上。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
    // Java field: private ThreadLocal<TextListStyle> bodyStyleThreadLocal = new ThreadLocal()
    //
    // DEFERRED_NON_FUNCTION: 同上。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
    // Java field: private ThreadLocal<TextListStyle> otherStyleThreadLocal = new ThreadLocal()
    //
    // DEFERRED_NON_FUNCTION: 同上。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
    // Java field: private ThreadLocal<TextListStyle> defaultStyleThreadLocal = new ThreadLocal()
    //
    // DEFERRED_NON_FUNCTION: 同上。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
}

// Java: public class KmoTextStyleManager
impl Default for KmoTextStyleManager {
    // Java: KmoTextStyleManager()（隐式无参构造器；见下方 `new()`）
    fn default() -> Self {
        KmoTextStyleManager {
            // Java field initializer: private boolean isLayoutCacheValid = true
            isLayoutCacheValid: true,
            // Java field initializer: private boolean isMasterCacheValid = true
            isMasterCacheValid: true,
            // Java field initializer: private boolean isTitleStyleCacheValid = true
            isTitleStyleCacheValid: true,
            // Java field initializer: private boolean isBodyStyleCacheValid = true
            isBodyStyleCacheValid: true,
            // Java field initializer: private boolean isOtherStyleCacheValid = true
            isOtherStyleCacheValid: true,
            // Java field initializer: private boolean isDefaultStyleCacheValid = true
            isDefaultStyleCacheValid: true,
            // Java field initializer: private boolean isCleared = true
            isCleared: true,
        }
    }
}

// Java: public class KmoTextStyleManager
impl KmoTextStyleManager {
    // Java: KmoTextStyleManager()
    pub fn new() -> Self {
      // Java: （隐式无参构造器：六个缓存有效位与 isCleared 默认 true，六个 ThreadLocal 默认未 set）
      Self::default()
    }

    // Java: public void clear()
    pub fn clear(&mut self) {
      // Java: this.phStyleLayoutThreadLocal.remove();
      // DEFERRED_NON_FUNCTION: `phStyleLayoutThreadLocal` 类型缺失（见字段注记）。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      // Java: this.phStyleMasterThreadLocal.remove();
      // DEFERRED_NON_FUNCTION: 同上。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      // Java: this.titleStyleThreadLocal.remove();
      // DEFERRED_NON_FUNCTION: 同上。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      // Java: this.bodyStyleThreadLocal.remove();
      // DEFERRED_NON_FUNCTION: 同上。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      // Java: this.otherStyleThreadLocal.remove();
      // DEFERRED_NON_FUNCTION: 同上。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      // Java: this.defaultStyleThreadLocal.remove();
      // DEFERRED_NON_FUNCTION: 同上。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      // Java: this.isLayoutCacheValid = false;
      self.isLayoutCacheValid = false;
      // Java: this.isMasterCacheValid = false;
      self.isMasterCacheValid = false;
      // Java: this.isTitleStyleCacheValid = false;
      self.isTitleStyleCacheValid = false;
      // Java: this.isBodyStyleCacheValid = false;
      self.isBodyStyleCacheValid = false;
      // Java: this.isOtherStyleCacheValid = false;
      self.isOtherStyleCacheValid = false;
      // Java: this.isDefaultStyleCacheValid = false;
      self.isDefaultStyleCacheValid = false;
      // Java: this.isCleared = true;
      self.isCleared = true;
    }

    // Java: public TextListStyle titleStyle()
    // DEFERRED_NON_FUNCTION: 返回类型 `TextListStyle` 默认构建下不可命名 ⇒ 方法**签名本身**无法表达。
    //   Java 体：`return (TextListStyle)this.titleStyleThreadLocal.get();`（`KmoTextStyleManager.java:46-48`）。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: public TextListStyle bodyStyle()
    // DEFERRED_NON_FUNCTION: 同上。Java 体：`return (TextListStyle)this.bodyStyleThreadLocal.get();`
    //   （`KmoTextStyleManager.java:50-52`）。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: public TextListStyle otherStyle()
    // DEFERRED_NON_FUNCTION: 同上。Java 体：`return (TextListStyle)this.otherStyleThreadLocal.get();`
    //   （`KmoTextStyleManager.java:54-56`）。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: public TextListStyle defaultStyle()
    // DEFERRED_NON_FUNCTION: 同上。Java 体：`return (TextListStyle)this.defaultStyleThreadLocal.get();`
    //   （`KmoTextStyleManager.java:58-60`）。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: public boolean isCleared()
    pub fn isCleared(&self) -> bool {
      // Java: return this.isCleared;
      self.isCleared
    }

    // Java: public void meansureTextStyle(int typePh, int indexPh, KmoShowShape shape)
    pub fn meansureTextStyle__int_int_kmoshowshape(&mut self, typePh: i32, indexPh: i32, shape: Option<&KmoShowShape>) {
      // Java: if (null != shape)
      if let Some(shape) = shape {
         // Java: this.init(typePh, shape.parent());
         self.init(typePh, shape.parent());
         // Java: if (-1 != typePh && 10 != typePh)
         if -1 != typePh && 10 != typePh {
            // Java: int key = this.key(typePh, indexPh);
            let _key = self.key(typePh, indexPh);
            // Java: Map<Integer, TextListStyle> phStyleLayout = this.phStyleLayout();
            // Java: if (!phStyleLayout.containsKey(key))
            // Java: {
            // Java:    KmoShowShape shapeLayout = PlaceHolderMerger.findLayoutPhShape(shape);
            // Java:    if (null != shapeLayout)
            // Java:    {
            // Java:       TextBody txBody = shapeLayout.getDgShape().textBody();
            // Java:       if (null != txBody && txBody.hasListStyle())
            // Java:       {
            // Java:          phStyleLayout.put(key, TextListStyle.open(txBody.listStyle().save()));
            // Java:       }
            // Java:    }
            // Java: }
            // Java: Map<Integer, TextListStyle> phStyleMaster = this.phStyleMaster();
            // Java: if (!phStyleMaster.containsKey(key))
            // Java: {
            // Java:    KmoSlideMaster master = this.findMaster(shape.parent());
            // Java:    if (null != master)
            // Java:    {
            // Java:       KmoShapeTree shpTree = master.film().spTree();
            // Java:       if (null != shpTree)
            // Java:       {
            // Java:          int type = typePh;
            // Java:          if (2 == typePh) { type = 0; }
            // Java:          else if (3 == typePh || 8 == typePh) { type = 1; }
            // Java:          KmoShowShape shapeMaster = shpTree.findPhByType(type);
            // Java:          if (null != shapeMaster)
            // Java:          {
            // Java:             TextBody txBody = shapeMaster.getDgShape().textBody();
            // Java:             if (null != txBody && txBody.hasListStyle())
            // Java:             {
            // Java:                phStyleMaster.put(type, TextListStyle.open(txBody.listStyle().save()));
            // Java:             }
            // Java:          }
            // Java:       }
            // Java:    }
            // Java: }
            // DEFERRED_NON_FUNCTION: 上述两个缓存块四类依赖同时缺失 ——
            //   (1) `phStyleLayout()`/`phStyleMaster()` 的值类型 `TextListStyle` 不可命名；
            //   (2) `PlaceHolderMerger.findLayoutPhShape` 绑定 `util/layout_shape::KmoShowShape` 影子，
            //       与本方法的 `show/app/shape::KmoShowShape` 真身不同型；
            //   (3) `DgShape.textBody()`（`cn.wps.drawing.app.shape.DgShape`）默认构建下不可命名；
            //   (4) `KmoSlideMaster`/`KmoShapeTree.findPhByType` 母版面不可达。
            //   SP3b 落地真文本模型后按上方 Java 体逐句接回（`key` 已由本波真实算出）。
            //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
         }
      }
    }

    // Java: public void meansureTextStyle(int typePh, int indexPh, KmoContext context)
    pub fn meansureTextStyle__int_int_kmocontext(&mut self, typePh: i32, indexPh: i32, context: Option<&super::KmoContext::KmoContext>) {
      // Java: if (null != context && null != context.getContext())
      if let Some(context) = context {
         if let Some(objContext) = context.getContext() {
            // Java: this.init(typePh, context.getContext());
            self.init(typePh, Some(std::rc::Rc::as_ptr(&objContext)));
            // Java: if (-1 != typePh && 10 != typePh)
            if -1 != typePh && 10 != typePh {
               // Java: int key = this.key(typePh, indexPh);
               let _key = self.key(typePh, indexPh);
               // Java: Map<Integer, TextListStyle> phStyleLayout = this.phStyleLayout();
               // Java: if (!phStyleLayout.containsKey(key))
               // Java: {
               // Java:    KmoShowShape shapeLayout = null;
               // Java:    ISlide objContext = context.getContext();
               // Java:    if (null != objContext && 0 == objContext.type())
               // Java:    {
               // Java:       KmoSlide slide = (KmoSlide)objContext;
               // Java:       KmoSlideLayout layout = slide.layout();
               // Java:       if (indexPh != -1) { shapeLayout = layout.spTree().findPhByIndex(indexPh); }
               // Java:       else if (0 <= typePh && typePh <= 8) { shapeLayout = layout.spTree().findPhByIndex(0); }
               // Java:    }
               // Java:    if (null != shapeLayout)
               // Java:    {
               // Java:       TextBody txBody = shapeLayout.getDgShape().textBody();
               // Java:       if (null != txBody && txBody.hasListStyle())
               // Java:       {
               // Java:          phStyleLayout.put(key, TextListStyle.open(txBody.listStyle().save()));
               // Java:       }
               // Java:    }
               // Java: }
               // Java: Map<Integer, TextListStyle> phStyleMaster = this.phStyleMaster();
               // Java: if (!phStyleMaster.containsKey(key))
               // Java: {
               // Java:    KmoSlideMaster master = this.findMaster(context.getContext());
               // Java:    if (null != master)
               // Java:    {
               // Java:       KmoShapeTree shpTree = master.film().spTree();
               // Java:       if (null != shpTree)
               // Java:       {
               // Java:          int type = typePh;
               // Java:          if (2 == typePh) { type = 0; }
               // Java:          else if (3 == typePh || 8 == typePh) { type = 1; }
               // Java:          KmoShowShape shapeMaster = shpTree.findPhByType(type);
               // Java:          if (null != shapeMaster)
               // Java:          {
               // Java:             TextBody txBody = shapeMaster.getDgShape().textBody();
               // Java:             if (null != txBody && txBody.hasListStyle())
               // Java:             {
               // Java:                phStyleMaster.put(type, TextListStyle.open(txBody.listStyle().save()));
               // Java:             }
               // Java:          }
               // Java:       }
               // Java:    }
               // Java: }
               // DEFERRED_NON_FUNCTION: 与 shape 版同源四类依赖缺失，且额外需要
               //   `(KmoSlide)objContext`（`KmoSlide.layout().spTree().findPhByIndex()`）的**具体类强转**，
               //   默认构建下 `KmoSlide` 不可命名。`key` 已由本波真实算出。
               //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
            }
         }
      }
    }

    // Java: public TextListStyle getStyleInLayoutPh(int typePh, int indexPh)
    // DEFERRED_NON_FUNCTION: 返回类型 `TextListStyle` 不可命名。Java 体：
    //   `int key = this.key(typePh, indexPh);
    //    Map<Integer, TextListStyle> phStyleLayout = this.phStyleLayout();
    //    return null != phStyleLayout && phStyleLayout.containsKey(key) ? (TextListStyle)phStyleLayout.get(key) : null;`
    //   （`KmoTextStyleManager.java:165-169`）。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: public TextListStyle getStyleInMasterPh(int typePh)
    // DEFERRED_NON_FUNCTION: 返回类型 `TextListStyle` 不可命名。Java 体：
    //   `int type = typePh; if (2 == typePh) { type = 0; } else if (3 == typePh || 8 == typePh) { type = 1; }
    //    Map<Integer, TextListStyle> phStyleMaster = this.phStyleMaster();
    //    return phStyleMaster.containsKey(type) ? (TextListStyle)phStyleMaster.get(type) : null;`
    //   （`KmoTextStyleManager.java:171-181`）。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: public TextListStyle getStyleInMaster(int typePh)
    // DEFERRED_NON_FUNCTION: 返回类型 `TextListStyle` 不可命名。Java 体：
    //   `if (0 != typePh && 2 != typePh)
    //       return 3 != typePh && 8 != typePh && 1 != typePh ? this.otherStyle() : this.bodyStyle();
    //    else return this.titleStyle();`
    //   （`KmoTextStyleManager.java:183-189`）。该「四级回退链」正是 SP3 子计划 §3.3 的 ★★ 验收项，
    //   待 `titleStyle()/bodyStyle()/otherStyle()` 随真 `TextListStyle` 落地后本方法可一句直译。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: private KmoSlideMaster findMaster(ISlide slide)
    // DEFERRED_NON_FUNCTION: 返回类型 `KmoSlideMaster` 默认构建下不可命名 ⇒ 方法**签名本身**无法表达。
    //   Java 体（`KmoTextStyleManager.java:191-208`）按 `slide.type()` 三路取母版：
    //   `case 0: KmoSlide slideTmp = (KmoSlide)slide; if (null == slideTmp.layout()) return null;
    //            return slideTmp.layout().master();
    //    case 1: KmoSlideLayout layout = (KmoSlideLayout)slide; return layout.master();
    //    case 2: return (KmoSlideMaster)slide;
    //    default: return null;`
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: private Map<Integer, TextListStyle> phStyleLayout()
    // DEFERRED_NON_FUNCTION: 返回类型 `Map<Integer, TextListStyle>` 不可命名。Java 体：
    //   `Map<Integer, TextListStyle> mapStyle = (Map)this.phStyleLayoutThreadLocal.get();
    //    if (null == mapStyle || !this.isLayoutCacheValid) {
    //       mapStyle = new HashMap(); this.phStyleLayoutThreadLocal.set(mapStyle); this.isLayoutCacheValid = true; }
    //    return mapStyle;`（`KmoTextStyleManager.java:210-219`）。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: private Map<Integer, TextListStyle> phStyleMaster()
    // DEFERRED_NON_FUNCTION: 同上。Java 体见 `KmoTextStyleManager.java:221-230`（`isMasterCacheValid` 同构）。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: private int key(int typePh, int indexPh)
    pub fn key(&self, typePh: i32, indexPh: i32) -> i32 {
      // Java: return typePh << 16 | 255 & indexPh;
      //
      // rust: Java 运算符优先级为 `(typePh << 16) | (255 & indexPh)`（`<<` 高于 `|`，`&` 高于 `|`），
      //   Rust 同优先级规则一致，故直译并显式加括号固定语义。
      (typePh << 16) | (255 & indexPh)
    }

    // Java: private void init(int typePh, ISlide slide)
    pub fn init(&mut self, typePh: i32, slide: Option<*const dyn ISlide>) {
      // Java: if (0 <= typePh)
      if 0 <= typePh {
         // Java: if (0 != typePh && 2 != typePh)
         if 0 != typePh && 2 != typePh {
            // Java: if (3 != typePh && 8 != typePh && 1 != typePh)
            if 3 != typePh && 8 != typePh && 1 != typePh {
               // Java: this.initOtherStyle(slide);
               self.initOtherStyle(slide);
            // Java: else
            } else {
               // Java: this.initBodyStyle(slide);
               self.initBodyStyle(slide);
            }
         // Java: else
         } else {
            // Java: this.initTitleStyle(slide);
            self.initTitleStyle(slide);
         }
      // Java: else
      } else {
         // Java: this.initDeault(slide);
         self.initDeault(slide);
      }
      // Java: this.isCleared = false;
      self.isCleared = false;
    }

    // Java: private void initTitleStyle(ISlide slide)
    //
    // rust: 偏差登记 —— Java 形参是接口 `ISlide`；Rust 侧具体类不可命名，且 `KmoShowShape.parent()`
    //   给出的是 `*const dyn ISlide`（`KmoShowShape.rs:846`），故形参按裸指针承载（与
    //   `KmoFilm.parent` 同一策略）。方法体因 `KmoSlideMaster`/`TextListStyle` 不可命名而 DEFERRED。
    pub fn initTitleStyle(&mut self, slide: Option<*const dyn ISlide>) {
      // Java: if (null == this.titleStyleThreadLocal.get() || !this.isTitleStyleCacheValid)
      // Java: {
      // Java:    KmoSlideMaster master = this.findMaster(slide);
      // Java:    if (null != master)
      // Java:    {
      // Java:       this.titleStyleThreadLocal.set(TextListStyle.open(master.titleStyle().save()));
      // Java:       this.isTitleStyleCacheValid = true;
      // Java:    }
      // Java: }
      // DEFERRED_NON_FUNCTION: `titleStyleThreadLocal` 的值类型 `TextListStyle`、`findMaster()` 的
      //   返回类型 `KmoSlideMaster` 在默认构建下均不可命名。Java 体见
      //   `KmoTextStyleManager.java:254-263`。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: private void initBodyStyle(ISlide slide)
    pub fn initBodyStyle(&mut self, slide: Option<*const dyn ISlide>) {
      // Java: if (null == this.bodyStyleThreadLocal.get() || !this.isBodyStyleCacheValid)
      // Java: {
      // Java:    KmoSlideMaster master = this.findMaster(slide);
      // Java:    if (null != master)
      // Java:    {
      // Java:       this.bodyStyleThreadLocal.set(TextListStyle.open(master.bodyStyle().save()));
      // Java:       this.isBodyStyleCacheValid = true;
      // Java:    }
      // Java: }
      // DEFERRED_NON_FUNCTION: 同 `initTitleStyle()`。Java 体见 `KmoTextStyleManager.java:265-274`。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: private void initOtherStyle(ISlide slide)
    pub fn initOtherStyle(&mut self, slide: Option<*const dyn ISlide>) {
      // Java: if (null == this.otherStyleThreadLocal.get() || !this.isOtherStyleCacheValid)
      // Java: {
      // Java:    KmoSlideMaster master = this.findMaster(slide);
      // Java:    if (null != master)
      // Java:    {
      // Java:       this.otherStyleThreadLocal.set(TextListStyle.open(master.otherStyle().save()));
      // Java:       this.isOtherStyleCacheValid = true;
      // Java:    }
      // Java: }
      // DEFERRED_NON_FUNCTION: 同 `initTitleStyle()`。Java 体见 `KmoTextStyleManager.java:276-285`。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: private void initDeault(ISlide slide)
    pub fn initDeault(&mut self, slide: Option<*const dyn ISlide>) {
      // Java: if (null == this.defaultStyleThreadLocal.get() || !this.isDefaultStyleCacheValid)
      // Java: {
      // Java:    TextListStyle ts = slide.getPresentation().defaultTextListStyle();
      // Java:    if (null != ts)
      // Java:    {
      // Java:       this.defaultStyleThreadLocal.set(TextListStyle.open(ts.save()));
      // Java:       this.isDefaultStyleCacheValid = true;
      // Java:    }
      // Java:    else if (31 == slide.getPresentation().getFileFormat())
      // Java:    {
      // Java:       KmoSlideMaster master = this.findMaster(slide);
      // Java:       if (null != master)
      // Java:       {
      // Java:          this.defaultStyleThreadLocal.set(TextListStyle.open(master.otherStyle().save()));
      // Java:          this.isDefaultStyleCacheValid = true;
      // Java:       }
      // Java:    }
      // Java: }
      // DEFERRED_NON_FUNCTION: `TextListStyle`/`KmoSlideMaster` 不可命名，且
      //   `slide.getPresentation().defaultTextListStyle()`/`getFileFormat()` 属 SP4 正在落地的
      //   `KmoPresentation`。Java 体见 `KmoTextStyleManager.java:287-302`。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }
}
