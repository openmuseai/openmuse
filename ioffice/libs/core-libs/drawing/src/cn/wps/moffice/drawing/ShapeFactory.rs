#![allow(non_snake_case, dead_code, unused_imports)]

// Java: import cn.wps.moffice.drawing.textbox.Text
use crate::cn::wps::moffice::drawing::textbox::Text::Text;
// Java: import cn.wps.moffice.drawing.Shape
use crate::cn::wps::moffice::drawing::Shape::Shape;
// Java: import cn.wps.moffice.drawing.GroupShape
use crate::cn::wps::moffice::drawing::GroupShape::GroupShape;
// Java: import cn.wps.moffice.drawing.IDrawingContainer
// LLM Rust Dependency Not Yet Rewritten: IDrawingContainer module is commented out in mod.rs

use std::sync::{Arc, Mutex, OnceLock};
use std::any::Any;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java field: ShapeFactory mFactory
fn get_mFactory() -> &'static Mutex<Option<ShapeFactory>> {
    static M_FACTORY: OnceLock<Mutex<Option<ShapeFactory>>> = OnceLock::new();
    M_FACTORY.get_or_init(|| Mutex::new(None))
}

// Java ShapeFactoryDoc replaces the static factory. drawing cannot name TextFrame,
// so the word factory installs the createText body here.
fn create_text_hook() -> &'static Mutex<Option<fn(Element) -> Option<Text>>> {
    static HOOK: OnceLock<Mutex<Option<fn(Element) -> Option<Text>>>> = OnceLock::new();
    HOOK.get_or_init(|| Mutex::new(None))
}

// Java: public class ShapeFactory
pub struct ShapeFactory {
}

// Java: public class ShapeFactory
impl ShapeFactory {
    // Java: ShapeFactory()
    pub fn new() -> Self {
        // Java: super();
        ShapeFactory {}
    }

    // Java: static ShapeFactory getShapeFactory()
    pub fn getShapeFactory() -> ShapeFactory {
        let mut factory = get_mFactory().lock().unwrap();
        // Java: if (mFactory == null)
        if factory.is_none() {
            // Java: createShapeFactory();
            *factory = Some(ShapeFactory::new());
        }
        // Java: return mFactory;
        ShapeFactory::new()
    }

    // Java: static void createShapeFactory()
    pub fn createShapeFactory() {
        // Java: mFactory = new ShapeFactory();
        let mut factory = get_mFactory().lock().unwrap();
        *factory = Some(ShapeFactory::new());
    }

    // Java: ShapeFactoryDoc.createShapeFactory installs the word createText override.
    pub fn install_create_text(create_text: fn(Element) -> Option<Text>) {
        *create_text_hook().lock().expect("createText") = Some(create_text);
    }

    // Java: Text createText(Shape shape)
    pub fn createText(&self, shape: Element) -> Option<Text> {
        // Java: ShapeFactory returns null; ShapeFactoryDoc returns new TextFrame(shape).
        if let Some(create_text) = *create_text_hook().lock().expect("createText") {
            return create_text(shape);
        }
        None
    }

    // Java: Shape createShape(IDrawingContainer drawingContainer)
    pub fn createShape(&self, drawingContainer: Element) -> Shape {
        // Java: return new Shape(drawingContainer);
        Shape::Shape_new(drawingContainer)
    }

    // Java: GroupShape createGroupShape(IDrawingContainer drawingContainer)
    pub fn createGroupShape(&self, drawingContainer: Element) -> GroupShape {
        // Java: return new GroupShape(drawingContainer);
        GroupShape::new(drawingContainer)
    }
}
