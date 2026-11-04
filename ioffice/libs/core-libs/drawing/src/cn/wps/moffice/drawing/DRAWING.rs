#![allow(non_snake_case, non_upper_case_globals, dead_code, non_camel_case_types)]

// Java: public class DRAWING
pub struct DRAWING {}

// Java: public class DRAWING
impl DRAWING {
    // Java: DRAWING()
    pub fn new() -> Self {
      // Java: super();
      DRAWING {}
    }
}

// Java: public class COLOR
pub mod COLOR {
    // Java field: int SCRGB
    pub const SCRGB: i32 = 0;
    // Java field: int SRGB
    pub const SRGB: i32 = 1;
    // Java field: int HSL
    pub const HSL: i32 = 2;
    // Java field: int SYSTEM
    pub const SYSTEM: i32 = 3;
    // Java field: int SCHEME
    pub const SCHEME: i32 = 4;
    // Java field: int PRESET
    pub const PRESET: i32 = 5;
    // Java field: int SCHEMECLR_FIRST
    pub const SCHEMECLR_FIRST: i32 = 0;
    // Java field: int SCHEMECLR_DK1
    pub const SCHEMECLR_DK1: i32 = 0;
    // Java field: int SCHEMECLR_LT1
    pub const SCHEMECLR_LT1: i32 = 1;
    // Java field: int SCHEMECLR_DK2
    pub const SCHEMECLR_DK2: i32 = 2;
    // Java field: int SCHEMECLR_LT2
    pub const SCHEMECLR_LT2: i32 = 3;
    // Java field: int SCHEMECLR_ACCENT1
    pub const SCHEMECLR_ACCENT1: i32 = 4;
    // Java field: int SCHEMECLR_ACCENT2
    pub const SCHEMECLR_ACCENT2: i32 = 5;
    // Java field: int SCHEMECLR_ACCENT3
    pub const SCHEMECLR_ACCENT3: i32 = 6;
    // Java field: int SCHEMECLR_ACCENT4
    pub const SCHEMECLR_ACCENT4: i32 = 7;
    // Java field: int SCHEMECLR_ACCENT5
    pub const SCHEMECLR_ACCENT5: i32 = 8;
    // Java field: int SCHEMECLR_ACCENT6
    pub const SCHEMECLR_ACCENT6: i32 = 9;
    // Java field: int SCHEMECLR_HLINK
    pub const SCHEMECLR_HLINK: i32 = 10;
    // Java field: int SCHEMECLR_FOL_HLINK
    pub const SCHEMECLR_FOL_HLINK: i32 = 11;
    // Java field: int SCHEMECLR_BG1
    pub const SCHEMECLR_BG1: i32 = 12;
    // Java field: int SCHEMECLR_BG2
    pub const SCHEMECLR_BG2: i32 = 13;
    // Java field: int SCHEMECLR_PH_CLR
    pub const SCHEMECLR_PH_CLR: i32 = 14;
    // Java field: int SCHEMECLR_TX1
    pub const SCHEMECLR_TX1: i32 = 15;
    // Java field: int SCHEMECLR_TX2
    pub const SCHEMECLR_TX2: i32 = 16;
    // Java field: int SCHEMECLR_LAST
    pub const SCHEMECLR_LAST: i32 = 16;
    // Java field: int SCHEMECLR_COUNT
    pub const SCHEMECLR_COUNT: i32 = 17;
    // Java field: int SYSCLR_3D_DK_SHADOW
    pub const SYSCLR_3D_DK_SHADOW: i32 = 0;
    // Java field: int SYSCLR_3D_LIGHT
    pub const SYSCLR_3D_LIGHT: i32 = 1;
    // Java field: int SYSCLR_ACTIVE_BORDER
    pub const SYSCLR_ACTIVE_BORDER: i32 = 2;
    // Java field: int SYSCLR_ACTIVE_CAPTION
    pub const SYSCLR_ACTIVE_CAPTION: i32 = 3;
    // Java field: int SYSCLR_APP_WORKSPACE
    pub const SYSCLR_APP_WORKSPACE: i32 = 4;
    // Java field: int SYSCLR_BACKGROUND
    pub const SYSCLR_BACKGROUND: i32 = 5;
    // Java field: int SYSCLR_BTN_FACE
    pub const SYSCLR_BTN_FACE: i32 = 6;
    // Java field: int SYSCLR_BTN_HIGHLIGHT
    pub const SYSCLR_BTN_HIGHLIGHT: i32 = 7;
    // Java field: int SYSCLR_BTN_SHADOW
    pub const SYSCLR_BTN_SHADOW: i32 = 8;
    // Java field: int SYSCLR_BTN_TEXT
    pub const SYSCLR_BTN_TEXT: i32 = 9;
    // Java field: int SYSCLR_CAPTION_TEXT
    pub const SYSCLR_CAPTION_TEXT: i32 = 10;
    // Java field: int SYSCLR_GRADIENT_ACTIVE_CAPTION
    pub const SYSCLR_GRADIENT_ACTIVE_CAPTION: i32 = 11;
    // Java field: int SYSCLR_GRADIENT_INACTIVE_CAPTION
    pub const SYSCLR_GRADIENT_INACTIVE_CAPTION: i32 = 12;
    // Java field: int SYSCLR_GRAY_TEXT
    pub const SYSCLR_GRAY_TEXT: i32 = 13;
    // Java field: int SYSCLR_HIGHLIGHT
    pub const SYSCLR_HIGHLIGHT: i32 = 14;
    // Java field: int SYSCLR_HIGHLIGHT_TEXT
    pub const SYSCLR_HIGHLIGHT_TEXT: i32 = 15;
    // Java field: int SYSCLR_HOT_LIGHT
    pub const SYSCLR_HOT_LIGHT: i32 = 16;
    // Java field: int SYSCLR_INACTIVE_BORDER
    pub const SYSCLR_INACTIVE_BORDER: i32 = 17;
    // Java field: int SYSCLR_INACTIVE_CAPTION
    pub const SYSCLR_INACTIVE_CAPTION: i32 = 18;
    // Java field: int SYSCLR_INACTIVE_CAPTION_TEXT
    pub const SYSCLR_INACTIVE_CAPTION_TEXT: i32 = 19;
    // Java field: int SYSCLR_INFO_BK
    pub const SYSCLR_INFO_BK: i32 = 20;
    // Java field: int SYSCLR_INFO_TEXT
    pub const SYSCLR_INFO_TEXT: i32 = 21;
    // Java field: int SYSCLR_MENU
    pub const SYSCLR_MENU: i32 = 22;
    // Java field: int SYSCLR_MENU_BAR
    pub const SYSCLR_MENU_BAR: i32 = 23;
    // Java field: int SYSCLR_MENU_HIGHLIGHT
    pub const SYSCLR_MENU_HIGHLIGHT: i32 = 24;
    // Java field: int SYSCLR_MENU_TEXT
    pub const SYSCLR_MENU_TEXT: i32 = 25;
    // Java field: int SYSCLR_SCROLL_BAR
    pub const SYSCLR_SCROLL_BAR: i32 = 26;
    // Java field: int SYSCLR_WINDOW
    pub const SYSCLR_WINDOW: i32 = 27;
    // Java field: int SYSCLR_WINDOW_FRAME
    pub const SYSCLR_WINDOW_FRAME: i32 = 28;
    // Java field: int SYSCLR_WINDOW_TEXT
    pub const SYSCLR_WINDOW_TEXT: i32 = 29;
    // Java field: int PRESETCLR_ALICE_BLUE
    pub const PRESETCLR_ALICE_BLUE: i32 = 0;
    // Java field: int[] PRESETCLR_RGB_ALICE_BLUE
    pub const PRESETCLR_RGB_ALICE_BLUE: [i32; 3] = [240, 248, 255];
    // Java field: int PRESETCLR_ANTIQUE_WHITE
    pub const PRESETCLR_ANTIQUE_WHITE: i32 = 1;
    // Java field: int[] PRESETCLR_RGB_ANTIQUE_WHITE
    pub const PRESETCLR_RGB_ANTIQUE_WHITE: [i32; 3] = [250, 235, 215];
    // Java field: int PRESETCLR_AQUA
    pub const PRESETCLR_AQUA: i32 = 2;
    // Java field: int[] PRESETCLR_RGB_AQUA
    pub const PRESETCLR_RGB_AQUA: [i32; 3] = [0, 255, 255];
    // Java field: int PRESETCLR_AQUAMARINE
    pub const PRESETCLR_AQUAMARINE: i32 = 3;
    // Java field: int[] PRESETCLR_RGB_AQUAMARINE
    pub const PRESETCLR_RGB_AQUAMARINE: [i32; 3] = [127, 255, 212];
    // Java field: int PRESETCLR_AZURE
    pub const PRESETCLR_AZURE: i32 = 4;
    // Java field: int[] PRESETCLR_RGB_AZURE
    pub const PRESETCLR_RGB_AZURE: [i32; 3] = [240, 255, 255];
    // Java field: int PRESETCLR_BEIGE
    pub const PRESETCLR_BEIGE: i32 = 5;
    // Java field: int[] PRESETCLR_RGB_BEIGE
    pub const PRESETCLR_RGB_BEIGE: [i32; 3] = [245, 245, 220];
    // Java field: int PRESETCLR_BISQUE
    pub const PRESETCLR_BISQUE: i32 = 6;
    // Java field: int[] PRESETCLR_RGB_BISQUE
    pub const PRESETCLR_RGB_BISQUE: [i32; 3] = [255, 228, 196];
    // Java field: int PRESETCLR_BLACK
    pub const PRESETCLR_BLACK: i32 = 7;
    // Java field: int[] PRESETCLR_RGB_BLACK
    pub const PRESETCLR_RGB_BLACK: [i32; 3] = [0, 0, 0];
    // Java field: int PRESETCLR_BLANCHED_ALMOND
    pub const PRESETCLR_BLANCHED_ALMOND: i32 = 8;
    // Java field: int[] PRESETCLR_RGB_BLANCHED_ALMOND
    pub const PRESETCLR_RGB_BLANCHED_ALMOND: [i32; 3] = [255, 235, 205];
    // Java field: int PRESETCLR_BLUE
    pub const PRESETCLR_BLUE: i32 = 9;
    // Java field: int[] PRESETCLR_RGB_BLUE
    pub const PRESETCLR_RGB_BLUE: [i32; 3] = [0, 0, 255];
    // Java field: int PRESETCLR_BLUE_VIOLET
    pub const PRESETCLR_BLUE_VIOLET: i32 = 10;
    // Java field: int[] PRESETCLR_RGB_BLUE_VIOLET
    pub const PRESETCLR_RGB_BLUE_VIOLET: [i32; 3] = [138, 43, 226];
    // Java field: int PRESETCLR_BROWN
    pub const PRESETCLR_BROWN: i32 = 11;
    // Java field: int[] PRESETCLR_RGB_BROWN
    pub const PRESETCLR_RGB_BROWN: [i32; 3] = [165, 42, 42];
    // Java field: int PRESETCLR_BURLY_WOOD
    pub const PRESETCLR_BURLY_WOOD: i32 = 12;
    // Java field: int[] PRESETCLR_RGB_BURLY_WOOD
    pub const PRESETCLR_RGB_BURLY_WOOD: [i32; 3] = [222, 184, 135];
    // Java field: int PRESETCLR_CADET_BLUE
    pub const PRESETCLR_CADET_BLUE: i32 = 13;
    // Java field: int[] PRESETCLR_RGB_CADET_BLUE
    pub const PRESETCLR_RGB_CADET_BLUE: [i32; 3] = [95, 158, 160];
    // Java field: int PRESETCLR_CHARTREUSE
    pub const PRESETCLR_CHARTREUSE: i32 = 14;
    // Java field: int[] PRESETCLR_RGB_CHARTREUSE
    pub const PRESETCLR_RGB_CHARTREUSE: [i32; 3] = [127, 255, 0];
    // Java field: int PRESETCLR_CHOCOLATE
    pub const PRESETCLR_CHOCOLATE: i32 = 15;
    // Java field: int[] PRESETCLR_RGB_CHOCOLATE
    pub const PRESETCLR_RGB_CHOCOLATE: [i32; 3] = [210, 105, 30];
    // Java field: int PRESETCLR_CORAL
    pub const PRESETCLR_CORAL: i32 = 16;
    // Java field: int[] PRESETCLR_RGB_CORAL
    pub const PRESETCLR_RGB_CORAL: [i32; 3] = [255, 127, 80];
    // Java field: int PRESETCLR_CORNFLOWER_BLUE
    pub const PRESETCLR_CORNFLOWER_BLUE: i32 = 17;
    // Java field: int[] PRESETCLR_RGB_CORNFLOWER_BLUE
    pub const PRESETCLR_RGB_CORNFLOWER_BLUE: [i32; 3] = [100, 149, 237];
    // Java field: int PRESETCLR_CORNSILK
    pub const PRESETCLR_CORNSILK: i32 = 18;
    // Java field: int[] PRESETCLR_RGB_CORNSILK
    pub const PRESETCLR_RGB_CORNSILK: [i32; 3] = [255, 248, 220];
    // Java field: int PRESETCLR_CRIMSON
    pub const PRESETCLR_CRIMSON: i32 = 19;
    // Java field: int[] PRESETCLR_RGB_CRIMSON
    pub const PRESETCLR_RGB_CRIMSON: [i32; 3] = [220, 20, 60];
    // Java field: int PRESETCLR_CYAN
    pub const PRESETCLR_CYAN: i32 = 20;
    // Java field: int[] PRESETCLR_RGB_CYAN
    pub const PRESETCLR_RGB_CYAN: [i32; 3] = [0, 255, 255];
    // Java field: int PRESETCLR_DARK_BLUE
    pub const PRESETCLR_DARK_BLUE: i32 = 21;
    // Java field: int[] PRESETCLR_RGB_DARK_BLUE
    pub const PRESETCLR_RGB_DARK_BLUE: [i32; 3] = [0, 0, 139];
    // Java field: int PRESETCLR_DARK_CYAN
    pub const PRESETCLR_DARK_CYAN: i32 = 22;
    // Java field: int[] PRESETCLR_RGB_DARK_CYAN
    pub const PRESETCLR_RGB_DARK_CYAN: [i32; 3] = [0, 139, 139];
    // Java field: int PRESETCLR_DARK_GOLDENROD
    pub const PRESETCLR_DARK_GOLDENROD: i32 = 23;
    // Java field: int[] PRESETCLR_RGB_DARK_GOLDENROD
    pub const PRESETCLR_RGB_DARK_GOLDENROD: [i32; 3] = [184, 134, 11];
    // Java field: int PRESETCLR_DARK_GRAY
    pub const PRESETCLR_DARK_GRAY: i32 = 24;
    // Java field: int[] PRESETCLR_RGB_DARK_GRAY
    pub const PRESETCLR_RGB_DARK_GRAY: [i32; 3] = [169, 169, 169];
    // Java field: int PRESETCLR_DARK_GREEN
    pub const PRESETCLR_DARK_GREEN: i32 = 25;
    // Java field: int[] PRESETCLR_RGB_DARK_GREEN
    pub const PRESETCLR_RGB_DARK_GREEN: [i32; 3] = [0, 100, 0];
    // Java field: int PRESETCLR_DARK_GREY
    pub const PRESETCLR_DARK_GREY: i32 = 26;
    // Java field: int[] PRESETCLR_RGB_DARK_GREY
    pub const PRESETCLR_RGB_DARK_GREY: [i32; 3] = [169, 169, 169];
    // Java field: int PRESETCLR_DARK_KHAKI
    pub const PRESETCLR_DARK_KHAKI: i32 = 27;
    // Java field: int[] PRESETCLR_RGB_DARK_KHAKI
    pub const PRESETCLR_RGB_DARK_KHAKI: [i32; 3] = [189, 183, 107];
    // Java field: int PRESETCLR_DARK_MAGENTA
    pub const PRESETCLR_DARK_MAGENTA: i32 = 28;
    // Java field: int[] PRESETCLR_RGB_DARK_MAGENTA
    pub const PRESETCLR_RGB_DARK_MAGENTA: [i32; 3] = [139, 0, 139];
    // Java field: int PRESETCLR_DARK_OLIVE_GREEN
    pub const PRESETCLR_DARK_OLIVE_GREEN: i32 = 29;
    // Java field: int[] PRESETCLR_RGB_DARK_OLIVE_GREEN
    pub const PRESETCLR_RGB_DARK_OLIVE_GREEN: [i32; 3] = [85, 107, 47];
    // Java field: int PRESETCLR_DARK_ORANGE
    pub const PRESETCLR_DARK_ORANGE: i32 = 30;
    // Java field: int[] PRESETCLR_RGB_DARK_ORANGE
    pub const PRESETCLR_RGB_DARK_ORANGE: [i32; 3] = [255, 140, 0];
    // Java field: int PRESETCLR_DARK_ORCHID
    pub const PRESETCLR_DARK_ORCHID: i32 = 31;
    // Java field: int[] PRESETCLR_RGB_DARK_ORCHID
    pub const PRESETCLR_RGB_DARK_ORCHID: [i32; 3] = [153, 50, 204];
    // Java field: int PRESETCLR_DARK_RED
    pub const PRESETCLR_DARK_RED: i32 = 32;
    // Java field: int[] PRESETCLR_RGB_DARK_RED
    pub const PRESETCLR_RGB_DARK_RED: [i32; 3] = [139, 0, 0];
    // Java field: int PRESETCLR_DARK_SALMON
    pub const PRESETCLR_DARK_SALMON: i32 = 33;
    // Java field: int[] PRESETCLR_RGB_DARK_SALMON
    pub const PRESETCLR_RGB_DARK_SALMON: [i32; 3] = [233, 150, 122];
    // Java field: int PRESETCLR_DARK_SEA_GREEN
    pub const PRESETCLR_DARK_SEA_GREEN: i32 = 34;
    // Java field: int[] PRESETCLR_RGB_DARK_SEA_GREEN
    pub const PRESETCLR_RGB_DARK_SEA_GREEN: [i32; 3] = [143, 188, 143];
    // Java field: int PRESETCLR_DARK_SLATE_BLUE
    pub const PRESETCLR_DARK_SLATE_BLUE: i32 = 35;
    // Java field: int[] PRESETCLR_RGB_DARK_SLATE_BLUE
    pub const PRESETCLR_RGB_DARK_SLATE_BLUE: [i32; 3] = [72, 61, 139];
    // Java field: int PRESETCLR_DARK_SLATE_GRAY
    pub const PRESETCLR_DARK_SLATE_GRAY: i32 = 36;
    // Java field: int[] PRESETCLR_RGB_DARK_SLATE_GRAY
    pub const PRESETCLR_RGB_DARK_SLATE_GRAY: [i32; 3] = [47, 79, 79];
    // Java field: int PRESETCLR_DARK_SLATE_GREY
    pub const PRESETCLR_DARK_SLATE_GREY: i32 = 37;
    // Java field: int[] PRESETCLR_RGB_DARK_SLATE_GREY
    pub const PRESETCLR_RGB_DARK_SLATE_GREY: [i32; 3] = [47, 79, 79];
    // Java field: int PRESETCLR_DARK_TURQUOISE
    pub const PRESETCLR_DARK_TURQUOISE: i32 = 38;
    // Java field: int[] PRESETCLR_RGB_DARK_TURQUOISE
    pub const PRESETCLR_RGB_DARK_TURQUOISE: [i32; 3] = [0, 206, 209];
    // Java field: int PRESETCLR_DARK_VIOLET
    pub const PRESETCLR_DARK_VIOLET: i32 = 39;
    // Java field: int[] PRESETCLR_RGB_DARK_VIOLET
    pub const PRESETCLR_RGB_DARK_VIOLET: [i32; 3] = [148, 0, 211];
    // Java field: int PRESETCLR_DEEP_PINK
    pub const PRESETCLR_DEEP_PINK: i32 = 40;
    // Java field: int[] PRESETCLR_RGB_DEEP_PINK
    pub const PRESETCLR_RGB_DEEP_PINK: [i32; 3] = [255, 20, 147];
    // Java field: int PRESETCLR_DEEP_SKY_BLUE
    pub const PRESETCLR_DEEP_SKY_BLUE: i32 = 41;
    // Java field: int[] PRESETCLR_RGB_DEEP_SKY_BLUE
    pub const PRESETCLR_RGB_DEEP_SKY_BLUE: [i32; 3] = [0, 191, 255];
    // Java field: int PRESETCLR_DIM_GRAY
    pub const PRESETCLR_DIM_GRAY: i32 = 42;
    // Java field: int[] PRESETCLR_RGB_DIM_GRAY
    pub const PRESETCLR_RGB_DIM_GRAY: [i32; 3] = [105, 105, 105];
    // Java field: int PRESETCLR_DIM_GREY
    pub const PRESETCLR_DIM_GREY: i32 = 43;
    // Java field: int[] PRESETCLR_RGB_DIM_GREY
    pub const PRESETCLR_RGB_DIM_GREY: [i32; 3] = [105, 105, 105];
    // Java field: int PRESETCLR_DK_BLUE
    pub const PRESETCLR_DK_BLUE: i32 = 44;
    // Java field: int[] PRESETCLR_RGB_DK_BLUE
    pub const PRESETCLR_RGB_DK_BLUE: [i32; 3] = [0, 0, 139];
    // Java field: int PRESETCLR_DK_CYAN
    pub const PRESETCLR_DK_CYAN: i32 = 45;
    // Java field: int[] PRESETCLR_RGB_DK_CYAN
    pub const PRESETCLR_RGB_DK_CYAN: [i32; 3] = [0, 139, 139];
    // Java field: int PRESETCLR_DK_GOLDENROD
    pub const PRESETCLR_DK_GOLDENROD: i32 = 46;
    // Java field: int[] PRESETCLR_RGB_DK_GOLDENROD
    pub const PRESETCLR_RGB_DK_GOLDENROD: [i32; 3] = [184, 134, 11];
    // Java field: int PRESETCLR_DK_GRAY
    pub const PRESETCLR_DK_GRAY: i32 = 47;
    // Java field: int[] PRESETCLR_RGB_DK_GRAY
    pub const PRESETCLR_RGB_DK_GRAY: [i32; 3] = [169, 169, 169];
    // Java field: int PRESETCLR_DK_GREEN
    pub const PRESETCLR_DK_GREEN: i32 = 48;
    // Java field: int[] PRESETCLR_RGB_DK_GREEN
    pub const PRESETCLR_RGB_DK_GREEN: [i32; 3] = [0, 100, 0];
    // Java field: int PRESETCLR_DK_GREY
    pub const PRESETCLR_DK_GREY: i32 = 49;
    // Java field: int[] PRESETCLR_RGB_DK_GREY
    pub const PRESETCLR_RGB_DK_GREY: [i32; 3] = [169, 169, 169];
    // Java field: int PRESETCLR_DK_KHAKI
    pub const PRESETCLR_DK_KHAKI: i32 = 50;
    // Java field: int[] PRESETCLR_RGB_DK_KHAKI
    pub const PRESETCLR_RGB_DK_KHAKI: [i32; 3] = [189, 183, 107];
    // Java field: int PRESETCLR_DK_MAGENTA
    pub const PRESETCLR_DK_MAGENTA: i32 = 51;
    // Java field: int[] PRESETCLR_RGB_DK_MAGENTA
    pub const PRESETCLR_RGB_DK_MAGENTA: [i32; 3] = [139, 0, 139];
    // Java field: int PRESETCLR_DK_OLIVE_GREEN
    pub const PRESETCLR_DK_OLIVE_GREEN: i32 = 52;
    // Java field: int[] PRESETCLR_RGB_DK_OLIVE_GREEN
    pub const PRESETCLR_RGB_DK_OLIVE_GREEN: [i32; 3] = [85, 107, 47];
    // Java field: int PRESETCLR_DK_ORANGE
    pub const PRESETCLR_DK_ORANGE: i32 = 53;
    // Java field: int[] PRESETCLR_RGB_DK_ORANGE
    pub const PRESETCLR_RGB_DK_ORANGE: [i32; 3] = [255, 140, 0];
    // Java field: int PRESETCLR_DK_ORCHID
    pub const PRESETCLR_DK_ORCHID: i32 = 54;
    // Java field: int[] PRESETCLR_RGB_DK_ORCHID
    pub const PRESETCLR_RGB_DK_ORCHID: [i32; 3] = [153, 50, 204];
    // Java field: int PRESETCLR_DK_RED
    pub const PRESETCLR_DK_RED: i32 = 55;
    // Java field: int[] PRESETCLR_RGB_DK_RED
    pub const PRESETCLR_RGB_DK_RED: [i32; 3] = [139, 0, 0];
    // Java field: int PRESETCLR_DK_SALMON
    pub const PRESETCLR_DK_SALMON: i32 = 56;
    // Java field: int[] PRESETCLR_RGB_DK_SALMON
    pub const PRESETCLR_RGB_DK_SALMON: [i32; 3] = [233, 150, 122];
    // Java field: int PRESETCLR_DK_SEA_GREEN
    pub const PRESETCLR_DK_SEA_GREEN: i32 = 57;
    // Java field: int[] PRESETCLR_RGB_DK_SEA_GREEN
    pub const PRESETCLR_RGB_DK_SEA_GREEN: [i32; 3] = [143, 188, 139];
    // Java field: int PRESETCLR_DK_SLATE_BLUE
    pub const PRESETCLR_DK_SLATE_BLUE: i32 = 58;
    // Java field: int[] PRESETCLR_RGB_DK_SLATE_BLUE
    pub const PRESETCLR_RGB_DK_SLATE_BLUE: [i32; 3] = [72, 61, 139];
    // Java field: int PRESETCLR_DK_SLATE_GRAY
    pub const PRESETCLR_DK_SLATE_GRAY: i32 = 59;
    // Java field: int[] PRESETCLR_RGB_DK_SLATE_GRAY
    pub const PRESETCLR_RGB_DK_SLATE_GRAY: [i32; 3] = [47, 79, 79];
    // Java field: int PRESETCLR_DK_SLATE_GREY
    pub const PRESETCLR_DK_SLATE_GREY: i32 = 60;
    // Java field: int[] PRESETCLR_RGB_DK_SLATE_GREY
    pub const PRESETCLR_RGB_DK_SLATE_GREY: [i32; 3] = [47, 79, 79];
    // Java field: int PRESETCLR_DK_TURQUOISE
    pub const PRESETCLR_DK_TURQUOISE: i32 = 61;
    // Java field: int[] PRESETCLR_RGB_DK_TURQUOISE
    pub const PRESETCLR_RGB_DK_TURQUOISE: [i32; 3] = [0, 206, 209];
    // Java field: int PRESETCLR_DK_VIOLET
    pub const PRESETCLR_DK_VIOLET: i32 = 62;
    // Java field: int[] PRESETCLR_RGB_DK_VIOLET
    pub const PRESETCLR_RGB_DK_VIOLET: [i32; 3] = [148, 0, 211];
    // Java field: int PRESETCLR_DODGER_BLUE
    pub const PRESETCLR_DODGER_BLUE: i32 = 63;
    // Java field: int[] PRESETCLR_RGB_DODGER_BLUE
    pub const PRESETCLR_RGB_DODGER_BLUE: [i32; 3] = [30, 144, 255];
    // Java field: int PRESETCLR_FIREBRICK
    pub const PRESETCLR_FIREBRICK: i32 = 64;
    // Java field: int[] PRESETCLR_RGB_FIREBRICK
    pub const PRESETCLR_RGB_FIREBRICK: [i32; 3] = [178, 34, 34];
    // Java field: int PRESETCLR_FLORAL_WHITE
    pub const PRESETCLR_FLORAL_WHITE: i32 = 65;
    // Java field: int[] PRESETCLR_RGB_FLORAL_WHITE
    pub const PRESETCLR_RGB_FLORAL_WHITE: [i32; 3] = [255, 250, 240];
    // Java field: int PRESETCLR_FOREST_GREEN
    pub const PRESETCLR_FOREST_GREEN: i32 = 66;
    // Java field: int[] PRESETCLR_RGB_FOREST_GREEN
    pub const PRESETCLR_RGB_FOREST_GREEN: [i32; 3] = [34, 139, 34];
    // Java field: int PRESETCLR_FUCHSIA
    pub const PRESETCLR_FUCHSIA: i32 = 67;
    // Java field: int[] PRESETCLR_RGB_FUCHSIA
    pub const PRESETCLR_RGB_FUCHSIA: [i32; 3] = [255, 0, 255];
    // Java field: int PRESETCLR_GAINSBORO
    pub const PRESETCLR_GAINSBORO: i32 = 68;
    // Java field: int[] PRESETCLR_RGB_GAINSBORO
    pub const PRESETCLR_RGB_GAINSBORO: [i32; 3] = [220, 220, 220];
    // Java field: int PRESETCLR_GHOST_WHITE
    pub const PRESETCLR_GHOST_WHITE: i32 = 69;
    // Java field: int[] PRESETCLR_RGB_GHOST_WHITE
    pub const PRESETCLR_RGB_GHOST_WHITE: [i32; 3] = [248, 248, 255];
    // Java field: int PRESETCLR_GOLD
    pub const PRESETCLR_GOLD: i32 = 70;
    // Java field: int[] PRESETCLR_RGB_GOLD
    pub const PRESETCLR_RGB_GOLD: [i32; 3] = [255, 215, 0];
    // Java field: int PRESETCLR_GOLDENROD
    pub const PRESETCLR_GOLDENROD: i32 = 71;
    // Java field: int[] PRESETCLR_RGB_GOLDENROD
    pub const PRESETCLR_RGB_GOLDENROD: [i32; 3] = [218, 165, 32];
    // Java field: int PRESETCLR_GRAY
    pub const PRESETCLR_GRAY: i32 = 72;
    // Java field: int[] PRESETCLR_RGB_GRAY
    pub const PRESETCLR_RGB_GRAY: [i32; 3] = [128, 128, 128];
    // Java field: int PRESETCLR_GREEN
    pub const PRESETCLR_GREEN: i32 = 73;
    // Java field: int[] PRESETCLR_RGB_GREEN
    pub const PRESETCLR_RGB_GREEN: [i32; 3] = [0, 128, 0];
    // Java field: int PRESETCLR_GREEN_YELLOW
    pub const PRESETCLR_GREEN_YELLOW: i32 = 74;
    // Java field: int[] PRESETCLR_RGB_GREEN_YELLOW
    pub const PRESETCLR_RGB_GREEN_YELLOW: [i32; 3] = [173, 255, 47];
    // Java field: int PRESETCLR_GREY
    pub const PRESETCLR_GREY: i32 = 75;
    // Java field: int[] PRESETCLR_RGB_GREY
    pub const PRESETCLR_RGB_GREY: [i32; 3] = [128, 128, 128];
    // Java field: int PRESETCLR_HONEYDEW
    pub const PRESETCLR_HONEYDEW: i32 = 76;
    // Java field: int[] PRESETCLR_RGB_HONEYDEW
    pub const PRESETCLR_RGB_HONEYDEW: [i32; 3] = [240, 255, 240];
    // Java field: int PRESETCLR_HOT_PINK
    pub const PRESETCLR_HOT_PINK: i32 = 77;
    // Java field: int[] PRESETCLR_RGB_HOT_PINK
    pub const PRESETCLR_RGB_HOT_PINK: [i32; 3] = [255, 105, 180];
    // Java field: int PRESETCLR_INDIAN_RED
    pub const PRESETCLR_INDIAN_RED: i32 = 78;
    // Java field: int[] PRESETCLR_RGB_INDIAN_RED
    pub const PRESETCLR_RGB_INDIAN_RED: [i32; 3] = [205, 92, 92];
    // Java field: int PRESETCLR_INDIGO
    pub const PRESETCLR_INDIGO: i32 = 79;
    // Java field: int[] PRESETCLR_RGB_INDIGO
    pub const PRESETCLR_RGB_INDIGO: [i32; 3] = [75, 0, 130];
    // Java field: int PRESETCLR_IVORY
    pub const PRESETCLR_IVORY: i32 = 80;
    // Java field: int[] PRESETCLR_RGB_IVORY
    pub const PRESETCLR_RGB_IVORY: [i32; 3] = [255, 255, 240];
    // Java field: int PRESETCLR_KHAKI
    pub const PRESETCLR_KHAKI: i32 = 81;
    // Java field: int[] PRESETCLR_RGB_KHAKI
    pub const PRESETCLR_RGB_KHAKI: [i32; 3] = [240, 230, 140];
    // Java field: int PRESETCLR_LAVENDER
    pub const PRESETCLR_LAVENDER: i32 = 82;
    // Java field: int[] PRESETCLR_RGB_LAVENDER
    pub const PRESETCLR_RGB_LAVENDER: [i32; 3] = [230, 230, 250];
    // Java field: int PRESETCLR_LAVENDER_BLUSH
    pub const PRESETCLR_LAVENDER_BLUSH: i32 = 83;
    // Java field: int[] PRESETCLR_RGB_LAVENDER_BLUSH
    pub const PRESETCLR_RGB_LAVENDER_BLUSH: [i32; 3] = [255, 240, 245];
    // Java field: int PRESETCLR_LAWN_GREEN
    pub const PRESETCLR_LAWN_GREEN: i32 = 84;
    // Java field: int[] PRESETCLR_RGB_LAWN_GREEN
    pub const PRESETCLR_RGB_LAWN_GREEN: [i32; 3] = [124, 252, 0];
    // Java field: int PRESETCLR_LEMON_CHIFFON
    pub const PRESETCLR_LEMON_CHIFFON: i32 = 85;
    // Java field: int[] PRESETCLR_RGB_LEMON_CHIFFON
    pub const PRESETCLR_RGB_LEMON_CHIFFON: [i32; 3] = [255, 250, 205];
    // Java field: int PRESETCLR_LIGHT_BLUE
    pub const PRESETCLR_LIGHT_BLUE: i32 = 86;
    // Java field: int[] PRESETCLR_RGB_LIGHT_BLUE
    pub const PRESETCLR_RGB_LIGHT_BLUE: [i32; 3] = [173, 216, 230];
    // Java field: int PRESETCLR_LIGHT_CORAL
    pub const PRESETCLR_LIGHT_CORAL: i32 = 87;
    // Java field: int[] PRESETCLR_RGB_LIGHT_CORAL
    pub const PRESETCLR_RGB_LIGHT_CORAL: [i32; 3] = [240, 128, 128];
    // Java field: int PRESETCLR_LIGHT_CYAN
    pub const PRESETCLR_LIGHT_CYAN: i32 = 88;
    // Java field: int[] PRESETCLR_RGB_LIGHT_CYAN
    pub const PRESETCLR_RGB_LIGHT_CYAN: [i32; 3] = [224, 255, 255];
    // Java field: int PRESETCLR_LIGHT_GOLDENROD_YELLOW
    pub const PRESETCLR_LIGHT_GOLDENROD_YELLOW: i32 = 89;
    // Java field: int[] PRESETCLR_RGB_LIGHT_GOLDENROD_YELLOW
    pub const PRESETCLR_RGB_LIGHT_GOLDENROD_YELLOW: [i32; 3] = [250, 250, 210];
    // Java field: int PRESETCLR_LIGHT_GRAY
    pub const PRESETCLR_LIGHT_GRAY: i32 = 90;
    // Java field: int[] PRESETCLR_RGB_LIGHT_GRAY
    pub const PRESETCLR_RGB_LIGHT_GRAY: [i32; 3] = [211, 211, 211];
    // Java field: int PRESETCLR_LIGHT_GREEN
    pub const PRESETCLR_LIGHT_GREEN: i32 = 91;
    // Java field: int[] PRESETCLR_RGB_LIGHT_GREEN
    pub const PRESETCLR_RGB_LIGHT_GREEN: [i32; 3] = [144, 238, 144];
    // Java field: int PRESETCLR_LIGHT_GREY
    pub const PRESETCLR_LIGHT_GREY: i32 = 92;
    // Java field: int[] PRESETCLR_RGB_LIGHT_GREY
    pub const PRESETCLR_RGB_LIGHT_GREY: [i32; 3] = [211, 211, 211];
    // Java field: int PRESETCLR_LIGHT_PINK
    pub const PRESETCLR_LIGHT_PINK: i32 = 93;
    // Java field: int[] PRESETCLR_RGB_LIGHT_PINK
    pub const PRESETCLR_RGB_LIGHT_PINK: [i32; 3] = [255, 182, 193];
    // Java field: int PRESETCLR_LIGHT_SALMON
    pub const PRESETCLR_LIGHT_SALMON: i32 = 94;
    // Java field: int[] PRESETCLR_RGB_LIGHT_SALMON
    pub const PRESETCLR_RGB_LIGHT_SALMON: [i32; 3] = [255, 160, 122];
    // Java field: int PRESETCLR_LIGHT_SEA_GREEN
    pub const PRESETCLR_LIGHT_SEA_GREEN: i32 = 95;
    // Java field: int[] PRESETCLR_RGB_LIGHT_SEA_GREEN
    pub const PRESETCLR_RGB_LIGHT_SEA_GREEN: [i32; 3] = [32, 178, 170];
    // Java field: int PRESETCLR_LIGHT_SKY_BLUE
    pub const PRESETCLR_LIGHT_SKY_BLUE: i32 = 96;
    // Java field: int[] PRESETCLR_RGB_LIGHT_SKY_BLUE
    pub const PRESETCLR_RGB_LIGHT_SKY_BLUE: [i32; 3] = [135, 206, 250];
    // Java field: int PRESETCLR_LIGHT_SLATE_GRAY
    pub const PRESETCLR_LIGHT_SLATE_GRAY: i32 = 97;
    // Java field: int[] PRESETCLR_RGB_LIGHT_SLATE_GRAY
    pub const PRESETCLR_RGB_LIGHT_SLATE_GRAY: [i32; 3] = [119, 136, 153];
    // Java field: int PRESETCLR_LIGHT_SLATE_GREY
    pub const PRESETCLR_LIGHT_SLATE_GREY: i32 = 98;
    // Java field: int[] PRESETCLR_RGB_LIGHT_SLATE_GREY
    pub const PRESETCLR_RGB_LIGHT_SLATE_GREY: [i32; 3] = [119, 136, 153];
    // Java field: int PRESETCLR_LIGHT_STEEL_BLUE
    pub const PRESETCLR_LIGHT_STEEL_BLUE: i32 = 99;
    // Java field: int[] PRESETCLR_RGB_LIGHT_STEEL_BLUE
    pub const PRESETCLR_RGB_LIGHT_STEEL_BLUE: [i32; 3] = [176, 196, 222];
    // Java field: int PRESETCLR_LIGHT_YELLOW
    pub const PRESETCLR_LIGHT_YELLOW: i32 = 100;
    // Java field: int[] PRESETCLR_RGB_LIGHT_YELLOW
    pub const PRESETCLR_RGB_LIGHT_YELLOW: [i32; 3] = [255, 255, 224];
    // Java field: int PRESETCLR_LIME
    pub const PRESETCLR_LIME: i32 = 101;
    // Java field: int[] PRESETCLR_RGB_LIME
    pub const PRESETCLR_RGB_LIME: [i32; 3] = [0, 255, 0];
    // Java field: int PRESETCLR_LIME_GREEN
    pub const PRESETCLR_LIME_GREEN: i32 = 102;
    // Java field: int[] PRESETCLR_RGB_LIME_GREEN
    pub const PRESETCLR_RGB_LIME_GREEN: [i32; 3] = [50, 205, 50];
    // Java field: int PRESETCLR_LINEN
    pub const PRESETCLR_LINEN: i32 = 103;
    // Java field: int[] PRESETCLR_RGB_LINEN
    pub const PRESETCLR_RGB_LINEN: [i32; 3] = [250, 240, 230];
    // Java field: int PRESETCLR_LT_BLUE
    pub const PRESETCLR_LT_BLUE: i32 = 104;
    // Java field: int[] PRESETCLR_RGB_LT_BLUE
    pub const PRESETCLR_RGB_LT_BLUE: [i32; 3] = [173, 216, 230];
    // Java field: int PRESETCLR_LT_CORAL
    pub const PRESETCLR_LT_CORAL: i32 = 105;
    // Java field: int[] PRESETCLR_RGB_LT_CORAL
    pub const PRESETCLR_RGB_LT_CORAL: [i32; 3] = [240, 128, 128];
    // Java field: int PRESETCLR_LT_CYAN
    pub const PRESETCLR_LT_CYAN: i32 = 106;
    // Java field: int[] PRESETCLR_RGB_LT_CYAN
    pub const PRESETCLR_RGB_LT_CYAN: [i32; 3] = [224, 255, 255];
    // Java field: int PRESETCLR_LT_GOLDENROD_YELLOW
    pub const PRESETCLR_LT_GOLDENROD_YELLOW: i32 = 107;
    // Java field: int[] PRESETCLR_RGB_LT_GOLDENROD_YELLOW
    pub const PRESETCLR_RGB_LT_GOLDENROD_YELLOW: [i32; 3] = [250, 250, 120];
    // Java field: int PRESETCLR_LT_GRAY
    pub const PRESETCLR_LT_GRAY: i32 = 108;
    // Java field: int[] PRESETCLR_RGB_LT_GRAY
    pub const PRESETCLR_RGB_LT_GRAY: [i32; 3] = [211, 211, 211];
    // Java field: int PRESETCLR_LT_GREEN
    pub const PRESETCLR_LT_GREEN: i32 = 109;
    // Java field: int[] PRESETCLR_RGB_LT_GREEN
    pub const PRESETCLR_RGB_LT_GREEN: [i32; 3] = [144, 238, 144];
    // Java field: int PRESETCLR_LT_GREY
    pub const PRESETCLR_LT_GREY: i32 = 110;
    // Java field: int[] PRESETCLR_RGB_LT_GREY
    pub const PRESETCLR_RGB_LT_GREY: [i32; 3] = [211, 211, 211];
    // Java field: int PRESETCLR_LT_PINK
    pub const PRESETCLR_LT_PINK: i32 = 111;
    // Java field: int[] PRESETCLR_RGB_LT_PINK
    pub const PRESETCLR_RGB_LT_PINK: [i32; 3] = [255, 182, 193];
    // Java field: int PRESETCLR_LT_SALMON
    pub const PRESETCLR_LT_SALMON: i32 = 112;
    // Java field: int[] PRESETCLR_RGB_LT_SALMON
    pub const PRESETCLR_RGB_LT_SALMON: [i32; 3] = [255, 160, 122];
    // Java field: int PRESETCLR_LT_SEA_GREEN
    pub const PRESETCLR_LT_SEA_GREEN: i32 = 113;
    // Java field: int[] PRESETCLR_RGB_LT_SEA_GREEN
    pub const PRESETCLR_RGB_LT_SEA_GREEN: [i32; 3] = [32, 178, 170];
    // Java field: int PRESETCLR_LT_SKY_BLUE
    pub const PRESETCLR_LT_SKY_BLUE: i32 = 114;
    // Java field: int[] PRESETCLR_RGB_LT_SKY_BLUE
    pub const PRESETCLR_RGB_LT_SKY_BLUE: [i32; 3] = [135, 206, 250];
    // Java field: int PRESETCLR_LT_SLATE_GRAY
    pub const PRESETCLR_LT_SLATE_GRAY: i32 = 115;
    // Java field: int[] PRESETCLR_RGB_LT_SLATE_GRAY
    pub const PRESETCLR_RGB_LT_SLATE_GRAY: [i32; 3] = [119, 136, 153];
    // Java field: int PRESETCLR_LT_SLATE_GREY
    pub const PRESETCLR_LT_SLATE_GREY: i32 = 116;
    // Java field: int[] PRESETCLR_RGB_LT_SLATE_GREY
    pub const PRESETCLR_RGB_LT_SLATE_GREY: [i32; 3] = [119, 136, 153];
    // Java field: int PRESETCLR_LT_STEEL_BLUE
    pub const PRESETCLR_LT_STEEL_BLUE: i32 = 117;
    // Java field: int[] PRESETCLR_RGB_LT_STEEL_BLUE
    pub const PRESETCLR_RGB_LT_STEEL_BLUE: [i32; 3] = [176, 196, 222];
    // Java field: int PRESETCLR_LT_YELLOW
    pub const PRESETCLR_LT_YELLOW: i32 = 118;
    // Java field: int[] PRESETCLR_RGB_LT_YELLOW
    pub const PRESETCLR_RGB_LT_YELLOW: [i32; 3] = [255, 255, 224];
    // Java field: int PRESETCLR_MAGENTA
    pub const PRESETCLR_MAGENTA: i32 = 119;
    // Java field: int[] PRESETCLR_RGB_MAGENTA
    pub const PRESETCLR_RGB_MAGENTA: [i32; 3] = [255, 0, 255];
    // Java field: int PRESETCLR_MAROON
    pub const PRESETCLR_MAROON: i32 = 120;
    // Java field: int[] PRESETCLR_RGB_MAROON
    pub const PRESETCLR_RGB_MAROON: [i32; 3] = [128, 0, 0];
    // Java field: int PRESETCLR_MED_AQUAMARINE
    pub const PRESETCLR_MED_AQUAMARINE: i32 = 121;
    // Java field: int[] PRESETCLR_RGB_MED_AQUAMARINE
    pub const PRESETCLR_RGB_MED_AQUAMARINE: [i32; 3] = [102, 205, 170];
    // Java field: int PRESETCLR_MED_BLUE
    pub const PRESETCLR_MED_BLUE: i32 = 122;
    // Java field: int[] PRESETCLR_RGB_MED_BLUE
    pub const PRESETCLR_RGB_MED_BLUE: [i32; 3] = [0, 0, 205];
    // Java field: int PRESETCLR_MEDIUM_AQUAMARINE
    pub const PRESETCLR_MEDIUM_AQUAMARINE: i32 = 123;
    // Java field: int[] PRESETCLR_RGB_MEDIUM_AQUAMARINE
    pub const PRESETCLR_RGB_MEDIUM_AQUAMARINE: [i32; 3] = [102, 205, 170];
    // Java field: int PRESETCLR_MEDIUM_BLUE
    pub const PRESETCLR_MEDIUM_BLUE: i32 = 124;
    // Java field: int[] PRESETCLR_RGB_MEDIUM_BLUE
    pub const PRESETCLR_RGB_MEDIUM_BLUE: [i32; 3] = [0, 0, 205];
    // Java field: int PRESETCLR_MEDIUM_ORCHID
    pub const PRESETCLR_MEDIUM_ORCHID: i32 = 125;
    // Java field: int[] PRESETCLR_RGB_MEDIUM_ORCHID
    pub const PRESETCLR_RGB_MEDIUM_ORCHID: [i32; 3] = [186, 85, 211];
    // Java field: int PRESETCLR_MEDIUM_PURPLE
    pub const PRESETCLR_MEDIUM_PURPLE: i32 = 126;
    // Java field: int[] PRESETCLR_RGB_MEDIUM_PURPLE
    pub const PRESETCLR_RGB_MEDIUM_PURPLE: [i32; 3] = [147, 112, 219];
    // Java field: int PRESETCLR_MEDIUM_SEA_GREEN
    pub const PRESETCLR_MEDIUM_SEA_GREEN: i32 = 127;
    // Java field: int[] PRESETCLR_RGB_MEDIUM_SEA_GREEN
    pub const PRESETCLR_RGB_MEDIUM_SEA_GREEN: [i32; 3] = [60, 179, 113];
    // Java field: int PRESETCLR_MEDIUM_SLATE_BLUE
    pub const PRESETCLR_MEDIUM_SLATE_BLUE: i32 = 128;
    // Java field: int[] PRESETCLR_RGB_MEDIUM_SLATE_BLUE
    pub const PRESETCLR_RGB_MEDIUM_SLATE_BLUE: [i32; 3] = [123, 104, 238];
    // Java field: int PRESETCLR_MEDIUM_SPRING_GREEN
    pub const PRESETCLR_MEDIUM_SPRING_GREEN: i32 = 129;
    // Java field: int[] PRESETCLR_RGB_MEDIUM_SPRING_GREEN
    pub const PRESETCLR_RGB_MEDIUM_SPRING_GREEN: [i32; 3] = [0, 250, 154];
    // Java field: int PRESETCLR_MEDIUM_TURQUOISE
    pub const PRESETCLR_MEDIUM_TURQUOISE: i32 = 130;
    // Java field: int[] PRESETCLR_RGB_MEDIUM_TURQUOISE
    pub const PRESETCLR_RGB_MEDIUM_TURQUOISE: [i32; 3] = [72, 209, 204];
    // Java field: int PRESETCLR_MEDIUM_VIOLET_RED
    pub const PRESETCLR_MEDIUM_VIOLET_RED: i32 = 131;
    // Java field: int[] PRESETCLR_RGB_MEDIUM_VIOLET_RED
    pub const PRESETCLR_RGB_MEDIUM_VIOLET_RED: [i32; 3] = [199, 21, 133];
    // Java field: int PRESETCLR_MED_ORCHID
    pub const PRESETCLR_MED_ORCHID: i32 = 132;
    // Java field: int[] PRESETCLR_RGB_MED_ORCHID
    pub const PRESETCLR_RGB_MED_ORCHID: [i32; 3] = [186, 85, 211];
    // Java field: int PRESETCLR_MED_PURPLE
    pub const PRESETCLR_MED_PURPLE: i32 = 133;
    // Java field: int[] PRESETCLR_RGB_MED_PURPLE
    pub const PRESETCLR_RGB_MED_PURPLE: [i32; 3] = [147, 112, 219];
    // Java field: int PRESETCLR_MED_SEA_GREEN
    pub const PRESETCLR_MED_SEA_GREEN: i32 = 134;
    // Java field: int[] PRESETCLR_RGB_MED_SEA_GREEN
    pub const PRESETCLR_RGB_MED_SEA_GREEN: [i32; 3] = [60, 179, 113];
    // Java field: int PRESETCLR_MED_SLATE_BLUE
    pub const PRESETCLR_MED_SLATE_BLUE: i32 = 135;
    // Java field: int[] PRESETCLR_RGB_MED_SLATE_BLUE
    pub const PRESETCLR_RGB_MED_SLATE_BLUE: [i32; 3] = [123, 104, 238];
    // Java field: int PRESETCLR_MED_SPRING_GREEN
    pub const PRESETCLR_MED_SPRING_GREEN: i32 = 136;
    // Java field: int[] PRESETCLR_RGB_MED_SPRING_GREEN
    pub const PRESETCLR_RGB_MED_SPRING_GREEN: [i32; 3] = [0, 250, 154];
    // Java field: int PRESETCLR_MED_TURQUOISE
    pub const PRESETCLR_MED_TURQUOISE: i32 = 137;
    // Java field: int[] PRESETCLR_RGB_MED_TURQUOISE
    pub const PRESETCLR_RGB_MED_TURQUOISE: [i32; 3] = [72, 209, 204];
    // Java field: int PRESETCLR_MED_VIOLET_RED
    pub const PRESETCLR_MED_VIOLET_RED: i32 = 138;
    // Java field: int[] PRESETCLR_RGB_MED_VIOLET_RED
    pub const PRESETCLR_RGB_MED_VIOLET_RED: [i32; 3] = [199, 21, 133];
    // Java field: int PRESETCLR_MIDNIGHT_BLUE
    pub const PRESETCLR_MIDNIGHT_BLUE: i32 = 139;
    // Java field: int[] PRESETCLR_RGB_MIDNIGHT_BLUE
    pub const PRESETCLR_RGB_MIDNIGHT_BLUE: [i32; 3] = [25, 25, 112];
    // Java field: int PRESETCLR_MINT_CREAM
    pub const PRESETCLR_MINT_CREAM: i32 = 140;
    // Java field: int[] PRESETCLR_RGB_MINT_CREAM
    pub const PRESETCLR_RGB_MINT_CREAM: [i32; 3] = [245, 255, 250];
    // Java field: int PRESETCLR_MISTY_ROSE
    pub const PRESETCLR_MISTY_ROSE: i32 = 141;
    // Java field: int[] PRESETCLR_RGB_MISTY_ROSE
    pub const PRESETCLR_RGB_MISTY_ROSE: [i32; 3] = [255, 228, 225];
    // Java field: int PRESETCLR_MOCCASIN
    pub const PRESETCLR_MOCCASIN: i32 = 142;
    // Java field: int[] PRESETCLR_RGB_MOCCASIN
    pub const PRESETCLR_RGB_MOCCASIN: [i32; 3] = [255, 228, 181];
    // Java field: int PRESETCLR_NAVAJO_WHITE
    pub const PRESETCLR_NAVAJO_WHITE: i32 = 143;
    // Java field: int[] PRESETCLR_RGB_NAVAJO_WHITE
    pub const PRESETCLR_RGB_NAVAJO_WHITE: [i32; 3] = [255, 222, 173];
    // Java field: int PRESETCLR_NAVY
    pub const PRESETCLR_NAVY: i32 = 144;
    // Java field: int[] PRESETCLR_RGB_NAVY
    pub const PRESETCLR_RGB_NAVY: [i32; 3] = [0, 0, 128];
    // Java field: int PRESETCLR_OLD_LACE
    pub const PRESETCLR_OLD_LACE: i32 = 145;
    // Java field: int[] PRESETCLR_RGB_OLD_LACE
    pub const PRESETCLR_RGB_OLD_LACE: [i32; 3] = [253, 245, 230];
    // Java field: int PRESETCLR_OLIVE
    pub const PRESETCLR_OLIVE: i32 = 146;
    // Java field: int[] PRESETCLR_RGB_OLIVE
    pub const PRESETCLR_RGB_OLIVE: [i32; 3] = [128, 128, 0];
    // Java field: int PRESETCLR_OLIVE_DRAB
    pub const PRESETCLR_OLIVE_DRAB: i32 = 147;
    // Java field: int[] PRESETCLR_RGB_OLIVE_DRAB
    pub const PRESETCLR_RGB_OLIVE_DRAB: [i32; 3] = [107, 142, 35];
    // Java field: int PRESETCLR_ORANGE
    pub const PRESETCLR_ORANGE: i32 = 148;
    // Java field: int[] PRESETCLR_RGB_ORANGE
    pub const PRESETCLR_RGB_ORANGE: [i32; 3] = [255, 165, 0];
    // Java field: int PRESETCLR_ORANGE_RED
    pub const PRESETCLR_ORANGE_RED: i32 = 149;
    // Java field: int[] PRESETCLR_RGB_ORANGE_RED
    pub const PRESETCLR_RGB_ORANGE_RED: [i32; 3] = [255, 69, 0];
    // Java field: int PRESETCLR_ORCHID
    pub const PRESETCLR_ORCHID: i32 = 150;
    // Java field: int[] PRESETCLR_RGB_ORCHID
    pub const PRESETCLR_RGB_ORCHID: [i32; 3] = [218, 112, 214];
    // Java field: int PRESETCLR_PALE_GOLDENROD
    pub const PRESETCLR_PALE_GOLDENROD: i32 = 151;
    // Java field: int[] PRESETCLR_RGB_PALE_GOLDENROD
    pub const PRESETCLR_RGB_PALE_GOLDENROD: [i32; 3] = [238, 232, 170];
    // Java field: int PRESETCLR_PALE_GREEN
    pub const PRESETCLR_PALE_GREEN: i32 = 152;
    // Java field: int[] PRESETCLR_RGB_PALE_GREEN
    pub const PRESETCLR_RGB_PALE_GREEN: [i32; 3] = [152, 251, 152];
    // Java field: int PRESETCLR_PALE_TURQUOISE
    pub const PRESETCLR_PALE_TURQUOISE: i32 = 153;
    // Java field: int[] PRESETCLR_RGB_PALE_TURQUOISE
    pub const PRESETCLR_RGB_PALE_TURQUOISE: [i32; 3] = [175, 238, 238];
    // Java field: int PRESETCLR_PALE_VIOLET_RED
    pub const PRESETCLR_PALE_VIOLET_RED: i32 = 154;
    // Java field: int[] PRESETCLR_RGB_PALE_VIOLET_RED
    pub const PRESETCLR_RGB_PALE_VIOLET_RED: [i32; 3] = [219, 112, 147];
    // Java field: int PRESETCLR_PAPAYA_WHIP
    pub const PRESETCLR_PAPAYA_WHIP: i32 = 155;
    // Java field: int[] PRESETCLR_RGB_PAPAYA_WHIP
    pub const PRESETCLR_RGB_PAPAYA_WHIP: [i32; 3] = [255, 239, 213];
    // Java field: int PRESETCLR_PEACH_PUFF
    pub const PRESETCLR_PEACH_PUFF: i32 = 156;
    // Java field: int[] PRESETCLR_RGB_PEACH_PUFF
    pub const PRESETCLR_RGB_PEACH_PUFF: [i32; 3] = [255, 218, 185];
    // Java field: int PRESETCLR_PERU
    pub const PRESETCLR_PERU: i32 = 157;
    // Java field: int[] PRESETCLR_RGB_PERU
    pub const PRESETCLR_RGB_PERU: [i32; 3] = [205, 133, 63];
    // Java field: int PRESETCLR_PINK
    pub const PRESETCLR_PINK: i32 = 158;
    // Java field: int[] PRESETCLR_RGB_PINK
    pub const PRESETCLR_RGB_PINK: [i32; 3] = [255, 192, 203];
    // Java field: int PRESETCLR_PLUM
    pub const PRESETCLR_PLUM: i32 = 159;
    // Java field: int[] PRESETCLR_RGB_PLUM
    pub const PRESETCLR_RGB_PLUM: [i32; 3] = [221, 160, 221];
    // Java field: int PRESETCLR_POWDER_BLUE
    pub const PRESETCLR_POWDER_BLUE: i32 = 160;
    // Java field: int[] PRESETCLR_RGB_POWDER_BLUE
    pub const PRESETCLR_RGB_POWDER_BLUE: [i32; 3] = [176, 224, 230];
    // Java field: int PRESETCLR_PURPLE
    pub const PRESETCLR_PURPLE: i32 = 161;
    // Java field: int[] PRESETCLR_RGB_PURPLE
    pub const PRESETCLR_RGB_PURPLE: [i32; 3] = [128, 0, 128];
    // Java field: int PRESETCLR_RED
    pub const PRESETCLR_RED: i32 = 162;
    // Java field: int[] PRESETCLR_RGB_RED
    pub const PRESETCLR_RGB_RED: [i32; 3] = [255, 0, 0];
    // Java field: int PRESETCLR_ROSY_BROWN
    pub const PRESETCLR_ROSY_BROWN: i32 = 163;
    // Java field: int[] PRESETCLR_RGB_ROSY_BROWN
    pub const PRESETCLR_RGB_ROSY_BROWN: [i32; 3] = [188, 143, 143];
    // Java field: int PRESETCLR_ROYAL_BLUE
    pub const PRESETCLR_ROYAL_BLUE: i32 = 164;
    // Java field: int[] PRESETCLR_RGB_ROYAL_BLUE
    pub const PRESETCLR_RGB_ROYAL_BLUE: [i32; 3] = [65, 105, 225];
    // Java field: int PRESETCLR_SADDLE_BROWN
    pub const PRESETCLR_SADDLE_BROWN: i32 = 165;
    // Java field: int[] PRESETCLR_RGB_SADDLE_BROWN
    pub const PRESETCLR_RGB_SADDLE_BROWN: [i32; 3] = [139, 69, 19];
    // Java field: int PRESETCLR_SALMON
    pub const PRESETCLR_SALMON: i32 = 166;
    // Java field: int[] PRESETCLR_RGB_SALMON
    pub const PRESETCLR_RGB_SALMON: [i32; 3] = [250, 128, 114];
    // Java field: int PRESETCLR_SANDY_BROWN
    pub const PRESETCLR_SANDY_BROWN: i32 = 167;
    // Java field: int[] PRESETCLR_RGB_SANDY_BROWN
    pub const PRESETCLR_RGB_SANDY_BROWN: [i32; 3] = [244, 164, 96];
    // Java field: int PRESETCLR_SEA_GREEN
    pub const PRESETCLR_SEA_GREEN: i32 = 168;
    // Java field: int[] PRESETCLR_RGB_SEA_GREEN
    pub const PRESETCLR_RGB_SEA_GREEN: [i32; 3] = [46, 139, 87];
    // Java field: int PRESETCLR_SEA_SHELL
    pub const PRESETCLR_SEA_SHELL: i32 = 169;
    // Java field: int[] PRESETCLR_RGB_SEA_SHELL
    pub const PRESETCLR_RGB_SEA_SHELL: [i32; 3] = [255, 245, 238];
    // Java field: int PRESETCLR_SIENNA
    pub const PRESETCLR_SIENNA: i32 = 170;
    // Java field: int[] PRESETCLR_RGB_SIENNA
    pub const PRESETCLR_RGB_SIENNA: [i32; 3] = [160, 82, 45];
    // Java field: int PRESETCLR_SILVER
    pub const PRESETCLR_SILVER: i32 = 171;
    // Java field: int[] PRESETCLR_RGB_SILVER
    pub const PRESETCLR_RGB_SILVER: [i32; 3] = [192, 192, 192];
    // Java field: int PRESETCLR_SKY_BLUE
    pub const PRESETCLR_SKY_BLUE: i32 = 172;
    // Java field: int[] PRESETCLR_RGB_SKY_BLUE
    pub const PRESETCLR_RGB_SKY_BLUE: [i32; 3] = [135, 206, 235];
    // Java field: int PRESETCLR_SLATE_BLUE
    pub const PRESETCLR_SLATE_BLUE: i32 = 173;
    // Java field: int[] PRESETCLR_RGB_SLATE_BLUE
    pub const PRESETCLR_RGB_SLATE_BLUE: [i32; 3] = [106, 90, 205];
    // Java field: int PRESETCLR_SLATE_GRAY
    pub const PRESETCLR_SLATE_GRAY: i32 = 174;
    // Java field: int[] PRESETCLR_RGB_SLATE_GRAY
    pub const PRESETCLR_RGB_SLATE_GRAY: [i32; 3] = [112, 128, 144];
    // Java field: int PRESETCLR_SLATE_GREY
    pub const PRESETCLR_SLATE_GREY: i32 = 175;
    // Java field: int[] PRESETCLR_RGB_SLATE_GREY
    pub const PRESETCLR_RGB_SLATE_GREY: [i32; 3] = [112, 128, 144];
    // Java field: int PRESETCLR_SNOW
    pub const PRESETCLR_SNOW: i32 = 176;
    // Java field: int[] PRESETCLR_RGB_SNOW
    pub const PRESETCLR_RGB_SNOW: [i32; 3] = [255, 250, 250];
    // Java field: int PRESETCLR_SPRING_GREEN
    pub const PRESETCLR_SPRING_GREEN: i32 = 177;
    // Java field: int[] PRESETCLR_RGB_SPRING_GREEN
    pub const PRESETCLR_RGB_SPRING_GREEN: [i32; 3] = [0, 255, 127];
    // Java field: int PRESETCLR_STEEL_BLUE
    pub const PRESETCLR_STEEL_BLUE: i32 = 178;
    // Java field: int[] PRESETCLR_RGB_STEEL_BLUE
    pub const PRESETCLR_RGB_STEEL_BLUE: [i32; 3] = [70, 130, 180];
    // Java field: int PRESETCLR_TAN
    pub const PRESETCLR_TAN: i32 = 179;
    // Java field: int[] PRESETCLR_RGB_TAN
    pub const PRESETCLR_RGB_TAN: [i32; 3] = [210, 180, 140];
    // Java field: int PRESETCLR_TEAL
    pub const PRESETCLR_TEAL: i32 = 180;
    // Java field: int[] PRESETCLR_RGB_TEAL
    pub const PRESETCLR_RGB_TEAL: [i32; 3] = [0, 128, 128];
    // Java field: int PRESETCLR_THISTLE
    pub const PRESETCLR_THISTLE: i32 = 181;
    // Java field: int[] PRESETCLR_RGB_THISTLE
    pub const PRESETCLR_RGB_THISTLE: [i32; 3] = [216, 191, 216];
    // Java field: int PRESETCLR_TOMATO
    pub const PRESETCLR_TOMATO: i32 = 182;
    // Java field: int[] PRESETCLR_RGB_TOMATO
    pub const PRESETCLR_RGB_TOMATO: [i32; 3] = [255, 99, 71];
    // Java field: int PRESETCLR_TURQUOISE
    pub const PRESETCLR_TURQUOISE: i32 = 183;
    // Java field: int[] PRESETCLR_RGB_TURQUOISE
    pub const PRESETCLR_RGB_TURQUOISE: [i32; 3] = [64, 224, 208];
    // Java field: int PRESETCLR_VIOLET
    pub const PRESETCLR_VIOLET: i32 = 184;
    // Java field: int[] PRESETCLR_RGB_VIOLET
    pub const PRESETCLR_RGB_VIOLET: [i32; 3] = [238, 130, 238];
    // Java field: int PRESETCLR_WHEAT
    pub const PRESETCLR_WHEAT: i32 = 185;
    // Java field: int[] PRESETCLR_RGB_WHEAT
    pub const PRESETCLR_RGB_WHEAT: [i32; 3] = [245, 222, 179];
    // Java field: int PRESETCLR_WHITE
    pub const PRESETCLR_WHITE: i32 = 186;
    // Java field: int[] PRESETCLR_RGB_WHITE
    pub const PRESETCLR_RGB_WHITE: [i32; 3] = [255, 255, 255];
    // Java field: int PRESETCLR_WHITE_SMOKE
    pub const PRESETCLR_WHITE_SMOKE: i32 = 187;
    // Java field: int[] PRESETCLR_RGB_WHITE_SMOKE
    pub const PRESETCLR_RGB_WHITE_SMOKE: [i32; 3] = [245, 245, 245];
    // Java field: int PRESETCLR_YELLOW
    pub const PRESETCLR_YELLOW: i32 = 188;
    // Java field: int[] PRESETCLR_RGB_YELLOW
    pub const PRESETCLR_RGB_YELLOW: [i32; 3] = [255, 255, 0];
    // Java field: int PRESETCLR_YELLOW_GREEN
    pub const PRESETCLR_YELLOW_GREEN: i32 = 189;
    // Java field: int[] PRESETCLR_RGB_YELLOW_GREEN
    pub const PRESETCLR_RGB_YELLOW_GREEN: [i32; 3] = [154, 205, 50];
    // Java field: int[][] PRESET_COLORS
    // PRESET_COLORS is a reference array pointing to the RGB arrays above, accessed by index
    // Java: COLOR()
    // Java: super();
}

