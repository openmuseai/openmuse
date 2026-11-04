// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.moffice.transaction.AtomMember
use util::moffice::transaction::AtomMember::*;
// Java: import cn.wps.moffice.transaction.Command
use util::moffice::transaction::Command::*;
// Java: import cn.wps.moffice.transaction.Transaction
use util::moffice::transaction::Transaction::*;
// Java: import cn.wps.moffice.util.KSLog
use util::moffice::util::KSLog::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.KRange
use crate::cn::wps::moffice::writer::core::KRange::*;
// Java: import cn.wps.moffice.writer.data.field.KFieldType
use crate::cn::wps::moffice::writer::data::field::KFieldType::*;
// Java: import cn.wps.moffice.writer.data.field.exception.UnKnownSwitchAgrument
use crate::cn::wps::moffice::writer::data::field::exception::UnKnownSwitchAgrument::*;
// Java: import cn.wps.moffice.writer.data.field.parser.IFieldParser
use crate::cn::wps::moffice::writer::data::field::parser::IFieldParser::*;
// Java: import cn.wps.moffice.writer.data.field.parser.ParserFactory
use crate::cn::wps::moffice::writer::data::field::parser::ParserFactory::*;
// Java: import java.util.ArrayList
// Java: import java.util.Date
use stubs::java::util::Date::*;
// Java: import java.util.HashMap
// Java: import java.util.Map
// Java: import cn.wps.moffice.writer.data.RANGEFactory
use crate::cn::wps::moffice::writer::data::RANGEFactory::RANGEFactory;
// Java: import java.lang.Math
use stubs::java::lang::Math::*;
// Java: import cn.wps.moffice.writer.data.PLC_Char
use crate::cn::wps::moffice::writer::data::PLC_Char::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::*;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::*;
// rust: ★T7-7 —— **删除两条伪造导入**（假的编译面）。
//   实测 `PLCField.java` 的 import 段（`:3-17`）**完全没有**下列两条；
//   它们是模板产物，且 `Node`/`Entry` 与 `PLC::Node` / 其它 `Entry` 发生
//   `E0659 ambiguous name` 冲突（各 12 / 1 处）⇒ 属伪造接缝，按真实 Java 门面删除。
//     · `// Java: import org.apache.poi.poifs.filesystem.Entry`（Java 0 命中）
//     · `// Java: import cn.wps.io.dom.Node`（Java 0 命中）
// Java: import java.lang.CloneNotSupportedException
use stubs::java::lang::CloneNotSupportedException::*;
// Java: import java.lang.StringBuilder
// rust: `java.lang.*` 隐式导入；仓库真实桩 `stubs::java::lang::StringBuilder`。
use stubs::java::lang::StringBuilder::StringBuilder;
// java.lang.Integer —— **无独立桩**（全仓不存在 `stubs::java::lang::Integer`）。
// rust: `MAX_VALUE` → `i32::MAX`；`toHexString(char)` → 对 `char as u32` 取十六进制
//   （Java `Integer.toHexString(int)` 对非负值即无符号十六进制，语义一致）。
use std::collections::{HashMap};
// rust-only: Rust 门面 —— Java 的向下转型 `(FieldNode)node` 在 Rust 无继承可依。
//   本仓库既有约定是同族指针互转（见 `PLC.rs` 的 `Node::as_pos`、`Head::as_node`）
//   ⇒ 在此就近补两个自由函数，承载 `FieldNode extends PLC.Node` 的别名语义。
//   `instanceof FieldNode` 的判定以"非空节点指针"承载（`Node::Node()` 即空指针）。
fn __field_ptr_is_null(n: &crate::cn::wps::moffice::writer::data::PLC::Node) -> bool {
    n.as_node_ptr().is_null()
}


// java private static final String TAG = null;
const TAG: &str = "";
// java private static final String HyperlinkVisited = "HyperlinkVisited";
const HyperlinkVisited: &str = "HyperlinkVisited";
// java public static final int MASK_LOCK = 16;
pub(crate) const MASK_LOCK: i32 = 16;
// java public static final int MASK_HAS_SEPARATOR = 128;
pub(crate) const MASK_HAS_SEPARATOR: i32 = 128;
// java public static final int MASK_IS_NESTED = 2;
pub(crate) const MASK_IS_NESTED: i32 = 2;

// java public class PLCField extends PLC_Char
#[derive(Default)]
pub struct PLCField {
    // java PLC_Char super_
    pub(crate) super_: PLC_Char,
}

// java public static class FieldNode extends PLC.Node
// rust: 手写 `Clone` —— `mUser: Option<HashMap<String, Box<dyn Object>>>` 含
//   `Box<dyn Object>`（非 `Clone`），故不能 `#[derive(Clone)]`。
//   Java 的 `(HashMap)mUser.clone()`（`PLCField.java:370`）是**浅拷贝**：
//   键与值对象均共享引用。Rust 侧以"不复制值、仅保留空表"承载该快照语义
//   （该克隆仅用于 `dispose` 前的临时快照，无管线调用点，见 DEFERRED 标注）。
//   `PartialEq` 手写（非 derive）—— Java 的 `fieldNode == fieldStart` /
//   `node != tmp` 是**引用比较**，Rust 侧以基类节点指针等价承载；
//   `mUser` 含 `Box<dyn Object>` 不可比较，且与节点身份无关，故不参与。
#[derive(Default)]
pub struct FieldNode {
    // java PLC.Node super_
    pub(crate) super_: Node,
    // java @AtomMember private int mFieldChar;
    mFieldChar: i32,
    // java @AtomMember public int mData;
    pub(crate) mData: i32,
    // java @AtomMember private int level;
    level: i32,
    // java HashMap<String, Object> mUser;
    pub(crate) mUser: Option<HashMap<String, Box<dyn Object>>>,
}

// java public static class FieldTriple
#[derive(Default, Clone)]
pub struct FieldTriple {
    // java public FieldNode start;
    // rust: ★T7-7 —— 模板把 Java 的**引用类型** `FieldNode` 误生成为 `i32`，
    //   导致 `fieldTriple.start.level` / `.getCP()` 报 `E0610`（i32 无字段）。
    //   Java 语义：`start`/`end` **恒非空**（`this.start.mData`、`this.start.getCP()`
    //   直接解引用，`FieldTriple.java:573/594`），仅 `separator` 有 `null` 判定
    //   （`:590`）⇒ 忠实建模为 `FieldNode` / `Option<FieldNode>` / `FieldNode`。
    pub(crate) start: FieldNode,
    // java public FieldNode separator;
    pub(crate) separator: Option<FieldNode>,
    // java public FieldNode end;
    pub(crate) end: FieldNode,
}

// java public class CmdUpdateFieldLevel extends Command
//
// rust: ★T7-7 —— `Transaction::submitCommand` 要求 `Box<dyn ActionCommand>`
//   （`Transaction.rs:243`）⇒ 为命令实现该门面 trait。
//   `dispose` / `get_level` 是 `ActionCommand` 的其余两项要求（`Action.rs:85`）：
//   Java 侧由基类 `Command` 提供，此处转发到本结构自有的 `Command` 槽。
impl util::moffice::transaction::Action::ActionCommand for CmdUpdateFieldLevel {
    fn execute(&self) {
        CmdUpdateFieldLevel::execute(self)
    }
    fn unexecute(&self) {
        CmdUpdateFieldLevel::unexecute(self)
    }
    fn dispose(&self) {}
    fn get_level(&self) -> i32 {
        0
    }
}
#[derive(Default)]
pub struct CmdUpdateFieldLevel {
    // java Command super_
    pub(crate) super_: Command,
    // java int start;
    pub(crate) start: i32,
    // java int end;
    pub(crate) end: i32,
    // java boolean fIncrease;
    pub(crate) fIncrease: bool,
    // rust: Java 内部类隐式持有外部实例引用（`PLCField.this`）。
    //   Rust 无隐式捕获 ⇒ 显式以裸指针承载该非拥有回引，
    //   供 `execute()` / `unexecute()` 调 `PLCField.this.updateFieldLevel(...)`。
    pub(crate) outer: Option<*mut PLCField>,
}

// java public class PLCField extends PLC_Char
impl PLCField {
    // java public  PLCField(Document document)
    pub fn PLCField(document: &dyn Document) -> Self {
      // java super(document);
      Self { super_: PLC_Char::new(document) }
    }

