// Java: import java.io.Externalizable
// TODO_RUST_CODE use ???; // java.io.Externalizable
// Java: import java.io.IOException
// TODO_RUST_CODE use ???; // java.io.IOException
// Java: import java.io.ObjectInput
// TODO_RUST_CODE use ???; // java.io.ObjectInput
// Java: import java.io.ObjectOutput
// TODO_RUST_CODE use ???; // java.io.ObjectOutput
// Java: import java.util.ArrayList
// TODO_RUST_CODE use ???; // java.util.ArrayList
// Java: import java.util.HashMap
// TODO_RUST_CODE use ???; // java.util.HashMap
// Java: import java.util.Iterator
// TODO_RUST_CODE use ???; // java.util.Iterator
// Java: import java.util.List
// TODO_RUST_CODE use ???; // java.util.List
// Java: import java.util.Map
// TODO_RUST_CODE use ???; // java.util.Map
// Java: import org.dom4j.Namespace
// use crate::org::dom4j::Namespace;
// Java: import org.dom4j.QName
// use crate::org::dom4j::QName;
// Java: import org.xml.sax.Attributes
// use kfc_kfc_alpha::cn::wps::kfc::html::reader::lexical::token::Attributes;
// Java: import org.xml.sax.ContentHandler
// use io_wps_io::io::xwpf::element_handler::ContentHandler;
// Java: import org.xml.sax.DTDHandler
// use crate::org::xml::sax::DTDHandler;
// Java: import org.xml.sax.SAXException
// use crate::org::xml::sax::SAXException;
// Java: import org.xml.sax.ext.DeclHandler
// use crate::org::xml::sax::ext::DeclHandler;
// Java: import org.xml.sax.ext.LexicalHandler
// use crate::org::xml::sax::ext::LexicalHandler;
// Java: import org.xml.sax.helpers.AttributesImpl
// use io_wps_io::cn::wps::io::opt::AttributesImpl;
// Java: import org.xml.sax.helpers.DefaultHandler
// use crate::org::xml::sax::helpers::DefaultHandler;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;
// Java field: byte STRING
pub(crate) const STRING: i8 = 0;
// Java field: byte OBJECT
pub(crate) const OBJECT: i8 = 0;
// Java field: byte NULL
pub(crate) const NULL: i8 = 0;
// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID_2: i64 = 0;
// Java field: byte PROCESSING_INSTRUCTION
pub(crate) const PROCESSING_INSTRUCTION: i8 = 0;
// Java field: byte START_PREFIX_MAPPING
pub(crate) const START_PREFIX_MAPPING: i8 = 0;
// Java field: byte END_PREFIX_MAPPING
pub(crate) const END_PREFIX_MAPPING: i8 = 0;
// Java field: byte START_DOCUMENT
pub(crate) const START_DOCUMENT: i8 = 0;
// Java field: byte END_DOCUMENT
pub(crate) const END_DOCUMENT: i8 = 0;
// Java field: byte START_ELEMENT
pub(crate) const START_ELEMENT: i8 = 0;
// Java field: byte END_ELEMENT
pub(crate) const END_ELEMENT: i8 = 0;
// Java field: byte CHARACTERS
pub(crate) const CHARACTERS: i8 = 0;
// Java field: byte START_DTD
pub(crate) const START_DTD: i8 = 0;
// Java field: byte END_DTD
pub(crate) const END_DTD: i8 = 0;
// Java field: byte START_ENTITY
pub(crate) const START_ENTITY: i8 = 0;
// Java field: byte END_ENTITY
pub(crate) const END_ENTITY: i8 = 0;
// Java field: byte START_CDATA
pub(crate) const START_CDATA: i8 = 0;
// Java field: byte END_CDATA
pub(crate) const END_CDATA: i8 = 0;
// Java field: byte COMMENT
pub(crate) const COMMENT: i8 = 0;
// Java field: byte ELEMENT_DECL
pub(crate) const ELEMENT_DECL: i8 = 0;
// Java field: byte ATTRIBUTE_DECL
pub(crate) const ATTRIBUTE_DECL: i8 = 0;
// Java field: byte INTERNAL_ENTITY_DECL
pub(crate) const INTERNAL_ENTITY_DECL: i8 = 0;
// Java field: byte EXTERNAL_ENTITY_DECL
pub(crate) const EXTERNAL_ENTITY_DECL: i8 = 0;

// Java: public class SAXEventRecorder extends DefaultHandler implements LexicalHandler, DeclHandler, DTDHandler, Externalizable
pub struct SAXEventRecorder {
    // Java field: List<SAXEvent> events
    events: i32,
    // Java field: Map prefixMappings
    prefixMappings: i32,
    // Java field: String XMLNS
    // Java field: String EMPTY_STRING
}

