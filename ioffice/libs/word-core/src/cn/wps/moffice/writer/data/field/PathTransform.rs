// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.moffice.writer.core.CharParse
// rust: DEFERRED_NON_FUNCTION —— `core::CharParse` → `core::CharMap` 闭包
//   （实测单独登记 +42 错误）整体未接线。本处唯一用途是
//   `CharParse.getInstance().isLetter(ch)`（`CharParse.java`：
//   `'A'<=ch&&ch<='Z' || 'a'<=ch&&ch<='z'`）—— 该判据为**纯字符分类**，
//   就地内联以保持逻辑等价，待闭包接线后改回真实调用。
// use crate::cn::wps::moffice::writer::core::CharParse::*;
use stubs::java::lang::StringBuffer::StringBuffer;

// java public static final String g_szPathSep = "/\\";
pub(crate) const g_szPathSep: &str = "/\\";
// java public static final String g_szLocalIP = "127.0.0.1";
pub(crate) const g_szLocalIP: &str = "127.0.0.1";
// java public static final char chForwardSlash = '/';
pub(crate) const chForwardSlash: char = '/';
// java public static final char chBackwardSlash = '\\';
pub(crate) const chBackwardSlash: char = '\\';
// java public static final char chDot = '.';
pub(crate) const chDot: char = '.';
// java public static final int _MAX_PATH = 512;
pub(crate) const _MAX_PATH: i32 = 512;
// java public static final int PATH_TYPE_RELATIVEFILEPATH = 0;
pub(crate) const PATH_TYPE_RELATIVEFILEPATH: i32 = 0;
// java public static final int PATH_TYPE_ABSOLUTEFILEPATH = 1;
pub(crate) const PATH_TYPE_ABSOLUTEFILEPATH: i32 = 1;
// java public static final int PATH_TYPE_NETURL = 2;
pub(crate) const PATH_TYPE_NETURL: i32 = 2;
// java public static final int PATH_TYPE_EMAILADDRESS = 3;
pub(crate) const PATH_TYPE_EMAILADDRESS: i32 = 3;
// java public static final int PATH_TYPE_MAX = 4;
pub(crate) const PATH_TYPE_MAX: i32 = 4;
// java public static String kszHttpHead = "http:";
pub(crate) static kszHttpHead: &str = "http:";
// java public static String kszHttpsHead = "https:";
pub(crate) static kszHttpsHead: &str = "https:";
// java public static String kszFtpHead = "ftp:";
pub(crate) static kszFtpHead: &str = "ftp:";
// java public static String kszFtpsHead = "ftps:";
pub(crate) static kszFtpsHead: &str = "ftps:";
// java public static String kszEmailHead = "mailto:";
pub(crate) static kszEmailHead: &str = "mailto:";

// java public class PathTransform
#[derive(Default)]
pub struct PathTransform {
}

// java public class PathTransform
impl PathTransform {
    // java public static int getPathType(String strPath)
    pub fn getPathType(strPath: Option<&str>) -> i32 {
      // java if (null != strPath && strPath.length() != 0)
      if let Some(strPath) = strPath {
        if (strPath.len() as i32) != 0 {
          // java if (beginWithPattern(strPath, kszHttpHead))
          if Self::beginWithPattern(strPath, kszHttpHead) {
            // java return 2;
            return 2;
          }
          // java else if (beginWithPattern(strPath, kszHttpsHead))
          else if Self::beginWithPattern(strPath, kszHttpsHead) {
            // java return 2;
            return 2;
          }
          // java else if (!beginWithPattern(strPath, kszFtpHead) && !beginWithPattern(strPath, kszFtpsHead))
          else if !Self::beginWithPattern(strPath, kszFtpHead) && !Self::beginWithPattern(strPath, kszFtpsHead) {
            // java if (beginWithPattern(strPath, kszEmailHead))
            if Self::beginWithPattern(strPath, kszEmailHead) {
              // java return 3;
              return 3;
            }
            // java else if (_IsAbsolutePath(strPath))
            else if Self::_IsAbsolutePath(Some(strPath)) {
              // java return 1;
              return 1;
            }
            // java else
            else {
              // java return _IsRelativePath(strPath) ? 0 : 1;
              return (if Self::_IsRelativePath(Some(strPath)) { 0 } else { 1 });
            }
          }
          // java else
          else {
            // java return 2;
            return 2;
          }
        } else {
          return 1;
        }
      }
      // java else
      else {
        // java return 1;
        return 1;
      }
    }

