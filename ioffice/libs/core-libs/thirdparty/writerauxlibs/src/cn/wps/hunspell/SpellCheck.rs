// Java: import cn.wps.moffice.kfs.File
// use kfs_kfs_app::cn::wps::moffice::kfs::mfs::File; // not available in this crate
// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import java.util.Map
use std::path::Path;

use super::CSUtil::CSUtil;
use super::HashMgr::HashMgr;

// Java: public interface IHunSpell
pub trait IHunSpell {
    fn spell(&self, var1: &str) -> i32;
    fn suggest(&self, var1: &str) -> Vec<String>;
    fn addUsrWord(&mut self, var1: &str);
}

// Java: public interface ISpellCheck
pub trait ISpellCheck {
    fn setDictPath(&mut self, var1: &str);
    fn getSpell(&mut self, var1: i32) -> Option<&dyn IHunSpell>;
}

// Placeholder: HunSpell is being converted by another worker
pub(crate) struct HunSpell;

#[allow(non_snake_case)]
impl HunSpell {
    pub fn new(_aff: &str, _dic: &str) -> Self {
        todo!("waiting for HunSpell conversion")
    }
    pub fn addExtDic(&mut self, _dic: &str) {
        todo!("waiting for HunSpell conversion")
    }
    pub fn addUserDic(&mut self, _dic: &str) {
        todo!("waiting for HunSpell conversion")
    }
    pub fn setUserDic(&mut self, _dic: &str) {
        todo!("waiting for HunSpell conversion")
    }
    pub fn getMainHashMgr(&self) -> Option<HashMgr> {
        todo!("waiting for HunSpell conversion")
    }
}

// Java: public class SpellCheck implements ISpellCheck
pub struct SpellCheck {
    // Java field: String mPath
    pub(crate) mPath: Option<String>,
    // Java field: Map<Integer, HunSpell> mSpellMap
    pub(crate) mSpellMap: HashMap<i32, HunSpell>,
    // Java field: HashMgr mHashMgr
    pub(crate) mHashMgr: Option<HashMgr>,
}

// Java: public class SpellCheck implements ISpellCheck
#[allow(non_snake_case)]
impl SpellCheck {
    // Java: SpellCheck(String dicPath)
    pub fn new(dicPath: &str) -> Self {
      // Java: super();
      // Java: this.mPath = dicPath;
      Self {
          mPath: Some(dicPath.to_string()),
          mSpellMap: HashMap::new(),
          mHashMgr: None,
      }
    }

    // Java: SpellCheck()
    pub fn new_default() -> Self {
      // Java: super();
      Self {
          mPath: None,
          mSpellMap: HashMap::new(),
          mHashMgr: None,
      }
    }

    // Java: void setDictPath(String dicPath)
    pub fn setDictPath(&mut self, dicPath: &str) {
      // Java: this.mPath = dicPath;
      self.mPath = Some(dicPath.to_string());
    }

    // Java: IHunSpell getSpell(int lang)
    pub fn getSpell(&mut self, lang: i32) -> Option<&HunSpell> {
      // Java: String path = CSUtil.getLangPath(lang);
      let path = CSUtil::getLangPath(lang);
      // Java: String name = CSUtil.getLangName(lang);
      let name = CSUtil::getLangName(lang);
      // Java: if (path != null && name != null)
      if let (Some(path_str), Some(name_str)) = (path, name) {
         // Java: HunSpell hunSpell = (HunSpell)this.mSpellMap.get(lang);
         // Java: if (hunSpell != null)
         if self.mSpellMap.contains_key(&lang) {
            // Java: return hunSpell;
            return self.mSpellMap.get(&lang);
         }
         // Java: else
         {
            // Java: String pathName = this.mPath + path + name;
            let base_path = self.mPath.as_deref().unwrap_or("");
            let path_name = format!("{}{}{}", base_path, path_str, name_str);
            // Java: String aff = pathName + ".aff";
            let aff = format!("{}.aff", path_name);
            // Java: String dic = pathName + ".dic";
            let mut dic = format!("{}.dic", path_name);
            // Java: if ((new File(aff)).exists() && (new File(dic)).exists())
            if Path::new(&aff).exists() && Path::new(&dic).exists() {
               // Java: hunSpell = new HunSpell(aff, dic);
               let mut hun_spell = HunSpell::new(&aff, &dic);
               // Java: this.mSpellMap.put(lang, hunSpell);
               // Java: dic = pathName + "_ext" + ".dic";
               dic = format!("{}_ext.dic", path_name);
               // Java: if ((new File(dic)).exists())
               if Path::new(&dic).exists() {
                  // Java: hunSpell.addExtDic(dic);
                  hun_spell.addExtDic(&dic);
               }
               // Java: dic = pathName + "_user" + ".dic";
               dic = format!("{}_user.dic", path_name);
               // Java: if ((new File(dic)).exists())
               if Path::new(&dic).exists() {
                  // Java: hunSpell.addUserDic(dic);
                  hun_spell.addUserDic(&dic);
               }
               // Java: else
               else {
                  // Java: hunSpell.setUserDic(dic);
                  hun_spell.setUserDic(&dic);
               }
               // Java: this.mHashMgr = hunSpell.getMainHashMgr();
               self.mHashMgr = hun_spell.getMainHashMgr();
               self.mSpellMap.insert(lang, hun_spell);
               // Java: return hunSpell;
               return self.mSpellMap.get(&lang);
            }
            // Java: else
            {
               // Java: return null;
               return None;
            }
         }
      }
      // Java: else
      {
         // Java: return null;
         None
      }
    }

    // Java: HunSpell getSpellLoad(int lang)
    pub fn getSpellLoad(&self, lang: i32) -> Option<&HunSpell> {
      // Java: String path = CSUtil.getLangPath(lang);
      let path = CSUtil::getLangPath(lang);
      // Java: String name = CSUtil.getLangName(lang);
      let name = CSUtil::getLangName(lang);
      // Java: return path != null && name != null ? (HunSpell)this.mSpellMap.get(lang) : null;
      if path.is_some() && name.is_some() {
          self.mSpellMap.get(&lang)
      } else {
          None
      }
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
      // Java: this.mSpellMap.clear();
      self.mSpellMap.clear();
      // Java: this.mHashMgr = null;
      self.mHashMgr = None;
    }

}

impl ISpellCheck for SpellCheck {
    fn setDictPath(&mut self, var1: &str) {
        self.mPath = Some(var1.to_string());
    }
    fn getSpell(&mut self, _var1: i32) -> Option<&dyn IHunSpell> {
        None
    }
}
