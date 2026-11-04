// Java: import java.util.Vector
use stubs::java::util::Vector::*;

// java public class Version
#[derive(Default, Clone)]
pub struct Version {
    // java Vector<Integer> mVec = new Vector();
    pub(crate) mVec: Option<Vec<i32>>,
}

// java public class Version
impl Version {
    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.clear();
      self.clear();
    }

    // java public void clear()
    pub fn clear(&mut self) {
      // java this.mVec.clear();
      if let Some(v) = self.mVec.as_mut() { v.clear(); }
    }

    // java public int getMajor()
    pub fn getMajor(&mut self) -> i32 {
      // java return this.mVec.size() > 0 ? (Integer)this.mVec.elementAt(0) : 0;
      return (if (self.mVec.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) > 0 { self.mVec.as_ref().expect("mVec")[0 as usize] } else { 0 });
    }

    // java public void setMajor(int ver)
    pub fn setMajor(&mut self, ver: i32) {
      // java if (this.mVec.size() < 1)
      if (self.mVec.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) < 1 {
        // java this.mVec.add(ver);
        self.mVec.as_mut().expect("mVec").push(ver);
      }
      // java else
      else {
        // java this.mVec.set(0, ver);
        self.mVec.as_mut().expect("mVec")[0 as usize] = ver;
      }
    }

    // java public int getMinor()
    pub fn getMinor(&mut self) -> i32 {
      // java return this.mVec.size() > 1 ? (Integer)this.mVec.elementAt(1) : 0;
      return (if (self.mVec.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) > 1 { self.mVec.as_ref().expect("mVec")[1 as usize] } else { 0 });
    }

    // java public void setMinor(int ver)
    pub fn setMinor(&mut self, ver: i32) {
      // java while (this.mVec.size() < 2)
      while (self.mVec.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) < 2 {
        // java this.mVec.add(0);
        self.mVec.as_mut().expect("mVec").push(0);
      }
      // java this.mVec.set(1, ver);
      self.mVec.as_mut().expect("mVec")[1 as usize] = ver;
    }

    // java public void appendOtherVersion(int ver)
    pub fn appendOtherVersion(&mut self, ver: i32) {
      // java while (this.mVec.size() < 2)
      while (self.mVec.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) < 2 {
        // java this.mVec.add(0);
        self.mVec.as_mut().expect("mVec").push(0);
      }
      // java this.mVec.add(ver);
      self.mVec.as_mut().expect("mVec").push(ver);
    }

    // java public int getVersionNum()
    pub fn getVersionNum(&self) -> i32 {
      // java return this.mVec.size();
      return (self.mVec.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
    }

    // java public int getVersion(int n)
    pub fn getVersion(&mut self, n: i32) -> i32 {
      // java return n < this.mVec.size() ? (Integer)this.mVec.elementAt(n) : 0;
      return (if n < (self.mVec.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) { self.mVec.as_ref().expect("mVec")[n as usize] } else { 0 });
    }

}
