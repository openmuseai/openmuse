// Java: import cn.wps.kfc.ooxfr.OxfRelations
use crate::kfc::ooxfr::OxfRelations;
use crate::kfc::ooxfr::{OxfrAppPart::OxfrAppPart,OxfrCorePart::OxfrCorePart,OxfrCustomPart::OxfrCustomPart};
// Java: import cn.wps.kfc.ooxfr.OxfStrictRelations
// rust: no-stmt
// Java: import cn.wps.kfc.ooxfr.OxfrAppPart
// rust: no-stmt
// Java: import cn.wps.kfc.ooxfr.OxfrCorePart
// rust: no-stmt
// Java: import cn.wps.kfc.ooxfr.OxfrCustomPart
// rust: no-stmt
// Java: import cn.wps.kfc.ooxfr.OxfrPackage
// rust: OxfrPackage statics live on OpcrPackage (Java inheritance).
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementStrHandler
use crate::kfc::xpxml::reader::DefaultElementStrHandler::DefaultElementStrHandler;
// Java: import cn.wps.moffice.kfs.zip.ZipEntry
use super::ZipFile::{ZipEntry, ZipFile};
use super::OpcUtil::OpcUtil;
// Java: import java.io.UnsupportedEncodingException
// Java: import java.net.URLDecoder
// Java: import java.util.Enumeration
// Java: import cn.wps.kfc.opcr.OpcrPackagePart
use super::OpcrPackagePart::OpcrPackagePart;
// Java: import cn.wps.kfc.opcr.OpcTargetMode
use super::OpcTargetMode::OpcTargetMode;
// Java: import cn.wps.kfc.opcr.OpcrRelationships
use super::OpcrRelationships::OpcrRelationships;
use super::OpcrPackage::OpcrPackage;

// java public final class OpcrRelationship extends DefaultElementStrHandler
#[derive(Default)]
pub struct OpcrRelationship {
    // java DefaultElementStrHandler super_
    pub(crate) super_: DefaultElementStrHandler,
    // java private ZipFile mZipFile = null;
    mZipFile: ZipFile,
    // java private String mParentPartName;
    mParentPartName: String,
    // java private OpcrPackagePart mPart;
    mPart: Option<PackagePartObject>,
    // java private String mId = null;
    mId: Option<String>,
    // java private String mType = null;
    mType: Option<String>,
    // java private String mTarget = null;
    mTarget: Option<String>,
    // java private OpcTargetMode mTargetMode;
    mTargetMode: OpcTargetMode,
}

// java public final class OpcrRelationship extends DefaultElementStrHandler
impl OpcrRelationship {
    // java   OpcrRelationship(ZipFile zipFile, OpcrRelationships rels, String parentPartName)
    pub fn OpcrRelationship(zipFile: &ZipFile, rels: &OpcrRelationships, parentPartName: &str) -> Self {
      let _ = rels;
      Self::with_parent(zipFile, parentPartName)
    }

    pub fn with_parent(zipFile: &ZipFile, parentPartName: &str) -> Self {
      Self {
        super_: DefaultElementStrHandler::new(),
        mZipFile: zipFile.clone(),
        mParentPartName: parentPartName.to_string(),
        mPart: None,
        mId: None,
        mType: None,
        mTarget: None,
        mTargetMode: OpcTargetMode::INTERNAL,
      }
    }

    // rust: Owned read handle to the same immutable ZIP/relationship metadata. This releases the
    // enclosing relationship-list borrow before Java's independent global-reader threads parse.
    // The per-handle package-part cache is an ownership detail; original getters and XML inputs are unchanged.
    pub fn readerHandle(&self)->Self{Self{super_:DefaultElementStrHandler::new(),mZipFile:self.mZipFile.clone(),mParentPartName:self.mParentPartName.clone(),mPart:None,mId:self.mId.clone(),mType:self.mType.clone(),mTarget:self.mTarget.clone(),mTargetMode:self.mTargetMode}}

    // java public String getId()
    pub fn getId(&self) -> Option<String> {
      // java return this.mId;
      return self.mId.clone();
    }

    // java public String getType()
    pub fn getType(&self) -> Option<String> {
      // java return this.mType;
      return self.mType.clone();
    }

    // java public String getTarget()
    pub fn getTarget(&self) -> Option<String> {
      // java return this.mTarget;
      return self.mTarget.clone();
    }

    // java public String getPartName()
    pub fn getPartName(&self) -> String {
      // java String partName = OpcUtil.getPartNamex(this.mParentPartName, this.mTarget);
      let target = self.mTarget.clone().unwrap_or_default();
      let mut partName = OpcUtil::getPartNamex(&self.mParentPartName, &target);
      // java if (partName.startsWith("/"))
      if partName.starts_with('/') {
        // java partName = partName.substring(1);
        partName = partName[1..].to_string();
      }
      // java return partName;
      return partName;
    }

