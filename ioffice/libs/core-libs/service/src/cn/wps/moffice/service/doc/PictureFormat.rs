// Java: import android.os.Parcel
use stubs::android::os::Parcel::*;
// Java: import android.os.Parcelable
use stubs::android::os::Parcelable::*;

// Java field:    static PictureFormat[] mFormats = new PictureFormat[]{JPEG, PNG, BMP24, BMP24GRAY, BMP8GRAY, BMP1};
pub static mFormats: std::sync::LazyLock<Vec<PictureFormat>> =
                    std::sync::LazyLock::new(|| vec![JPEG, PNG, BMP24, BMP24GRAY, BMP8GRAY, BMP1]);

// Java: public enum PictureFormat implements Parcelable 
#[derive(Copy, Clone, Debug)]
pub enum PictureFormat {
    // Java field:    JPEG(0),
    JPEG,
    // Java field:    PNG(1),
    PNG,
    // Java field:    BMP24(2),
    BMP24,
    // Java field:    BMP24GRAY(3),
    BMP24GRAY,
    // Java field:    BMP8GRAY(4),
    BMP8GRAY,
    // Java field:    BMP1(5);
    BMP1,
}

// Java: public enum PictureFormat implements Parcelable 
impl PictureFormat {
    // Java: enum constructor mapping
    pub fn from_i32(value: i32) -> PictureFormat {
      match value {
        0 => PictureFormat::JPEG,
        1 => PictureFormat::PNG,
        2 => PictureFormat::BMP24,
        3 => PictureFormat::BMP24GRAY,
        4 => PictureFormat::BMP8GRAY,
        5 => PictureFormat::BMP1,
        _ => PictureFormat::BMP24GRAY,
      }
    }

    // Java: enum field accessor
    pub fn to_i32(&self) -> i32 {
      match self {
        PictureFormat::JPEG => 0,
        PictureFormat::PNG => 1,
        PictureFormat::BMP24 => 2,
        PictureFormat::BMP24GRAY => 3,
        PictureFormat::BMP8GRAY => 4,
        PictureFormat::BMP1 => 5,
      }
    }

    // Java: public int describeContents() 
    pub fn describeContents(&mut self) -> i32 {
      // Java: return 0;
      return 0;
    }

    // Java: public void writeToParcel(Parcel dest, int flags) 
    pub fn writeToParcel(&mut self, dest: Parcel, flags: i32) {
      // Java: dest.writeInt(this.format);
      dest.writeInt(self.format);
    }

}
