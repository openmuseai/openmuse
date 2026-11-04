#![allow(dead_code, non_snake_case, unused_variables, unused_imports)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// Java: import cn.wps.show.app.KmoPresentation
use crate::cn::wps::show::util::layout_shape::KmoPresentation;
// Java: import java.util.HashMap
// Java: import java.util.Map

// java public class CommentIdContext
#[derive(Clone, Default)]
pub struct CommentIdContext {
    // java private Map<Integer, Integer> mCommentIdMap;
    mCommentIdMap: HashMap<i32, i32>,
    // java private KmoPresentation mPres;
    mPres: Option<Rc<RefCell<KmoPresentation>>>,
}

// java public class CommentIdContext
impl CommentIdContext {
    // java public  CommentIdContext(KmoPresentation pres)
    pub fn CommentIdContext(pres: Rc<RefCell<KmoPresentation>>) -> Self {
      // java this.mPres = pres;
      // java this.mCommentIdMap = new HashMap();
      Self {
        mCommentIdMap: HashMap::new(),
        mPres: Some(pres),
      }
    }

    // java public void put(int ioId, int newId)
    pub fn put(&mut self, ioId: i32, newId: i32) {
      // java this.mCommentIdMap.put(ioId, newId);
      self.mCommentIdMap.insert(ioId, newId);
    }

    // java public int get(int ioId)
    pub fn get(&mut self, ioId: i32) -> i32 {
      // java if (this.mCommentIdMap.get(ioId) != null)
      if let Some(existing) = self.mCommentIdMap.get(&ioId).copied() {
        // java return (Integer)this.mCommentIdMap.get(ioId);
        return existing;
      }
      // java else
      else {
        // java int nextId = this.mPres.nextCommentId();
        let nextId: i32 = self
          .mPres
          .as_ref()
          .map(|p| p.borrow_mut().nextCommentId())
          .unwrap_or(-1);
        // java this.mCommentIdMap.put(ioId, nextId);
        self.mCommentIdMap.insert(ioId, nextId);
        // java return nextId;
        return nextId;
      }
    }

    // rust: 可观测面（Java 无此方法；等价于 `mCommentIdMap.size()`）
    pub fn mapped_len(&self) -> usize {
        self.mCommentIdMap.len()
    }
}
