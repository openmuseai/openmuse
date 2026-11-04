#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unused_mut, unused_assignments)]

// Java: package cn.wps.show.app.text
// Java: import cn.wps.drawing.app.graphic.GraphicObject
// rust: no-stmt —— 仅用于 `shape.graphic()` 的返回值承接（`Option<GraphicObject>`，类型由
//   `KmoShowShape.graphic()` 签名给出，无需显式命名）。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3
// Java: import cn.wps.drawing.app.table.KmoTable
// Java: import cn.wps.drawing.app.table.KmoTableCell
// rust: no-stmt —— 默认构建下 `KmoTable` 只有空宿主、`KmoTableCell` 不可达 ⇒ 三处表遍历
//   （`anchor()`/`__isAnchorCenter()`/`__textDirection()`）与 `save()` 的表写回整体 DEFERRED。
//   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
// Java: import cn.wps.drawing.app.text.KmoTextBody
// rust: no-stmt —— 两个以 `KmoTextBody` 为形参的构造器整体 DEFERRED（见构造器注记）。
//   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
// Java: import cn.wps.drawing.app.util.KmoRange
// Java: import cn.wps.drawing.data.table.TableCell
// rust: no-stmt —— `KmoRange`/`TableCell` 默认构建下不可达（见 `KmoTable` 注记）。
//   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
// Java: import cn.wps.show.app.shape.KmoShowShape
use crate::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
// Java: import java.util.concurrent.locks.ReentrantReadWriteLock
use std::sync::{Arc, RwLock};
// Java: （父类）cn.wps.show.app.text.KmoTextOperator
use super::KmoTextOperator::KmoTextOperator;

// Java: public class KmoCellTextOperator extends KmoTextOperator
//
// rust: 偏差登记 —— Java 用**继承**（`extends KmoTextOperator`）；Rust 无继承，按仓库既有口径
//   （`KmoFilm.super_: KmoAtom<FilmAtomData>`）以 `super_` 字段组合承载，`save()` 的
//   `super.save(shape)` 直译为 `self.super_.save(shape)`。
//
// 【SP3 落地范围】全部字段、`setCell()`/`getRowIdx()`/`getColIdx()`、三个 `put*()` 的状态机写入、
//   四个读方法的**加锁骨架**、`save()` 的加锁 + `super.save()` 尾调用为真实实现；三处表遍历
//   （`anchor()`/`__isAnchorCenter()`/`__textDirection()`）与 `save()` 的表写回因 `KmoTable`/
//   `KmoTableCell`/`KmoRange`/`TableCell` 不可达而 DEFERRED（均带 owner/wave + Java 体原文）。
// owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3
pub struct KmoCellTextOperator {
    // Java: extends KmoTextOperator
    pub(crate) super_: KmoTextOperator,
    // Java field: private int _anchor = 0
    pub(crate) _anchor: i32,
    // Java field: private int textDirection = 0
    pub(crate) textDirection: i32,
    // Java field: private boolean isAnchorCenter = false
    pub(crate) isAnchorCenter: bool,
    // Java field: private boolean isAnchorChange = false
    pub(crate) isAnchorChange: bool,
    // Java field: private boolean isAnchorCenterChange = false
    pub(crate) isAnchorCenterChange: bool,
    // Java field: private boolean isTextDirectionChange = false
    pub(crate) isTextDirectionChange: bool,
    // Java field: private int rowIdx
    pub(crate) rowIdx: i32,
    // Java field: private int colIdx
    pub(crate) colIdx: i32,
    // Java field: private ReentrantReadWriteLock lock = new ReentrantReadWriteLock()
    //
    // rust: 偏差登记 —— Java 用**可重入**读写锁 `ReentrantReadWriteLock`；Rust `std::sync::RwLock`
    //   不可重入。本类 Java 侧不存在同线程重入取锁的路径（读方法只调各自 `__xxx()` 私有实现，
    //   不回调公开读方法），故 `RwLock<()>` 语义等价；若 SP3b 接线后出现重入，改用可重入实现并登记。
    //   rust: 另需 `Arc<RwLock<()>>` 而非裸 `RwLock<()>` —— Java `lock.readLock().lock()` 不借用
    //   `this`，故读锁可与 `this.__isAnchorCenter(shape)` 并存；Rust 若直接 `self.lock.read()`
    //   则守卫借走整个 `*self`，随后 `self.anchor(shape)` 的 `&mut self` 会 E0502（已由探针实测）。
    //   改用 `Arc::clone(&self.lock)` 取锁后，守卫只借用局部 `Arc`，Java 的「持锁调用私有实现」
    //   语义得以 1:1 保留（而非降级为「先取数再放锁」）。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3
    pub(crate) lock: Arc<RwLock<()>>,
}

