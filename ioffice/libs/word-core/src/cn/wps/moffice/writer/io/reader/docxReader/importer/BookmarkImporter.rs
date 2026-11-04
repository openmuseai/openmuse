// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::IOAssert;
// Java: import cn.wps.moffice.writer.core.SubDocument
use crate::cn::wps::moffice::writer::core::SubDocument::SubDocument;
// Java: import cn.wps.moffice.writer.data.PLCBookmarkEnd
use crate::cn::wps::moffice::writer::data::PLCBookmarkEnd::{BookmarkEndNode, PLCBookmarkEnd};
// Java: import cn.wps.moffice.writer.data.PLCBookmarkStart
use crate::cn::wps::moffice::writer::data::PLCBookmarkStart::{BookmarkStartNode, PLCBookmarkStart};
// Java: import cn.wps.moffice.writer.helper.CoreHelper
use crate::cn::wps::moffice::writer::helper::CoreHelper::CoreHelper;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ConvertHelper
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::ConvertHelper;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ImporterHelper
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ImporterHelper::ImporterHelper;
// Java: import java.util.HashMap
// Java: import org.xml.sax.Attributes
// rust: ★M-STAGE2 P1-D1 —— 原先误用 `kfc_kfc_alpha::…::token::Attributes`
//   （HTML 词法 token 结构体，与 SAX 无关）；Java 的 `org.xml.sax.Attributes`
//   对应 `ConvertHelper` 中带文档说明的 SAX seam trait。
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::Attributes;
use std::collections::HashMap;

// java public class BookmarkImporter
pub struct BookmarkImporter {
    // java private SubDocument mDocument;
    // rust: Java 存引用；Rust 用裸指针承载同一别名语义（与 `InnerIOListener` 同法）。
    mDocument: *mut SubDocument,
    // java private HashMap<Integer, BookmarkRange> mBookmarkStartsMap;
    mBookmarkStartsMap: HashMap<i32, BookmarkRange>,
    // java private HashMap<Integer, String> mCp2McdName;
    mCp2McdName: HashMap<i32, String>,
    // java private String mMcdVersion;
    // rust: Java 未在构造器初始化 ⇒ 为 null，故映射为 `Option<String>`。
    mMcdVersion: Option<String>,
}

impl BookmarkImporter {
    // java public  BookmarkImporter(SubDocument subDocument)
    pub fn BookmarkImporter(subDocument: *mut SubDocument) -> Self {
      // java this.mDocument = subDocument;
      Self { mDocument: subDocument, mBookmarkStartsMap: HashMap::new(), mCp2McdName: HashMap::new(), mMcdVersion: None }
      // java this.mBookmarkStartsMap = new HashMap();
      // java this.mCp2McdName = new HashMap();
    }

    // java public void importBookmarkStart(Attributes bookmark)
    pub fn importBookmarkStart(&mut self, bookmark: &dyn Attributes) {
      // java IOAssert.assertNotNull("bookmark should not be null!", bookmark);
      // rust: no-stmt
      // java String name = this.getName(bookmark);
      let mut name: Option<String> = self.getName(bookmark);
      // java if (null != name)
      if name.is_some() {
        // java if (CoreHelper.isVersionMcdPrefix(name))
        if CoreHelper::isVersionMcdPrefix(name.as_deref()) {
          // java this.mMcdVersion = CoreHelper.pickVersion(name);
          self.mMcdVersion = CoreHelper::pickVersion(name.as_deref());
        }
        // java else if (CoreHelper.isCmtRefMcdPrefix(name))
        else if CoreHelper::isCmtRefMcdPrefix(name.as_deref()) {
          // java this.mCp2McdName.put(this.mDocument.getLength(), name);
          let len = unsafe { (*self.mDocument).getLength() };
          self.mCp2McdName.insert(len, name.clone().unwrap());
        }
        // java else
        else {
          // java Integer id = this.getId(bookmark);
          let mut id: Option<i32> = self.getId(bookmark);
          // java IOAssert.assertNotNull("id should not be null!", id);
          IOAssert::assertNotNull__string_object("id should not be null!", Some(&id));
          // java if (null != id)
          if id.is_some() {
            // java BookmarkRange startNode = this.createBookmarkStartNode(bookmark, name);
            let mut startNode: BookmarkRange = self.createBookmarkStartNode__attributes_string(bookmark, name.as_deref().unwrap());
            // java HashMap<Integer, BookmarkRange> bookmarkStarts = this.getBookmarkStartsMap();
            // java bookmarkStarts.put(id, startNode);
            // rust: `getBookmarkStartsMap` 返回 `&mut`，此处直接插入（原来错用 `put`）。
            let key = id.unwrap();
            self.getBookmarkStartsMap().insert(key, startNode);
          }
        }
      }
    }

