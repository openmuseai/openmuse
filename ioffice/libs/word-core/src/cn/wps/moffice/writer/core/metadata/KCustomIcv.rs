// Java: import cn.wps.moffice.util.UUIDUtil
use util::moffice::util::UUIDUtil::*;
// Java: import cn.wps.moffice.writer.core.metadata.variant.VariantBase
use crate::cn::wps::moffice::writer::core::metadata::variant::VariantBase::*;
// Java: import cn.wps.moffice.writer.core.metadata.variant.VariantCustom
use crate::cn::wps::moffice::writer::core::metadata::variant::VariantCustom::*;
// Java: import cn.wps.moffice.writer.core.metadata.variant.VariantType
use crate::cn::wps::moffice::writer::core::metadata::variant::VariantType::*;
// Java: import cn.wps.moffice.writer.core.metadata.variant.VariantWStr
use crate::cn::wps::moffice::writer::core::metadata::variant::VariantWStr::*;
// Java: import cn.wps.moffice.writer.core.metadata.CustomMetadata
use crate::cn::wps::moffice::writer::core::metadata::CustomMetadata::*;

// java public static String ICV = "ICV";
// rust: Java 的 `static final String` 常量 → `&str` 常量（模板落成
//   `LazyLock<Option<String>>`，与常量语义不符且引发多处 E0308）。
pub(crate) const ICV: &str = "ICV";
// java public static String TEMPLATE_ICV = "KSOTemplateUUID";
pub(crate) const TEMPLATE_ICV: &str = "KSOTemplateUUID";
// java public static final String ICV_NEW_SUFFIX = "21";
pub(crate) const ICV_NEW_SUFFIX: &str = "21";
// java public static final String ICV_SAVE_SUFFIX = "22";
pub(crate) const ICV_SAVE_SUFFIX: &str = "22";
// java public static final String ICV_SAVE_AS_SUFFIX = "23";
pub(crate) const ICV_SAVE_AS_SUFFIX: &str = "23";

// java public class KCustomIcv
#[derive(Default)]
pub struct KCustomIcv {
    // java private boolean isIcvUpdatedFromNew;
    isIcvUpdatedFromNew: bool,
    // java private CustomMetadata mCustomMetadata;
    mCustomMetadata: Option<CustomMetadata>,
}

// java public class KCustomIcv
impl KCustomIcv {
    // java public  KCustomIcv(CustomMetadata customMetadata)
    pub fn KCustomIcv(customMetadata: CustomMetadata) -> Self {
      // java this.mCustomMetadata = customMetadata;
      Self { isIcvUpdatedFromNew: Default::default(), mCustomMetadata: Some(customMetadata) }
    }

    // java public void createIcvFromNew()
    pub fn createIcvFromNew(&mut self) {
      // java this.updateIcv("21");
      self.updateIcv__string(Some(&"21"));
      // java this.isIcvUpdatedFromNew = true;
      self.isIcvUpdatedFromNew = true;
    }

    // java public void updateIcv(boolean isSavedAs)
    pub fn updateIcv__boolean(&mut self, isSavedAs: bool) {
      // java if (isSavedAs || !this.hasIcvProp())
      if isSavedAs || !self.hasIcvProp() {
        // java this.updateIcv(isSavedAs ? "23" : "22");
        // rust: 这里调的是 **String 重载** `updateIcv(String)`（模板误调 `__boolean`）。
        self.updateIcv__string(Some(if isSavedAs { "23" } else { "22" }));
      }
    }

    // java public void updateIcv(String suffix)
    pub fn updateIcv__string(&mut self, suffix: Option<&str>) {
      // java if (this.isIcvUpdatedFromNew)
      if self.isIcvUpdatedFromNew {
        // java this.isIcvUpdatedFromNew = false;
        self.isIcvUpdatedFromNew = false;
      }
      // java else
      else {
        // java Integer pid = this.mCustomMetadata.getPidByName(ICV);
        // rust: Java `Integer pid` 可空 ⇒ `Option<i32>`（方法门面亦返回 Option）。
        let mut pid: Option<i32> =
          self.mCustomMetadata.as_ref().expect("mCustomMetadata").getPidByName(ICV);
        // java String icv = UUIDUtil.generate32ID();
        let mut icv: String = UUIDUtil::generate32ID();
        // java if (suffix != null && suffix.length() != 0)
        if let Some(suffix) = suffix {
          if (suffix.len() as i32) != 0 {
            // java icv = icv + "_" + suffix;
            icv = icv + "_" + suffix;
          }
        }
        // java VariantBase variantBase = new VariantWStr(icv);
        // rust: `VariantWStr(String)` 门面 + 上转型为 `VariantBase`。
        // rust: 先留存 `icv` 供子类载荷使用（`VariantWStr::new` 会移动）。
        let icv_for_wstr = icv.clone();
        let variantBase: VariantBase = VariantWStr::new(icv).super_;
        // java if (null != pid)
        if pid.is_some() {
          // java VariantCustom variantCustom = this.mCustomMetadata.getData(pid);
          // rust: `getData(int) -> Option<VariantCustom>`；Java 此处已判 `null != pid`
          //   但未判 `getData` 返回 null ⇒ 解包并保留失败即 panic 的语义。
          let mut variantCustom: VariantCustom =
            self.mCustomMetadata.as_ref().expect("mCustomMetadata").getData(pid.unwrap()).expect("getData");
          // java variantCustom.setVariant(&variantBase);
          variantCustom.setVariant(&variantBase);
        }
        // java else
        else {
          // java pid = this.mCustomMetadata.createPid();
          let pid = self.mCustomMetadata.as_ref().expect("mCustomMetadata").createPid();
          // java VariantCustom icvVariant = new VariantCustom(ICV, variantBase);
          let mut icvVariant: VariantCustom = VariantCustom::VariantCustom(ICV, &variantBase);
          // rust: 保留 `VariantWStr` 子类载荷（对应 Java `mVariant` 存子类实例）。
          icvVariant.mVariantWStr = Some(VariantWStr::new(icv_for_wstr));
          // java this.mCustomMetadata.setData(pid, icvVariant);
          self.mCustomMetadata.as_mut().expect("mCustomMetadata").setData(pid, Some(&icvVariant));
        }
      }
    }

