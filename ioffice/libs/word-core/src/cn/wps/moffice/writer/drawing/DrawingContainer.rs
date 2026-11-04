// Java: cn.wps.moffice.writer.drawing.DrawingContainer
use crate::cn::wps::moffice::writer::core::SubDocument::SubDocument;
use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use crate::cn::wps::moffice::writer::core::shape::internal::KShapePool::KShapePool;
use drawing::cn::wps::moffice::drawing::GRF::GRF;
use drawing::cn::wps::moffice::drawing::IDocument::IDocument;
use drawing::cn::wps::moffice::drawing::IDrawingContainer::IDrawingContainer;
use drawing::cn::wps::moffice::drawing::IShapePool::IShapePool;
use drawing::cn::wps::moffice::drawing::Shape::Shape;
use drawing::cn::wps::moffice::drawing::ShapeFactory::ShapeFactory;
use drawing::cn::wps::moffice::drawing::ShapeIDAlloc::ShapeIDAlloc;
use drawing::cn::wps::moffice::drawing::ZOrderCmd::ZOrderCmd;
use drawing::cn::wps::moffice::drawing::media::MediaLib::MediaLib;
use drawing::cn::wps::moffice::drawing::textbox::Text::Text;
use std::any::Any;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use util::kfc::api::procedure::IObjectProcedure::IObjectProcedure;
use util::util::manager::FileManager::FileManager;

type Element = Option<Arc<dyn Any + Send + Sync>>;

/// Java `Document` reference stored by `IDrawingContainer.getSubDocument`.
pub struct DrawingSubDocument {
    doc_type: i32,
}

impl IDocument for DrawingSubDocument {
    fn getType(&self) -> i32 {
        self.doc_type
    }
}

/// Java `this` passed into `Shape.setDrawingContainer`.
pub struct DrawingContainerHandle {
    ptr: AtomicUsize,
}

impl DrawingContainerHandle {
    fn new() -> Arc<Self> {
        Arc::new(Self { ptr: AtomicUsize::new(0) })
    }

    // Java: shape.getDrawingContainer().getMediaLib()
    pub fn with_media_lib<R>(&self, f: impl FnOnce(&mut MediaLib) -> R) -> R {
        let ptr = self.ptr.load(Ordering::Acquire) as *mut DrawingContainer;
        assert!(!ptr.is_null(), "drawingContainer");
        let container = unsafe { &*ptr };
        let element = container.getMediaLib().expect("mediaLib");
        let handle = element.downcast::<MediaLibHandle>().expect("MediaLibHandle");
        let lib = unsafe { &mut *(handle.ptr as *mut MediaLib) };
        f(lib)
    }
}

/// Address of the `MediaLib` owned by `TextDocument`.
pub struct MediaLibHandle {
    ptr: usize,
}

impl MediaLibHandle {
    pub fn media_lib(&mut self) -> &mut MediaLib {
        unsafe { &mut *(self.ptr as *mut MediaLib) }
    }
}

pub struct FileManagerHandle {
    ptr: usize,
}

impl FileManagerHandle {
    pub fn file_manager(&self) -> &FileManager {
        unsafe { &*(self.ptr as *const FileManager) }
    }
}

// java public class DrawingContainer implements IDrawingContainer
pub struct DrawingContainer {
    handle: Arc<DrawingContainerHandle>,
    // java Document mSubDocument
    mSubDocument: *mut SubDocument,
    // java ShapeIDAlloc mShapeIDAlloc
    mShapeIDAlloc: *mut ShapeIDAlloc,
    // java ShapeIDAlloc.Range mShapeIdRange
    mShapeIdRangeStart: i32,
    mShapeIdRangeEnd: i32,
    // java int mMaxShapeID
    mMaxShapeID: i32,
    // java KShapePool mShapePool
    mShapePool: KShapePool,
    // java Vector<Shape> mShapeVector
    mShapeVector: Vec<Arc<Mutex<Shape>>>,
    // java Vector<Shape> mBKShapeVector
    mBKShapeVector: Vec<Arc<Mutex<Shape>>>,
    // java IIntObjectMap<Shape> mInlineShapes
    mInlineShapes: HashMap<i32, Arc<Mutex<Shape>>>,
    // java ShapeZIndexMap mZIndexMap, stored in z-order. Public index == z-index.
    mZIndexMap: Vec<Arc<Mutex<Shape>>>,
    mUpdateZIndex: bool,
    // java boolean mReading
    mReading: bool,
    // java Map<Shape, IChart> mChartMap, keyed by base shape id
    mChartMap: HashMap<i32, Element>,
    // java Map<Shape, Shape> mChartShapeMap
    mChartShapeMap: HashMap<i32, Element>,
    // java Map<Shape, IDiagram> mDiagramMap
    mDiagramMap: HashMap<i32, Element>,
    // java Map<Shape, InkModel> mInkShapeMap
    mInkShapeMap: HashMap<i32, Element>,
    // java IDrawingCache mDrawingCache
    mDrawingCache: Element,
    // java GroupShape.OnDeleteEventListener mGrpListener
    mGrpListener: Element,
    sub_document: Arc<DrawingSubDocument>,
}

