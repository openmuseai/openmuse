// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::IOAssert;
// Java: import java.util.HashMap
// Java: import java.util.Map
use std::collections::HashMap;
use std::sync::Mutex;

// java private static final String TAG = null;
const TAG: &str = "";
// java private static Map<String, TypeSti> mMapName2TypeSti;
static mMapName2TypeSti: Mutex<Option<HashMap<String, TypeSti>>> = Mutex::new(None);

// java public final class BuildinStyleSheet
#[derive(Default)]
pub struct BuildinStyleSheet {
}

// java public static class TypeSti
#[derive(Default, Clone, Copy)]
pub struct TypeSti {
    // java public int type;
    pub r#type: i32,
    // java public int sti;
    pub sti: i32,
}

// java public final class BuildinStyleSheet
impl BuildinStyleSheet {
    // java public static boolean isReservedName(String value)
    pub fn isReservedName(value: &str) -> bool {
      // java IOAssert.assertNotNull("value should not be null", value);
      let v_opt = Some(value);
      IOAssert::assertNotNull_msg("value should not be null", &v_opt);
      // java if (null == mMapName2TypeSti)
      if mMapName2TypeSti.lock().unwrap().is_none() {
        // java initMapName2TypeSti();
        Self::initMapName2TypeSti();
      }
      // java return null != mMapName2TypeSti.get(value);
      return mMapName2TypeSti
        .lock()
        .unwrap()
        .as_ref()
        .map(|m| m.contains_key(value))
        .unwrap_or(false);
    }

    // java public static boolean isReservedIndex(int value)
    pub fn isReservedIndex(value: i32) -> bool {
      // java return 0 <= value && value <= 267;
      return 0 <= value && value <= 267;
    }

    // java public static TypeSti getTypeSti(String name)
    pub fn getTypeSti(name: &str) -> Option<TypeSti> {
      // java if (null == mMapName2TypeSti)
      if mMapName2TypeSti.lock().unwrap().is_none() {
        // java initMapName2TypeSti();
        Self::initMapName2TypeSti();
      }
      // java return (TypeSti)mMapName2TypeSti.get(name);
      return mMapName2TypeSti
        .lock()
        .unwrap()
        .as_ref()
        .and_then(|m| m.get(name).copied());
    }

    // java public static void clearMapName2TypeSti()
    pub fn clearMapName2TypeSti() {
      // java if (null != mMapName2TypeSti)
      let mut guard = mMapName2TypeSti.lock().unwrap();
      if let Some(map) = guard.as_mut() {
        // java mMapName2TypeSti.clear();
        map.clear();
        // java mMapName2TypeSti = null;
      }
      *guard = None;
    }

