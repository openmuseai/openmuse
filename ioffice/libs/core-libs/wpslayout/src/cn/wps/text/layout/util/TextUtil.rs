// Java: import java.math.BigDecimal
// Java: import java.math.RoundingMode
// Java: import java.text.DecimalFormat
// Java: import java.util.ArrayList
// Java: import java.util.regex.Matcher
// Java: import java.util.regex.Pattern

use super::TextRangeType::TextRangeType;

pub struct Range {
    start_pos: i32,
    end_pos: i32,
    range_type: TextRangeType,
}

impl Range {
    pub fn new(start: i32, end: i32, range_type: TextRangeType) -> Self {
        Range { start_pos: start, end_pos: end, range_type }
    }
    pub fn start(&self) -> i32 { self.start_pos }
    pub fn end(&self) -> i32 { self.end_pos }
    pub fn r#type(&self) -> TextRangeType { self.range_type }
}

// Java: public class TextUtil
pub struct TextUtil {
}

// Java: public class TextUtil
impl TextUtil {
    // Java: TextUtil()
    fn new() -> Self {
      // Java: super();
      TextUtil {}
    }

    // Java: static String updateDynamicTextContent(String typoText, double originFraction, double endRate, double src, boolean destAbs, boolean srcAbs, int mode, ArrayList<Range> digitRange, boolean optimizeRate)
    pub fn updateDynamicTextContent(typoText: &str, originFraction: f64, mut endRate: f64, mut src: f64, destAbs: bool, srcAbs: bool, mode: i32, digitRange: &mut Vec<Range>, optimizeRate: bool) -> String {
        // Java: if (!splitTextDigit(typoText, digitRange))
        if !Self::splitTextDigit(typoText, digitRange)
        {
            // Java: return typoText;
            return typoText.to_string();
        }
        // Java: else
        else
        {
            // Java: StringBuilder sb = new StringBuilder();
            let mut sb = String::new();
            // Java: for(Range range : digitRange)
            for range in digitRange.iter()
            {
                // Java: if (range.type() == TextRangeType.Text_Digit)
                if range.r#type() == TextRangeType::Text_Digit
                {
                    // Java: String subStr = typoText.substring(range.start(), range.end());
                    let subStr = &typoText[range.start() as usize..range.end() as usize];
                    // Java: ArrayList<Range> ranges = new ArrayList();
                    let mut ranges: Vec<Range> = Vec::new();
                    // Java: if (calcDynamicDigitsFromText(subStr, ranges))
                    if Self::calcDynamicDigitsFromText(subStr, &mut ranges)
                    {
                        // Java: for(Range rng : ranges)
                        for rng in ranges.iter()
                        {
                            // Java: String digitStr = subStr.substring(rng.start(), rng.end());
                            let digitStr = &subStr[rng.start() as usize..rng.end() as usize];
                            // Java: double destValue = toDoubleValue(digitStr);
                            let destValue = Self::toDoubleValue(digitStr);
                            // Java: if (Double.isNaN(destValue))
                            if destValue.is_nan()
                            {
                                // Java: break;
                                break;
                            }

                            // Java: double fraction = originFraction;
                            let mut fraction = originFraction;
                            // Java: switch (mode)
                            match mode
                            {
                                // Java: case 1:
                                1 => {
                                    // Java: double srcRate = src;
                                    let mut srcRate = src;
                                    // Java: double destRate = endRate;
                                    let mut destRate = endRate;
                                    // Java: if (srcAbs)
                                    if srcAbs
                                    {
                                        // Java: srcRate = src / destValue;
                                        srcRate = src / destValue;
                                    }
                                    // Java: if (destAbs)
                                    if destAbs
                                    {
                                        // Java: destRate = endRate / destValue;
                                        destRate = endRate / destValue;
                                    }
                                    // Java: if (optimizeRate)
                                    if optimizeRate
                                    {
                                        // Java: fraction = updateFraction(digitStr, originFraction);
                                        fraction = Self::updateFraction(digitStr, originFraction);
                                    }
                                    // Java: fraction = fraction * (destRate - srcRate) + srcRate;
                                    fraction = fraction * (destRate - srcRate) + srcRate;
                                    // Java: break;
                                }
                                // Java: case 2:
                                2 => {
                                    // Java: double byRate = endRate;
                                    let mut byRate = endRate;
                                    // Java: if (destAbs)
                                    if destAbs
                                    {
                                        // Java: byRate = endRate / destValue;
                                        byRate = endRate / destValue;
                                    }
                                    // Java: long delta = Math.round(destValue * byRate);
                                    let delta = (destValue * byRate).round() as i64;
                                    // Java: byRate = (double)delta / destValue;
                                    byRate = delta as f64 / destValue;
                                    // Java: String v = String.valueOf(delta);
                                    let v = delta.to_string();
                                    // Java: if (optimizeRate)
                                    if optimizeRate
                                    {
                                        // Java: fraction = updateFraction(v, originFraction);
                                        fraction = Self::updateFraction(&v, originFraction);
                                    }
                                    // Java: fraction = src + fraction * byRate;
                                    fraction = src + fraction * byRate;
                                    // Java: endRate = src;
                                    endRate = src;
                                    // Java: break;
                                }
                                // Java: case 3:
                                3 => {
                                    // Java: if (destAbs)
                                    if destAbs
                                    {
                                        // Java: endRate /= destValue;
                                        endRate /= destValue;
                                    }
                                    // Java: if (optimizeRate)
                                    if optimizeRate
                                    {
                                        // Java: fraction = updateFraction(digitStr, originFraction);
                                        fraction = Self::updateFraction(digitStr, originFraction);
                                    }
                                    // Java: break;
                                }
                                // Java: default:
                                _ => {
                                    // Java: if (optimizeRate)
                                    if optimizeRate
                                    {
                                        // Java: fraction = updateFraction(digitStr, originFraction);
                                        fraction = Self::updateFraction(digitStr, originFraction);
                                    }
                                    // Java: if (srcAbs)
                                    if srcAbs
                                    {
                                        // Java: if (destValue == (double)0.0F)
                                        if destValue == 0.0f64
                                        {
                                            // Java: src = (double)0.0F;
                                            src = 0.0f64;
                                        }
                                        // Java: else
                                        else
                                        {
                                            // Java: src /= destValue;
                                            src /= destValue;
                                        }
                                    }
                                    // Java: if (destAbs)
                                    if destAbs
                                    {
                                        // Java: if (destValue == (double)0.0F)
                                        if destValue == 0.0f64
                                        {
                                            // Java: endRate = (double)1.0F;
                                            endRate = 1.0f64;
                                        }
                                        // Java: else
                                        else
                                        {
                                            // Java: endRate /= destValue;
                                            endRate /= destValue;
                                        }
                                    }
                                    // Java: if (src > endRate)
                                    if src > endRate
                                    {
                                        // Java: fraction = (double)1.0F - fraction;
                                        fraction = 1.0f64 - fraction;
                                    }
                                    // Java: double st = Math.min(src, endRate);
                                    let st = src.min(endRate);
                                    // Java: double ed = Math.max(src, endRate);
                                    let ed = src.max(endRate);
                                    // Java: double targetRange = ed - st;
                                    let targetRange = ed - st;
                                    // Java: if (targetRange != (double)0.0F)
                                    if targetRange != 0.0f64
                                    {
                                        // Java: endRate = ed;
                                        endRate = ed;
                                        // Java: fraction = (fraction * targetRange + st) / ed;
                                        fraction = (fraction * targetRange + st) / ed;
                                    }
                                    // Java: else
                                    else
                                    {
                                        // Java: fraction = (double)1.0F;
                                        fraction = 1.0f64;
                                    }
                                }
                            }

                            // Java: if (rng.type() == TextRangeType.Text_Normal)
                            if rng.r#type() == TextRangeType::Text_Normal
                            {
                                // Java: BigDecimal origin = (new BigDecimal(digitStr)).multiply(new BigDecimal(Double.toString(endRate)));
                                let origin_val: f64 = Self::toDoubleValue(digitStr) * endRate;
                                // Java: BigDecimal dest = origin.multiply(new BigDecimal(Double.toString(fraction))).setScale(0, RoundingMode.HALF_UP);
                                let dest_val = (origin_val * fraction).round();
                                // Java: String doubleStr = decimalToString(dest);
                                let doubleStr = Self::decimalToString(dest_val);
                                // Java: int dotIndex = doubleStr.indexOf(46);
                                let dotIndex = doubleStr.find('.').map(|i| i as i32).unwrap_or(-1);
                                // Java: String destStr;
                                let destStr: String;
                                // Java: if (dotIndex > 0)
                                if dotIndex > 0
                                {
                                    // Java: destStr = doubleStr.substring(0, dotIndex);
                                    destStr = doubleStr[..dotIndex as usize].to_string();
                                }
                                // Java: else
                                else
                                {
                                    // Java: destStr = doubleStr;
                                    destStr = doubleStr;
                                }
                                // Java: String destText = fillDynamicDigitWithBlank(destStr, digitStr);
                                let destText = Self::fillDynamicDigitWithBlank(&destStr, digitStr);
                                // Java: sb.append(destText);
                                sb.push_str(&destText);
                            }
                            // Java: else if (rng.type() == TextRangeType.Text_ThousandsFloat)
                            else if rng.r#type() == TextRangeType::Text_ThousandsFloat
                            {
                                // Java: int dotIndex = digitStr.indexOf(46);
                                let dotIndex = digitStr.find('.').map(|i| i as i32).unwrap_or(-1);
                                // Java: int keepLength = digitStr.length() - dotIndex;
                                let keepLength = digitStr.len() as i32 - dotIndex;
                                // Java: String standardStr = digitStr.replace(",", "");
                                let standardStr = digitStr.replace(",", "");
                                // Java: BigDecimal origin = (new BigDecimal(standardStr)).multiply(new BigDecimal(Double.toString(endRate)));
                                let origin_val: f64 = standardStr.parse::<f64>().unwrap_or(0.0) * endRate;
                                // Java: BigDecimal dest = origin.multiply(new BigDecimal(Double.toString(fraction)));
                                let dest_val = origin_val * fraction;
                                // Java: String doubleStr = decimalToString(dest, fractionLength(digitStr));
                                let doubleStr = Self::decimalToString_with_len(dest_val, Self::fractionLength(digitStr));
                                // Java: String thousandsStr = addThousandsSymbol(doubleStr);
                                let thousandsStr = Self::addThousandsSymbol(&doubleStr);
                                // Java: int destDotIndex = thousandsStr.indexOf(46);
                                let destDotIndex = thousandsStr.find('.').map(|i| i as i32).unwrap_or(-1);
                                // Java: String destText = thousandsStr.substring(0, destDotIndex + keepLength);
                                let destText = &thousandsStr[..(destDotIndex + keepLength) as usize];
                                // Java: String destTextWithBlank = fillDynamicDigitWithBlank(destText, digitStr);
                                let destTextWithBlank = Self::fillDynamicDigitWithBlank(destText, digitStr);
                                // Java: sb.append(destTextWithBlank);
                                sb.push_str(&destTextWithBlank);
                            }
                            // Java: else if (rng.type() == TextRangeType.Text_ThousandsInt)
                            else if rng.r#type() == TextRangeType::Text_ThousandsInt
                            {
                                // Java: String standardStr = digitStr.replace(",", "");
                                let standardStr = digitStr.replace(",", "");
                                // Java: BigDecimal origin = (new BigDecimal(standardStr)).multiply(new BigDecimal(Double.toString(endRate)));
                                let origin_val: f64 = standardStr.parse::<f64>().unwrap_or(0.0) * endRate;
                                // Java: BigDecimal dest = origin.multiply(new BigDecimal(Double.toString(fraction))).setScale(0, RoundingMode.HALF_UP);
                                let dest_val = (origin_val * fraction).round();
                                // Java: String doubleStr = decimalToString(dest);
                                let doubleStr = Self::decimalToString(dest_val);
                                // Java: int dotIndex = doubleStr.indexOf(46);
                                let dotIndex = doubleStr.find('.').map(|i| i as i32).unwrap_or(-1);
                                // Java: String destStr;
                                let destStr: String;
                                // Java: if (dotIndex > 0)
                                if dotIndex > 0
                                {
                                    // Java: destStr = doubleStr.substring(0, dotIndex);
                                    destStr = doubleStr[..dotIndex as usize].to_string();
                                }
                                // Java: else
                                else
                                {
                                    // Java: destStr = doubleStr;
                                    destStr = doubleStr;
                                }
                                // Java: String thousandsStr = addThousandsSymbol(destStr);
                                let thousandsStr = Self::addThousandsSymbol(&destStr);
                                // Java: String destText = fillDynamicDigitWithBlank(thousandsStr, digitStr);
                                let destText = Self::fillDynamicDigitWithBlank(&thousandsStr, digitStr);
                                // Java: sb.append(destText);
                                sb.push_str(&destText);
                            }
                            // Java: else if (rng.type() == TextRangeType.Text_Float)
                            else if rng.r#type() == TextRangeType::Text_Float
                            {
                                // Java: int dotIndex = digitStr.indexOf(46);
                                let dotIndex = digitStr.find('.').map(|i| i as i32).unwrap_or(-1);
                                // Java: int keepLength = digitStr.length() - dotIndex;
                                let keepLength = digitStr.len() as i32 - dotIndex;
                                // Java: BigDecimal origin = (new BigDecimal(digitStr)).multiply(new BigDecimal(Double.toString(endRate)));
                                let origin_val: f64 = digitStr.parse::<f64>().unwrap_or(0.0) * endRate;
                                // Java: BigDecimal dest = origin.multiply(new BigDecimal(Double.toString(fraction)));
                                let dest_val = origin_val * fraction;
                                // Java: String doubleStr = decimalToString(dest, fractionLength(digitStr));
                                let doubleStr = Self::decimalToString_with_len(dest_val, Self::fractionLength(digitStr));
                                // Java: int destDotIndex = doubleStr.indexOf(46);
                                let destDotIndex = doubleStr.find('.').map(|i| i as i32).unwrap_or(-1);
                                // Java: String destText = doubleStr.substring(0, destDotIndex + keepLength);
                                let mut destText = doubleStr[..(destDotIndex + keepLength) as usize].to_string();
                                // Java: destText = fillDynamicDigitWithBlank(destText, digitStr);
                                destText = Self::fillDynamicDigitWithBlank(&destText, digitStr);
                                // Java: sb.append(destText);
                                sb.push_str(&destText);
                            }
                        }
                    }
                    // Java: else
                    else
                    {
                        // Java: sb.append(subStr);
                        sb.push_str(subStr);
                    }
                }
                // Java: else if (range.type() == TextRangeType.Text_Text)
                else if range.r#type() == TextRangeType::Text_Text
                {
                    // Java: String subStr = typoText.substring(range.start(), range.end());
                    let subStr = &typoText[range.start() as usize..range.end() as usize];
                    // Java: sb.append(subStr);
                    sb.push_str(subStr);
                }
            }
            // Java: return sb.toString();
            return sb;
        }
    }

