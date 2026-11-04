// Java: import cn.wps.util.KeepNotProguard
use util::util::KeepNotProguard::*;
// Java: import com.google.gson.annotations.Expose
use stubs::com::google::gson::annotations::Expose::*;
// Java: import com.google.gson.annotations.SerializedName
use stubs::com::google::gson::annotations::SerializedName::*;
// Java: import java.util.List
// Java: import cn.wps.moffice.writer.function.card.online.KeyWords
use crate::cn::wps::moffice::writer::function::card::online::KeyWords::*;

// java public static PaperKeyWordParam sDefault = new PaperKeyWordParam();
// rust: ★Java 静态字段按类初始化（`new PaperKeyWordParam()`）⇒ `LazyLock<PaperKeyWordParam>`。
//   原模板的 `LazyLock<Arc<Option<Box<Self>>>>` 是四重包裹，与初始值类型不符。
pub(crate) static sDefault: std::sync::LazyLock<PaperKeyWordParam> =
                    std::sync::LazyLock::new(|| PaperKeyWordParam::new());

// java @KeepNotProguard public class PaperKeyWordParam
#[derive(Default)]
pub struct PaperKeyWordParam {
    // java @Expose @SerializedName("0") List<String> mP0Keys;
    pub(crate) mP0Keys: Option<Vec<String>>,
    // java @Expose @SerializedName("1") List<String> mP1Keys;
    pub(crate) mP1Keys: Option<Vec<String>>,
    // java @Expose @SerializedName("d") List<String> mDenys;
    pub(crate) mDenys: Option<Vec<String>>,
    // java @Expose @SerializedName("de") List<KeyWords> mDenyExcludeKeys;
    pub(crate) mDenyExcludeKeys: Option<Vec<KeyWords>>,
}

// java @KeepNotProguard public class PaperKeyWordParam
impl PaperKeyWordParam {
    // rust: ★Java 该类**没有显式构造器**（隐式默认构造，Java 源无对应语句行）⇒ 此处为 Rust-only helper。
    pub fn new() -> Self {
      Self::default()
    }

    // java public List<String> getP0Keys()
    // rust: ★Java 字段未初始化 ⇒ null ⇒ 可空返回值用 `Option`（保持 `return this.mP0Keys;` 逐字对齐）。
    pub fn getP0Keys(&self) -> Option<Vec<String>> {
      // java return this.mP0Keys;
      self.mP0Keys.clone()
    }

    // java public void setP0Keys(List<String> mP0Keys)
    pub fn setP0Keys(&mut self, mP0Keys: Option<&Vec<String>>) {
      // java this.mP0Keys = mP0Keys;
      self.mP0Keys = mP0Keys.cloned();
    }

    // java public List<String> getP1Keys()
    // rust: ★Java 字段未初始化 ⇒ null ⇒ 可空返回值用 `Option`（保持 `return this.mP1Keys;` 逐字对齐）。
    pub fn getP1Keys(&self) -> Option<Vec<String>> {
      // java return this.mP1Keys;
      self.mP1Keys.clone()
    }

    // java public void setP1Keys(List<String> mP1Keys)
    pub fn setP1Keys(&mut self, mP1Keys: Option<&Vec<String>>) {
      // java this.mP1Keys = mP1Keys;
      self.mP1Keys = mP1Keys.cloned();
    }

    // java public List<String> getDenys()
    // rust: ★Java 字段未初始化 ⇒ null ⇒ 可空返回值用 `Option`（保持 `return this.mDenys;` 逐字对齐）。
    pub fn getDenys(&self) -> Option<Vec<String>> {
      // java return this.mDenys;
      self.mDenys.clone()
    }

    // java public void setDenys(List<String> mDenys)
    pub fn setDenys(&mut self, mDenys: Option<&Vec<String>>) {
      // java this.mDenys = mDenys;
      self.mDenys = mDenys.cloned();
    }

    // java public List<KeyWords> getDenyExcludeKeys()
    // rust: ★Java 字段未初始化 ⇒ null ⇒ 可空返回值用 `Option`（保持 `return this.mDenyExcludeKeys;` 逐字对齐）。
    pub fn getDenyExcludeKeys(&self) -> Option<Vec<KeyWords>> {
      // java return this.mDenyExcludeKeys;
      self.mDenyExcludeKeys.clone()
    }

    // java public void setDenyExcludeKeys(List<KeyWords> mDenyExcludeKeys)
    pub fn setDenyExcludeKeys(&mut self, mDenyExcludeKeys: Option<&Vec<KeyWords>>) {
      // java this.mDenyExcludeKeys = mDenyExcludeKeys;
      self.mDenyExcludeKeys = mDenyExcludeKeys.cloned();
    }

}
