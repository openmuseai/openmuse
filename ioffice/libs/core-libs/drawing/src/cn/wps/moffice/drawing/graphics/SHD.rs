#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

// Java: import cn.wps.collections.IShareObjIndexdMap
// Java: import java.util.HashMap
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::OnceLock;

// Java field: HashMap<SHD, SHD> sPool
static sPool: OnceLock<Mutex<HashMap<SHD, SHD>>> = OnceLock::new();
// Java field: SHD sTemp
static sTemp: OnceLock<Mutex<SHD>> = OnceLock::new();
// Java field: SHD sDefault
static sDefault: OnceLock<SHD> = OnceLock::new();

#[derive(Clone)]
pub struct SHD {
    // Java field: int colorFore
    pub(crate) colorFore: i32,
    // Java field: int colorBack
    pub(crate) colorBack: i32,
    // Java field: int ipat
    pub(crate) ipat: i32,
    // Java field: int mIndex
    pub(crate) mIndex: i32,
}

// Java: public class SHD implements IShareObjIndexdMap.IShareObjIndexed
impl SHD {
    // Java field: int NONE
    pub const NONE: i32 = -2;
    // Java field: int AUTO
    pub const AUTO: i32 = -1;
    // Java field: int patNil
    pub const patNil: i32 = 65535;
    // Java field: int patAutomatic
    pub const patAutomatic: i32 = 0;
    // Java field: int patSolid
    pub const patSolid: i32 = 1;
    // Java field: int pat5Percent
    pub const pat5Percent: i32 = 2;
    // Java field: int pat10Percent
    pub const pat10Percent: i32 = 3;
    // Java field: int pat20Percent
    pub const pat20Percent: i32 = 4;
    // Java field: int pat25Percent
    pub const pat25Percent: i32 = 5;
    // Java field: int pat30Percent
    pub const pat30Percent: i32 = 6;
    // Java field: int pat40Percent
    pub const pat40Percent: i32 = 7;
    // Java field: int pat50Percent
    pub const pat50Percent: i32 = 8;
    // Java field: int pat60Percent
    pub const pat60Percent: i32 = 9;
    // Java field: int pat70Percent
    pub const pat70Percent: i32 = 10;
    // Java field: int pat75Percent
    pub const pat75Percent: i32 = 11;
    // Java field: int pat80Percent
    pub const pat80Percent: i32 = 12;
    // Java field: int pat90Percent
    pub const pat90Percent: i32 = 13;
    // Java field: int patDarkHorizontal
    pub const patDarkHorizontal: i32 = 14;
    // Java field: int patDarkVertical
    pub const patDarkVertical: i32 = 15;
    // Java field: int patDarkForwardDiagonal
    pub const patDarkForwardDiagonal: i32 = 16;
    // Java field: int patDarkBackwardDiagonal
    pub const patDarkBackwardDiagonal: i32 = 17;
    // Java field: int patDarkCross
    pub const patDarkCross: i32 = 18;
    // Java field: int patDarkDiagonalCross
    pub const patDarkDiagonalCross: i32 = 19;
    // Java field: int patHorizontal
    pub const patHorizontal: i32 = 20;
    // Java field: int patVertical
    pub const patVertical: i32 = 21;
    // Java field: int patForwardDiagonal
    pub const patForwardDiagonal: i32 = 22;
    // Java field: int patBackwardDiagonal
    pub const patBackwardDiagonal: i32 = 23;
    // Java field: int patCross
    pub const patCross: i32 = 24;
    // Java field: int patDiagonalCross
    pub const patDiagonalCross: i32 = 25;
    // Java field: int pat2Percent
    pub const pat2Percent: i32 = 35;
    // Java field: int pat7Percent
    pub const pat7Percent: i32 = 36;
    // Java field: int pat12Percent
    pub const pat12Percent: i32 = 37;
    // Java field: int pat15Percent
    pub const pat15Percent: i32 = 38;
    // Java field: int pat17Percent
    pub const pat17Percent: i32 = 39;
    // Java field: int pat22Percent
    pub const pat22Percent: i32 = 40;
    // Java field: int pat27Percent
    pub const pat27Percent: i32 = 41;
    // Java field: int pat32Percent
    pub const pat32Percent: i32 = 42;
    // Java field: int pat35Percent
    pub const pat35Percent: i32 = 43;
    // Java field: int pat37Percent
    pub const pat37Percent: i32 = 44;
    // Java field: int pat42Percent
    pub const pat42Percent: i32 = 45;
    // Java field: int pat45Percent
    pub const pat45Percent: i32 = 46;
    // Java field: int pat47Percent
    pub const pat47Percent: i32 = 47;
    // Java field: int pat52Percent
    pub const pat52Percent: i32 = 48;
    // Java field: int pat55Percent
    pub const pat55Percent: i32 = 49;
    // Java field: int pat57Percent
    pub const pat57Percent: i32 = 50;
    // Java field: int pat62Percent
    pub const pat62Percent: i32 = 51;
    // Java field: int pat65Percent
    pub const pat65Percent: i32 = 52;
    // Java field: int pat67Percent
    pub const pat67Percent: i32 = 53;
    // Java field: int pat72Percent
    pub const pat72Percent: i32 = 54;
    // Java field: int pat77Percent
    pub const pat77Percent: i32 = 55;
    // Java field: int pat82Percent
    pub const pat82Percent: i32 = 56;
    // Java field: int pat85Percent
    pub const pat85Percent: i32 = 57;
    // Java field: int pat87Percent
    pub const pat87Percent: i32 = 58;
    // Java field: int pat92Percent
    pub const pat92Percent: i32 = 59;
    // Java field: int pat95Percent
    pub const pat95Percent: i32 = 60;
    // Java field: int pat975Percent
    pub const pat975Percent: i32 = 61;
    // Java field: int pat97Percent
    pub const pat97Percent: i32 = 62;