// Java: public class COLOR_TRANSFORM
pub mod COLOR_TRANSFORM {
    // Java field: int ALPHA
    pub const ALPHA: i32 = 0;
    // Java field: int ALPHA_MODULATION
    pub const ALPHA_MODULATION: i32 = 1;
    // Java field: int ALPHA_OFFSET
    pub const ALPHA_OFFSET: i32 = 2;
    // Java field: int BLUE
    pub const BLUE: i32 = 3;
    // Java field: int BLUE_MODIFICATION
    pub const BLUE_MODIFICATION: i32 = 4;
    // Java field: int BLUE_OFFSET
    pub const BLUE_OFFSET: i32 = 5;
    // Java field: int COMPLEMENT
    pub const COMPLEMENT: i32 = 6;
    // Java field: int GAMMA
    pub const GAMMA: i32 = 7;
    // Java field: int GRAY
    pub const GRAY: i32 = 8;
    // Java field: int GREEN
    pub const GREEN: i32 = 9;
    // Java field: int GREEN_MODIFICATION
    pub const GREEN_MODIFICATION: i32 = 10;
    // Java field: int GREEN_OFFSET
    pub const GREEN_OFFSET: i32 = 11;
    // Java field: int HUE
    pub const HUE: i32 = 12;
    // Java field: int HUE_MODULATE
    pub const HUE_MODULATE: i32 = 13;
    // Java field: int HUE_OFFSET
    pub const HUE_OFFSET: i32 = 14;
    // Java field: int INVERSE
    pub const INVERSE: i32 = 15;
    // Java field: int INVERSE_GAMMA
    pub const INVERSE_GAMMA: i32 = 16;
    // Java field: int LUMINANCE
    pub const LUMINANCE: i32 = 17;
    // Java field: int LUMINANCE_MODULATION
    pub const LUMINANCE_MODULATION: i32 = 18;
    // Java field: int LUMINANCE_OFFSET
    pub const LUMINANCE_OFFSET: i32 = 19;
    // Java field: int RED
    pub const RED: i32 = 20;
    // Java field: int RED_MODULATION
    pub const RED_MODULATION: i32 = 21;
    // Java field: int RED_OFFSET
    pub const RED_OFFSET: i32 = 22;
    // Java field: int SATURATION
    pub const SATURATION: i32 = 23;
    // Java field: int SATURATION_MODULATION
    pub const SATURATION_MODULATION: i32 = 24;
    // Java field: int SATURATION_OFFSET
    pub const SATURATION_OFFSET: i32 = 25;
    // Java field: int SHADE
    pub const SHADE: i32 = 26;
    // Java field: int TINT
    pub const TINT: i32 = 27;
    // Java: COLOR_TRANSFORM()
    // Java: super();
}

