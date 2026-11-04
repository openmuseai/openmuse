#![allow(dead_code, non_snake_case, unused_variables, unused_imports)]

// Java: import cn.wps.drawing.data.table.TableStyle
// Java: import cn.wps.drawing.dmlr.handler.table.tablestyle.TableStyleHandler
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
// Java: import cn.wps.show.app.table.KmoTableStyleList
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;

/// `a:tblStyleLst` 的元素 id（`kfc/xsd/dml/A.java`）
pub const TBL_STYLE_LST: i32 = 1114530;
/// `a:tblStyle` 的元素 id（`kfc/xsd/dml/A.java`）
pub const TBL_STYLE: i32 = 1114528;
/// `a:tblStyleLst/@def` 的属性 id（`kfc/xsd/dml/A.java`）
pub const TBL_STYLE_LST_DEF: i32 = 1114529;

// java public class TableStyleListHandler extends DefaultElementHandler
//
// `rust:` 端口说明（已登记）：
// - Java 的 `mTableStyleList`（`cn.wps.show.app.table.KmoTableStyleList`）在 Rust 侧
//   （`app/table/KmoTableStyleList.rs`）没有构造函数，且其 `styleLst` 元素类型是本地桩
//   `TableStyle{style_id}`，与真实 `cn.wps.drawing.data.table.TableStyle::create()` 不是同一类型，
//   故端口把真实解析结果（`@def` + `a:tblStyle` 计数）保存在 handler 自身，
//   由 `PptxrTableStyles.open()` 写入 `KmoPresentation` 的装配面。
// - Java 的 `tableStyleHandler`（`TableStyleHandler` 子树）属于表格样式明细，
//   非本里程碑管线核心，见 DEFERRED 说明。
#[derive(Default)]
pub struct TableStyleListHandler {
    // java private KmoTableStyleList mTableStyleList;
    // rust: 见上方端口说明
    // java private HandlerCallback mHandlerCallback;
    // rust: 见 DEFERRED 说明
    // java private TableStyleHandler tableStyleHandler;
    // rust: 见 DEFERRED 说明
    // rust: 可观测面 —— `mTableStyleList.setDefaultStyle(temp.toString())` 的真实入参
    default_style: Option<String>,
    // rust: 可观测面 —— `mTableStyleList.addStyle(...)` 的调用次数
    style_count: i32,
}

impl TableStyleListHandler {
    // java public TableStyleListHandler(KmoTableStyleList tableStyleList, HandlerCallback handlerCallback)
    pub fn TableStyleListHandler() -> Self {
        // java this.mTableStyleList = tableStyleList;
        // java this.mHandlerCallback = handlerCallback;
        Self::default()
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
        // java XmlRoAttr temp = null;
        // java if ((temp = attrsNotCopy.getByName(1114529)) != null)
        if let Some(temp) = attrsNotCopy.get_by_name(TBL_STYLE_LST_DEF) {
            // java this.mTableStyleList.setDefaultStyle(temp.toString());
            self.default_style = Some(temp.to_string());
        }
    }

    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // java ElementHandler handler = null;
        let handler: Option<Box<dyn ElementHandler>> = None;
        // java switch (subElement)
        match subElement {
            // java case 1114528:
            TBL_STYLE => {
                // java TableStyle tableStyle = TableStyle.create();
                // java this.mTableStyleList.addStyle(tableStyle);
                self.style_count += 1;
                // java if (this.tableStyleHandler == null) { this.tableStyleHandler = new TableStyleHandler(tableStyle, this.mHandlerCallback); }
                // java else { this.tableStyleHandler.init(tableStyle); }
                /* DEFERRED_NON_FUNCTION: `TableStyleHandler`（`a:tblStyle` 明细子树）与
                 * `KmoTableStyleList.addStyle` 的端口面尚未建立 —— `KmoTableStyleList` 的
                 * `styleLst` 元素类型是本地桩 `TableStyle{style_id}`（无可写构造），
                 * 真实 `cn.wps.drawing.data.table.TableStyle::create()` 是另一个类型。
                 * owner: pp-04 tableStyles / wave: S2。 */
                // java handler = this.tableStyleHandler;
            }
            // java default: return handler;
            _ => {}
        }
        // java return handler;
        handler
    }

    // rust: 可观测面（Java 无此方法）
    pub fn defaultStyle(&self) -> Option<String> {
        self.default_style.clone()
    }
    pub fn styleCount(&self) -> i32 {
        self.style_count
    }
}

impl ElementHandler for TableStyleListHandler {
    fn kind(&self) -> &'static str {
        "TableStyleListHandler"
    }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        TableStyleListHandler::startElement(self, element, attrs)
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        TableStyleListHandler::enterSubElement(self, subElement)
    }
    fn endElement(&mut self, _element: i32) {}
}
