// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.io.util.URI
// Java: import cn.wps.io.util.WpsUriUtils
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidFormatException
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidOperationException
// Java: import java.net.URISyntaxException

use std::sync::OnceLock;
use crate::io::util::URI::URI;
use crate::io::opc::PackagePartName::PackagePartName;

// Java field: String TAG
// Java field: URI packageRootUri

// Java: static final PackagePartName PACKAGE_RELATIONSHIPS_ROOT_PART_NAME（静态块 `:401` 初始化）
/* Java: static 块里的构造：
   PackagePartName tmpPACKAGE_RELATIONSHIPS_ROOT_PART_NAME = null;
   tmpPACKAGE_RELATIONSHIPS_ROOT_PART_NAME = createPartName(PACKAGE_RELATIONSHIPS_ROOT_URI);
   PACKAGE_RELATIONSHIPS_ROOT_PART_NAME = tmpPACKAGE_RELATIONSHIPS_ROOT_PART_NAME; */
pub fn package_relationships_root_part_name() -> &'static PackagePartName {
    static V: OnceLock<PackagePartName> = OnceLock::new();
    V.get_or_init(|| match PackagingURIHelper::createPartName_uri(package_relationships_root_uri()) {
        Ok(v) => v,
        // Java：静态块 `catch (InvalidFormatException)` 后常量留 null ⇒ 后续使用 NPE；Rust 用 panic 表达同一「不可能」
        Err(e) => panic!("{}", e),
    })
}

// Java field: String RELATIONSHIP_PART_EXTENSION_NAME
pub const RELATIONSHIP_PART_EXTENSION_NAME: &str = ".rels";
// Java field: String RELATIONSHIP_PART_SEGMENT_NAME
pub const RELATIONSHIP_PART_SEGMENT_NAME: &str = "_rels";
// Java field: String PACKAGE_PROPERTIES_SEGMENT_NAME
pub const PACKAGE_PROPERTIES_SEGMENT_NAME: &str = "docProps";
// Java field: String PACKAGE_CORE_PROPERTIES_NAME
pub const PACKAGE_CORE_PROPERTIES_NAME: &str = "core.xml";
// Java field: String PACKAGE_EXTENDED_PROPERTIES_NAME
pub const PACKAGE_EXTENDED_PROPERTIES_NAME: &str = "app.xml";
// Java field: String CUSTOM_EXTENDED_PROPERTIES_NAME
pub const CUSTOM_EXTENDED_PROPERTIES_NAME: &str = "custom.xml";
// Java field: char FORWARD_SLASH_CHAR
pub const FORWARD_SLASH_CHAR: char = '/';
// Java field: String FORWARD_SLASH_STRING
pub const FORWARD_SLASH_STRING: &str = "/";

// Java field: URI PACKAGE_RELATIONSHIPS_ROOT_URI
// Java field: PackagePartName PACKAGE_RELATIONSHIPS_ROOT_PART_NAME
// Java field: URI CORE_PROPERTIES_URI
// Java field: URI EXTENDED_PROPERTIES_URI
// Java field: URI CUSTOM_PROPERTIES_URI
// Java field: PackagePartName CORE_PROPERTIES_PART_NAME
// Java field: PackagePartName EXTENDED_PROPERTIES_PART_NAME
// Java field: PackagePartName CUSTOM_PROPERTIES_PART_NAME
// Java field: URI PACKAGE_ROOT_URI
// Java field: PackagePartName PACKAGE_ROOT_PART_NAME

// Java: （同包可见）static final URI PACKAGE_ROOT_URI ⇒ pub(crate)：`PackagePartName`（同包）构造器要读它
pub(crate) fn package_root_uri() -> &'static URI {
    static V: OnceLock<URI> = OnceLock::new();
    V.get_or_init(|| URI::new("/").unwrap())
}

// Java: static final PackagePartName PACKAGE_ROOT_PART_NAME（`:30` 声明 / `:402` 初始化）
/* Java: static 块里的构造（全仓唯一 `checkConformance = false` 的构造点）：
   PackagePartName tmpPACKAGE_ROOT_PART_NAME = null;
   tmpPACKAGE_ROOT_PART_NAME = new PackagePartName(PACKAGE_ROOT_URI, false);
   PACKAGE_ROOT_PART_NAME = tmpPACKAGE_ROOT_PART_NAME; */
