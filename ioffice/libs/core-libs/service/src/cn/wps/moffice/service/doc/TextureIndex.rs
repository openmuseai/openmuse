// Java: import android.os.Parcel
use stubs::android::os::Parcel::*;
// Java: import android.os.Parcelable
use stubs::android::os::Parcelable::*;

// Java field:    static TextureIndex[] mIndexs = new TextureIndex[]{wdTextureAuto, wdTextureSolid, wdTexture5Percent, wdTexture10Percent, wdTexture20Percent, wdTexture25Percent, wdTexture30Percent, wdTexture40Percent, wdTexture50Percent, wdTexture60Percent, wdTexture70Percent, wdTexture75Percent, wdTexture80Percent, wdTexture90Percent, wdTextureDarkHorizontal, wdTextureDarkVertical, wdTextureDarkDiagonalUp, wdTextureDarkDiagonalDown, wdTextureDarkCross, wdTextureDarkDiagonalCross, wdTextureHorizontal, wdTextureVertical, wdTextureDiagonalUp, wdTextureDiagonalDown, wdTextureCross, wdTextureDiagonalCross, wdTexture2Pt5Percent, wdTexture7Pt5Percent, wdTexture12Pt5Percent, wdTexture15Pt5Percent, wdTexture17Pt5Percent, wdTexture22Pt5Percent, wdTexture27Pt5Percent, wdTexture32Pt5Percent, wdTexture35Pt5Percent, wdTexture37Pt5Percent, wdTexture42Pt5Percent, wdTexture45Pt5Percent, wdTexture47Pt5Percent, wdTexture52Pt5Percent, wdTexture55Pt5Percent, wdTexture57Pt5Percent, wdTexture62Pt5Percent, wdTexture65Pt5Percent, wdTexture67Pt5Percent, wdTexture72Pt5Percent, wdTexture77Pt5Percent, wdTexture82Pt5Percent, wdTexture85Pt5Percent, wdTexture87Pt5Percent, wdTexture92Pt5Percent, wdTexture95Pt5Percent, wdTexture97Pt5Percent, wdTextureNone};
pub static mIndexs: std::sync::LazyLock<Vec<TextureIndex>> =
                    std::sync::LazyLock::new(|| vec![wdTextureAuto, wdTextureSolid, wdTexture5Percent, wdTexture10Percent, wdTexture20Percent, wdTexture25Percent, wdTexture30Percent, wdTexture40Percent, wdTexture50Percent, wdTexture60Percent, wdTexture70Percent, wdTexture75Percent, wdTexture80Percent, wdTexture90Percent, wdTextureDarkHorizontal, wdTextureDarkVertical, wdTextureDarkDiagonalUp, wdTextureDarkDiagonalDown, wdTextureDarkCross, wdTextureDarkDiagonalCross, wdTextureHorizontal, wdTextureVertical, wdTextureDiagonalUp, wdTextureDiagonalDown, wdTextureCross, wdTextureDiagonalCross, wdTexture2Pt5Percent, wdTexture7Pt5Percent, wdTexture12Pt5Percent, wdTexture15Pt5Percent, wdTexture17Pt5Percent, wdTexture22Pt5Percent, wdTexture27Pt5Percent, wdTexture32Pt5Percent, wdTexture35Pt5Percent, wdTexture37Pt5Percent, wdTexture42Pt5Percent, wdTexture45Pt5Percent, wdTexture47Pt5Percent, wdTexture52Pt5Percent, wdTexture55Pt5Percent, wdTexture57Pt5Percent, wdTexture62Pt5Percent, wdTexture65Pt5Percent, wdTexture67Pt5Percent, wdTexture72Pt5Percent, wdTexture77Pt5Percent, wdTexture82Pt5Percent, wdTexture85Pt5Percent, wdTexture87Pt5Percent, wdTexture92Pt5Percent, wdTexture95Pt5Percent, wdTexture97Pt5Percent, wdTextureNone]);