pub(crate) const XMLNS: &str = "";
pub(crate) const EMPTY_STRING: &str = "";

// Java: public class SAXEvent
pub struct SAXEvent {
    // Java field: byte event
    pub(crate) event: i8,
    // Java field: List parms
    pub(crate) parms: i32,
}

// Java: public class SAXEventRecorder extends DefaultHandler implements LexicalHandler, DeclHandler, DTDHandler, Externalizable
impl SAXEventRecorder {
    // Java: SAXEventRecorder()
    pub fn new() -> Self {
      // Java: super();
      Self { events: 0, prefixMappings: 0 }
    }

    // Java: void replay(ContentHandler handler)
    pub fn replay(&self, _handler: i32) {
      // Java: for(SAXEvent saxEvent : this.events)
        // TODO_RUST_CODE
      {
         // Java: switch (saxEvent.event)
            // TODO_RUST_CODE
         {
            // Java: case 1:
            // TODO_RUST_CODE
               // Java: handler.processingInstruction((String)saxEvent.getParm(0), (String)saxEvent.getParm(1));
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case 2:
            // TODO_RUST_CODE
               // Java: handler.startPrefixMapping((String)saxEvent.getParm(0), (String)saxEvent.getParm(1));
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case 3:
            // TODO_RUST_CODE
               // Java: handler.endPrefixMapping((String)saxEvent.getParm(0));
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case 4:
            // TODO_RUST_CODE
               // Java: handler.startDocument();
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case 5:
            // TODO_RUST_CODE
               // Java: handler.endDocument();
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case 6:
            // TODO_RUST_CODE
               // Java: AttributesImpl attributes = new AttributesImpl();
               // TODO_RUST_CODE
               // Java: List<String[]> attParmList = (List<String[]>)saxEvent.getParm(3);
               // TODO_RUST_CODE
               // Java: if (attParmList != null)
                // TODO_RUST_CODE
               {
                  // Java: for(String[] attParms : attParmList)
                    // TODO_RUST_CODE
                  {
                     // Java: attributes.addAttribute(attParms[0], attParms[1], attParms[2], attParms[3], attParms[4]);
                     // TODO_RUST_CODE
                  }
               }
               // Java: handler.startElement((String)saxEvent.getParm(0), (String)saxEvent.getParm(1), (String)saxEvent.getParm(2), attributes);
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case 7:
            // TODO_RUST_CODE
               // Java: handler.endElement((String)saxEvent.getParm(0), (String)saxEvent.getParm(1), (String)saxEvent.getParm(2));
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case 8:
            // TODO_RUST_CODE
               // Java: char[] chars = (char[])saxEvent.getParm(0);
               // TODO_RUST_CODE
               // Java: int start = (Integer)saxEvent.getParm(1);
               // TODO_RUST_CODE
               // Java: int end = (Integer)saxEvent.getParm(2);
               // TODO_RUST_CODE
               // Java: handler.characters(chars, start, end);
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case 9:
            // TODO_RUST_CODE
               // Java: ((LexicalHandler)handler).startDTD((String)saxEvent.getParm(0), (String)saxEvent.getParm(1), (String)saxEvent.getParm(2));
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case 10:
            // TODO_RUST_CODE
               // Java: ((LexicalHandler)handler).endDTD();
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case 11:
            // TODO_RUST_CODE
               // Java: ((LexicalHandler)handler).startEntity((String)saxEvent.getParm(0));
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case 12:
            // TODO_RUST_CODE
               // Java: ((LexicalHandler)handler).endEntity((String)saxEvent.getParm(0));
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case 13:
            // TODO_RUST_CODE
               // Java: ((LexicalHandler)handler).startCDATA();
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case 14:
            // TODO_RUST_CODE
               // Java: ((LexicalHandler)handler).endCDATA();
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case 15:
            // TODO_RUST_CODE
               // Java: char[] cchars = (char[])saxEvent.getParm(0);
               // TODO_RUST_CODE
               // Java: int cstart = (Integer)saxEvent.getParm(1);
               // TODO_RUST_CODE
               // Java: int cend = (Integer)saxEvent.getParm(2);
               // TODO_RUST_CODE
               // Java: ((LexicalHandler)handler).comment(cchars, cstart, cend);
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case 16:
            // TODO_RUST_CODE
               // Java: ((DeclHandler)handler).elementDecl((String)saxEvent.getParm(0), (String)saxEvent.getParm(1));
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case 17:
            // TODO_RUST_CODE
               // Java: ((DeclHandler)handler).attributeDecl((String)saxEvent.getParm(0), (String)saxEvent.getParm(1), (String)saxEvent.getParm(2), (String)saxEvent.getParm(3), (String)saxEvent.getParm(4));
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case 18:
            // TODO_RUST_CODE
               // Java: ((DeclHandler)handler).internalEntityDecl((String)saxEvent.getParm(0), (String)saxEvent.getParm(1));
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case 19:
            // TODO_RUST_CODE
               // Java: ((DeclHandler)handler).externalEntityDecl((String)saxEvent.getParm(0), (String)saxEvent.getParm(1), (String)saxEvent.getParm(2));
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: default:
            // TODO_RUST_CODE
               // Java: throw new SAXException("Unrecognized event: " + saxEvent.event);
               // TODO_RUST_CODE
         }
      }
    }