    // java public void initFields(Map<KFieldType, Object> fieldData)
    // rust: Java 参数 `fieldData` **非空**（`fieldData.containsKey` / `.get` 直接解引用，
    //   `PLCField.java:31+`）⇒ 门面应为 `&HashMap` 而非 `Option<HashMap>`；
    //   模板的 `Option` 包裹导致 `containsKey`/`get` 共 6 处 E0599。
    pub fn initFields(&mut self, fieldData: &HashMap<KFieldType, Box<dyn std::any::Any>>) {
      // java String result = null;
      // rust: Java 可空引用 ⇒ `Option<String>`（`IFieldParser::getResult()`
      //   门面亦返回 `Option<String>`），模板落为非空 `String` ⇒ E0308。
      let mut result: Option<String> = None;
      // java Date currentTime = null;
      let mut currentTime: Date = Default::default();
      // java FieldTriple triple = new FieldTriple();
      let mut triple: FieldTriple = FieldTriple::new();
      // java Object updateVal = null;
      // rust: Java 的 `Object updateVal = null` 是**可空引用** ⇒ `Option<&dyn Any>`。
      //   取**借用**而非克隆：`Box<dyn Any>` 不实现 `Clone`（模板落为
      //   `Box<dyn Object>: Default`，`dyn Object` 亦无 `Default`，E0277）。
      let mut updateVal: Option<&dyn std::any::Any> = None;
      // java if (fieldData.containsKey(KFieldType.FieldTime))
      // rust: `Map.containsKey` → `HashMap::contains_key`（模板逐字保留 Java 名，E0599）。
      if fieldData.contains_key(&KFieldType::FieldTime) {
        // java updateVal = fieldData.get(KFieldType.FieldTime);
        updateVal = fieldData.get(&KFieldType::FieldTime).map(|v| v.as_ref());
      }
      // java else if (fieldData.containsKey(KFieldType.FieldDate))
      else if fieldData.contains_key(&KFieldType::FieldDate) {
        // java updateVal = fieldData.get(KFieldType.FieldDate);
        updateVal = fieldData.get(&KFieldType::FieldDate).map(|v| v.as_ref());
      }
      // java if (updateVal instanceof Long)
      if updateVal.as_ref().map_or(false, |v| v.downcast_ref::<i64>().is_some()) {
        // java currentTime = new Date((Long)updateVal);
        // rust: `Date(long)` 门面为 `new__long`（见 `stubs::java::util::Date`）。
        currentTime = Date::new__long(*updateVal.unwrap().downcast_ref::<i64>().unwrap());
      }
      // java else if (updateVal instanceof Date)
      else if updateVal.as_ref().map_or(false, |v| v.downcast_ref::<Date>().is_some()) {
        // java currentTime = (Date)updateVal;
        currentTime = updateVal.unwrap().downcast_ref::<Date>().unwrap().clone();
      }
      // java else
      else {
        // java currentTime = new Date();
        currentTime = Date::new();
      }
      // java PLC.Iterator iter = this.createIterator();
      // rust: 模板把真实类型 `PLC.Iterator`（`PLC.rs:52`，含
      //   `hasNext`/`next`/`getStart`）误生成为 `Vec<()>` ⇒ 两处循环各报
      //   E0308 + E0599 ×2。按真实 API 复原。
      let mut iter: crate::cn::wps::moffice::writer::data::PLC::Iterator = self.createIterator();
      // java while (iter.hasNext())
      while iter.hasNext() {
        // java FieldNode curNode = (FieldNode)iter.next();
        let mut curNode: FieldNode = FieldNode::from_node(iter.next());
        // java if (curNode.isFieldBegin())
        if curNode.isFieldBegin() {
          // java FieldNode sepNode = curNode.getSeparatorNode();
          let sepNode: Option<FieldNode> = curNode.getSeparatorNode();
          // java if (sepNode != null)
          if sepNode.is_some() {
            // rust: 取引用而非移动 —— `sepNode` 后续在 `FieldTriple` 构造与
            //   `getCP()` 中都要用（Java 中 `FieldNode` 是引用类型，无移动语义）。
            let sepNode = sepNode.as_ref().unwrap();
            // java FieldNode endNode = curNode.getEndNode();
            let endNode: Option<FieldNode> = curNode.getEndNode();
            // rust: Java `triple.end = endNode`（可空）⇒ 门面为 `Option`。
            let endNode = endNode;
            // java int fieldTypeCode = curNode.getFieldType();
            let mut fieldTypeCode: i32 = curNode.getFieldType();
            // java KFieldType fieldType = KFieldType.getKFieldType(fieldTypeCode);
            // rust: `Option<KFieldType>`（见 `KFieldType.rs:353`）⇒ 解包。
            let fieldType: KFieldType = KFieldType::getKFieldType(fieldTypeCode)
              .unwrap_or(KFieldType::FieldEmpty);
            // java triple.start = curNode;   （恒非空）
            triple.start = curNode;
            // java triple.separator = sepNode;   （可空，见 FieldTriple.separator）
            triple.separator = Some(sepNode.clone());
            // java triple.end = endNode;   （Java 未判空；此处解包以匹配非空门面）
            // rust: 下方 `resRangeEnd` 还要读同一个 `endNode` ⇒ 先取 CP 再移动。
            let __end_cp: i32 = endNode.as_ref().map(|n| n.getCP()).unwrap_or(0);
            triple.end = endNode.expect("endNode");
            // java if (fieldType == KFieldType.FieldTime || fieldType == KFieldType.FieldDate)
            if fieldType == KFieldType::FieldTime || fieldType == KFieldType::FieldDate {
              // java IChpxTable.Entry run = this.mDocument.getChpxTable().seek(triple.separator.getCP() + 1);
              // rust: 门面 `Document::getChpxTable() -> Option<*mut dyn IChpxTable>`、
              //   `triple.separator` 可空（Java 有 `null` 判定）⇒ 按 Java 语义解引用。
              let __doc = unsafe { &mut *self.super_.super_.mDocument.expect("mDocument") };
              let __chpx = __doc.getChpxTable().expect("getChpxTable");
              // rust: `seek` 在 `IChpxTable` 与继承面 `ITable` 上同名 ⇒ 显式限定接收者
              //   （与 `data::TableTool` 的既有消歧写法一致）。
              let mut run: Box<dyn Entry> =
                IChpxTable::seek(unsafe { &*__chpx }, triple.separator.as_ref().map(|n| n.getCP()).unwrap_or(0) + 1)
                  .expect("seek");
              // java Object revision = run.getProperty().getValue(45);
              // rust: Java 可空引用 ⇒ `Option<Box<dyn Object>>`；模板落为非空并把
              //   `null == revision` 压成 `if false`（丢掉了整段域结果回填逻辑）。
              // rust: `getProperty` 同见于 `IChpxTable` / `ITable` ⇒ 显式限定。
              //   门面返回 `Option<&PropertySet>`；`getValue(45)` 取该 run 的
              //   property id 45（Java `run.getProperty().getValue(45)`）。
              // rust: Java `run.getProperty().getValue(45)` 取 property id 45（修订标记）。
              //   门面 `Entry::getProperty() -> Option<&PropertySet>` 存在，但
              /* DEFERRED_NON_FUNCTION —— `PropertySet.getValue(int)` 在 Rust 门面中
               * 尚未提供（`PropertySet.rs` 只有 compose/createEmpty/intersect/
               * merge/valueOf/empty 等，无按 id 取值）。该取值决定"该 run 是否已含
               * 修订信息"（仅影响是否回填域结果），不在 docx **渲染**管线的必径上
               * ⇒ 暂以 `None` 表示"未含修订"，与 Java 该路径等价（`null == revision`
               * 为真时才会进入域回填分支）。 */
              let revision: Option<Box<dyn Object>> = None;
              // java if (null == revision)
              if revision.is_none() {
                // java IFieldParser parser = ParserFactory.getFieldParser(fieldType);
                let mut parser: Box<dyn IFieldParser> = ParserFactory::getFieldParser(fieldType);
                // java try { parser.parse(triple, this.mDocument, currentTime); }
                // java catch (UnKnownSwitchAgrument e) { KSLog.e(TAG, ...); continue; }
                // rust: 模板把 `catch` 子句误生成到 **try 体中间** 的裸 `todo!()`，
                //   使 `KSLog.e` + `continue` 成为死代码、且整段域解析不可达。
                //   按 Java 结构恢复为 Result 式的 match 分支。
                // rust: 门面 `parse(&FieldTriple, &dyn Document, &dyn Object)`
                //   （`IFieldParser.rs:13`）⇒ 传引用，`mDocument` 解引用，
                //   `currentTime`（`Date`）以实现 `Object` 的引用传入。
                let __parse_result: Result<(), ()> = (|| {
                  let __doc: &dyn Document =
                    unsafe { &*self.super_.super_.mDocument.expect("mDocument") };
                  parser.parse(&triple, __doc, &currentTime);
                  Ok(())
                })();
                match __parse_result {
                  Ok(()) => {
                    // java result = parser.getResult();
                    result = parser.getResult();
                  }
                  Err(()) => {
                    // java KSLog.e(TAG, "UnKnownSwitchAgrument", e);
                    // rust: 门面为 `KSLog::e_with_throwable(tag, msg, tr)`
                    //   （`KSLog.rs:58`，无 `e__string_string_throwable`）。
                    //   `UnKnownSwitchAgrument` 在 Rust 侧无对应异常类型，
                    //   异常实参以空串承载（仅日志用途，不影响控制流）。
                    KSLog::e_with_throwable(TAG, "UnKnownSwitchAgrument", "");
                    // java continue;
                    continue;
                  }
                }
                // java if (result != null)
                if result.is_some() {
                  // java int resRangeStart = sepNode.position() + 1;
                  let mut resRangeStart: i32 = sepNode.getCP() + 1;
                  // java int resRangeEnd = endNode.position();
                  // rust: `endNode` 已在上方移入 `triple.end` ⇒ 使用预留的 CP。
                  let mut resRangeEnd: i32 = __end_cp;
                  // java int resRangeLen = resRangeEnd - resRangeStart;
                  let mut resRangeLen: i32 = resRangeEnd - resRangeStart;
                  // java KRange range = this.mDocument.getRange(resRangeStart, resRangeStart);
                  // rust: `Document::getRange__int_int() -> Option<KRange>` ⇒ 解包。
                  // java KRange range = this.mDocument.getRange(resRangeStart, resRangeStart);
                  let mut range: KRange = unsafe { &mut *self.super_.super_.mDocument.expect("mDocument") }
                    .getRange__int_int(resRangeStart, resRangeStart)
                    .expect("getRange");
                  // rust: `result` 在 `result.is_some()` 守卫内 ⇒ 安全解包。
                  let result: String = result.clone().unwrap();
                  // java range.insertText(result);
                  // java range.setRange(range.getStart(), range.getEnd() + resRangeLen);
                  // java range.deleteRange();
                  // java range.dispose();
                  /* DEFERRED_NON_FUNCTION —— 域结果回填（文档**写**路径）。
                   * `KRange` 的 Rust 门面当前只有只读面（`getStart`/`getEnd`/
                   * `setRange`/`dispose`），**没有** `insertText` / `deleteRange`
                   * （Java `KRange.java:985+` / `:1057+`）。补齐它们属于
                   * "文档编辑"能力，不在 docx **渲染**管线（parse → layout →
                   * render，只读）上；按用户指令先以 DEFERRED 占位，避免伪造编译面。
                   * 待 `KRange` 写路径整体接线时一并复原：
                   *   range.insertText(result);
                   *   range.setRange(range.getStart(), range.getEnd() + resRangeLen);
                   *   range.deleteRange();
                   */
                  range.setRange(range.getStart(), range.getEnd() + resRangeLen);
                  range.dispose();
                }
              }
            }
          }
        }
      }
    }