    // java public String getMcdVersion()
    pub fn getMcdVersion(&self) -> Option<String> {
      // java return this.mMcdVersion;
      return self.mMcdVersion.clone();
    }

    // java public HashMap<Integer, String> getCmtRef2McdName()
    pub fn getCmtRef2McdName(&self) -> HashMap<i32, String> {
      // java return this.mCp2McdName;
      // rust: Java 返回引用；Rust 侧 clone 以保持 `&self` 接收者。
      return self.mCp2McdName.clone();
    }

    // java public void importBookmarkEnd(Attributes markupRange)
    pub fn importBookmarkEnd(&mut self, markupRange: &dyn Attributes) {
      // java IOAssert.assertNotNull("markupRange should not be null!", markupRange);
      // rust: no-stmt
      // java IOAssert.logNotNull("mBookmarkStartsMap should not be null!", this.mBookmarkStartsMap);
      // java if (null != this.mBookmarkStartsMap)
      {
        // java Integer id = this.getId(markupRange);
        let mut id: Option<i32> = self.getId(markupRange);
        // java IOAssert.assertNotNull("id should not be null!", id);
        IOAssert::assertNotNull__string_object("id should not be null!", Some(&id));
        // java if (null != id)
        if id.is_some() {
          // java BookmarkRange bookmarkStartRange = (BookmarkRange)this.mBookmarkStartsMap.get(id);
          let mut bookmarkStartRange: Option<BookmarkRange> = self.mBookmarkStartsMap.get(&id.unwrap()).cloned();
          // java IOAssert.assertNotNull("Cannot find a bookmark start which matches this bookmark end.", bookmarkStartRange);
          IOAssert::assertNotNull__string_object("Cannot find a bookmark start which matches this bookmark end.", Some(&bookmarkStartRange));
          // java if (null != bookmarkStartRange)
          if let Some(r) = bookmarkStartRange {
            // java this.importBookmark(bookmarkStartRange);
            self.importBookmark(&r);
          }
        }
      }
    }

