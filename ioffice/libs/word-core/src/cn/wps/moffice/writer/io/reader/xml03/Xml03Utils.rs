// Java: cn.wps.moffice.writer.io.reader.xml03.Xml03Utils
//
// ★M-STAGE1 P2 —— 从 **compile-surface 空壳**（2 行 / 仅 `pub struct Xml03Utils;`）
//   **做实**为 Java 原文的 4 个静态方法。
//
// 它是 `SettingsImporter.setCharacterSpacingControl` 的必经依赖
// （`isXml03()` 为真时对 `charSpacingControl` 做首字母小写化）。
//
// ★`replaceHyphenMinusToUpperCase` 的 Java 原文用**先自增再取值**的写法：
//   `else { ++i; if (i < size) sb.append(Character.toUpperCase(content.charAt(i))); }`
//   语义 = **丢弃连字符，把紧跟其后的字符大写并消费掉**（该字符不再被循环重复处理）。
//   `"a-bc"` → `"aBc"`（逐轮追踪：i=0 收 'a'；i=1 见 '-'，++i 后收 'b'→'B'；
//   循环 ++i 跳到 i=3 收 'c'）。由测试锁定。

// java public class Xml03Utils
pub struct Xml03Utils;

impl Xml03Utils {
    // java public static String firstCharToLowerCase(String content)
    pub fn firstCharToLowerCase(content: Option<&str>) -> Option<String> {
        // java if (null != content && content.length() != 0)
        if content.is_some() && content.unwrap().len() != 0 {
            // java char[] arr = content.toCharArray();
            let mut arr: Vec<char> = content.unwrap().chars().collect();
            // java arr[0] = Character.toLowerCase(arr[0]);
            arr[0] = arr[0].to_lowercase().next().unwrap_or(arr[0]);
            // java return new String(arr);
            return Some(arr.into_iter().collect());
        // java else
        } else {
            // java return content;
            return content.map(|s| s.to_string());
        }
    }

    // java public static String firstCharToUpperCase(String content)
    pub fn firstCharToUpperCase(content: Option<&str>) -> Option<String> {
        // java if (null != content && content.length() != 0)
        if content.is_some() && content.unwrap().len() != 0 {
            // java char[] arr = content.toCharArray();
            let mut arr: Vec<char> = content.unwrap().chars().collect();
            // java arr[0] = Character.toUpperCase(arr[0]);
            arr[0] = arr[0].to_uppercase().next().unwrap_or(arr[0]);
            // java return new String(arr);
            return Some(arr.into_iter().collect());
        // java else
        } else {
            // java return content;
            return content.map(|s| s.to_string());
        }
    }

    // java public static String replaceHyphenMinusToUpperCase(String content)
    pub fn replaceHyphenMinusToUpperCase(content: Option<&str>) -> Option<String> {
        // java if (null != content && content.length() != 0)
        if content.is_some() && content.unwrap().len() != 0 {
            // java StringBuilder sb = new StringBuilder();
            let mut sb = String::new();
            // java int size = content.length();
            let size: i32 = content.unwrap().chars().count() as i32;
            let chars: Vec<char> = content.unwrap().chars().collect();
            // java for(int i = 0; i < size; ++i)
            let mut i: i32 = 0;
            while i < size {
                // java char ch = content.charAt(i);
                let ch: char = chars[i as usize];
                // java if ('-' != ch)
                if '-' != ch {
                    // java sb.append(ch);
                    sb.push(ch);
                // java else
                } else {
                    // java ++i;
                    i += 1;
                    // java if (i < size)
                    if i < size {
                        // java sb.append(Character.toUpperCase(content.charAt(i)));
                        let c = chars[i as usize];
                        sb.push(c.to_uppercase().next().unwrap_or(c));
                    }
                }
                // java ++i
                i += 1;
            }
            // java return sb.toString();
            return Some(sb);
        // java else
        } else {
            // java return content;
            return content.map(|s| s.to_string());
        }
    }

    // java public static String replaceUpperCaseToHyphenMinus(String content)
    pub fn replaceUpperCaseToHyphenMinus(content: Option<&str>) -> Option<String> {
        // java if (null != content && content.length() != 0)
        if content.is_some() && content.unwrap().len() != 0 {
            // java char[] chars = content.toCharArray();
            let chars: Vec<char> = content.unwrap().chars().collect();
            // java StringBuilder sb = new StringBuilder();
            let mut sb = String::new();
            // java for(char ch : chars)
            for ch in chars {
                // java if (Character.isUpperCase(ch))
                if ch.is_uppercase() {
                    // java sb.append('-');
                    sb.push('-');
                    // java sb.append(Character.toLowerCase(ch));
                    sb.push(ch.to_lowercase().next().unwrap_or(ch));
                // java else
                } else {
                    // java sb.append(ch);
                    sb.push(ch);
                }
            }
            // java return sb.toString();
            return Some(sb);
        // java else
        } else {
            // java return content;
            return content.map(|s| s.to_string());
        }
    }
}