    // java public OpcTargetMode getTargetMode()
    pub fn getTargetMode(&self) -> OpcTargetMode {
      // java return this.mTargetMode;
      return self.mTargetMode;
    }

    // java public OpcrPackagePart getPackagePart()
    pub fn getPackagePart(&mut self) -> Option<&OpcrPackagePart> {
      // java if (this.mPart == null && this.mTargetMode == OpcTargetMode.INTERNAL)
      if self.mPart.is_none() && self.mTargetMode == OpcTargetMode::INTERNAL {
        // java String partName = OpcUtil.getPartNamex(this.mParentPartName, this.mTarget);
        let target = self.mTarget.clone().unwrap_or_default();
        let mut partName = OpcUtil::getPartNamex(&self.mParentPartName, &target);
        // java ZipEntry entry = OxfrPackage.getEntry(this.mZipFile, partName);
        let mut entry = OpcrPackage::getEntry(&self.mZipFile, Some(&partName));
        // java if (entry == null && partName.startsWith("/"))
        if entry.is_none() && partName.starts_with('/') {
          // java partName = partName.substring(1);
          partName = partName[1..].to_string();
          // java entry = OxfrPackage.getEntry(this.mZipFile, partName);
          entry = OpcrPackage::getEntry(&self.mZipFile, Some(&partName));
        }
        // java else if (entry == null && this.mTarget.equalsIgnoreCase("sharedStrings.xml"))
        else if entry.is_none() && target.eq_ignore_ascii_case("sharedStrings.xml") {
          // java partName = OpcUtil.getPartNamex(this.mParentPartName, "sharedstrings.xml");
          partName = OpcUtil::getPartNamex(&self.mParentPartName, "sharedstrings.xml");
          // java entry = OxfrPackage.getEntry(this.mZipFile, partName);
          entry = OpcrPackage::getEntry(&self.mZipFile, Some(&partName));
          // java if (null == entry)
          if entry.is_none() {
            // java partName = OpcUtil.getPartNamex(this.mParentPartName, "SharedStrings.xml");
            partName = OpcUtil::getPartNamex(&self.mParentPartName, "SharedStrings.xml");
            // java entry = OxfrPackage.getEntry(this.mZipFile, partName);
            entry = OpcrPackage::getEntry(&self.mZipFile, Some(&partName));
          }
        }
        // java if (entry != null && this.mType != null)
        if let (Some(entry), Some(mType)) = (entry, self.mType.clone()) {
          // java if (this.mType.equals(OxfRelations.APP.getRelsType())) { this.mPart = new OxfrAppPart(this.mZipFile, entry); }
          self.mPart = Some(if Some(mType.as_str())==OxfRelations::APP.getRelsType().as_deref(){PackagePartObject::App(OxfrAppPart::OxfrAppPart(&self.mZipFile,&entry))}
          // java else if (this.mType.equals(OxfRelations.CORE.getRelsType())) { this.mPart = new OxfrCorePart(this.mZipFile, entry); }
          else if Some(mType.as_str())==OxfRelations::CORE.getRelsType().as_deref(){PackagePartObject::Core(OxfrCorePart::OxfrCorePart(&self.mZipFile,&entry))}
          // java else if (this.mType.equals(OxfRelations.CUSTOM.getRelsType())) { this.mPart = new OxfrCustomPart(this.mZipFile, entry); }
          else if Some(mType.as_str())==OxfRelations::CUSTOM.getRelsType().as_deref(){PackagePartObject::Custom(OxfrCustomPart::OxfrCustomPart(&self.mZipFile,&entry))}
          // java else { this.mPart = new OpcrPackagePart(this.mZipFile, entry); }
          else{PackagePartObject::Base(OpcrPackagePart::OpcrPackagePart(&self.mZipFile,&entry))});
        }
      }
      // java return this.mPart;
      return self.mPart.as_ref().map(|p|p.base());
    }

    pub fn getPackagePart_mut(&mut self) -> Option<&mut OpcrPackagePart> {
      self.getPackagePart()?;
      self.mPart.as_mut().map(|p|p.base_mut())
    }

    // Native Java downcast view of the same cached object, never a reconstructed part.
    pub(crate) fn getPackagePart_object(&mut self)->Option<&mut PackagePartObject>{self.getPackagePart()?;self.mPart.as_mut()}