// Java: public class KmoCellTextOperator extends KmoTextOperator
impl Default for KmoCellTextOperator {
    // Java: KmoCellTextOperator()（隐式；Java 三个 public 构造器都先 `super(..)`）
    fn default() -> Self {
        KmoCellTextOperator {
            // Java: super()（KmoTextOperator 的隐式父构造）
            super_: KmoTextOperator::default(),
            // Java field initializer: private int _anchor = 0
            _anchor: 0,
            // Java field initializer: private int textDirection = 0
            textDirection: 0,
            // Java field initializer: private boolean isAnchorCenter = false
            isAnchorCenter: false,
            // Java field initializer: private boolean isAnchorChange = false
            isAnchorChange: false,
            // Java field initializer: private boolean isAnchorCenterChange = false
            isAnchorCenterChange: false,
            // Java field initializer: private boolean isTextDirectionChange = false
            isTextDirectionChange: false,
            // Java field initializer: private int rowIdx（Java 默认 0）
            rowIdx: 0,
            // Java field initializer: private int colIdx（Java 默认 0）
            colIdx: 0,
            // Java field initializer: private ReentrantReadWriteLock lock = new ReentrantReadWriteLock()
            lock: Arc::new(RwLock::new(())),
        }
    }
}

// Java: public class KmoCellTextOperator extends KmoTextOperator
impl KmoCellTextOperator {
    // Java: public KmoCellTextOperator(KmoShowShape shape)
    pub fn new__kmoshowshape(shape: KmoShowShape) -> Self {
      // Java: super(shape);
      let mut inst = Self::default();
      inst.super_ = KmoTextOperator::new__kmoshowshape(shape);
      inst
    }

    // Java: public KmoCellTextOperator(int typePh, int idxPh, KmoTextBody textBody)
    // DEFERRED_NON_FUNCTION: 形参类型 `KmoTextBody` 默认构建下不可命名 ⇒ 构造器**签名本身**无法表达。
    //   Java 体：`super(typePh, idxPh, textBody);`（`KmoCellTextOperator.java:27-29`）。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: public KmoCellTextOperator(int typePh, int idxPh, KmoTextBody textBody, KmoShowShape shape)
    // DEFERRED_NON_FUNCTION: 同上。Java 体：`super(typePh, idxPh, textBody, shape);`
    //   （`KmoCellTextOperator.java:31-33`）。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: public void setCell(int rowIdx, int colIdx)
    pub fn setCell(&mut self, rowIdx: i32, colIdx: i32) {
      // Java: this.rowIdx = rowIdx;
      self.rowIdx = rowIdx;
      // Java: this.colIdx = colIdx;
      self.colIdx = colIdx;
    }

    // Java: public int getRowIdx()
    pub fn getRowIdx(&self) -> i32 {
      // Java: return this.rowIdx;
      self.rowIdx
    }

    // Java: public int getColIdx()
    pub fn getColIdx(&self) -> i32 {
      // Java: return this.colIdx;
      self.colIdx
    }

    // Java: public boolean putAnchor(int anchor)
    pub fn putAnchor(&mut self, anchor: i32) -> bool {
      // Java: try
      {
         // Java: this.lock.writeLock().lock();
         let lock = Arc::clone(&self.lock);
         let _guard = lock.write().unwrap();
         // Java: this._anchor = anchor;
         self._anchor = anchor;
         // Java: this.isAnchorChange = true;
         self.isAnchorChange = true;
         // Java: finally { this.lock.writeLock().unlock(); }
         // rust: `_guard` 出作用域自动 unlock（Java finally 的等价物）
      }
      // Java: return true;
      true
    }

