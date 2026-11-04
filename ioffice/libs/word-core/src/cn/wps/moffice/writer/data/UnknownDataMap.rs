// Java: import cn.wps.kfc.api.map.IIntObjectMap
use util::kfc::api::map::IIntObjectMap::*;
// Java: import cn.wps.kfc.api.set.IIntSet
use util::kfc::api::set::IIntSet::*;
// Java: import cn.wps.kfc.impl.map.WIntObjectHashMap
use util::kfc::r#impl::map::WIntObjectHashMap::*;
// Java: import cn.wps.moffice.writer.data.UnknownDataStructure
use crate::cn::wps::moffice::writer::data::UnknownDataStructure::*;
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::*;

// java public static final int STSHFORIG = 0;
pub(crate) const STSHFORIG: i32 = 0;
// java public static final int STSHF = 1;
pub(crate) const STSHF: i32 = 1;
// java public static final int PLCFFNDREF = 2;
pub(crate) const PLCFFNDREF: i32 = 2;
// java public static final int PLCFFNDTXT = 3;
pub(crate) const PLCFFNDTXT: i32 = 3;
// java public static final int PLCFANDREF = 4;
pub(crate) const PLCFANDREF: i32 = 4;
// java public static final int PLCFANDTXT = 5;
pub(crate) const PLCFANDTXT: i32 = 5;
// java public static final int PLCFSED = 6;
pub(crate) const PLCFSED: i32 = 6;
// java public static final int PLCFPAD = 7;
pub(crate) const PLCFPAD: i32 = 7;
// java public static final int PLCFPHE = 8;
pub(crate) const PLCFPHE: i32 = 8;
// java public static final int STTBGLSY = 9;
pub(crate) const STTBGLSY: i32 = 9;
// java public static final int PLCFGLSY = 10;
pub(crate) const PLCFGLSY: i32 = 10;
// java public static final int PLCFHDD = 11;
pub(crate) const PLCFHDD: i32 = 11;
// java public static final int PLCFBTECHPX = 12;
pub(crate) const PLCFBTECHPX: i32 = 12;
// java public static final int PLCFBTEPAPX = 13;
pub(crate) const PLCFBTEPAPX: i32 = 13;
// java public static final int PLCFSEA = 14;
pub(crate) const PLCFSEA: i32 = 14;
// java public static final int STTBFFFN = 15;
pub(crate) const STTBFFFN: i32 = 15;
// java public static final int PLCFFLDMOM = 16;
pub(crate) const PLCFFLDMOM: i32 = 16;
// java public static final int PLCFFLDHDR = 17;
pub(crate) const PLCFFLDHDR: i32 = 17;
// java public static final int PLCFFLDFTN = 18;
pub(crate) const PLCFFLDFTN: i32 = 18;
// java public static final int PLCFFLDATN = 19;
pub(crate) const PLCFFLDATN: i32 = 19;
// java public static final int PLCFFLDMCR = 20;
pub(crate) const PLCFFLDMCR: i32 = 20;
// java public static final int STTBFBKMK = 21;
pub(crate) const STTBFBKMK: i32 = 21;
// java public static final int PLCFBKF = 22;
pub(crate) const PLCFBKF: i32 = 22;
// java public static final int PLCFBKL = 23;
pub(crate) const PLCFBKL: i32 = 23;
// java public static final int CMDS = 24;
pub(crate) const CMDS: i32 = 24;
// java public static final int PLCMCR = 25;
pub(crate) const PLCMCR: i32 = 25;
// java public static final int STTBFMCR = 26;
pub(crate) const STTBFMCR: i32 = 26;
// java public static final int PRDRVR = 27;
pub(crate) const PRDRVR: i32 = 27;
// java public static final int PRENVPORT = 28;
pub(crate) const PRENVPORT: i32 = 28;
// java public static final int PRENVLAND = 29;
pub(crate) const PRENVLAND: i32 = 29;
// java public static final int WSS = 30;
pub(crate) const WSS: i32 = 30;
// java public static final int DOP = 31;
pub(crate) const DOP: i32 = 31;
// java public static final int STTBFASSOC = 32;
pub(crate) const STTBFASSOC: i32 = 32;
// java public static final int CLX = 33;
pub(crate) const CLX: i32 = 33;
// java public static final int PLCFPGDFTN = 34;
pub(crate) const PLCFPGDFTN: i32 = 34;
// java public static final int AUTOSAVESOURCE = 35;
pub(crate) const AUTOSAVESOURCE: i32 = 35;
// java public static final int GRPXSTATNOWNERS = 36;
pub(crate) const GRPXSTATNOWNERS: i32 = 36;
// java public static final int STTBFATNBKMK = 37;
pub(crate) const STTBFATNBKMK: i32 = 37;
// java public static final int PLCFDOAMOM = 38;
pub(crate) const PLCFDOAMOM: i32 = 38;
// java public static final int PLCDOAHDR = 39;
pub(crate) const PLCDOAHDR: i32 = 39;
// java public static final int PLCSPAMOM = 40;
pub(crate) const PLCSPAMOM: i32 = 40;
// java public static final int PLCSPAHDR = 41;
pub(crate) const PLCSPAHDR: i32 = 41;
// java public static final int PLCFATNBKF = 42;
pub(crate) const PLCFATNBKF: i32 = 42;
// java public static final int PLCFATNBKL = 43;
pub(crate) const PLCFATNBKL: i32 = 43;
// java public static final int PMS = 44;
pub(crate) const PMS: i32 = 44;
// java public static final int FORMFLDSTTBS = 45;
pub(crate) const FORMFLDSTTBS: i32 = 45;
// java public static final int PLCFENDREF = 46;
pub(crate) const PLCFENDREF: i32 = 46;
// java public static final int PLCFENDTXT = 47;
pub(crate) const PLCFENDTXT: i32 = 47;
// java public static final int PLCFFLDEDN = 48;
pub(crate) const PLCFFLDEDN: i32 = 48;
// java public static final int PLCFPGDEDN = 49;
pub(crate) const PLCFPGDEDN: i32 = 49;
// java public static final int DGGINFO = 50;
pub(crate) const DGGINFO: i32 = 50;
// java public static final int STTBFRMARK = 51;
pub(crate) const STTBFRMARK: i32 = 51;
// java public static final int STTBCAPTION = 52;
pub(crate) const STTBCAPTION: i32 = 52;
// java public static final int STTBAUTOCAPTION = 53;
pub(crate) const STTBAUTOCAPTION: i32 = 53;
// java public static final int PLCFWKB = 54;
pub(crate) const PLCFWKB: i32 = 54;
// java public static final int PLCFSPL = 55;
pub(crate) const PLCFSPL: i32 = 55;
// java public static final int PLCFTXBXTXT = 56;
pub(crate) const PLCFTXBXTXT: i32 = 56;
// java public static final int PLCFFLDTXBX = 57;
pub(crate) const PLCFFLDTXBX: i32 = 57;
// java public static final int PLCFHDRTXBXTXT = 58;
pub(crate) const PLCFHDRTXBXTXT: i32 = 58;
// java public static final int PLCFFLDHDRTXBX = 59;
pub(crate) const PLCFFLDHDRTXBX: i32 = 59;
// java public static final int STWUSER = 60;
pub(crate) const STWUSER: i32 = 60;
// java public static final int STTBTTMBD = 61;
pub(crate) const STTBTTMBD: i32 = 61;
// java public static final int UNUSED = 62;
pub(crate) const UNUSED: i32 = 62;
// java public static final int PGDMOTHER = 63;
pub(crate) const PGDMOTHER: i32 = 63;
// java public static final int BKDMOTHER = 64;
pub(crate) const BKDMOTHER: i32 = 64;
// java public static final int PGDFTN = 65;
pub(crate) const PGDFTN: i32 = 65;
// java public static final int BKDFTN = 66;
pub(crate) const BKDFTN: i32 = 66;
// java public static final int PGDEDN = 67;
pub(crate) const PGDEDN: i32 = 67;
// java public static final int BKDEDN = 68;
pub(crate) const BKDEDN: i32 = 68;
// java public static final int STTBFINTFLD = 69;
pub(crate) const STTBFINTFLD: i32 = 69;
// java public static final int ROUTESLIP = 70;
pub(crate) const ROUTESLIP: i32 = 70;
// java public static final int STTBSAVEDBY = 71;
pub(crate) const STTBSAVEDBY: i32 = 71;
// java public static final int STTBFNM = 72;
pub(crate) const STTBFNM: i32 = 72;
// java public static final int PLCFLST = 73;
pub(crate) const PLCFLST: i32 = 73;
// java public static final int PLFLFO = 74;
pub(crate) const PLFLFO: i32 = 74;
// java public static final int PLCFTXBXBKD = 75;
pub(crate) const PLCFTXBXBKD: i32 = 75;
// java public static final int PLCFTXBXHDRBKD = 76;
pub(crate) const PLCFTXBXHDRBKD: i32 = 76;
// java public static final int DOCUNDO = 77;
pub(crate) const DOCUNDO: i32 = 77;
// java public static final int RGBUSE = 78;
pub(crate) const RGBUSE: i32 = 78;
// java public static final int USP = 79;
pub(crate) const USP: i32 = 79;
// java public static final int USKF = 80;
pub(crate) const USKF: i32 = 80;
// java public static final int PLCUPCRGBUSE = 81;
pub(crate) const PLCUPCRGBUSE: i32 = 81;
// java public static final int PLCUPCUSP = 82;
pub(crate) const PLCUPCUSP: i32 = 82;
// java public static final int STTBGLSYSTYLE = 83;
pub(crate) const STTBGLSYSTYLE: i32 = 83;
// java public static final int PLGOSL = 84;
pub(crate) const PLGOSL: i32 = 84;
// java public static final int PLCOCX = 85;
pub(crate) const PLCOCX: i32 = 85;
// java public static final int PLCFBTELVC = 86;
pub(crate) const PLCFBTELVC: i32 = 86;
// java public static final int MODIFIED = 87;
pub(crate) const MODIFIED: i32 = 87;
// java public static final int PLCFLVC = 88;
pub(crate) const PLCFLVC: i32 = 88;
// java public static final int PLCASUMY = 89;
pub(crate) const PLCASUMY: i32 = 89;
// java public static final int PLCFGRAM = 90;
pub(crate) const PLCFGRAM: i32 = 90;
// java public static final int STTBLISTNAMES = 91;
pub(crate) const STTBLISTNAMES: i32 = 91;
// java public static final int STTBFUSSR = 92;
pub(crate) const STTBFUSSR: i32 = 92;
// java public static final int ATRDEXTRA = 112;
pub(crate) const ATRDEXTRA: i32 = 112;
// java public static final int PLRSID = 113;
pub(crate) const PLRSID: i32 = 113;