    // java static boolean _IsAbsolutePath(String AbsolutePath)
    pub fn _IsAbsolutePath(AbsolutePath: Option<&str>) -> bool {
      // java if (AbsolutePath == null)
      if AbsolutePath.is_none() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java StringBuffer RootPath = new StringBuffer();
        let mut RootPath: StringBuffer = StringBuffer::new();
        // java int nRoot = 0;
        let mut nRoot: i32 = 0;
        // java _GetRootPath(RootPath, AbsolutePath, false);
        Self::_GetRootPath(&mut RootPath, AbsolutePath, false);
        // java nRoot = RootPath.length();
        nRoot = RootPath.length();
        // java if (nRoot == 0)
        if nRoot == 0 {
          // java return false;
          return false;
        }
        // java else if (RootPath.charAt(0) == '/' && RootPath.charAt(1) == '/' && RootPath.charAt(2) != 0)
        else if RootPath.charAt(0) == '/' && RootPath.charAt(1) == '/' && nRoot > 2 && RootPath.charAt(2) != '\0' {
          // java return true;
          return true;
        }
        // java else
        else {
          // java return (nRoot & 1) == 1;
          return (nRoot & 1) == 1;
        }
      }
    }

    // java static boolean _IsRelativePath(String RelativePath)
    pub fn _IsRelativePath(RelativePath: Option<&str>) -> bool {
      // java if (RelativePath == null)
      if RelativePath.is_none() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java StringBuffer RootPath = new StringBuffer();
        let mut RootPath: StringBuffer = StringBuffer::new();
        // java int nRoot = 0;
        let mut nRoot: i32 = 0;
        // java _GetRootPath(RootPath, RelativePath, false);
        Self::_GetRootPath(&mut RootPath, RelativePath, false);
        // java nRoot = RootPath.length();
        nRoot = RootPath.length();
        // java if (nRoot == 0)
        if nRoot == 0 {
          // java return false;
          return false;
        }
        // java else if (RootPath.charAt(0) == '/' && RootPath.charAt(1) == '/' && RootPath.charAt(2) != 0)
        else if RootPath.charAt(0) == '/' && RootPath.charAt(1) == '/' && nRoot > 2 && RootPath.charAt(2) != '\0' {
          // java return false;
          return false;
        }
        // java else
        else {
          // java return (nRoot & 1) != 1;
          return (nRoot & 1) != 1;
        }
      }
    }

    // java static void _GetRootPath(StringBuffer szRoot, String szPath, boolean bUseNetWork)
    pub fn _GetRootPath(szRoot: &mut StringBuffer, szPath: Option<&str>, bUseNetWork: bool) {
      // java szRoot.setLength(0);
      szRoot.setLength(0);
      // java if (szPath != null && szPath.length() < 512 && szPath.length() > 1)
      if let Some(szPath) = szPath {
        if (szPath.len() as i32) < 512 && (szPath.len() as i32) > 1 {
          // java int szIndex = 0;
          let mut szIndex: i32 = 0;
          let char_at = |s: &str, i: i32| -> char {
            s.chars().nth(i as usize).unwrap_or('\0')
          };
          // java if (szPath.charAt(szIndex + 1) == ':')
          if char_at(szPath, szIndex + 1) == ':' {
            // java if (!CharParse.getInstance().isLetter(szPath.charAt(szIndex + 0)))
            // rust: 内联 `CharParse.isLetter`（见上方 DEFERRED 说明，逻辑逐字等价）。
            let __ch = char_at(szPath, szIndex + 0);
            if !('A' <= __ch && __ch <= 'Z' || 'a' <= __ch && __ch <= 'z') {
              // java return;
              return;
            }
            // java szRoot.append(szPath.charAt(szIndex + 0));
            szRoot.append(char_at(szPath, szIndex + 0));
            // java szRoot.append(':');
            szRoot.append(':');
            // java szIndex += 2;
            szIndex = szIndex + 2;
          }
          // java if ((szPath.charAt(szIndex + 0) == '\\' || szPath.charAt(szIndex + 0) == '/') && (szPath.charAt(szIndex + 1) == '\\' || szPath.charAt(szIndex + 1) == '/') && szPath.charAt(szIndex + 2) != 0)
          if (char_at(szPath, szIndex + 0) == '\\' || char_at(szPath, szIndex + 0) == '/')
            && (char_at(szPath, szIndex + 1) == '\\' || char_at(szPath, szIndex + 1) == '/')
            && char_at(szPath, szIndex + 2) != '\0'
          {
            // java if (szPath.charAt(szIndex + 2) != '\\' && szPath.charAt(szIndex + 2) != '/')
            if char_at(szPath, szIndex + 2) != '\\' && char_at(szPath, szIndex + 2) != '/' {
              // java szRoot.append('/');
              szRoot.append('/');
              // java szRoot.append('/');
              szRoot.append('/');
              // java szIndex += 2;
              szIndex = szIndex + 2;
              // java StringBuffer Servername;
              let mut Servername: StringBuffer = StringBuffer::new();
              // java for(Servername = new StringBuffer() szPath.charAt(szIndex) != 0 && szPath.charAt(szIndex) != '\\' && szPath.charAt(szIndex) != '/' ++szIndex)
              while char_at(szPath, szIndex) != '\0' && char_at(szPath, szIndex) != '\\' && char_at(szPath, szIndex) != '/' {
                // java Servername.append(szPath.charAt(szIndex));
                Servername.append(char_at(szPath, szIndex));
                szIndex += 1;
              }
              // java String ServerIP = null;
              let mut ServerIP: Option<String> = None;
              // java if (!bUseNetWork)
              if !bUseNetWork {
                // java ServerIP = Servername.toString();
                ServerIP = Some(Servername.toString());
              }
              // java szRoot.append(ServerIP);
              if let Some(ref ip) = ServerIP {
                szRoot.append_str(ip);
              }
              // java szRoot.append('/');
              szRoot.append('/');
            }
            // java else
            else {
              let mut i: i32 = szIndex;
              // java for(int i = szIndex; i < szPath.length(); ++i)
              while i < (szPath.len() as i32) {
                // java if (szPath.charAt(i) != '\\' && szPath.charAt(i) != '/')
                if char_at(szPath, i) != '\\' && char_at(szPath, i) != '/' {
                  // java szRoot.setLength(0);
                  szRoot.setLength(0);
                  // java return;
                  return;
                }
                i += 1;
              }
              // java szRoot.append('/');
              szRoot.append('/');
            }
          }
          // java else if (szPath.charAt(szIndex) != '/' && szPath.charAt(szIndex) != '\\')
          else if char_at(szPath, szIndex) != '/' && char_at(szPath, szIndex) != '\\' {
            // java szRoot.append('.');
            szRoot.append('.');
            // java szRoot.append('/');
            szRoot.append('/');
          }
          // java else
          else {
            // java szRoot.append('/');
            szRoot.append('/');
          }
        }
      }
    }

    // java public static boolean beginWithPattern(String value, String pattern)
    pub fn beginWithPattern(value: &str, pattern: &str) -> bool {
      // java Assert.assertNotNull("value should not be null!", value);
      Assert::assertNotNull_msg("value should not be null!", &Some(value));
      // java Assert.assertNotNull("pattern should not be null!", pattern);
      Assert::assertNotNull_msg("pattern should not be null!", &Some(pattern));
      // java return 0 == value.indexOf(pattern);
      return value.find(pattern).map(|i| i as i32).unwrap_or(-1) == 0;
    }

}