pub fn package_root_part_name() -> &'static PackagePartName {
    static V: OnceLock<PackagePartName> = OnceLock::new();
    V.get_or_init(|| {
        PackagePartName::new(package_root_uri(), false).expect("PACKAGE_ROOT_PART_NAME 恒可构造")
    })
}

fn package_relationships_root_uri() -> &'static URI {
    static V: OnceLock<URI> = OnceLock::new();
    V.get_or_init(|| {
        let path = format!("/{}/{}", RELATIONSHIP_PART_SEGMENT_NAME, RELATIONSHIP_PART_EXTENSION_NAME);
        URI::new(&path).unwrap()
    })
}

fn core_properties_uri() -> &'static URI {
    static V: OnceLock<URI> = OnceLock::new();
    V.get_or_init(|| {
        let path = format!("/{}/{}", PACKAGE_PROPERTIES_SEGMENT_NAME, PACKAGE_CORE_PROPERTIES_NAME);
        URI::new(&path).unwrap()
    })
}

fn extended_properties_uri() -> &'static URI {
    static V: OnceLock<URI> = OnceLock::new();
    V.get_or_init(|| {
        let path = format!("/{}/{}", PACKAGE_PROPERTIES_SEGMENT_NAME, PACKAGE_EXTENDED_PROPERTIES_NAME);
        URI::new(&path).unwrap()
    })
}

fn custom_properties_uri() -> &'static URI {
    static V: OnceLock<URI> = OnceLock::new();
    V.get_or_init(|| {
        let path = format!("/{}/{}", PACKAGE_PROPERTIES_SEGMENT_NAME, CUSTOM_EXTENDED_PROPERTIES_NAME);
        URI::new(&path).unwrap()
    })
}

// Java: public class PackagingURIHelper
pub struct PackagingURIHelper {
}

// Java: public class PackagingURIHelper
impl PackagingURIHelper {
    // Java: PackagingURIHelper()
    pub fn new() -> Self {
      // Java: super();
      PackagingURIHelper {}
    }

    // Rust-native: Java 构造器 `public PackagingURIHelper()` 的**同名入口**（仓内 ctor 约定：
    // 与 Java 同名；`new()` 与它同一实现，供既有调用方使用）。
    pub fn PackagingURIHelper() -> Self {
      // Java: super();
      PackagingURIHelper {}
    }

    // Java: static URI getPackageRootUri()
    pub fn getPackageRootUri() -> URI {
      // Java: return packageRootUri;
      package_root_uri().clone()
    }

    // Java: static boolean isRelationshipPartURI(URI partUri)
    pub fn isRelationshipPartURI(part_uri: &URI) -> bool {
      // Java: if (partUri == null)
      // (Rust: reference always valid)
      {
         // Java: throw new IllegalArgumentException("partUri");
      }
      // Java: else
      {
         // Java: return partUri.getPath().matches(".*" + RELATIONSHIP_PART_SEGMENT_NAME + ".*" + RELATIONSHIP_PART_EXTENSION_NAME + "$");
         // NOTE(Java 正则保真)：模式串由常量拼成，`RELATIONSHIP_PART_EXTENSION_NAME` = ".rels" 里的 `.`
         // **没有被转义** ⇒ 在 Java 正则里它是**通配符**。因此真实条件不是「以 ".rels" 结尾」，而是：
         //   ① 末 5 个字符 = 任意字符 + "rels"（即必须以 "rels" 结尾，且其前还有 1 个字符）
         //   ② `_rels` 必须整体出现在末尾这 5 个字符**之前**（正则按顺序消费 ⇒ ∃p: p + 5 <= len - 5，
         //      故 `_rels` 落在 head = chars[..len-5] 内）
         // ⇒ `/_rels/arels`（无点）与 `/_rels/rels` 在 Java 侧都返回 true；旧的 `ends_with(".rels")`
         // 实现比 Java **更严**，会漏判（见 test 25 的分歧探针）。
         let path = part_uri.getPath().unwrap_or_default();
         let chars: Vec<char> = path.chars().collect();
         // Java: （`String.matches` 的 `.` 默认不匹配行终止符；URI 路径不含行终止符 ⇒ 与 `chars()` 等价）
         if chars.len() < 5 || chars[chars.len() - 4..] != ['r', 'e', 'l', 's']
         {
            // Java: （不匹配 ⇒ false）
            false
         }
         else
         {
            // Java: .* + RELATIONSHIP_PART_SEGMENT_NAME（中间 `.*` 可为空）
            let head: String = chars[..chars.len() - 5].iter().collect();
            head.contains(RELATIONSHIP_PART_SEGMENT_NAME)
         }
      }
    }