// Java: public class PRESET_PATTERN
pub mod PRESET_PATTERN {
    // Java field: int PCT5
    pub const PCT5: i32 = 0;
    // Java field: int PCT10
    pub const PCT10: i32 = 1;
    // Java field: int PCT20
    pub const PCT20: i32 = 2;
    // Java field: int PCT25
    pub const PCT25: i32 = 3;
    // Java field: int PCT30
    pub const PCT30: i32 = 4;
    // Java field: int PCT40
    pub const PCT40: i32 = 5;
    // Java field: int PCT50
    pub const PCT50: i32 = 6;
    // Java field: int PCT60
    pub const PCT60: i32 = 7;
    // Java field: int PCT70
    pub const PCT70: i32 = 8;
    // Java field: int PCT75
    pub const PCT75: i32 = 9;
    // Java field: int PCT80
    pub const PCT80: i32 = 10;
    // Java field: int PCT90
    pub const PCT90: i32 = 11;
    // Java field: int HORZ
    pub const HORZ: i32 = 12;
    // Java field: int VERT
    pub const VERT: i32 = 13;
    // Java field: int LTHORZ
    pub const LTHORZ: i32 = 14;
    // Java field: int LTVERT
    pub const LTVERT: i32 = 15;
    // Java field: int DKHORZ
    pub const DKHORZ: i32 = 16;
    // Java field: int DKVERT
    pub const DKVERT: i32 = 17;
    // Java field: int NARHORZ
    pub const NARHORZ: i32 = 18;
    // Java field: int NARVERT
    pub const NARVERT: i32 = 19;
    // Java field: int DASHHORZ
    pub const DASHHORZ: i32 = 20;
    // Java field: int DASHVERT
    pub const DASHVERT: i32 = 21;
    // Java field: int CROSS
    pub const CROSS: i32 = 22;
    // Java field: int DNDIAG
    pub const DNDIAG: i32 = 23;
    // Java field: int UPDIAG
    pub const UPDIAG: i32 = 24;
    // Java field: int LTDNDIAG
    pub const LTDNDIAG: i32 = 25;
    // Java field: int LTUPDIAG
    pub const LTUPDIAG: i32 = 26;
    // Java field: int DKDNDIAG
    pub const DKDNDIAG: i32 = 27;
    // Java field: int DKUPDIAG
    pub const DKUPDIAG: i32 = 28;
    // Java field: int WDDNDIAG
    pub const WDDNDIAG: i32 = 29;
    // Java field: int WDUPDIAG
    pub const WDUPDIAG: i32 = 30;
    // Java field: int DASHDNDIAG
    pub const DASHDNDIAG: i32 = 31;
    // Java field: int DASHUPDIAG
    pub const DASHUPDIAG: i32 = 32;
    // Java field: int DIAGCROSS
    pub const DIAGCROSS: i32 = 33;
    // Java field: int SMCHECK
    pub const SMCHECK: i32 = 34;
    // Java field: int LGCHECK
    pub const LGCHECK: i32 = 35;
    // Java field: int SMGRID
    pub const SMGRID: i32 = 36;
    // Java field: int LGGRID
    pub const LGGRID: i32 = 37;
    // Java field: int DOTGRID
    pub const DOTGRID: i32 = 38;
    // Java field: int SMCONFETTI
    pub const SMCONFETTI: i32 = 39;
    // Java field: int LGCONFETTI
    pub const LGCONFETTI: i32 = 40;
    // Java field: int HORZBRICK
    pub const HORZBRICK: i32 = 41;
    // Java field: int DIAGBRICK
    pub const DIAGBRICK: i32 = 42;
    // Java field: int SOLIDDMND
    pub const SOLIDDMND: i32 = 43;
    // Java field: int OPENDMND
    pub const OPENDMND: i32 = 44;
    // Java field: int DOTDMND
    pub const DOTDMND: i32 = 45;
    // Java field: int PLAID
    pub const PLAID: i32 = 46;
    // Java field: int SPHERE
    pub const SPHERE: i32 = 47;
    // Java field: int WEAVE
    pub const WEAVE: i32 = 48;
    // Java field: int DIVOT
    pub const DIVOT: i32 = 49;
    // Java field: int SHINGLE
    pub const SHINGLE: i32 = 50;
    // Java field: int WAVE
    pub const WAVE: i32 = 51;
    // Java field: int TRELLIS
    pub const TRELLIS: i32 = 52;
    // Java field: int ZIGZAG
    pub const ZIGZAG: i32 = 53;
    // Java: PRESET_PATTERN()
    // Java: super();
}