    // java public static String getName(int sti)
    pub fn getName(sti: i32) -> String {
      // java switch (sti)
      match sti {
        // java case 0:
        0 => {
          // java return "Normal";
          return "Normal".to_string();
        }
        // java case 1:
        1 => {
          // java return "heading 1";
          return "heading 1".to_string();
        }
        // java case 2:
        2 => {
          // java return "heading 2";
          return "heading 2".to_string();
        }
        // java case 3:
        3 => {
          // java return "heading 3";
          return "heading 3".to_string();
        }
        // java case 4:
        4 => {
          // java return "heading 4";
          return "heading 4".to_string();
        }
        // java case 5:
        5 => {
          // java return "heading 5";
          return "heading 5".to_string();
        }
        // java case 6:
        6 => {
          // java return "heading 6";
          return "heading 6".to_string();
        }
        // java case 7:
        7 => {
          // java return "heading 7";
          return "heading 7".to_string();
        }
        // java case 8:
        8 => {
          // java return "heading 8";
          return "heading 8".to_string();
        }
        // java case 9:
        9 => {
          // java return "heading 9";
          return "heading 9".to_string();
        }
        // java case 10:
        10 => {
          // java return "index 1";
          return "index 1".to_string();
        }
        // java case 11:
        11 => {
          // java return "index 2";
          return "index 2".to_string();
        }
        // java case 12:
        12 => {
          // java return "index 3";
          return "index 3".to_string();
        }
        // java case 13:
        13 => {
          // java return "index 4";
          return "index 4".to_string();
        }
        // java case 14:
        14 => {
          // java return "index 5";
          return "index 5".to_string();
        }
        // java case 15:
        15 => {
          // java return "index 6";
          return "index 6".to_string();
        }
        // java case 16:
        16 => {
          // java return "index 7";
          return "index 7".to_string();
        }
        // java case 17:
        17 => {
          // java return "index 8";
          return "index 8".to_string();
        }
        // java case 18:
        18 => {
          // java return "index 9";
          return "index 9".to_string();
        }
        // java case 19:
        19 => {
          // java return "toc 1";
          return "toc 1".to_string();
        }
        // java case 20:
        20 => {
          // java return "toc 2";
          return "toc 2".to_string();
        }
        // java case 21:
        21 => {
          // java return "toc 3";
          return "toc 3".to_string();
        }
        // java case 22:
        22 => {
          // java return "toc 4";
          return "toc 4".to_string();
        }
        // java case 23:
        23 => {
          // java return "toc 5";
          return "toc 5".to_string();
        }
        // java case 24:
        24 => {
          // java return "toc 6";
          return "toc 6".to_string();
        }
        // java case 25:
        25 => {
          // java return "toc 7";
          return "toc 7".to_string();
        }
        // java case 26:
        26 => {
          // java return "toc 8";
          return "toc 8".to_string();
        }
        // java case 27:
        27 => {
          // java return "toc 9";
          return "toc 9".to_string();
        }
        // java case 28:
        28 => {
          // java return "Normal Indent";
          return "Normal Indent".to_string();
        }
        // java case 29:
        29 => {
          // java return "footnote text";
          return "footnote text".to_string();
        }
        // java case 30:
        30 => {
          // java return "annotation text";
          return "annotation text".to_string();
        }
        // java case 31:
        31 => {
          // java return "header";
          return "header".to_string();
        }
        // java case 32:
        32 => {
          // java return "footer";
          return "footer".to_string();
        }
        // java case 33:
        33 => {
          // java return "index heading";
          return "index heading".to_string();
        }
        // java case 34:
        34 => {
          // java return "caption";
          return "caption".to_string();
        }
        // java case 35:
        35 => {
          // java return "table of figures";
          return "table of figures".to_string();
        }
        // java case 36:
        36 => {
          // java return "envelope address";
          return "envelope address".to_string();
        }
        // java case 37:
        37 => {
          // java return "envelope return";
          return "envelope return".to_string();
        }
        // java case 38:
        38 => {
          // java return "footnote reference";
          return "footnote reference".to_string();
        }
        // java case 39:
        39 => {
          // java return "annotation reference";
          return "annotation reference".to_string();
        }
        // java case 40:
        40 => {
          // java return "line number";
          return "line number".to_string();
        }
        // java case 41:
        41 => {
          // java return "page number";
          return "page number".to_string();
        }
        // java case 42:
        42 => {
          // java return "endnote reference";
          return "endnote reference".to_string();
        }
        // java case 43:
        43 => {
          // java return "endnote text";
          return "endnote text".to_string();
        }
        // java case 44:
        44 => {
          // java return "table of authorities";
          return "table of authorities".to_string();
        }
        // java case 45:
        45 => {
          // java return "macro";
          return "macro".to_string();
        }
        // java case 46:
        46 => {
          // java return "toa heading";
          return "toa heading".to_string();
        }
        // java case 47:
        47 => {
          // java return "List";
          return "List".to_string();
        }
        // java case 48:
        48 => {
          // java return "List Bullet";
          return "List Bullet".to_string();
        }
        // java case 49:
        49 => {
          // java return "List Number";
          return "List Number".to_string();
        }
        // java case 50:
        50 => {
          // java return "List 2";
          return "List 2".to_string();
        }
        // java case 51:
        51 => {
          // java return "List 3";
          return "List 3".to_string();
        }
        // java case 52:
        52 => {
          // java return "List 4";
          return "List 4".to_string();
        }
        // java case 53:
        53 => {
          // java return "List 5";
          return "List 5".to_string();
        }
        // java case 54:
        54 => {
          // java return "List Bullet 2";
          return "List Bullet 2".to_string();
        }
        // java case 55:
        55 => {
          // java return "List Bullet 3";
          return "List Bullet 3".to_string();
        }
        // java case 56:
        56 => {
          // java return "List Bullet 4";
          return "List Bullet 4".to_string();
        }
        // java case 57:
        57 => {
          // java return "List Bullet 5";
          return "List Bullet 5".to_string();
        }
        // java case 58:
        58 => {
          // java return "List Number 2";
          return "List Number 2".to_string();
        }
        // java case 59:
        59 => {
          // java return "List Number 3";
          return "List Number 3".to_string();
        }
        // java case 60:
        60 => {
          // java return "List Number 4";
          return "List Number 4".to_string();
        }
        // java case 61:
        61 => {
          // java return "List Number 5";
          return "List Number 5".to_string();
        }
        // java case 62:
        62 => {
          // java return "Title";
          return "Title".to_string();
        }
        // java case 63:
        63 => {
          // java return "Closing";
          return "Closing".to_string();
        }
        // java case 64:
        64 => {
          // java return "Signature";
          return "Signature".to_string();
        }
        // java case 65:
        65 => {
          // java return "Default Paragraph Font";
          return "Default Paragraph Font".to_string();
        }
        // java case 66:
        66 => {
          // java return "Body Text";
          return "Body Text".to_string();
        }
        // java case 67:
        67 => {
          // java return "Body Text Indent";
          return "Body Text Indent".to_string();
        }
        // java case 68:
        68 => {
          // java return "List Continue";
          return "List Continue".to_string();
        }
        // java case 69:
        69 => {
          // java return "List Continue 2";
          return "List Continue 2".to_string();
        }
        // java case 70:
        70 => {
          // java return "List Continue 3";
          return "List Continue 3".to_string();
        }
        // java case 71:
        71 => {
          // java return "List Continue 4";
          return "List Continue 4".to_string();
        }
        // java case 72:
        72 => {
          // java return "List Continue 5";
          return "List Continue 5".to_string();
        }
        // java case 73:
        73 => {
          // java return "Message Header";
          return "Message Header".to_string();
        }
        // java case 74:
        74 => {
          // java return "Subtitle";
          return "Subtitle".to_string();
        }
        // java case 75:
        75 => {
          // java return "Salutation";
          return "Salutation".to_string();
        }
        // java case 76:
        76 => {
          // java return "Date";
          return "Date".to_string();
        }
        // java case 77:
        77 => {
          // java return "Body Text First Indent";
          return "Body Text First Indent".to_string();
        }
        // java case 78:
        78 => {
          // java return "Body Text First Indent 2";
          return "Body Text First Indent 2".to_string();
        }
        // java case 79:
        79 => {
          // java return "Note Heading";
          return "Note Heading".to_string();
        }
        // java case 80:
        80 => {
          // java return "Body Text 2";
          return "Body Text 2".to_string();
        }
        // java case 81:
        81 => {
          // java return "Body Text 3";
          return "Body Text 3".to_string();
        }
        // java case 82:
        82 => {
          // java return "Body Text Indent 2";
          return "Body Text Indent 2".to_string();
        }
        // java case 83:
        83 => {
          // java return "Body Text Indent 3";
          return "Body Text Indent 3".to_string();
        }
        // java case 84:
        84 => {
          // java return "Block Text";
          return "Block Text".to_string();
        }
        // java case 85:
        85 => {
          // java return "Hyperlink";
          return "Hyperlink".to_string();
        }
        // java case 86:
        86 => {
          // java return "FollowedHyperlink";
          return "FollowedHyperlink".to_string();
        }
        // java case 87:
        87 => {
          // java return "Strong";
          return "Strong".to_string();
        }
        // java case 88:
        88 => {
          // java return "Emphasis";
          return "Emphasis".to_string();
        }
        // java case 89:
        89 => {
          // java return "Document Map";
          return "Document Map".to_string();
        }
        // java case 90:
        90 => {
          // java return "Plain Text";
          return "Plain Text".to_string();
        }
        // java case 91:
        91 => {
          // java return "E-mail Signature";
          return "E-mail Signature".to_string();
        }
        // java case 92:
        92 => {
          // java return "HTML Top of Form";
          return "HTML Top of Form".to_string();
        }
        // java case 93:
        93 => {
          // java return "HTML Bottom of Form";
          return "HTML Bottom of Form".to_string();
        }
        // java case 94:
        94 => {
          // java return "Normal (Web)";
          return "Normal (Web)".to_string();
        }
        // java case 95:
        95 => {
          // java return "HTML Acronym";
          return "HTML Acronym".to_string();
        }
        // java case 96:
        96 => {
          // java return "HTML Address";
          return "HTML Address".to_string();
        }
        // java case 97:
        97 => {
          // java return "HTML Cite";
          return "HTML Cite".to_string();
        }
        // java case 98:
        98 => {
          // java return "HTML Code";
          return "HTML Code".to_string();
        }
        // java case 99:
        99 => {
          // java return "HTML Definition";
          return "HTML Definition".to_string();
        }
        // java case 100:
        100 => {
          // java return "HTML Keyboard";
          return "HTML Keyboard".to_string();
        }
        // java case 101:
        101 => {
          // java return "HTML Preformatted";
          return "HTML Preformatted".to_string();
        }
        // java case 102:
        102 => {
          // java return "HTML Sample";
          return "HTML Sample".to_string();
        }
        // java case 103:
        103 => {
          // java return "HTML Typewriter";
          return "HTML Typewriter".to_string();
        }
        // java case 104:
        104 => {
          // java return "HTML Variable";
          return "HTML Variable".to_string();
        }
        // java case 105:
        105 => {
          // java return "Normal Table";
          return "Normal Table".to_string();
        }
        // java case 106:
        106 => {
          // java return "annotation subject";
          return "annotation subject".to_string();
        }
        // java case 107:
        107 => {
          // java return "No List";
          return "No List".to_string();
        }
        // java case 108:
        108 => {
          // java return "Outline List 1";
          return "Outline List 1".to_string();
        }
        // java case 109:
        109 => {
          // java return "Outline List 2";
          return "Outline List 2".to_string();
        }
        // java case 110:
        110 => {
          // java return "Outline List 3";
          return "Outline List 3".to_string();
        }
        // java case 111:
        111 => {
          // java return "Table Simple 1";
          return "Table Simple 1".to_string();
        }
        // java case 112:
        112 => {
          // java return "Table Simple 2";
          return "Table Simple 2".to_string();
        }
        // java case 113:
        113 => {
          // java return "Table Simple 3";
          return "Table Simple 3".to_string();
        }
        // java case 114:
        114 => {
          // java return "Table Classic 1";
          return "Table Classic 1".to_string();
        }
        // java case 115:
        115 => {
          // java return "Table Classic 2";
          return "Table Classic 2".to_string();
        }
        // java case 116:
        116 => {
          // java return "Table Classic 3";
          return "Table Classic 3".to_string();
        }
        // java case 117:
        117 => {
          // java return "Table Classic 4";
          return "Table Classic 4".to_string();
        }
        // java case 118:
        118 => {
          // java return "Table Colorful 1";
          return "Table Colorful 1".to_string();
        }
        // java case 119:
        119 => {
          // java return "Table Colorful 2";
          return "Table Colorful 2".to_string();
        }
        // java case 120:
        120 => {
          // java return "Table Colorful 3";
          return "Table Colorful 3".to_string();
        }
        // java case 121:
        121 => {
          // java return "Table Columns 1";
          return "Table Columns 1".to_string();
        }
        // java case 122:
        122 => {
          // java return "Table Columns 2";
          return "Table Columns 2".to_string();
        }
        // java case 123:
        123 => {
          // java return "Table Columns 3";
          return "Table Columns 3".to_string();
        }
        // java case 124:
        124 => {
          // java return "Table Columns 4";
          return "Table Columns 4".to_string();
        }
        // java case 125:
        125 => {
          // java return "Table Columns 5";
          return "Table Columns 5".to_string();
        }
        // java case 126:
        126 => {
          // java return "Table Grid 1";
          return "Table Grid 1".to_string();
        }
        // java case 127:
        127 => {
          // java return "Table Grid 2";
          return "Table Grid 2".to_string();
        }
        // java case 128:
        128 => {
          // java return "Table Grid 3";
          return "Table Grid 3".to_string();
        }
        // java case 129:
        129 => {
          // java return "Table Grid 4";
          return "Table Grid 4".to_string();
        }
        // java case 130:
        130 => {
          // java return "Table Grid 5";
          return "Table Grid 5".to_string();
        }
        // java case 131:
        131 => {
          // java return "Table Grid 6";
          return "Table Grid 6".to_string();
        }
        // java case 132:
        132 => {
          // java return "Table Grid 7";
          return "Table Grid 7".to_string();
        }
        // java case 133:
        133 => {
          // java return "Table Grid 8";
          return "Table Grid 8".to_string();
        }
        // java case 134:
        134 => {
          // java return "Table List 1";
          return "Table List 1".to_string();
        }
        // java case 135:
        135 => {
          // java return "Table List 2";
          return "Table List 2".to_string();
        }
        // java case 136:
        136 => {
          // java return "Table List 3";
          return "Table List 3".to_string();
        }
        // java case 137:
        137 => {
          // java return "Table List 4";
          return "Table List 4".to_string();
        }
        // java case 138:
        138 => {
          // java return "Table List 5";
          return "Table List 5".to_string();
        }
        // java case 139:
        139 => {
          // java return "Table List 6";
          return "Table List 6".to_string();
        }
        // java case 140:
        140 => {
          // java return "Table List 7";
          return "Table List 7".to_string();
        }
        // java case 141:
        141 => {
          // java return "Table List 8";
          return "Table List 8".to_string();
        }
        // java case 142:
        142 => {
          // java return "Table 3D effects 1";
          return "Table 3D effects 1".to_string();
        }
        // java case 143:
        143 => {
          // java return "Table 3D effects 2";
          return "Table 3D effects 2".to_string();
        }
        // java case 144:
        144 => {
          // java return "Table 3D effects 3";
          return "Table 3D effects 3".to_string();
        }
        // java case 145:
        145 => {
          // java return "Table Contemporary";
          return "Table Contemporary".to_string();
        }
        // java case 146:
        146 => {
          // java return "Table Elegant";
          return "Table Elegant".to_string();
        }
        // java case 147:
        147 => {
          // java return "Table Professional";
          return "Table Professional".to_string();
        }
        // java case 148:
        148 => {
          // java return "Table Subtle 1";
          return "Table Subtle 1".to_string();
        }
        // java case 149:
        149 => {
          // java return "Table Subtle 2";
          return "Table Subtle 2".to_string();
        }
        // java case 150:
        150 => {
          // java return "Table Web 1";
          return "Table Web 1".to_string();
        }
        // java case 151:
        151 => {
          // java return "Table Web 2";
          return "Table Web 2".to_string();
        }
        // java case 152:
        152 => {
          // java return "Table Web 3";
          return "Table Web 3".to_string();
        }
        // java case 153:
        153 => {
          // java return "Balloon Text";
          return "Balloon Text".to_string();
        }
        // java case 154:
        154 => {
          // java return "Table Grid";
          return "Table Grid".to_string();
        }
        // java case 155:
        155 => {
          // java return "Table Theme";
          return "Table Theme".to_string();
        }
        // java case 156:
        156 => {
          // java return "Placeholder Text";
          return "Placeholder Text".to_string();
        }
        // java case 157:
        157 => {
          // java return "No Spacing";
          return "No Spacing".to_string();
        }
        // java case 158:
        158 => {
          // java return "Light Shading";
          return "Light Shading".to_string();
        }
        // java case 159:
        159 => {
          // java return "Light List";
          return "Light List".to_string();
        }
        // java case 160:
        160 => {
          // java return "Light Grid";
          return "Light Grid".to_string();
        }
        // java case 161:
        161 => {
          // java return "Medium Shading 1";
          return "Medium Shading 1".to_string();
        }
        // java case 162:
        162 => {
          // java return "Medium Shading 2";
          return "Medium Shading 2".to_string();
        }
        // java case 163:
        163 => {
          // java return "Medium List 1";
          return "Medium List 1".to_string();
        }
        // java case 164:
        164 => {
          // java return "Medium List 2";
          return "Medium List 2".to_string();
        }
        // java case 165:
        165 => {
          // java return "Medium Grid 1";
          return "Medium Grid 1".to_string();
        }
        // java case 166:
        166 => {
          // java return "Medium Grid 2";
          return "Medium Grid 2".to_string();
        }
        // java case 167:
        167 => {
          // java return "Medium Grid 3";
          return "Medium Grid 3".to_string();
        }
        // java case 168:
        168 => {
          // java return "Dark List";
          return "Dark List".to_string();
        }
        // java case 169:
        169 => {
          // java return "Colorful Shading";
          return "Colorful Shading".to_string();
        }
        // java case 170:
        170 => {
          // java return "Colorful List";
          return "Colorful List".to_string();
        }
        // java case 171:
        171 => {
          // java return "Colorful Grid";
          return "Colorful Grid".to_string();
        }
        // java case 172:
        172 => {
          // java return "Light Shading Accent 1";
          return "Light Shading Accent 1".to_string();
        }
        // java case 173:
        173 => {
          // java return "Light List Accent 1";
          return "Light List Accent 1".to_string();
        }
        // java case 174:
        174 => {
          // java return "Light Grid Accent 1";
          return "Light Grid Accent 1".to_string();
        }
        // java case 175:
        175 => {
          // java return "Medium Shading 1 Accent 1";
          return "Medium Shading 1 Accent 1".to_string();
        }
        // java case 176:
        176 => {
          // java return "Medium Shading 2 Accent 1";
          return "Medium Shading 2 Accent 1".to_string();
        }
        // java case 177:
        177 => {
          // java return "Medium List 1 Accent 1";
          return "Medium List 1 Accent 1".to_string();
        }
        // java case 178:
        178 => {
          // java return "Revision";
          return "Revision".to_string();
        }
        // java case 179:
        179 => {
          // java return "List Paragraph";
          return "List Paragraph".to_string();
        }
        // java case 180:
        180 => {
          // java return "Quote";
          return "Quote".to_string();
        }
        // java case 181:
        181 => {
          // java return "Intense Quote";
          return "Intense Quote".to_string();
        }
        // java case 182:
        182 => {
          // java return "Medium List 2 Accent 1";
          return "Medium List 2 Accent 1".to_string();
        }
        // java case 183:
        183 => {
          // java return "Medium Grid 1 Accent 1";
          return "Medium Grid 1 Accent 1".to_string();
        }
        // java case 184:
        184 => {
          // java return "Medium Grid 2 Accent 1";
          return "Medium Grid 2 Accent 1".to_string();
        }
        // java case 185:
        185 => {
          // java return "Medium Grid 3 Accent 1";
          return "Medium Grid 3 Accent 1".to_string();
        }
        // java case 186:
        186 => {
          // java return "Dark List Accent 1";
          return "Dark List Accent 1".to_string();
        }
        // java case 187:
        187 => {
          // java return "Colorful Shading Accent 1";
          return "Colorful Shading Accent 1".to_string();
        }
        // java case 188:
        188 => {
          // java return "Colorful List Accent 1";
          return "Colorful List Accent 1".to_string();
        }
        // java case 189:
        189 => {
          // java return "Colorful Grid Accent 1";
          return "Colorful Grid Accent 1".to_string();
        }
        // java case 190:
        190 => {
          // java return "Light Shading Accent 2";
          return "Light Shading Accent 2".to_string();
        }
        // java case 191:
        191 => {
          // java return "Light List Accent 2";
          return "Light List Accent 2".to_string();
        }
        // java case 192:
        192 => {
          // java return "Light Grid Accent 2";
          return "Light Grid Accent 2".to_string();
        }
        // java case 193:
        193 => {
          // java return "Medium Shading 1 Accent 2";
          return "Medium Shading 1 Accent 2".to_string();
        }
        // java case 194:
        194 => {
          // java return "Medium Shading 2 Accent 2";
          return "Medium Shading 2 Accent 2".to_string();
        }
        // java case 195:
        195 => {
          // java return "Medium List 1 Accent 2";
          return "Medium List 1 Accent 2".to_string();
        }
        // java case 196:
        196 => {
          // java return "Medium List 2 Accent 2";
          return "Medium List 2 Accent 2".to_string();
        }
        // java case 197:
        197 => {
          // java return "Medium Grid 1 Accent 2";
          return "Medium Grid 1 Accent 2".to_string();
        }
        // java case 198:
        198 => {
          // java return "Medium Grid 2 Accent 2";
          return "Medium Grid 2 Accent 2".to_string();
        }
        // java case 199:
        199 => {
          // java return "Medium Grid 3 Accent 2";
          return "Medium Grid 3 Accent 2".to_string();
        }
        // java case 200:
        200 => {
          // java return "Dark List Accent 2";
          return "Dark List Accent 2".to_string();
        }
        // java case 201:
        201 => {
          // java return "Colorful Shading Accent 2";
          return "Colorful Shading Accent 2".to_string();
        }
        // java case 202:
        202 => {
          // java return "Colorful List Accent 2";
          return "Colorful List Accent 2".to_string();
        }
        // java case 203:
        203 => {
          // java return "Colorful Grid Accent 2";
          return "Colorful Grid Accent 2".to_string();
        }
        // java case 204:
        204 => {
          // java return "Light Shading Accent 3";
          return "Light Shading Accent 3".to_string();
        }
        // java case 205:
        205 => {
          // java return "Light List Accent 3";
          return "Light List Accent 3".to_string();
        }
        // java case 206:
        206 => {
          // java return "Light Grid Accent 3";
          return "Light Grid Accent 3".to_string();
        }
        // java case 207:
        207 => {
          // java return "Medium Shading 1 Accent 3";
          return "Medium Shading 1 Accent 3".to_string();
        }
        // java case 208:
        208 => {
          // java return "Medium Shading 2 Accent 3";
          return "Medium Shading 2 Accent 3".to_string();
        }
        // java case 209:
        209 => {
          // java return "Medium List 1 Accent 3";
          return "Medium List 1 Accent 3".to_string();
        }
        // java case 210:
        210 => {
          // java return "Medium List 2 Accent 3";
          return "Medium List 2 Accent 3".to_string();
        }
        // java case 211:
        211 => {
          // java return "Medium Grid 1 Accent 3";
          return "Medium Grid 1 Accent 3".to_string();
        }
        // java case 212:
        212 => {
          // java return "Medium Grid 2 Accent 3";
          return "Medium Grid 2 Accent 3".to_string();
        }
        // java case 213:
        213 => {
          // java return "Medium Grid 3 Accent 3";
          return "Medium Grid 3 Accent 3".to_string();
        }
        // java case 214:
        214 => {
          // java return "Dark List Accent 3";
          return "Dark List Accent 3".to_string();
        }
        // java case 215:
        215 => {
          // java return "Colorful Shading Accent 3";
          return "Colorful Shading Accent 3".to_string();
        }
        // java case 216:
        216 => {
          // java return "Colorful List Accent 3";
          return "Colorful List Accent 3".to_string();
        }
        // java case 217:
        217 => {
          // java return "Colorful Grid Accent 3";
          return "Colorful Grid Accent 3".to_string();
        }
        // java case 218:
        218 => {
          // java return "Light Shading Accent 4";
          return "Light Shading Accent 4".to_string();
        }
        // java case 219:
        219 => {
          // java return "Light List Accent 4";
          return "Light List Accent 4".to_string();
        }
        // java case 220:
        220 => {
          // java return "Light Grid Accent 4";
          return "Light Grid Accent 4".to_string();
        }
        // java case 221:
        221 => {
          // java return "Medium Shading 1 Accent 4";
          return "Medium Shading 1 Accent 4".to_string();
        }
        // java case 222:
        222 => {
          // java return "Medium Shading 2 Accent 4";
          return "Medium Shading 2 Accent 4".to_string();
        }
        // java case 223:
        223 => {
          // java return "Medium List 1 Accent 4";
          return "Medium List 1 Accent 4".to_string();
        }
        // java case 224:
        224 => {
          // java return "Medium List 2 Accent 4";
          return "Medium List 2 Accent 4".to_string();
        }
        // java case 225:
        225 => {
          // java return "Medium Grid 1 Accent 4";
          return "Medium Grid 1 Accent 4".to_string();
        }
        // java case 226:
        226 => {
          // java return "Medium Grid 2 Accent 4";
          return "Medium Grid 2 Accent 4".to_string();
        }
        // java case 227:
        227 => {
          // java return "Medium Grid 3 Accent 4";
          return "Medium Grid 3 Accent 4".to_string();
        }
        // java case 228:
        228 => {
          // java return "Dark List Accent 4";
          return "Dark List Accent 4".to_string();
        }
        // java case 229:
        229 => {
          // java return "Colorful Shading Accent 4";
          return "Colorful Shading Accent 4".to_string();
        }
        // java case 230:
        230 => {
          // java return "Colorful List Accent 4";
          return "Colorful List Accent 4".to_string();
        }
        // java case 231:
        231 => {
          // java return "Colorful Grid Accent 4";
          return "Colorful Grid Accent 4".to_string();
        }
        // java case 232:
        232 => {
          // java return "Light Shading Accent 5";
          return "Light Shading Accent 5".to_string();
        }
        // java case 233:
        233 => {
          // java return "Light List Accent 5";
          return "Light List Accent 5".to_string();
        }
        // java case 234:
        234 => {
          // java return "Light Grid Accent 5";
          return "Light Grid Accent 5".to_string();
        }
        // java case 235:
        235 => {
          // java return "Medium Shading 1 Accent 5";
          return "Medium Shading 1 Accent 5".to_string();
        }
        // java case 236:
        236 => {
          // java return "Medium Shading 2 Accent 5";
          return "Medium Shading 2 Accent 5".to_string();
        }
        // java case 237:
        237 => {
          // java return "Medium List 1 Accent 5";
          return "Medium List 1 Accent 5".to_string();
        }
        // java case 238:
        238 => {
          // java return "Medium List 2 Accent 5";
          return "Medium List 2 Accent 5".to_string();
        }
        // java case 239:
        239 => {
          // java return "Medium Grid 1 Accent 5";
          return "Medium Grid 1 Accent 5".to_string();
        }
        // java case 240:
        240 => {
          // java return "Medium Grid 2 Accent 5";
          return "Medium Grid 2 Accent 5".to_string();
        }
        // java case 241:
        241 => {
          // java return "Medium Grid 3 Accent 5";
          return "Medium Grid 3 Accent 5".to_string();
        }
        // java case 242:
        242 => {
          // java return "Dark List Accent 5";
          return "Dark List Accent 5".to_string();
        }
        // java case 243:
        243 => {
          // java return "Colorful Shading Accent 5";
          return "Colorful Shading Accent 5".to_string();
        }
        // java case 244:
        244 => {
          // java return "Colorful List Accent 5";
          return "Colorful List Accent 5".to_string();
        }
        // java case 245:
        245 => {
          // java return "Colorful Grid Accent 5";
          return "Colorful Grid Accent 5".to_string();
        }
        // java case 246:
        246 => {
          // java return "Light Shading Accent 6";
          return "Light Shading Accent 6".to_string();
        }
        // java case 247:
        247 => {
          // java return "Light List Accent 6";
          return "Light List Accent 6".to_string();
        }
        // java case 248:
        248 => {
          // java return "Light Grid Accent 6";
          return "Light Grid Accent 6".to_string();
        }
        // java case 249:
        249 => {
          // java return "Medium Shading 1 Accent 6";
          return "Medium Shading 1 Accent 6".to_string();
        }
        // java case 250:
        250 => {
          // java return "Medium Shading 2 Accent 6";
          return "Medium Shading 2 Accent 6".to_string();
        }
        // java case 251:
        251 => {
          // java return "Medium List 1 Accent 6";
          return "Medium List 1 Accent 6".to_string();
        }
        // java case 252:
        252 => {
          // java return "Medium List 2 Accent 6";
          return "Medium List 2 Accent 6".to_string();
        }
        // java case 253:
        253 => {
          // java return "Medium Grid 1 Accent 6";
          return "Medium Grid 1 Accent 6".to_string();
        }
        // java case 254:
        254 => {
          // java return "Medium Grid 2 Accent 6";
          return "Medium Grid 2 Accent 6".to_string();
        }
        // java case 255:
        255 => {
          // java return "Medium Grid 3 Accent 6";
          return "Medium Grid 3 Accent 6".to_string();
        }
        // java case 256:
        256 => {
          // java return "Dark List Accent 6";
          return "Dark List Accent 6".to_string();
        }
        // java case 257:
        257 => {
          // java return "Colorful Shading Accent 6";
          return "Colorful Shading Accent 6".to_string();
        }
        // java case 258:
        258 => {
          // java return "Colorful List Accent 6";
          return "Colorful List Accent 6".to_string();
        }
        // java case 259:
        259 => {
          // java return "Colorful Grid Accent 6";
          return "Colorful Grid Accent 6".to_string();
        }
        // java case 260:
        260 => {
          // java return "Subtle Emphasis";
          return "Subtle Emphasis".to_string();
        }
        // java case 261:
        261 => {
          // java return "Intense Emphasis";
          return "Intense Emphasis".to_string();
        }
        // java case 262:
        262 => {
          // java return "Subtle Reference";
          return "Subtle Reference".to_string();
        }
        // java case 263:
        263 => {
          // java return "Intense Reference";
          return "Intense Reference".to_string();
        }
        // java case 264:
        264 => {
          // java return "Book Title";
          return "Book Title".to_string();
        }
        // java case 265:
        265 => {
          // java return "Bibliography";
          return "Bibliography".to_string();
        }
        // java case 266:
        266 => {
          // java return "TOC Heading";
          return "TOC Heading".to_string();
        }
        // java case 12289:
        12289 => {
          // java return "Heading 1 Char";
          return "Heading 1 Char".to_string();
        }
        // java case 12290:
        12290 => {
          // java return "Heading 2 Char";
          return "Heading 2 Char".to_string();
        }
        // java case 12291:
        12291 => {
          // java return "Heading 3 Char";
          return "Heading 3 Char".to_string();
        }
        // java case 12292:
        12292 => {
          // java return "Heading 4 Char";
          return "Heading 4 Char".to_string();
        }
        // java case 12293:
        12293 => {
          // java return "Heading 5 Char";
          return "Heading 5 Char".to_string();
        }
        // java case 12294:
        12294 => {
          // java return "Heading 6 Char";
          return "Heading 6 Char".to_string();
        }
        // java case 12295:
        12295 => {
          // java return "Heading 7 Char";
          return "Heading 7 Char".to_string();
        }
        // java case 12296:
        12296 => {
          // java return "Heading 8 Char";
          return "Heading 8 Char".to_string();
        }
        // java case 12297:
        12297 => {
          // java return "Heading 9 Char";
          return "Heading 9 Char".to_string();
        }
        // java case 12319:
        12319 => {
          // java return "Header Char";
          return "Header Char".to_string();
        }
        // java case 12320:
        12320 => {
          // java return "Footer Char";
          return "Footer Char".to_string();
        }
        // java case 12350:
        12350 => {
          // java return "Title Char";
          return "Title Char".to_string();
        }
        // java case 12362:
        12362 => {
          // java return "SubtitleChar";
          return "SubtitleChar".to_string();
        }
        // java case 12468:
        12468 => {
          // java return "Quote Char";
          return "Quote Char".to_string();
        }
        // java case 12469:
        12469 => {
          // java return "Intense Quote Char";
          return "Intense Quote Char".to_string();
        }
        // java default:
        _ => {
          // java return null;
          return Default::default();
        }
      }
    }