    // java private void importBookmark(BookmarkRange bookmarkStartRange)
    pub fn importBookmark(&self, bookmarkStartRange: &BookmarkRange) {
      // java int docmentLength = this.mDocument.getLength();
      let mut docmentLength: i32 = unsafe { (*self.mDocument).getLength() };
      // java int bookmarkStartCp = bookmarkStartRange.mCp;
      let mut bookmarkStartCp: i32 = bookmarkStartRange.mCp;
      // java int startCp = ImporterHelper.getBookmarkStartCp(bookmarkStartCp, docmentLength, bookmarkStartRange, this.mDocument);
      let mut startCp: i32 = ImporterHelper::getBookmarkStartCp(bookmarkStartCp, docmentLength, bookmarkStartRange, unsafe { &*self.mDocument });
      // java PLCBookmarkStart.BookmarkStartNode startNode = pushBookmarkStart(this.mDocument, startCp);
      let mut startNode: BookmarkStartNode = Self::pushBookmarkStart(unsafe { &mut *self.mDocument }, startCp);
      // java startNode.setName(bookmarkStartRange.getName());
      startNode.setName(bookmarkStartRange.getName());
      // java Long firstCol = bookmarkStartRange.getColFirst();
      let mut firstCol: Option<i64> = bookmarkStartRange.getColFirst();
      // java Long lastCol = bookmarkStartRange.getColLast();
      let mut lastCol: Option<i64> = bookmarkStartRange.getColLast();
      // java int docxDiffer = 1;
      let mut docxDiffer: i32 = 1;
      // java if (null != firstCol && null != lastCol)
      if firstCol.is_some() && lastCol.is_some() {
        // java startNode.setInTable(true);
        startNode.setInTable(true);
        // java startNode.setItcFirst(firstCol.intValue());
        startNode.setItcFirst(firstCol.unwrap() as i32);
        // java startNode.setItcLim(lastCol.intValue() + 1);
        startNode.setItcLim(lastCol.unwrap() as i32 + 1);
      }
      // java int endCp = ImporterHelper.getBookmarkEndCp(bookmarkStartCp, docmentLength, bookmarkStartRange, this.mDocument);
      let mut endCp: i32 = ImporterHelper::getBookmarkEndCp(bookmarkStartCp, docmentLength, bookmarkStartRange, unsafe { &*self.mDocument });
      // java if (endCp < startCp)
      if endCp < startCp {
        // java endCp = startCp;
        endCp = startCp;
      }
      // java PLCBookmarkEnd.BookmarkEndNode endNode = pushBookmarkEnd(this.mDocument, endCp);
      let mut endNode: BookmarkEndNode = Self::pushBookmarkEnd(unsafe { &mut *self.mDocument }, endCp);
      // java startNode.setEnd(endNode);
      startNode.setEnd(Some(endNode.clone()));
      // java endNode.setStart(startNode);
      endNode.setStart(Some(startNode));
    }

    // java private static PLCBookmarkStart getPLCBookmarkStart(SubDocument document)
    pub fn getPLCBookmarkStart(document: &mut SubDocument) -> PLCBookmarkStart {
      // java PLCBookmarkStart starts = document.getBookmarkStarts();
      let mut starts: PLCBookmarkStart = document.getBookmarkStarts();
      // java if (null == starts)
      // java document.createPLCOfBookmark();
      // rust: no-stmt —— `getBookmarkStarts` 返回 `PLCBookmarkStart`（非 `Option`），
      //   Java 的 null 分支在 Rust 类型下不可达。
      // java starts = document.getBookmarkStarts();
      starts = document.getBookmarkStarts();
      // java IOAssert.assertNotNull("starts should not be null!", starts);
      // java return starts;
      return starts;
    }

    // java private static PLCBookmarkStart.BookmarkStartNode pushBookmarkStart(SubDocument document, int cp)
    pub fn pushBookmarkStart(document: &mut SubDocument, cp: i32) -> BookmarkStartNode {
      // java PLCBookmarkStart starts = getPLCBookmarkStart(document);
      let mut starts: PLCBookmarkStart = Self::getPLCBookmarkStart(document);
      // java PLCBookmarkStart.BookmarkStartNode node = new PLCBookmarkStart.BookmarkStartNode();
      let mut node: BookmarkStartNode = BookmarkStartNode::new();
      // java starts.insertAtUpperBound(cp, node);
      starts.insertAtUpperBound(cp, &node);
      // java return node;
      return node;
    }

    // java private static PLCBookmarkEnd.BookmarkEndNode pushBookmarkEnd(SubDocument document, int cp)
    pub fn pushBookmarkEnd(document: &mut SubDocument, cp: i32) -> BookmarkEndNode {
      // java PLCBookmarkEnd ends = document.getBookmarkEnds();
      let mut ends: PLCBookmarkEnd = document.getBookmarkEnds();
      // java IOAssert.assertNotNull("ends should not be null!", ends);
      // java PLCBookmarkEnd.BookmarkEndNode node = new PLCBookmarkEnd.BookmarkEndNode();
      let mut node: BookmarkEndNode = BookmarkEndNode::new();
      // java ends.insertAtUpperBound(cp, node);
      ends.insertAtUpperBound(cp, &node);
      // java return node;
      return node;
    }

