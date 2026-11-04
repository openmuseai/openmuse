// Java: cn.wps.moffice.writer.layout.base.graph.DrawingNode
// rust: ★wr-29 恢复 —— 真实面（原文件为 j2r 垃圾：native-code toString 不可编译）
#![allow(dead_code, unused_imports, non_snake_case, unused_variables)]

use crate::cn::wps::moffice::writer::core::Document::Document;

// java public class DrawingNode extends TypoMemoryObj
#[derive(Default, Clone)]
pub struct DrawingNode {
    // java public Document document;
    pub document: Option<*mut dyn Document>,
    // java public ProfileData mProfileData;
    pub mProfileData: Option<()>,
    // java public boolean mIsSimpleTextFrame;
    pub mIsSimpleTextFrame: bool,
}
impl DrawingNode {
    pub fn new() -> Self { Self::default() }
    pub fn init(&mut self) {}
    pub fn dispose(&mut self) {}
    pub fn alloc(&mut self) {}
}
