#![allow(non_snake_case)]

// Java: package cn.wps.show.app.slide
// Java: import cn.wps.memor4j.KmoAtomData
use kfc::cn::wps::memor4j::KmoAtomData::{KmoAtomData, KmoAtomDataSub};
// Java: import cn.wps.show.app.theme.KmoTheme
// ★ 阻塞：`show::app::theme` 在默认构建下被 `#[cfg(feature = "full")]` 关（假的编译面，形态 D）。
//   放开它会把整条未装配的 theme 链（ColorScheme/EffectStyle/FontScheme/StyleMatrix …）拖进默认构建，
//   超出 SP1 边界。`theme` 字段属 SP3 波次，此处按「类型未就绪」停车。
//   owner: show.app.theme    wave: M-PPT-KMOSLIDE-SP3
// Java: import cn.wps.show.data.CommonSlideData
use crate::cn::wps::show::data::CommonSlideData::CommonSlideData;

// Java: class FilmAtomData extends KmoAtomData
pub struct FilmAtomData {
    // Java: extends KmoAtomData
    pub base: KmoAtomData,
    // Java field: protected CommonSlideData cSlide = CommonSlideData.create();
    //
    // rust: Java 侧 `cSlide` 是普通字段，`copy()` / `protocol()` 以 `&this` 调 `cSlide.save()`；
    //       Rust `CommonSlideData::save()` 需要 `&mut self`，故以 `RefCell` 承载同一对象
    //       （与 `SlideAtomData.slide` 同处置，台账登记为 rust: 偏差）。
    pub cSlide: std::cell::RefCell<CommonSlideData>,
    // Java field: protected KmoTheme theme
    //
    // ★ SP1 阻塞点（登记，不擅自改语义）：
    //   Java `FilmAtomData.copy()` 直接赋引用 `cpy.theme = this.theme;`（引用语义，零成本）。
    //   Rust 侧要表达「可共享的主题引用」有两条路：
    //     (a) `Option<Rc<RefCell<KmoTheme>>>` —— 语义最接近 Java 引用，且 `KmoPane`
    //         `getTheme()` 与 `setTheme()` 都能无拷贝共享同一份主题；**推荐**。
    //     (b) `Option<KmoTheme>` + `KmoTheme: Clone` —— 但当前 `KmoTheme` 持有
    //         `Option<Box<dyn ThemePathUpdaterTrait>>`（trait object 不可 Clone）与
    //         尚未泛型化的 `KmoAtom`，强行 Clone 需要改 `KmoTheme` 本体（超出 SP1 边界）。
    //   在 `KmoTheme` 装配波次（SP3）裁定前，本字段先按 (a) 的**类型**登记，装配时再接线：
    //     owner: show.app.theme    wave: M-PPT-KMOSLIDE-SP3
    // rust: 待 SP3 放开 `app::theme` 后按 `Option<Rc<RefCell<KmoTheme>>>` 装配（Java 引用语义）。
    pub theme: Option<()>,
}

// Java: class FilmAtomData extends KmoAtomData
impl FilmAtomData {
    // Java: FilmAtomData()
    pub fn new() -> Self {
      // Java: super();
      let base = KmoAtomData::new();
      // Java: protected CommonSlideData cSlide = CommonSlideData.create();
      let cSlide = std::cell::RefCell::new(CommonSlideData::create());
      // Java: protected KmoTheme theme;  —— 默认 null
      // Java: protected KmoTheme theme;  —— 默认 null（SP3 装配）
      let theme = None;
      FilmAtomData {
          base,
          cSlide,
          theme,
      }
    }

    // Java: public KmoAtomData copy()
    pub fn copy(&self) -> FilmAtomData {
      // Java: FilmAtomData cpy = new FilmAtomData();
      let mut cpy = FilmAtomData::new();
      // Java: cpy.cSlide.load(this.cSlide.save());
      let saved = self.cSlide.borrow_mut().save();
      cpy.cSlide.borrow_mut().load_bin(saved.as_ref());
      // Java: cpy.theme = this.theme;
      // rust: `Option<()>` 占位不可 clone（SP3 换 `Rc` 后为引用赋值）
      cpy.theme = self.theme;
      // Java: return cpy;
      cpy
    }
}

// Java: extends KmoAtomData
impl KmoAtomDataSub for FilmAtomData {
    fn as_base(&self) -> &KmoAtomData {
        &self.base
    }
    fn as_base_mut(&mut self) -> &mut KmoAtomData {
        &mut self.base
    }
    // Java: public KmoAtomData copy()
    fn copy(&self) -> Self {
        FilmAtomData::copy(self)
    }
}
