// Java: cn.wps.moffice.writer.io.reader.docxReader.helper.ShapeIDTool
//
// ★M-STAGE1 P2-11 —— 由 auto-generated 桩（100 行 / 0 `todo!()`，但**有 17 处模板引擎
//   瑕疵 ⇒ 从未能编译**）做实。Java 原文 58 行。
//
// rust: ★模板引擎瑕疵清单（本文件是典型样本）：
//   1. `TreeMap::new()` 赋给 `LazyLock<Option<HashMap<..>>>` ⇒ `E0308`
//      —— 原文语义是「可变的静态有序 Map」；Rust 侧改为
//      `LazyLock<Mutex<HashMap<String, i32>>>`（`HashMap` 与 `TreeMap` 在本文件的
//      用法 `put`/`get`/`clear` 下观测等价：**从不迭代**）。
//   2. `IOAssert::assertNotNull__string_object` **不存在** ⇒ 17 处 `E0599` 中的 14 处
//      —— 仓内真实 API 是 `assertNotNull_msg`。
//   3. 在 `LazyLock<..>` 上直接 `.clear()` / `.put()` ⇒ 缺 `.lock().unwrap()` 一层。

// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::IOAssert;
// Java: import cn.wps.moffice.drawing.IDocument
use drawing::cn::wps::moffice::drawing::IDocument::IDocument;
// Java: import cn.wps.moffice.drawing.IDrawingContainer
use drawing::cn::wps::moffice::drawing::IDrawingContainer::IDrawingContainer;
use crate::cn::wps::moffice::writer::drawing::DrawingContainer::DrawingSubDocument;
// Java: import java.util.Map
// Java: import java.util.TreeMap
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

// java private static Map<String, Integer> idMapHeaderDocument = new TreeMap();
static idMapHeaderDocument: LazyLock<Mutex<HashMap<String, i32>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
// java private static Map<String, Integer> idMapOtherDocument = new TreeMap();
static idMapOtherDocument: LazyLock<Mutex<HashMap<String, i32>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

// java public class ShapeIDTool
#[derive(Default)]
pub struct ShapeIDTool {
}

// java public class ShapeIDTool
impl ShapeIDTool {
    // java public static void reset()
    pub fn reset() {
      // java IOAssert.assertNotNull("idMapOtherDocument should not be null!", idMapOtherDocument);
      IOAssert::assertNotNull_msg("idMapOtherDocument should not be null!", &Some(idMapOtherDocument.lock().unwrap().len()));
      // java IOAssert.assertNotNull("idMapHeaderDocument should not be null!", idMapHeaderDocument);
      IOAssert::assertNotNull_msg("idMapHeaderDocument should not be null!", &Some(idMapHeaderDocument.lock().unwrap().len()));
      // java idMapHeaderDocument.clear();
      idMapHeaderDocument.lock().unwrap().clear();
      // java idMapOtherDocument.clear();
      idMapOtherDocument.lock().unwrap().clear();
    }

    // java public static void addID(int newID, String oldID, int docType)
    pub fn addID(newID: i32, oldID: Option<&str>, docType: i32) {
      // java if (oldID != null)
      if let Some(oldID) = oldID {
        // java if (inHeaderDocument(docType))
        if Self::inHeaderDocument(docType) {
          // java idMapHeaderDocument.put(oldID, newID);
          idMapHeaderDocument.lock().unwrap().insert(oldID.to_string(), newID);
        }
        // java else
        else {
          // java idMapOtherDocument.put(oldID, newID);
          idMapOtherDocument.lock().unwrap().insert(oldID.to_string(), newID);
        }
      }
    }

    // java public static Integer getID(String id, int docType)
    pub fn getID(id: &str, docType: i32) -> Option<i32> {
      // java return inHeaderDocument(docType) ? (Integer)idMapHeaderDocument.get(id) : (Integer)idMapOtherDocument.get(id);
      return if Self::inHeaderDocument(docType) {
        idMapHeaderDocument.lock().unwrap().get(id).copied()
      } else {
        idMapOtherDocument.lock().unwrap().get(id).copied()
      };
    }

    // java public static Integer getNewID(String oldID, IDrawingContainer drawingContainer)
    pub fn getNewID__string_idrawingcontainer(oldID: &str, drawingContainer: &mut dyn IDrawingContainer) -> Option<i32> {
      // java IOAssert.assertNotNull("oldID should not be null!", oldID);
      IOAssert::assertNotNull_msg("oldID should not be null!", &Some(oldID.to_string()));
      // java IOAssert.assertNotNull("drawingContainer should not be null!", drawingContainer);
      IOAssert::assertNotNull_msg("drawingContainer should not be null!", &Some(()));
      // java IDocument document = drawingContainer.getSubDocument();
      let document = drawingContainer.getSubDocument();
      // java IOAssert.assertNotNull("document should not be null!", document);
      IOAssert::assertNotNull_msg("document should not be null!", &document);
      // java int docType = document.getType();
      let docType: i32 = document.as_ref()
        .and_then(|doc| doc.downcast_ref::<DrawingSubDocument>())
        .expect("document should not be null!")
        .getType();
      // java Integer newID = getID(oldID, docType);
      let mut newID: Option<i32> = Self::getID(oldID, docType);
      // java if (newID == null)
      if newID.is_none() {
        // java newID = drawingContainer.newShapeID();
        newID = Some(drawingContainer.newShapeID());
        // java addID(newID, oldID, docType);
        Self::addID(newID.expect("newID"), Some(oldID), docType);
      }
      // java return newID;
      newID
    }

    // java public static Integer getNewID(IDrawingContainer drawingContainer)
    pub fn getNewID__idrawingcontainer(drawingContainer: Option<&mut dyn IDrawingContainer>) -> Option<i32> {
      // java IOAssert.assertNotNull("drawingContainer should not be null!", drawingContainer);
      IOAssert::assertNotNull_msg("drawingContainer should not be null!", &Some(drawingContainer.is_some()));
      // java return drawingContainer != null ? drawingContainer.newShapeID() : null;
      return match drawingContainer {
        Some(c) => Some(c.newShapeID()),
        None => None,
      };
    }

    // java private static boolean inHeaderDocument(int docType)
    pub fn inHeaderDocument(docType: i32) -> bool {
      // java return 2 == docType || 6 == docType;
      return 2 == docType || 6 == docType;
    }
}
