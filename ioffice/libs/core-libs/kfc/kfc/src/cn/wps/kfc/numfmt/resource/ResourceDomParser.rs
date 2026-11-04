#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

// Java: import java.io.InputStream
// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import javax.xml.parsers.DocumentBuilder
// Java: import javax.xml.parsers.DocumentBuilderFactory
// Java: import javax.xml.parsers.FactoryConfigurationError
// Java: import javax.xml.parsers.ParserConfigurationException
// Java: import org.w3c.dom.Document
// Java: import org.w3c.dom.Element
// Java: import org.w3c.dom.Node
// Java: import org.w3c.dom.NodeList

use super::IResourceParser::IResourceParser;

// Java: public class ResourceDomParser implements IResourceParser
pub struct ResourceDomParser {
    // Java field: DocumentBuilder mDocBuilder
    mDocBuilder: i32,
    // Java field: Document mDocument
    mDocument: i32,
}

// Java: public class Factory
pub struct Factory {
}

// Java: public class ResourceDomParser implements IResourceParser
impl ResourceDomParser {
    // Java: ResourceDomParser()
    pub fn new() -> Self {
        // Java: super();
        // Java: DocumentBuilderFactory factory = DocumentBuilderFactory.newInstance();
        // Java: this.mDocBuilder = factory.newDocumentBuilder();
        ResourceDomParser { mDocBuilder: 0, mDocument: 0 }
    }

    // Java: void destroy()
    pub fn destroy(&mut self) {
        // Java: this.mDocument = null;
        todo!("ResourceDomParser::destroy")
    }

    // Java: boolean parse(InputStream input)
    pub fn parse(&mut self, _input: i32) -> bool {
        // Java: this.clear();
        // Java: try
        {
            // Java: this.mDocument = this.mDocBuilder.parse(input);
            // Java: return true;
        }
        // Java: catch (Exception var3)
        {
            // Java: return false;
        }
        todo!("ResourceDomParser::parse")
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: try
        {
            // Java: this.mDocBuilder.reset();
        }
        // Java: catch (UnsupportedOperationException var5)
        {
            // Java: DocumentBuilderFactory factory = DocumentBuilderFactory.newInstance();
            // Java: try
            {
                // Java: this.mDocBuilder = factory.newDocumentBuilder();
            }
            // Java: catch (Exception var4)
            {
            }
        }
        // Java: this.mDocument = null;
        todo!("ResourceDomParser::clear")
    }

    // Java: String getItem(String nodeName)
    pub fn getItem(&self, _node_name: &str) -> String {
        // Java: Node node = this.getNode(nodeName);
        // Java: return node.getFirstChild().getNodeValue();
        todo!("ResourceDomParser::getItem")
    }

    // Java: List<String> getArray(String nodeName)
    pub fn getArray(&self, _node_name: &str) -> Vec<String> {
        // Java: Node node = this.getNode(nodeName);
        // Java: NodeList list = node.getChildNodes();
        // Java: ArrayList<String> values = new ArrayList(list.getLength() / 2);
        // Java: for(int i = 0; i < list.getLength(); ++i)
        {
            // Java: Node item = list.item(i);
            // Java: if (item instanceof Element)
            {
                // Java: values.add(item.getFirstChild().getNodeValue());
            }
        }
        // Java: return values;
        todo!("ResourceDomParser::getArray")
    }

    // Java: Node getNode(String nodeName)
    pub fn getNode(&self, _node_name: &str) -> i32 {
        // Java: NodeList list = this.mDocument.getElementsByTagName(nodeName);
        // Java: return list.item(0);
        todo!("ResourceDomParser::getNode")
    }
}

// Java: public class Factory
impl Factory {
    // Java: Factory()
    pub fn new() -> Self {
        // Java: super();
        Factory {}
    }

    // Java: static ResourceDomParser newInstance()
    pub fn newInstance() -> Option<ResourceDomParser> {
        // Java: try
        {
            // Java: return new ResourceDomParser();
        }
        // Java: catch (Exception var1)
        {
            // Java: return null;
        }
        todo!("Factory::newInstance")
    }
}

impl IResourceParser for ResourceDomParser {
    fn parse(&mut self, _input: &mut dyn std::io::Read) -> bool {
        self.parse(0)
    }
    fn getItem(&self, key: &str) -> String {
        self.getItem(key)
    }
    fn getArray(&self, key: &str) -> Vec<String> {
        self.getArray(key)
    }
    fn destroy(&mut self) {
        self.destroy()
    }
}