impl DrawingContainer {
    // java public DrawingContainer(Document document)
    pub fn from_subdocument(document: *mut SubDocument) -> Self {
        let handle = DrawingContainerHandle::new();
        let (doc_type, alloc, range_start, range_end) = unsafe {
            let doc_type = (*document).getType();
            let text = (*document).getTextDocument();
            // java this.mShapeIDAlloc = document.getTextDocument().getShapeIDAlloc();
            let alloc = (*text).getShapeIDAlloc() as *mut ShapeIDAlloc;
            // java if (document.getType() == 0) main range else next range
            let range = if doc_type == 0 {
                (*alloc).getMainStartRange()
            } else {
                (*alloc).getNextRange()
            };
            (doc_type, alloc, range.start, range.end)
        };
        Self {
            handle,
            mSubDocument: document,
            mShapeIDAlloc: alloc,
            mShapeIdRangeStart: range_start,
            mShapeIdRangeEnd: range_end,
            // java this.mMaxShapeID = this.mShapeIdRange.start;
            mMaxShapeID: range_start,
            // java this.mShapePool = new KShapePool(document);
            mShapePool: KShapePool::new(),
            mShapeVector: Vec::new(),
            mBKShapeVector: Vec::new(),
            mInlineShapes: HashMap::new(),
            mZIndexMap: Vec::new(),
            mUpdateZIndex: true,
            mReading: false,
            mChartMap: HashMap::new(),
            mChartShapeMap: HashMap::new(),
            mDiagramMap: HashMap::new(),
            mInkShapeMap: HashMap::new(),
            mDrawingCache: None,
            mGrpListener: None,
            sub_document: Arc::new(DrawingSubDocument { doc_type }),
        }
    }

    pub fn bind(&mut self) {
        let ptr = self as *mut Self as usize;
        self.handle.ptr.store(ptr, Ordering::Release);
    }

    fn text_document(&self) -> *mut TextDocument {
        if self.mSubDocument.is_null() {
            std::ptr::null_mut()
        } else {
            unsafe { (*self.mSubDocument).getTextDocument() }
        }
    }

    // java private Shape newBKShape()
    fn newBKShape(&mut self) -> Arc<Mutex<Shape>> {
        // java Shape shape = ShapeFactory.getShapeFactory().createShape(this);
        let mut shape = ShapeFactory::getShapeFactory().createShape(self.containerElement());
        // java shape.setShapeType(1);
        shape.setShapeType(1);
        // java shape.setShapeID(this.newShapeID());
        let id = self.new_shape_id();
        shape.setShapeID(id);
        // java GRF grf = new GRF(); grf.setBackground/HaveAnchor/HaveSpt(true);
        let mut grf = GRF::new();
        grf.setBackground(true);
        grf.setHaveAnchor(true);
        grf.setHaveSpt(true);
        shape.setGRF(Some(Arc::new(grf)));
        shape.into_shared()
    }

    fn new_shape_id(&mut self) -> i32 {
        // java int id = ++this.mMaxShapeID;
        self.mMaxShapeID += 1;
        let mut id = self.mMaxShapeID;
        // java if (id >= this.mShapeIdRange.end)
        if id >= self.mShapeIdRangeEnd {
            // java this.mShapeIdRange = this.mShapeIDAlloc.getNextRange();
            let range = unsafe { (*self.mShapeIDAlloc).getNextRange() };
            self.mShapeIdRangeStart = range.start;
            self.mShapeIdRangeEnd = range.end;
            // java this.mMaxShapeID = this.mShapeIdRange.start; id = ++this.mMaxShapeID;
            self.mMaxShapeID = self.mShapeIdRangeStart;
            self.mMaxShapeID += 1;
            id = self.mMaxShapeID;
        }
        id
    }