    // Java: static String getFilename(URI uri)
    pub fn getFilename(uri: &URI) -> String {
      // Java: if (uri != null)
      {
         // Java: String path = uri.getPath();
         let path = uri.getPath().unwrap_or_default();
         // Java: int len = path.length();
         let len = path.len();
         // Java: int num2 = len;
         let mut num2 = len as i32;

         // Java: while(true)
         loop
         {
            // Java: --num2;
            num2 -= 1;
            // Java: if (num2 < 0)
            if num2 < 0
            {
               // Java: break;
               break;
            }

            // Java: if (path.charAt(num2) == FORWARD_SLASH_CHAR)
            if path.as_bytes()[num2 as usize] == b'/'
            {
               // Java: return path.substring(num2 + 1, len);
               return path[num2 as usize + 1..len].to_string();
            }
         }
      }

      // Java: return "";
      String::new()
    }

    // Java: static String getFilenameWithoutExtension(URI uri)
    pub fn getFilenameWithoutExtension(uri: &URI) -> String {
      // Java: String filename = getFilename(uri);
      let filename = Self::getFilename(uri);
      // Java: int dotIndex = filename.lastIndexOf(".");
      let dot_index = filename.rfind('.');
      // Java: return dotIndex == -1 ? filename : filename.substring(0, dotIndex);
      match dot_index {
          None => filename,
          Some(idx) => filename[..idx].to_string(),
      }
    }

    // Java: static URI getPath(URI uri)
    pub fn getPath(uri: &URI) -> Option<URI> {
      // Java: if (uri != null)
      {
         // Java: String path = uri.getPath();
         let path = uri.getPath().unwrap_or_default();
         // Java: int num2 = path.length();
         let mut num2 = path.len() as i32;

         // Java: while(true)
         loop
         {
            // Java: --num2;
            num2 -= 1;
            // Java: if (num2 < 0)
            if num2 < 0
            {
               // Java: break;
               break;
            }

            // Java: if (path.charAt(num2) == FORWARD_SLASH_CHAR)
            if path.as_bytes()[num2 as usize] == b'/'
            {
               // Java: try
               {
                  // Java: return new URI(path.substring(0, num2));
                  return URI::new(&path[..num2 as usize]).ok();
               }
               // Java: catch (URISyntaxException var4)
               {
                  // Java: return null;
                  #[allow(unreachable_code)]
                  return None;
               }
            }
         }
      }

      // Java: return null;
      None
    }

    // Java: static URI combine(URI prefix, URI suffix)
    pub fn combine_uri(prefix: &URI, suffix: &URI) -> URI {
      // Java: URI retUri = null;
      let retUri: URI;
      // Java: try
      {
         // Java: retUri = new URI(combine(prefix.getPath(), suffix.getPath()));
         retUri = URI::new(&Self::combine_str(
             &prefix.getPath().unwrap_or_default(),
             &suffix.getPath().unwrap_or_default(),
         ))
         .unwrap();
         // Java: return retUri;
         return retUri;
      }
      // Java: catch (URISyntaxException var4)
      {
         // Java: throw new IllegalArgumentException("Prefix and suffix can't be combine !");
         #[allow(unreachable_code)]
         panic!("Prefix and suffix can't be combine !");
      }
    }