    // java public static int getType(int sti)
    pub fn getType(sti: i32) -> i32 {
      // java switch (sti)
      match sti {
        // java case 0:
        0 => {
          // java return 1;
          return 1;
        }
        // java case 1:
        1 => {
          // java return 1;
          return 1;
        }
        // java case 2:
        2 => {
          // java return 1;
          return 1;
        }
        // java case 3:
        3 => {
          // java return 1;
          return 1;
        }
        // java case 4:
        4 => {
          // java return 1;
          return 1;
        }
        // java case 5:
        5 => {
          // java return 1;
          return 1;
        }
        // java case 6:
        6 => {
          // java return 1;
          return 1;
        }
        // java case 7:
        7 => {
          // java return 1;
          return 1;
        }
        // java case 8:
        8 => {
          // java return 1;
          return 1;
        }
        // java case 9:
        9 => {
          // java return 1;
          return 1;
        }
        // java case 10:
        10 => {
          // java return 1;
          return 1;
        }
        // java case 11:
        11 => {
          // java return 1;
          return 1;
        }
        // java case 12:
        12 => {
          // java return 1;
          return 1;
        }
        // java case 13:
        13 => {
          // java return 1;
          return 1;
        }
        // java case 14:
        14 => {
          // java return 1;
          return 1;
        }
        // java case 15:
        15 => {
          // java return 1;
          return 1;
        }
        // java case 16:
        16 => {
          // java return 1;
          return 1;
        }
        // java case 17:
        17 => {
          // java return 1;
          return 1;
        }
        // java case 18:
        18 => {
          // java return 1;
          return 1;
        }
        // java case 19:
        19 => {
          // java return 1;
          return 1;
        }
        // java case 20:
        20 => {
          // java return 1;
          return 1;
        }
        // java case 21:
        21 => {
          // java return 1;
          return 1;
        }
        // java case 22:
        22 => {
          // java return 1;
          return 1;
        }
        // java case 23:
        23 => {
          // java return 1;
          return 1;
        }
        // java case 24:
        24 => {
          // java return 1;
          return 1;
        }
        // java case 25:
        25 => {
          // java return 1;
          return 1;
        }
        // java case 26:
        26 => {
          // java return 1;
          return 1;
        }
        // java case 27:
        27 => {
          // java return 1;
          return 1;
        }
        // java case 28:
        28 => {
          // java return 1;
          return 1;
        }
        // java case 29:
        29 => {
          // java return 1;
          return 1;
        }
        // java case 30:
        30 => {
          // java return 1;
          return 1;
        }
        // java case 31:
        31 => {
          // java return 1;
          return 1;
        }
        // java case 32:
        32 => {
          // java return 1;
          return 1;
        }
        // java case 33:
        33 => {
          // java return 1;
          return 1;
        }
        // java case 34:
        34 => {
          // java return 1;
          return 1;
        }
        // java case 35:
        35 => {
          // java return 1;
          return 1;
        }
        // java case 36:
        36 => {
          // java return 1;
          return 1;
        }
        // java case 37:
        37 => {
          // java return 1;
          return 1;
        }
        // java case 38:
        38 => {
          // java return 2;
          return 2;
        }
        // java case 39:
        39 => {
          // java return 2;
          return 2;
        }
        // java case 40:
        40 => {
          // java return 2;
          return 2;
        }
        // java case 41:
        41 => {
          // java return 2;
          return 2;
        }
        // java case 42:
        42 => {
          // java return 2;
          return 2;
        }
        // java case 43:
        43 => {
          // java return 1;
          return 1;
        }
        // java case 44:
        44 => {
          // java return 1;
          return 1;
        }
        // java case 45:
        45 => {
          // java return 1;
          return 1;
        }
        // java case 46:
        46 => {
          // java return 1;
          return 1;
        }
        // java case 47:
        47 => {
          // java return 1;
          return 1;
        }
        // java case 48:
        48 => {
          // java return 1;
          return 1;
        }
        // java case 49:
        49 => {
          // java return 1;
          return 1;
        }
        // java case 50:
        50 => {
          // java return 1;
          return 1;
        }
        // java case 51:
        51 => {
          // java return 1;
          return 1;
        }
        // java case 52:
        52 => {
          // java return 1;
          return 1;
        }
        // java case 53:
        53 => {
          // java return 1;
          return 1;
        }
        // java case 54:
        54 => {
          // java return 1;
          return 1;
        }
        // java case 55:
        55 => {
          // java return 1;
          return 1;
        }
        // java case 56:
        56 => {
          // java return 1;
          return 1;
        }
        // java case 57:
        57 => {
          // java return 1;
          return 1;
        }
        // java case 58:
        58 => {
          // java return 1;
          return 1;
        }
        // java case 59:
        59 => {
          // java return 1;
          return 1;
        }
        // java case 60:
        60 => {
          // java return 1;
          return 1;
        }
        // java case 61:
        61 => {
          // java return 1;
          return 1;
        }
        // java case 62:
        62 => {
          // java return 1;
          return 1;
        }
        // java case 63:
        63 => {
          // java return 1;
          return 1;
        }
        // java case 64:
        64 => {
          // java return 1;
          return 1;
        }
        // java case 65:
        65 => {
          // java return 2;
          return 2;
        }
        // java case 66:
        66 => {
          // java return 1;
          return 1;
        }
        // java case 67:
        67 => {
          // java return 1;
          return 1;
        }
        // java case 68:
        68 => {
          // java return 1;
          return 1;
        }
        // java case 69:
        69 => {
          // java return 1;
          return 1;
        }
        // java case 70:
        70 => {
          // java return 1;
          return 1;
        }
        // java case 71:
        71 => {
          // java return 1;
          return 1;
        }
        // java case 72:
        72 => {
          // java return 1;
          return 1;
        }
        // java case 73:
        73 => {
          // java return 1;
          return 1;
        }
        // java case 74:
        74 => {
          // java return 1;
          return 1;
        }
        // java case 75:
        75 => {
          // java return 1;
          return 1;
        }
        // java case 76:
        76 => {
          // java return 1;
          return 1;
        }
        // java case 77:
        77 => {
          // java return 1;
          return 1;
        }
        // java case 78:
        78 => {
          // java return 1;
          return 1;
        }
        // java case 79:
        79 => {
          // java return 1;
          return 1;
        }
        // java case 80:
        80 => {
          // java return 1;
          return 1;
        }
        // java case 81:
        81 => {
          // java return 1;
          return 1;
        }
        // java case 82:
        82 => {
          // java return 1;
          return 1;
        }
        // java case 83:
        83 => {
          // java return 1;
          return 1;
        }
        // java case 84:
        84 => {
          // java return 1;
          return 1;
        }
        // java case 85:
        85 => {
          // java return 2;
          return 2;
        }
        // java case 86:
        86 => {
          // java return 2;
          return 2;
        }
        // java case 87:
        87 => {
          // java return 2;
          return 2;
        }
        // java case 88:
        88 => {
          // java return 2;
          return 2;
        }
        // java case 89:
        89 => {
          // java return 1;
          return 1;
        }
        // java case 90:
        90 => {
          // java return 1;
          return 1;
        }
        // java case 91:
        91 => {
          // java return 1;
          return 1;
        }
        // java case 92:
        92 => {
          // java return 1;
          return 1;
        }
        // java case 93:
        93 => {
          // java return 1;
          return 1;
        }
        // java case 94:
        94 => {
          // java return 1;
          return 1;
        }
        // java case 95:
        95 => {
          // java return 2;
          return 2;
        }
        // java case 96:
        96 => {
          // java return 1;
          return 1;
        }
        // java case 97:
        97 => {
          // java return 2;
          return 2;
        }
        // java case 98:
        98 => {
          // java return 2;
          return 2;
        }
        // java case 99:
        99 => {
          // java return 2;
          return 2;
        }
        // java case 100:
        100 => {
          // java return 2;
          return 2;
        }
        // java case 101:
        101 => {
          // java return 1;
          return 1;
        }
        // java case 102:
        102 => {
          // java return 2;
          return 2;
        }
        // java case 103:
        103 => {
          // java return 2;
          return 2;
        }
        // java case 104:
        104 => {
          // java return 2;
          return 2;
        }
        // java case 105:
        105 => {
          // java return 3;
          return 3;
        }
        // java case 106:
        106 => {
          // java return 1;
          return 1;
        }
        // java case 107:
        107 => {
          // java return 4;
          return 4;
        }
        // java case 108:
        108 => {
          // java return 4;
          return 4;
        }
        // java case 109:
        109 => {
          // java return 4;
          return 4;
        }
        // java case 110:
        110 => {
          // java return 4;
          return 4;
        }
        // java case 111:
        111 => {
          // java return 3;
          return 3;
        }
        // java case 112:
        112 => {
          // java return 3;
          return 3;
        }
        // java case 113:
        113 => {
          // java return 3;
          return 3;
        }
        // java case 114:
        114 => {
          // java return 3;
          return 3;
        }
        // java case 115:
        115 => {
          // java return 3;
          return 3;
        }
        // java case 116:
        116 => {
          // java return 3;
          return 3;
        }
        // java case 117:
        117 => {
          // java return 3;
          return 3;
        }
        // java case 118:
        118 => {
          // java return 3;
          return 3;
        }
        // java case 119:
        119 => {
          // java return 3;
          return 3;
        }
        // java case 120:
        120 => {
          // java return 3;
          return 3;
        }
        // java case 121:
        121 => {
          // java return 3;
          return 3;
        }
        // java case 122:
        122 => {
          // java return 3;
          return 3;
        }
        // java case 123:
        123 => {
          // java return 3;
          return 3;
        }
        // java case 124:
        124 => {
          // java return 3;
          return 3;
        }
        // java case 125:
        125 => {
          // java return 3;
          return 3;
        }
        // java case 126:
        126 => {
          // java return 3;
          return 3;
        }
        // java case 127:
        127 => {
          // java return 3;
          return 3;
        }
        // java case 128:
        128 => {
          // java return 3;
          return 3;
        }
        // java case 129:
        129 => {
          // java return 3;
          return 3;
        }
        // java case 130:
        130 => {
          // java return 3;
          return 3;
        }
        // java case 131:
        131 => {
          // java return 3;
          return 3;
        }
        // java case 132:
        132 => {
          // java return 3;
          return 3;
        }
        // java case 133:
        133 => {
          // java return 3;
          return 3;
        }
        // java case 134:
        134 => {
          // java return 3;
          return 3;
        }
        // java case 135:
        135 => {
          // java return 3;
          return 3;
        }
        // java case 136:
        136 => {
          // java return 3;
          return 3;
        }
        // java case 137:
        137 => {
          // java return 3;
          return 3;
        }
        // java case 138:
        138 => {
          // java return 3;
          return 3;
        }
        // java case 139:
        139 => {
          // java return 3;
          return 3;
        }
        // java case 140:
        140 => {
          // java return 3;
          return 3;
        }
        // java case 141:
        141 => {
          // java return 3;
          return 3;
        }
        // java case 142:
        142 => {
          // java return 3;
          return 3;
        }
        // java case 143:
        143 => {
          // java return 3;
          return 3;
        }
        // java case 144:
        144 => {
          // java return 3;
          return 3;
        }
        // java case 145:
        145 => {
          // java return 3;
          return 3;
        }
        // java case 146:
        146 => {
          // java return 3;
          return 3;
        }
        // java case 147:
        147 => {
          // java return 3;
          return 3;
        }
        // java case 148:
        148 => {
          // java return 3;
          return 3;
        }
        // java case 149:
        149 => {
          // java return 3;
          return 3;
        }
        // java case 150:
        150 => {
          // java return 3;
          return 3;
        }
        // java case 151:
        151 => {
          // java return 3;
          return 3;
        }
        // java case 152:
        152 => {
          // java return 3;
          return 3;
        }
        // java case 153:
        153 => {
          // java return 1;
          return 1;
        }
        // java case 154:
        154 => {
          // java return 3;
          return 3;
        }
        // java case 155:
        155 => {
          // java return 3;
          return 3;
        }
        // java case 156:
        156 => {
          // java return 2;
          return 2;
        }
        // java case 157:
        157 => {
          // java return 1;
          return 1;
        }
        // java case 158:
        158 => {
          // java return 3;
          return 3;
        }
        // java case 159:
        159 => {
          // java return 3;
          return 3;
        }
        // java case 160:
        160 => {
          // java return 3;
          return 3;
        }
        // java case 161:
        161 => {
          // java return 3;
          return 3;
        }
        // java case 162:
        162 => {
          // java return 3;
          return 3;
        }
        // java case 163:
        163 => {
          // java return 3;
          return 3;
        }
        // java case 164:
        164 => {
          // java return 3;
          return 3;
        }
        // java case 165:
        165 => {
          // java return 3;
          return 3;
        }
        // java case 166:
        166 => {
          // java return 3;
          return 3;
        }
        // java case 167:
        167 => {
          // java return 3;
          return 3;
        }
        // java case 168:
        168 => {
          // java return 3;
          return 3;
        }
        // java case 169:
        169 => {
          // java return 3;
          return 3;
        }
        // java case 170:
        170 => {
          // java return 3;
          return 3;
        }
        // java case 171:
        171 => {
          // java return 3;
          return 3;
        }
        // java case 172:
        172 => {
          // java return 3;
          return 3;
        }
        // java case 173:
        173 => {
          // java return 3;
          return 3;
        }
        // java case 174:
        174 => {
          // java return 3;
          return 3;
        }
        // java case 175:
        175 => {
          // java return 3;
          return 3;
        }
        // java case 176:
        176 => {
          // java return 3;
          return 3;
        }
        // java case 177:
        177 => {
          // java return 3;
          return 3;
        }
        // java case 178:
        178 => {
          // java return 1;
          return 1;
        }
        // java case 179:
        179 => {
          // java return 1;
          return 1;
        }
        // java case 180:
        180 => {
          // java return 1;
          return 1;
        }
        // java case 181:
        181 => {
          // java return 1;
          return 1;
        }
        // java case 182:
        182 => {
          // java return 3;
          return 3;
        }
        // java case 183:
        183 => {
          // java return 3;
          return 3;
        }
        // java case 184:
        184 => {
          // java return 3;
          return 3;
        }
        // java case 185:
        185 => {
          // java return 3;
          return 3;
        }
        // java case 186:
        186 => {
          // java return 3;
          return 3;
        }
        // java case 187:
        187 => {
          // java return 3;
          return 3;
        }
        // java case 188:
        188 => {
          // java return 3;
          return 3;
        }
        // java case 189:
        189 => {
          // java return 3;
          return 3;
        }
        // java case 190:
        190 => {
          // java return 3;
          return 3;
        }
        // java case 191:
        191 => {
          // java return 3;
          return 3;
        }
        // java case 192:
        192 => {
          // java return 3;
          return 3;
        }
        // java case 193:
        193 => {
          // java return 3;
          return 3;
        }
        // java case 194:
        194 => {
          // java return 3;
          return 3;
        }
        // java case 195:
        195 => {
          // java return 3;
          return 3;
        }
        // java case 196:
        196 => {
          // java return 3;
          return 3;
        }
        // java case 197:
        197 => {
          // java return 3;
          return 3;
        }
        // java case 198:
        198 => {
          // java return 3;
          return 3;
        }
        // java case 199:
        199 => {
          // java return 3;
          return 3;
        }
        // java case 200:
        200 => {
          // java return 3;
          return 3;
        }
        // java case 201:
        201 => {
          // java return 3;
          return 3;
        }
        // java case 202:
        202 => {
          // java return 3;
          return 3;
        }
        // java case 203:
        203 => {
          // java return 3;
          return 3;
        }
        // java case 204:
        204 => {
          // java return 3;
          return 3;
        }
        // java case 205:
        205 => {
          // java return 3;
          return 3;
        }
        // java case 206:
        206 => {
          // java return 3;
          return 3;
        }
        // java case 207:
        207 => {
          // java return 3;
          return 3;
        }
        // java case 208:
        208 => {
          // java return 3;
          return 3;
        }
        // java case 209:
        209 => {
          // java return 3;
          return 3;
        }
        // java case 210:
        210 => {
          // java return 3;
          return 3;
        }
        // java case 211:
        211 => {
          // java return 3;
          return 3;
        }
        // java case 212:
        212 => {
          // java return 3;
          return 3;
        }
        // java case 213:
        213 => {
          // java return 3;
          return 3;
        }
        // java case 214:
        214 => {
          // java return 3;
          return 3;
        }
        // java case 215:
        215 => {
          // java return 3;
          return 3;
        }
        // java case 216:
        216 => {
          // java return 3;
          return 3;
        }
        // java case 217:
        217 => {
          // java return 3;
          return 3;
        }
        // java case 218:
        218 => {
          // java return 3;
          return 3;
        }
        // java case 219:
        219 => {
          // java return 3;
          return 3;
        }
        // java case 220:
        220 => {
          // java return 3;
          return 3;
        }
        // java case 221:
        221 => {
          // java return 3;
          return 3;
        }
        // java case 222:
        222 => {
          // java return 3;
          return 3;
        }
        // java case 223:
        223 => {
          // java return 3;
          return 3;
        }
        // java case 224:
        224 => {
          // java return 3;
          return 3;
        }
        // java case 225:
        225 => {
          // java return 3;
          return 3;
        }
        // java case 226:
        226 => {
          // java return 3;
          return 3;
        }
        // java case 227:
        227 => {
          // java return 3;
          return 3;
        }
        // java case 228:
        228 => {
          // java return 3;
          return 3;
        }
        // java case 229:
        229 => {
          // java return 3;
          return 3;
        }
        // java case 230:
        230 => {
          // java return 3;
          return 3;
        }
        // java case 231:
        231 => {
          // java return 3;
          return 3;
        }
        // java case 232:
        232 => {
          // java return 3;
          return 3;
        }
        // java case 233:
        233 => {
          // java return 3;
          return 3;
        }
        // java case 234:
        234 => {
          // java return 3;
          return 3;
        }
        // java case 235:
        235 => {
          // java return 3;
          return 3;
        }
        // java case 236:
        236 => {
          // java return 3;
          return 3;
        }
        // java case 237:
        237 => {
          // java return 3;
          return 3;
        }
        // java case 238:
        238 => {
          // java return 3;
          return 3;
        }
        // java case 239:
        239 => {
          // java return 3;
          return 3;
        }
        // java case 240:
        240 => {
          // java return 3;
          return 3;
        }
        // java case 241:
        241 => {
          // java return 3;
          return 3;
        }
        // java case 242:
        242 => {
          // java return 3;
          return 3;
        }
        // java case 243:
        243 => {
          // java return 3;
          return 3;
        }
        // java case 244:
        244 => {
          // java return 3;
          return 3;
        }
        // java case 245:
        245 => {
          // java return 3;
          return 3;
        }
        // java case 246:
        246 => {
          // java return 3;
          return 3;
        }
        // java case 247:
        247 => {
          // java return 3;
          return 3;
        }
        // java case 248:
        248 => {
          // java return 3;
          return 3;
        }
        // java case 249:
        249 => {
          // java return 3;
          return 3;
        }
        // java case 250:
        250 => {
          // java return 3;
          return 3;
        }
        // java case 251:
        251 => {
          // java return 3;
          return 3;
        }
        // java case 252:
        252 => {
          // java return 3;
          return 3;
        }
        // java case 253:
        253 => {
          // java return 3;
          return 3;
        }
        // java case 254:
        254 => {
          // java return 3;
          return 3;
        }
        // java case 255:
        255 => {
          // java return 3;
          return 3;
        }
        // java case 256:
        256 => {
          // java return 3;
          return 3;
        }
        // java case 257:
        257 => {
          // java return 3;
          return 3;
        }
        // java case 258:
        258 => {
          // java return 3;
          return 3;
        }
        // java case 259:
        259 => {
          // java return 3;
          return 3;
        }
        // java case 260:
        260 => {
          // java return 2;
          return 2;
        }
        // java case 261:
        261 => {
          // java return 2;
          return 2;
        }
        // java case 262:
        262 => {
          // java return 2;
          return 2;
        }
        // java case 263:
        263 => {
          // java return 2;
          return 2;
        }
        // java case 264:
        264 => {
          // java return 2;
          return 2;
        }
        // java case 265:
        265 => {
          // java return 1;
          return 1;
        }
        // java case 266:
        266 => {
          // java return 1;
          return 1;
        }
        // java case 12289:
        12289 => {
          // java return 2;
          return 2;
        }
        // java case 12290:
        12290 => {
          // java return 2;
          return 2;
        }
        // java case 12291:
        12291 => {
          // java return 2;
          return 2;
        }
        // java case 12292:
        12292 => {
          // java return 2;
          return 2;
        }
        // java case 12293:
        12293 => {
          // java return 2;
          return 2;
        }
        // java case 12294:
        12294 => {
          // java return 2;
          return 2;
        }
        // java case 12295:
        12295 => {
          // java return 2;
          return 2;
        }
        // java case 12296:
        12296 => {
          // java return 2;
          return 2;
        }
        // java case 12297:
        12297 => {
          // java return 2;
          return 2;
        }
        // java case 12319:
        12319 => {
          // java return 2;
          return 2;
        }
        // java case 12320:
        12320 => {
          // java return 2;
          return 2;
        }
        // java case 12350:
        12350 => {
          // java return 2;
          return 2;
        }
        // java case 12362:
        12362 => {
          // java return 2;
          return 2;
        }
        // java case 12468:
        12468 => {
          // java return 2;
          return 2;
        }
        // java case 12469:
        12469 => {
          // java return 2;
          return 2;
        }
        // java default:
        _ => {
          // java return 1;
          return 1;
        }
      }
    }

