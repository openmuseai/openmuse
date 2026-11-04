// Java: import java.util.ArrayList
// Java: import java.util.Arrays
// Java: import java.util.List
// Java: import java.util.regex.Matcher
// Java: import java.util.regex.Pattern

use super::RichTextRange::RichTextRange;

// Java: public class RichTextUtils
pub struct RichTextUtils {
}

// Java: public class RichTextUtils
impl RichTextUtils {
    // Java: RichTextUtils()
    pub fn new() -> Self {
      // Java: super();
      RichTextUtils {}
    }

    // Java: static String removeHtmlElement(String htmlStr)
    pub fn removeHtmlElement(_htmlStr: &str) -> String {
      // Java: return isEmpty(htmlStr) ? "" : htmlStr.replaceAll("</?[^>]+>", "");
      todo!()
    }

    // Java: static String adapterHtmlStr(String htmlStr)
    pub fn adapterHtmlStr(_htmlStr: &str) -> String {
      // Java: if (isEmpty(htmlStr))
      if Self::isEmpty(Some(_htmlStr)) {
         // Java: return "";
         return String::new();
      }
      // Java: else
      // Java: String convertPStr = htmlStr.replaceAll("</p>", "\n</p>");
      // Java: String convertLiStr = convertPStr.replaceAll("</li>", "\n</li>");
      // Java: String convertBrStr = convertLiStr.replaceAll("<br>", "\n");
      // Java: String convertJoinStr = convertBrStr.replaceAll("&amp;", "&");
      // Java: return convertJoinStr.replaceAll("&nbsp;", " ");
      todo!()
    }

    // Java: static void parseHtmlStr(int left, int right, String htmlStr, List<RichTextRange> rangeList)
    pub fn parseHtmlStr(_left: i32, _right: i32, _htmlStr: &str, _rangeList: &mut Vec<RichTextRange>) {
      // Java: if (!isEmpty(htmlStr))
      // Java: int paddingLeftIndex = htmlStr.indexOf("<");
      // Java: if (paddingLeftIndex != -1)
      // Java: String startElementStr = getStartElementStr(htmlStr);
      // Java: String endElementStr = getEndElementStr(htmlStr);
      // Java: String startFlagStr = getStartFlagStr(startElementStr);
      // Java: String endFlagStr = getEndFlagStr(endElementStr);
      // Java: if (!isEmpty(startElementStr) && !isEmpty(endElementStr))
      // Java: int startFlagIndex = htmlStr.indexOf(startFlagStr);
      // Java: int endFlagIndex = htmlStr.indexOf(endFlagStr);
      // Java: if (startFlagIndex != -1 && endFlagIndex != -1)
      // Java: String rightHtmlStr = htmlStr.substring(endFlagIndex + endFlagStr.length());
      // Java: String rightPureStr = removeHtmlElement(rightHtmlStr);
      // Java: int paddingRightIndex = 0;
      // Java: if (!isEmpty(rightPureStr))
      // Java: paddingRightIndex = rightPureStr.length();
      // Java: int startRange = left + paddingLeftIndex;
      // Java: int endRange = right + paddingRightIndex;
      // Java: RichTextRange richTextRange = new RichTextRange();
      // Java: richTextRange.startRange = startRange;
      // Java: richTextRange.endRange = endRange;
      // Java: richTextRange.element = endElementStr;
      // Java: makeClassPro(richTextRange, startElementStr);
      // Java: makeStylePro(richTextRange, startElementStr);
      // Java: if (rangeList == null)
      // Java: rangeList = new ArrayList();
      // Java: rangeList.add(richTextRange);
      // Java: String values = htmlStr.substring(startFlagIndex + startFlagStr.length(), endFlagIndex);
      // Java: if (!isEmpty(values))
      // Java: parseHtmlStr(startRange, endRange, values, rangeList);
      // Java: if (!isEmpty(rightHtmlStr))
      // Java: parseHtmlStr(startRange + removeHtmlElement(values).length(), right, rightHtmlStr, rangeList);
      todo!()
    }

    // Java: static boolean isEmpty(CharSequence str)
    pub fn isEmpty(_str: Option<&str>) -> bool {
      // Java: return str == null || str.length() == 0;
      _str.map_or(true, |s| s.is_empty())
    }

    // Java: static String getEndElementStr(String htmlStr)
    pub fn getEndElementStr(_htmlStr: &str) -> String {
      // Java: int start = htmlStr.indexOf("<");
      // Java: int end = htmlStr.indexOf(">");
      // Java: String element = htmlStr.substring(start + 1, end);
      // Java: if (!isEmpty(element))
      // Java: element.trim();
      // Java: int elementEndIndex = element.indexOf(" ");
      // Java: return elementEndIndex != -1 ? element.substring(0, elementEndIndex) : element;
      // Java: else
      // Java: return "";
      todo!()
    }

    // Java: static String getStartElementStr(String htmlStr)
    pub fn getStartElementStr(_htmlStr: &str) -> String {
      // Java: int start = htmlStr.indexOf("<");
      // Java: int end = htmlStr.indexOf(">");
      // Java: String element = htmlStr.substring(start + 1, end);
      // Java: return !isEmpty(element) ? element.trim() : "";
      todo!()
    }

    // Java: static void makeClassPro(RichTextRange range, String proStr)
    pub fn makeClassPro(_range: &mut RichTextRange, _proStr: &str) {
      // Java: String classProStr = getProStr("class", proStr);
      // Java: if (!isEmpty(classProStr))
      // Java: splitClassPro(range, classProStr);
      todo!()
    }

    // Java: static void makeStylePro(RichTextRange range, String proStr)
    pub fn makeStylePro(_range: &mut RichTextRange, _proStr: &str) {
      // Java: String styleProStr = getProStr("style", proStr);
      // Java: if (!isEmpty(styleProStr))
      // Java: if (range.styleList == null)
      // Java: range.styleList = new ArrayList();
      // Java: range.styleList.add(styleProStr);
      todo!()
    }

    // Java: static String getProStr(String typeStr, String proStr)
    pub fn getProStr(_typeStr: &str, _proStr: &str) -> String {
      // Java: if (!isEmpty(proStr) && !isEmpty(typeStr) && proStr.contains(typeStr))
      // Java: proStr = proStr.trim();
      // Java: if (proStr.contains("color") && proStr.contains("background-color"))
      // Java: return "";
      // Java: else
      // Java: proStr = proStr + " ";
      // Java: String patternStr = typeStr + "\\s*=\\s*\"(.*)\"\\s+";
      // Java: Pattern pattern = Pattern.compile(patternStr);
      // Java: Matcher matcher = pattern.matcher(proStr);
      // Java: return matcher.find() ? matcher.group(1) : "";
      // Java: else
      // Java: return "";
      todo!()
    }

    // Java: static void splitClassPro(RichTextRange range, String proStr)
    pub fn splitClassPro(_range: &mut RichTextRange, _proStr: &str) {
      // Java: String[] pros = proStr.split(" ");
      // Java: range.proList = Arrays.asList(pros);
      _range.proList = _proStr.split(' ').map(|s| s.to_string()).collect();
    }

    // Java: static String getStartFlagStr(String element)
    pub fn getStartFlagStr(_element: &str) -> String {
      // Java: return "<" + element + ">";
      format!("<{}>", _element)
    }

    // Java: static String getEndFlagStr(String element)
    pub fn getEndFlagStr(_element: &str) -> String {
      // Java: return "</" + element + ">";
      format!("</{}>", _element)
    }

}
