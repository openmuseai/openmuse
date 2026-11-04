use super::PlaceHolder::{Holder, HolderRef};
use std::sync::{Arc, LazyLock, Mutex, MutexGuard};
// java public class PlaceHolderToken
pub struct PlaceHolderToken {
    // java private boolean mRecycled=false;public String mToken;public PlaceHolder mPlaceHolder;
    mRecycled: bool,
    pub mToken: String,
    pub mPlaceHolder: Option<HolderRef>,
}
pub type TokenRef = Arc<Mutex<PlaceHolderToken>>;
// java private static PlaceHolderToken mPool;private static int mPoolIndex=0;MAX_POOL_SIZE=16;
// Rust owned vector stores the same Java linked-list references in LIFO order.
static POOL: LazyLock<Mutex<Vec<TokenRef>>> = LazyLock::new(|| Mutex::new(Vec::new()));
impl PlaceHolderToken {
    pub fn new() -> Self {
        Self {
            mRecycled: false,
            mToken: String::new(),
            mPlaceHolder: None,
        }
    }
    // java synchronized(mPoolSync){if(mPool!=null){...m.mRecycled=false;--mPoolIndex;return m;}}return new PlaceHolderToken();
    pub fn obtain() -> TokenRef {
        let mut pool = POOL
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(token) = pool.pop() {
            token
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .mRecycled = false;
            token
        } else {
            Arc::new(Mutex::new(Self::new()))
        }
    }
    // java synchronized(mPoolSync){if(!this.mRecycled&&mPoolIndex<16){this.clear();this.mNext=mPool;mPool=this;++mPoolIndex;this.mRecycled=true;}}
    pub fn recycle(token: &TokenRef) {
        let mut pool = POOL
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut data = token
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if !data.mRecycled && pool.len() < 16 {
            data.mToken.clear();
            data.mPlaceHolder = None;
            data.mRecycled = true;
            pool.push(token.clone());
        }
    }
    // java this.mPlaceHolder=pHolder;
    pub fn setPlaceHolder(&mut self, pHolder: HolderRef) {
        self.mPlaceHolder = Some(pHolder);
    }
    // java this.mPlaceHolder.setHolderStr(this.mToken);return this.mPlaceHolder;
    pub fn doMutexHolder(&self) -> MutexGuard<'_, Box<dyn Holder>> {
        let mut holder = self
            .mPlaceHolder
            .as_ref()
            .expect("PlaceHolderToken holder")
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        holder.base_mut().setHolderStr(self.mToken.clone());
        holder
    }
    // java return this.mPlaceHolder.mPlaceHolderID;
    pub fn getHolderID(&self) -> i16 {
        self.mPlaceHolder
            .as_ref()
            .expect("holder")
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .base()
            .mPlaceHolderID
    }
    pub fn isEmpty(&self) -> bool {
        self.mPlaceHolder.is_none()
    }
    pub fn getPoolSize() -> i32 {
        POOL.lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .len() as i32
    }
}
