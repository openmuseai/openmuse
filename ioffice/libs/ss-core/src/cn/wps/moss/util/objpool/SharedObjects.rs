use std::sync::{Arc,Mutex,LazyLock};
use super::{ObjectPool::ObjectPool,PoolableObjectFactory::PoolableObjectFactory};
// java public class SharedObjects {
// Field slice only: other pools are not yet ported.
pub struct SharedObjects;
use crate::{java_runtime::{Rc,RefCell},cn::wps::moss::engine::grid::{KmoCellData::KmoCellData,KmoFormulaData::KmoFormulaData}};
// java public static final ObjectPool<KmoCellData> kmoCellDataPool = new ObjectPool<KmoCellData>(new PoolableObjectFactory<KmoCellData>() {
pub static kmoCellDataPool:LazyLock<ObjectPool<Rc<RefCell<KmoCellData>>,KmoCellDataFactory>>=LazyLock::new(||ObjectPool::ObjectPool(KmoCellDataFactory));
// Native receiver for the original anonymous PoolableObjectFactory<KmoCellData>.
pub struct KmoCellDataFactory;
impl PoolableObjectFactory<Rc<RefCell<KmoCellData>>> for KmoCellDataFactory{
    // java public KmoCellData newObject() { return new KmoCellData(); }
    fn newObject(&self)->Rc<RefCell<KmoCellData>>{Rc::new(RefCell::new(KmoCellData::default()))}
    // java public void passivateObject(KmoCellData obj) { obj.clear(); }
    fn passivateObject(&self,obj:&Rc<RefCell<KmoCellData>>){obj.borrow_mut().clear();}
}
// java public static final ObjectPool<KmoFormulaData> kmoFormulaDataPool = new ObjectPool<KmoFormulaData>(new PoolableObjectFactory<KmoFormulaData>() {
pub static kmoFormulaDataPool:LazyLock<ObjectPool<Rc<RefCell<KmoFormulaData>>,KmoFormulaDataFactory>>=LazyLock::new(||ObjectPool::ObjectPool(KmoFormulaDataFactory));
// Native receiver for the original anonymous PoolableObjectFactory<KmoFormulaData>.
pub struct KmoFormulaDataFactory;
impl PoolableObjectFactory<Rc<RefCell<KmoFormulaData>>> for KmoFormulaDataFactory{
    // java public KmoFormulaData newObject() { return new KmoFormulaData(); }
    fn newObject(&self)->Rc<RefCell<KmoFormulaData>>{Rc::new(RefCell::new(KmoFormulaData::KmoFormulaData()))}
    // java public void passivateObject(KmoFormulaData obj) { obj.clear(); }
    fn passivateObject(&self,obj:&Rc<RefCell<KmoFormulaData>>){obj.borrow_mut().clear();}
}
// Native java.nio.ByteBuffer.allocate(8) transport; clear resets position only.
#[derive(Default)]pub struct ByteBuffer8{pub bytes:[u8;8],pub position:usize}
pub struct ByteBuffer8Factory;
impl PoolableObjectFactory<Arc<Mutex<ByteBuffer8>>> for ByteBuffer8Factory{
    // java public ByteBuffer newObject() { return ByteBuffer.allocate(8); }
    fn newObject(&self)->Arc<Mutex<ByteBuffer8>>{Arc::new(Mutex::new(ByteBuffer8::default()))}
    // java public void passivateObject(ByteBuffer obj) { obj.clear(); }
    fn passivateObject(&self,obj:&Arc<Mutex<ByteBuffer8>>){obj.lock().unwrap_or_else(|e|e.into_inner()).position=0;}
    // java public int maxPooled() { return 5; }
    fn maxPooled(&self)->i32{5}
}
// java public static final ObjectPool<ByteBuffer> byteBuffer8 = new ObjectPool<ByteBuffer>(new PoolableObjectFactory<ByteBuffer>() { ... });
pub static byteBuffer8:LazyLock<ObjectPool<Arc<Mutex<ByteBuffer8>>,ByteBuffer8Factory>>=LazyLock::new(||ObjectPool::ObjectPool(ByteBuffer8Factory));