    // Java: static String combine(String prefix, String suffix)
    pub fn combine_str(prefix: &str, suffix: &str) -> String {
      // Java: int pLen = prefix.length();
      let p_len = prefix.len();
      // Java: int n = (pLen > 0 && prefix.charAt(pLen - 1) == FORWARD_SLASH_CHAR ? 1 : 0) + (!suffix.isEmpty() && suffix.charAt(0) == FORWARD_SLASH_CHAR ? 1 : 0);
      let n = (if p_len > 0 && prefix.ends_with('/') { 1 } else { 0 })
            + (if !suffix.is_empty() && suffix.starts_with('/') { 1 } else { 0 });
      // Java: if (n == 0)
      if n == 0
      {
         // Java: return prefix + FORWARD_SLASH_CHAR + suffix;
         return format!("{}/{}", prefix, suffix);
      }
      // Java: else
      {
         // Java: return n == 1 ? prefix + suffix : "";
         if n == 1 { format!("{}{}", prefix, suffix) } else { String::new() }
      }
    }

    // Java: static URI relativizeURI(URI sourceURI, URI targetURI, boolean msCompatible)
    pub fn relativizeURI_compat(source_uri: &URI, target_uri: &URI, ms_compatible: bool) -> Option<URI> {
      // Java: StringBuilder retVal = new StringBuilder();
      let mut retVal = String::new();
      // Java: String[] segmentsSource = sourceURI.getPath().split("/", -1);
      let src_path = source_uri.getPath().unwrap_or_default();
      let segments_source: Vec<&str> = src_path.split('/').collect();
      // Java: String[] segmentsTarget = targetURI.getPath().split("/", -1);
      let tgt_path = target_uri.getPath().unwrap_or_default();
      let segments_target: Vec<&str> = tgt_path.split('/').collect();
      // Java: if (segmentsSource.length == 0)
      if segments_source.is_empty()
      {
         // Java: throw new IllegalArgumentException("Can't relativize an empty source URI !");
         panic!("Can't relativize an empty source URI !");
      }
      // Java: else if (segmentsTarget.length == 0)
      else if segments_target.is_empty()
      {
         // Java: throw new IllegalArgumentException("Can't relativize an empty target URI !");
         panic!("Can't relativize an empty target URI !");
      }
      // Java: else if (sourceURI.toString().equals("/"))
      else if source_uri.toString() == "/"
      {
         // Java: String path = targetURI.getPath();
         let path = target_uri.getPath().unwrap_or_default();
         // Java: if (msCompatible && path.charAt(0) == '/')
         if ms_compatible && path.starts_with('/')
         {
            // Java: try
            {
               // Java: targetURI = new URI(path.substring(1));
               return URI::new(&path[1..]).ok();
            }
            // Java: catch (Exception e)
            {
               // Java: System.err.println(e);
               // Java: return null;
               #[allow(unreachable_code)]
               return None;
            }
         }

         // Java: return targetURI;
         return Some(target_uri.clone());
      }
      // Java: else
      {
         // Java: int segmentsTheSame = 0;
         let mut segmentsTheSame: usize = 0;

         // Java: for(int i = 0; i < segmentsSource.length && i < segmentsTarget.length && segmentsSource[i].equals(segmentsTarget[i]); ++i)
         for i in 0..segments_source.len().min(segments_target.len())
         {
            if segments_source[i] != segments_target[i] { break; }
            // Java: ++segmentsTheSame;
            segmentsTheSame += 1;
         }

         // Java: if ((segmentsTheSame == 0 || segmentsTheSame == 1) && segmentsSource[0].equals("") && segmentsTarget[0].equals(""))
         if (segmentsTheSame == 0 || segmentsTheSame == 1) && segments_source[0].is_empty() && segments_target[0].is_empty()
         {
            // Java: for(int i = 0; i < segmentsSource.length - 2; ++i)
            for _i in 0..segments_source.len().saturating_sub(2)
            {
               // Java: retVal.append("../");
               retVal.push_str("../");
            }

            // Java: for(int i = 0; i < segmentsTarget.length; ++i)
            for i in 0..segments_target.len()
            {
               // Java: if (!segmentsTarget[i].equals(""))
               if !segments_target[i].is_empty()
               {
                  // Java: retVal.append(segmentsTarget[i]);
                  retVal.push_str(segments_target[i]);
                  // Java: if (i != segmentsTarget.length - 1)
                  if i != segments_target.len() - 1
                  {
                     // Java: retVal.append("/");
                     retVal.push('/');
                  }
               }
            }

            // Java: try
            {
               // Java: return new URI(retVal.toString());
               return URI::new(&retVal).ok();
            }
            // Java: catch (Exception e)
            {
               // Java: System.err.println(e);
               // Java: return null;
               #[allow(unreachable_code)]
               return None;
            }
         }
         // Java: else
         {
            // Java: if (segmentsTheSame == segmentsSource.length && segmentsTheSame == segmentsTarget.length)
            if segmentsTheSame == segments_source.len() && segmentsTheSame == segments_target.len()
            {
               // Java: retVal.append("");
            }
            // Java: else
            else
            {
               // Java: if (segmentsTheSame == 1)
               if segmentsTheSame == 1
               {
                  // Java: retVal.append("/");
                  retVal.push('/');
               }
               // Java: else
               else
               {
                  // Java: for(int j = segmentsTheSame; j < segmentsSource.length - 1; ++j)
                  for _j in segmentsTheSame..segments_source.len().saturating_sub(1)
                  {
                     // Java: retVal.append("../");
                     retVal.push_str("../");
                  }
               }

               // Java: for(int j = segmentsTheSame; j < segmentsTarget.length; ++j)
               for j in segmentsTheSame..segments_target.len()
               {
                  // Java: if (retVal.length() > 0 && retVal.charAt(retVal.length() - 1) != '/')
                  if !retVal.is_empty() && !retVal.ends_with('/')
                  {
                     // Java: retVal.append("/");
                     retVal.push('/');
                  }

                  // Java: retVal.append(segmentsTarget[j]);
                  retVal.push_str(segments_target[j]);
               }
            }

            // Java: try
            {
               // Java: return new URI(retVal.toString());
               return URI::new(&retVal).ok();
            }
            // Java: catch (Exception e)
            {
               // Java: IOLog.e(TAG, "Exception:", e);
               // Java: return null;
               #[allow(unreachable_code)]
               return None;
            }
         }
      }
    }