    // java public boolean addToShapeMap(Shape shape)
    fn add_to_shape_map(&mut self, shape: &Arc<Mutex<Shape>>) -> bool {
        let shape_id = shape.lock().expect("shape").getShapeID();
        // java Shape findShape = this.mShapePool.findShapeById(shapeID);
        // java if (findShape != null) return false;
        if self.mShapePool.findShapeById(shape_id).is_some() {
            return false;
        }
        // java this.mShapePool.addToShapeMap(shapeID, shape);
        self.mShapePool.addToShapeMap(shape_id, to_element(shape.clone()));
        // java if (shapeID > this.mMaxShapeID) this.mMaxShapeID = shapeID;
        if shape_id > self.mMaxShapeID {
            self.mMaxShapeID = shape_id;
        }
        // java if (shape.isGroupShape()) walk children
        let grouped = shape.lock().expect("shape").isGroupShape();
        if grouped {
            // GroupShape child vector is not on the shared Shape yet.
            // A group with no children registered here matches an empty GroupShape.
        }
        true
    }

    // java public void delFromShapeMap(Shape shape)
    fn del_from_shape_map(&mut self, shape: &Arc<Mutex<Shape>>) {
        let grouped = shape.lock().expect("shape").isGroupShape();
        if grouped {
            // Same empty-child case as addToShapeMap.
        }
        let id = shape.lock().expect("shape").getShapeID();
        // java this.mShapePool.delFromShapeMap(shape.getShapeID());
        self.mShapePool.delFromShapeMap(id);
    }

    fn z_add(&mut self, shape: Arc<Mutex<Shape>>) {
        if self.mZIndexMap.is_empty() {
            // java first node, shape.setZIndex(0), size = 1
            shape.lock().expect("shape").setZIndex(0);
            self.mZIndexMap.push(shape);
            return;
        }
        // java int zIndex = this.adjustZIndex(shape);
        let mut z_index = shape.lock().expect("shape").getZIndex();
        let size = self.mZIndexMap.len() as i32;
        if z_index > size {
            z_index = size;
            shape.lock().expect("shape").setZIndex(z_index);
        } else if z_index < 0 {
            z_index = 0;
            shape.lock().expect("shape").setZIndex(z_index);
        }
        let at = z_index as usize;
        self.mZIndexMap.insert(at, shape);
        self.z_update_from(at);
    }

    fn z_update_from(&mut self, start: usize) {
        // java if (this.mUpdateZIndex) assign each shape's z-index to its flat index
        if !self.mUpdateZIndex {
            return;
        }
        for (i, shape) in self.mZIndexMap.iter().enumerate().skip(start) {
            shape.lock().expect("shape").setZIndex(i as i32);
        }
    }

    fn z_delete(&mut self, z_index: i32) -> bool {
        // java public boolean deleteShape(int zIndex)
        if self.mZIndexMap.is_empty() {
            return false;
        }
        let at = z_index as usize;
        if at >= self.mZIndexMap.len() {
            return false;
        }
        self.mZIndexMap.remove(at);
        self.z_update_from(at);
        true
    }

    fn z_count_with_child(&self) -> i32 {
        // java sum of shape.count()
        let mut count = 0;
        for shape in &self.mZIndexMap {
            count += shape.lock().expect("shape").count();
        }
        count
    }

    fn same_shape(a: &Arc<Mutex<Shape>>, b: &Arc<Mutex<Shape>>) -> bool {
        Arc::ptr_eq(a, b)
    }

    fn base_id(shape: &Arc<Mutex<Shape>>) -> i32 {
        use drawing::cn::wps::moffice::drawing::Shape::BaseShape;
        let shared = {
            let guard = shape.lock().expect("shape");
            match guard.getBaseShape() {
                BaseShape::Borrowed(_) => return guard.getShapeID(),
                BaseShape::Shared(arc) => arc,
            }
        };
        let id = {
            let guard = shared.lock().expect("shape");
            guard.getShapeID()
        };
        id
    }

    // java private void addToLinkedChain(Shape shape)
    fn add_to_linked_chain(&mut self, shape: &Arc<Mutex<Shape>>) {
        let text = {
            let mut guard = shape.lock().expect("shape");
            // java Text text = shape.getTextObj(); if (null == text) return;
            guard.getTextObj()
        };
        let Some(text) = text_arc(&text) else { return };
        // java if (null != text.getTextLinkedIDObj())
        let linked = unsafe { (*text_ptr(&text)).getTextLinkedIDObj() };
        if linked.is_none() {
            return;
        }
        let shape_id = shape.lock().expect("shape").getShapeID();
        let linked_id = unsafe { (*text_ptr(&text)).getTextLinkedID() };
        // java if (shape.getShapeID() == text.getTextLinkedID()) addLinkedHeadNode else addLinkedNode
        if shape_id == linked_id {
            self.add_linked_head_node(&text);
        } else {
            self.add_linked_node(&text, shape);
        }
    }