    // Java: void setIndex(int index)
    pub fn setIndex(&mut self, index: i32) {
        // Java: this.mIndex = index;
        self.mIndex = index;
    }

    // Java: int getIndex()
    pub fn getIndex(&self) -> i32 {
        // Java: return this.mIndex;
        self.mIndex
    }

    // Java: Object getHostObj()
    pub fn getHostObj(&self) -> &dyn std::any::Any {
        // Java: return this;
        self
    }

    // Java: SHD()
    pub fn new() -> Self {
        // Java: this(-2, -2, 65535);
        Self::with_all(-2, -2, 65535)
    }

    // Java: SHD(int crBack)
    pub fn with_back(crBack: i32) -> Self {
        // Java: this(-2, crBack, 65535);
        Self::with_all(-2, crBack, 65535)
    }

    // Java: SHD(int crFore, int crBack, int ipat)
    pub fn with_all(crFore: i32, crBack: i32, ipat: i32) -> Self {
        // Java: super();
        let mut instance = Self { colorFore: 0, colorBack: 0, ipat: 0, mIndex: 0 };
        instance.init_with_all(crFore, crBack, ipat);
        instance
    }

    fn init_with_all(&mut self, crFore: i32, crBack: i32, ipat: i32) {
        // Java: this.mIndex = 0;
        self.mIndex = 0;
        // Java: this.colorBack = crBack;
        self.colorBack = crBack;
        // Java: this.colorFore = crFore;
        self.colorFore = crFore;
        // Java: this.ipat = ipat;
        self.ipat = ipat;
    }

    // Java: int getColorFore()
    pub fn getColorFore(&self) -> i32 {
        // Java: return this.colorFore;
        self.colorFore
    }

    // Java: int getColorBack()
    pub fn getColorBack(&self) -> i32 {
        // Java: return this.colorBack;
        self.colorBack
    }

    // Java: int getPattern()
    pub fn getPattern(&self) -> i32 {
        // Java: return this.ipat;
        self.ipat
    }

