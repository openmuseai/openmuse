// Java: import cn.wps.moffice.writer.io.reader.html.helper.AbstractPool
use crate::cn::wps::moffice::writer::io::reader::html::helper::AbstractPool::AbstractPool;

// Java field: String TAG
pub(crate) const TAG: Option<&str> = None;
// Java field: VmlCssStylePool sInstance

// Java: public class VmlCssStylePool extends AbstractPool
pub struct VmlCssStylePool {
    pub(crate) inner: AbstractPool,
}

// Java: public class VmlCssStylePool extends AbstractPool
impl VmlCssStylePool {
    // Java: VmlCssStylePool(Class<VmlCssStyle> cls, int maxPoolSize)
    pub fn new(maxPoolSize: i32) -> Self {
      // Java: super(cls, maxPoolSize);
      VmlCssStylePool { inner: AbstractPool::new(maxPoolSize) }
    }

    // Java: String tag()
    pub fn tag(&self) -> String {
      // Java: return TAG;
      TAG.unwrap_or("").to_string()
    }

}