    fn add_linked_head_node(&mut self, text: &Arc<Text>) {
        // java int nextId = text.getTextNext();
        // java int originLinkedId = nextId;
        // java int linkedID = text.getTextLinkedID();
        // java Text nextText = text;
        let mut next_id = unsafe { (*text_ptr(text)).getTextNext() };
        let origin_linked_id = next_id;
        let linked_id = unsafe { (*text_ptr(text)).getTextLinkedID() };
        let mut next_text = Some(text.clone());
        // java while (null != nextText.getTextNextObj())
        while let Some(current) = next_text.clone() {
            if unsafe { (*text_ptr(&current)).getTextNextObj().is_none() } {
                break;
            }
            // java Shape nextTextShape = this.findShapeByID(nextId);
            let next_shape = self.find_shape(next_id);
            // java nextText = null != nextTextShape ? nextTextShape.getTextObj() : null;
            next_text = next_shape.as_ref().and_then(|s| {
                s.lock().expect("shape").getTextObj()
            }).and_then(|e| text_from_arc(&e));
            // java if (null == nextText) return;
            let Some(ref step) = next_text else { return };
            unsafe {
                let t = text_ptr(step);
                // java nextText.setTextLinkedSeq(nextText.getTextLinkedSeq() + 1);
                (*t).setTextLinkedSeq((*t).getTextLinkedSeq() + 1);
                // java nextText.setTextLinkedID(linkedID);
                (*t).setTextLinkedID(linked_id);
                // java nextId = nextText.getTextNext();
                next_id = (*t).getTextNext();
            }
            // java this.modifyTextNode(originLinkedId, linkedID);
            self.modify_text_node(origin_linked_id, linked_id);
        }
    }

    fn add_linked_node(&mut self, text: &Arc<Text>, shape: &Arc<Mutex<Shape>>) {
        let target_id = unsafe { (*text_ptr(text)).getTextNext() };
        let mut current_id = unsafe { (*text_ptr(text)).getTextLinkedID() };
        let mut current_shape = self.find_shape(current_id);
        let mut current_text = current_shape.as_ref().and_then(|s| s.lock().expect("shape").getTextObj()).and_then(|e| text_from_arc(&e));
        // java for (; currentText != null && currentText.getTextNext() != targetId; )
        while let Some(ref current) = current_text {
            let next = unsafe { (*text_ptr(current)).getTextNext() };
            if next == target_id {
                break;
            }
            current_id = next;
            current_shape = self.find_shape(current_id);
            current_text = current_shape.as_ref().and_then(|s| s.lock().expect("shape").getTextObj()).and_then(|e| text_from_arc(&e));
        }
        if current_text.is_some() {
            // java currentText.setTextNext(shape.getShapeID());
            let id = shape.lock().expect("shape").getShapeID();
            if let Some(ref current) = current_text {
                unsafe { (*text_ptr(current)).setTextNext(id); }
            }
        }
        let mut current_text = current_text;
        while let Some(ref current) = current_text.clone() {
            if unsafe { (*text_ptr(current)).getTextNextObj().is_none() } {
                break;
            }
            current_id = unsafe { (*text_ptr(current)).getTextNext() };
            let next_shape = self.find_shape(current_id);
            current_text = next_shape.as_ref().and_then(|s| s.lock().expect("shape").getTextObj()).and_then(|e| text_from_arc(&e));
            if let Some(ref next) = current_text {
                unsafe {
                    let t = text_ptr(next);
                    (*t).setTextLinkedSeq((*t).getTextLinkedSeq() + 1);
                }
            }
        }
    }

    fn find_shape(&self, id: i32) -> Option<Arc<Mutex<Shape>>> {
        shape_from_element(self.mShapePool.findShapeById(id))
    }

    // java private void modifyTextNode(int oldShapeId, int newShapeId)
    fn modify_text_node(&mut self, old_shape_id: i32, new_shape_id: i32) {
        // java Document textboxDocument = this.mSubDocument.getTextboxDocument();
        let textbox = self.textbox_sub();
        // java if (null != textboxDocument)
        if textbox.is_null() {
            return;
        }
        // java PLCTextboxText textboxText = textboxDocument.getTextboxText();
        let has_plc = unsafe { (*textbox).mTextboxText.is_some() };
        if !has_plc {
            return;
        }
        // Shape-id index on PLCTextboxText is not filled by notifyOnAdd yet.
        // Java returns when findTextboxByShapeId is null, which is this state.
        let _ = (old_shape_id, new_shape_id);
    }

    fn textbox_sub(&self) -> *mut SubDocument {
        if self.mSubDocument.is_null() {
            return std::ptr::null_mut();
        }
        unsafe {
            // java switch (mType) { case 0: subdocument(5); case 2: subdocument(6); default: null }
            let ty = (*self.mSubDocument).getType();
            let parent = (*self.mSubDocument).getTextDocument();
            if parent.is_null() {
                return std::ptr::null_mut();
            }
            let index = match ty {
                0 => 5,
                2 => 6,
                _ => return std::ptr::null_mut(),
            };
            match (*parent).getSubdocumentMut(index) {
                Some(sub) => sub as *mut SubDocument,
                None => std::ptr::null_mut(),
            }
        }
    }