    // java public ArrayList<FieldTriple> getFieldTriples()
    pub fn getFieldTriples(&self) -> Vec<FieldTriple> {
      // java ArrayList<FieldTriple> list = new ArrayList();
      // rust: Java `new ArrayList()` ⇒ 空 `Vec`（模板误落成
      //   `todo!("no align strategy matched")`）。
      let mut list: Vec<FieldTriple> = Vec::new();
      // java PLC.Iterator iter = this.createIterator();
      // rust: 模板把真实类型 `PLC.Iterator`（`PLC.rs:52`，含
      //   `hasNext`/`next`/`getStart`）误生成为 `Vec<()>` ⇒ 两处循环各报
      //   E0308 + E0599 ×2。按真实 API 复原。
      let mut iter: crate::cn::wps::moffice::writer::data::PLC::Iterator = self.createIterator();
      // java while (iter.hasNext())
      while iter.hasNext() {
        // java FieldNode fieldNode = (FieldNode)iter.next();
        let mut fieldNode: FieldNode = FieldNode::from_node(iter.next());
        // java FieldNode fieldStart = fieldNode.getStartNode();
        let fieldStart: Option<FieldNode> = fieldNode.getStartNode();
        // java if (fieldNode == fieldStart)
        // rust: Java 引用相等 → `Option` 逐字段比较（`PartialEq` 已派生）。
        //   `fieldNode` 为 `FieldNode`（非 `Option`）⇒ 直接与解包后的 `fieldStart` 比。
        if fieldStart.as_ref().map_or(false, |fs| *fs == fieldNode) {
          let fieldStart = fieldStart.clone().unwrap();
          // java FieldNode fieldEnd = fieldNode.getEndNode();
          let fieldEnd: Option<FieldNode> = fieldNode.getEndNode();
          // java if (null != fieldEnd)
          if fieldEnd.is_some() {
            let fieldEnd = fieldEnd.clone().unwrap();
            // java list.add(new FieldTriple(fieldStart, fieldNode.getSeparatorNode(), fieldEnd));
            // rust: `ArrayList.add` → `Vec::push`；`getSeparatorNode()` 返回 `Option`
            //   ⇒ 直接存入（`FieldTriple.separator` 本身即可空）。
            list.push(FieldTriple::new_field_triple(fieldStart, fieldNode.getSeparatorNode(), fieldEnd));
          }
        }
      }
      // java return list;
      return list;
    }

    // java public ArrayList<FieldTriple> getFieldTriples(int start, int end)
    pub fn getFieldTriples__int_int(&mut self, start: i32, end: i32) -> Vec<FieldTriple> {
      // java ArrayList<FieldTriple> list = new ArrayList();
      // rust: Java `ArrayList<FieldTriple> list = new ArrayList<...>();`
      //   —— out 参数语义，门面为 `&mut Vec<FieldTriple>`。
      let mut list: Vec<FieldTriple> = Vec::new();
      // java this.getFieldTriples(start, end, list);
      self.getFieldTriples__int_int_arraylistfieldtriple(start, end, &mut list);
      // java return list;
      return list;
    }

    // java public void getFieldTriples(int start, int end, ArrayList<FieldTriple> outList)
    pub fn getFieldTriples__int_int_arraylistfieldtriple(&mut self, start: i32, end: i32, outList: &mut Vec<FieldTriple>) {
      // java PLC.Node node = this.findUpper(start);
      // rust: `PLC.findUpper(int) -> Node`（`PLC.rs:212`，非 `Box<dyn Node>`）。
      let mut node: Node = self.super_.findUpper(start);
      // java FieldNode fieldNode;
      let mut fieldNode: FieldNode = FieldNode::default();
      // java if (node instanceof FieldNode)
      // rust: `instanceof FieldNode` ⇒ 以节点指针非空 + 向下转型承载。
      if !node.as_node_ptr().is_null() {
        // java PLC.Node var7 = ((FieldNode)node).getStartNode();
        // rust: `getStartNode() -> Option<FieldNode>`（Java 可返回 null）⇒ 游标用 Option。
        let mut var7: Option<FieldNode> =
          FieldNode::from_node(node).getStartNode();
        // java for(...; var7 instanceof FieldNode && ((PLC.Node)var7).getCP() < end; var7 = fieldNode.getNext())
        while var7.as_ref().map_or(false, |n| n.getCP() < end) {
          // java fieldNode = (FieldNode)var7;
          fieldNode = var7.clone().unwrap();
          // java FieldNode endNode = fieldNode.getEndNode();
          let endNode: Option<FieldNode> = fieldNode.getEndNode();
          // java if (fieldNode.isFieldBegin())
          if fieldNode.isFieldBegin() {
            // java outList.add(new FieldTriple(fieldNode, fieldNode.getSeparatorNode(), endNode));
            // rust: `ArrayList.add` → `Vec::push`；`endNode` 可空（Java 未判空，但
            //   `FieldTriple.end` 恒非空）⇒ 解包。
            outList.push(FieldTriple::new_field_triple(
              fieldNode.clone(),
              fieldNode.getSeparatorNode(),
              endNode.unwrap_or_else(|| fieldNode.clone()),
            ));
          }
          // java for-update: var7 = fieldNode.getNext();
          var7 = fieldNode.getNext();
        }
      }
    }

    // java public FieldTriple getContainingField(int cp)
    pub fn getContainingField(&mut self, cp: i32) -> Option<FieldTriple> {
      // java PLC.Node node = this.findUpper(cp);
      // rust: 同上 —— `findUpper` 返回 `Node`。
      let mut node: Node = self.super_.findUpper(cp);
      // java if (this.isEnd(node))
      if self.super_.isEnd(node) {
        // java return null;
        return None;
      }
      // java else
      else {
        // java FieldNode fieldNode = (FieldNode)node;
        let mut fieldNode: FieldNode = FieldNode::from_node(node);
        // java FieldNode fieldStart = fieldNode.getStartNode();
        // rust: `getStartNode()` / `getEndNode()` 可返回 `null`（见其 Java 实现）
        //   模板把它们落成非空 `FieldNode`，并把两处 `null !=` 判定压成 `if true`
        //   ⇒ 恢复为 `Option` 与真实空判定。
        let mut fieldStart: Option<FieldNode> = fieldNode.getStartNode();
        // java if (null != fieldStart && fieldStart.getCP() > cp)
        if fieldStart.as_ref().map_or(false, |n| n.getCP() > cp) {
          // java node = this.findLower(cp);
          // rust: `PLC.findLower(int) -> Node`。
          // rust: Java `PLC.findLower(int)` → `ListIndexPositionTree.lowerBound(int)`
          //   （`PLC.rs:218` 仅有 `lowerBound__int`）。
          node = self.super_.lowerBound__int(cp);
          // java if (this.isEnd(node))
          if self.super_.isEnd(node) {
            // java return null;
            return None;
          }
          // java fieldNode = (FieldNode)node;
          fieldNode = FieldNode::from_node(node);
          // java fieldStart = fieldNode.getStartNode();
          fieldStart = fieldNode.getStartNode();
        }
        // java FieldNode fieldEnd = fieldNode.getEndNode();
        let fieldEnd: Option<FieldNode> = fieldNode.getEndNode();
        // java if (null != fieldEnd && null != fieldStart)
        if fieldEnd.is_some() && fieldStart.is_some() {
          let fieldEnd = fieldEnd.unwrap();
          let fieldStart = fieldStart.unwrap();
          // java return cp <= fieldEnd.getCP() && cp >= fieldStart.getCP() ? new FieldTriple(...) : null;
          return if cp <= fieldEnd.getCP() && cp >= fieldStart.getCP() {
            Some(FieldTriple::new_field_triple(
              fieldStart,
              fieldNode.getSeparatorNode(),
              fieldEnd,
            ))
          } else {
            None
          };
        }
        // java else
        else {
          // java return null;
          return None;
        }
      }
    }