    // Java: void processingInstruction(String target, String data)
    pub fn processingInstruction(&self, _target: &str, _data: &str) {
      // Java: SAXEvent saxEvent = new SAXEvent((byte)1);
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(target);
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(data);
      // TODO_RUST_CODE
      // Java: this.events.add(saxEvent);
      // TODO_RUST_CODE
    }

    // Java: void startPrefixMapping(String prefix, String uri)
    pub fn startPrefixMapping(&self, _prefix: &str, _uri: &str) {
      // Java: SAXEvent saxEvent = new SAXEvent((byte)2);
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(prefix);
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(uri);
      // TODO_RUST_CODE
      // Java: this.events.add(saxEvent);
      // TODO_RUST_CODE
    }

    // Java: void endPrefixMapping(String prefix)
    pub fn endPrefixMapping(&self, _prefix: &str) {
      // Java: SAXEvent saxEvent = new SAXEvent((byte)3);
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(prefix);
      // TODO_RUST_CODE
      // Java: this.events.add(saxEvent);
      // TODO_RUST_CODE
    }

    // Java: void startDocument()
    pub fn startDocument(&self) {
      // Java: SAXEvent saxEvent = new SAXEvent((byte)4);
      // TODO_RUST_CODE
      // Java: this.events.add(saxEvent);
      // TODO_RUST_CODE
    }

    // Java: void endDocument()
    pub fn endDocument(&self) {
      // Java: SAXEvent saxEvent = new SAXEvent((byte)5);
      // TODO_RUST_CODE
      // Java: this.events.add(saxEvent);
      // TODO_RUST_CODE
    }