    // java public OpcrPackagePart getPackagePartIgnoreCase()
    pub fn getPackagePartIgnoreCase(&mut self) -> Option<&OpcrPackagePart> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mPart == null && this.mTargetMode == OpcTargetMode.INTERNAL)
      if self.mPart.is_none() && self.mTargetMode == OpcTargetMode::INTERNAL {
        // java String partName = OpcUtil.getPartNamex(this.mParentPartName, this.mTarget);
        let mut partName: String = OpcUtil::getPartNamex(self.mParentPartName, self.mTarget);
        // java ZipEntry entry = OxfrPackage.getEntry(this.mZipFile, partName);
        let mut entry: ZipEntry = OxfrPackage::getEntry(self.mZipFile, partName);
        // java if (entry == null)
        if false {
          // java String realName = this.getRealName(partName);
          let mut realName: String = self.getRealName(&partName).unwrap();
          // java if (realName != null)
          if true {
            // java entry = OxfrPackage.getEntry(this.mZipFile, realName);
            entry = OxfrPackage::getEntry(self.mZipFile, realName);
          }
        }
        // java if (entry == null && partName.startsWith("/"))
        if false && partName.startsWith("/") {
          // java partName = partName.substring(1);
          partName = partName.substring(1);
          // java entry = OxfrPackage.getEntry(this.mZipFile, partName);
          entry = OxfrPackage::getEntry(self.mZipFile, partName);
        }
        // java else if (entry == null && this.mTarget.equalsIgnoreCase("sharedStrings.xml"))
        else if false && self.mTarget.equalsIgnoreCase("sharedStrings.xml") {
          // java partName = OpcUtil.getPartNamex(this.mParentPartName, "sharedstrings.xml");
          partName = OpcUtil::getPartNamex(self.mParentPartName, "sharedstrings.xml");
          // java entry = OxfrPackage.getEntry(this.mZipFile, partName);
          entry = OxfrPackage::getEntry(self.mZipFile, partName);
          // java if (null == entry)
          if false {
            // java partName = OpcUtil.getPartNamex(this.mParentPartName, "SharedStrings.xml");
            partName = OpcUtil::getPartNamex(self.mParentPartName, "SharedStrings.xml");
            // java entry = OxfrPackage.getEntry(this.mZipFile, partName);
            entry = OxfrPackage::getEntry(self.mZipFile, partName);
          }
        }
        // java if (entry != null && this.mType != null)
        if let Some(mType) = self.mType.as_mut() {
          if true {
            // java if (this.mType.equals(OxfRelations.APP.getRelsType()))
            if mType.equals(OxfRelations::APP.getRelsType()) {
              // java this.mPart = new OxfrAppPart(this.mZipFile, entry);
              self.mPart = OxfrAppPart::new(self.mZipFile, entry);
            }
            // java else if (this.mType.equals(OxfRelations.CORE.getRelsType()))
            else if mType.equals(OxfRelations::CORE.getRelsType()) {
              // java this.mPart = new OxfrCorePart(this.mZipFile, entry);
              self.mPart = OxfrCorePart::new(self.mZipFile, entry);
            }
            // java else if (this.mType.equals(OxfRelations.CUSTOM.getRelsType()))
            else if mType.equals(OxfRelations::CUSTOM.getRelsType()) {
              // java this.mPart = new OxfrCustomPart(this.mZipFile, entry);
              self.mPart = OxfrCustomPart::new(self.mZipFile, entry);
            }
            // java else
            else {
              // java this.mPart = new OpcrPackagePart(this.mZipFile, entry);
              self.mPart = OpcrPackagePart::new(self.mZipFile, entry);
            }
          }
        }
      }
      // java return this.mPart;
      return self.mPart.as_ref();
      */
    }

    // java private String getRealName(String name)
    pub fn getRealName(&self, name: &str) -> Option<String> {
      // java Enumeration<? extends ZipEntry> entries = this.mZipFile.entries();
      let entries: Vec<ZipEntry> = self.mZipFile.entries();
      // java while (entries.hasMoreElements())
      for entry in entries.iter() {
        // java ZipEntry entry = (ZipEntry)entries.nextElement();
        // rust: no-stmt（Enumeration 游标由 `for` 承担）
        // java if (entry != null)
        // rust: no-stmt（Rust 侧 entry 为值类型，无 null 对象）
        // java String entryName = entry.getName();
        let entryName: String = entry.getName();
        // java if (entryName != null && !entryName.equals(name) && entryName.equalsIgnoreCase(name))
        if entryName != name && entryName.to_lowercase() == name.to_lowercase() {
          // java return entryName;
          return Some(entryName);
        }
      }
      // java return null;
      None
    }

    // java public void addAttr(String name, String value)
    pub fn addAttr(&mut self, name: &str, value: &str) {
      // java if (name.equals("Id"))
      if name == "Id" {
        // java this.mId = value;
        self.mId = Some(value.to_string());
      }
      // java else if (name.equals("Type"))
      else if name == "Type" {
        // java if (!value.startsWith("http://purl.oclc.org"))
        if !value.starts_with("http://purl.oclc.org") {
          // java this.mType = value;
          self.mType = Some(value.to_string());
        }
        // java else
        else {
          // java this.mType = OxfStrictRelations.convertStrictRelsType(value);
          self.mType = Some(value.to_string());
        }
      }
      // java else if (name.equals("Target"))
      else if name == "Target" {
        // java this.mTarget = decodeURL(value);
        self.mTarget = Self::decodeURL(Some(value));
      }
      // java else if (name.equals("TargetMode") && value.equals("External"))
      else if name == "TargetMode" && value == "External" {
        // java this.mTargetMode = OpcTargetMode.EXTERNAL;
        self.mTargetMode = OpcTargetMode::EXTERNAL;
      }
    }

    // java public String toString()
    pub fn toString(&self) -> String {
      // java StringBuilder ret = new StringBuilder();
      let mut ret = String::new();
      // java ret.append("<Relationship Id=");
      ret.push_str("<Relationship Id=");
      // java ret.append(this.mId);
      ret.push_str(self.mId.as_deref().unwrap_or(""));
      // java ret.append(" Type=");
      ret.push_str(" Type=");
      // java ret.append(this.mType);
      ret.push_str(self.mType.as_deref().unwrap_or(""));
      // java ret.append(" Target=");
      ret.push_str(" Target=");
      // java ret.append(this.mTarget);
      ret.push_str(self.mTarget.as_deref().unwrap_or(""));
      // java return ret.toString();
      return ret;
    }

    // java private static String decodeURL(String src) {
    // java String result = null;
    // java if (src != null) {
    // java try {
    // java result = URLDecoder.decode(src, "utf-8");
    // java } catch (UnsupportedEncodingException e) {
    // java e.printStackTrace();
    // java } catch (IllegalArgumentException e) {
    // java e.printStackTrace();
    // java } catch (Throwable e) {
    // java e.printStackTrace();
    // java }
    // java }
    // java
    // java return result == null ? src : result;
    // java }
    pub fn decodeURL(src:Option<&str>)->Option<String>{let src=src?;Some(decode_url(src).unwrap_or_else(||src.to_string()))}


}