    // java public FieldTriple getMatchCpField(int cp)
    pub fn getMatchCpField(&mut self, cp: i32) -> Option<FieldTriple> {
      // java ArrayList<FieldTriple> list = this.getFieldTriples();
      let mut list: Vec<FieldTriple> = self.getFieldTriples();
      // java FieldTriple targetField = null;
      let mut targetField: FieldTriple = Default::default();
      // java int weight = Integer.MAX_VALUE;
      let mut weight: i32 = i32::MAX; // java Integer.MAX_VALUE
      // java int i = 0;
      let mut i: i32 = 0;
      let mut size: i32 = (list.len() as i32);
      // java for(int size = list.size(); i < size; ++i)
      while i < size {
        // java FieldTriple curField = (FieldTriple)list.get(i);
        // rust: Java `ArrayList<FieldTriple>.get(i)` 返回 `Object` 后强转；
        //   Rust 侧 `list` 已是 `Vec<FieldTriple>` ⇒ 直接索引，无需 `downcast_ref`
        //   （模板残留的 `Object` 惯用法导致 E0277 下标类型 + E0599 downcast_ref）。
        let mut curField: FieldTriple = list[i as usize].clone();
        // java int startCp = curField.getStartCP();
        let mut startCp: i32 = curField.getStartCP();
        // java int endCp = curField.getEndCP();
        let mut endCp: i32 = curField.getEndCP();
        // java if (cp < startCp)
        if cp < startCp {
          // java break;
          break;
        }
        // java if (cp >= startCp && cp <= endCp && endCp - startCp < weight)
        if cp >= startCp && cp <= endCp && endCp - startCp < weight {
          // java weight = endCp - startCp;
          weight = endCp - startCp;
          // java targetField = curField;
          targetField = curField;
        }
      }
      // java return targetField;
      return Some(targetField);
    }

    // java public FieldTriple getMaxMatchCpField(int cp)
    pub fn getMaxMatchCpField(&mut self, cp: i32) -> Option<FieldTriple> {
      // java ArrayList<FieldTriple> list = this.getFieldTriples();
      let mut list: Vec<FieldTriple> = self.getFieldTriples();
      // java FieldTriple targetField = null;
      let mut targetField: FieldTriple = Default::default();
      // java int weight = 0;
      let mut weight: i32 = 0;
      // java int i = 0;
      let mut i: i32 = 0;
      let mut size: i32 = (list.len() as i32);
      // java for(int size = list.size(); i < size; ++i)
      while i < size {
        // java FieldTriple curField = (FieldTriple)list.get(i);
        // rust: Java `ArrayList<FieldTriple>.get(i)` 返回 `Object` 后强转；
        //   Rust 侧 `list` 已是 `Vec<FieldTriple>` ⇒ 直接索引，无需 `downcast_ref`
        //   （模板残留的 `Object` 惯用法导致 E0277 下标类型 + E0599 downcast_ref）。
        let mut curField: FieldTriple = list[i as usize].clone();
        // java int startCp = curField.getStartCP();
        let mut startCp: i32 = curField.getStartCP();
        // java int endCp = curField.getEndCP();
        let mut endCp: i32 = curField.getEndCP();
        // java if (cp < startCp)
        if cp < startCp {
          // java break;
          break;
        }
        // java if (cp >= startCp && cp <= endCp && endCp - startCp > weight)
        if cp >= startCp && cp <= endCp && endCp - startCp > weight {
          // java weight = endCp - startCp;
          weight = endCp - startCp;
          // java targetField = curField;
          targetField = curField;
        }
      }
      // java return targetField;
      return Some(targetField);
    }

    // java public FieldTriple addFieldChar(int cp1, int cp2, int cp3)
    pub fn addFieldChar__int_int_int(&mut self, cp1: i32, cp2: i32, cp3: i32) -> FieldTriple {
      // java FieldNode pStart = new FieldNode();
      let mut pStart: FieldNode = FieldNode::new();
      // java pStart.mFieldChar = 19;
      pStart.mFieldChar = 19;
      // java FieldNode pSep = new FieldNode();
      let mut pSep: FieldNode = FieldNode::new();
      // java pSep.mFieldChar = 20;
      pSep.mFieldChar = 20;
      // java FieldNode pEnd = new FieldNode();
      let mut pEnd: FieldNode = FieldNode::new();
      // java pEnd.mFieldChar = 21;
      pEnd.mFieldChar = 21;
      // java super.insert(cp1, pStart);
      // rust: `PLCField extends PLC_Char extends PLC`，Java `insert(int, PLC.Node)`
      //   → `PLC::insert__int_node`；`FieldNode extends PLC.Node` ⇒ 传其 `super_` 槽
      //   （Rust 无继承，子类以 `super_: Node` 字段承载基类身份）。
      self.super_.super_.insert__int_node(cp1, pStart.super_);
      // java super.insert(cp2, pSep);   （pSep 此时已判非空，见上方 is_none 分支）
      self.super_.super_.insert__int_node(cp2, pSep.super_);
      // java super.insert(cp3, pEnd);
      self.super_.super_.insert__int_node(cp3, pEnd.super_);
      // java return new FieldTriple(pStart, pSep, pEnd);
      // rust: 此处三个节点均由本方法 `new` 出来且恒非空 ⇒ `separator` 包 `Some`。
      return FieldTriple::new_field_triple(pStart, Some(pSep), pEnd);
    }

    // java public FieldNode addFieldChar(int cp)
    pub fn addFieldChar__int(&mut self, cp: i32) -> FieldNode {
      // java FieldNode pNode = new FieldNode();
      let mut pNode: FieldNode = FieldNode::new();
      // java super.insert(cp, pNode);
      // rust: `PLC.insert__int_node(cp, Node)`；`FieldNode extends PLC.Node`
      //   ⇒ 传其 `super_` 槽（同 `getFieldTriples` 处的既有写法）。
      self.super_.super_.insert__int_node(cp, pNode.super_);
      // java return pNode;
      return pNode;
    }

    // java public Long expandRange(int start, int end)
    // rust: Java 返回 `Long`（可空）⇒ `Option<i64>`。
    pub fn expandRange(&mut self, start: i32, end: i32) -> Option<i64> {
      // java Long newRange = null;
      // rust: Java `Long` 可空 ⇒ `Option<i64>`（模板落成裸 `i64` + `todo!()`）。
      let mut newRange: Option<i64> = None;
      // java FieldTriple fieldStart = this.getContainingField(start);
      let mut fieldStart: FieldTriple = self.getContainingField(start).unwrap();
      // java FieldTriple fieldEnd = this.getContainingField(end - 1);
      let mut fieldEnd: FieldTriple = self.getContainingField(end - 1).unwrap();
      // java if (fieldStart == null && fieldEnd != null)
      if false && true {
        // java if (end != fieldEnd.getEnd())
        if end != fieldEnd.getEnd() {
          // java newRange = RANGEFactory.getRANGEObject(start, fieldEnd.getEnd());
          newRange = Some(RANGEFactory::getRANGEObject(start, fieldEnd.getEnd()));
        }
        // java else
        else {
          // java newRange = RANGEFactory.getRANGEObject(Math.min(start, fieldEnd.getStartCP()), end);
          newRange = Some(RANGEFactory::getRANGEObject(Math::min(start, fieldEnd.getStartCP()), end));
        }
      }
      // java else if (fieldStart != null && fieldEnd == null)
      else if true && false {
        // java if (start != fieldStart.getStartCP())
        if start != fieldStart.getStartCP() {
          // java newRange = RANGEFactory.getRANGEObject(fieldStart.getStartCP(), end);
          newRange = Some(RANGEFactory::getRANGEObject(fieldStart.getStartCP(), end));
        }
      }
      // java else if (fieldStart != null && fieldEnd != null)
      else if true && true {
        // java if (fieldStart.start != fieldEnd.start)
        if fieldStart.start != fieldEnd.start {
          // java if (start != fieldStart.getStartCP() || end != fieldEnd.getEnd())
          if start != fieldStart.getStartCP() || end != fieldEnd.getEnd() {
            // java newRange = RANGEFactory.getRANGEObject(fieldStart.getStartCP(), fieldEnd.getEnd());
            newRange = Some(RANGEFactory::getRANGEObject(fieldStart.getStartCP(), fieldEnd.getEnd()));
          }
        }
        // java else
        else {
          // java long rg = RANGEFactory.getRANGE(start, end);
          let mut rg: i64 = RANGEFactory::getRANGE(start, end);
          // java if (fieldStart.getFieldType() >= KFieldType.FieldEmpty.getFlt() && fieldStart.getFieldType() <= KFieldType.FieldBibliography.getFlt() && (fieldStart.getFieldType() == KFieldType.FieldShape.getFlt() || start == fieldStart.getSepCP() + 1 && end == fieldStart.getEnd() - 1 || RANGEFactory.contains(rg, fieldStart.getStartCP()) || RANGEFactory.contains(rg, fieldStart.getEnd() - 1)))
          if fieldStart.getFieldType() >= KFieldType::FieldEmpty.getFlt() && fieldStart.getFieldType() <= KFieldType::FieldBibliography.getFlt() && fieldStart.getFieldType() == KFieldType::FieldShape.getFlt() || start == fieldStart.getSepCP() + 1 && end == fieldStart.getEnd() - 1 || RANGEFactory::contains(rg, fieldStart.getStartCP()) || RANGEFactory::contains(rg, fieldStart.getEnd() - 1) {
            // java newRange = RANGEFactory.getRANGEObject(fieldStart.getStartCP(), fieldStart.getEnd());
            newRange = Some(RANGEFactory::getRANGEObject(fieldStart.getStartCP(), fieldStart.getEnd()));
          }
        }
      }
      // java return newRange;
      return newRange;
    }

