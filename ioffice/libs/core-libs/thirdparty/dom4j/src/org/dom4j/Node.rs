// Java: import java.io.IOException
use std::io;
// Java: import java.io.Writer
use std::io::Write;
// Java: import java.util.List
use std::any::Any;

use super::Element::Element;
use super::Document::Document;
use super::NodeType::NodeType;
use super::xpath::XPath;
use super::Visitor::Visitor;

// Java: public interface Node extends Cloneable
pub trait Node {
    // Java: abstract boolean supportsParent()
    fn supportsParent(&self) -> bool;

    // Java: abstract Element getParent()
    fn getParent(&self) -> Option<&dyn Element>;

    // Java: abstract void setParent(Element var1)
    fn setParent(&mut self, var1: Option<&dyn Element>);

    // Java: abstract Document getDocument()
    fn getDocument(&self) -> Option<&dyn Document>;

    // Java: abstract void setDocument(Document var1)
    fn setDocument(&mut self, var1: Option<&dyn Document>);

    // Java: abstract boolean isReadOnly()
    fn isReadOnly(&self) -> bool;

    // Java: abstract boolean hasContent()
    fn hasContent(&self) -> bool;

    // Java: abstract String getName()
    fn getName(&self) -> &str;

    // Java: abstract void setName(String var1)
    fn setName(&mut self, var1: &str);

    // Java: abstract String getText()
    fn getText(&self) -> &str;

    // Java: abstract void setText(String var1)
    fn setText(&mut self, var1: &str);

    // Java: abstract String getStringValue()
    fn getStringValue(&self) -> String;

    // Java: abstract String getPath()
    fn getPath(&self) -> String;

    // Java: abstract String getPath(Element var1)
    fn getPath_with_context(&self, var1: &dyn Element) -> String;

    // Java: abstract String getUniquePath()
    fn getUniquePath(&self) -> String;

    // Java: abstract String getUniquePath(Element var1)
    fn getUniquePath_with_context(&self, var1: &dyn Element) -> String;

    // Java: abstract String asXML()
    fn asXML(&self) -> String;

    // Java: abstract void write(Writer var1)
    fn write(&self, var1: &mut dyn Write) -> io::Result<()>;

    // Java: abstract NodeType getNodeTypeEnum()
    fn getNodeTypeEnum(&self) -> NodeType;

    // Java: abstract short getNodeType()
    fn getNodeType(&self) -> i16;

    // Java: abstract String getNodeTypeName()
    fn getNodeTypeName(&self) -> &str;

    // Java: abstract Node detach()
    fn detach(&mut self) -> Box<dyn Node>;

    // Java: abstract List<? extends Node> selectNodes(String var1)
    fn selectNodes(&self, var1: &str) -> Vec<Box<dyn Node>>;

    // Java: abstract Object selectObject(String var1)
    fn selectObject(&self, var1: &str) -> Option<Box<dyn Any>>;

    // Java: abstract List<? extends Node> selectNodes(String var1, String var2)
    fn selectNodes_sorted(&self, var1: &str, var2: &str) -> Vec<Box<dyn Node>>;

    // Java: abstract List<? extends Node> selectNodes(String var1, String var2, boolean var3)
    fn selectNodes_sorted_distinct(&self, var1: &str, var2: &str, var3: bool) -> Vec<Box<dyn Node>>;

    // Java: abstract Node selectSingleNode(String var1)
    fn selectSingleNode(&self, var1: &str) -> Option<Box<dyn Node>>;

    // Java: abstract String valueOf(String var1)
    fn valueOf(&self, var1: &str) -> String;

    // Java: abstract Number numberValueOf(String var1)
    fn numberValueOf(&self, var1: &str) -> Option<f64>;

    // Java: abstract boolean matches(String var1)
    fn matches(&self, var1: &str) -> bool;

    // Java: abstract XPath createXPath(String var1)
    fn createXPath(&self, var1: &str) -> Box<dyn XPath>;

    // Java: abstract Node asXPathResult(Element var1)
    fn asXPathResult(&self, var1: &dyn Element) -> Box<dyn Node>;

    // Java: abstract void accept(Visitor var1)
    fn accept(&self, var1: &dyn Visitor);

    // Java: abstract Object clone()
    fn clone_node(&self) -> Box<dyn Node>;
}
