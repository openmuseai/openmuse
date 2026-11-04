// Java: import cn.wps.io.dom.DocumentType
// Java: import cn.wps.io.dom.Element
// Java: import cn.wps.io.dom.NodeType

// Java: public class AbstractDocumentType extends AbstractNode implements DocumentType
pub struct AbstractDocumentType {
}

// Java: public class AbstractDocumentType extends AbstractNode implements DocumentType
impl AbstractDocumentType {
    // Java: AbstractDocumentType()
    pub fn new() -> Self {
      // Java: super();
      todo!();
    }

    // Java: NodeType getNodeTypeEnum()
    pub fn getNodeTypeEnum(&mut self) -> Option<super::super::NodeType::NodeType> {
      // Java: return NodeType.DOCUMENT_TYPE_NODE;
      todo!();
    }

    // Java: String getName()
    pub fn getName(&mut self) -> String {
      // Java: return this.getElementName();
      todo!();
    }

    // Java: void setName(String name)
    pub fn setName(&mut self) {
      // Java: this.setElementName(name);
      todo!();
    }

    // Java: String getPath(Element context)
    pub fn getPath(&mut self) -> String {
      // Java: return "";
      todo!();
    }

    // Java: String getUniquePath(Element context)
    pub fn getUniquePath(&mut self) -> String {
      // Java: return "";
      todo!();
    }

    // Java: void toString(StringBuilder builder)
    pub fn toString(&mut self) {
      // Java: super.toString(builder);
      todo!();
      // Java: builder.append(" [DocumentType: ");
      todo!();
      // Java: this.asXML(builder);
      todo!();
      // Java: builder.append(']');
      todo!();
    }

    // Java: String asXML()
    pub fn asXML(&mut self) -> String {
      // Java: StringBuilder builder = new StringBuilder();
      todo!();
      // Java: this.asXML(builder);
      todo!();
      // Java: return builder.toString();
      todo!();
    }

    // Java: void asXML(StringBuilder builder)
    pub fn asXML_2(&mut self) {
      // Java: builder.append("<!DOCTYPE ");
      todo!();
      // Java: builder.append(this.getElementName());
      todo!();
      // Java: boolean hasPublicID = false;
      todo!();
      // Java: String publicID = this.getPublicID();
      todo!();
      // Java: if (publicID != null && publicID.length() > 0)
      todo!();
      {
         // Java: builder.append(" PUBLIC ");
      todo!();
         // Java: builder.append('"');
      todo!();
         // Java: builder.append(publicID);
      todo!();
         // Java: builder.append('"');
      todo!();
         // Java: hasPublicID = true;
      todo!();
      }
      // Java: String systemID = this.getSystemID();
      todo!();
      // Java: if (systemID != null && systemID.length() > 0)
      todo!();
      {
         // Java: if (!hasPublicID)
      todo!();
         {
            // Java: builder.append(" SYSTEM");
      todo!();
         }
         // Java: builder.append(" \"");
      todo!();
         // Java: builder.append(systemID);
      todo!();
         // Java: builder.append('"');
      todo!();
      }
      // Java: builder.append('>');
      todo!();
    }

}