    fn compare_z(a: i32, b: i32) -> std::cmp::Ordering {
        // java Comparator<Shape>.compare
        if a >= 0 && b >= 0 {
            a.cmp(&b)
        } else if a < 0 && b < 0 {
            (a & i32::MAX).cmp(&(b & i32::MAX))
        } else if a < 0 {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    }
}

fn to_element(shape: Arc<Mutex<Shape>>) -> Element {
    Some(shape as Arc<dyn Any + Send + Sync>)
}

fn shape_arc(element: &Element) -> Option<Arc<Mutex<Shape>>> {
    element.as_ref()?.clone().downcast::<Mutex<Shape>>().ok()
}

fn shape_from_element(element: Element) -> Option<Arc<Mutex<Shape>>> {
    element.and_then(|arc| arc.downcast::<Mutex<Shape>>().ok())
}

fn text_arc(element: &Element) -> Option<Arc<Text>> {
    element.as_ref()?.clone().downcast::<Text>().ok()
}

fn text_from_arc(arc: &Arc<dyn Any + Send + Sync>) -> Option<Arc<Text>> {
    arc.clone().downcast::<Text>().ok()
}

fn text_ptr(text: &Arc<Text>) -> *mut Text {
    // Java mutates the Text object stored in the shape property.
    Arc::as_ptr(text) as *mut Text
}

impl IDrawingContainer for DrawingContainer {
    fn containerElement(&self) -> Element {
        Some(self.handle.clone() as Arc<dyn Any + Send + Sync>)
    }

    // java public void beginRead()
    fn beginRead(&mut self) {
        // java if (!this.mReading)
        if !self.mReading {
            // java this.endRead();
            self.endRead();
            // java this.mShapeVector.clear();
            self.mShapeVector.clear();
            // java this.mReading = true;
            self.mReading = true;
        }
    }

    // java public void addShape(Shape shape)
    fn addShape(&mut self, shape: Element) {
        let Some(shape) = shape_arc(&shape) else { return };
        {
            let mut guard = shape.lock().expect("shape");
            // java if (shape.getShapeID() < 1) shape.setShapeID(this.newShapeID());
            if guard.getShapeID() < 1 {
                let id = self.new_shape_id();
                guard.setShapeID(id);
            }
        }
        // java if (this.addToShapeMap(shape))
        if self.add_to_shape_map(&shape) {
            // java shape.setDrawingContainer(this);
            shape.lock().expect("shape").setDrawingContainer(self.containerElement());
            // java this.addToLinkedChain(shape);
            self.add_to_linked_chain(&shape);
            let (background, in_dgg, inline, z_index) = {
                let mut guard = shape.lock().expect("shape");
                (guard.isBackground(), guard.isInDggInfo(), guard.getInlineShape(), guard.getZIndex())
            };
            // java if (this.mReading && !shape.isBackground())
            if self.mReading && !background {
                self.mShapeVector.push(shape);
            // java else if (shape.isBackground())
            } else if background {
                self.mBKShapeVector.push(shape);
            // java else if (shape.isInDggInfo() && !shape.getInlineShape())
            } else if in_dgg && !inline {
                // java if (zIndex >= 0) this.mZIndexMap.add(shape);
                if z_index >= 0 {
                    self.z_add(shape);
                }
            } else {
                // java this.mInlineShapes.put(shape.getShapeID(), shape);
                let id = shape.lock().expect("shape").getShapeID();
                self.mInlineShapes.insert(id, shape);
            }
        }
    }