    // Java: static URI relativizeURI(URI sourceURI, URI targetURI)
    pub fn relativizeURI(source_uri: &URI, target_uri: &URI) -> Option<URI> {
      // Java: return relativizeURI(sourceURI, targetURI, false);
      Self::relativizeURI_compat(source_uri, target_uri, false)
    }

    // Java: static URI resolvePartUri(URI sourcePartUri, URI targetUri)
    pub fn resolvePartUri(source_part_uri: &URI, target_uri: &URI) -> URI {
      // Java: if (sourcePartUri != null && !sourcePartUri.isAbsolute())
      if !source_part_uri.isAbsolute()
      {
         // Java: if (targetUri != null && !targetUri.isAbsolute())
         if !target_uri.isAbsolute()
         {
            // Java: return sourcePartUri.resolve(targetUri);
            return source_part_uri.resolve(target_uri);
         }
         // Java: else
         {
            // Java: throw new IllegalArgumentException("targetUri invalid - " + targetUri);
            // Java 文案是字符串拼接（`+ targetUri`）⇒ 用 `toString()`，不是 Debug 形态
            panic!("targetUri invalid - {}", target_uri.toString());
         }
      }
      // Java: else
      {
         // Java: throw new IllegalArgumentException("sourcePartUri invalid - " + sourcePartUri);
         // Java 文案是字符串拼接（`+ sourcePartUri`）⇒ 用 `toString()`，不是 Debug 形态
         panic!("sourcePartUri invalid - {}", source_part_uri.toString());
      }
    }

    // Java: static URI getURIFromPath(String path)
    pub fn getURIFromPath(path: &str) -> URI {
      // Java: URI retUri = null;
      let retUri: URI;
      // Java: try
      {
         // Java: retUri = new URI(path);
         retUri = URI::new(path).unwrap();
         // Java: return retUri;
         return retUri;
      }
      // Java: catch (URISyntaxException var3)
      {
         // Java: throw new IllegalArgumentException("path");
         panic!("path");
      }
    }