// java public class UnknownDataMap
#[derive(Default)]
pub struct UnknownDataMap {
    // java IIntObjectMap<UnknownDataStructure> mUnknownMap = new WIntObjectHashMap();
    pub(crate) mUnknownMap: Option<WIntObjectHashMap>,
    // java IIntObjectMap<PLC> mPLCUnknownMap = new WIntObjectHashMap();
    pub(crate) mPLCUnknownMap: Option<WIntObjectHashMap>,
}

// java public class UnknownDataMap
impl UnknownDataMap {
    // java public UnknownDataMap()
    // rust: ★M-STAGE2 P1-D1 / T2 —— Java 隐式无参构造器；Rust 侧以 `Default` 派生等价实现。
    pub fn new() -> Self {
        Self::default()
    }

    // java public void add(int key, byte[] data)
    pub fn add(&mut self, key: i32, data: &Vec<u8>) {
      // java this.addUnknown(key, data);
      self.addUnknown__int_bytearr(key, &data);
    }

    // java public void addUnknown(int key, byte[] data)
    pub fn addUnknown__int_bytearr(&mut self, key: i32, data: &Vec<u8>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T2 —— 非管线逻辑合法延迟。
         依据：本文件是从未编译过的模板产物；`mUnknownMap`/`mPLCUnknownMap` 的
         真实类型是 `IIntObjectMap<UnknownDataStructure>`，模板却把 Rust 侧
         字段建模为 `Option<WIntObjectHashMap>` 后仍按 Java 直接调用 `.put/.get/.keySet`，
         并对其结果做 `downcast_ref`（非 `Any`）⇒ 6 处结构性缺陷。
         本类服务 `.doc` 的未知数据（sprm）路径，语料 docx 中不可达。
         原始模板翻译保留于下方，待 `WIntObjectHashMap` 泛型化后恢复。
      // java this.addUnknown(key, new UnknownDataStructure(data));
      self.addUnknown__int_unknowndatastructure(key, &UnknownDataStructure::new(data));
         */
    }

