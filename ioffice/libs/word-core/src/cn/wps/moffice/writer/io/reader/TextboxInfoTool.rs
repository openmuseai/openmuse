// Java: cn.wps.moffice.writer.io.reader.TextboxInfoTool
use std::cell::RefCell;
use std::collections::{BTreeMap, HashSet};
use std::sync::{Arc, Mutex};
use drawing::cn::wps::moffice::drawing::Shape::Shape;
use drawing::cn::wps::moffice::drawing::textbox::Text::Text;
use crate::cn::wps::moffice::writer::data::PLCTextboxText::TextboxTextNode;

thread_local! {
    static MAIN_DOC: RefCell<BTreeMap<i32, Arc<Text>>> = RefCell::new(BTreeMap::new());
    static HEADER_DOC: RefCell<BTreeMap<i32, Arc<Text>>> = RefCell::new(BTreeMap::new());
    static NODE_MAP: RefCell<BTreeMap<i32, TextboxTextNode>> = RefCell::new(BTreeMap::new());
}

fn text_mut(text: &Arc<Text>) -> &mut Text {
    unsafe { &mut *(Arc::as_ptr(text) as *mut Text) }
}

fn shape_id(shape: &dyn std::any::Any) -> Option<i32> {
    if let Some(shape) = shape.downcast_ref::<Mutex<Shape>>() {
        return Some(shape.lock().expect("shape").getShapeID());
    }
    shape.downcast_ref::<Shape>().map(|shape| shape.getShapeID())
}

fn clear_shape_text(shape: &dyn std::any::Any) {
    if let Some(shape) = shape.downcast_ref::<Mutex<Shape>>() {
        shape.lock().expect("shape").setText(None);
    }
}

// Java: public class TextboxInfoTool
pub struct TextboxInfoTool;

impl TextboxInfoTool {
    // Java: public static void reset()
    pub fn reset() {
        MAIN_DOC.with(|map| map.borrow_mut().clear());
        HEADER_DOC.with(|map| map.borrow_mut().clear());
        NODE_MAP.with(|map| map.borrow_mut().clear());
    }

    // Java: public static void collect(Shape shape, int type)
    pub fn collect(shape: &Arc<Mutex<Shape>>, doc_type: i32) {
        let shape_id = shape.lock().expect("shape").getShapeID();
        let text = shape.lock().expect("shape").getTextObj()
            .and_then(|arc| arc.downcast::<Text>().ok());
        let Some(text) = text else { return; };
        let store = |map: &RefCell<BTreeMap<i32, Arc<Text>>>| { map.borrow_mut().insert(shape_id, text); };
        if doc_type == 2 { HEADER_DOC.with(store); } else { MAIN_DOC.with(store); }
    }

    // Java: public static void collectTextNode(int shapeId, TextboxTextNode node)
    pub fn collectTextNode(shape_id: i32, node: TextboxTextNode) {
        NODE_MAP.with(|map| { map.borrow_mut().insert(shape_id, node); });
    }

    // Java: public static void run()
    pub fn run() {
        MAIN_DOC.with(|map| correct(&mut map.borrow_mut()));
        HEADER_DOC.with(|map| correct(&mut map.borrow_mut()));
        MAIN_DOC.with(|map| process(&map.borrow()));
        HEADER_DOC.with(|map| process(&map.borrow()));
        MAIN_DOC.with(|text_map| {
            NODE_MAP.with(|node_map| correct_text(&text_map.borrow(), &node_map.borrow()));
        });
        Self::reset();
    }
}

fn correct(map: &mut BTreeMap<i32, Arc<Text>>) {
    let keys: Vec<i32> = map.keys().copied().collect();
    for key in keys {
        correct_line(map, &mut HashSet::new(), key);
    }
}

fn correct_line(map: &BTreeMap<i32, Arc<Text>>, id_set: &mut HashSet<i32>, id: i32) {
    let Some(text) = map.get(&id).cloned() else { return; };
    id_set.insert(id);
    if let Some(next_id) = text_mut(&text).getTextNextObj() {
        if id_set.contains(&next_id) {
            text_mut(&text).setTextNext_nullable(None);
        } else {
            correct_line(map, id_set, next_id);
        }
    }
}

fn process(map: &BTreeMap<i32, Arc<Text>>) {
    let keys: Vec<i32> = map.keys().copied().collect();
    for key in keys {
        let text = map.get(&key).expect("text").clone();
        let shpid = text.getShape().as_ref().and_then(|shape| shape_id(shape.as_ref())).unwrap_or(0);
        update(map, &text, shpid, 0);
    }
}

fn update(map: &BTreeMap<i32, Arc<Text>>, text: &Arc<Text>, shpid: i32, seq: i32) {
    let seq_obj = text.getTextLinkedSeqObj();
    if seq_obj.is_none() || seq_obj.unwrap_or(0) < seq {
        let next_id = text.getTextNextObj();
        if next_id.is_some() || seq > 0 {
            text_mut(text).setTextLinkedID(shpid);
            text_mut(text).setTextLinkedSeq(seq);
        }
        if let Some(next_id) = next_id {
            if let Some(next) = map.get(&next_id) {
                update(map, next, shpid, seq + 1);
            }
        }
    }
}

fn correct_text(text_map: &BTreeMap<i32, Arc<Text>>, node_map: &BTreeMap<i32, TextboxTextNode>) {
    for key in text_map.keys().copied() {
        let text = text_map.get(&key).expect("text");
        let Some(shape) = text.getShape().clone() else { return; };
        if !node_map.contains_key(&key) && text.getTextLinkedIDObj().is_none() {
            clear_shape_text(shape.as_ref());
        }
    }
}