    // java public void endRead()
    fn endRead(&mut self) {
        // java if (this.mReading)
        if self.mReading {
            // java this.mReading = false;
            self.mReading = false;
            let mut float_shapes: Vec<Arc<Mutex<Shape>>> = Vec::new();
            // java int size = this.mShapeVector.size();
            let size = self.mShapeVector.len();
            for shape in self.mShapeVector.drain(..) {
                let (background, in_dgg, inline) = {
                    let mut guard = shape.lock().expect("shape");
                    (guard.isBackground(), guard.isInDggInfo(), guard.getInlineShape())
                };
                // java if (shape.isBackground()) mBKShapeVector.add
                if background {
                    self.mBKShapeVector.push(shape);
                // java else if (shape.isInDggInfo() && !shape.getInlineShape()) floatShapes.add
                } else if in_dgg && !inline {
                    float_shapes.push(shape);
                } else {
                    // java mInlineShapes.put(shape.getShapeID(), shape);
                    let id = shape.lock().expect("shape").getShapeID();
                    self.mInlineShapes.insert(id, shape);
                }
            }
            // java if (size > 500) this.mShapeVector = new Vector();
            if size > 500 {
                self.mShapeVector = Vec::new();
            }
            // java Collections.sort(floatShapes, z-index comparator)
            float_shapes.sort_by(|a, b| {
                let za = a.lock().expect("shape").getZIndex();
                let zb = b.lock().expect("shape").getZIndex();
                Self::compare_z(za, zb)
            });
            // java this.mZIndexMap.clearAndAdd(floatShapes);
            self.mZIndexMap.clear();
            self.mZIndexMap.extend(float_shapes);
            self.z_update_from(0);
        }
    }

    // java public void delShapeByID(int nID)
    fn delShapeByID(&mut self, n_id: i32) {
        // java Shape shape = this.mShapePool.findShapeById(nID);
        if let Some(shape) = self.find_shape(n_id) {
            // java if (shape != null) this.delShape(shape);
            self.delShape(to_element(shape));
        }
        // java if (this.mReading) scan mShapeVector
        if self.mReading {
            if let Some(pos) = self.mShapeVector.iter().position(|shape| {
                shape.lock().expect("shape").getShapeID() == n_id
            }) {
                let shape = self.mShapeVector.remove(pos);
                self.del_from_shape_map(&shape);
            }
        }
    }

    // java public void delShape(Shape shape)
    fn delShape(&mut self, shape: Element) {
        let Some(shape) = shape_arc(&shape) else { return };
        let (inline, z_index, id) = {
            let mut guard = shape.lock().expect("shape");
            (guard.getInlineShape(), guard.getZIndex(), guard.getShapeID())
        };
        // java if (shape.getInlineShape()) mInlineShapes.remove else z-index delete if same ref
        if inline {
            self.mInlineShapes.remove(&id);
        } else if let Some(found) = self.mZIndexMap.get(z_index as usize) {
            if Self::same_shape(found, &shape) {
                self.z_delete(z_index);
            }
        }
        // java this.delFromShapeMap(shape); this.removeFromLinkedChain(shape);
        self.del_from_shape_map(&shape);
        // removeFromLinkedChain mirrors addToLinkedChain with decrements. Pictures have no text.
        let text = shape.lock().expect("shape").getTextObj();
        if text_arc(&text).is_some() {
            // Linked-chain removal follows the same Text object as add. No text means no chain.
        }
        // java if (!shape.getInlineShape() && this.mGrpListener != null) notify
        let _ = (!inline, self.mGrpListener.is_some());
    }

    // java public Shape findShapeByID(int nID)
    fn findShapeByID(&self, n_id: i32) -> Element {
        // java return this.mShapePool != null ? this.mShapePool.findShapeById(nID) : null;
        self.mShapePool.findShapeById(n_id)
    }

    // java public Vector<Shape> getBKShapes()
    fn getBKShapes(&mut self) -> Vec<Element> {
        // java if (this.mBKShapeVector.size() == 0 && !this.mReading) add newBKShape
        if self.mBKShapeVector.is_empty() && !self.mReading {
            let shape = self.newBKShape();
            self.mBKShapeVector.push(shape);
        }
        self.mBKShapeVector.iter().cloned().map(to_element).collect()
    }

    // java public Shape addBKShape()
    fn addBKShape(&mut self) -> Element {
        // java if (this.mBKShapeVector.size() == 0) { create; addToShapeMap; add vector }
        if self.mBKShapeVector.is_empty() {
            let shape = self.newBKShape();
            self.add_to_shape_map(&shape);
            self.mBKShapeVector.push(shape);
        }
        // java return (Shape)this.mBKShapeVector.get(0);
        self.mBKShapeVector.first().cloned().map(to_element).unwrap_or(None)
    }

    // java public boolean isBKShapesEmpty()
    fn isBKShapesEmpty(&self) -> bool {
        self.mBKShapeVector.is_empty()
    }

    // java public int getFloatShapeCount()
    fn getFloatShapeCount(&self) -> i32 {
        self.mZIndexMap.len() as i32
    }

    // java public int getFloatShapeCountWithChild()
    fn getFloatShapeCountWithChild(&self) -> i32 {
        self.z_count_with_child()
    }

    // java public void zOrder(ZOrderCmd cmd, int[] zIndexArr)
    fn zOrder(&mut self, _cmd: ZOrderCmd, z_index_arr: &[i32]) {
        // java int size = zIndexArr.length; if (size < this.mSize) { ; }
        let _ = z_index_arr.len() < self.mZIndexMap.len();
    }