    // Java: public int getAnchor(KmoShowShape shape)
    pub fn getAnchor(&mut self, shape: Option<&KmoShowShape>) -> i32 {
      // Java: int anchor = 0;
      let mut anchor: i32 = 0;
      // Java: try
      {
         // Java: this.lock.readLock().lock();
         let lock = Arc::clone(&self.lock);
         let _guard = lock.read().unwrap();
         // Java: anchor = this.anchor(shape);
         anchor = self.anchor(shape);
         // Java: finally { this.lock.readLock().unlock(); }
      }
      // Java: return anchor;
      anchor
    }

    // Java: private int anchor(KmoShowShape shape)
    //
    // rust: Java `private` → Rust 侧放开为 `pub`（登记），便于 SP3b 与验收单测直达。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3
    pub fn anchor(&mut self, shape: Option<&KmoShowShape>) -> i32 {
      // Java: int anchor = 0;
      // Java: if (shape.graphic().type() == 1)
      // Java: {
      // Java:    GraphicObject graphicObject = shape.graphic();
      // Java:    if (graphicObject != null)
      // Java:    {
      // Java:       KmoTable table = (KmoTable)graphicObject;
      // Java:       KmoRange range = table.selectedCellRange();
      // Java:       int leftCol = range != null ? range.leftCol : 0;
      // Java:       int rightCol = range != null ? range.rightCol : table.colSize() - 1;
      // Java:       int topRow = range != null ? range.topRow : 0;
      // Java:       int bottomRow = range != null ? range.bottomRow : table.rowSize() - 1;
      // Java:       boolean isFirst = true;
      // Java:       for (int col = leftCol; col <= rightCol; ++col)
      // Java:       {
      // Java:          for (int row = topRow; row <= bottomRow; ++row)
      // Java:          {
      // Java:             KmoTableCell cell = table.getCell(row, col);
      // Java:             TableCell cellData = TableCell.open(cell.protocol());
      // Java:             if (isFirst) { anchor = cellData.cellProperties().anchor(); isFirst = false; }
      // Java:             else if (cellData.hasCellProperties() && cellData.cellProperties().hasAnchor())
      // Java:             { if (anchor != cellData.cellProperties().anchor()) { return -1; } }
      // Java:             else if (anchor != 0) { return -1; }
      // Java:          }
      // Java:       }
      // Java:    }
      // Java: }
      // Java: return anchor;
      // DEFERRED_NON_FUNCTION: `KmoTable`（默认构建下仅空宿主，无 `selectedCellRange()`/`getCell()`/
      //   `colSize()`/`rowSize()`）/`KmoRange`/`TableCell`/`KmoTableCell` 均不可达。
      //   Java 体见 `KmoCellTextOperator.java:73-106`。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      let _ = shape;
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public Boolean isAnchorCenter(KmoShowShape shape)
    //
    // rust: Java 返回装箱 `Boolean` 且 `__isAnchorCenter()` 可能返回 `null`（不齐一分支）
    //   ⇒ Rust 侧 `Option<bool>`（`None` ≙ Java `null`）。
    pub fn isAnchorCenter(&mut self, shape: Option<&KmoShowShape>) -> Option<bool> {
      // Java: Boolean isAnchorCenter = false;
      let mut isAnchorCenter: Option<bool> = Some(false);
      // Java: try
      {
         // Java: this.lock.readLock().lock();
         let lock = Arc::clone(&self.lock);
         let _guard = lock.read().unwrap();
         // Java: isAnchorCenter = this.__isAnchorCenter(shape);
         isAnchorCenter = self.__isAnchorCenter(shape);
         // Java: finally { this.lock.readLock().unlock(); }
      }
      // Java: return isAnchorCenter;
      isAnchorCenter
    }

    // Java: private Boolean __isAnchorCenter(KmoShowShape shape)
    //
    // rust: 方法名保留 Java 的双下划线前缀（Rust 侧与 `anchor()` 区分）。Java `private` → `pub`（登记）。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3
    pub fn __isAnchorCenter(&mut self, shape: Option<&KmoShowShape>) -> Option<bool> {
      // Java: if (null == shape) { return false; }
      // Java: else
      // Java: {
      // Java:    boolean isAnchorCtr = false;
      // Java:    if (shape.graphic().type() == 1)
      // Java:    {
      // Java:       GraphicObject graphicObject = shape.graphic();
      // Java:       if (graphicObject != null)
      // Java:       {
      // Java:          KmoTable table = (KmoTable)graphicObject;
      // Java:          KmoRange range = table.selectedCellRange();
      // Java:          int leftCol = range != null ? range.leftCol : 0;
      // Java:          int rightCol = range != null ? range.rightCol : table.colSize() - 1;
      // Java:          int topRow = range != null ? range.topRow : 0;
      // Java:          int bottomRow = range != null ? range.bottomRow : table.rowSize() - 1;
      // Java:          boolean isFirst = true;
      // Java:          for (int col = leftCol; col <= rightCol; ++col)
      // Java:          {
      // Java:             for (int row = topRow; row <= bottomRow; ++row)
      // Java:             {
      // Java:                KmoTableCell cell = table.getCell(row, col);
      // Java:                TableCell cellData = TableCell.open(cell.protocol());
      // Java:                if (isFirst) { isAnchorCtr = cellData.cellProperties().isAnchorCenter(); isFirst = false; }
      // Java:                else if (cellData.hasCellProperties() && cellData.cellProperties().hasAnchor())
      // Java:                { if (isAnchorCtr != cellData.cellProperties().isAnchorCenter()) { return null; } }
      // Java:                else if (isAnchorCtr) { return null; }
      // Java:             }
      // Java:          }
      // Java:       }
      // Java:    }
      // Java:    return isAnchorCtr;
      // Java: }
      // DEFERRED_NON_FUNCTION: 依赖同 `anchor()`（`KmoTable`/`KmoRange`/`TableCell`/`KmoTableCell`
      //   均不可达）。Java 体见 `KmoCellTextOperator.java:121-158`。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      let _ = shape;
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public boolean putAnchorCenter(boolean isAnchorCenter)
    pub fn putAnchorCenter(&mut self, isAnchorCenter: bool) -> bool {
      // Java: try
      {
         // Java: this.lock.writeLock().lock();
         let lock = Arc::clone(&self.lock);
         let _guard = lock.write().unwrap();
         // Java: this.isAnchorCenter = isAnchorCenter;
         self.isAnchorCenter = isAnchorCenter;
         // Java: this.isAnchorCenterChange = true;
         self.isAnchorCenterChange = true;
         // Java: finally { this.lock.writeLock().unlock(); }
      }
      // Java: return true;
      true
    }

    // Java: public int textDirection(KmoShowShape shape)
    pub fn textDirection(&mut self, shape: Option<&KmoShowShape>) -> i32 {
      // Java: int textDirection = -1;
      let mut textDirection: i32 = -1;
      // Java: try
      {
         // Java: this.lock.readLock().lock();
         let lock = Arc::clone(&self.lock);
         let _guard = lock.read().unwrap();
         // Java: textDirection = this.__textDirection(shape);
         textDirection = self.__textDirection(shape);
         // Java: finally { this.lock.readLock().unlock(); }
      }
      // Java: return textDirection;
      textDirection
    }

    // Java: private int __textDirection(KmoShowShape shape)
    //
    // rust: 方法名保留 Java 的双下划线前缀。Java `private` → `pub`（登记）。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3
    pub fn __textDirection(&mut self, shape: Option<&KmoShowShape>) -> i32 {
      // Java: if (null != shape && 4 == shape.type())
      // Java: {
      // Java:    GraphicObject graphicObject = shape.graphic();
      // Java:    if (null != graphicObject && 1 == graphicObject.type())
      // Java:    {
      // Java:       int textDirection = 0;
      // Java:       KmoTable table = (KmoTable)graphicObject;
      // Java:       KmoRange range = table.selectedCellRange();
      // Java:       int leftCol = range != null ? range.leftCol : 0;
      // Java:       int rightCol = range != null ? range.rightCol : table.colSize() - 1;
      // Java:       int topRow = range != null ? range.topRow : 0;
      // Java:       int bottomRow = range != null ? range.bottomRow : table.rowSize() - 1;
      // Java:       boolean isFirst = true;
      // Java:       for (int col = leftCol; col <= rightCol; ++col)
      // Java:       {
      // Java:          for (int row = topRow; row <= bottomRow; ++row)
      // Java:          {
      // Java:             KmoTableCell cell = table.getCell(row, col);
      // Java:             TableCell cellData = TableCell.open(cell.protocol());
      // Java:             if (isFirst) { textDirection = cellData.cellProperties().textDirection(); isFirst = false; }
      // Java:             else if (cellData.hasCellProperties() && cellData.cellProperties().hasTextDirection())
      // Java:             { if (textDirection != cellData.cellProperties().textDirection()) { return -1; } }
      // Java:             else if (textDirection != 0) { return -1; }
      // Java:          }
      // Java:       }
      // Java:       return textDirection;
      // Java:    }
      // Java:    else { return -1; }
      // Java: }
      // Java: else { return -1; }
      // DEFERRED_NON_FUNCTION: 依赖同 `anchor()`。Java 体见 `KmoCellTextOperator.java:185-222`。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      let _ = shape;
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public boolean putTextDirection(int textDirection)
    pub fn putTextDirection(&mut self, textDirection: i32) -> bool {
      // Java: try
      {
         // Java: this.lock.writeLock().lock();
         let lock = Arc::clone(&self.lock);
         let _guard = lock.write().unwrap();
         // Java: this.textDirection = textDirection;
         self.textDirection = textDirection;
         // Java: this.isTextDirectionChange = true;
         self.isTextDirectionChange = true;
         // Java: finally { this.lock.writeLock().unlock(); }
      }
      // Java: return true;
      true
    }

    // Java: public boolean save(KmoShowShape shape)
    pub fn save(&mut self, shape: Option<&KmoShowShape>) -> bool {
      // Java: boolean result = false;
      let mut result = false;
      // Java: this.lock.writeLock().lock();
      let lock = Arc::clone(&self.lock);
      let _guard = lock.write().unwrap();
      // Java: try
      {
         // Java: if (shape.graphic().type() == 1)
         if let Some(shape_ref) = shape {
            if let Some(graphicObject) = shape_ref.graphic() {
               if graphicObject.r#type() == 1 {
                  // Java: GraphicObject graphicObject = shape.graphic();
                  // Java: if (graphicObject != null)
                  // Java: {
                  // Java:    KmoTable table = (KmoTable)graphicObject;
                  // Java:    KmoRange range = table.selectedCellRange();
                  // Java:    int leftCol = range != null ? range.leftCol : 0;
                  // Java:    int rightCol = range != null ? range.rightCol : table.colSize() - 1;
                  // Java:    int topRow = range != null ? range.topRow : 0;
                  // Java:    int bottomRow = range != null ? range.bottomRow : table.rowSize() - 1;
                  // Java:    for (int col = leftCol; col <= rightCol; ++col)
                  // Java:    {
                  // Java:       for (int row = topRow; row <= bottomRow; ++row)
                  // Java:       {
                  // Java:          KmoTableCell cell = table.getCell(row, col);
                  // Java:          if (this.isAnchorChange) { cell.setAnchor(this._anchor); }
                  // Java:          if (this.isAnchorCenterChange) { cell.setAnchorCenter(this.isAnchorCenter); }
                  // Java:          if (this.isTextDirectionChange) { cell.setTextDirection(this.textDirection); }
                  // Java:       }
                  // Java:    }
                  // Java: }
                  // DEFERRED_NON_FUNCTION: `KmoTable`/`KmoRange`/`KmoTableCell` 不可达（表写回段）。
                  //   Java 体见 `KmoCellTextOperator.java:241-268`。
                  //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
               }
            }
         }
         // Java: result = super.save(shape);
         result = self.super_.save(shape);
      }
      // Java: finally { this.lock.writeLock().unlock(); }
      // rust: `_guard` 出作用域自动 unlock（Java finally 的等价物）
      // Java: return result;
      result
    }
}
