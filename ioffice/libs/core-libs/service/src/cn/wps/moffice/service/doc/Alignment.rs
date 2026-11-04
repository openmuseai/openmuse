// Java: import android.os.Parcel
use stubs::android::os::Parcel::*;
// Java: import android.os.Parcelable
use stubs::android::os::Parcelable::*;

// Java field:    static Alignment[] mTypes = new Alignment[]{LEFT, CENTER, RIGHT, BOTH, DISTRIBUTE};
pub static mTypes: std::sync::LazyLock<Vec<Alignment>> =
                    std::sync::LazyLock::new(|| vec![LEFT, CENTER, RIGHT, BOTH, DISTRIBUTE]);

// Java: public enum Alignment implements Parcelable 
#[derive(Copy, Clone, Debug)]
pub enum Alignment {
    // Java field:    LEFT(0),
    LEFT,
    // Java field:    CENTER(1),
    CENTER,
    // Java field:    RIGHT(2),
    RIGHT,
    // Java field:    BOTH(3),
    BOTH,
    // Java field:    DISTRIBUTE(4);
    DISTRIBUTE,
}

// Java: public enum Alignment implements Parcelable 
impl Alignment {
    // Java: enum constructor mapping
    pub fn from_i32(value: i32) -> Alignment {
      match value {
        0 => Alignment::LEFT,
        1 => Alignment::CENTER,
        2 => Alignment::RIGHT,
        3 => Alignment::BOTH,
        4 => Alignment::DISTRIBUTE,
        _ => Alignment::BOTH,
      }
    }

    // Java: enum field accessor
    pub fn to_i32(&self) -> i32 {
      match self {
        Alignment::LEFT => 0,
        Alignment::CENTER => 1,
        Alignment::RIGHT => 2,
        Alignment::BOTH => 3,
        Alignment::DISTRIBUTE => 4,
      }
    }

    // Java: public int getVal() 
    pub fn getVal(&mut self) -> i32 {
      // Java: return this.type;
      return self.type;
    }

    // Java: public static Alignment fromValue(int value) 
    pub fn fromValue(value: i32) -> Alignment {
      // Java: return value >= 0 && value < mTypes.length ? mTypes[value] : mTypes[3];
      return (if value >= 0 && value < mTypes.length { mTypes[value] } else { mTypes[3] });
    }

    // Java: public int describeContents() 
    pub fn describeContents(&mut self) -> i32 {
      // Java: return 0;
      return 0;
    }

    // Java: public void writeToParcel(Parcel dest, int flags) 
    pub fn writeToParcel(&mut self, dest: Parcel, flags: i32) {
      // Java: dest.writeInt(this.type);
      dest.writeInt(self.type);
    }

}