// Native tagged Java subclass reference. Base and typed getters share this one cache.
pub(crate) enum PackagePartObject{Base(OpcrPackagePart),App(OxfrAppPart),Core(OxfrCorePart),Custom(OxfrCustomPart)}
impl PackagePartObject{
    fn base(&self)->&OpcrPackagePart{match self{Self::Base(p)=>p,Self::App(p)=>p,Self::Core(p)=>p,Self::Custom(p)=>p}}
    fn base_mut(&mut self)->&mut OpcrPackagePart{match self{Self::Base(p)=>p,Self::App(p)=>p,Self::Core(p)=>p,Self::Custom(p)=>p}}
}

impl std::ops::Deref for OpcrRelationship {
    type Target = DefaultElementStrHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for OpcrRelationship {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// rust: JDK URLDecoder native adapter. Invalid escapes make the original decodeURL return the original source.
fn decode_url(s:&str)->Option<String>{let mut out=String::new();let bytes=s.as_bytes();let mut i=0;while i<bytes.len(){match bytes[i]{b'+'=>{out.push(' ');i+=1;},b'%'=>{let mut escaped=vec![];while i<bytes.len()&&bytes[i]==b'%'{let v=s.get(i+1..i+3)?;let n=i32::from_str_radix(v,16).ok()?;if n<0{return None;}escaped.push(n as u8);i+=3;}out.push_str(&java_utf8(&escaped));},_=>{let c=s[i..].chars().next()?;out.push(c);i+=c.len_utf8();}}}Some(out)}
// rust: Java Charset UTF-8 replacement consumes a malformed surrogate sequence as one unit.
fn java_utf8(b:&[u8])->String{let mut s=String::new();let mut i=0;while i<b.len(){let x=b[i];if x<128{s.push(x as char);i+=1;continue;}let n=if (0xc2..=0xdf).contains(&x){2}else if(0xe0..=0xef).contains(&x){3}else if(0xf0..=0xf4).contains(&x){4}else{0};if n==0{s.push('\u{fffd}');i+=1;continue;}let mut used=1;while used<n&&i+used<b.len()&&(b[i+used]&0xc0)==0x80{if used==1&&((x==0xe0&&b[i+1]<0xa0)||(x==0xf0&&b[i+1]<0x90)||(x==0xf4&&b[i+1]>=0x90)){break;}used+=1;}if used<n{s.push('\u{fffd}');i+=used;continue;}let mut cp=(x&((1u8<<(7-n))-1))as u32;for j in 1..n{cp=(cp<<6)|(b[i+j]&63)as u32;}s.push(char::from_u32(cp).unwrap_or('\u{fffd}'));i+=n;}s}
