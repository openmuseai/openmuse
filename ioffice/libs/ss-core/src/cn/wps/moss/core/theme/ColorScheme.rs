use super::ColorElement::ColorElement;
use super::ThemeColorUtil::ThemeColorUtil;
use crate::java_runtime::Rc;
use std::sync::RwLock;
// java static final int[] clrScheme=new int[]{...};private static List<ColorElement> sElements=new ArrayList();
static ELEMENTS: RwLock<Vec<Rc<ColorElement>>> = RwLock::new(Vec::new());
const CLR_SCHEME: [i32; 12] = [
    0, 16777215, 2050429, 15658209, 5210557, 12603469, 10206041, 8414370, 4959430, 16225862, 255,
    8388736,
];
pub struct ColorScheme;
impl ColorScheme {
    // java public static void clean() {sElements.clear();}
    pub fn clean() {
        ELEMENTS.write().unwrap().clear();
    }
    // java public static void addColor(ColorElement colorElement) {sElements.add(colorElement);}
    pub fn addColor(value: Rc<ColorElement>) {
        ELEMENTS.write().unwrap().push(value);
    }
    // java public static int getColor(int theme,float tint)
    pub fn getColor(mut theme: i32, tint: f32) -> i32 {
        assert!(
            (0..=11).contains(&theme),
            "IndexOutOfBoundsException: {theme}"
        );
        assert!(
            !(tint > 1.0 || tint < -1.0),
            "IllegalStateException: {tint}"
        );
        // java switch(theme) {case 0:theme=1;break;case 1:theme=0;break;case 2:theme=3;break;case 3:theme=2;}
        theme = match theme {
            0 => 1,
            1 => 0,
            2 => 3,
            3 => 2,
            _ => theme,
        };
        // java if(0==sElements.size()) color=clrScheme[theme];else color=sElements.get(theme).colorValue();
        let elements = ELEMENTS.read().unwrap();
        let color = if elements.is_empty() {
            CLR_SCHEME[theme as usize]
        } else {
            elements[theme as usize].colorValue()
        };
        ThemeColorUtil::colorWithTint2RGB(
            (color >> 16) & 255,
            (color >> 8) & 255,
            color & 255,
            tint,
        )
    }
}