// Java: public enum TextureIndex implements Parcelable 
#[derive(Copy, Clone, Debug)]
pub enum TextureIndex {
    // Java field:    wdTextureAuto(0),
    wdTextureAuto,
    // Java field:    wdTextureSolid(1),
    wdTextureSolid,
    // Java field:    wdTexture5Percent(2),
    wdTexture5Percent,
    // Java field:    wdTexture10Percent(3),
    wdTexture10Percent,
    // Java field:    wdTexture20Percent(4),
    wdTexture20Percent,
    // Java field:    wdTexture25Percent(5),
    wdTexture25Percent,
    // Java field:    wdTexture30Percent(6),
    wdTexture30Percent,
    // Java field:    wdTexture40Percent(7),
    wdTexture40Percent,
    // Java field:    wdTexture50Percent(8),
    wdTexture50Percent,
    // Java field:    wdTexture60Percent(9),
    wdTexture60Percent,
    // Java field:    wdTexture70Percent(10),
    wdTexture70Percent,
    // Java field:    wdTexture75Percent(11),
    wdTexture75Percent,
    // Java field:    wdTexture80Percent(12),
    wdTexture80Percent,
    // Java field:    wdTexture90Percent(13),
    wdTexture90Percent,
    // Java field:    wdTextureDarkHorizontal(14),
    wdTextureDarkHorizontal,
    // Java field:    wdTextureDarkVertical(15),
    wdTextureDarkVertical,
    // Java field:    wdTextureDarkDiagonalUp(16),
    wdTextureDarkDiagonalUp,
    // Java field:    wdTextureDarkDiagonalDown(17),
    wdTextureDarkDiagonalDown,
    // Java field:    wdTextureDarkCross(18),
    wdTextureDarkCross,
    // Java field:    wdTextureDarkDiagonalCross(19),
    wdTextureDarkDiagonalCross,
    // Java field:    wdTextureHorizontal(20),
    wdTextureHorizontal,
    // Java field:    wdTextureVertical(21),
    wdTextureVertical,
    // Java field:    wdTextureDiagonalUp(22),
    wdTextureDiagonalUp,
    // Java field:    wdTextureDiagonalDown(23),
    wdTextureDiagonalDown,
    // Java field:    wdTextureCross(24),
    wdTextureCross,
    // Java field:    wdTextureDiagonalCross(25),
    wdTextureDiagonalCross,
    // Java field:    wdTexture2Pt5Percent(35),
    wdTexture2Pt5Percent,
    // Java field:    wdTexture7Pt5Percent(36),
    wdTexture7Pt5Percent,
    // Java field:    wdTexture12Pt5Percent(37),
    wdTexture12Pt5Percent,
    // Java field:    wdTexture15Pt5Percent(38),
    wdTexture15Pt5Percent,
    // Java field:    wdTexture17Pt5Percent(39),
    wdTexture17Pt5Percent,
    // Java field:    wdTexture22Pt5Percent(40),
    wdTexture22Pt5Percent,
    // Java field:    wdTexture27Pt5Percent(41),
    wdTexture27Pt5Percent,
    // Java field:    wdTexture32Pt5Percent(42),
    wdTexture32Pt5Percent,
    // Java field:    wdTexture35Pt5Percent(43),
    wdTexture35Pt5Percent,
    // Java field:    wdTexture37Pt5Percent(44),
    wdTexture37Pt5Percent,
    // Java field:    wdTexture42Pt5Percent(45),
    wdTexture42Pt5Percent,
    // Java field:    wdTexture45Pt5Percent(46),
    wdTexture45Pt5Percent,
    // Java field:    wdTexture47Pt5Percent(47),
    wdTexture47Pt5Percent,
    // Java field:    wdTexture52Pt5Percent(48),
    wdTexture52Pt5Percent,
    // Java field:    wdTexture55Pt5Percent(49),
    wdTexture55Pt5Percent,
    // Java field:    wdTexture57Pt5Percent(50),
    wdTexture57Pt5Percent,
    // Java field:    wdTexture62Pt5Percent(51),
    wdTexture62Pt5Percent,
    // Java field:    wdTexture65Pt5Percent(52),
    wdTexture65Pt5Percent,
    // Java field:    wdTexture67Pt5Percent(53),
    wdTexture67Pt5Percent,
    // Java field:    wdTexture72Pt5Percent(54),
    wdTexture72Pt5Percent,
    // Java field:    wdTexture77Pt5Percent(55),
    wdTexture77Pt5Percent,
    // Java field:    wdTexture82Pt5Percent(56),
    wdTexture82Pt5Percent,
    // Java field:    wdTexture85Pt5Percent(57),
    wdTexture85Pt5Percent,
    // Java field:    wdTexture87Pt5Percent(58),
    wdTexture87Pt5Percent,
    // Java field:    wdTexture92Pt5Percent(59),
    wdTexture92Pt5Percent,
    // Java field:    wdTexture95Pt5Percent(60),
    wdTexture95Pt5Percent,
    // Java field:    wdTexture97Pt5Percent(61),
    wdTexture97Pt5Percent,
    // Java field:    wdTextureNone(62);
    wdTextureNone,
}

