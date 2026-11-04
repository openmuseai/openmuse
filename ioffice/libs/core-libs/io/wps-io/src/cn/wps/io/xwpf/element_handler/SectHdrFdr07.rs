// Java: import cn.wps.io.xwpf.element_handler.vml.HdrFtr07
// use io_wps_io::cn::wps::io::xwpf::element_handler::vml::HdrFtr07;
// Java: import java.util.ArrayList
// // TODO_RUST_CODE use ???; // java.util.ArrayList

// Java: public class SectHdrFdr07
pub struct SectHdrFdr07 {
    // Java field: ArrayList<HdrFtr07> mHdrFdrIds
    pub(crate) mHdrFdrIds: Option<Box<dyn std::any::Any + Send + Sync>>,
}

// Java: public class SectHdrFdr07
impl SectHdrFdr07 {
    // Java: SectHdrFdr07()
    pub fn new() -> Self {
      // Java: super();
      Self {
          mHdrFdrIds: None,
      }
    }

    // Java: ArrayList<HdrFtr07> getHdrFdrIds()
    pub fn getHdrFdrIds(&self) -> &Option<Box<dyn std::any::Any + Send + Sync>> {
      // Java: return this.mHdrFdrIds;
      &self.mHdrFdrIds
    }

}