    // Java: static boolean splitTextDigit(String s, ArrayList<Range> digitRange)
    pub fn splitTextDigit(s: &str, digitRange: &mut Vec<Range>) -> bool {
        // Java: boolean result = false;
        let mut result = false;
        // Java: Pattern pattern = Pattern.compile("(\\d{1,3}(,\\d{3}(?!\\d))+(\\.\\d+)?)|(\\d+(\\.?\\d+)?)");
        // Java: Matcher matcher = pattern.matcher(s);
        let chars: Vec<char> = s.chars().collect();
        // Java: int lastNumberEnd;
        let mut lastNumberEnd: usize = 0;
        // Java: int end;
        // Java: for(lastNumberEnd = 0; matcher.find(); lastNumberEnd = end)
        let mut i = 0;
        while i < chars.len() {
            if chars[i].is_ascii_digit() {
                // Java: int start = matcher.start();
                let start = i;
                // Java: end = matcher.end();
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == ',' || chars[i] == '.') {
                    i += 1;
                }
                let end = i;
                // Java: if (lastNumberEnd < start)
                if lastNumberEnd < start {
                    // Java: digitRange.add(new Range(lastNumberEnd, start, TextRangeType.Text_Text));
                    digitRange.push(Range::new(lastNumberEnd as i32, start as i32, TextRangeType::Text_Text));
                }
                // Java: digitRange.add(new Range(start, end, TextRangeType.Text_Digit));
                digitRange.push(Range::new(start as i32, end as i32, TextRangeType::Text_Digit));
                // Java: result = true;
                result = true;
                lastNumberEnd = end;
            } else {
                i += 1;
            }
        }
        // Java: if (lastNumberEnd < s.length())
        if lastNumberEnd < s.len() {
            // Java: digitRange.add(new Range(lastNumberEnd, s.length(), TextRangeType.Text_Text));
            digitRange.push(Range::new(lastNumberEnd as i32, s.len() as i32, TextRangeType::Text_Text));
        }
        // Java: return result;
        result
    }

    // Java: static boolean calcDynamicDigitsFromText(String s, ArrayList<Range> ranges)
    pub fn calcDynamicDigitsFromText(s: &str, ranges: &mut Vec<Range>) -> bool {
        // Java: boolean result = false;
        let mut result = false;
        // Java: if (s != null && !s.isEmpty())
        if !s.is_empty()
        {
        let chars: Vec<char> = s.chars().collect();
        // Java: int i = 0;
        let mut i: usize = 0;
        // Java: while(i < s.length())
        while i < chars.len()
        {
            // Java: int start = i;
            let start = i;
            // Java: int count = 0;
            let mut count: usize = 0;
            // Java: boolean bThousandSymbol = false;
            let mut bThousandSymbol = false;
            // Java: boolean bFloat = false;
            let mut bFloat = false;

            // Java: while(i < s.length() && Character.isDigit(s.charAt(i)))
            while i < chars.len() && chars[i].is_ascii_digit()
            {
                // Java: ++i;
                i += 1;
                // Java: ++count;
                count += 1;
                // Java: if (i < s.length() && !bFloat && count <= 3 && s.charAt(i) == ',')
                if i < chars.len() && !bFloat && count <= 3 && chars[i] == ','
                {
                    // Java: boolean resultThousand;
                    let mut resultThousand = false;
                    // Java: for(resultThousand = false; i < s.length() && s.charAt(i) == ','; resultThousand = true)
                    while i < chars.len() && chars[i] == ','
                    {
                        // Java: ++i;
                        i += 1;
                        // Java: ++count;
                        count += 1;
                        // Java: int n;
                        let mut n: usize = 0;
                        // Java: for(n = 0; i < s.length() && Character.isDigit(s.charAt(i)); ++n)
                        while i < chars.len() && chars[i].is_ascii_digit()
                        {
                            // Java: ++i;
                            i += 1;
                            // Java: ++count;
                            count += 1;
                            n += 1;
                        }
                        // Java: if (n != 3)
                        if n != 3
                        {
                            // Java: i = i - n - 1;
                            i = i - n - 1;
                            // Java: count = count - n - 1;
                            count = count - n - 1;
                            // Java: break;
                            break;
                        }
                        resultThousand = true;
                    }
                    // Java: if (resultThousand)
                    if resultThousand
                    {
                        // Java: bThousandSymbol = true;
                        bThousandSymbol = true;
                    }
                }
                // Java: if (i < s.length() && s.charAt(i) == '.')
                if i < chars.len() && chars[i] == '.'
                {
                    // Java: ++i;
                    i += 1;
                    // Java: ++count;
                    count += 1;
                    // Java: if (i < s.length() && Character.isDigit(s.charAt(i)) && !bFloat)
                    if i < chars.len() && chars[i].is_ascii_digit() && !bFloat
                    {
                        // Java: bFloat = true;
                        bFloat = true;
                    }
                    // Java: else
                    else
                    {
                        // Java: --i;
                        i -= 1;
                        // Java: --count;
                        count -= 1;
                    }
                }
            }

            // Java: if (i > start && count >= 1 && count <= 20)
            if i > start && count >= 1 && count <= 20
            {
                // Java: TextRangeType type = TextRangeType.Text_Normal;
                let mut range_type = TextRangeType::Text_Normal;
                // Java: if (bFloat && bThousandSymbol)
                if bFloat && bThousandSymbol
                {
                    // Java: type = TextRangeType.Text_ThousandsFloat;
                    range_type = TextRangeType::Text_ThousandsFloat;
                }
                // Java: else if (bThousandSymbol)
                else if bThousandSymbol
                {
                    // Java: type = TextRangeType.Text_ThousandsInt;
                    range_type = TextRangeType::Text_ThousandsInt;
                }
                // Java: else if (bFloat)
                else if bFloat
                {
                    // Java: type = TextRangeType.Text_Float;
                    range_type = TextRangeType::Text_Float;
                }
                // Java: ranges.add(new Range(start, start + count, type));
                ranges.push(Range::new(start as i32, (start + count) as i32, range_type));
                // Java: result = true;
                result = true;
                // Java: ++i;
                i += 1;
            }
            // Java: else
            else
            {
                // Java: ++i;
                i += 1;
            }
        }

        // Java: return result;
        return result;
        }
        // Java: else
        else
        {
            // Java: return result;
            return result;
        }
    }

    // Java: static String addThousandsSymbol(String numStr)
    pub fn addThousandsSymbol(numStr: &str) -> String {
        // Java: StringBuilder sb = new StringBuilder();
        let mut sb = String::new();
        // Java: int dotIndex = numStr.indexOf(46);
        let dotIndex = numStr.find('.').map(|i| i as i32).unwrap_or(-1);
        // Java: int loop = numStr.length();
        let mut loop_val = numStr.len() as i32;
        // Java: if (dotIndex > 0)
        if dotIndex > 0
        {
            // Java: loop = dotIndex;
            loop_val = dotIndex;
        }
        // Java: for(int i = loop; i > 0; i -= 3)
        let mut i = loop_val;
        while i > 0
        {
            // Java: if (i > 3)
            if i > 3
            {
                // Java: sb.insert(0, "," + numStr.substring(i - 3, i));
                sb = format!(",{}{}", &numStr[(i - 3) as usize..i as usize], sb);
            }
            // Java: else
            else
            {
                // Java: sb.insert(0, numStr.substring(0, i));
                sb = format!("{}{}", &numStr[..i as usize], sb);
            }
            i -= 3;
        }
        // Java: if (dotIndex > 0)
        if dotIndex > 0
        {
            // Java: sb.append(numStr.substring(dotIndex));
            sb.push_str(&numStr[dotIndex as usize..]);
        }
        // Java: return sb.toString();
        sb
    }

    // Java: static String fillDynamicDigitWithBlank(String inputString, String origin)
    pub fn fillDynamicDigitWithBlank(inputString: &str, origin: &str) -> String {
        // Java: int length = origin.length();
        let length = origin.len();
        // Java: if (inputString.length() >= length)
        if inputString.len() >= length
        {
            // Java: return inputString;
            return inputString.to_string();
        }
        // Java: else
        else
        {
            // Java: StringBuilder sb = new StringBuilder();
            let mut sb = String::new();
            // Java: if (origin.startsWith("0"))
            if origin.starts_with('0')
            {
                // Java: while(sb.length() < length - inputString.length())
                while sb.len() < length - inputString.len()
                {
                    // Java: sb.append('0');
                    sb.push('0');
                }
            }
            // Java: else
            else
            {
                // Java: while(sb.length() < length - inputString.length())
                while sb.len() < length - inputString.len()
                {
                    // Java: sb.append(' ');
                    sb.push(' ');
                }
            }
            // Java: sb.append(inputString);
            sb.push_str(inputString);
            // Java: return sb.toString();
            return sb;
        }
    }

    // Java: static double getSigmoidFraction(double rate)
    pub fn getSigmoidFraction(rate: f64) -> f64 {
        // Java: return rate > (double)0.0F && rate < (double)1.0F ? (double)1.0F / ((double)1.0F + Math.exp((double)-18.0F * (rate - (double)0.5F))) : rate;
        if rate > 0.0 && rate < 1.0 { 1.0 / (1.0 + (-18.0 * (rate - 0.5)).exp()) } else { rate }
    }

    // Java: static double getTwoPhaseSin(double rate)
    pub fn getTwoPhaseSin(rate: f64) -> f64 {
        // Java: if (rate >= (double)0.0F && rate <= (double)1.0F)
        if rate >= 0.0 && rate <= 1.0
        {
            // Java: return rate <= (double)0.5F ? (double)0.25F * Math.sin((Math.PI * 2D) * (rate - (double)0.25F)) + (double)0.25F : (double)0.25F * Math.sin((Math.PI * 2D) * (rate - (double)0.75F)) + (double)0.75F;
            if rate <= 0.5 { 0.25 * ((std::f64::consts::PI * 2.0) * (rate - 0.25)).sin() + 0.25 } else { 0.25 * ((std::f64::consts::PI * 2.0) * (rate - 0.75)).sin() + 0.75 }
        }
        // Java: else
        else
        {
            // Java: return rate;
            rate
        }
    }

    // Java: static int dotLength(String str)
    pub fn dotLength(s: &str) -> i32 {
        // Java: int dotIndex = str.indexOf(46);
        let dotIndex = s.find('.').map(|i| i as i32).unwrap_or(-1);
        // Java: return dotIndex > 0 ? str.length() + 1 : str.length();
        if dotIndex > 0 { s.len() as i32 + 1 } else { s.len() as i32 }
    }

    // Java: static double updateFraction(String subStr, double fraction)
    pub fn updateFraction(subStr: &str, fraction: f64) -> f64 {
        // Java: String newStr = subStr.replace(",", "");
        let newStr = subStr.replace(",", "");
        // Java: try
        if let Ok(value) = newStr.parse::<f64>()
        {
            // Java: double value = Double.parseDouble(newStr);
            // Java: if (value >= (double)1000.0F)
            if value >= 1000.0
            {
                // Java: double ret = getSigmoidFraction(fraction);
                let ret = Self::getSigmoidFraction(fraction);
                // Java: if (value < (double)10000.0F && ret > 0.9995)
                if value < 10000.0 && ret > 0.9995
                {
                    // Java: return (double)1.0F;
                    return 1.0;
                }
            }
            // Java: else if (value < (double)1000.0F && dotLength(newStr) >= 3)
            else if value < 1000.0 && Self::dotLength(&newStr) >= 3
            {
                // Java: if (fraction > 0.97)
                if fraction > 0.97
                {
                    // Java: return (double)1.0F;
                    return 1.0;
                }
                // Java: return getTwoPhaseSin(fraction);
                return Self::getTwoPhaseSin(fraction);
            }
        }
        // Java: catch (NumberFormatException var8)

        // Java: if (dotLength(newStr) < 2 && fraction > 0.96)
        if Self::dotLength(&newStr) < 2 && fraction > 0.96
        {
            // Java: return (double)1.0F;
            return 1.0;
        }
        // Java: else
        else
        {
            // Java: return dotLength(newStr) < 3 && fraction > 0.96 ? (double)1.0F : fraction;
            return if Self::dotLength(&newStr) < 3 && fraction > 0.96 { 1.0 } else { fraction };
        }
    }

    // Java: static double toDoubleValue(String subStr)
    pub fn toDoubleValue(subStr: &str) -> f64 {
        // Java: String newStr = subStr.replace(",", "");
        let newStr = subStr.replace(",", "");
        // Java: double value = Double.NaN;
        let mut value = f64::NAN;
        // Java: try
        // Java: value = Double.parseDouble(newStr);
        if let Ok(v) = newStr.parse::<f64>() { value = v; }
        // Java: catch (NumberFormatException var5)
        // Java: return value;
        value
    }

    // Java: static int fractionLength(String str)
    pub fn fractionLength(s: &str) -> i32 {
        // Java: int dotIndex = str.indexOf(46);
        let dotIndex = s.find('.').map(|i| i as i32).unwrap_or(-1);
        // Java: return dotIndex > 0 ? str.length() - dotIndex : 0;
        if dotIndex > 0 { s.len() as i32 - dotIndex } else { 0 }
    }

    // Java: static String decimalToString(BigDecimal d)
    pub fn decimalToString(d: f64) -> String {
        // Java: DecimalFormat df = new DecimalFormat();
        // Java: df.setMaximumFractionDigits(19);
        // Java: df.setMinimumFractionDigits(0);
        // Java: df.setGroupingUsed(false);
        // Java: return df.format(d);
        format!("{}", d)
    }

    // Java: static String decimalToString(BigDecimal d, int fractionLen)
    pub fn decimalToString_with_len(d: f64, fractionLen: i32) -> String {
        // Java: DecimalFormat df = new DecimalFormat();
        // Java: df.setMaximumFractionDigits(fractionLen);
        // Java: df.setMinimumFractionDigits(fractionLen);
        // Java: df.setGroupingUsed(false);
        // Java: return df.format(d);
        format!("{:.1$}", d, fractionLen as usize)
    }

}
