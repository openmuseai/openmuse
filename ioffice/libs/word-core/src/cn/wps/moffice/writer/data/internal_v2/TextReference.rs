// Java: import cn.wps.moffice.writer.data.internal_v2.ListNode
use crate::cn::wps::moffice::writer::data::internal_v2::ListNode::*;
use std::ptr;

// java public class TextReference extends ListNode
#[repr(C)]
pub struct TextReference<T> {
    // java ListNode super_
    pub(crate) super_: ListNode,
    // java T data;
    pub(crate) data: Option<T>,
    /// Back-pointer to owning CPTable (Java outer instance).
    pub(crate) table: *mut (),
}

// java public class TextReference extends ListNode
impl<T> TextReference<T> {
    // java public  TextReference(int offset, T data)
    pub fn new__int_t(offset: i32, data: Option<T>) -> Self {
      // java super(offset);
      // java this.data = data;
      Self { super_: ListNode::ListNode(offset), data, table: ptr::null_mut() }
    }

    pub fn alloc(offset: i32, data: Option<T>, table: *mut ()) -> *mut Self {
      Box::into_raw(Box::new(Self {
        super_: ListNode::ListNode(offset),
        data,
        table,
      }))
    }

    // java public  TextReference(T data)
    pub fn new__t(data: Option<T>) -> Self {
      // java this(0, data);
      Self::new__int_t(0, data)
    }

    // java public final T getData()
    pub fn getData(&self) -> Option<&T> {
      // java return this.data;
      return self.data.as_ref();
    }

    // java public void setData(T data)
    pub fn setData(&mut self, data: Option<T>) {
      // java this.data = data;
      self.data = data;
    }

    pub fn as_list_node(&self) -> *mut ListNode {
      self as *const TextReference<T> as *mut ListNode
    }

    pub fn as_pos(&self) -> *mut util::collections::PositionNode::PositionNode {
      self as *const TextReference<T> as *mut util::collections::PositionNode::PositionNode
    }

    pub fn as_node(&self) -> *mut util::collections::Node::Node {
      self as *const TextReference<T> as *mut util::collections::Node::Node
    }

    // java public String toString()
    pub fn toString(&self) -> String {
      // java StringBuilder sb = new StringBuilder();
      let mut sb: String = String::new();
      // java sb.append(this.getClass().getSimpleName() + ": ");
      sb.push_str("TextReference: ");
      // java sb.append("cp = " + this.position());
      sb.push_str(&format!("cp = {}", self.super_.position()));
      // java return sb.toString();
      return sb;
    }

}

impl<T> Default for TextReference<T> {
    fn default() -> Self {
        Self {
            super_: ListNode::default(),
            data: None,
            table: ptr::null_mut(),
        }
    }
}

impl<T> std::ops::Deref for TextReference<T> {
    type Target = ListNode;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl<T> std::ops::DerefMut for TextReference<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