// Java: public enum TextureIndex implements Parcelable 
impl TextureIndex {
    // Java: enum constructor mapping
    pub fn from_i32(value: i32) -> TextureIndex {
      match value {
        0 => TextureIndex::wdTextureAuto,
        1 => TextureIndex::wdTextureSolid,
        2 => TextureIndex::wdTexture5Percent,
        3 => TextureIndex::wdTexture10Percent,
        4 => TextureIndex::wdTexture20Percent,
        5 => TextureIndex::wdTexture25Percent,
        6 => TextureIndex::wdTexture30Percent,
        7 => TextureIndex::wdTexture40Percent,
        8 => TextureIndex::wdTexture50Percent,
        9 => TextureIndex::wdTexture60Percent,
        10 => TextureIndex::wdTexture70Percent,
        11 => TextureIndex::wdTexture75Percent,
        12 => TextureIndex::wdTexture80Percent,
        13 => TextureIndex::wdTexture90Percent,
        14 => TextureIndex::wdTextureDarkHorizontal,
        15 => TextureIndex::wdTextureDarkVertical,
        16 => TextureIndex::wdTextureDarkDiagonalUp,
        17 => TextureIndex::wdTextureDarkDiagonalDown,
        18 => TextureIndex::wdTextureDarkCross,
        19 => TextureIndex::wdTextureDarkDiagonalCross,
        20 => TextureIndex::wdTextureHorizontal,
        21 => TextureIndex::wdTextureVertical,
        22 => TextureIndex::wdTextureDiagonalUp,
        23 => TextureIndex::wdTextureDiagonalDown,
        24 => TextureIndex::wdTextureCross,
        25 => TextureIndex::wdTextureDiagonalCross,
        35 => TextureIndex::wdTexture2Pt5Percent,
        36 => TextureIndex::wdTexture7Pt5Percent,
        37 => TextureIndex::wdTexture12Pt5Percent,
        38 => TextureIndex::wdTexture15Pt5Percent,
        39 => TextureIndex::wdTexture17Pt5Percent,
        40 => TextureIndex::wdTexture22Pt5Percent,
        41 => TextureIndex::wdTexture27Pt5Percent,
        42 => TextureIndex::wdTexture32Pt5Percent,
        43 => TextureIndex::wdTexture35Pt5Percent,
        44 => TextureIndex::wdTexture37Pt5Percent,
        45 => TextureIndex::wdTexture42Pt5Percent,
        46 => TextureIndex::wdTexture45Pt5Percent,
        47 => TextureIndex::wdTexture47Pt5Percent,
        48 => TextureIndex::wdTexture52Pt5Percent,
        49 => TextureIndex::wdTexture55Pt5Percent,
        50 => TextureIndex::wdTexture57Pt5Percent,
        51 => TextureIndex::wdTexture62Pt5Percent,
        52 => TextureIndex::wdTexture65Pt5Percent,
        53 => TextureIndex::wdTexture67Pt5Percent,
        54 => TextureIndex::wdTexture72Pt5Percent,
        55 => TextureIndex::wdTexture77Pt5Percent,
        56 => TextureIndex::wdTexture82Pt5Percent,
        57 => TextureIndex::wdTexture85Pt5Percent,
        58 => TextureIndex::wdTexture87Pt5Percent,
        59 => TextureIndex::wdTexture92Pt5Percent,
        60 => TextureIndex::wdTexture95Pt5Percent,
        61 => TextureIndex::wdTexture97Pt5Percent,
        62 => TextureIndex::wdTextureNone,
        _ => TextureIndex::wdTexture10Percent,
      }
    }

