/* DEFERRED_NON_FUNCTION —— 以下两者依赖 `core::property` 闭包
 *   （`IPropertyCalculate` / `PropertyCalculateFactory`）：
 *     · `IndentsUtil`  —— 字段 `mCal: Option<Box<dyn IPropertyCalculate>>`，
 *       构造器调用 `PropertyCalculateFactory.create(document)`；
 *     · `KListTemplate` —— 同样以 `PropertyTool` + `core::property` 为类型面。
 *   登记 `core::property` 实测 99 错误、`core::PropertyTool` 单登记 48 错误，
 *   均对 docx 解析→排版→渲染零收益 ⇒ 按既定规则延后（与 `DateTimeParser`
 *   `getLanguageCode`、`HyperlinkParser.filterMath` 同一处置）。
 *   其余 5 个兄弟模块正常登记。 */
// pub mod IndentsUtil;
pub mod KList;
// pub mod KListTemplate;
/* DEFERRED_NON_FUNCTION —— `KListTemplates`（复数）依赖上面的
 *   `KListTemplate`（单数），随其一同延后；`KLists` 不依赖该链，正常登记。 */
// pub mod KListTemplates;
pub mod KLists;
pub mod KListLevel;
pub mod KListLevels;
