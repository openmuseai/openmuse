// Java: import android.os.Parcel
use stubs::android::os::Parcel::*;
// Java: import android.os.Parcelable
use stubs::android::os::Parcelable::*;

// Java field:    static AroundType[] mTypes = new AroundType[]{NONE, AROUND, NILL};
pub static mTypes: std::sync::LazyLock<Vec<AroundType>> =
                    std::sync::LazyLock::new(|| vec![NONE, AROUND, NILL]);

// Java: public enum AroundType implements Parcelable 
#[derive(Copy, Clone, Debug)]
pub enum AroundType {
    // Java field:    NONE(0),
    NONE,
    // Java field:    AROUND(1),
    AROUND,
    // Java field:    NILL(2);
    NILL,
}

// Java: public enum AroundType implements Parcelable 
impl AroundType {
    // Java: enum constructor mapping
    pub fn from_i32(value: i32) -> AroundType {
      match value {
        0 => AroundType::NONE,
        1 => AroundType::AROUND,
        2 => AroundType::NILL,
        _ => AroundType::NILL,
      }
    }

    // Java: enum field accessor
    pub fn to_i32(&self) -> i32 {
      match self {
        AroundType::NONE => 0,
        AroundType::AROUND => 1,
        AroundType::NILL => 2,
      }
    }

    // Java: public int getVal() 
    pub fn getVal(&mut self) -> i32 {
      // Java: return this.type;
      return self.type;
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

    // Java: public static AroundType fromValue(int value) 
    pub fn fromValue(value: i32) -> AroundType {
      // Java: return value >= 0 && value < mTypes.length ? mTypes[value] : mTypes[2];
      return (if value >= 0 && value < mTypes.length { mTypes[value] } else { mTypes[2] });
    }

}