    // Java: void startElement(String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn startElement(&self, _namespaceURI: &str, _localName: &str, _qualifiedName: &str, _attributes: i32) {
      // Java: SAXEvent saxEvent = new SAXEvent((byte)6);
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(namespaceURI);
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(localName);
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(qualifiedName);
      // TODO_RUST_CODE
      // Java: QName qName = null;
      // TODO_RUST_CODE
      // Java: if (namespaceURI != null)
        // TODO_RUST_CODE
      {
         // Java: qName = new QName(localName, Namespace.get(namespaceURI));
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: qName = new QName(localName);
         // TODO_RUST_CODE
      }
      // Java: if (attributes != null && attributes.getLength() > 0)
        // TODO_RUST_CODE
      {
         // Java: List attParmList = new ArrayList(attributes.getLength());
         // TODO_RUST_CODE
         // Java: String[] attParms = null;
         // TODO_RUST_CODE
         // Java: for(int i = 0; i < attributes.getLength(); ++i)
            // TODO_RUST_CODE
         {
            // Java: String attLocalName = attributes.getLocalName(i);
            // TODO_RUST_CODE
            // Java: if (attLocalName.startsWith("xmlns"))
                // TODO_RUST_CODE
            {
               // Java: String prefix = null;
               // TODO_RUST_CODE
               // Java: if (attLocalName.length() > 5)
                    // TODO_RUST_CODE
               {
                  // Java: prefix = attLocalName.substring(6);
                  // TODO_RUST_CODE
               }
               // Java: else
                    // TODO_RUST_CODE
               {
                  // Java: prefix = "";
                  // TODO_RUST_CODE
               }
               // Java: SAXEvent prefixEvent = new SAXEvent((byte)2);
               // TODO_RUST_CODE
               // Java: prefixEvent.addParm(prefix);
               // TODO_RUST_CODE
               // Java: prefixEvent.addParm(attributes.getValue(i));
               // TODO_RUST_CODE
               // Java: this.events.add(prefixEvent);
               // TODO_RUST_CODE
               // Java: List prefixes = (List)this.prefixMappings.get(qName);
               // TODO_RUST_CODE
               // Java: if (prefixes == null)
                    // TODO_RUST_CODE
               {
                  // Java: prefixes = new ArrayList();
                  // TODO_RUST_CODE
                  // Java: this.prefixMappings.put(qName, prefixes);
                  // TODO_RUST_CODE
               }
               // Java: prefixes.add(prefix);
               // TODO_RUST_CODE
            }
            // Java: else
                // TODO_RUST_CODE
            {
               // Java: attParms = new String[]{attributes.getURI(i), attLocalName, attributes.getQName(i), attributes.getType(i), attributes.getValue(i)};
               // TODO_RUST_CODE
               // Java: attParmList.add(attParms);
               // TODO_RUST_CODE
            }
         }
         // Java: saxEvent.addParm(attParmList);
         // TODO_RUST_CODE
      }
      // Java: this.events.add(saxEvent);
      // TODO_RUST_CODE
    }

    // Java: void endElement(String namespaceURI, String localName, String qName)
    pub fn endElement(&self, _namespaceURI: &str, _localName: &str, _qName: &str) {
      // Java: SAXEvent saxEvent = new SAXEvent((byte)7);
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(namespaceURI);
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(localName);
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(qName);
      // TODO_RUST_CODE
      // Java: this.events.add(saxEvent);
      // TODO_RUST_CODE
      // Java: QName elementName = null;
      // TODO_RUST_CODE
      // Java: if (namespaceURI != null)
        // TODO_RUST_CODE
      {
         // Java: elementName = new QName(localName, Namespace.get(namespaceURI));
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: elementName = new QName(localName);
         // TODO_RUST_CODE
      }
      // Java: List prefixes = (List)this.prefixMappings.get(elementName);
      // TODO_RUST_CODE
      // Java: if (prefixes != null)
        // TODO_RUST_CODE
      {
         // Java: Iterator itr = prefixes.iterator();
         // TODO_RUST_CODE
         // Java: while(itr.hasNext())
            // TODO_RUST_CODE
         {
            // Java: SAXEvent prefixEvent = new SAXEvent((byte)3);
            // TODO_RUST_CODE
            // Java: prefixEvent.addParm(itr.next());
            // TODO_RUST_CODE
            // Java: this.events.add(prefixEvent);
            // TODO_RUST_CODE
         }
      }
    }

    // Java: void characters(char[] ch, int start, int end)
    pub fn characters(&self, _ch: &[char], _start: i32, _end: i32) {
      // Java: SAXEvent saxEvent = new SAXEvent((byte)8);
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(ch);
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(new Integer(start));
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(new Integer(end));
      // TODO_RUST_CODE
      // Java: this.events.add(saxEvent);
      // TODO_RUST_CODE
    }

    // Java: void startDTD(String name, String publicId, String systemId)
    pub fn startDTD(&self, _name: &str, _publicId: &str, _systemId: &str) {
      // Java: SAXEvent saxEvent = new SAXEvent((byte)9);
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(name);
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(publicId);
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(systemId);
      // TODO_RUST_CODE
      // Java: this.events.add(saxEvent);
      // TODO_RUST_CODE
    }

    // Java: void endDTD()
    pub fn endDTD(&self) {
      // Java: SAXEvent saxEvent = new SAXEvent((byte)10);
      // TODO_RUST_CODE
      // Java: this.events.add(saxEvent);
      // TODO_RUST_CODE
    }

    // Java: void startEntity(String name)
    pub fn startEntity(&self, _name: &str) {
      // Java: SAXEvent saxEvent = new SAXEvent((byte)11);
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(name);
      // TODO_RUST_CODE
      // Java: this.events.add(saxEvent);
      // TODO_RUST_CODE
    }

    // Java: void endEntity(String name)
    pub fn endEntity(&self, _name: &str) {
      // Java: SAXEvent saxEvent = new SAXEvent((byte)12);
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(name);
      // TODO_RUST_CODE
      // Java: this.events.add(saxEvent);
      // TODO_RUST_CODE
    }

    // Java: void startCDATA()
    pub fn startCDATA(&self) {
      // Java: SAXEvent saxEvent = new SAXEvent((byte)13);
      // TODO_RUST_CODE
      // Java: this.events.add(saxEvent);
      // TODO_RUST_CODE
    }

    // Java: void endCDATA()
    pub fn endCDATA(&self) {
      // Java: SAXEvent saxEvent = new SAXEvent((byte)14);
      // TODO_RUST_CODE
      // Java: this.events.add(saxEvent);
      // TODO_RUST_CODE
    }

    // Java: void comment(char[] ch, int start, int end)
    pub fn comment(&self, _ch: &[char], _start: i32, _end: i32) {
      // Java: SAXEvent saxEvent = new SAXEvent((byte)15);
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(ch);
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(new Integer(start));
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(new Integer(end));
      // TODO_RUST_CODE
      // Java: this.events.add(saxEvent);
      // TODO_RUST_CODE
    }

    // Java: void elementDecl(String name, String model)
    pub fn elementDecl(&self, _name: &str, _model: &str) {
      // Java: SAXEvent saxEvent = new SAXEvent((byte)16);
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(name);
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(model);
      // TODO_RUST_CODE
      // Java: this.events.add(saxEvent);
      // TODO_RUST_CODE
    }

    // Java: void attributeDecl(String eName, String aName, String type, String valueDefault, String value)
    pub fn attributeDecl(&self, _eName: &str, _aName: &str, _type: &str, _valueDefault: &str, _value: &str) {
      // Java: SAXEvent saxEvent = new SAXEvent((byte)17);
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(eName);
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(aName);
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(type);
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(valueDefault);
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(value);
      // TODO_RUST_CODE
      // Java: this.events.add(saxEvent);
      // TODO_RUST_CODE
    }

    // Java: void internalEntityDecl(String name, String value)
    pub fn internalEntityDecl(&self, _name: &str, _value: &str) {
      // Java: SAXEvent saxEvent = new SAXEvent((byte)18);
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(name);
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(value);
      // TODO_RUST_CODE
      // Java: this.events.add(saxEvent);
      // TODO_RUST_CODE
    }

    // Java: void externalEntityDecl(String name, String publicId, String sysId)
    pub fn externalEntityDecl(&self, _name: &str, _publicId: &str, _sysId: &str) {
      // Java: SAXEvent saxEvent = new SAXEvent((byte)19);
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(name);
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(publicId);
      // TODO_RUST_CODE
      // Java: saxEvent.addParm(sysId);
      // TODO_RUST_CODE
      // Java: this.events.add(saxEvent);
      // TODO_RUST_CODE
    }

    // Java: void writeExternal(ObjectOutput out)
    pub fn writeExternal(&self, _out: i32) {
      // Java: if (this.events == null)
        // TODO_RUST_CODE
      {
         // Java: out.writeByte(2);
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: out.writeByte(1);
         // TODO_RUST_CODE
         // Java: out.writeObject(this.events);
         // TODO_RUST_CODE
      }
    }

    // Java: void readExternal(ObjectInput in)
    pub fn readExternal(&self, _in: i32) {
      // Java: if (in.readByte() != 2)
        // TODO_RUST_CODE
      {
         // Java: this.events = (List)in.readObject();
         // TODO_RUST_CODE
      }
    }

}