// Java: public class EFFECT
pub mod EFFECT {
    // Java field: int ALPHABILEVEL
    pub const ALPHABILEVEL: i32 = 0;
    // Java field: int ALPHACEILING
    pub const ALPHACEILING: i32 = 1;
    // Java field: int ALPHAFLOOR
    pub const ALPHAFLOOR: i32 = 2;
    // Java field: int ALPHAINVERSE
    pub const ALPHAINVERSE: i32 = 3;
    // Java field: int ALPHAMOD
    pub const ALPHAMOD: i32 = 4;
    // Java field: int ALPHAMODFIX
    pub const ALPHAMODFIX: i32 = 5;
    // Java field: int ALPHAOUTSET
    pub const ALPHAOUTSET: i32 = 6;
    // Java field: int ALPHAREPLACE
    pub const ALPHAREPLACE: i32 = 7;
    // Java field: int BILEVEL
    pub const BILEVEL: i32 = 8;
    // Java field: int BLEND
    pub const BLEND: i32 = 9;
    // Java field: int BLUR
    pub const BLUR: i32 = 10;
    // Java field: int COLORCHANGE
    pub const COLORCHANGE: i32 = 11;
    // Java field: int COLORREPLACEMENT
    pub const COLORREPLACEMENT: i32 = 12;
    // Java field: int DUOTONE
    pub const DUOTONE: i32 = 13;
    // Java field: int EFFECT
    pub const EFFECT: i32 = 14;
    // Java field: int EFFECTCONTAINER
    pub const EFFECTCONTAINER: i32 = 15;
    // Java field: int FILL
    pub const FILL: i32 = 16;
    // Java field: int FILLOVERLAY
    pub const FILLOVERLAY: i32 = 17;
    // Java field: int GLOW
    pub const GLOW: i32 = 18;
    // Java field: int GRAYSCALE
    pub const GRAYSCALE: i32 = 19;
    // Java field: int HSL
    pub const HSL: i32 = 20;
    // Java field: int INNERSHADOW
    pub const INNERSHADOW: i32 = 21;
    // Java field: int LUMINANCE
    pub const LUMINANCE: i32 = 22;
    // Java field: int OUTERSHADOW
    pub const OUTERSHADOW: i32 = 23;
    // Java field: int PRESETSHADOW
    pub const PRESETSHADOW: i32 = 24;
    // Java field: int REFLECTION
    pub const REFLECTION: i32 = 25;
    // Java field: int RELATIVEOFFSET
    pub const RELATIVEOFFSET: i32 = 26;
    // Java field: int SOFTEDGE
    pub const SOFTEDGE: i32 = 27;
    // Java field: int TINT
    pub const TINT: i32 = 28;
    // Java field: int XFRM
    pub const XFRM: i32 = 29;
    // Java field: int CONTRAST
    pub const CONTRAST: i32 = 30;
    // Java: EFFECT()
    // Java: super();
}

