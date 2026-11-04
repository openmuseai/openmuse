#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: package cn.wps.show.app.slide
// Java: import cn.wps.memor4j.KmoAtomData
use kfc::cn::wps::memor4j::KmoAtomData::{KmoAtomData, KmoAtomDataSub};
// Java: import cn.wps.show.data.slidelayouts.SlideLayout
use crate::cn::wps::show::data::slidelayouts::SlideLayout::SlideLayout;

// Java: class LayoutAtomData extends KmoAtomData
pub struct LayoutAtomData {
    // Java: extends KmoAtomData
    pub base: KmoAtomData,
    // Java field: protected SlideLayout layout = SlideLayout.create();
    //
    // rust: Java 侧 `layout` 是普通字段，`copy()` 以 `&this` 调 `this.layout.save()`；
    //       Rust `SlideLayout::save()` 需要 `&mut self`，而 `copy()` 按 Java 签名是 `&self`，
    //       故以 `RefCell` 承载同一对象（与 `SlideAtomData.slide` / `ShowShapeAtomData.shape`
    //       同策略，台账登记为 rust: 偏差）。
    pub layout: std::cell::RefCell<SlideLayout>,
    // Java field: protected KmoSlideMaster master
    //
    // ★ SP5 阻塞点（登记，不擅自改语义）：
    //   Java `master` 是 `KmoSlideMaster` 的**共享引用**：`KmoSlideLayout.master()` 直接返回它、
    //   `setMaster()` 直接赋值；`KmoSlideMaster` 自身又持有 `List<KmoSlideLayout>`，即
    //   「一主多版、多页共享同一 layout / master 实例」。`LayoutAtomData.copy()` 里同样是
    //   `cpy.master = this.master`（引用复制，不是深拷贝）。
    //   目标 Rust 类型：`Option<Rc<RefCell<KmoSlideMaster>>>`（正向共享引用，见
    //   M_PPT_KMOSLIDE_PLAN §5-R1 裁定与 SP5 §5 接线图）。
    //   但 `app/slide/KmoSlideMaster.rs` 目前是 60 行 / 48 `todo!` 的**未登记死壳**，其依赖
    //   `KmoTextStyleManager` / `KmoTiming` / `KmoTransition` / `KmoAnimOperator` /
    //   `TransitionUtil` 在默认构建下**整类不存在**（`app/text`、`app/timing` 为空目录/缺失、
    //   `app/util/mod.rs` 不存在），SP5 不得凭空造壳（SP5 §7-R2「若为死壳，`master()` 先返回
    //   真实槽位 + 登记」）。故本字段先落**真实类型槽位**，装配归 master 波次：
    //     owner: show.app.slide    wave: M-PPT-MASTER-ASM
    // rust: 待 M-PPT-MASTER-ASM 落地 `KmoSlideMaster` 后按 `Option<Rc<RefCell<KmoSlideMaster>>>`
    //       装配（Java 引用语义）；`KmoSlideLayout.master()/setMaster()` 随之转真。
    pub master: Option<()>,
}

// Java: class LayoutAtomData extends KmoAtomData
impl LayoutAtomData {
    // Java: LayoutAtomData()
    pub fn new() -> Self {
      // Java: super();
      let base = KmoAtomData::new();
      // Java: protected SlideLayout layout = SlideLayout.create();
      let layout = std::cell::RefCell::new(SlideLayout::create());
      // Java: protected KmoSlideMaster master;  —— 默认 null
      let master = None;
      LayoutAtomData {
          base,
          layout,
          master,
      }
    }

    // Java: public KmoAtomData copy()
    pub fn copy(&self) -> LayoutAtomData {
      // Java: LayoutAtomData cpy = new LayoutAtomData();
      let mut cpy = LayoutAtomData::new();
      // Java: cpy.layout.load(this.layout.save());
      let bin = self.layout.borrow_mut().save();
      cpy.layout.borrow_mut().load(bin.as_ref());
      // Java: cpy.master = this.master;
      // rust: `Option<()>` 占位不可共享（M-PPT-MASTER-ASM 落地 `KmoSlideMaster` 后改为
      //       `Weak`/`Rc` 引用赋值，语义同 Java）。
      cpy.master = self.master;
      // Java: return cpy;
      cpy
    }
}

// Java: extends KmoAtomData
impl KmoAtomDataSub for LayoutAtomData {
    fn as_base(&self) -> &KmoAtomData {
        &self.base
    }
    fn as_base_mut(&mut self) -> &mut KmoAtomData {
        &mut self.base
    }
    // Java: public KmoAtomData copy()
    fn copy(&self) -> Self {
        LayoutAtomData::copy(self)
    }
}