    // java public Long expandRangeToContainsWholeField(int start, int end)
    // rust: Java 返回 `Long`（可空）⇒ `Option<i64>`。
    // rust: Java 的 `start`/`end` 是**可变局部**（循环内被重写）⇒ 加 `mut`。
    pub fn expandRangeToContainsWholeField(&mut self, mut start: i32, mut end: i32) -> Option<i64> {
      // java if (start == end)
      if start == end {
        // java return null;
        return None;
      }
      // java else
      else {
        // java ArrayList<FieldTriple> list = this.getFieldTriples();
        let mut list: Vec<FieldTriple> = self.getFieldTriples();
        // java Assert.assertNotNull("list should not be null!", list);
        // rust: 门面 `assertNotNull__string_object<T>(&str, &Option<T>)`；`list`
        //   是 `Vec` 而非 `Option` ⇒ Java 的"非空断言"在此恒真，仅保留断言语义。
        assert!(!list.is_empty() || true, "list should not be null!");
        // java int size = list.size();
        let mut size: i32 = (list.len() as i32);
        // java Long newRange = null;
        // rust: Java `Long` 可空 ⇒ `Option<i64>`（模板落成裸 `i64` + `todo!()`）。
        let mut newRange: Option<i64> = None;
        let mut i: i32 = 0;
        // java for(int i = 0; i < size; ++i)
        while i < size {
          // java FieldTriple field = (FieldTriple)list.get(i);
          // rust: 同上 —— `Vec<FieldTriple>` 直接索引。
          let mut field: FieldTriple = list[i as usize].clone();
          // java Assert.assertNotNull("field should not be null!", field);
          // rust: 同上 —— `field` 为值类型，非空断言恒真。
          assert!(true, "field should not be null!");
          // java int fieldStart = field.getStartCP();
          let mut fieldStart: i32 = field.getStartCP();
          // java int fieldEnd = field.getEnd();
          let mut fieldEnd: i32 = field.getEnd();
          // java if (fieldStart > end)
          if fieldStart > end {
            // java break;
            break;
          }
          // java if (start <= fieldStart && end > fieldStart && end <= fieldEnd)
          if start <= fieldStart && end > fieldStart && end <= fieldEnd {
            // java newRange = RANGEFactory.getRANGEObject(start, fieldEnd);
            newRange = Some(RANGEFactory::getRANGEObject(start, fieldEnd));
            // java end = fieldEnd;
            end = fieldEnd;
          }
          // java if (start >= fieldStart && start < fieldEnd && end >= fieldEnd)
          if start >= fieldStart && start < fieldEnd && end >= fieldEnd {
            // java newRange = RANGEFactory.getRANGEObject(fieldStart, end);
            newRange = Some(RANGEFactory::getRANGEObject(fieldStart, end));
            // java start = fieldStart;
            start = fieldStart;
          }
        }
        // java return newRange;
        return newRange;
      }
    }

    // java public String toString()
    // rust: Java `toString()` 无 receiver 约束；Rust 侧因调用
    //   `Document.charAt(&mut self)` 而需 `&mut self`（同 `PLC` 的调试输出路径）。
    pub fn toString(&mut self) -> String {
      // java StringBuilder stringBuilder = new StringBuilder();
      // rust: 模板把 `java.lang.StringBuilder` 误生成为 `String`（`String` 无 `append`，
      //   共 10 处 E0599），且 ctor 落为 `todo!("no align strategy matched")`。
      //   仓库已有真实 `stubs::java::lang::StringBuilder`（`append__string` / `append` /
      //   `toString`）⇒ 按 Java 门面改用真实类型。
      let mut stringBuilder: StringBuilder = StringBuilder::new();
      // java stringBuilder.append(super.toString());
      stringBuilder.append__string(&self.super_.toString());
      // java try {
      // rust: Java 的 `catch (Exception var5) { return super.toString(); }` 被模板误生成为
      //   **try 体中部**的裸 `todo!()`：既破坏编译，又令已累积内容被丢弃，并使
      //   `return super.toString()` 沦为不可达死码 ⇒ 按 Java 结构恢复为 catch 分支。
      let __try_result: Option<String> = (|| -> Option<String> {
        // java stringBuilder.append("\n\t{\n");
        stringBuilder.append__string("\n\t{\n");
        // java PLC.Iterator iter = this.createIterator();
        let mut iter: crate::cn::wps::moffice::writer::data::PLC::Iterator = self.createIterator();
        // java while(iter.hasNext())
        while iter.hasNext() {
          // java iter.next();
          iter.next();
          // java int cp = iter.getStart();
          let mut cp: i32 = iter.getStart();
          // java stringBuilder.append("\t");
          stringBuilder.append__string("\t");
          // java stringBuilder.append(cp);
          stringBuilder.append__string(&cp.to_string());
          // java char cha = this.getDocument().charAt(cp);
          // rust: `PLC.getDocument() -> Option<&dyn Document>`；Java 未判空 ⇒ 解包。
          // rust: `charAt` 取 `&mut self` ⇒ 用可变访问器（`PLC::getDocument_mut`）。
          let mut cha: char = self.super_.getDocument_mut().expect("getDocument").charAt(cp);
          // java stringBuilder.append(" : ");
          stringBuilder.append__string(" : ");
          // java stringBuilder.append(cha);
          stringBuilder.append(cha);
          // java stringBuilder.append("[0x");
          stringBuilder.append__string("[0x");
          // java stringBuilder.append(Integer.toHexString(cha));
          // java stringBuilder.append(Integer.toHexString(cha));
          stringBuilder.append__string(&format!("{:x}", cha as u32));
          // java stringBuilder.append("]\n");
          stringBuilder.append__string("]\n");
        }
        // java stringBuilder.append("\t}");
        stringBuilder.append__string("\t}");
        Some(stringBuilder.toString())
      })();
      // java catch (Exception var5)
      let __result = match __try_result {
        Some(v) => v,
        // java return super.toString();
        None => return self.super_.toString(),
      };
      // java return stringBuilder.toString();
      return __result;
    }

    // java public void updateFieldLevel(int start, int end, boolean fIncrease)
    pub fn updateFieldLevel(&mut self, start: i32, end: i32, fIncrease: bool) {
      // java ArrayList<FieldTriple> fieldTriples = this.getFieldTriples(start, end);
      // rust: 完全拥有该本地列表 —— Java 里 `fieldTriples` 是独立 ArrayList，
      //   增删改都不影响 `this`；这里显式 clone 出来，避免与后续
      //   `self.super_.getTransaction_mut()` 的可变借用冲突。
      let mut fieldTriples: Vec<FieldTriple> = self.getFieldTriples__int_int(start, end);
      // java if (fieldTriples.size() != 0)
      if (fieldTriples.len() as i32) != 0 {
        // java for(FieldTriple fieldTriple : fieldTriples)
        for fieldTriple in fieldTriples.iter_mut() {
          // java if (fIncrease)
          if fIncrease {
            // java fieldTriple.start.level++;
            { let _tmp = fieldTriple.start.level; fieldTriple.start.level += 1; _tmp };
            // java if (null != fieldTriple.separator)
            if let Some(fieldTriple_separator) = fieldTriple.separator.as_mut() {
              // java fieldTriple.separator.level++;
              { let _tmp = fieldTriple_separator.level; fieldTriple_separator.level += 1; _tmp };
            }
            // java fieldTriple.end.level++;
            { let _tmp = fieldTriple.end.level; fieldTriple.end.level += 1; _tmp };
          }
          // java else
          else {
            // java fieldTriple.start.level--;
            { let _tmp = fieldTriple.start.level; fieldTriple.start.level -= 1; _tmp };
            // java if (null != fieldTriple.separator)
            if let Some(fieldTriple_separator) = fieldTriple.separator.as_mut() {
              // java fieldTriple.separator.level--;
              { let _tmp = fieldTriple_separator.level; fieldTriple_separator.level -= 1; _tmp };
            }
            // java fieldTriple.end.level--;
            { let _tmp = fieldTriple.end.level; fieldTriple.end.level -= 1; _tmp };
          }
        }
        // java Transaction t = this.getTransaction();
        // rust: `PLC.getTransaction() -> Option<&Transaction>`；Java 有 `t != null` 判定
        //   （模板压成 `if true`）⇒ 恢复为空判定。
        // rust: `submitCommand` 需要 `&mut Transaction`（`Transaction.rs:243`）
        //   ⇒ 取可变借用（Java `getTransaction()` 返回对象引用，天然可写）。
        let t: Option<&mut Transaction> = self.super_.getTransaction_mut();
        // java if (t != null && t.getState() == 1)
        if t.is_some() && t.as_ref().unwrap().getState() == 1 {
          let t = t.unwrap();
          // java t.submitCommand(new CmdUpdateFieldLevel(start, end, fIncrease));
          // rust: 门面 `submitCommand(Option<Box<dyn ActionCommand>>)`
          //   （`Transaction.rs:243`）。
          t.submitCommand(Some(Box::new(CmdUpdateFieldLevel::new(start, end, fIncrease))));
        }
      }
    }

}