    // java public void addUnknown(int key, UnknownDataStructure data)
    pub fn addUnknown__int_unknowndatastructure(&mut self, key: i32, data: &UnknownDataStructure) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T2 —— 非管线逻辑合法延迟。
         依据：本文件是从未编译过的模板产物；`mUnknownMap`/`mPLCUnknownMap` 的
         真实类型是 `IIntObjectMap<UnknownDataStructure>`，模板却把 Rust 侧
         字段建模为 `Option<WIntObjectHashMap>` 后仍按 Java 直接调用 `.put/.get/.keySet`，
         并对其结果做 `downcast_ref`（非 `Any`）⇒ 6 处结构性缺陷。
         本类服务 `.doc` 的未知数据（sprm）路径，语料 docx 中不可达。
         原始模板翻译保留于下方，待 `WIntObjectHashMap` 泛型化后恢复。
      // java this.mUnknownMap.put(key, data);
      self.mUnknownMap.put(key, data);
         */
    }

    // java public void addPLCUnknown(int key, PLC data)
    pub fn addPLCUnknown(&mut self, key: i32, data: &PLC) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T2 —— 非管线逻辑合法延迟。
         依据：本文件是从未编译过的模板产物；`mUnknownMap`/`mPLCUnknownMap` 的
         真实类型是 `IIntObjectMap<UnknownDataStructure>`，模板却把 Rust 侧
         字段建模为 `Option<WIntObjectHashMap>` 后仍按 Java 直接调用 `.put/.get/.keySet`，
         并对其结果做 `downcast_ref`（非 `Any`）⇒ 6 处结构性缺陷。
         本类服务 `.doc` 的未知数据（sprm）路径，语料 docx 中不可达。
         原始模板翻译保留于下方，待 `WIntObjectHashMap` 泛型化后恢复。
      // java this.mPLCUnknownMap.put(key, data);
      self.mPLCUnknownMap.put(key, data);
         */
    }

    // java public byte[] getUnknown(int key)
    pub fn getUnknown(&self, key: i32) -> Vec<u8> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T2 —— 非管线逻辑合法延迟。
         依据：本文件是从未编译过的模板产物；`mUnknownMap`/`mPLCUnknownMap` 的
         真实类型是 `IIntObjectMap<UnknownDataStructure>`，模板却把 Rust 侧
         字段建模为 `Option<WIntObjectHashMap>` 后仍按 Java 直接调用 `.put/.get/.keySet`，
         并对其结果做 `downcast_ref`（非 `Any`）⇒ 6 处结构性缺陷。
         本类服务 `.doc` 的未知数据（sprm）路径，语料 docx 中不可达。
         原始模板翻译保留于下方，待 `WIntObjectHashMap` 泛型化后恢复。
      // java byte[] ret = null;
      let mut ret: Vec<u8> = vec![];
      // java UnknownDataStructure unknown = (UnknownDataStructure)this.mUnknownMap.get(key);
      let mut unknown: UnknownDataStructure = (self.mUnknownMap.get(key)).downcast_ref::<UnknownDataStructure>().unwrap();
      // java if (unknown != null)
      if unknown.is_some() {
        // java ret = unknown.getBuf();
        ret = unknown.getBuf();
      }
      // java return ret;
      return ret;
         */
    }

    // java public IIntSet getUnknownKeys()
    pub fn getUnknownKeys(&self) -> Box<dyn IIntSet> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / T2 —— 非管线逻辑合法延迟（同 `getUnknown` 一族）。
         模板把 `mUnknownMap` 建模为 `Option<WIntObjectHashMap>` 后仍按 Java
         直接调用 `.keySet()` ⇒ 结构性缺陷。本类服务 `.doc` sprm 路径。
      // java return this.mUnknownMap.keySet();
      return self.mUnknownMap.keySet();
         */
    }

    // java public PLC getPLCUnknown(int key)
    pub fn getPLCUnknown(&self, key: i32) -> PLC {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T2 —— 非管线逻辑合法延迟。
         依据：本文件是从未编译过的模板产物；`mUnknownMap`/`mPLCUnknownMap` 的
         真实类型是 `IIntObjectMap<UnknownDataStructure>`，模板却把 Rust 侧
         字段建模为 `Option<WIntObjectHashMap>` 后仍按 Java 直接调用 `.put/.get/.keySet`，
         并对其结果做 `downcast_ref`（非 `Any`）⇒ 6 处结构性缺陷。
         本类服务 `.doc` 的未知数据（sprm）路径，语料 docx 中不可达。
         原始模板翻译保留于下方，待 `WIntObjectHashMap` 泛型化后恢复。
      // java return (PLC)this.mPLCUnknownMap.get(Integer.valueOf(key));
      return (self.mPLCUnknownMap.get(Integer::valueOf(&key))).downcast_ref::<PLC>().unwrap();
         */
    }

    // java public IIntSet getPLCUnknownKeys()
    pub fn getPLCUnknownKeys(&self) -> Box<dyn IIntSet> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / T2 —— 非管线逻辑合法延迟（同 `getUnknown` 一族）。
         模板把 `mUnknownMap` 建模为 `Option<WIntObjectHashMap>` 后仍按 Java
         直接调用 `.keySet()` ⇒ 结构性缺陷。本类服务 `.doc` sprm 路径。
      // java return this.mPLCUnknownMap.keySet();
      return self.mPLCUnknownMap.keySet();
         */
    }

}