    // Java: enum field accessor
    pub fn to_i32(&self) -> i32 {
      match self {
        TextureIndex::wdTextureAuto => 0,
        TextureIndex::wdTextureSolid => 1,
        TextureIndex::wdTexture5Percent => 2,
        TextureIndex::wdTexture10Percent => 3,
        TextureIndex::wdTexture20Percent => 4,
        TextureIndex::wdTexture25Percent => 5,
        TextureIndex::wdTexture30Percent => 6,
        TextureIndex::wdTexture40Percent => 7,
        TextureIndex::wdTexture50Percent => 8,
        TextureIndex::wdTexture60Percent => 9,
        TextureIndex::wdTexture70Percent => 10,
        TextureIndex::wdTexture75Percent => 11,
        TextureIndex::wdTexture80Percent => 12,
        TextureIndex::wdTexture90Percent => 13,
        TextureIndex::wdTextureDarkHorizontal => 14,
        TextureIndex::wdTextureDarkVertical => 15,
        TextureIndex::wdTextureDarkDiagonalUp => 16,
        TextureIndex::wdTextureDarkDiagonalDown => 17,
        TextureIndex::wdTextureDarkCross => 18,
        TextureIndex::wdTextureDarkDiagonalCross => 19,
        TextureIndex::wdTextureHorizontal => 20,
        TextureIndex::wdTextureVertical => 21,
        TextureIndex::wdTextureDiagonalUp => 22,
        TextureIndex::wdTextureDiagonalDown => 23,
        TextureIndex::wdTextureCross => 24,
        TextureIndex::wdTextureDiagonalCross => 25,
        TextureIndex::wdTexture2Pt5Percent => 35,
        TextureIndex::wdTexture7Pt5Percent => 36,
        TextureIndex::wdTexture12Pt5Percent => 37,
        TextureIndex::wdTexture15Pt5Percent => 38,
        TextureIndex::wdTexture17Pt5Percent => 39,
        TextureIndex::wdTexture22Pt5Percent => 40,
        TextureIndex::wdTexture27Pt5Percent => 41,
        TextureIndex::wdTexture32Pt5Percent => 42,
        TextureIndex::wdTexture35Pt5Percent => 43,
        TextureIndex::wdTexture37Pt5Percent => 44,
        TextureIndex::wdTexture42Pt5Percent => 45,
        TextureIndex::wdTexture45Pt5Percent => 46,
        TextureIndex::wdTexture47Pt5Percent => 47,
        TextureIndex::wdTexture52Pt5Percent => 48,
        TextureIndex::wdTexture55Pt5Percent => 49,
        TextureIndex::wdTexture57Pt5Percent => 50,
        TextureIndex::wdTexture62Pt5Percent => 51,
        TextureIndex::wdTexture65Pt5Percent => 52,
        TextureIndex::wdTexture67Pt5Percent => 53,
        TextureIndex::wdTexture72Pt5Percent => 54,
        TextureIndex::wdTexture77Pt5Percent => 55,
        TextureIndex::wdTexture82Pt5Percent => 56,
        TextureIndex::wdTexture85Pt5Percent => 57,
        TextureIndex::wdTexture87Pt5Percent => 58,
        TextureIndex::wdTexture92Pt5Percent => 59,
        TextureIndex::wdTexture95Pt5Percent => 60,
        TextureIndex::wdTexture97Pt5Percent => 61,
        TextureIndex::wdTextureNone => 62,
      }
    }

    // Java: public int getVal() 
    pub fn getVal(&mut self) -> i32 {
      // Java: return this.index;
      return self.index;
    }

    // Java: public static TextureIndex fromValue(int value) 
    pub fn fromValue(value: i32) -> TextureIndex {
      // Java: int index = binarySearch(value);
      let index: i32 = binarySearch(value);
      // Java: return index < 0 ? mIndexs[0] : mIndexs[index];
      return (if index < 0 { mIndexs[0] } else { mIndexs[index] });
    }

    // Java: public static int binarySearch(int value) 
    pub fn binarySearch(value: i32) -> i32 {
      // Java: int low = 0;
      let low: i32 = 0;
      // Java: int high = mIndexs.length;
      let high: i32 = mIndexs.length;
      // Java: while(low < high)
      if true {
         // Java: int mid = low + high >>> 1;
         let mid: i32 = low + high >> 1;
         // Java: int midValue = mIndexs[mid].index;
         let midValue: i32 = mIndexs[mid].index;
         // Java: if (midValue == value)
         if midValue == value {
            // Java: return mid;
            return mid;
         }
         // Java: if (midValue < value)
         if midValue < value {
            // Java: low = mid + 1;
            low = mid + 1;
         }
         // Java: else
         else {
            // Java: high = mid;
            high = mid;
         }
      }
      // Java: return -1 - low;
      return -1 - low;
    }

    // Java: public int describeContents() 
    pub fn describeContents(&mut self) -> i32 {
      // Java: return 0;
      return 0;
    }

    // Java: public void writeToParcel(Parcel dest, int flags) 
    pub fn writeToParcel(&mut self, dest: Parcel, flags: i32) {
      // Java: dest.writeInt(this.ordinal());
      dest.writeInt(self.ordinal());
    }

}