// Java: public class EFFECT_CONTAINER
pub mod EFFECT_CONTAINER {
    // Java field: int SIB
    pub const SIB: i32 = 0;
    // Java field: int TREE
    pub const TREE: i32 = 1;
    // Java: EFFECT_CONTAINER()
    // Java: super();
}

// Java: public class BLEND
pub mod BLEND {
    // Java field: int DARKEN
    pub const DARKEN: i32 = 0;
    // Java field: int LIGHTEN
    pub const LIGHTEN: i32 = 1;
    // Java field: int MULT
    pub const MULT: i32 = 2;
    // Java field: int OVER
    pub const OVER: i32 = 3;
    // Java field: int SCREEN
    pub const SCREEN: i32 = 4;
    // Java: BLEND()
    // Java: super();
}

// Java: public class RECT_ALIGNMENT
pub mod RECT_ALIGNMENT {
    // Java field: int B
    pub const B: i32 = 0;
    // Java field: int BL
    pub const BL: i32 = 1;
    // Java field: int BR
    pub const BR: i32 = 2;
    // Java field: int CTR
    pub const CTR: i32 = 3;
    // Java field: int L
    pub const L: i32 = 4;
    // Java field: int R
    pub const R: i32 = 5;
    // Java field: int T
    pub const T: i32 = 6;
    // Java field: int TL
    pub const TL: i32 = 7;
    // Java field: int TR
    pub const TR: i32 = 8;
    // Java field: int NONE
    pub const NONE: i32 = -1;
    // Java: RECT_ALIGNMENT()
    // Java: super();
}

