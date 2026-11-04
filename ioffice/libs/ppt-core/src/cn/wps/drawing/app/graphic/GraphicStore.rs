#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.drawing.app.chart.KmoOleChart
// rust: chart 域未移植（`drawing/app/chart` 仍是 form-E 桩 / `#[cfg(feature="full")]` 门禁），
//   `KmoOleChart` 类型在本仓**不存在** ⇒ `clear()` 的 `instanceof KmoOleChart` 分支按 Java
//   语义对其它 GraphicObject 恒为空体（见该函数注）。KmoTable（`drawing/app/table`，
//   `#[cfg(any())]` 关）是唯一实现 GraphicObject 的产品类，同样未参与默认构建。
// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import cn.wps.drawing.app.graphic.GraphicObject（同包接口）

use super::GraphicObject::GraphicObject;

// Java: public class GraphicStore
// rust: ★W6c-10 —— 本文件由 form-E 半桩重写为**真身**（Java 54 行逐条对应）。
//   原实现内联了 `pub struct GraphicObject;`（单元桩，遮蔽同目录真 trait `GraphicObject.rs`）
//   且 `copyOf` 为空体 ⇒ 形态 F（同名空壳遮蔽真实实现）。
//   owner: cn.wps.drawing.app.graphic.GraphicStore    wave: M-PPT-KMOSLIDE-W6c-10
pub struct GraphicStore {
    // Java field: List<GraphicObject> graphicLst
    // rust: Java 的 `List<GraphicObject>` 存放**引用**；Rust 以 `Box<dyn GraphicObject>`
    //   承载（独占所有权 + trait 对象），`query` 按 Java 语义返回**借用**（同一对象）。
    pub(crate) graphicLst: Vec<Box<dyn GraphicObject>>,
}

// Java: public class GraphicStore
impl GraphicStore {
    pub fn new() -> Self {
        GraphicStore {
            graphicLst: Vec::new(),
        }
    }

    // Java: int add(GraphicObject g)
    pub fn add(&mut self, g: Box<dyn GraphicObject>) -> i32 {
        // Java: this.graphicLst.add(g);
        self.graphicLst.push(g);
        // Java: return this.graphicLst.size() - 1;
        (self.graphicLst.len() - 1) as i32
    }

    // Java: GraphicObject query(int id)
    pub fn query(&self, id: i32) -> Option<&dyn GraphicObject> {
        // Java: return id >= 0 && id < this.graphicLst.size() ? (GraphicObject)this.graphicLst.get(id) : null;
        if id >= 0 && (id as usize) < self.graphicLst.len() {
            Some(self.graphicLst[id as usize].as_ref())
        } else {
            None
        }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: for(int i = 0; i < this.graphicLst.size(); ++i)
        for object in &self.graphicLst {
            // Java: GraphicObject object = (GraphicObject)this.graphicLst.get(i);
            // Java: if (object instanceof KmoOleChart)
            // Java:    ((KmoOleChart)object).clear();
            // rust: ★W6c-10 —— `KmoOleChart` 类型在本仓不存在（chart 域整体门禁/deferred，
            //   见文件头注）⇒ `instanceof` 分支无任何对象可命中，按 Java 语义对其它
            //   GraphicObject（非 KmoOleChart）本就没有动作 ⇒ 循环体为空。
            let _ = object;
        }
        // Java: this.graphicLst.clear();
        self.graphicLst.clear();
    }

    // Java: void copyOf(GraphicStore graphicStore)
    pub fn copyOf(&mut self, graphicStore: &GraphicStore) {
        // Java: this.graphicLst.addAll(graphicStore.graphicLst);
        // rust: ★登记 —— Java `addAll` 共享**同一批引用**；`Box<dyn GraphicObject>` 是独占
        //   所有权，无法共享，只能逐元素 `copy()`（深拷贝）。`GraphicObject::copy()` 正是
        //   Java 接口的 `copy()`（`GraphicObject.java:6`），语义差异在此**显式登记**。
        //   owner: cn.wps.drawing.app.graphic.GraphicStore    wave: M-PPT-KMOSLIDE-W6c-10
        self.graphicLst
            .extend(graphicStore.graphicLst.iter().map(|g| g.copy()));
    }
}