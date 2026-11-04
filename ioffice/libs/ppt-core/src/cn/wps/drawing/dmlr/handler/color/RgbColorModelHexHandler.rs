use std::cell::RefCell;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// rust: ★2B-③-2 —— `FillObj`（影子）改为**真身** `data::fill::Fill`。
//   这是颜色→填充的**关键写回路径**：`p:spPr/a:solidFill/a:srgbClr` 的 RGB 值
//   必须落到真 `Fill.solidFill()` 的 `Color` 上，才满足"真树属性为真"。
use crate::cn::wps::show::util::layout_shape::ColorSlot;
use crate::cn::wps::drawing::data::fill::Fill::Fill;

// java public class RgbColorModelHexHandler implements ElementAttrHandler
pub struct RgbColorModelHexHandler {
    // java private Color color
    color: Rc<RefCell<ColorSlot>>,
    fill: Option<Rc<RefCell<Fill>>>,
}

impl RgbColorModelHexHandler {
    // java public RgbColorModelHexHandler(Color color)
    pub fn RgbColorModelHexHandler(
        color: Rc<RefCell<ColorSlot>>,
        fill: Option<Rc<RefCell<Fill>>>,
    ) -> Self {
        color.borrow_mut().putMode(1);
        Self { color, fill }
    }

    // java public void addElementAttr(int element, XmlRoAttr attrsNotCopy)
    pub fn addElementAttr(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // java if ((tmp = attrsNotCopy.getByName(1114158)) != null)
        if let Some(tmp) = attrsNotCopy.get_by_name(1114158) {
            // java int[] rgbClr = ValueHelper.colorToByte(tmp.toString());
            let s = tmp.to_string();
            let hex = s.trim_start_matches('#');
            if let Ok(rgb) = u32::from_str_radix(hex, 16) {
                let packed = (0xFF000000u32 | rgb) as i32;
                self.color.borrow_mut().srgb = Some(packed);
                if let Some(fill) = &self.fill {
                    // rust: ★2B-③-2 —— Java: `this.fill.solidFill()` 后写 `srgb`/`mode`。
                    //   真身经 `Fill::solidFill_mut()` 取 `Color`，用真身访问器写入。
                    let mut f = fill.borrow_mut();
                    let c = f.solidFill_mut();
                    c.putMode(1);
                    // Java: this.color.srgb().putR(rgbClr[0]); / putG / putB
                    //
                    // rust: ★★★转写错误修正（R15 归因闭合）——
                    //   原实现构造一个**游离**的真 `RGB`，写入 r/g/b 后调 `c.writeSrgb(Some(&rgb))`。
                    //   但 `writeSrgb` 内部走 `value.lastBin(zone)` ⇒ `Factory::dup(this.box.get(), zone)`，
                    //   而 Java/Rust 的 `RGB::putR/putG/putB` **只写 `map`、不刷新 `box`**
                    //   （`box` 仅在 `saveBin()` 里 `map.dirty()` 时刷新）。
                    //   ⇒ `map[3]` 里落进去的是一个 `bh` 为空的句柄，随后 `Color::srgb()`
                    //     经 `RGB::open(map[3])` 解出的 r/g/b 全为 **0**。
                    //   实测：`hasSrgb()==true` 而 `r=g=b=0`（105/页），使
                    //   `apply_scheme_fill` 把 `pr.fill` 写成 `0xFF000000`（黑），
                    //   于是这些形状多出 `DrawRect` ⇒ R15 的 6 页漂移（§35.4）。
                    //   **Java 的写法是直接在 `Color` 缓存的那个 `RGB` 实例上写 r/g/b**
                    //   （`srgb()` 返回 `map[3]` 对应的**活对象**，`putR` 就地改其 `map`），
                    //   故这里逐字对齐：取活 `RGB`，就地 `putR/putG/putB`。
                    let live = c.srgb();
                    live.putR(((packed >> 16) & 0xFF) as i32);
                    live.putG(((packed >> 8) & 0xFF) as i32);
                    live.putB((packed & 0xFF) as i32);
                }
            }
        }
        // java ColorTransferReader.readColorTransferList(...)
        // rust: no-stmt
    }
}

impl ElementHandler for RgbColorModelHexHandler {
    fn kind(&self) -> &'static str {
        "RgbColorModelHexHandler"
    }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        self.addElementAttr(element, attrs);
    }
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}