    // Java: static URI getSourcePartUriFromRelationshipPartUri(URI relationshipPartUri)
    pub fn getSourcePartUriFromRelationshipPartUri(relationship_part_uri: &URI) -> URI {
      // Java: if (relationshipPartUri == null)
      // (Rust: reference always valid)
      {
         // Java: throw new IllegalArgumentException("Must not be null");
      }
      // Java: else if (!isRelationshipPartURI(relationshipPartUri))
      if !Self::isRelationshipPartURI(relationship_part_uri)
      {
         // Java: throw new IllegalArgumentException("Must be a relationship part");
         panic!("Must be a relationship part");
      }
      // Java: else if (relationshipPartUri.compareTo(PACKAGE_RELATIONSHIPS_ROOT_URI) == 0)
      else if relationship_part_uri.compareTo(package_relationships_root_uri()) == 0
      {
         // Java: return PACKAGE_ROOT_URI;
         return package_root_uri().clone();
      }
      // Java: else
      {
         // Java: String filename = relationshipPartUri.getPath();
         let mut filename = relationship_part_uri.getPath().unwrap_or_default();
         // Java: String filenameWithoutExtension = getFilenameWithoutExtension(relationshipPartUri);
         let filenameWithoutExtension = Self::getFilenameWithoutExtension(relationship_part_uri);
         // Java: filename = filename.substring(0, filename.length() - filenameWithoutExtension.length() - RELATIONSHIP_PART_EXTENSION_NAME.length());
         let cut1 = filename.len() - filenameWithoutExtension.len() - RELATIONSHIP_PART_EXTENSION_NAME.len();
         filename = filename[..cut1].to_string();
         // Java: filename = filename.substring(0, filename.length() - RELATIONSHIP_PART_SEGMENT_NAME.length() - 1);
         let cut2 = filename.len() - RELATIONSHIP_PART_SEGMENT_NAME.len() - 1;
         filename = filename[..cut2].to_string();
         // Java: filename = combine(filename, filenameWithoutExtension);
         filename = Self::combine_str(&filename, &filenameWithoutExtension);
         // Java: return getURIFromPath(filename);
         Self::getURIFromPath(&filename)
      }
    }

    // Java: static PackagePartName createPartName(URI partUri)
    pub fn createPartName_uri(part_uri: &URI) -> Result<PackagePartName, String> {
      // Java: if (partUri == null)
      // (Rust: reference always valid)
      {
         // Java: throw new IllegalArgumentException("partName");
      }
      // Java: else
      {
         // Java: return new PackagePartName(partUri, true);
         // ⇒ 4 个重载的唯一汇聚点：一律走 Java 构造器（内含 throwExceptionIfInvalidPartUri 校验链）
         PackagePartName::new(part_uri, true)
      }
    }

    // Java: static PackagePartName createPartName(String partName)
    pub fn createPartName_str(part_name: &str) -> Result<PackagePartName, String> {
      // Java: URI partNameURI;
      let resolved = Self::resolvePartName(part_name);
      let partNameURI: URI;
      // Java: try
      {
         // Java: partNameURI = new URI(resolvePartName(partName));
         partNameURI = URI::new(&resolved).map_err(|e| format!("{:?}", e))?;
      }
      // Java: catch (URISyntaxException e)
      {
         // Java: throw new InvalidFormatException(e.getMessage());
      }

      // Java: return createPartName(partNameURI);
      Self::createPartName_uri(&partNameURI)
    }

    // Java: static PackagePartName createPartName(String partName, PackagePart relativePart)
    pub fn createPartName_str_relative(part_name: &str, relative_part_name_uri: &URI) -> Result<PackagePartName, String> {
      // Java: URI newPartNameURI;
      let new_uri: URI;
      // Java: try
      {
         // Java: newPartNameURI = resolvePartUri(relativePart.getPartName().getURI(), new URI(partName));
         let part_uri = URI::new(part_name).map_err(|e| format!("{:?}", e))?;
         new_uri = Self::resolvePartUri(relative_part_name_uri, &part_uri);
      }
      // Java: catch (URISyntaxException e)
      {
         // Java: throw new InvalidFormatException(e.getMessage());
      }

      // Java: return createPartName(newPartNameURI);
      Self::createPartName_uri(&new_uri)
    }