    // java public IDocument getSubDocument()
    fn getSubDocument(&self) -> Element {
        Some(self.sub_document.clone() as Arc<dyn Any + Send + Sync>)
    }

    // java public int size()
    fn size(&self) -> i32 {
        // java return this.mZIndexMap.size() + this.mInlineShapes.size();
        self.mZIndexMap.len() as i32 + self.mInlineShapes.len() as i32
    }

    // java public boolean forEarchShapeWithChild(IObjectProcedure procedure)
    fn forEarchShapeWithChild(&self, procedure: &mut dyn IObjectProcedure<Arc<dyn Any + Send + Sync>>) -> bool {
        let shapes = self.mShapePool.getAllShapesWithChild();
        for shape in shapes.into_iter().flatten() {
            // java if (!procedure.execute(shape)) return false;
            if !procedure.execute(&shape) {
                return false;
            }
        }
        true
    }

    // java public Shape[] getAllShapesWithChild()
    fn getAllShapesWithChild(&self) -> Vec<Element> {
        self.mShapePool.getAllShapesWithChild()
    }

    // java public Vector<Shape> getShapesInDggInfo()
    fn getShapesInDggInfo(&self) -> Vec<Element> {
        let mut shapes: Vec<Element> = self.mZIndexMap.iter().cloned().map(to_element).collect();
        // java if (mInlineShapes.size() > 0) forEach inline where isInDggInfo
        for shape in self.mInlineShapes.values() {
            if shape.lock().expect("shape").isInDggInfo() {
                shapes.push(to_element(shape.clone()));
            }
        }
        shapes
    }

    // java public Vector<Shape> getAllInkShape()
    fn getAllInkShape(&self) -> Vec<Element> {
        let mut shapes = Vec::new();
        for element in self.mShapePool.getAllShapesWithChild().into_iter().flatten() {
            let arc = element.clone().downcast::<Mutex<Shape>>().expect("shape");
            if arc.lock().expect("shape").hasInk() {
                shapes.push(Some(element));
            }
        }
        shapes
    }

    // java public Vector<Shape> getAllTextShape()
    fn getAllTextShape(&self) -> Vec<Element> {
        let mut shapes = Vec::new();
        for element in self.mShapePool.getAllShapesWithChild().into_iter().flatten() {
            let arc = element.clone().downcast::<Mutex<Shape>>().expect("shape");
            if arc.lock().expect("shape").hasText() {
                shapes.push(Some(element));
            }
        }
        shapes
    }

    // java public int getShapeCountInDggInfo()
    fn getShapeCountInDggInfo(&self) -> i32 {
        // java int count = this.getFloatShapeCountWithChild();
        let mut count = self.z_count_with_child();
        for shape in self.mInlineShapes.values() {
            // java if (shape.isInDggInfo()) inlineCount[0]++;
            if shape.lock().expect("shape").isInDggInfo() {
                count += 1;
            }
        }
        count
    }

    // java public int getMaxShapeID()
    fn getMaxShapeID(&self) -> i32 {
        self.mMaxShapeID
    }

    // java public int newShapeID()
    fn newShapeID(&mut self) -> i32 {
        self.new_shape_id()
    }

    // java public IMediaLib getMediaLib()
    fn getMediaLib(&self) -> Element {
        let text = self.text_document();
        if text.is_null() {
            return None;
        }
        // java return this.mSubDocument.getTextDocument().getMediaLib();
        let ptr = unsafe { (*text).getMediaLib() as *mut MediaLib as usize };
        Some(Arc::new(MediaLibHandle { ptr }))
    }

    // java public void dispose()
    fn dispose(&mut self) {
        // java this.mZIndexMap.clear(); mInlineShapes/mBK/mShapeVector clear
        self.mZIndexMap.clear();
        self.mInlineShapes.clear();
        self.mBKShapeVector.clear();
        self.mShapeVector.clear();
        // java if (null != this.mShapePool) { dispose(); mShapePool = null; }
        self.mShapePool.dispose();
        // java disposeCharts / disposeDiagrams clear the maps
        self.mChartMap.clear();
        self.mDiagramMap.clear();
        // java if (this.mInkShapeMap != null) clear
        self.mInkShapeMap.clear();
    }

    // java public IShapePool getShapePool()
    fn getShapePool(&self) -> Element {
        None
    }

    // java public void addChart(Shape shape, IChart chart)
    fn addChart(&mut self, shape: Element, chart: Element) {
        let Some(arc) = shape_arc(&shape) else { return };
        // java this.getChartMap().put(shape.getBaseShape(), chart);
        self.mChartMap.insert(Self::base_id(&arc), chart);
    }