impl std::ops::Deref for PLCField {
    type Target = PLC_Char;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for PLCField {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java public static class FieldNode extends PLC.Node
impl PartialEq for FieldNode {
    // rust: 见类型上方说明 —— 等价于 Java 的引用相等（同一 `PLC.Node` 指针）。
    fn eq(&self, other: &Self) -> bool {
        self.super_.as_node_ptr() == other.super_.as_node_ptr()
    }
}

impl Clone for FieldNode {
    fn clone(&self) -> Self {
        Self {
            super_: self.super_,
            mFieldChar: self.mFieldChar,
            mData: self.mData.clone(),
            level: self.level,
            // java `(HashMap)this.mUser.clone()` 为浅拷贝（值共享）；
            //   `Box<dyn Object>` 在 Rust 侧不可共享，且该路径无管线调用点
            //   ⇒ 以空表承载，行为差异记录于本注释与 DEFERRED 说明。
            mUser: self.mUser.as_ref().map(|_| HashMap::new()),
        }
    }
}

impl FieldNode {
    // Java FieldNode is the same object as its PLC.Node base. Reconstruct a
    // Rust view from the shared node state instead of casting a HeapNode to
    // the unrelated FieldNode struct layout.
    pub fn from_node(node: Node) -> Self {
        let (mFieldChar, mData, level) = node.field_state();
        Self { super_: node, mFieldChar, mData, level, mUser: None }
    }

    fn sync_node(&self) {
        self.super_.set_field_state(self.mFieldChar, self.mData, self.level);
    }

    // java public  FieldNode()
    pub fn FieldNode() -> Self {
      // java this.mData = KFieldType.FieldEmpty.getFlt();
      let node = Self { super_: Node::new(), mFieldChar: Default::default(), mData: KFieldType::FieldEmpty.getFlt(), level: Default::default(), mUser: Default::default() };
      node.sync_node();
      node
    }

    // rust-only: Rust 门面别名 —— 4 处调用点写作 `FieldNode::new()`
    //   （对应 Java `new FieldNode()`，见 `PLCField.java:202/204/206/215`），
    //   与仓库既有 `Type::new()` 约定一致；语义与 `FieldNode()` 逐字相同。
    pub fn new() -> Self {
      Self::FieldNode()
    }

    // java FieldNode extends PLC.Node —— `getPrev()` / `getNext()` 由基类继承，
    //   Java 侧返回 `PLC.Node`，调用点再强转 `(FieldNode)`。
    // rust: 以 `Option<FieldNode>` 承载"强转成功/为 null"，供
    //   `getStartNode` / `getSeparatorNode` / `getEndNode` 的
    //   `node instanceof FieldNode` 循环条件使用（Java `null` → `None`）。
    pub fn getPrev(&self) -> Option<FieldNode> {
      let p: Node = self.super_.getPrev();
      if p.isEnd() || p.as_node_ptr().is_null() { None } else { Some(Self::from_node(p)) }
    }

    pub fn getNext(&self) -> Option<FieldNode> {
      let n: Node = self.super_.getNext();
      if n.isEnd() || n.as_node_ptr().is_null() { None } else { Some(Self::from_node(n)) }
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java super.dispose();
      self.super_.dispose();
      // java this.mUser.clear();
      // rust: `Option<HashMap>` ⇒ 判空后清空。
      if let Some(m) = self.mUser.as_mut() {
        m.clear();
      }
      // java this.mUser = null;
      self.mUser = None;
    }

    // java public FieldNode clone()
    pub fn clone(&self) -> FieldNode {
      // java FieldNode node = null;
      let mut node: FieldNode = Default::default();
      // java try { node = (FieldNode)super.clone(); }
      // java catch (CloneNotSupportedException var3) { }
      // rust: Java 的 catch 体为**空**（不做任何处理，`node` 保持 null）。
      //   模板把空 catch 落成裸 `todo!()` ⇒ 整条 clone 路径 panic。
      //   超类克隆恒成功 ⇒ 直接赋值，catch 分支无对应 Rust 语句。
      node = Self::from_node(self.super_);
      // java if (this.mUser != null)
      if let Some(mUser) = self.mUser.as_ref() {
        // java node.mUser = (HashMap)this.mUser.clone();
        // rust: 见结构体上方手写 `Clone` 的说明 —— `Box<dyn Object>` 非 `Clone`，
        //   Java 的浅拷贝语义在 Rust 侧以"不复制值、保留空表"承载。
        let _ = mUser;
        node.mUser = Some(HashMap::new());
      }
      // java return node;
      return node;
    }

    // java public Boolean getHyperlinkVisited()
    pub fn getHyperlinkVisited(&mut self) -> Option<bool> {
      // java this.mUser = null != this.mUser ? this.mUser : new HashMap();
      // rust: `null != this.mUser ? this.mUser : new HashMap()` —— 空则建新表。
      if self.mUser.is_none() {
        self.mUser = Some(HashMap::new());
      }
      // java Object value = this.mUser.get("HyperlinkVisited");
      // rust: 表值即 `Box<dyn Object>`；Java 的 `(Boolean)value` 强转在 Rust 侧
      //   以 `downcast_ref::<bool>` 承载（表内只存 `Boolean`，见
      //   `setHyperlinkVisited`）。
      let value: Option<bool> = self
        .mUser
        .as_ref()
        .and_then(|m| m.get("HyperlinkVisited"))
        .and_then(|v| v.as_any().downcast_ref::<bool>().copied());
      // java return null != value ? (Boolean)value : null;
      return value;
    }

    // java public void setHyperlinkVisited(boolean value)
    pub fn setHyperlinkVisited(&mut self, value: bool) {
      // java this.mUser = null != this.mUser ? this.mUser : new HashMap();
      // rust: `Option<HashMap>` —— 空则建新表（模板把 `? :` 落成类型不符的表达式）。
      if self.mUser.is_none() {
        self.mUser = Some(HashMap::new());
      }
      // java this.mUser.put("HyperlinkVisited", value);
      // rust: `Map.put` → `HashMap::insert`；`Object` 值以 `Box<dyn Object>` 承载。
      if let Some(m) = self.mUser.as_mut() {
        m.insert(
          "HyperlinkVisited".to_string(),
          Box::new(value) as Box<dyn Object>,
        );
      }
    }

    // java public int getLevel()
    pub fn getLevel(&self) -> i32 {
      // java return this.level;
      return self.level;
    }

    // java public boolean isFieldBegin()
    pub fn isFieldBegin(&self) -> bool {
      // java return this.mFieldChar == 19;
      return self.mFieldChar == 19;
    }

    // java public boolean isFieldSeperator()
    pub fn isFieldSeperator(&self) -> bool {
      // java return this.mFieldChar == 20;
      return self.mFieldChar == 20;
    }

    // java public boolean isFieldEnd()
    pub fn isFieldEnd(&self) -> bool {
      // java return this.mFieldChar == 21;
      return self.mFieldChar == 21;
    }

    // java public int getFieldChar()
    pub fn getFieldChar(&self) -> i32 {
      // java return this.mFieldChar;
      return self.mFieldChar;
    }

    // java public void setFieldChar(int nFieldChar)
    pub fn setFieldChar(&mut self, nFieldChar: i32) {
      // java this.mFieldChar = nFieldChar;
      self.mFieldChar = nFieldChar;
      // java if (this.getPrev().isEnd())
      // rust: `FieldNode::getPrev() -> Option<FieldNode>`（承载 `null`）⇒ 解包。
      if self.super_.getPrev().isEnd() {
        // java this.level = 1;
        self.level = 1;
      }
      // java else
      else {
        // java FieldNode prev = (FieldNode)this.getPrev();
        // rust: `getPrev() -> Option<FieldNode>`；Java 此处未判空（直接强转）
        //   ⇒ `expect` 保持失败即 panic 的等价语义。
        let mut prev: FieldNode = self.getPrev().expect("getPrev");
        // java switch (nFieldChar)
        match nFieldChar {
          // java case 19:
          19 => {
            // java if (prev.isFieldEnd())
            if prev.isFieldEnd() {
              // java this.level = prev.level;
              self.level = prev.level;
            }
            // java else
            else {
              // java this.level = prev.level + 1;
              self.level = prev.level + 1;
            }
          }
          // java case 20:
          20 => {
            // java if (prev.isFieldEnd())
            if prev.isFieldEnd() {
              // java this.level = prev.level - 1;
              self.level = prev.level - 1;
            }
            // java else
            else {
              // java this.level = prev.level;
              self.level = prev.level;
            }
          }
          // java case 21:
          21 => {
            // java if (prev.isFieldEnd())
            if prev.isFieldEnd() {
              // java this.level = prev.level - 1;
              self.level = prev.level - 1;
            }
            // java else
            else {
              // java this.level = prev.level;
              self.level = prev.level;
            }
          }
          // java default:
          _ => {
            // java Assert.fail("It should not be herer!");
            // rust: 门面为 `Assert::fail_msg(&str)`（`Assert.rs:179`）。
            Assert::fail_msg("It should not be herer!");
          }
        }
      }
      self.sync_node();
    }

    // java public int getFieldType()
    pub fn getFieldType(&self) -> i32 {
      // java return this.mData;
      return self.mData;
    }

    // java @Deprecated public void setFieldType(int nType)
    pub fn setFieldType(&mut self, nType: i32) {
      // java this.mData = nType;
      self.mData = nType;
      self.sync_node();
    }

    // java public int getFieldFlags()
    pub fn getFieldFlags(&self) -> i32 {
      // java return this.mData;
      return self.mData;
    }

    // java @Deprecated public void setFieldFlags(int nFlags)
    pub fn setFieldFlags(&mut self, nFlags: i32) {
      // java this.mData = nFlags;
      self.mData = nFlags;
      self.sync_node();
    }

    // java public Long getCodeRange()
    pub fn getCodeRange(&self) -> Option<i64> {
      // java FieldNode pStart = this.getStartNode();
      // rust: Java 的 `getStartNode()` / `getSeparatorNode()` / `getEndNode()`
      //   均可返回 `null`；模板把它们落为**非空 `FieldNode`** 并把 `pSep == null`
      //   判定压成 `if false`（丢弃了 Java 的**回退到 endNode** 逻辑）。
      //   按 Java 语义恢复为 `Option`。
      let pStart: Option<FieldNode> = self.getStartNode();
      // java FieldNode pSep = this.getSeparatorNode();
      let mut pSep: Option<FieldNode> = self.getSeparatorNode();
      // java if (pSep == null)
      if pSep.is_none() {
        // java pSep = this.getEndNode();
        pSep = self.getEndNode();
      }
      // java return pSep != null && pStart != null ? RANGEFactory.getRANGE(pStart.getCP() + 1, pSep.getCP()) : null;
      return match (pStart, pSep) {
        (Some(a), Some(b)) => Some(RANGEFactory::getRANGE(a.getCP() + 1, b.getCP())),
        _ => None,
      };
    }

    // java public FieldNode getStartNode()
    pub fn getStartNode(&self) -> Option<FieldNode> {
      // java PLC.Node node = this;
      // rust: Java `PLC.Node node = this;` —— 游标以 `Option<FieldNode>` 承载
      //   （本遍历**只读**，按下标沿 `getPrev`/`getNext` 前进；用 owned 值避免
      //   `Box::leak` 泄漏，语义与 Java 引用游标在这条只读路径上等价）。
      //   模板落为 `todo!("no align strategy matched")`（3 处，见 getStartNode/
      //   getSeparatorNode/getEndNode），使三条管线可达方法整体不可用。
      let mut node: Option<FieldNode> = Some(self.clone());
      // java int startCount = 0;
      let mut startCount: i32 = 0;
      // java int endCount = 0;
      let mut endCount: i32 = 0;
      // java if (!this.isFieldEnd())
      if !self.isFieldEnd() {
        // java ++endCount;
        { let _tmp = endCount; endCount += 1; _tmp };
      }
      // java for(; node instanceof FieldNode; node = node.getPrev())
      // rust: Java `for` 的第三段（update）是 `node = node.getPrev()`；
      //   模板把它并入了条件位（`while (...)`），导致**不推进 ⇒ 死循环**。
      //   按 Java 结构改为循环体尾部推进。
      while let Some(t) = node.clone() {
        // java FieldNode tmp = (FieldNode)node;
        let mut tmp: FieldNode = t;
        // java if (tmp.isFieldBegin())
        if tmp.isFieldBegin() {
          // java ++startCount;
          { let _tmp = startCount; startCount += 1; _tmp };
          // java if (startCount == endCount)
          if startCount == endCount {
            // java return tmp;
            return Some(tmp);
          }
        }
        // java else if (tmp.isFieldEnd())
        else if tmp.isFieldEnd() {
          // java ++endCount;
          { let _tmp = endCount; endCount += 1; _tmp };
        }
        // java for-update: node = node.getPrev();（`instanceof FieldNode` 由 Option 承载）
        node = tmp.getPrev();
      }
      // java return null;
      return None;
    }

    // java public FieldNode getSeparatorNode()
    pub fn getSeparatorNode(&self) -> Option<FieldNode> {
      // java if (this.isFieldSeperator())
      if self.isFieldSeperator() {
        // java return this;
        return Some(self.clone());
      }
      // java else
      else {
        // java PLC.Node node = this;
        // rust: Java `PLC.Node node = this;` —— 游标以 `Option<FieldNode>` 承载
      //   （本遍历**只读**，按下标沿 `getPrev`/`getNext` 前进；用 owned 值避免
      //   `Box::leak` 泄漏，语义与 Java 引用游标在这条只读路径上等价）。
      //   模板落为 `todo!("no align strategy matched")`（3 处，见 getStartNode/
      //   getSeparatorNode/getEndNode），使三条管线可达方法整体不可用。
      let mut node: Option<FieldNode> = Some(self.clone());
        // java int startCount = 0;
        let mut startCount: i32 = 0;
        // java int endCount = 0;
        let mut endCount: i32 = 0;
        let mut findNext: bool = self.isFieldBegin();
        // java for(boolean findNext = this.isFieldBegin(); node instanceof FieldNode node = findNext ? node.getNext() : node.getPrev())
        while let Some(t) = node.clone() {
          // java FieldNode tmp = (FieldNode)node;
          let mut tmp: FieldNode = t;
          // java if (tmp.isFieldBegin())
          if tmp.isFieldBegin() {
            // java ++startCount;
            { let _tmp = startCount; startCount += 1; _tmp };
            // java if (endCount == startCount && !findNext)
            if endCount == startCount && !findNext {
              // java return null;
              return None;
            }
          }
          // java else if (tmp.isFieldEnd())
          else if tmp.isFieldEnd() {
            // java ++endCount;
            { let _tmp = endCount; endCount += 1; _tmp };
            // java if (endCount == startCount && findNext)
            if endCount == startCount && findNext {
              // java return null;
              return None;
            }
          }
          // java else if (tmp.isFieldSeperator())
          else if tmp.isFieldSeperator() {
            // java if (endCount == startCount + 1 && !findNext)
            if endCount == startCount + 1 && !findNext {
              // java return tmp;
              return Some(tmp);
            }
            // java if (startCount == endCount + 1 && findNext)
            if startCount == endCount + 1 && findNext {
              // java return tmp;
              return Some(tmp);
            }
          }
          // java for-update: node = findNext ? node.getNext() : node.getPrev();
          node = if findNext { tmp.getNext() } else { tmp.getPrev() };
        }
        // java return null;
        return None;
      }
    }

    // java public FieldNode getEndNode()
    pub fn getEndNode(&self) -> Option<FieldNode> {
      // java PLC.Node node = this;
      // rust: Java `PLC.Node node = this;` —— 游标以 `Option<FieldNode>` 承载
      //   （本遍历**只读**，按下标沿 `getPrev`/`getNext` 前进；用 owned 值避免
      //   `Box::leak` 泄漏，语义与 Java 引用游标在这条只读路径上等价）。
      //   模板落为 `todo!("no align strategy matched")`（3 处，见 getStartNode/
      //   getSeparatorNode/getEndNode），使三条管线可达方法整体不可用。
      let mut node: Option<FieldNode> = Some(self.clone());
      // java int startCount = 0;
      let mut startCount: i32 = 0;
      // java int endCount = 0;
      let mut endCount: i32 = 0;
      // java if (!this.isFieldBegin())
      if !self.isFieldBegin() {
        // java ++startCount;
        { let _tmp = startCount; startCount += 1; _tmp };
      }
      // java for(; node instanceof FieldNode; node = node.getNext())
      while let Some(t) = node.clone() {
        // java FieldNode tmp = (FieldNode)node;
        let mut tmp: FieldNode = t;
        // java if (tmp.isFieldBegin())
        if tmp.isFieldBegin() {
          // java ++startCount;
          { let _tmp = startCount; startCount += 1; _tmp };
        }
        // java else if (tmp.isFieldEnd())
        else if tmp.isFieldEnd() {
          // java ++endCount;
          { let _tmp = endCount; endCount += 1; _tmp };
          // java if (startCount == endCount)
          if startCount == endCount {
            // java return tmp;
            return Some(tmp);
          }
        }
        // java for-update: node = node.getNext();
        node = tmp.getNext();
      }
      // java return null;
      return None;
    }

    // java public void correctLevel(int level)
    pub fn correctLevel(&mut self, level: i32) {
      // java this.level = level;
      self.level = level;
    }

}

impl std::ops::Deref for FieldNode {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for FieldNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java public static class FieldTriple
impl FieldTriple {
    // java public  FieldTriple()
    pub fn new() -> Self {
      Self { start: Default::default(), separator: Default::default(), end: Default::default() }
    }

    // java public  FieldTriple(FieldNode start, FieldNode separator, FieldNode end)
    // rust: 忠实建模 —— `FieldTriple.separator` 可空（`FieldTriple.java:590`
    //   有 `null != separator` 判定），而 `start` / `end` 恒非空
    //   （`this.start.mData` / `this.end.getCP()` 直接解引用）
    //   ⇒ 形参取 `FieldNode` / `Option<FieldNode>` / `FieldNode` 并转移所有权。
    //   模板落为三个 `&FieldNode`（既丢可空性又借用），导致 3 处 E0308。
    pub fn new__fieldnode_optfieldnode_fieldnode(start: FieldNode, separator: Option<FieldNode>, end: FieldNode) -> Self {
      // java this.start = start;
      Self { start: start, separator: separator, end: end }
      // java this.separator = separator;
      // java this.end = end;
    }

    // rust-only: Rust 门面别名 —— 调用点写作 `FieldTriple::new(a, b, c)`
    //   （对应 Java `new FieldTriple(start, separator, end)`）。
    pub fn new_field_triple(start: FieldNode, separator: Option<FieldNode>, end: FieldNode) -> Self {
      Self::new__fieldnode_optfieldnode_fieldnode(start, separator, end)
    }

    // java public int getFieldType()
    pub fn getFieldType(&self) -> i32 {
      // java return this.start.mData;
      return self.start.mData;
    }

    // java public boolean isLocked()
    pub fn isLocked(&self) -> bool {
      // java int flags = this.end.getFieldFlags();
      let mut flags: i32 = self.end.getFieldFlags();
      // java return (flags & 16) != 0;
      return flags & 16 != 0;
    }

    // java public boolean hasSep()
    pub fn hasSep(&self) -> bool {
      // java int flags = this.end.getFieldFlags();
      let mut flags: i32 = self.end.getFieldFlags();
      // java return (flags & 128) != 0;
      return flags & 128 != 0;
    }

    // java public boolean isZombieEmbed()
    pub fn isZombieEmbed(&self) -> bool {
      // java int flags = this.end.getFieldFlags();
      let mut flags: i32 = self.end.getFieldFlags();
      // java return (flags & 2) != 0;
      return flags & 2 != 0;
    }

    // java public long getCodeRange()
    pub fn getCodeRange(&self) -> i64 {
      // java FieldNode pSep = this.separator;
      // rust: `&self` 不能移动 ⇒ clone；Java 的 `pSep == null` 分支按
      //   `Option` 还原（模板压成 `if false`）。
      let mut pSep: FieldNode = match self.separator.as_ref() {
        Some(sep) => sep.clone(),
        // java if (pSep == null) { pSep = this.end; }
        None => self.end.clone(),
      };
      // java return RANGEFactory.getRANGE(this.start.getCP() + 1, pSep.getCP());
      return RANGEFactory::getRANGE(self.start.getCP() + 1, pSep.getCP());
    }

    // java public Long getResultRange()
    pub fn getResultRange(&self) -> Option<i64> {
      // java return this.separator == null ? null : RANGEFactory.getRANGEObject(this.separator.getCP() + 1, this.end.getCP());
      return match &self.separator {
        None => None,
        Some(sep) => Some(RANGEFactory::getRANGEObject(sep.getCP() + 1, self.end.getCP())),
      };
    }

    // java public boolean hasVisiableContent()
    pub fn hasVisiableContent(&self) -> bool {
      // java if (this.separator != null)
      if let Some(separator) = self.separator.as_ref() {
        // java return true;
        return true;
      }
      // java else
      else {
        // java KFieldType kFieldType = KFieldType.getKFieldType(this.getFieldType());
        // rust: `getKFieldType(int) -> Option<KFieldType>`（可空）；模板落为非空
        //   并把 Java 的 `null == kFieldType` 判定压成 `if false` ⇒ 恢复语义。
        let kFieldType: Option<KFieldType> = KFieldType::getKFieldType(self.getFieldType());
        // java if (null == kFieldType)
        if kFieldType.is_none() {
          // java return false;
          return false;
        }
        // java else
        else {
          let kFieldType = kFieldType.clone().unwrap();
          // java switch (kFieldType)
          match kFieldType {
            // java case FieldAuthor: case FieldComments: case FieldLastSavedBy: case FieldSubject: case FieldTitle: case FieldKeyWord: case FieldUserAddress: case FieldUserInitials: case FieldUserName: case FieldFileName: case FieldNumWords: case FieldNumChars: case FieldFileSize: case FieldExpression: case FieldTime: case FieldDate: case FieldEditTime: case FieldPrintDate: case FieldSaveDate: case FieldCreateDate: case FieldTemplate: case FieldInfo: case FieldRevisionNum: case FieldAutoNum: case FieldAutoNumLegal: case FieldPage: case FieldSection: case FieldSectionPages: case FieldNumPages:
            KFieldType::FieldAuthor | KFieldType::FieldComments | KFieldType::FieldLastSavedBy | KFieldType::FieldSubject | KFieldType::FieldTitle | KFieldType::FieldKeyWord | KFieldType::FieldUserAddress | KFieldType::FieldUserInitials | KFieldType::FieldUserName | KFieldType::FieldFileName | KFieldType::FieldNumWords | KFieldType::FieldNumChars | KFieldType::FieldFileSize | KFieldType::FieldExpression | KFieldType::FieldTime | KFieldType::FieldDate | KFieldType::FieldEditTime | KFieldType::FieldPrintDate | KFieldType::FieldSaveDate | KFieldType::FieldCreateDate | KFieldType::FieldTemplate | KFieldType::FieldInfo | KFieldType::FieldRevisionNum | KFieldType::FieldAutoNum | KFieldType::FieldAutoNumLegal | KFieldType::FieldPage | KFieldType::FieldSection | KFieldType::FieldSectionPages | KFieldType::FieldNumPages => {
              // java return true;
              return true;
            }
            // java default:
            _ => {
              // java return false;
              return false;
            }
          }
        }
      }
    }

    // java public boolean isDateOrTime()
    pub fn isDateOrTime(&self) -> bool {
      // java KFieldType kFieldType = KFieldType.getKFieldType(this.getFieldType());
      // rust: `getKFieldType(int) -> Option<KFieldType>`（可空）⇒ 恢复 Java 空判定。
      let kFieldType: Option<KFieldType> = KFieldType::getKFieldType(self.getFieldType());
      // java if (null == kFieldType)
      if kFieldType.is_none() {
        // java return false;
        return false;
      }
      // java else
      else {
        let kFieldType = kFieldType.clone().unwrap();
        // java switch (kFieldType)
        match kFieldType {
          // java case FieldTime: case FieldDate:
          // rust: 显式限定枚举路径 —— `core` 侧同名 glob 导入会遮蔽变体名，
          //   导致 E0408（见本文件 1329 行同样处理）。
          KFieldType::FieldTime | KFieldType::FieldDate => {
            // java return true;
            return true;
          }
          // java default:
          _ => {
            // java return false;
            return false;
          }
        }
      }
    }

    // java public int getEnd()
    pub fn getEnd(&self) -> i32 {
      // java return this.end.getCP() + 1;
      return self.end.getCP() + 1;
    }

    // java public int getEndCP()
    pub fn getEndCP(&self) -> i32 {
      // java return this.end.getCP();
      return self.end.getCP();
    }

    // java public int getStartCP()
    pub fn getStartCP(&self) -> i32 {
      // java return this.start.getCP();
      return self.start.getCP();
    }

    // java public int getSepCP()
    pub fn getSepCP(&self) -> i32 {
      // java return this.separator != null ? this.separator.getCP() : -1;
      return match &self.separator {
        Some(sep) => sep.getCP(),
        None => -1,
      };
    }

}

// java public class CmdUpdateFieldLevel extends Command
impl CmdUpdateFieldLevel {
    // java public  CmdUpdateFieldLevel(int start, int end, boolean fIncrease)
    pub fn CmdUpdateFieldLevel(start: i32, end: i32, fIncrease: bool) -> Self {
      // java this.start = start;
      Self { super_: Command::new(), start: start, end: end, fIncrease: fIncrease, outer: None }
      // java this.end = end;
      // java this.fIncrease = fIncrease;
    }

    // rust-only: Rust 门面别名 —— 调用点写作 `CmdUpdateFieldLevel::new(...)`
    //   （对应 Java `new CmdUpdateFieldLevel(start, end, fIncrease)`）。
    pub fn new(start: i32, end: i32, fIncrease: bool) -> Self {
      Self::CmdUpdateFieldLevel(start, end, fIncrease)
    }

    // java public void execute()
    pub fn execute(&self) {
      // java PLCField.this.updateFieldLevel(this.start, this.end, this.fIncrease);
      // rust: 经显式回引调用外部实例方法（模板误当作自身方法 ⇒ E0599）。
      let outer = self.outer.expect("CmdUpdateFieldLevel.outer");
      unsafe { (*outer).updateFieldLevel(self.start, self.end, self.fIncrease) };
    }

    // java public void unexecute()
    pub fn unexecute(&self) {
      // java PLCField.this.updateFieldLevel(this.start, this.end, !this.fIncrease);
      let outer = self.outer.expect("CmdUpdateFieldLevel.outer");
      unsafe { (*outer).updateFieldLevel(self.start, self.end, !self.fIncrease) };
    }

}

impl std::ops::Deref for CmdUpdateFieldLevel {
    type Target = Command;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for CmdUpdateFieldLevel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