// Java: public class SAXEvent
impl SAXEvent {
    // Java: SAXEvent()
    pub fn new() -> Self {
         // Java: super();
         Self { event: 0, parms: 0 }
    }

    // Java: void addParm(Object parm)
    pub fn addParm(&self, _parm: i32) {
         // Java: if (this.parms == null)
        // TODO_RUST_CODE
         {
            // Java: this.parms = new ArrayList(3);
            // TODO_RUST_CODE
         }
         // Java: this.parms.add(parm);
         // TODO_RUST_CODE
    }

    // Java: Object getParm(int index)
    pub fn getParm(&self, _index: i32) -> i32 {
         // Java: return this.parms != null && index < this.parms.size() ? this.parms.get(index) : null;
         0
    }

    // Java: void writeExternal(ObjectOutput out)
    pub fn writeExternal(&self, _out: i32) {
         // Java: out.writeByte(this.event);
         // TODO_RUST_CODE
         // Java: if (this.parms == null)
        // TODO_RUST_CODE
         {
            // Java: out.writeByte(2);
            // TODO_RUST_CODE
         }
         // Java: else
        // TODO_RUST_CODE
         {
            // Java: out.writeByte(1);
            // TODO_RUST_CODE
            // Java: out.writeObject(this.parms);
            // TODO_RUST_CODE
         }
    }

    // Java: void readExternal(ObjectInput in)
    pub fn readExternal(&self, _in: i32) {
         // Java: this.event = in.readByte();
         // TODO_RUST_CODE
         // Java: if (in.readByte() != 2)
        // TODO_RUST_CODE
         {
            // Java: this.parms = (List)in.readObject();
            // TODO_RUST_CODE
         }
    }

}

// impl LexicalHandler for SAXEventRecorder {
// }

// impl DeclHandler for SAXEventRecorder {
// }

// impl DTDHandler for SAXEventRecorder {
// }

// impl Externalizable for SAXEventRecorder {
// }