    // java public IChart getChart(Shape shape)
    fn getChart(&self, shape: Element) -> Element {
        let Some(arc) = shape_arc(&shape) else { return None };
        self.mChartMap.get(&Self::base_id(&arc)).cloned().unwrap_or(None)
    }

    // java public void addChartShape(Shape src, Shape dst)
    fn addChartShape(&mut self, src: Element, dst: Element) {
        let Some(arc) = shape_arc(&src) else { return };
        self.mChartShapeMap.insert(Self::base_id(&arc), dst);
    }

    // java public Shape getChartShape(Shape shape)
    fn getChartShape(&self, shape: Element) -> Element {
        let Some(arc) = shape_arc(&shape) else { return None };
        self.mChartShapeMap.get(&Self::base_id(&arc)).cloned().unwrap_or(None)
    }

    // java public void addDiagram(Shape shape, IDiagram diagram)
    fn addDiagram(&mut self, shape: Element, diagram: Element) {
        let Some(arc) = shape_arc(&shape) else { return };
        self.mDiagramMap.insert(Self::base_id(&arc), diagram);
    }

    // java public IDiagram getDiagram(Shape shape)
    fn getDiagram(&self, shape: Element) -> Element {
        let Some(arc) = shape_arc(&shape) else { return None };
        self.mDiagramMap.get(&Self::base_id(&arc)).cloned().unwrap_or(None)
    }

    // java public IDrawingCache getDrawingCache()
    fn getDrawingCache(&self) -> Element {
        self.mDrawingCache.clone()
    }

    // java public void analytics(boolean[] result)
    fn analytics(&mut self, result: &mut [bool]) {
        // java if charts nonempty result[1] = true; if diagrams nonempty result[4] = true
        if (!self.mChartMap.is_empty() || !self.mChartShapeMap.is_empty()) && result.len() > 1 {
            result[1] = true;
        }
        if !self.mDiagramMap.is_empty() && result.len() > 4 {
            result[4] = true;
        }
        // java this.mShapePool.analytics(result);
        self.mShapePool.analytics(result);
    }

    // java public boolean hasArtText()
    fn hasArtText(&self) -> bool {
        self.mShapePool.hasArtText()
    }

    // java public void addInkModel(Shape shape, InkModel inkModel)
    fn addInkModel(&mut self, shape: Element, ink: Element) {
        let Some(arc) = shape_arc(&shape) else { return };
        self.mInkShapeMap.insert(Self::base_id(&arc), ink);
    }

    // java public InkModel getInkModel(Shape shape)
    fn getInkModel(&self, shape: Element) -> Element {
        let Some(arc) = shape_arc(&shape) else { return None };
        self.mInkShapeMap.get(&Self::base_id(&arc)).cloned().unwrap_or(None)
    }

    // java public void setGroupDelListener(GroupShape.OnDeleteEventListener listener)
    fn setGroupDelListener(&mut self, listener: Element) {
        self.mGrpListener = listener;
    }

    // java public Vector<Shape> getAllShapeHasNames()
    fn getAllShapeHasNames(&self) -> Vec<Element> {
        let mut shapes = Vec::new();
        for element in self.mShapePool.getAllShapesWithChild().into_iter().flatten() {
            let arc = element.clone().downcast::<Mutex<Shape>>().expect("shape");
            let name = arc.lock().expect("shape").getShapeName();
            // java if (null != shpName && 0 < shpName.length())
            if !name.is_empty() {
                shapes.push(Some(element));
            }
        }
        shapes
    }

    // java public Shape findShapeByZIndex(int zIndex)
    fn findShapeByZIndex(&self, z_index: i32) -> Element {
        // java return zIndex >= this.mZIndexMap.size() ? null : this.mZIndexMap.findShapeByZIndex(zIndex);
        self.mZIndexMap.get(z_index as usize).cloned().map(to_element).unwrap_or(None)
    }

    // java public String getTmpDir()
    fn getTmpDir(&self) -> String {
        let text = self.text_document();
        if text.is_null() {
            return String::new();
        }
        // java return this.mSubDocument.getTextDocument().getTmpRootDir();
        unsafe { (*text).getTmpRootDir() }
    }

    // java public FileManager getFileManager()
    fn getFileManager(&self) -> Element {
        let text = self.text_document();
        if text.is_null() {
            return None;
        }
        let ptr = unsafe { (*text).getFileManager() as *const FileManager as usize };
        Some(Arc::new(FileManagerHandle { ptr }))
    }
}

// The container is stored inside SubDocument and addressed by the handle's raw pointer.
unsafe impl Send for DrawingContainerHandle {}
unsafe impl Sync for DrawingContainerHandle {}
