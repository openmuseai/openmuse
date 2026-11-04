#![allow(non_snake_case, dead_code, unused_variables)]

// Java: import cn.wps.core.runtime.Platform
// Java: import cn.wps.moffice.kfs.FileInputStream
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.util.HashMap
// Java: import java.util.Map
// Java: import java.util.Objects

use std::collections::HashMap;
use util_adapter::cn::wps::core::runtime::Platform::Platform;

// Java field: String JAR_RESOURCE
pub(crate) const JAR_RESOURCE: &str = "";
// Java field: String ASSERTS_RESOURCE
pub(crate) const ASSERTS_RESOURCE: &str = "";

// Java: public class HostTheme
pub struct HostTheme {
    // Java field: String mTheme
    pub(crate) mTheme: Option<String>,
    // Java field: Map<String, Integer> mMediaMap
    pub(crate) mMediaMap: HashMap<String, i32>,
}

// Java: public class HostTheme
impl HostTheme {
    // Java: HostTheme()
    pub fn new() -> Self {
      // Java: super();
      HostTheme { mTheme: None, mMediaMap: HashMap::new() }
    }

    // Java: void setTheme(String themePath)
    pub fn setTheme(&mut self, themePath: String) {
      // Java: this.mTheme = themePath;
      self.mTheme = Some(themePath);
    }

    // Java: String theme()
    pub fn theme(&self) -> String {
      // Java: return this.mTheme;
      self.mTheme.clone().unwrap_or_default()
    }

    // Java: InputStream themeSteam()
    pub fn themeSteam(&self) -> Result<Option<Box<dyn std::io::Read>>, std::io::Error> {
      // Java: if (this.mTheme == null)
      if self.mTheme.is_none() {
         // Java: return null;
         return Ok(None);
      }
      // Java: else if (this.mTheme.startsWith("file:"))
      else if self.mTheme.as_ref().unwrap().starts_with("file:") {
         // Java: return HostTheme.class.getResourceAsStream(this.mTheme);
         return Ok(None);
      }
      // Java: else if (this.mTheme.startsWith("assets:"))
      else if self.mTheme.as_ref().unwrap().starts_with("assets:") {
         // Java: String path = this.mTheme.substring("assets:".length());
         let path = &self.mTheme.as_ref().unwrap()["assets:".len()..];
         // Java: return Platform.getAssetManager().open(path);
         let _manager = Platform::getAssetManager();
         return Ok(None);
      }
      // Java: else
      else {
         // Java: return new FileInputStream(this.mTheme);
         let path = self.mTheme.as_ref().unwrap();
         let f = std::fs::File::open(path)?;
         Ok(Some(Box::new(f)))
      }
    }

    // Java: void addMediaItem(String rID, int mediaID)
    pub fn addMediaItem(&mut self, rID: String, mediaID: i32) {
      // Java: this.mMediaMap.put(rID, mediaID);
      self.mMediaMap.insert(rID, mediaID);
    }

    // Java: void removeMediaItem(String rID)
    pub fn removeMediaItem(&mut self, rID: &str) {
      // Java: this.mMediaMap.remove(rID);
      self.mMediaMap.remove(rID);
    }

    // Java: public Map<String, Integer> mediaMap()
    pub fn mediaMap(&self) -> HashMap<String, i32> {
      // Java: return new HashMap(this.mMediaMap);
      self.mMediaMap.clone()
    }

    // Java: int mediaID(String rID)
    pub fn mediaID(&self, rID: &str) -> i32 {
      // Java: Integer mediaID = (Integer)this.mMediaMap.get(rID);
      let mediaID = self.mMediaMap.get(rID);
      // Java: return mediaID != null ? mediaID : -1;
      mediaID.copied().unwrap_or(-1)
    }

    // Java: void copyFrom(HostTheme src)
    pub fn copyFrom(&mut self, src: Option<&HostTheme>) {
      // Java: if (src != null)
      if let Some(src) = src {
         // Java: this.mTheme = src.theme();
         self.mTheme = src.mTheme.clone();
         // Java: this.mMediaMap.clear();
         self.mMediaMap.clear();
         // Java: this.mMediaMap.putAll(src.mMediaMap);
         self.mMediaMap.extend(src.mMediaMap.iter().map(|(k, v)| (k.clone(), *v)));
      }
    }

    // Java: void update(String themePath, Map<String, Integer> rids)
    pub fn update(&mut self, themePath: String, rids: Option<&HashMap<String, i32>>) {
      // Java: this.mTheme = themePath;
      self.mTheme = Some(themePath);
      // Java: this.mMediaMap.clear();
      self.mMediaMap.clear();
      // Java: if (rids != null)
      if let Some(rids) = rids {
         // Java: this.mMediaMap.putAll(rids);
         self.mMediaMap.extend(rids.iter().map(|(k, v)| (k.clone(), *v)));
      }
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: Option<&HostTheme>) -> bool {
      // Java: if (o != null && o instanceof HostTheme)
      if o.is_some() && o.is_some() {
         // Java: HostTheme hostTheme = (HostTheme)o;
         let hostTheme = o.unwrap();
         // Java: String theme = this.mTheme != null ? this.mTheme : "";
         let theme = self.mTheme.as_deref().unwrap_or("");
         // Java: String oTheme = hostTheme.mTheme != null ? hostTheme.mTheme : "";
         let oTheme = hostTheme.mTheme.as_deref().unwrap_or("");
         // Java: return theme.equals(oTheme) && this.mMediaMap.equals(hostTheme.mMediaMap);
         return theme == oTheme && self.mMediaMap == hostTheme.mMediaMap;
      }
      // Java: else
      else {
         // Java: return false;
         false
      }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: return this.mTheme == null ? Objects.hash(new Object[]{"", this.mMediaMap}) : Objects.hash(new Object[]{this.mTheme, this.mMediaMap});
      use std::collections::hash_map::DefaultHasher;
      use std::hash::{Hash, Hasher};
      let mut hasher = DefaultHasher::new();
      self.mTheme.as_deref().unwrap_or("").hash(&mut hasher);
      hasher.finish() as i32
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: StringBuilder builder = new StringBuilder();
      let mut builder = String::new();
      // Java: builder.append(this.mTheme != null ? this.mTheme : "");
      builder.push_str(self.mTheme.as_deref().unwrap_or(""));
      // Java: builder.append(this.mMediaMap.toString());
      builder.push_str(&format!("{:?}", self.mMediaMap));
      // Java: return builder.toString();
      builder
    }

}