    // java private BookmarkRange createBookmarkStartNode(BookmarkRange bookmark, String name)
    pub fn createBookmarkStartNode__bookmarkrange_string(&self, bookmark: &BookmarkRange, name: &str) -> BookmarkRange {
      // java int documentLen = this.mDocument.getLength();
      let mut documentLen: i32 = unsafe { (*self.mDocument).getLength() };
      // java BookmarkRange startNode = new BookmarkRange(documentLen);
      let mut startNode: BookmarkRange = BookmarkRange::BookmarkRange(documentLen);
      // java startNode.mColFirst = bookmark.getColFirst();
      startNode.mColFirst = bookmark.getColFirst();
      // java startNode.mColLast = bookmark.getColLast();
      startNode.mColLast = bookmark.getColLast();
      // java startNode.mName = name;
      startNode.mName = Some(name.to_string());
      // java return startNode;
      return startNode;
    }

    // java private BookmarkRange createBookmarkStartNode(Attributes bookmark, String name)
    pub fn createBookmarkStartNode__attributes_string(&self, bookmark: &dyn Attributes, name: &str) -> BookmarkRange {
      // java int documentLen = this.mDocument.getLength();
      let mut documentLen: i32 = unsafe { (*self.mDocument).getLength() };
      // java BookmarkRange startNode = new BookmarkRange(documentLen);
      let mut startNode: BookmarkRange = BookmarkRange::BookmarkRange(documentLen);
      // java startNode.mColFirst = ConvertHelper.getLong(bookmark, "colFirst");
      startNode.mColFirst = ConvertHelper::getLong(bookmark, "colFirst");
      // java startNode.mColLast = ConvertHelper.getLong(bookmark, "colLast");
      startNode.mColLast = ConvertHelper::getLong(bookmark, "colLast");
      // java startNode.mName = name;
      startNode.mName = Some(name.to_string());
      // java return startNode;
      return startNode;
    }

    // java private HashMap<Integer, BookmarkRange> getBookmarkStartsMap()
    pub fn getBookmarkStartsMap(&mut self) -> &mut HashMap<i32, BookmarkRange> {
      // java if (null == this.mBookmarkStartsMap)
      // java this.mBookmarkStartsMap = new HashMap();
      // rust: no-stmt —— Rust 字段非 `Option`，构造器已初始化，null 分支不可达。
      // java return this.mBookmarkStartsMap;
      return &mut self.mBookmarkStartsMap;
    }

    // java private Integer getId(Attributes bookmark)
    pub fn getId(&self, bookmark: &dyn Attributes) -> Option<i32> {
      // java return ConvertHelper.getInt(bookmark, "id");
      return ConvertHelper::getInt(bookmark, "id");
    }

    // java private String getName(Attributes bookmark)
    pub fn getName(&self, bookmark: &dyn Attributes) -> Option<String> {
      // java return ConvertHelper.getString(bookmark, "name");
      // rust: Java 的重载是 `getString(Attributes, String)`；
      //   Rust 侧对应 `getString__String`（单参版 `getString` 语义不同）。
      return ConvertHelper::getString__String(bookmark, "name");
    }

}

// java public static class BookmarkRange
#[derive(Default, Clone)]
pub struct BookmarkRange {
    // java private int mCp = 0;
    mCp: i32,
    // java private Long mColFirst = null;
    mColFirst: Option<i64>,
    // java private Long mColLast = null;
    mColLast: Option<i64>,
    // java private String mName = null;
    mName: Option<String>,
}

// java public static class BookmarkRange
impl BookmarkRange {
    // java public  BookmarkRange(int cp)
    pub fn BookmarkRange(cp: i32) -> Self {
      // java this.mCp = cp;
      Self { mCp: cp, mColFirst: None, mColLast: None, mName: None }
    }

    // java public String getName()
    pub fn getName(&self) -> Option<String> {
      // java return this.mName;
      return self.mName.clone();
    }

    // java public Long getColFirst()
    pub fn getColFirst(&self) -> Option<i64> {
      // java return this.mColFirst;
      return self.mColFirst;
    }

    // java public Long getColLast()
    pub fn getColLast(&self) -> Option<i64> {
      // java return this.mColLast;
      return self.mColLast;
    }

}