    // java private static void initMapName2TypeSti()
    pub fn initMapName2TypeSti() {
      // java if (null == mMapName2TypeSti)
      let mut guard = mMapName2TypeSti.lock().unwrap();
      if guard.is_none() {
        // java long beforeMillis = 0L;
        let beforeMillis: i64 = 0;
        let _ = beforeMillis;
        // java mMapName2TypeSti = new HashMap();
        let mut map: HashMap<String, TypeSti> = HashMap::new();
        // java mMapName2TypeSti.put("annotation reference", new TypeSti(2, 39));
        map.insert("annotation reference".to_string(), TypeSti::TypeSti(2, 39));
        // java mMapName2TypeSti.put("annotation subject", new TypeSti(1, 106));
        map.insert("annotation subject".to_string(), TypeSti::TypeSti(1, 106));
        // java mMapName2TypeSti.put("annotation text", new TypeSti(1, 30));
        map.insert("annotation text".to_string(), TypeSti::TypeSti(1, 30));
        // java mMapName2TypeSti.put("Balloon Text", new TypeSti(1, 153));
        map.insert("Balloon Text".to_string(), TypeSti::TypeSti(1, 153));
        // java mMapName2TypeSti.put("Bibliography", new TypeSti(1, 265));
        map.insert("Bibliography".to_string(), TypeSti::TypeSti(1, 265));
        // java mMapName2TypeSti.put("Block Text", new TypeSti(1, 84));
        map.insert("Block Text".to_string(), TypeSti::TypeSti(1, 84));
        // java mMapName2TypeSti.put("Body Text", new TypeSti(1, 66));
        map.insert("Body Text".to_string(), TypeSti::TypeSti(1, 66));
        // java mMapName2TypeSti.put("Body Text 2", new TypeSti(1, 80));
        map.insert("Body Text 2".to_string(), TypeSti::TypeSti(1, 80));
        // java mMapName2TypeSti.put("Body Text 3", new TypeSti(1, 81));
        map.insert("Body Text 3".to_string(), TypeSti::TypeSti(1, 81));
        // java mMapName2TypeSti.put("Body Text First Indent", new TypeSti(1, 77));
        map.insert("Body Text First Indent".to_string(), TypeSti::TypeSti(1, 77));
        // java mMapName2TypeSti.put("Body Text First Indent 2", new TypeSti(1, 78));
        map.insert("Body Text First Indent 2".to_string(), TypeSti::TypeSti(1, 78));
        // java mMapName2TypeSti.put("Body Text Indent", new TypeSti(1, 67));
        map.insert("Body Text Indent".to_string(), TypeSti::TypeSti(1, 67));
        // java mMapName2TypeSti.put("Body Text Indent 2", new TypeSti(1, 82));
        map.insert("Body Text Indent 2".to_string(), TypeSti::TypeSti(1, 82));
        // java mMapName2TypeSti.put("Body Text Indent 3", new TypeSti(1, 83));
        map.insert("Body Text Indent 3".to_string(), TypeSti::TypeSti(1, 83));
        // java mMapName2TypeSti.put("Book Title", new TypeSti(2, 264));
        map.insert("Book Title".to_string(), TypeSti::TypeSti(2, 264));
        // java mMapName2TypeSti.put("caption", new TypeSti(1, 34));
        map.insert("caption".to_string(), TypeSti::TypeSti(1, 34));
        // java mMapName2TypeSti.put("Closing", new TypeSti(1, 63));
        map.insert("Closing".to_string(), TypeSti::TypeSti(1, 63));
        // java mMapName2TypeSti.put("Colorful Grid", new TypeSti(3, 171));
        map.insert("Colorful Grid".to_string(), TypeSti::TypeSti(3, 171));
        // java mMapName2TypeSti.put("Colorful Grid Accent 1", new TypeSti(3, 189));
        map.insert("Colorful Grid Accent 1".to_string(), TypeSti::TypeSti(3, 189));
        // java mMapName2TypeSti.put("Colorful Grid Accent 2", new TypeSti(3, 203));
        map.insert("Colorful Grid Accent 2".to_string(), TypeSti::TypeSti(3, 203));
        // java mMapName2TypeSti.put("Colorful Grid Accent 3", new TypeSti(3, 217));
        map.insert("Colorful Grid Accent 3".to_string(), TypeSti::TypeSti(3, 217));
        // java mMapName2TypeSti.put("Colorful Grid Accent 4", new TypeSti(3, 231));
        map.insert("Colorful Grid Accent 4".to_string(), TypeSti::TypeSti(3, 231));
        // java mMapName2TypeSti.put("Colorful Grid Accent 5", new TypeSti(3, 245));
        map.insert("Colorful Grid Accent 5".to_string(), TypeSti::TypeSti(3, 245));
        // java mMapName2TypeSti.put("Colorful Grid Accent 6", new TypeSti(3, 259));
        map.insert("Colorful Grid Accent 6".to_string(), TypeSti::TypeSti(3, 259));
        // java mMapName2TypeSti.put("Colorful List", new TypeSti(3, 170));
        map.insert("Colorful List".to_string(), TypeSti::TypeSti(3, 170));
        // java mMapName2TypeSti.put("Colorful List Accent 1", new TypeSti(3, 188));
        map.insert("Colorful List Accent 1".to_string(), TypeSti::TypeSti(3, 188));
        // java mMapName2TypeSti.put("Colorful List Accent 2", new TypeSti(3, 202));
        map.insert("Colorful List Accent 2".to_string(), TypeSti::TypeSti(3, 202));
        // java mMapName2TypeSti.put("Colorful List Accent 3", new TypeSti(3, 216));
        map.insert("Colorful List Accent 3".to_string(), TypeSti::TypeSti(3, 216));
        // java mMapName2TypeSti.put("Colorful List Accent 4", new TypeSti(3, 230));
        map.insert("Colorful List Accent 4".to_string(), TypeSti::TypeSti(3, 230));
        // java mMapName2TypeSti.put("Colorful List Accent 5", new TypeSti(3, 244));
        map.insert("Colorful List Accent 5".to_string(), TypeSti::TypeSti(3, 244));
        // java mMapName2TypeSti.put("Colorful List Accent 6", new TypeSti(3, 258));
        map.insert("Colorful List Accent 6".to_string(), TypeSti::TypeSti(3, 258));
        // java mMapName2TypeSti.put("Colorful Shading", new TypeSti(3, 169));
        map.insert("Colorful Shading".to_string(), TypeSti::TypeSti(3, 169));
        // java mMapName2TypeSti.put("Colorful Shading Accent 1", new TypeSti(3, 187));
        map.insert("Colorful Shading Accent 1".to_string(), TypeSti::TypeSti(3, 187));
        // java mMapName2TypeSti.put("Colorful Shading Accent 2", new TypeSti(3, 201));
        map.insert("Colorful Shading Accent 2".to_string(), TypeSti::TypeSti(3, 201));
        // java mMapName2TypeSti.put("Colorful Shading Accent 3", new TypeSti(3, 215));
        map.insert("Colorful Shading Accent 3".to_string(), TypeSti::TypeSti(3, 215));
        // java mMapName2TypeSti.put("Colorful Shading Accent 4", new TypeSti(3, 229));
        map.insert("Colorful Shading Accent 4".to_string(), TypeSti::TypeSti(3, 229));
        // java mMapName2TypeSti.put("Colorful Shading Accent 5", new TypeSti(3, 243));
        map.insert("Colorful Shading Accent 5".to_string(), TypeSti::TypeSti(3, 243));
        // java mMapName2TypeSti.put("Colorful Shading Accent 6", new TypeSti(3, 257));
        map.insert("Colorful Shading Accent 6".to_string(), TypeSti::TypeSti(3, 257));
        // java mMapName2TypeSti.put("Dark List", new TypeSti(3, 168));
        map.insert("Dark List".to_string(), TypeSti::TypeSti(3, 168));
        // java mMapName2TypeSti.put("Dark List Accent 1", new TypeSti(3, 186));
        map.insert("Dark List Accent 1".to_string(), TypeSti::TypeSti(3, 186));
        // java mMapName2TypeSti.put("Dark List Accent 2", new TypeSti(3, 200));
        map.insert("Dark List Accent 2".to_string(), TypeSti::TypeSti(3, 200));
        // java mMapName2TypeSti.put("Dark List Accent 3", new TypeSti(3, 214));
        map.insert("Dark List Accent 3".to_string(), TypeSti::TypeSti(3, 214));
        // java mMapName2TypeSti.put("Dark List Accent 4", new TypeSti(3, 228));
        map.insert("Dark List Accent 4".to_string(), TypeSti::TypeSti(3, 228));
        // java mMapName2TypeSti.put("Dark List Accent 5", new TypeSti(3, 242));
        map.insert("Dark List Accent 5".to_string(), TypeSti::TypeSti(3, 242));
        // java mMapName2TypeSti.put("Dark List Accent 6", new TypeSti(3, 256));
        map.insert("Dark List Accent 6".to_string(), TypeSti::TypeSti(3, 256));
        // java mMapName2TypeSti.put("Date", new TypeSti(1, 76));
        map.insert("Date".to_string(), TypeSti::TypeSti(1, 76));
        // java mMapName2TypeSti.put("Default Paragraph Font", new TypeSti(2, 65));
        map.insert("Default Paragraph Font".to_string(), TypeSti::TypeSti(2, 65));
        // java mMapName2TypeSti.put("Document Map", new TypeSti(1, 89));
        map.insert("Document Map".to_string(), TypeSti::TypeSti(1, 89));
        // java mMapName2TypeSti.put("E-mail Signature", new TypeSti(1, 91));
        map.insert("E-mail Signature".to_string(), TypeSti::TypeSti(1, 91));
        // java mMapName2TypeSti.put("Emphasis", new TypeSti(2, 88));
        map.insert("Emphasis".to_string(), TypeSti::TypeSti(2, 88));
        // java mMapName2TypeSti.put("endnote reference", new TypeSti(2, 42));
        map.insert("endnote reference".to_string(), TypeSti::TypeSti(2, 42));
        // java mMapName2TypeSti.put("endnote text", new TypeSti(1, 43));
        map.insert("endnote text".to_string(), TypeSti::TypeSti(1, 43));
        // java mMapName2TypeSti.put("envelope address", new TypeSti(1, 36));
        map.insert("envelope address".to_string(), TypeSti::TypeSti(1, 36));
        // java mMapName2TypeSti.put("envelope return", new TypeSti(1, 37));
        map.insert("envelope return".to_string(), TypeSti::TypeSti(1, 37));
        // java mMapName2TypeSti.put("FollowedHyperlink", new TypeSti(2, 86));
        map.insert("FollowedHyperlink".to_string(), TypeSti::TypeSti(2, 86));
        // java mMapName2TypeSti.put("footer", new TypeSti(1, 32));
        map.insert("footer".to_string(), TypeSti::TypeSti(1, 32));
        // java mMapName2TypeSti.put("Footer Char", new TypeSti(2, 12320));
        map.insert("Footer Char".to_string(), TypeSti::TypeSti(2, 12320));
        // java mMapName2TypeSti.put("footnote reference", new TypeSti(2, 38));
        map.insert("footnote reference".to_string(), TypeSti::TypeSti(2, 38));
        // java mMapName2TypeSti.put("footnote text", new TypeSti(1, 29));
        map.insert("footnote text".to_string(), TypeSti::TypeSti(1, 29));
        // java mMapName2TypeSti.put("header", new TypeSti(1, 31));
        map.insert("header".to_string(), TypeSti::TypeSti(1, 31));
        // java mMapName2TypeSti.put("Header Char", new TypeSti(2, 12319));
        map.insert("Header Char".to_string(), TypeSti::TypeSti(2, 12319));
        // java mMapName2TypeSti.put("heading 1", new TypeSti(1, 1));
        map.insert("heading 1".to_string(), TypeSti::TypeSti(1, 1));
        // java mMapName2TypeSti.put("Heading 1 Char", new TypeSti(2, 12289));
        map.insert("Heading 1 Char".to_string(), TypeSti::TypeSti(2, 12289));
        // java mMapName2TypeSti.put("heading 2", new TypeSti(1, 2));
        map.insert("heading 2".to_string(), TypeSti::TypeSti(1, 2));
        // java mMapName2TypeSti.put("Heading 2 Char", new TypeSti(2, 12290));
        map.insert("Heading 2 Char".to_string(), TypeSti::TypeSti(2, 12290));
        // java mMapName2TypeSti.put("heading 3", new TypeSti(1, 3));
        map.insert("heading 3".to_string(), TypeSti::TypeSti(1, 3));
        // java mMapName2TypeSti.put("Heading 3 Char", new TypeSti(2, 12291));
        map.insert("Heading 3 Char".to_string(), TypeSti::TypeSti(2, 12291));
        // java mMapName2TypeSti.put("heading 4", new TypeSti(1, 4));
        map.insert("heading 4".to_string(), TypeSti::TypeSti(1, 4));
        // java mMapName2TypeSti.put("Heading 4 Char", new TypeSti(2, 12292));
        map.insert("Heading 4 Char".to_string(), TypeSti::TypeSti(2, 12292));
        // java mMapName2TypeSti.put("heading 5", new TypeSti(1, 5));
        map.insert("heading 5".to_string(), TypeSti::TypeSti(1, 5));
        // java mMapName2TypeSti.put("Heading 5 Char", new TypeSti(2, 12293));
        map.insert("Heading 5 Char".to_string(), TypeSti::TypeSti(2, 12293));
        // java mMapName2TypeSti.put("heading 6", new TypeSti(1, 6));
        map.insert("heading 6".to_string(), TypeSti::TypeSti(1, 6));
        // java mMapName2TypeSti.put("Heading 6 Char", new TypeSti(2, 12294));
        map.insert("Heading 6 Char".to_string(), TypeSti::TypeSti(2, 12294));
        // java mMapName2TypeSti.put("heading 7", new TypeSti(1, 7));
        map.insert("heading 7".to_string(), TypeSti::TypeSti(1, 7));
        // java mMapName2TypeSti.put("Heading 7 Char", new TypeSti(2, 12295));
        map.insert("Heading 7 Char".to_string(), TypeSti::TypeSti(2, 12295));
        // java mMapName2TypeSti.put("heading 8", new TypeSti(1, 8));
        map.insert("heading 8".to_string(), TypeSti::TypeSti(1, 8));
        // java mMapName2TypeSti.put("Heading 8 Char", new TypeSti(2, 12296));
        map.insert("Heading 8 Char".to_string(), TypeSti::TypeSti(2, 12296));
        // java mMapName2TypeSti.put("heading 9", new TypeSti(1, 9));
        map.insert("heading 9".to_string(), TypeSti::TypeSti(1, 9));
        // java mMapName2TypeSti.put("Heading 9 Char", new TypeSti(2, 12297));
        map.insert("Heading 9 Char".to_string(), TypeSti::TypeSti(2, 12297));
        // java mMapName2TypeSti.put("HTML Acronym", new TypeSti(2, 95));
        map.insert("HTML Acronym".to_string(), TypeSti::TypeSti(2, 95));
        // java mMapName2TypeSti.put("HTML Address", new TypeSti(1, 96));
        map.insert("HTML Address".to_string(), TypeSti::TypeSti(1, 96));
        // java mMapName2TypeSti.put("HTML Bottom of Form", new TypeSti(1, 93));
        map.insert("HTML Bottom of Form".to_string(), TypeSti::TypeSti(1, 93));
        // java mMapName2TypeSti.put("HTML Cite", new TypeSti(2, 97));
        map.insert("HTML Cite".to_string(), TypeSti::TypeSti(2, 97));
        // java mMapName2TypeSti.put("HTML Code", new TypeSti(2, 98));
        map.insert("HTML Code".to_string(), TypeSti::TypeSti(2, 98));
        // java mMapName2TypeSti.put("HTML Definition", new TypeSti(2, 99));
        map.insert("HTML Definition".to_string(), TypeSti::TypeSti(2, 99));
        // java mMapName2TypeSti.put("HTML Keyboard", new TypeSti(2, 100));
        map.insert("HTML Keyboard".to_string(), TypeSti::TypeSti(2, 100));
        // java mMapName2TypeSti.put("HTML Preformatted", new TypeSti(1, 101));
        map.insert("HTML Preformatted".to_string(), TypeSti::TypeSti(1, 101));
        // java mMapName2TypeSti.put("HTML Sample", new TypeSti(2, 102));
        map.insert("HTML Sample".to_string(), TypeSti::TypeSti(2, 102));
        // java mMapName2TypeSti.put("HTML Top of Form", new TypeSti(1, 92));
        map.insert("HTML Top of Form".to_string(), TypeSti::TypeSti(1, 92));
        // java mMapName2TypeSti.put("HTML Typewriter", new TypeSti(2, 103));
        map.insert("HTML Typewriter".to_string(), TypeSti::TypeSti(2, 103));
        // java mMapName2TypeSti.put("HTML Variable", new TypeSti(2, 104));
        map.insert("HTML Variable".to_string(), TypeSti::TypeSti(2, 104));
        // java mMapName2TypeSti.put("Hyperlink", new TypeSti(2, 85));
        map.insert("Hyperlink".to_string(), TypeSti::TypeSti(2, 85));
        // java mMapName2TypeSti.put("index 1", new TypeSti(1, 10));
        map.insert("index 1".to_string(), TypeSti::TypeSti(1, 10));
        // java mMapName2TypeSti.put("index 2", new TypeSti(1, 11));
        map.insert("index 2".to_string(), TypeSti::TypeSti(1, 11));
        // java mMapName2TypeSti.put("index 3", new TypeSti(1, 12));
        map.insert("index 3".to_string(), TypeSti::TypeSti(1, 12));
        // java mMapName2TypeSti.put("index 4", new TypeSti(1, 13));
        map.insert("index 4".to_string(), TypeSti::TypeSti(1, 13));
        // java mMapName2TypeSti.put("index 5", new TypeSti(1, 14));
        map.insert("index 5".to_string(), TypeSti::TypeSti(1, 14));
        // java mMapName2TypeSti.put("index 6", new TypeSti(1, 15));
        map.insert("index 6".to_string(), TypeSti::TypeSti(1, 15));
        // java mMapName2TypeSti.put("index 7", new TypeSti(1, 16));
        map.insert("index 7".to_string(), TypeSti::TypeSti(1, 16));
        // java mMapName2TypeSti.put("index 8", new TypeSti(1, 17));
        map.insert("index 8".to_string(), TypeSti::TypeSti(1, 17));
        // java mMapName2TypeSti.put("index 9", new TypeSti(1, 18));
        map.insert("index 9".to_string(), TypeSti::TypeSti(1, 18));
        // java mMapName2TypeSti.put("index heading", new TypeSti(1, 33));
        map.insert("index heading".to_string(), TypeSti::TypeSti(1, 33));
        // java mMapName2TypeSti.put("Intense Emphasis", new TypeSti(2, 261));
        map.insert("Intense Emphasis".to_string(), TypeSti::TypeSti(2, 261));
        // java mMapName2TypeSti.put("Intense Quote", new TypeSti(1, 181));
        map.insert("Intense Quote".to_string(), TypeSti::TypeSti(1, 181));
        // java mMapName2TypeSti.put("Intense Quote Char", new TypeSti(2, 12469));
        map.insert("Intense Quote Char".to_string(), TypeSti::TypeSti(2, 12469));
        // java mMapName2TypeSti.put("Intense Reference", new TypeSti(2, 263));
        map.insert("Intense Reference".to_string(), TypeSti::TypeSti(2, 263));
        // java mMapName2TypeSti.put("Light Grid", new TypeSti(3, 160));
        map.insert("Light Grid".to_string(), TypeSti::TypeSti(3, 160));
        // java mMapName2TypeSti.put("Light Grid Accent 1", new TypeSti(3, 174));
        map.insert("Light Grid Accent 1".to_string(), TypeSti::TypeSti(3, 174));
        // java mMapName2TypeSti.put("Light Grid Accent 2", new TypeSti(3, 192));
        map.insert("Light Grid Accent 2".to_string(), TypeSti::TypeSti(3, 192));
        // java mMapName2TypeSti.put("Light Grid Accent 3", new TypeSti(3, 206));
        map.insert("Light Grid Accent 3".to_string(), TypeSti::TypeSti(3, 206));
        // java mMapName2TypeSti.put("Light Grid Accent 4", new TypeSti(3, 220));
        map.insert("Light Grid Accent 4".to_string(), TypeSti::TypeSti(3, 220));
        // java mMapName2TypeSti.put("Light Grid Accent 5", new TypeSti(3, 234));
        map.insert("Light Grid Accent 5".to_string(), TypeSti::TypeSti(3, 234));
        // java mMapName2TypeSti.put("Light Grid Accent 6", new TypeSti(3, 248));
        map.insert("Light Grid Accent 6".to_string(), TypeSti::TypeSti(3, 248));
        // java mMapName2TypeSti.put("Light List", new TypeSti(3, 159));
        map.insert("Light List".to_string(), TypeSti::TypeSti(3, 159));
        // java mMapName2TypeSti.put("Light List Accent 1", new TypeSti(3, 173));
        map.insert("Light List Accent 1".to_string(), TypeSti::TypeSti(3, 173));
        // java mMapName2TypeSti.put("Light List Accent 2", new TypeSti(3, 191));
        map.insert("Light List Accent 2".to_string(), TypeSti::TypeSti(3, 191));
        // java mMapName2TypeSti.put("Light List Accent 3", new TypeSti(3, 205));
        map.insert("Light List Accent 3".to_string(), TypeSti::TypeSti(3, 205));
        // java mMapName2TypeSti.put("Light List Accent 4", new TypeSti(3, 219));
        map.insert("Light List Accent 4".to_string(), TypeSti::TypeSti(3, 219));
        // java mMapName2TypeSti.put("Light List Accent 5", new TypeSti(3, 233));
        map.insert("Light List Accent 5".to_string(), TypeSti::TypeSti(3, 233));
        // java mMapName2TypeSti.put("Light List Accent 6", new TypeSti(3, 247));
        map.insert("Light List Accent 6".to_string(), TypeSti::TypeSti(3, 247));
        // java mMapName2TypeSti.put("Light Shading", new TypeSti(3, 158));
        map.insert("Light Shading".to_string(), TypeSti::TypeSti(3, 158));
        // java mMapName2TypeSti.put("Light Shading Accent 1", new TypeSti(3, 172));
        map.insert("Light Shading Accent 1".to_string(), TypeSti::TypeSti(3, 172));
        // java mMapName2TypeSti.put("Light Shading Accent 2", new TypeSti(3, 190));
        map.insert("Light Shading Accent 2".to_string(), TypeSti::TypeSti(3, 190));
        // java mMapName2TypeSti.put("Light Shading Accent 3", new TypeSti(3, 204));
        map.insert("Light Shading Accent 3".to_string(), TypeSti::TypeSti(3, 204));
        // java mMapName2TypeSti.put("Light Shading Accent 4", new TypeSti(3, 218));
        map.insert("Light Shading Accent 4".to_string(), TypeSti::TypeSti(3, 218));
        // java mMapName2TypeSti.put("Light Shading Accent 5", new TypeSti(3, 232));
        map.insert("Light Shading Accent 5".to_string(), TypeSti::TypeSti(3, 232));
        // java mMapName2TypeSti.put("Light Shading Accent 6", new TypeSti(3, 246));
        map.insert("Light Shading Accent 6".to_string(), TypeSti::TypeSti(3, 246));
        // java mMapName2TypeSti.put("line number", new TypeSti(2, 40));
        map.insert("line number".to_string(), TypeSti::TypeSti(2, 40));
        // java mMapName2TypeSti.put("List", new TypeSti(1, 47));
        map.insert("List".to_string(), TypeSti::TypeSti(1, 47));
        // java mMapName2TypeSti.put("List 2", new TypeSti(1, 50));
        map.insert("List 2".to_string(), TypeSti::TypeSti(1, 50));
        // java mMapName2TypeSti.put("List 3", new TypeSti(1, 51));
        map.insert("List 3".to_string(), TypeSti::TypeSti(1, 51));
        // java mMapName2TypeSti.put("List 4", new TypeSti(1, 52));
        map.insert("List 4".to_string(), TypeSti::TypeSti(1, 52));
        // java mMapName2TypeSti.put("List 5", new TypeSti(1, 53));
        map.insert("List 5".to_string(), TypeSti::TypeSti(1, 53));
        // java mMapName2TypeSti.put("List Bullet", new TypeSti(1, 48));
        map.insert("List Bullet".to_string(), TypeSti::TypeSti(1, 48));
        // java mMapName2TypeSti.put("List Bullet 2", new TypeSti(1, 54));
        map.insert("List Bullet 2".to_string(), TypeSti::TypeSti(1, 54));
        // java mMapName2TypeSti.put("List Bullet 3", new TypeSti(1, 55));
        map.insert("List Bullet 3".to_string(), TypeSti::TypeSti(1, 55));
        // java mMapName2TypeSti.put("List Bullet 4", new TypeSti(1, 56));
        map.insert("List Bullet 4".to_string(), TypeSti::TypeSti(1, 56));
        // java mMapName2TypeSti.put("List Bullet 5", new TypeSti(1, 57));
        map.insert("List Bullet 5".to_string(), TypeSti::TypeSti(1, 57));
        // java mMapName2TypeSti.put("List Continue", new TypeSti(1, 68));
        map.insert("List Continue".to_string(), TypeSti::TypeSti(1, 68));
        // java mMapName2TypeSti.put("List Continue 2", new TypeSti(1, 69));
        map.insert("List Continue 2".to_string(), TypeSti::TypeSti(1, 69));
        // java mMapName2TypeSti.put("List Continue 3", new TypeSti(1, 70));
        map.insert("List Continue 3".to_string(), TypeSti::TypeSti(1, 70));
        // java mMapName2TypeSti.put("List Continue 4", new TypeSti(1, 71));
        map.insert("List Continue 4".to_string(), TypeSti::TypeSti(1, 71));
        // java mMapName2TypeSti.put("List Continue 5", new TypeSti(1, 72));
        map.insert("List Continue 5".to_string(), TypeSti::TypeSti(1, 72));
        // java mMapName2TypeSti.put("List Number", new TypeSti(1, 49));
        map.insert("List Number".to_string(), TypeSti::TypeSti(1, 49));
        // java mMapName2TypeSti.put("List Number 2", new TypeSti(1, 58));
        map.insert("List Number 2".to_string(), TypeSti::TypeSti(1, 58));
        // java mMapName2TypeSti.put("List Number 3", new TypeSti(1, 59));
        map.insert("List Number 3".to_string(), TypeSti::TypeSti(1, 59));
        // java mMapName2TypeSti.put("List Number 4", new TypeSti(1, 60));
        map.insert("List Number 4".to_string(), TypeSti::TypeSti(1, 60));
        // java mMapName2TypeSti.put("List Number 5", new TypeSti(1, 61));
        map.insert("List Number 5".to_string(), TypeSti::TypeSti(1, 61));
        // java mMapName2TypeSti.put("List Paragraph", new TypeSti(1, 179));
        map.insert("List Paragraph".to_string(), TypeSti::TypeSti(1, 179));
        // java mMapName2TypeSti.put("macro", new TypeSti(1, 45));
        map.insert("macro".to_string(), TypeSti::TypeSti(1, 45));
        // java mMapName2TypeSti.put("Medium Grid 1", new TypeSti(3, 165));
        map.insert("Medium Grid 1".to_string(), TypeSti::TypeSti(3, 165));
        // java mMapName2TypeSti.put("Medium Grid 1 Accent 1", new TypeSti(3, 183));
        map.insert("Medium Grid 1 Accent 1".to_string(), TypeSti::TypeSti(3, 183));
        // java mMapName2TypeSti.put("Medium Grid 1 Accent 2", new TypeSti(3, 197));
        map.insert("Medium Grid 1 Accent 2".to_string(), TypeSti::TypeSti(3, 197));
        // java mMapName2TypeSti.put("Medium Grid 1 Accent 3", new TypeSti(3, 211));
        map.insert("Medium Grid 1 Accent 3".to_string(), TypeSti::TypeSti(3, 211));
        // java mMapName2TypeSti.put("Medium Grid 1 Accent 4", new TypeSti(3, 225));
        map.insert("Medium Grid 1 Accent 4".to_string(), TypeSti::TypeSti(3, 225));
        // java mMapName2TypeSti.put("Medium Grid 1 Accent 5", new TypeSti(3, 239));
        map.insert("Medium Grid 1 Accent 5".to_string(), TypeSti::TypeSti(3, 239));
        // java mMapName2TypeSti.put("Medium Grid 1 Accent 6", new TypeSti(3, 253));
        map.insert("Medium Grid 1 Accent 6".to_string(), TypeSti::TypeSti(3, 253));
        // java mMapName2TypeSti.put("Medium Grid 2", new TypeSti(3, 166));
        map.insert("Medium Grid 2".to_string(), TypeSti::TypeSti(3, 166));
        // java mMapName2TypeSti.put("Medium Grid 2 Accent 1", new TypeSti(3, 184));
        map.insert("Medium Grid 2 Accent 1".to_string(), TypeSti::TypeSti(3, 184));
        // java mMapName2TypeSti.put("Medium Grid 2 Accent 2", new TypeSti(3, 198));
        map.insert("Medium Grid 2 Accent 2".to_string(), TypeSti::TypeSti(3, 198));
        // java mMapName2TypeSti.put("Medium Grid 2 Accent 3", new TypeSti(3, 212));
        map.insert("Medium Grid 2 Accent 3".to_string(), TypeSti::TypeSti(3, 212));
        // java mMapName2TypeSti.put("Medium Grid 2 Accent 4", new TypeSti(3, 226));
        map.insert("Medium Grid 2 Accent 4".to_string(), TypeSti::TypeSti(3, 226));
        // java mMapName2TypeSti.put("Medium Grid 2 Accent 5", new TypeSti(3, 240));
        map.insert("Medium Grid 2 Accent 5".to_string(), TypeSti::TypeSti(3, 240));
        // java mMapName2TypeSti.put("Medium Grid 2 Accent 6", new TypeSti(3, 254));
        map.insert("Medium Grid 2 Accent 6".to_string(), TypeSti::TypeSti(3, 254));
        // java mMapName2TypeSti.put("Medium Grid 3", new TypeSti(3, 167));
        map.insert("Medium Grid 3".to_string(), TypeSti::TypeSti(3, 167));
        // java mMapName2TypeSti.put("Medium Grid 3 Accent 1", new TypeSti(3, 185));
        map.insert("Medium Grid 3 Accent 1".to_string(), TypeSti::TypeSti(3, 185));
        // java mMapName2TypeSti.put("Medium Grid 3 Accent 2", new TypeSti(3, 199));
        map.insert("Medium Grid 3 Accent 2".to_string(), TypeSti::TypeSti(3, 199));
        // java mMapName2TypeSti.put("Medium Grid 3 Accent 3", new TypeSti(3, 213));
        map.insert("Medium Grid 3 Accent 3".to_string(), TypeSti::TypeSti(3, 213));
        // java mMapName2TypeSti.put("Medium Grid 3 Accent 4", new TypeSti(3, 227));
        map.insert("Medium Grid 3 Accent 4".to_string(), TypeSti::TypeSti(3, 227));
        // java mMapName2TypeSti.put("Medium Grid 3 Accent 5", new TypeSti(3, 241));
        map.insert("Medium Grid 3 Accent 5".to_string(), TypeSti::TypeSti(3, 241));
        // java mMapName2TypeSti.put("Medium Grid 3 Accent 6", new TypeSti(3, 255));
        map.insert("Medium Grid 3 Accent 6".to_string(), TypeSti::TypeSti(3, 255));
        // java mMapName2TypeSti.put("Medium List 1", new TypeSti(3, 163));
        map.insert("Medium List 1".to_string(), TypeSti::TypeSti(3, 163));
        // java mMapName2TypeSti.put("Medium List 1 Accent 1", new TypeSti(3, 177));
        map.insert("Medium List 1 Accent 1".to_string(), TypeSti::TypeSti(3, 177));
        // java mMapName2TypeSti.put("Medium List 1 Accent 2", new TypeSti(3, 195));
        map.insert("Medium List 1 Accent 2".to_string(), TypeSti::TypeSti(3, 195));
        // java mMapName2TypeSti.put("Medium List 1 Accent 3", new TypeSti(3, 209));
        map.insert("Medium List 1 Accent 3".to_string(), TypeSti::TypeSti(3, 209));
        // java mMapName2TypeSti.put("Medium List 1 Accent 4", new TypeSti(3, 223));
        map.insert("Medium List 1 Accent 4".to_string(), TypeSti::TypeSti(3, 223));
        // java mMapName2TypeSti.put("Medium List 1 Accent 5", new TypeSti(3, 237));
        map.insert("Medium List 1 Accent 5".to_string(), TypeSti::TypeSti(3, 237));
        // java mMapName2TypeSti.put("Medium List 1 Accent 6", new TypeSti(3, 251));
        map.insert("Medium List 1 Accent 6".to_string(), TypeSti::TypeSti(3, 251));
        // java mMapName2TypeSti.put("Medium List 2", new TypeSti(3, 164));
        map.insert("Medium List 2".to_string(), TypeSti::TypeSti(3, 164));
        // java mMapName2TypeSti.put("Medium List 2 Accent 1", new TypeSti(3, 182));
        map.insert("Medium List 2 Accent 1".to_string(), TypeSti::TypeSti(3, 182));
        // java mMapName2TypeSti.put("Medium List 2 Accent 2", new TypeSti(3, 196));
        map.insert("Medium List 2 Accent 2".to_string(), TypeSti::TypeSti(3, 196));
        // java mMapName2TypeSti.put("Medium List 2 Accent 3", new TypeSti(3, 210));
        map.insert("Medium List 2 Accent 3".to_string(), TypeSti::TypeSti(3, 210));
        // java mMapName2TypeSti.put("Medium List 2 Accent 4", new TypeSti(3, 224));
        map.insert("Medium List 2 Accent 4".to_string(), TypeSti::TypeSti(3, 224));
        // java mMapName2TypeSti.put("Medium List 2 Accent 5", new TypeSti(3, 238));
        map.insert("Medium List 2 Accent 5".to_string(), TypeSti::TypeSti(3, 238));
        // java mMapName2TypeSti.put("Medium List 2 Accent 6", new TypeSti(3, 252));
        map.insert("Medium List 2 Accent 6".to_string(), TypeSti::TypeSti(3, 252));
        // java mMapName2TypeSti.put("Medium Shading 1", new TypeSti(3, 161));
        map.insert("Medium Shading 1".to_string(), TypeSti::TypeSti(3, 161));
        // java mMapName2TypeSti.put("Medium Shading 1 Accent 1", new TypeSti(3, 175));
        map.insert("Medium Shading 1 Accent 1".to_string(), TypeSti::TypeSti(3, 175));
        // java mMapName2TypeSti.put("Medium Shading 1 Accent 2", new TypeSti(3, 193));
        map.insert("Medium Shading 1 Accent 2".to_string(), TypeSti::TypeSti(3, 193));
        // java mMapName2TypeSti.put("Medium Shading 1 Accent 3", new TypeSti(3, 207));
        map.insert("Medium Shading 1 Accent 3".to_string(), TypeSti::TypeSti(3, 207));
        // java mMapName2TypeSti.put("Medium Shading 1 Accent 4", new TypeSti(3, 221));
        map.insert("Medium Shading 1 Accent 4".to_string(), TypeSti::TypeSti(3, 221));
        // java mMapName2TypeSti.put("Medium Shading 1 Accent 5", new TypeSti(3, 235));
        map.insert("Medium Shading 1 Accent 5".to_string(), TypeSti::TypeSti(3, 235));
        // java mMapName2TypeSti.put("Medium Shading 1 Accent 6", new TypeSti(3, 249));
        map.insert("Medium Shading 1 Accent 6".to_string(), TypeSti::TypeSti(3, 249));
        // java mMapName2TypeSti.put("Medium Shading 2", new TypeSti(3, 162));
        map.insert("Medium Shading 2".to_string(), TypeSti::TypeSti(3, 162));
        // java mMapName2TypeSti.put("Medium Shading 2 Accent 1", new TypeSti(3, 176));
        map.insert("Medium Shading 2 Accent 1".to_string(), TypeSti::TypeSti(3, 176));
        // java mMapName2TypeSti.put("Medium Shading 2 Accent 2", new TypeSti(3, 194));
        map.insert("Medium Shading 2 Accent 2".to_string(), TypeSti::TypeSti(3, 194));
        // java mMapName2TypeSti.put("Medium Shading 2 Accent 3", new TypeSti(3, 208));
        map.insert("Medium Shading 2 Accent 3".to_string(), TypeSti::TypeSti(3, 208));
        // java mMapName2TypeSti.put("Medium Shading 2 Accent 4", new TypeSti(3, 222));
        map.insert("Medium Shading 2 Accent 4".to_string(), TypeSti::TypeSti(3, 222));
        // java mMapName2TypeSti.put("Medium Shading 2 Accent 5", new TypeSti(3, 236));
        map.insert("Medium Shading 2 Accent 5".to_string(), TypeSti::TypeSti(3, 236));
        // java mMapName2TypeSti.put("Medium Shading 2 Accent 6", new TypeSti(3, 250));
        map.insert("Medium Shading 2 Accent 6".to_string(), TypeSti::TypeSti(3, 250));
        // java mMapName2TypeSti.put("Message Header", new TypeSti(1, 73));
        map.insert("Message Header".to_string(), TypeSti::TypeSti(1, 73));
        // java mMapName2TypeSti.put("No List", new TypeSti(4, 107));
        map.insert("No List".to_string(), TypeSti::TypeSti(4, 107));
        // java mMapName2TypeSti.put("No Spacing", new TypeSti(1, 157));
        map.insert("No Spacing".to_string(), TypeSti::TypeSti(1, 157));
        // java mMapName2TypeSti.put("Normal", new TypeSti(1, 0));
        map.insert("Normal".to_string(), TypeSti::TypeSti(1, 0));
        // java mMapName2TypeSti.put("Normal (Web)", new TypeSti(1, 94));
        map.insert("Normal (Web)".to_string(), TypeSti::TypeSti(1, 94));
        // java mMapName2TypeSti.put("Normal Indent", new TypeSti(1, 28));
        map.insert("Normal Indent".to_string(), TypeSti::TypeSti(1, 28));
        // java mMapName2TypeSti.put("Normal Table", new TypeSti(3, 105));
        map.insert("Normal Table".to_string(), TypeSti::TypeSti(3, 105));
        // java mMapName2TypeSti.put("Note Heading", new TypeSti(1, 79));
        map.insert("Note Heading".to_string(), TypeSti::TypeSti(1, 79));
        // java mMapName2TypeSti.put("Outline List 1", new TypeSti(4, 108));
        map.insert("Outline List 1".to_string(), TypeSti::TypeSti(4, 108));
        // java mMapName2TypeSti.put("Outline List 2", new TypeSti(4, 109));
        map.insert("Outline List 2".to_string(), TypeSti::TypeSti(4, 109));
        // java mMapName2TypeSti.put("Outline List 3", new TypeSti(4, 110));
        map.insert("Outline List 3".to_string(), TypeSti::TypeSti(4, 110));
        // java mMapName2TypeSti.put("page number", new TypeSti(2, 41));
        map.insert("page number".to_string(), TypeSti::TypeSti(2, 41));
        // java mMapName2TypeSti.put("Placeholder Text", new TypeSti(2, 156));
        map.insert("Placeholder Text".to_string(), TypeSti::TypeSti(2, 156));
        // java mMapName2TypeSti.put("Plain Text", new TypeSti(1, 90));
        map.insert("Plain Text".to_string(), TypeSti::TypeSti(1, 90));
        // java mMapName2TypeSti.put("Quote", new TypeSti(1, 180));
        map.insert("Quote".to_string(), TypeSti::TypeSti(1, 180));
        // java mMapName2TypeSti.put("Quote Char", new TypeSti(2, 12468));
        map.insert("Quote Char".to_string(), TypeSti::TypeSti(2, 12468));
        // java mMapName2TypeSti.put("Revision", new TypeSti(1, 178));
        map.insert("Revision".to_string(), TypeSti::TypeSti(1, 178));
        // java mMapName2TypeSti.put("Salutation", new TypeSti(1, 75));
        map.insert("Salutation".to_string(), TypeSti::TypeSti(1, 75));
        // java mMapName2TypeSti.put("Signature", new TypeSti(1, 64));
        map.insert("Signature".to_string(), TypeSti::TypeSti(1, 64));
        // java mMapName2TypeSti.put("Strong", new TypeSti(2, 87));
        map.insert("Strong".to_string(), TypeSti::TypeSti(2, 87));
        // java mMapName2TypeSti.put("Subtitle", new TypeSti(1, 74));
        map.insert("Subtitle".to_string(), TypeSti::TypeSti(1, 74));
        // java mMapName2TypeSti.put("SubtitleChar", new TypeSti(2, 12362));
        map.insert("SubtitleChar".to_string(), TypeSti::TypeSti(2, 12362));
        // java mMapName2TypeSti.put("Subtle Emphasis", new TypeSti(2, 260));
        map.insert("Subtle Emphasis".to_string(), TypeSti::TypeSti(2, 260));
        // java mMapName2TypeSti.put("Subtle Reference", new TypeSti(2, 262));
        map.insert("Subtle Reference".to_string(), TypeSti::TypeSti(2, 262));
        // java mMapName2TypeSti.put("Table 3D effects 1", new TypeSti(3, 142));
        map.insert("Table 3D effects 1".to_string(), TypeSti::TypeSti(3, 142));
        // java mMapName2TypeSti.put("Table 3D effects 2", new TypeSti(3, 143));
        map.insert("Table 3D effects 2".to_string(), TypeSti::TypeSti(3, 143));
        // java mMapName2TypeSti.put("Table 3D effects 3", new TypeSti(3, 144));
        map.insert("Table 3D effects 3".to_string(), TypeSti::TypeSti(3, 144));
        // java mMapName2TypeSti.put("Table Classic 1", new TypeSti(3, 114));
        map.insert("Table Classic 1".to_string(), TypeSti::TypeSti(3, 114));
        // java mMapName2TypeSti.put("Table Classic 2", new TypeSti(3, 115));
        map.insert("Table Classic 2".to_string(), TypeSti::TypeSti(3, 115));
        // java mMapName2TypeSti.put("Table Classic 3", new TypeSti(3, 116));
        map.insert("Table Classic 3".to_string(), TypeSti::TypeSti(3, 116));
        // java mMapName2TypeSti.put("Table Classic 4", new TypeSti(3, 117));
        map.insert("Table Classic 4".to_string(), TypeSti::TypeSti(3, 117));
        // java mMapName2TypeSti.put("Table Colorful 1", new TypeSti(3, 118));
        map.insert("Table Colorful 1".to_string(), TypeSti::TypeSti(3, 118));
        // java mMapName2TypeSti.put("Table Colorful 2", new TypeSti(3, 119));
        map.insert("Table Colorful 2".to_string(), TypeSti::TypeSti(3, 119));
        // java mMapName2TypeSti.put("Table Colorful 3", new TypeSti(3, 120));
        map.insert("Table Colorful 3".to_string(), TypeSti::TypeSti(3, 120));
        // java mMapName2TypeSti.put("Table Columns 1", new TypeSti(3, 121));
        map.insert("Table Columns 1".to_string(), TypeSti::TypeSti(3, 121));
        // java mMapName2TypeSti.put("Table Columns 2", new TypeSti(3, 122));
        map.insert("Table Columns 2".to_string(), TypeSti::TypeSti(3, 122));
        // java mMapName2TypeSti.put("Table Columns 3", new TypeSti(3, 123));
        map.insert("Table Columns 3".to_string(), TypeSti::TypeSti(3, 123));
        // java mMapName2TypeSti.put("Table Columns 4", new TypeSti(3, 124));
        map.insert("Table Columns 4".to_string(), TypeSti::TypeSti(3, 124));
        // java mMapName2TypeSti.put("Table Columns 5", new TypeSti(3, 125));
        map.insert("Table Columns 5".to_string(), TypeSti::TypeSti(3, 125));
        // java mMapName2TypeSti.put("Table Contemporary", new TypeSti(3, 145));
        map.insert("Table Contemporary".to_string(), TypeSti::TypeSti(3, 145));
        // java mMapName2TypeSti.put("Table Elegant", new TypeSti(3, 146));
        map.insert("Table Elegant".to_string(), TypeSti::TypeSti(3, 146));
        // java mMapName2TypeSti.put("Table Grid", new TypeSti(3, 154));
        map.insert("Table Grid".to_string(), TypeSti::TypeSti(3, 154));
        // java mMapName2TypeSti.put("Table Grid 1", new TypeSti(3, 126));
        map.insert("Table Grid 1".to_string(), TypeSti::TypeSti(3, 126));
        // java mMapName2TypeSti.put("Table Grid 2", new TypeSti(3, 127));
        map.insert("Table Grid 2".to_string(), TypeSti::TypeSti(3, 127));
        // java mMapName2TypeSti.put("Table Grid 3", new TypeSti(3, 128));
        map.insert("Table Grid 3".to_string(), TypeSti::TypeSti(3, 128));
        // java mMapName2TypeSti.put("Table Grid 4", new TypeSti(3, 129));
        map.insert("Table Grid 4".to_string(), TypeSti::TypeSti(3, 129));
        // java mMapName2TypeSti.put("Table Grid 5", new TypeSti(3, 130));
        map.insert("Table Grid 5".to_string(), TypeSti::TypeSti(3, 130));
        // java mMapName2TypeSti.put("Table Grid 6", new TypeSti(3, 131));
        map.insert("Table Grid 6".to_string(), TypeSti::TypeSti(3, 131));
        // java mMapName2TypeSti.put("Table Grid 7", new TypeSti(3, 132));
        map.insert("Table Grid 7".to_string(), TypeSti::TypeSti(3, 132));
        // java mMapName2TypeSti.put("Table Grid 8", new TypeSti(3, 133));
        map.insert("Table Grid 8".to_string(), TypeSti::TypeSti(3, 133));
        // java mMapName2TypeSti.put("Table List 1", new TypeSti(3, 134));
        map.insert("Table List 1".to_string(), TypeSti::TypeSti(3, 134));
        // java mMapName2TypeSti.put("Table List 2", new TypeSti(3, 135));
        map.insert("Table List 2".to_string(), TypeSti::TypeSti(3, 135));
        // java mMapName2TypeSti.put("Table List 3", new TypeSti(3, 136));
        map.insert("Table List 3".to_string(), TypeSti::TypeSti(3, 136));
        // java mMapName2TypeSti.put("Table List 4", new TypeSti(3, 137));
        map.insert("Table List 4".to_string(), TypeSti::TypeSti(3, 137));
        // java mMapName2TypeSti.put("Table List 5", new TypeSti(3, 138));
        map.insert("Table List 5".to_string(), TypeSti::TypeSti(3, 138));
        // java mMapName2TypeSti.put("Table List 6", new TypeSti(3, 139));
        map.insert("Table List 6".to_string(), TypeSti::TypeSti(3, 139));
        // java mMapName2TypeSti.put("Table List 7", new TypeSti(3, 140));
        map.insert("Table List 7".to_string(), TypeSti::TypeSti(3, 140));
        // java mMapName2TypeSti.put("Table List 8", new TypeSti(3, 141));
        map.insert("Table List 8".to_string(), TypeSti::TypeSti(3, 141));
        // java mMapName2TypeSti.put("table of authorities", new TypeSti(1, 44));
        map.insert("table of authorities".to_string(), TypeSti::TypeSti(1, 44));
        // java mMapName2TypeSti.put("table of figures", new TypeSti(1, 35));
        map.insert("table of figures".to_string(), TypeSti::TypeSti(1, 35));
        // java mMapName2TypeSti.put("Table Professional", new TypeSti(3, 147));
        map.insert("Table Professional".to_string(), TypeSti::TypeSti(3, 147));
        // java mMapName2TypeSti.put("Table Simple 1", new TypeSti(3, 111));
        map.insert("Table Simple 1".to_string(), TypeSti::TypeSti(3, 111));
        // java mMapName2TypeSti.put("Table Simple 2", new TypeSti(3, 112));
        map.insert("Table Simple 2".to_string(), TypeSti::TypeSti(3, 112));
        // java mMapName2TypeSti.put("Table Simple 3", new TypeSti(3, 113));
        map.insert("Table Simple 3".to_string(), TypeSti::TypeSti(3, 113));
        // java mMapName2TypeSti.put("Table Subtle 1", new TypeSti(3, 148));
        map.insert("Table Subtle 1".to_string(), TypeSti::TypeSti(3, 148));
        // java mMapName2TypeSti.put("Table Subtle 2", new TypeSti(3, 149));
        map.insert("Table Subtle 2".to_string(), TypeSti::TypeSti(3, 149));
        // java mMapName2TypeSti.put("Table Theme", new TypeSti(3, 155));
        map.insert("Table Theme".to_string(), TypeSti::TypeSti(3, 155));
        // java mMapName2TypeSti.put("Table Web 1", new TypeSti(3, 150));
        map.insert("Table Web 1".to_string(), TypeSti::TypeSti(3, 150));
        // java mMapName2TypeSti.put("Table Web 2", new TypeSti(3, 151));
        map.insert("Table Web 2".to_string(), TypeSti::TypeSti(3, 151));
        // java mMapName2TypeSti.put("Table Web 3", new TypeSti(3, 152));
        map.insert("Table Web 3".to_string(), TypeSti::TypeSti(3, 152));
        // java mMapName2TypeSti.put("Title", new TypeSti(1, 62));
        map.insert("Title".to_string(), TypeSti::TypeSti(1, 62));
        // java mMapName2TypeSti.put("Title Char", new TypeSti(2, 12350));
        map.insert("Title Char".to_string(), TypeSti::TypeSti(2, 12350));
        // java mMapName2TypeSti.put("toa heading", new TypeSti(1, 46));
        map.insert("toa heading".to_string(), TypeSti::TypeSti(1, 46));
        // java mMapName2TypeSti.put("toc 1", new TypeSti(1, 19));
        map.insert("toc 1".to_string(), TypeSti::TypeSti(1, 19));
        // java mMapName2TypeSti.put("toc 2", new TypeSti(1, 20));
        map.insert("toc 2".to_string(), TypeSti::TypeSti(1, 20));
        // java mMapName2TypeSti.put("toc 3", new TypeSti(1, 21));
        map.insert("toc 3".to_string(), TypeSti::TypeSti(1, 21));
        // java mMapName2TypeSti.put("toc 4", new TypeSti(1, 22));
        map.insert("toc 4".to_string(), TypeSti::TypeSti(1, 22));
        // java mMapName2TypeSti.put("toc 5", new TypeSti(1, 23));
        map.insert("toc 5".to_string(), TypeSti::TypeSti(1, 23));
        // java mMapName2TypeSti.put("toc 6", new TypeSti(1, 24));
        map.insert("toc 6".to_string(), TypeSti::TypeSti(1, 24));
        // java mMapName2TypeSti.put("toc 7", new TypeSti(1, 25));
        map.insert("toc 7".to_string(), TypeSti::TypeSti(1, 25));
        // java mMapName2TypeSti.put("toc 8", new TypeSti(1, 26));
        map.insert("toc 8".to_string(), TypeSti::TypeSti(1, 26));
        // java mMapName2TypeSti.put("toc 9", new TypeSti(1, 27));
        map.insert("toc 9".to_string(), TypeSti::TypeSti(1, 27));
        // java mMapName2TypeSti.put("TOC Heading", new TypeSti(1, 266));
        map.insert("TOC Heading".to_string(), TypeSti::TypeSti(1, 266));
        *guard = Some(map);
      }
    }

}

// java public static class TypeSti
impl TypeSti {
    // java public  TypeSti(int type, int sti)
    pub fn TypeSti(r#type: i32, sti: i32) -> Self {
      // java this.type = type;
      Self { r#type: r#type, sti: sti }
      // java this.sti = sti;
    }

}
