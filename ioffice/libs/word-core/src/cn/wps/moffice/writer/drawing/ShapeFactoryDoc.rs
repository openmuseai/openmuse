// Java: import cn.wps.moffice.drawing.Shape
use drawing::cn::wps::moffice::drawing::Shape::*;
// Java: import cn.wps.moffice.drawing.ShapeFactory
use drawing::cn::wps::moffice::drawing::ShapeFactory::*;
// Java: import cn.wps.moffice.drawing.textbox.Text
use drawing::cn::wps::moffice::drawing::textbox::Text::*;
// Java: import cn.wps.moffice.writer.drawing.TextFrame
// rust: `writer/drawing/TextFrame.rs` 尚未登记（其依赖未就绪）；`createText` 已合法延迟
//   ⇒ 导入一并延迟。
// rust: no-stmt

// java public class ShapeFactoryDoc extends ShapeFactory
// rust: 模板加的 `#[derive(Default)]` 删除 —— Java 无默认构造器，且父类
//   `ShapeFactory` 未实现 `Default`（core-libs/drawing 侧为 `Mutex<Option<..>>` 单例）。
pub struct ShapeFactoryDoc {
    // java ShapeFactory super_
    pub(crate) super_: ShapeFactory,
}

// java public class ShapeFactoryDoc extends ShapeFactory
impl ShapeFactoryDoc {
    // rust: 与 Java 隐式无参构造器对齐（父类 `ShapeFactory::new()` 已存在）。
    pub fn new() -> Self {
        Self { super_: ShapeFactory::new() }
    }

    // java public static void createShapeFactory()
    // rust: ★M-STAGE2 P1-D1 / T2 —— **保留工作 no-op（管线可达！）**。
    //   Java 原文 `mFactory = new ShapeFactoryDoc();`：`mFactory` 是
    //   `ShapeFactory` 的 `protected static` 字段，Java 子类可赋值。
    //   Rust 侧 `core-libs/drawing/ShapeFactory.rs` 把该字段建模为
    //   `static M_FACTORY: OnceLock<Mutex<Option<ShapeFactory>>>` + **私有**
    //   `get_mFactory()`，且 Rust 无继承（`ShapeFactoryDoc` 以 `super_` 组合建模，
    //   无法作为 `ShapeFactory` 装入单例）⇒ 当前无法忠实赋值。
    //   本方法由 `TextDocument.rs:1089`（`createAllSubDocument` 之后）在**管线路径**调用，
    //   且此前就地影子即为空实现、全部回归测试在此语义下通过
    //   ⇒ 保留 no-op 以维持行为，避免把可用路径变成 `todo!`。
    //   TODO(drawing-subsystem): 待 `core-libs/drawing` 导出 `mFactory` 写入接口
    //   且 `ShapeFactoryDoc` 改为 trait 对象分派后恢复真实赋值。
    // Java: public static void createShapeFactory() { mFactory = new ShapeFactoryDoc(); }
    pub fn createShapeFactory() {
        // Java: createText returns new TextFrame(shape), which is a Text.
        ShapeFactory::install_create_text(|shape| Some(Text::new_with_shape(shape)));
    }

    // java public Text createText(Shape shape)
    pub fn createText(&mut self, _shape: &Shape) -> Text {
        // Java: return new TextFrame(shape);
        Text::new()
    }

}

impl std::ops::Deref for ShapeFactoryDoc {
    type Target = ShapeFactory;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for ShapeFactoryDoc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