// Java: public class TILE_FLIP
pub mod TILE_FLIP {
    // Java field: int NONE
    pub const NONE: i32 = 0;
    // Java field: int HORIZONTAL
    pub const HORIZONTAL: i32 = 1;
    // Java field: int VERTICAL
    pub const VERTICAL: i32 = 2;
    // Java field: int HORIZONTAL_VERTICAL
    pub const HORIZONTAL_VERTICAL: i32 = 3;
    // Java: TILE_FLIP()
    // Java: super();
}

// Java: public class EFFECT_PROPERTIES
pub mod EFFECT_PROPERTIES {
    // Java field: int LIST
    pub const LIST: i32 = 0;
    // Java field: int CONTAINER
    pub const CONTAINER: i32 = 1;
    // Java: EFFECT_PROPERTIES()
    // Java: super();
}

// Java: public class PRESET_CAMERA
pub mod PRESET_CAMERA {
    // Java field: int ISOMETRIC_BOTTOM_DOWN
    pub const ISOMETRIC_BOTTOM_DOWN: i32 = 0;
    // Java field: int ISOMETRIC_BOTTOM_UP
    pub const ISOMETRIC_BOTTOM_UP: i32 = 1;
    // Java field: int ISOMETRIC_LEFT_DOWN
    pub const ISOMETRIC_LEFT_DOWN: i32 = 2;
    // Java field: int ISOMETRIC_LEFT_UP
    pub const ISOMETRIC_LEFT_UP: i32 = 3;
    // Java field: int ISOMETRIC_OFF_AXIS_1_LEFT
    pub const ISOMETRIC_OFF_AXIS_1_LEFT: i32 = 4;
    // Java field: int ISOMETRIC_OFF_AXIS_1_RIGHT
    pub const ISOMETRIC_OFF_AXIS_1_RIGHT: i32 = 5;
    // Java field: int ISOMETRIC_OFF_AXIS_1_TOP
    pub const ISOMETRIC_OFF_AXIS_1_TOP: i32 = 6;
    // Java field: int ISOMETRIC_OFF_AXIS_2_LEFT
    pub const ISOMETRIC_OFF_AXIS_2_LEFT: i32 = 7;
    // Java field: int ISOMETRIC_OFF_AXIS_2_RIGHT
    pub const ISOMETRIC_OFF_AXIS_2_RIGHT: i32 = 8;
    // Java field: int ISOMETRIC_OFF_AXIS_2_TOP
    pub const ISOMETRIC_OFF_AXIS_2_TOP: i32 = 9;
    // Java field: int ISOMETRIC_OFF_AXIS_3_BOTTOM
    pub const ISOMETRIC_OFF_AXIS_3_BOTTOM: i32 = 10;
    // Java field: int ISOMETRIC_OFF_AXIS_3_LEFT
    pub const ISOMETRIC_OFF_AXIS_3_LEFT: i32 = 11;
    // Java field: int ISOMETRIC_OFF_AXIS_3_RIGHT
    pub const ISOMETRIC_OFF_AXIS_3_RIGHT: i32 = 12;
    // Java field: int ISOMETRIC_OFF_AXIS_4_BOTTOM
    pub const ISOMETRIC_OFF_AXIS_4_BOTTOM: i32 = 13;
    // Java field: int ISOMETRIC_OFF_AXIS_4_LEFT
    pub const ISOMETRIC_OFF_AXIS_4_LEFT: i32 = 14;
    // Java field: int ISOMETRIC_OFF_AXIS_4_RIGHT
    pub const ISOMETRIC_OFF_AXIS_4_RIGHT: i32 = 15;
    // Java field: int ISOMETRIC_RIGHT_DOWN
    pub const ISOMETRIC_RIGHT_DOWN: i32 = 16;
    // Java field: int ISOMETRIC_RIGHT_UP
    pub const ISOMETRIC_RIGHT_UP: i32 = 17;
    // Java field: int ISOMETRIC_TOP_DOWN
    pub const ISOMETRIC_TOP_DOWN: i32 = 18;
    // Java field: int ISOMETRIC_TOP_UP
    pub const ISOMETRIC_TOP_UP: i32 = 19;
    // Java field: int LEGACY_OBLIQUE_BOTTOM
    pub const LEGACY_OBLIQUE_BOTTOM: i32 = 20;
    // Java field: int LEGACY_OBLIQUE_BOTTOM_LEFT
    pub const LEGACY_OBLIQUE_BOTTOM_LEFT: i32 = 21;
    // Java field: int LEGACY_OBLIQUE_BOTTOM_RIGHT
    pub const LEGACY_OBLIQUE_BOTTOM_RIGHT: i32 = 22;
    // Java field: int LEGACY_OBLIQUE_FRONT
    pub const LEGACY_OBLIQUE_FRONT: i32 = 23;
    // Java field: int LEGACY_OBLIQUE_LEFT
    pub const LEGACY_OBLIQUE_LEFT: i32 = 24;
    // Java field: int LEGACY_OBLIQUE_RIGHT
    pub const LEGACY_OBLIQUE_RIGHT: i32 = 25;
    // Java field: int LEGACY_OBLIQUE_TOP
    pub const LEGACY_OBLIQUE_TOP: i32 = 26;
    // Java field: int LEGACY_OBLIQUE_TOP_LEFT
    pub const LEGACY_OBLIQUE_TOP_LEFT: i32 = 27;
    // Java field: int LEGACY_OBLIQUE_TOP_RIGHT
    pub const LEGACY_OBLIQUE_TOP_RIGHT: i32 = 28;
    // Java field: int LEGACY_PRESPECTIVE_BOTTOM
    pub const LEGACY_PRESPECTIVE_BOTTOM: i32 = 29;
    // Java field: int LEGACY_PRESPECTIVE_BOTTOM_LEFT
    pub const LEGACY_PRESPECTIVE_BOTTOM_LEFT: i32 = 30;
    // Java field: int LEGACY_PRESPECTIVE_BOTTOM_RIGHT
    pub const LEGACY_PRESPECTIVE_BOTTOM_RIGHT: i32 = 31;
    // Java field: int LEGACY_PRESPECTIVE_FRONT
    pub const LEGACY_PRESPECTIVE_FRONT: i32 = 32;
    // Java field: int LEGACY_PRESPECTIVE_LEFT
    pub const LEGACY_PRESPECTIVE_LEFT: i32 = 33;
    // Java field: int LEGACY_PRESPECTIVE_RIGHT
    pub const LEGACY_PRESPECTIVE_RIGHT: i32 = 34;
    // Java field: int LEGACY_PRESPECTIVE_TOP
    pub const LEGACY_PRESPECTIVE_TOP: i32 = 35;
    // Java field: int LEGACY_PRESPECTIVE_TOP_LEFT
    pub const LEGACY_PRESPECTIVE_TOP_LEFT: i32 = 36;
    // Java field: int LEGACY_PRESPECTIVE_TOP_RIGHT
    pub const LEGACY_PRESPECTIVE_TOP_RIGHT: i32 = 37;
    // Java field: int OBLIQUE_BOTTOM
    pub const OBLIQUE_BOTTOM: i32 = 38;
    // Java field: int OBLIQUE_BOTTOM_LEFT
    pub const OBLIQUE_BOTTOM_LEFT: i32 = 39;
    // Java field: int OBLIQUE_BOTTOM_RIGHT
    pub const OBLIQUE_BOTTOM_RIGHT: i32 = 40;
    // Java field: int OBLIQUE_LEFT
    pub const OBLIQUE_LEFT: i32 = 41;
    // Java field: int OBLIQUE_RIGHT
    pub const OBLIQUE_RIGHT: i32 = 42;
    // Java field: int OBLIQUE_TOP
    pub const OBLIQUE_TOP: i32 = 43;
    // Java field: int OBLIQUE_TOP_LEFT
    pub const OBLIQUE_TOP_LEFT: i32 = 44;
    // Java field: int OBLIQUE_TOP_RIGHT
    pub const OBLIQUE_TOP_RIGHT: i32 = 45;
    // Java field: int ORTHOGRAPHIC_FRONT
    pub const ORTHOGRAPHIC_FRONT: i32 = 46;
    // Java field: int PERSPECTIVE_ABOVE
    pub const PERSPECTIVE_ABOVE: i32 = 47;
    // Java field: int PERSPECTIVE_ABOVE_LEFT_FACING
    pub const PERSPECTIVE_ABOVE_LEFT_FACING: i32 = 48;
    // Java field: int PERSPECTIVE_ABOVE_RIGHT_FACONG
    pub const PERSPECTIVE_ABOVE_RIGHT_FACONG: i32 = 49;
    // Java field: int PERSPECTIVE_BELOW
    pub const PERSPECTIVE_BELOW: i32 = 50;
    // Java field: int PERSPECTIVE_CONTRASTING_LEFT_FACING
    pub const PERSPECTIVE_CONTRASTING_LEFT_FACING: i32 = 51;
    // Java field: int PERSPECTIVE_CONTRASTING_RIGHT_FACING
    pub const PERSPECTIVE_CONTRASTING_RIGHT_FACING: i32 = 52;
    // Java field: int PERSPECTIVE_FRONT
    pub const PERSPECTIVE_FRONT: i32 = 53;
    // Java field: int PERSPECTIVE_HEROIC_EXTREME_LEFT_FACING
    pub const PERSPECTIVE_HEROIC_EXTREME_LEFT_FACING: i32 = 54;
    // Java field: int PERSPECTIVE_HEROIC_EXTREME_RIGHT_FACING
    pub const PERSPECTIVE_HEROIC_EXTREME_RIGHT_FACING: i32 = 55;
    // Java field: int PERSPECTIVE_HEROIC_LEFT_FACING
    pub const PERSPECTIVE_HEROIC_LEFT_FACING: i32 = 56;
    // Java field: int PERSPECTIVE_HEROIC_RIGHT_FACING
    pub const PERSPECTIVE_HEROIC_RIGHT_FACING: i32 = 56;
    // Java field: int PERSPECTIVE_LEFT
    pub const PERSPECTIVE_LEFT: i32 = 57;
    // Java field: int PERSPECTIVE_RELAXED
    pub const PERSPECTIVE_RELAXED: i32 = 58;
    // Java field: int PERSPECTIVE_RELAXED_MODERATELY
    pub const PERSPECTIVE_RELAXED_MODERATELY: i32 = 59;
    // Java field: int PERSPECTIVE_RIGHT
    pub const PERSPECTIVE_RIGHT: i32 = 60;
    // Java field: int NONE
    pub const NONE: i32 = -1;
    // Java: PRESET_CAMERA()
    // Java: super();
}