    // Java: static PackagePartName createPartName(URI partName, PackagePart relativePart)
    pub fn createPartName_uri_relative(part_name: &URI, relative_part_name_uri: &URI) -> Result<PackagePartName, String> {
      // Java: URI newPartNameURI = resolvePartUri(relativePart.getPartName().getURI(), partName);
      let new_uri = Self::resolvePartUri(relative_part_name_uri, part_name);
      // Java: return createPartName(newPartNameURI);
      Self::createPartName_uri(&new_uri)
    }

    // Java: static boolean isValidPartName(URI partUri)
    pub fn isValidPartName(part_uri: &URI) -> bool {
      // Java: if (partUri == null)
      // (Rust: reference always valid)
      {
         // Java: throw new IllegalArgumentException("partUri");
      }
      // Java: else
      {
         // Java: try
         {
            // Java: createPartName(partUri);
            // Java: return true;
            return Self::createPartName_uri(part_uri).is_ok();
         }
         // Java: catch (Exception var2)
         {
            // Java: return false;
            #[allow(unreachable_code)]
            false
         }
      }
    }

    // Java: static String decodeURI(URI uri)
    pub fn decodeURI(uri: &URI) -> String {
      // Java: StringBuffer retVal = new StringBuffer();
      let mut retVal = String::new();
      // Java: String uriStr = WpsUriUtils.toASCIIString2(uri.toString());
      let uri_str = uri.toString();

      // Java: for(int i = 0; i < uriStr.length(); ++i)
      let chars: Vec<char> = uri_str.chars().collect();
      let mut i = 0;
      while i < chars.len()
      {
         // Java: char c = uriStr.charAt(i);
         let c = chars[i];
         // Java: if (c == '%')
         if c == '%'
         {
            // Java: if (uriStr.length() - i < 2)
            if chars.len() - i < 3
            {
               // Java: throw new IllegalArgumentException("The uri " + uriStr + " contain invalid encoded character !");
               panic!("The uri {} contain invalid encoded character !", uri_str);
            }

            // Java: char decodedChar = (char)Integer.parseInt(uriStr.substring(i + 1, i + 3), 16);
            let hex_str: String = chars[i + 1..i + 3].iter().collect();
            let decoded = u8::from_str_radix(&hex_str, 16).unwrap_or(b'?') as char;
            // Java: retVal.append(decodedChar);
            retVal.push(decoded);
            // Java: i += 2;
            i += 2;
         }
         // Java: else
         else
         {
            // Java: retVal.append(c);
            retVal.push(c);
         }
         i += 1;
      }

      // Java: return retVal.toString();
      retVal
    }

