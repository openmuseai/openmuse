// Java: import cn.wps.util.KeepNotProguard
use util::util::KeepNotProguard::*;
// Java: import com.google.gson.annotations.Expose
use stubs::com::google::gson::annotations::Expose::*;
// Java: import com.google.gson.annotations.SerializedName
use stubs::com::google::gson::annotations::SerializedName::*;

// rust: ★T7-5a-3 —— 原文件是自动生成的 **纯 stub**（`todo!()`，无字段），已按 Java 真实改写。
// java @KeepNotProguard
// java public class KeyWords
#[derive(Default, Clone)]
pub struct KeyWords {
    // java @Expose
    // java @SerializedName("key")
    // java String key;
    // rust: ★Java 字段未初始化 ⇒ null；`setKey` 之前读为 null ⇒ Rust 用 `Option`。
    pub(crate) key: Option<String>,
    // java @Expose
    // java @SerializedName("words")
    // java String words;
    pub(crate) words: Option<String>,
}

// java public class KeyWords
impl KeyWords {
    // rust: ★Java 该类**没有显式构造器**（隐式默认构造，Java 源无对应语句行）⇒ 此处为 Rust-only helper。
    pub fn new() -> Self {
      Self::default()
    }

    // java public String getKey()
    pub fn getKey(&self) -> Option<String> {
      // java return this.key;
      self.key.clone()
    }

    // java public void setKey(String key)
    pub fn setKey(&mut self, key: Option<&str>) {
      // java this.key = key;
      self.key = key.map(|__s| __s.to_string());
    }

    // java public String getWords()
    pub fn getWords(&self) -> Option<String> {
      // java return this.words;
      self.words.clone()
    }

    // java public void setWords(String words)
    pub fn setWords(&mut self, words: Option<&str>) {
      // java this.words = words;
      self.words = words.map(|__s| __s.to_string());
    }
}