// Java: public class PRESET_MATERIAL
pub mod PRESET_MATERIAL {
    // Java field: int CLEAR
    pub const CLEAR: i32 = 0;
    // Java field: int DK_EDGE
    pub const DK_EDGE: i32 = 1;
    // Java field: int FLAT
    pub const FLAT: i32 = 2;
    // Java field: int LEGACY_MATTE
    pub const LEGACY_MATTE: i32 = 3;
    // Java field: int LEGACY_METAL
    pub const LEGACY_METAL: i32 = 4;
    // Java field: int LEGACY_PLASTIC
    pub const LEGACY_PLASTIC: i32 = 5;
    // Java field: int LEGACY_WIRE_FRAME
    pub const LEGACY_WIRE_FRAME: i32 = 6;
    // Java field: int MATTE
    pub const MATTE: i32 = 7;
    // Java field: int METAL
    pub const METAL: i32 = 8;
    // Java field: int PLASTIC
    pub const PLASTIC: i32 = 9;
    // Java field: int POWDER
    pub const POWDER: i32 = 10;
    // Java field: int SOFT_EDGE
    pub const SOFT_EDGE: i32 = 11;
    // Java field: int SOFT_METAL
    pub const SOFT_METAL: i32 = 12;
    // Java field: int TRANSLUCENT_POWDER
    pub const TRANSLUCENT_POWDER: i32 = 13;
    // Java field: int WARM_MATTE
    pub const WARM_MATTE: i32 = 14;
    // Java field: int NONE
    pub const NONE: i32 = -1;
    // Java: PRESET_MATERIAL()
    // Java: super();
}