    // Java: static PackagePartName getRelationshipPartName(PackagePartName partName)
    pub fn getRelationshipPartName(part_name: &PackagePartName) -> Option<PackagePartName> {
      // Java: if (partName == null)
      // (Rust: reference always valid)
      {
         // Java: throw new IllegalArgumentException("partName");
      }
      // Java: else
      {
         // Java: URI partNameUri = partName.getURI();
         let part_name_uri = part_name.getURI();
         // Java: String fullPath = partNameUri.getPath();
         let full_path = part_name_uri.getPath().unwrap_or_default();
         // Java: int lenFullPath = fullPath.length();
         // NOTE(Java 保真)：`String.length()` 数的是 char（UTF-16 码元），Rust `String::len()` 是**字节数**
         // ⇒ 非 ASCII 路径（如 `/word/文档.xml`）下按字节算会让下面的下标整体错位，必须按 chars 计数。
         let len_full_path = full_path.chars().count();
         // Java: if (lenFullPath == 1 && fullPath.charAt(0) == '/')
         if len_full_path == 1 && full_path.starts_with('/')
         {
            // Java: return PACKAGE_RELATIONSHIPS_ROOT_PART_NAME;
            // （Rust 的静态量是**模块级**函数，与 `package_root_uri()` 同一约定）
            return Some(package_relationships_root_part_name().clone());
         }
         // Java: else if (partName.isRelationshipPartURI())
         else if part_name.isRelationshipPartURI()
         {
            // Java: throw new InvalidOperationException("Can't be a relationship part");
            panic!("Can't be a relationship part");
         }
         // Java: else
         {
            // Java: int lenPartName = lenFullPath + 12;
            let lenPartName = len_full_path + 12;
            // Java: char[] partname = new char[lenPartName];
            let mut partname: Vec<char> = vec!['\0'; lenPartName];
            // Java: fullPath.getChars(0, lenFullPath, partname, 0);
            for (idx, ch) in full_path.chars().enumerate() {
                partname[idx] = ch;
            }

            // Java: int i;
            let mut i: i32;
            // Java: for(i = lenFullPath - 1; i >= 0 && partname[i] != FORWARD_SLASH_CHAR; --i)
            i = len_full_path as i32 - 1;
            while i >= 0 && partname[i as usize] != FORWARD_SLASH_CHAR
            {
               i -= 1;
            }

            let mut lenPartName = lenPartName;
            // Java: if (i < 0)
            if i < 0
            {
               // Java: PACKAGE_RELATIONSHIPS_ROOT_URI.getRawPath().getChars(0, 12, partname, lenFullPath);
               let rel_path = package_relationships_root_uri().getRawPath().unwrap_or_default();
               for (idx, ch) in rel_path.chars().take(12).enumerate() {
                   if len_full_path + idx < partname.len() {
                       partname[len_full_path + idx] = ch;
                   }
               }
            }
            // Java: else
            else
            {
               // Java: --lenPartName;
               lenPartName -= 1;
               // Java: int lenFileName = lenFullPath - i - 1;
               let lenFileName = len_full_path as i32 - i - 1;
               // Java: if (lenFileName > 0)
               if lenFileName > 0
               {
                  // Java: fullPath.getChars(i + 1, lenFullPath, partname, i + 7);
                  let src_start = (i + 1) as usize;
                  let dst_start = (i + 7) as usize;
                  let fp_chars: Vec<char> = full_path.chars().collect();
                  for j in 0..lenFileName as usize {
                      if dst_start + j < partname.len() && src_start + j < fp_chars.len() {
                          partname[dst_start + j] = fp_chars[src_start + j];
                      }
                  }
               }

               // Java: ++i;
               i += 1;
               // Java: partname[i++] = '_';
               partname[i as usize] = '_'; i += 1;
               // Java: partname[i++] = 'r';
               partname[i as usize] = 'r'; i += 1;
               // Java: partname[i++] = 'e';
               partname[i as usize] = 'e'; i += 1;
               // Java: partname[i++] = 'l';
               partname[i as usize] = 'l'; i += 1;
               // Java: partname[i++] = 's';
               partname[i as usize] = 's'; i += 1;
               // Java: partname[i++] = '/';
               partname[i as usize] = '/'; i += 1;
               // Java: i = lenFullPath + 6;
               i = len_full_path as i32 + 6;
               // Java: partname[i++] = '.';
               partname[i as usize] = '.'; i += 1;
               // Java: partname[i++] = 'r';
               partname[i as usize] = 'r'; i += 1;
               // Java: partname[i++] = 'e';
               partname[i as usize] = 'e'; i += 1;
               // Java: partname[i++] = 'l';
               partname[i as usize] = 'l'; i += 1;
               // Java: partname[i++] = 's';
               partname[i as usize] = 's'; i += 1;
            }

            // Java: try
            {
               // Java: return createPartName(new String(partname, 0, lenPartName));
               let name_str: String = partname[..lenPartName].iter().collect();
               return Self::createPartName_str(&name_str).ok();
            }
            // Java: catch (InvalidFormatException var8)
            {
               // Java: return null;
               #[allow(unreachable_code)]
               return None;
            }
         }
      }
    }

    // Java: static String resolvePartName(String partName)
    pub fn resolvePartName(part_name: &str) -> String {
      // Java: return partName.replace('\\', '/');
      part_name.replace('\\', "/")
    }
}
