use crate::java_runtime::{Rc,RefCell};
use std::sync::{LazyLock,OnceLock};
pub type Triplet=Rc<RefCell<[i16;3]>>;
// rust: Trait dispatch preserves KmoColor's original subclass overrides.
pub trait Color:Send+Sync {fn getIndex(&self)->i16{8} fn getTriplet(&self)->Triplet{BLACK::triplet()} fn getHexString(&self)->String{"0:0:0".into()} fn getColorName(&self)->String{String::new()}}
// java public class KmoColor
pub struct KmoColor;
impl Color for KmoColor {}
static BUILTIN_COLORS:OnceLock<Rc<RefCell<Vec<Rc<dyn Color>>>>>=OnceLock::new();
impl KmoColor {
// java public static final int AUTOMATIC_COLOR = 16777215;
pub const AUTOMATIC_COLOR:i32=16777215;
// java public static final int BLACK_RGB = -16777216;
pub const BLACK_RGB:i32=-16777216;
// java public static final int NORMAL_BG = 167772160;
pub const NORMAL_BG:i32=167772160;
// java public static final int DEFAULT_FONT_COLOR = 32767;
pub const DEFAULT_FONT_COLOR:i32=32767;
// java public static final int DEFAULT_FILL_FG_COLOR = 64;
pub const DEFAULT_FILL_FG_COLOR:i32=64;
// java public static final int DEFAULT_FILL_BG_COLOR = 65;
pub const DEFAULT_FILL_BG_COLOR:i32=65;
// java public static final int DEFAULT_SHEET_TAB_COLOR = 127;
pub const DEFAULT_SHEET_TAB_COLOR:i32=127;
// java public static final int DEFAULT_CELL_BORDER_COLOR = 0;
pub const DEFAULT_CELL_BORDER_COLOR:i32=0;
// java public static final int DEFAULT_CELL_BORDER_COLOR_2 = 64;
pub const DEFAULT_CELL_BORDER_COLOR_2:i32=64;
// java public static final boolean isAutomaticColor(int color) {
// java return (-16777216 & color) == 0;
// java }
pub fn isAutomaticColor(color:i32)->bool {
(-16777216 & color)==0
}
// java public static int rgb(int red, int green, int blue) {
// java return -16777216 | red << 16 | green << 8 | blue;
// java }
pub fn rgb(red:i32,green:i32,blue:i32)->i32 {
-16777216 | red.wrapping_shl(16) | green.wrapping_shl(8) | blue
}
// java private static KmoColor[] getAllColors() {
// java return new KmoColor[]{new BLACK(), new BROWN(), new OLIVE_GREEN(), new DARK_GREEN(), new DARK_TEAL(), new DARK_BLUE(), new INDIGO(), new GREY_80_PERCENT(), new ORANGE(), new DARK_YELLOW(), new GREEN(), new TEAL(), new BLUE(), new BLUE_GREY(), new GREY_50_PERCENT(), new RED(), new LIGHT_ORANGE(), new LIME(), new SEA_GREEN(), new AQUA(), new LIGHT_BLUE(), new VIOLET(), new GREY_40_PERCENT(), new PINK(), new GOLD(), new YELLOW(), new BRIGHT_GREEN(), new TURQUOISE(), new DARK_RED(), new SKY_BLUE(), new PLUM(), new GREY_25_PERCENT(), new ROSE(), new LIGHT_YELLOW(), new LIGHT_GREEN(), new LIGHT_TURQUOISE(), new PALE_BLUE(), new LAVENDER(), new WHITE(), new CORNFLOWER_BLUE(), new LEMON_CHIFFON(), new MAROON(), new ORCHID(), new CORAL(), new ROYAL_BLUE(), new LIGHT_CORNFLOWER_BLUE(), new TAN()};
// java }
pub fn getAllColors()->Rc<RefCell<Vec<Rc<dyn Color>>>> {
Rc::new(RefCell::new(vec![Rc::new(BLACK),Rc::new(BROWN),Rc::new(OLIVE_GREEN),Rc::new(DARK_GREEN),Rc::new(DARK_TEAL),Rc::new(DARK_BLUE),Rc::new(INDIGO),Rc::new(GREY_80_PERCENT),Rc::new(ORANGE),Rc::new(DARK_YELLOW),Rc::new(GREEN),Rc::new(TEAL),Rc::new(BLUE),Rc::new(BLUE_GREY),Rc::new(GREY_50_PERCENT),Rc::new(RED),Rc::new(LIGHT_ORANGE),Rc::new(LIME),Rc::new(SEA_GREEN),Rc::new(AQUA),Rc::new(LIGHT_BLUE),Rc::new(VIOLET),Rc::new(GREY_40_PERCENT),Rc::new(PINK),Rc::new(GOLD),Rc::new(YELLOW),Rc::new(BRIGHT_GREEN),Rc::new(TURQUOISE),Rc::new(DARK_RED),Rc::new(SKY_BLUE),Rc::new(PLUM),Rc::new(GREY_25_PERCENT),Rc::new(ROSE),Rc::new(LIGHT_YELLOW),Rc::new(LIGHT_GREEN),Rc::new(LIGHT_TURQUOISE),Rc::new(PALE_BLUE),Rc::new(LAVENDER),Rc::new(WHITE),Rc::new(CORNFLOWER_BLUE),Rc::new(LEMON_CHIFFON),Rc::new(MAROON),Rc::new(ORCHID),Rc::new(CORAL),Rc::new(ROYAL_BLUE),Rc::new(LIGHT_CORNFLOWER_BLUE),Rc::new(TAN)]))
}
// java public static final KmoColor[] getBuiltinColors(boolean isCache) {
// java if (isCache) {
// java if (builtinColors == null) {
// java builtinColors = getAllColors();
// java }
// java
// java return builtinColors;
// java } else {
// java return getAllColors();
// java }
// java }
pub fn getBuiltinColors(isCache:bool)->Rc<RefCell<Vec<Rc<dyn Color>>>> {
if isCache{BUILTIN_COLORS.get_or_init(Self::getAllColors).clone()}else{Self::getAllColors()}
}
}
// java public static final class BLACK extends KmoColor {
// java public static final String COLOR_NAME = "BLACK";
// java public static final short index = 8;
// java public static final short[] triplet = new short[]{0, 0, 0};
// java public static final String hexString = "0:0:0";
// java
// java public short getIndex() {
// java return 8;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "0:0:0";
// java }
// java
// java public String getColorName() {
// java return "BLACK";
// java }
// java }
pub struct BLACK;
static BLACK_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([0, 0, 0])));
impl BLACK {pub const index:i16=8;pub const COLOR_NAME:&'static str="BLACK";pub fn triplet()->Triplet{BLACK_TRIPLET.clone()}}
impl Color for BLACK {fn getIndex(&self)->i16{8}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"0:0:0".into()}fn getColorName(&self)->String{"BLACK".into()}}
// java public static final class BROWN extends KmoColor {
// java public static final String COLOR_NAME = "BROWN";
// java public static final short index = 60;
// java public static final short[] triplet = new short[]{153, 51, 0};
// java public static final String hexString = "9999:3333:0";
// java
// java public short getIndex() {
// java return 60;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "9999:3333:0";
// java }
// java
// java public String getColorName() {
// java return "BROWN";
// java }
// java }
pub struct BROWN;
static BROWN_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([153, 51, 0])));
impl BROWN {pub const index:i16=60;pub const COLOR_NAME:&'static str="BROWN";pub fn triplet()->Triplet{BROWN_TRIPLET.clone()}}
impl Color for BROWN {fn getIndex(&self)->i16{60}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"9999:3333:0".into()}fn getColorName(&self)->String{"BROWN".into()}}
// java public static class OLIVE_GREEN extends KmoColor {
// java public static final String COLOR_NAME = "OLIVE_GREEN";
// java public static final short index = 59;
// java public static final short[] triplet = new short[]{51, 51, 0};
// java public static final String hexString = "3333:3333:0";
// java
// java public short getIndex() {
// java return 59;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "3333:3333:0";
// java }
// java
// java public String getColorName() {
// java return "OLIVE_GREEN";
// java }
// java }
pub struct OLIVE_GREEN;
static OLIVE_GREEN_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([51, 51, 0])));
impl OLIVE_GREEN {pub const index:i16=59;pub const COLOR_NAME:&'static str="OLIVE_GREEN";pub fn triplet()->Triplet{OLIVE_GREEN_TRIPLET.clone()}}
impl Color for OLIVE_GREEN {fn getIndex(&self)->i16{59}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"3333:3333:0".into()}fn getColorName(&self)->String{"OLIVE_GREEN".into()}}
// java public static final class DARK_GREEN extends KmoColor {
// java public static final String COLOR_NAME = "DARK_GREEN";
// java public static final short index = 58;
// java public static final short[] triplet = new short[]{0, 51, 0};
// java public static final String hexString = "0:3333:0";
// java
// java public short getIndex() {
// java return 58;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "0:3333:0";
// java }
// java
// java public String getColorName() {
// java return "DARK_GREEN";
// java }
// java }
pub struct DARK_GREEN;
static DARK_GREEN_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([0, 51, 0])));
impl DARK_GREEN {pub const index:i16=58;pub const COLOR_NAME:&'static str="DARK_GREEN";pub fn triplet()->Triplet{DARK_GREEN_TRIPLET.clone()}}
impl Color for DARK_GREEN {fn getIndex(&self)->i16{58}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"0:3333:0".into()}fn getColorName(&self)->String{"DARK_GREEN".into()}}
// java public static final class DARK_TEAL extends KmoColor {
// java public static final String COLOR_NAME = "DARK_TEAL";
// java public static final short index = 56;
// java public static final short[] triplet = new short[]{0, 51, 102};
// java public static final String hexString = "0:3333:6666";
// java
// java public short getIndex() {
// java return 56;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "0:3333:6666";
// java }
// java
// java public String getColorName() {
// java return "DARK_TEAL";
// java }
// java }
pub struct DARK_TEAL;
static DARK_TEAL_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([0, 51, 102])));
impl DARK_TEAL {pub const index:i16=56;pub const COLOR_NAME:&'static str="DARK_TEAL";pub fn triplet()->Triplet{DARK_TEAL_TRIPLET.clone()}}
impl Color for DARK_TEAL {fn getIndex(&self)->i16{56}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"0:3333:6666".into()}fn getColorName(&self)->String{"DARK_TEAL".into()}}
// java public static final class DARK_BLUE extends KmoColor {
// java public static final String COLOR_NAME = "DARK_BLUE";
// java public static final short index = 18;
// java public static final short index2 = 32;
// java public static final short[] triplet = new short[]{0, 0, 128};
// java public static final String hexString = "0:0:8080";
// java
// java public short getIndex() {
// java return 18;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "0:0:8080";
// java }
// java
// java public String getColorName() {
// java return "DARK_BLUE";
// java }
// java }
pub struct DARK_BLUE;
static DARK_BLUE_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([0, 0, 128])));
impl DARK_BLUE {pub const index:i16=18;pub const COLOR_NAME:&'static str="DARK_BLUE";pub fn triplet()->Triplet{DARK_BLUE_TRIPLET.clone()}}
impl Color for DARK_BLUE {fn getIndex(&self)->i16{18}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"0:0:8080".into()}fn getColorName(&self)->String{"DARK_BLUE".into()}}
// java public static final class INDIGO extends KmoColor {
// java public static final String COLOR_NAME = "INDIGO";
// java public static final short index = 62;
// java public static final short[] triplet = new short[]{51, 51, 153};
// java public static final String hexString = "3333:3333:9999";
// java
// java public short getIndex() {
// java return 62;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "3333:3333:9999";
// java }
// java
// java public String getColorName() {
// java return "INDIGO";
// java }
// java }
pub struct INDIGO;
static INDIGO_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([51, 51, 153])));
impl INDIGO {pub const index:i16=62;pub const COLOR_NAME:&'static str="INDIGO";pub fn triplet()->Triplet{INDIGO_TRIPLET.clone()}}
impl Color for INDIGO {fn getIndex(&self)->i16{62}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"3333:3333:9999".into()}fn getColorName(&self)->String{"INDIGO".into()}}
// java public static final class GREY_80_PERCENT extends KmoColor {
// java public static final String COLOR_NAME = "GREY_80_PERCENT";
// java public static final short index = 63;
// java public static final short[] triplet = new short[]{51, 51, 51};
// java public static final String hexString = "3333:3333:3333";
// java
// java public short getIndex() {
// java return 63;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "3333:3333:3333";
// java }
// java
// java public String getColorName() {
// java return "GREY_80_PERCENT";
// java }
// java }
pub struct GREY_80_PERCENT;
static GREY_80_PERCENT_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([51, 51, 51])));
impl GREY_80_PERCENT {pub const index:i16=63;pub const COLOR_NAME:&'static str="GREY_80_PERCENT";pub fn triplet()->Triplet{GREY_80_PERCENT_TRIPLET.clone()}}
impl Color for GREY_80_PERCENT {fn getIndex(&self)->i16{63}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"3333:3333:3333".into()}fn getColorName(&self)->String{"GREY_80_PERCENT".into()}}
// java public static final class DARK_RED extends KmoColor {
// java public static final String COLOR_NAME = "DARK_RED";
// java public static final short index = 16;
// java public static final short index2 = 37;
// java public static final short[] triplet = new short[]{128, 0, 0};
// java public static final String hexString = "8080:0:0";
// java
// java public short getIndex() {
// java return 16;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "8080:0:0";
// java }
// java
// java public String getColorName() {
// java return "DARK_RED";
// java }
// java }
pub struct DARK_RED;
static DARK_RED_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([128, 0, 0])));
impl DARK_RED {pub const index:i16=16;pub const COLOR_NAME:&'static str="DARK_RED";pub fn triplet()->Triplet{DARK_RED_TRIPLET.clone()}}
impl Color for DARK_RED {fn getIndex(&self)->i16{16}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"8080:0:0".into()}fn getColorName(&self)->String{"DARK_RED".into()}}
// java public static final class ORANGE extends KmoColor {
// java public static final String COLOR_NAME = "ORANGE";
// java public static final short index = 53;
// java public static final short[] triplet = new short[]{255, 102, 0};
// java public static final String hexString = "FFFF:6666:0";
// java
// java public short getIndex() {
// java return 53;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "FFFF:6666:0";
// java }
// java
// java public String getColorName() {
// java return "ORANGE";
// java }
// java }
pub struct ORANGE;
static ORANGE_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([255, 102, 0])));
impl ORANGE {pub const index:i16=53;pub const COLOR_NAME:&'static str="ORANGE";pub fn triplet()->Triplet{ORANGE_TRIPLET.clone()}}
impl Color for ORANGE {fn getIndex(&self)->i16{53}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"FFFF:6666:0".into()}fn getColorName(&self)->String{"ORANGE".into()}}
// java public static final class DARK_YELLOW extends KmoColor {
// java public static final String COLOR_NAME = "DARK_YELLOW";
// java public static final short index = 19;
// java public static final short[] triplet = new short[]{128, 128, 0};
// java public static final String hexString = "8080:8080:0";
// java
// java public short getIndex() {
// java return 19;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "8080:8080:0";
// java }
// java
// java public String getColorName() {
// java return "DARK_YELLOW";
// java }
// java }
pub struct DARK_YELLOW;
static DARK_YELLOW_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([128, 128, 0])));
impl DARK_YELLOW {pub const index:i16=19;pub const COLOR_NAME:&'static str="DARK_YELLOW";pub fn triplet()->Triplet{DARK_YELLOW_TRIPLET.clone()}}
impl Color for DARK_YELLOW {fn getIndex(&self)->i16{19}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"8080:8080:0".into()}fn getColorName(&self)->String{"DARK_YELLOW".into()}}
// java public static final class GREEN extends KmoColor {
// java public static final String COLOR_NAME = "GREEN";
// java public static final short index = 17;
// java public static final short[] triplet = new short[]{0, 128, 0};
// java public static final String hexString = "0:8080:0";
// java
// java public short getIndex() {
// java return 17;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "0:8080:0";
// java }
// java
// java public String getColorName() {
// java return "GREEN";
// java }
// java }
pub struct GREEN;
static GREEN_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([0, 128, 0])));
impl GREEN {pub const index:i16=17;pub const COLOR_NAME:&'static str="GREEN";pub fn triplet()->Triplet{GREEN_TRIPLET.clone()}}
impl Color for GREEN {fn getIndex(&self)->i16{17}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"0:8080:0".into()}fn getColorName(&self)->String{"GREEN".into()}}
// java public static final class TEAL extends KmoColor {
// java public static final String COLOR_NAME = "TEAL";
// java public static final short index = 21;
// java public static final short index2 = 38;
// java public static final short[] triplet = new short[]{0, 128, 128};
// java public static final String hexString = "0:8080:8080";
// java
// java public short getIndex() {
// java return 21;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "0:8080:8080";
// java }
// java
// java public String getColorName() {
// java return "TEAL";
// java }
// java }
pub struct TEAL;
static TEAL_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([0, 128, 128])));
impl TEAL {pub const index:i16=21;pub const COLOR_NAME:&'static str="TEAL";pub fn triplet()->Triplet{TEAL_TRIPLET.clone()}}
impl Color for TEAL {fn getIndex(&self)->i16{21}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"0:8080:8080".into()}fn getColorName(&self)->String{"TEAL".into()}}
// java public static final class BLUE extends KmoColor {
// java public static final String COLOR_NAME = "BLUE";
// java public static final short index = 12;
// java public static final short index2 = 39;
// java public static final short[] triplet = new short[]{0, 0, 255};
// java public static final String hexString = "0:0:FFFF";
// java
// java public short getIndex() {
// java return 12;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "0:0:FFFF";
// java }
// java
// java public String getColorName() {
// java return "BLUE";
// java }
// java }
pub struct BLUE;
static BLUE_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([0, 0, 255])));
impl BLUE {pub const index:i16=12;pub const COLOR_NAME:&'static str="BLUE";pub fn triplet()->Triplet{BLUE_TRIPLET.clone()}}
impl Color for BLUE {fn getIndex(&self)->i16{12}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"0:0:FFFF".into()}fn getColorName(&self)->String{"BLUE".into()}}
// java public static final class BLUE_GREY extends KmoColor {
// java public static final String COLOR_NAME = "BLUE_GREY";
// java public static final short index = 54;
// java public static final short[] triplet = new short[]{102, 102, 153};
// java public static final String hexString = "6666:6666:9999";
// java
// java public short getIndex() {
// java return 54;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "6666:6666:9999";
// java }
// java
// java public String getColorName() {
// java return "BLUE_GREY";
// java }
// java }
pub struct BLUE_GREY;
static BLUE_GREY_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([102, 102, 153])));
impl BLUE_GREY {pub const index:i16=54;pub const COLOR_NAME:&'static str="BLUE_GREY";pub fn triplet()->Triplet{BLUE_GREY_TRIPLET.clone()}}
impl Color for BLUE_GREY {fn getIndex(&self)->i16{54}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"6666:6666:9999".into()}fn getColorName(&self)->String{"BLUE_GREY".into()}}
// java public static final class GREY_50_PERCENT extends KmoColor {
// java public static final String COLOR_NAME = "GREY_50_PERCENT";
// java public static final short index = 23;
// java public static final short[] triplet = new short[]{128, 128, 128};
// java public static final String hexString = "8080:8080:8080";
// java
// java public short getIndex() {
// java return 23;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "8080:8080:8080";
// java }
// java
// java public String getColorName() {
// java return "GREY_50_PERCENT";
// java }
// java }
pub struct GREY_50_PERCENT;
static GREY_50_PERCENT_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([128, 128, 128])));
impl GREY_50_PERCENT {pub const index:i16=23;pub const COLOR_NAME:&'static str="GREY_50_PERCENT";pub fn triplet()->Triplet{GREY_50_PERCENT_TRIPLET.clone()}}
impl Color for GREY_50_PERCENT {fn getIndex(&self)->i16{23}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"8080:8080:8080".into()}fn getColorName(&self)->String{"GREY_50_PERCENT".into()}}
// java public static final class RED extends KmoColor {
// java public static final String COLOR_NAME = "RED";
// java public static final short index = 10;
// java public static final short[] triplet = new short[]{255, 0, 0};
// java public static final String hexString = "FFFF:0:0";
// java
// java public short getIndex() {
// java return 10;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "FFFF:0:0";
// java }
// java
// java public String getColorName() {
// java return "RED";
// java }
// java }
pub struct RED;
static RED_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([255, 0, 0])));
impl RED {pub const index:i16=10;pub const COLOR_NAME:&'static str="RED";pub fn triplet()->Triplet{RED_TRIPLET.clone()}}
impl Color for RED {fn getIndex(&self)->i16{10}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"FFFF:0:0".into()}fn getColorName(&self)->String{"RED".into()}}
// java public static final class LIGHT_ORANGE extends KmoColor {
// java public static final String COLOR_NAME = "LIGHT_ORANGE";
// java public static final short index = 52;
// java public static final short[] triplet = new short[]{255, 153, 0};
// java public static final String hexString = "FFFF:9999:0";
// java
// java public short getIndex() {
// java return 52;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "FFFF:9999:0";
// java }
// java
// java public String getColorName() {
// java return "LIGHT_ORANGE";
// java }
// java }
pub struct LIGHT_ORANGE;
static LIGHT_ORANGE_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([255, 153, 0])));
impl LIGHT_ORANGE {pub const index:i16=52;pub const COLOR_NAME:&'static str="LIGHT_ORANGE";pub fn triplet()->Triplet{LIGHT_ORANGE_TRIPLET.clone()}}
impl Color for LIGHT_ORANGE {fn getIndex(&self)->i16{52}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"FFFF:9999:0".into()}fn getColorName(&self)->String{"LIGHT_ORANGE".into()}}
// java public static final class LIME extends KmoColor {
// java public static final String COLOR_NAME = "LIME";
// java public static final short index = 50;
// java public static final short[] triplet = new short[]{153, 204, 0};
// java public static final String hexString = "9999:CCCC:0";
// java
// java public short getIndex() {
// java return 50;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "9999:CCCC:0";
// java }
// java
// java public String getColorName() {
// java return "LIME";
// java }
// java }
pub struct LIME;
static LIME_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([153, 204, 0])));
impl LIME {pub const index:i16=50;pub const COLOR_NAME:&'static str="LIME";pub fn triplet()->Triplet{LIME_TRIPLET.clone()}}
impl Color for LIME {fn getIndex(&self)->i16{50}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"9999:CCCC:0".into()}fn getColorName(&self)->String{"LIME".into()}}
// java public static final class SEA_GREEN extends KmoColor {
// java public static final String COLOR_NAME = "SEA_GREEN";
// java public static final short index = 57;
// java public static final short[] triplet = new short[]{51, 153, 102};
// java public static final String hexString = "3333:9999:6666";
// java
// java public short getIndex() {
// java return 57;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "3333:9999:6666";
// java }
// java
// java public String getColorName() {
// java return "SEA_GREEN";
// java }
// java }
pub struct SEA_GREEN;
static SEA_GREEN_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([51, 153, 102])));
impl SEA_GREEN {pub const index:i16=57;pub const COLOR_NAME:&'static str="SEA_GREEN";pub fn triplet()->Triplet{SEA_GREEN_TRIPLET.clone()}}
impl Color for SEA_GREEN {fn getIndex(&self)->i16{57}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"3333:9999:6666".into()}fn getColorName(&self)->String{"SEA_GREEN".into()}}
// java public static final class AQUA extends KmoColor {
// java public static final String COLOR_NAME = "AQUA";
// java public static final short index = 49;
// java public static final short[] triplet = new short[]{51, 204, 204};
// java public static final String hexString = "3333:CCCC:CCCC";
// java
// java public short getIndex() {
// java return 49;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "3333:CCCC:CCCC";
// java }
// java
// java public String getColorName() {
// java return "AQUA";
// java }
// java }
pub struct AQUA;
static AQUA_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([51, 204, 204])));
impl AQUA {pub const index:i16=49;pub const COLOR_NAME:&'static str="AQUA";pub fn triplet()->Triplet{AQUA_TRIPLET.clone()}}
impl Color for AQUA {fn getIndex(&self)->i16{49}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"3333:CCCC:CCCC".into()}fn getColorName(&self)->String{"AQUA".into()}}
// java public static final class LIGHT_BLUE extends KmoColor {
// java public static final String COLOR_NAME = "LIGHT_BLUE";
// java public static final short index = 48;
// java public static final short[] triplet = new short[]{51, 102, 255};
// java public static final String hexString = "3333:6666:FFFF";
// java
// java public short getIndex() {
// java return 48;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "3333:6666:FFFF";
// java }
// java
// java public String getColorName() {
// java return "LIGHT_BLUE";
// java }
// java }
pub struct LIGHT_BLUE;
static LIGHT_BLUE_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([51, 102, 255])));
impl LIGHT_BLUE {pub const index:i16=48;pub const COLOR_NAME:&'static str="LIGHT_BLUE";pub fn triplet()->Triplet{LIGHT_BLUE_TRIPLET.clone()}}
impl Color for LIGHT_BLUE {fn getIndex(&self)->i16{48}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"3333:6666:FFFF".into()}fn getColorName(&self)->String{"LIGHT_BLUE".into()}}
// java public static final class VIOLET extends KmoColor {
// java public static final String COLOR_NAME = "VIOLET";
// java public static final short index = 20;
// java public static final short index2 = 36;
// java public static final short[] triplet = new short[]{128, 0, 128};
// java public static final String hexString = "8080:0:8080";
// java
// java public short getIndex() {
// java return 20;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "8080:0:8080";
// java }
// java
// java public String getColorName() {
// java return "VIOLET";
// java }
// java }
pub struct VIOLET;
static VIOLET_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([128, 0, 128])));
impl VIOLET {pub const index:i16=20;pub const COLOR_NAME:&'static str="VIOLET";pub fn triplet()->Triplet{VIOLET_TRIPLET.clone()}}
impl Color for VIOLET {fn getIndex(&self)->i16{20}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"8080:0:8080".into()}fn getColorName(&self)->String{"VIOLET".into()}}
// java public static final class GREY_40_PERCENT extends KmoColor {
// java public static final String COLOR_NAME = "GREY_40_PERCENT";
// java public static final short index = 55;
// java public static final short[] triplet = new short[]{150, 150, 150};
// java public static final String hexString = "9696:9696:9696";
// java
// java public short getIndex() {
// java return 55;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "9696:9696:9696";
// java }
// java
// java public String getColorName() {
// java return "GREY_40_PERCENT";
// java }
// java }
pub struct GREY_40_PERCENT;
static GREY_40_PERCENT_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([150, 150, 150])));
impl GREY_40_PERCENT {pub const index:i16=55;pub const COLOR_NAME:&'static str="GREY_40_PERCENT";pub fn triplet()->Triplet{GREY_40_PERCENT_TRIPLET.clone()}}
impl Color for GREY_40_PERCENT {fn getIndex(&self)->i16{55}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"9696:9696:9696".into()}fn getColorName(&self)->String{"GREY_40_PERCENT".into()}}
// java public static final class PINK extends KmoColor {
// java public static final String COLOR_NAME = "PINK";
// java public static final short index = 14;
// java public static final short index2 = 33;
// java public static final short[] triplet = new short[]{255, 0, 255};
// java public static final String hexString = "FFFF:0:FFFF";
// java
// java public short getIndex() {
// java return 14;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "FFFF:0:FFFF";
// java }
// java
// java public String getColorName() {
// java return "PINK";
// java }
// java }
pub struct PINK;
static PINK_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([255, 0, 255])));
impl PINK {pub const index:i16=14;pub const COLOR_NAME:&'static str="PINK";pub fn triplet()->Triplet{PINK_TRIPLET.clone()}}
impl Color for PINK {fn getIndex(&self)->i16{14}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"FFFF:0:FFFF".into()}fn getColorName(&self)->String{"PINK".into()}}
// java public static final class GOLD extends KmoColor {
// java public static final String COLOR_NAME = "GOLD";
// java public static final short index = 51;
// java public static final short[] triplet = new short[]{255, 204, 0};
// java public static final String hexString = "FFFF:CCCC:0";
// java
// java public short getIndex() {
// java return 51;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "FFFF:CCCC:0";
// java }
// java
// java public String getColorName() {
// java return "GOLD";
// java }
// java }
pub struct GOLD;
static GOLD_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([255, 204, 0])));
impl GOLD {pub const index:i16=51;pub const COLOR_NAME:&'static str="GOLD";pub fn triplet()->Triplet{GOLD_TRIPLET.clone()}}
impl Color for GOLD {fn getIndex(&self)->i16{51}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"FFFF:CCCC:0".into()}fn getColorName(&self)->String{"GOLD".into()}}
// java public static final class YELLOW extends KmoColor {
// java public static final String COLOR_NAME = "YELLOW";
// java public static final short index = 13;
// java public static final short index2 = 34;
// java public static final short[] triplet = new short[]{255, 255, 0};
// java public static final String hexString = "FFFF:FFFF:0";
// java
// java public short getIndex() {
// java return 13;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "FFFF:FFFF:0";
// java }
// java
// java public String getColorName() {
// java return "YELLOW";
// java }
// java }
pub struct YELLOW;
static YELLOW_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([255, 255, 0])));
impl YELLOW {pub const index:i16=13;pub const COLOR_NAME:&'static str="YELLOW";pub fn triplet()->Triplet{YELLOW_TRIPLET.clone()}}
impl Color for YELLOW {fn getIndex(&self)->i16{13}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"FFFF:FFFF:0".into()}fn getColorName(&self)->String{"YELLOW".into()}}
// java public static final class BRIGHT_GREEN extends KmoColor {
// java public static final String COLOR_NAME = "BRIGHT_GREEN";
// java public static final short index = 11;
// java public static final short index2 = 35;
// java public static final short[] triplet = new short[]{0, 255, 0};
// java public static final String hexString = "0:FFFF:0";
// java
// java public short getIndex() {
// java return 11;
// java }
// java
// java public String getHexString() {
// java return "0:FFFF:0";
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getColorName() {
// java return "BRIGHT_GREEN";
// java }
// java }
pub struct BRIGHT_GREEN;
static BRIGHT_GREEN_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([0, 255, 0])));
impl BRIGHT_GREEN {pub const index:i16=11;pub const COLOR_NAME:&'static str="BRIGHT_GREEN";pub fn triplet()->Triplet{BRIGHT_GREEN_TRIPLET.clone()}}
impl Color for BRIGHT_GREEN {fn getIndex(&self)->i16{11}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"0:FFFF:0".into()}fn getColorName(&self)->String{"BRIGHT_GREEN".into()}}
// java public static final class TURQUOISE extends KmoColor {
// java public static final String COLOR_NAME = "TURQUOISE";
// java public static final short index = 15;
// java public static final short index2 = 35;
// java public static final short[] triplet = new short[]{0, 255, 255};
// java public static final String hexString = "0:FFFF:FFFF";
// java
// java public short getIndex() {
// java return 15;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "0:FFFF:FFFF";
// java }
// java
// java public String getColorName() {
// java return "TURQUOISE";
// java }
// java }
pub struct TURQUOISE;
static TURQUOISE_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([0, 255, 255])));
impl TURQUOISE {pub const index:i16=15;pub const COLOR_NAME:&'static str="TURQUOISE";pub fn triplet()->Triplet{TURQUOISE_TRIPLET.clone()}}
impl Color for TURQUOISE {fn getIndex(&self)->i16{15}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"0:FFFF:FFFF".into()}fn getColorName(&self)->String{"TURQUOISE".into()}}
// java public static final class SKY_BLUE extends KmoColor {
// java public static final String COLOR_NAME = "SKY_BLUE";
// java public static final short index = 40;
// java public static final short[] triplet = new short[]{0, 204, 255};
// java public static final String hexString = "0:CCCC:FFFF";
// java
// java public short getIndex() {
// java return 40;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "0:CCCC:FFFF";
// java }
// java
// java public String getColorName() {
// java return "SKY_BLUE";
// java }
// java }
pub struct SKY_BLUE;
static SKY_BLUE_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([0, 204, 255])));
impl SKY_BLUE {pub const index:i16=40;pub const COLOR_NAME:&'static str="SKY_BLUE";pub fn triplet()->Triplet{SKY_BLUE_TRIPLET.clone()}}
impl Color for SKY_BLUE {fn getIndex(&self)->i16{40}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"0:CCCC:FFFF".into()}fn getColorName(&self)->String{"SKY_BLUE".into()}}
// java public static final class PLUM extends KmoColor {
// java public static final String COLOR_NAME = "PLUM";
// java public static final short index = 61;
// java public static final short index2 = 25;
// java public static final short[] triplet = new short[]{153, 51, 102};
// java public static final String hexString = "9999:3333:6666";
// java
// java public short getIndex() {
// java return 61;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "9999:3333:6666";
// java }
// java
// java public String getColorName() {
// java return "PLUM";
// java }
// java }
pub struct PLUM;
static PLUM_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([153, 51, 102])));
impl PLUM {pub const index:i16=61;pub const COLOR_NAME:&'static str="PLUM";pub fn triplet()->Triplet{PLUM_TRIPLET.clone()}}
impl Color for PLUM {fn getIndex(&self)->i16{61}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"9999:3333:6666".into()}fn getColorName(&self)->String{"PLUM".into()}}
// java public static final class GREY_25_PERCENT extends KmoColor {
// java public static final String COLOR_NAME = "GREY_25_PERCENT";
// java public static final short index = 22;
// java public static final short[] triplet = new short[]{192, 192, 192};
// java public static final String hexString = "C0C0:C0C0:C0C0";
// java
// java public short getIndex() {
// java return 22;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "C0C0:C0C0:C0C0";
// java }
// java
// java public String getColorName() {
// java return "GREY_25_PERCENT";
// java }
// java }
pub struct GREY_25_PERCENT;
static GREY_25_PERCENT_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([192, 192, 192])));
impl GREY_25_PERCENT {pub const index:i16=22;pub const COLOR_NAME:&'static str="GREY_25_PERCENT";pub fn triplet()->Triplet{GREY_25_PERCENT_TRIPLET.clone()}}
impl Color for GREY_25_PERCENT {fn getIndex(&self)->i16{22}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"C0C0:C0C0:C0C0".into()}fn getColorName(&self)->String{"GREY_25_PERCENT".into()}}
// java public static final class ROSE extends KmoColor {
// java public static final String COLOR_NAME = "ROSE";
// java public static final short index = 45;
// java public static final short[] triplet = new short[]{255, 153, 204};
// java public static final String hexString = "FFFF:9999:CCCC";
// java
// java public short getIndex() {
// java return 45;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "FFFF:9999:CCCC";
// java }
// java
// java public String getColorName() {
// java return "ROSE";
// java }
// java }
pub struct ROSE;
static ROSE_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([255, 153, 204])));
impl ROSE {pub const index:i16=45;pub const COLOR_NAME:&'static str="ROSE";pub fn triplet()->Triplet{ROSE_TRIPLET.clone()}}
impl Color for ROSE {fn getIndex(&self)->i16{45}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"FFFF:9999:CCCC".into()}fn getColorName(&self)->String{"ROSE".into()}}
// java public static final class TAN extends KmoColor {
// java public static final String COLOR_NAME = "TAN";
// java public static final short index = 47;
// java public static final short[] triplet = new short[]{255, 204, 153};
// java public static final String hexString = "FFFF:CCCC:9999";
// java
// java public short getIndex() {
// java return 47;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "FFFF:CCCC:9999";
// java }
// java
// java public String getColorName() {
// java return "TAN";
// java }
// java }
pub struct TAN;
static TAN_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([255, 204, 153])));
impl TAN {pub const index:i16=47;pub const COLOR_NAME:&'static str="TAN";pub fn triplet()->Triplet{TAN_TRIPLET.clone()}}
impl Color for TAN {fn getIndex(&self)->i16{47}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"FFFF:CCCC:9999".into()}fn getColorName(&self)->String{"TAN".into()}}
// java public static final class LIGHT_YELLOW extends KmoColor {
// java public static final String COLOR_NAME = "LIGHT_YELLOW";
// java public static final short index = 43;
// java public static final short[] triplet = new short[]{255, 255, 153};
// java public static final String hexString = "FFFF:FFFF:9999";
// java
// java public short getIndex() {
// java return 43;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "FFFF:FFFF:9999";
// java }
// java
// java public String getColorName() {
// java return "LIGHT_YELLOW";
// java }
// java }
pub struct LIGHT_YELLOW;
static LIGHT_YELLOW_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([255, 255, 153])));
impl LIGHT_YELLOW {pub const index:i16=43;pub const COLOR_NAME:&'static str="LIGHT_YELLOW";pub fn triplet()->Triplet{LIGHT_YELLOW_TRIPLET.clone()}}
impl Color for LIGHT_YELLOW {fn getIndex(&self)->i16{43}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"FFFF:FFFF:9999".into()}fn getColorName(&self)->String{"LIGHT_YELLOW".into()}}
// java public static final class LIGHT_GREEN extends KmoColor {
// java public static final String COLOR_NAME = "LIGHT_GREEN";
// java public static final short index = 42;
// java public static final short[] triplet = new short[]{204, 255, 204};
// java public static final String hexString = "CCCC:FFFF:CCCC";
// java
// java public short getIndex() {
// java return 42;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "CCCC:FFFF:CCCC";
// java }
// java
// java public String getColorName() {
// java return "LIGHT_GREEN";
// java }
// java }
pub struct LIGHT_GREEN;
static LIGHT_GREEN_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([204, 255, 204])));
impl LIGHT_GREEN {pub const index:i16=42;pub const COLOR_NAME:&'static str="LIGHT_GREEN";pub fn triplet()->Triplet{LIGHT_GREEN_TRIPLET.clone()}}
impl Color for LIGHT_GREEN {fn getIndex(&self)->i16{42}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"CCCC:FFFF:CCCC".into()}fn getColorName(&self)->String{"LIGHT_GREEN".into()}}
// java public static final class LIGHT_TURQUOISE extends KmoColor {
// java public static final String COLOR_NAME = "LIGHT_TURQUOISE";
// java public static final short index = 41;
// java public static final short index2 = 27;
// java public static final short[] triplet = new short[]{204, 255, 255};
// java public static final String hexString = "CCCC:FFFF:FFFF";
// java
// java public short getIndex() {
// java return 41;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "CCCC:FFFF:FFFF";
// java }
// java
// java public String getColorName() {
// java return "LIGHT_TURQUOISE";
// java }
// java }
pub struct LIGHT_TURQUOISE;
static LIGHT_TURQUOISE_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([204, 255, 255])));
impl LIGHT_TURQUOISE {pub const index:i16=41;pub const COLOR_NAME:&'static str="LIGHT_TURQUOISE";pub fn triplet()->Triplet{LIGHT_TURQUOISE_TRIPLET.clone()}}
impl Color for LIGHT_TURQUOISE {fn getIndex(&self)->i16{41}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"CCCC:FFFF:FFFF".into()}fn getColorName(&self)->String{"LIGHT_TURQUOISE".into()}}
// java public static final class PALE_BLUE extends KmoColor {
// java public static final String COLOR_NAME = "PALE_BLUE";
// java public static final short index = 44;
// java public static final short[] triplet = new short[]{153, 204, 255};
// java public static final String hexString = "9999:CCCC:FFFF";
// java
// java public short getIndex() {
// java return 44;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "9999:CCCC:FFFF";
// java }
// java
// java public String getColorName() {
// java return "PALE_BLUE";
// java }
// java }
pub struct PALE_BLUE;
static PALE_BLUE_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([153, 204, 255])));
impl PALE_BLUE {pub const index:i16=44;pub const COLOR_NAME:&'static str="PALE_BLUE";pub fn triplet()->Triplet{PALE_BLUE_TRIPLET.clone()}}
impl Color for PALE_BLUE {fn getIndex(&self)->i16{44}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"9999:CCCC:FFFF".into()}fn getColorName(&self)->String{"PALE_BLUE".into()}}
// java public static final class LAVENDER extends KmoColor {
// java public static final String COLOR_NAME = "COLOR_NAME";
// java public static final short index = 46;
// java public static final short[] triplet = new short[]{204, 153, 255};
// java public static final String hexString = "CCCC:9999:FFFF";
// java
// java public short getIndex() {
// java return 46;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "CCCC:9999:FFFF";
// java }
// java
// java public String getColorName() {
// java return "COLOR_NAME";
// java }
// java }
pub struct LAVENDER;
static LAVENDER_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([204, 153, 255])));
impl LAVENDER {pub const index:i16=46;pub const COLOR_NAME:&'static str="COLOR_NAME";pub fn triplet()->Triplet{LAVENDER_TRIPLET.clone()}}
impl Color for LAVENDER {fn getIndex(&self)->i16{46}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"CCCC:9999:FFFF".into()}fn getColorName(&self)->String{"COLOR_NAME".into()}}
// java public static final class WHITE extends KmoColor {
// java public static final String COLOR_NAME = "WHITE";
// java public static final short index = 9;
// java public static final short[] triplet = new short[]{255, 255, 255};
// java public static final String hexString = "FFFF:FFFF:FFFF";
// java
// java public short getIndex() {
// java return 9;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "FFFF:FFFF:FFFF";
// java }
// java
// java public String getColorName() {
// java return "WHITE";
// java }
// java }
pub struct WHITE;
static WHITE_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([255, 255, 255])));
impl WHITE {pub const index:i16=9;pub const COLOR_NAME:&'static str="WHITE";pub fn triplet()->Triplet{WHITE_TRIPLET.clone()}}
impl Color for WHITE {fn getIndex(&self)->i16{9}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"FFFF:FFFF:FFFF".into()}fn getColorName(&self)->String{"WHITE".into()}}
// java public static final class CORNFLOWER_BLUE extends KmoColor {
// java public static final String COLOR_NAME = "CORNFLOWER_BLUE";
// java public static final short index = 24;
// java public static final short[] triplet = new short[]{153, 153, 255};
// java public static final String hexString = "9999:9999:FFFF";
// java
// java public short getIndex() {
// java return 24;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "9999:9999:FFFF";
// java }
// java
// java public String getColorName() {
// java return "CORNFLOWER_BLUE";
// java }
// java }
pub struct CORNFLOWER_BLUE;
static CORNFLOWER_BLUE_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([153, 153, 255])));
impl CORNFLOWER_BLUE {pub const index:i16=24;pub const COLOR_NAME:&'static str="CORNFLOWER_BLUE";pub fn triplet()->Triplet{CORNFLOWER_BLUE_TRIPLET.clone()}}
impl Color for CORNFLOWER_BLUE {fn getIndex(&self)->i16{24}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"9999:9999:FFFF".into()}fn getColorName(&self)->String{"CORNFLOWER_BLUE".into()}}
// java public static final class LEMON_CHIFFON extends KmoColor {
// java public static final String COLOR_NAME = "LEMON_CHIFFON";
// java public static final short index = 26;
// java public static final short[] triplet = new short[]{255, 255, 204};
// java public static final String hexString = "FFFF:FFFF:CCCC";
// java
// java public short getIndex() {
// java return 26;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "FFFF:FFFF:CCCC";
// java }
// java
// java public String getColorName() {
// java return "LEMON_CHIFFON";
// java }
// java }
pub struct LEMON_CHIFFON;
static LEMON_CHIFFON_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([255, 255, 204])));
impl LEMON_CHIFFON {pub const index:i16=26;pub const COLOR_NAME:&'static str="LEMON_CHIFFON";pub fn triplet()->Triplet{LEMON_CHIFFON_TRIPLET.clone()}}
impl Color for LEMON_CHIFFON {fn getIndex(&self)->i16{26}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"FFFF:FFFF:CCCC".into()}fn getColorName(&self)->String{"LEMON_CHIFFON".into()}}
// java public static final class MAROON extends KmoColor {
// java public static final String COLOR_NAME = "MAROON";
// java public static final short index = 25;
// java public static final short[] triplet = new short[]{127, 0, 0};
// java public static final String hexString = "8000:0:0";
// java
// java public short getIndex() {
// java return 25;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "8000:0:0";
// java }
// java
// java public String getColorName() {
// java return "MAROON";
// java }
// java }
pub struct MAROON;
static MAROON_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([127, 0, 0])));
impl MAROON {pub const index:i16=25;pub const COLOR_NAME:&'static str="MAROON";pub fn triplet()->Triplet{MAROON_TRIPLET.clone()}}
impl Color for MAROON {fn getIndex(&self)->i16{25}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"8000:0:0".into()}fn getColorName(&self)->String{"MAROON".into()}}
// java public static final class ORCHID extends KmoColor {
// java public static final String COLOR_NAME = "ORCHID";
// java public static final short index = 28;
// java public static final short[] triplet = new short[]{102, 0, 102};
// java public static final String hexString = "6666:0:6666";
// java
// java public short getIndex() {
// java return 28;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "6666:0:6666";
// java }
// java
// java public String getColorName() {
// java return "ORCHID";
// java }
// java }
pub struct ORCHID;
static ORCHID_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([102, 0, 102])));
impl ORCHID {pub const index:i16=28;pub const COLOR_NAME:&'static str="ORCHID";pub fn triplet()->Triplet{ORCHID_TRIPLET.clone()}}
impl Color for ORCHID {fn getIndex(&self)->i16{28}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"6666:0:6666".into()}fn getColorName(&self)->String{"ORCHID".into()}}
// java public static final class CORAL extends KmoColor {
// java public static final String COLOR_NAME = "CORAL";
// java public static final short index = 29;
// java public static final short[] triplet = new short[]{255, 128, 128};
// java public static final String hexString = "FFFF:8080:8080";
// java
// java public short getIndex() {
// java return 29;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "FFFF:8080:8080";
// java }
// java
// java public String getColorName() {
// java return "CORAL";
// java }
// java }
pub struct CORAL;
static CORAL_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([255, 128, 128])));
impl CORAL {pub const index:i16=29;pub const COLOR_NAME:&'static str="CORAL";pub fn triplet()->Triplet{CORAL_TRIPLET.clone()}}
impl Color for CORAL {fn getIndex(&self)->i16{29}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"FFFF:8080:8080".into()}fn getColorName(&self)->String{"CORAL".into()}}
// java public static final class ROYAL_BLUE extends KmoColor {
// java public static final String COLOR_NAME = "ROYAL_BLUE";
// java public static final short index = 30;
// java public static final short[] triplet = new short[]{0, 102, 204};
// java public static final String hexString = "0:6666:CCCC";
// java
// java public short getIndex() {
// java return 30;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "0:6666:CCCC";
// java }
// java
// java public String getColorName() {
// java return "ROYAL_BLUE";
// java }
// java }
pub struct ROYAL_BLUE;
static ROYAL_BLUE_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([0, 102, 204])));
impl ROYAL_BLUE {pub const index:i16=30;pub const COLOR_NAME:&'static str="ROYAL_BLUE";pub fn triplet()->Triplet{ROYAL_BLUE_TRIPLET.clone()}}
impl Color for ROYAL_BLUE {fn getIndex(&self)->i16{30}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"0:6666:CCCC".into()}fn getColorName(&self)->String{"ROYAL_BLUE".into()}}
// java public static final class LIGHT_CORNFLOWER_BLUE extends KmoColor {
// java public static final String COLOR_NAME = "LIGHT_CORNFLOWER_BLUE";
// java public static final short index = 31;
// java public static final short[] triplet = new short[]{204, 204, 255};
// java public static final String hexString = "CCCC:CCCC:FFFF";
// java
// java public short getIndex() {
// java return 31;
// java }
// java
// java public short[] getTriplet() {
// java return triplet;
// java }
// java
// java public String getHexString() {
// java return "CCCC:CCCC:FFFF";
// java }
// java
// java public String getColorName() {
// java return "LIGHT_CORNFLOWER_BLUE";
// java }
// java }
pub struct LIGHT_CORNFLOWER_BLUE;
static LIGHT_CORNFLOWER_BLUE_TRIPLET:LazyLock<Triplet>=LazyLock::new(||Rc::new(RefCell::new([204, 204, 255])));
impl LIGHT_CORNFLOWER_BLUE {pub const index:i16=31;pub const COLOR_NAME:&'static str="LIGHT_CORNFLOWER_BLUE";pub fn triplet()->Triplet{LIGHT_CORNFLOWER_BLUE_TRIPLET.clone()}}
impl Color for LIGHT_CORNFLOWER_BLUE {fn getIndex(&self)->i16{31}fn getTriplet(&self)->Triplet{Self::triplet()}fn getHexString(&self)->String{"CCCC:CCCC:FFFF".into()}fn getColorName(&self)->String{"LIGHT_CORNFLOWER_BLUE".into()}}
// java public static final class AUTOMATIC extends KmoColor {
// java private static KmoColor instance = new AUTOMATIC();
// java public static final short index = 64;
// java public static final short Index2 = 65;
// java
// java public short getIndex() {
// java return 64;
// java }
// java
// java public short[] getTriplet() {
// java return KmoColor.BLACK.triplet;
// java }
// java
// java public String getHexString() {
// java return "0:0:0";
// java }
// java
// java public static KmoColor getInstance() {
// java return instance;
// java }
// java
// java public String getColorName() {
// java return "BLACK";
// java }
// java }
pub struct AUTOMATIC;
static AUTOMATIC_INSTANCE:LazyLock<Rc<dyn Color>>=LazyLock::new(||Rc::new(AUTOMATIC));
impl AUTOMATIC {pub const index:i16=64;pub const Index2:i16=65;pub fn getInstance()->Rc<dyn Color>{AUTOMATIC_INSTANCE.clone()}}
impl Color for AUTOMATIC {fn getIndex(&self)->i16{64}fn getColorName(&self)->String{"BLACK".into()}}