    // Java: boolean equals(Object rhs)
    pub fn equals(&self, rhs: &SHD) -> bool {
        // Java: if (rhs == this)
        if std::ptr::eq(rhs, self) {
            // Java: return true;
            return true;
        // Java: else if (!(rhs instanceof SHD))
        } else if false {
            // Java: return false;
            return false;
        // Java: else
        } else {
            // Java: SHD r = (SHD)rhs;
            let r = rhs;
            // Java: return this.colorBack == r.colorBack && this.colorFore == r.colorFore && this.ipat == r.ipat;
            self.colorBack == r.colorBack && self.colorFore == r.colorFore && self.ipat == r.ipat
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return this.colorBack + this.colorFore + this.ipat;
        self.colorBack + self.colorFore + self.ipat
    }

    // Java: boolean isValid()
    pub fn isValid(&self) -> bool {
        // Java: boolean result = true;
        let mut result = true;
        // Java: if (this.ipat == 65535)
        if self.ipat == 65535 {
            // Java: result = false;
            result = false;
        // Java: else if (this.ipat == 0 && this.colorBack < 0)
        } else if self.ipat == 0 && self.colorBack < 0 {
            // Java: result = false;
            result = false;
        }
        // Java: return result;
        result
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: StringBuilder sb = new StringBuilder();
        let mut sb = String::new();
        // Java: sb.append("colorFore=0x" + Integer.toHexString(this.colorFore));
        sb.push_str(&format!("colorFore=0x{:x}", self.colorFore));
        // Java: sb.append(" colorBack=0x" + Integer.toHexString(this.colorBack));
        sb.push_str(&format!(" colorBack=0x{:x}", self.colorBack));
        // Java: sb.append(" ipat=" + this.ipat);
        sb.push_str(&format!(" ipat={}", self.ipat));
        // Java: return sb.toString();
        sb
    }

    // Java: boolean isPatSolidBlack()
    pub fn isPatSolidBlack(&self) -> bool {
        // Java: return this.ipat == 1 || this.ipat == 13 || this.ipat == 12 || this.ipat >= 56 && this.ipat <= 62;
        self.ipat == 1 || self.ipat == 13 || self.ipat == 12 || (self.ipat >= 56 && self.ipat <= 62)
    }

    // Java: static SHD obtain(int crFore, int crBack, int ipat)
    pub fn obtain(crFore: i32, crBack: i32, ipat: i32) -> SHD {
        let pool = sPool.get_or_init(|| Mutex::new(HashMap::new()));
        let temp = sTemp.get_or_init(|| Mutex::new(SHD::new()));
        let mut temp_guard = temp.lock().unwrap();
        // Java: sTemp.colorFore = crFore;
        temp_guard.colorFore = crFore;
        // Java: sTemp.colorBack = crBack;
        temp_guard.colorBack = crBack;
        // Java: sTemp.ipat = ipat;
        temp_guard.ipat = ipat;
        let mut pool_guard = pool.lock().unwrap();
        // Java: SHD cached = (SHD)sPool.get(sTemp);
        let cached = pool_guard.get(&*temp_guard).cloned();
        // Java: if (cached == null)
        if cached.is_none() {
            // Java: cached = new SHD(crFore, crBack, ipat);
            let new_shd = SHD::with_all(crFore, crBack, ipat);
            // Java: sPool.put(cached, cached);
            pool_guard.insert(new_shd.clone(), new_shd.clone());
            // Java: return cached;
            return new_shd;
        }
        // Java: return cached;
        cached.unwrap()
    }

    // Java: static SHD setColorFore(SHD old, int color)
    pub fn setColorFore(old: &SHD, color: i32) -> SHD {
        // Java: return obtain(color, old.getColorBack(), old.getPattern());
        SHD::obtain(color, old.getColorBack(), old.getPattern())
    }

    // Java: static SHD setColorBack(SHD old, int color)
    pub fn setColorBack(old: &SHD, color: i32) -> SHD {
        // Java: return obtain(old.getColorFore(), color, old.getPattern());
        SHD::obtain(old.getColorFore(), color, old.getPattern())
    }

    // Java: static SHD setPattern(SHD old, int pattern)
    pub fn setPattern(old: &SHD, pattern: i32) -> SHD {
        // Java: return obtain(old.getColorFore(), old.getColorBack(), pattern);
        SHD::obtain(old.getColorFore(), old.getColorBack(), pattern)
    }

    // Java: static SHD obtain()
    pub fn obtainDefault() -> &'static SHD {
        // Java: return sDefault;
        sDefault.get_or_init(|| SHD::new())
    }

    // Java: static void clear()
    pub fn clear() {
        // Java: sPool.clear();
        if let Some(pool) = sPool.get() {
            pool.lock().unwrap().clear();
        }
    }
}

impl PartialEq for SHD {
    fn eq(&self, other: &Self) -> bool {
        self.colorBack == other.colorBack && self.colorFore == other.colorFore && self.ipat == other.ipat
    }
}

impl Eq for SHD {}

impl std::hash::Hash for SHD {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_i32(self.hashCode());
    }
}

impl std::fmt::Display for SHD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.toString())
    }
}

mod IShareObjIndexdMap {
    pub trait IShareObjIndexed {
        fn setIndex(&mut self, index: i32);
        fn getIndex(&self) -> i32;
        fn getHostObj(&self) -> Option<&dyn std::any::Any>;
    }
}

impl IShareObjIndexdMap::IShareObjIndexed for SHD { // impl IShareObjIndexdMap.IShareObjIndexed for SHD
    fn setIndex(&mut self, index: i32) {
        self.mIndex = index;
    }

    fn getIndex(&self) -> i32 {
        self.mIndex
    }

    fn getHostObj(&self) -> Option<&dyn std::any::Any> {
        Some(self)
    }
}