// Java: public class LINE_JOIN
pub mod LINE_JOIN {
    // Java field: int BEVEL
    pub const BEVEL: i32 = 0;
    // Java field: int MITER
    pub const MITER: i32 = 1;
    // Java field: int ROUND
    pub const ROUND: i32 = 2;
    // Java: LINE_JOIN()
    // Java: super();
}

// Java: public class COMPOUND_LINE
pub mod COMPOUND_LINE {
    // Java field: int SINGLE_LINE
    pub const SINGLE_LINE: i32 = 0;
    // Java field: int DOUBLE_LINES
    pub const DOUBLE_LINES: i32 = 1;
    // Java field: int THICK_THIN
    pub const THICK_THIN: i32 = 2;
    // Java field: int THIN_THICK
    pub const THIN_THICK: i32 = 3;
    // Java field: int THIN_THICK_THIN
    pub const THIN_THICK_THIN: i32 = 4;
    // Java: COMPOUND_LINE()
    // Java: super();
}

// Java: public class LINE_CAP
pub mod LINE_CAP {
    // Java field: int ROUND
    pub const ROUND: i32 = 0;
    // Java field: int SQUARE
    pub const SQUARE: i32 = 1;
    // Java field: int FLAT
    pub const FLAT: i32 = 2;
    // Java: LINE_CAP()
    // Java: super();
}