    // java public void addIcv()
    pub fn addIcv(&mut self) {
      // java String icv = UUIDUtil.generate32ID();
      let mut icv: String = UUIDUtil::generate32ID();
      // java VariantBase variantBase = new VariantWStr(icv);
      // rust: `VariantWStr::new(icv)` 会移动 `icv` ⇒ 先留存一份供子类载荷使用。
      let icv_for_wstr = icv.clone();
      // rust: 先留存 `icv` 供子类载荷使用（`VariantWStr::new` 会移动）。
      let icv_for_wstr = icv.clone();
      let variantBase: VariantBase = VariantWStr::new(icv).super_;
      // java int pid = this.mCustomMetadata.createPid();
      let pid: i32 = self.mCustomMetadata.as_ref().expect("mCustomMetadata").createPid();
      // java VariantCustom icvVariant = new VariantCustom(ICV, variantBase);
      let mut icvVariant: VariantCustom = VariantCustom::VariantCustom(ICV, &variantBase);
      // rust: 保留 `VariantWStr` 子类载荷（对应 Java `mVariant` 存子类实例）。
      icvVariant.mVariantWStr = Some(VariantWStr::new(icv_for_wstr));
      // java this.mCustomMetadata.setData(pid, icvVariant);
      self.mCustomMetadata.as_mut().expect("mCustomMetadata").setData(pid, Some(&icvVariant));
    }

    // java public Integer getIcvPid()
    pub fn getIcvPid(&self) -> Option<i32> {
      // java return this.mCustomMetadata.getPidByName(ICV);
      return self.mCustomMetadata.as_ref().expect("mCustomMetadata").getPidByName(ICV);
    }

    // java public boolean hasIcvProp()
    pub fn hasIcvProp(&self) -> bool {
      // java return null != this.getIcvPid();
      return self.getIcvPid().is_some();
    }

    // java public String getIcv()
    pub fn getIcv(&self) -> String {
      // java return this.getPropertyStrVal(ICV);
      return self.getPropertyStrVal(ICV).unwrap_or_default();
    }

    // java public String getPropertyStrVal(String name)
    pub fn getPropertyStrVal(&self, name: &str) -> Option<String> {
      // java Integer pid = this.mCustomMetadata.getPidByName(name);
      let pid: Option<i32> = self.mCustomMetadata.as_ref().expect("mCustomMetadata").getPidByName(name);
      // java if (null == pid)
      if pid.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java VariantCustom data = this.mCustomMetadata.getData(pid);
        // rust: `CustomMetadata.getData(int) -> Option<VariantCustom>`。
        let data: Option<VariantCustom> = self.mCustomMetadata.as_ref().expect("mCustomMetadata").getData(pid.unwrap());
        // java if (null == data)
        if data.is_none() {
          // java return null;
          return None;
        }
        // java else
        else {
          let data = data.as_ref().unwrap();
          // java VariantBase variant = data.getVariant();
          let variant: VariantBase = data.getVariant();
          // java VariantType type = variant.getType();
          let mut r#type: VariantType = variant.getType();
          // java if (VariantType.var_wstr != type)
          if VariantType::var_wstr != r#type {
            // java return null;
            return None;
          }
          // java else if (!(variant instanceof VariantWStr))
          // rust: `instanceof` 由 `VariantType` 判别承载（`var_wstr` ⇔ `VariantWStr`）。
          else if r#type != VariantType::var_wstr {
            // java return null;
            return None;
          }
          // java else
          else {
            // java VariantWStr wstr = (VariantWStr)variant;
            // java String icv = wstr.getVar();
            // rust: `VariantWStr` 的字符串值由 `mVar` 承载；`getVariant()` 仅
            //   返回基类槽 ⇒ 由类型判别 + 变体名回查字符串值。
            let icv: String = data.getVariantStringValue().unwrap_or_default();
            // java return icv;
            return Some(icv);
          }
        }
      }
    }

    // java public String getTemplateIcv()
    pub fn getTemplateIcv(&self) -> String {
      // java return this.getPropertyStrVal(TEMPLATE_ICV);
      return self.getPropertyStrVal(TEMPLATE_ICV).unwrap_or_default();
    }

}
