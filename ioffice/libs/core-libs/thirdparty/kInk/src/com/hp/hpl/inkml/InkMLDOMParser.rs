// Java: import cn.wps.base.io.XmlFeatureSecUtil
// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.FileInputStream
#[allow(unused_imports)]
use util::moffice::util::KSLog;
// Java: import cn.wps.moffice.util.KSLog
// Java: import java.io.IOException
// Java: import java.util.ArrayList
// Java: import java.util.Objects
// Java: import javax.xml.parsers.DocumentBuilder
// Java: import javax.xml.parsers.DocumentBuilderFactory
// Java: import javax.xml.parsers.ParserConfigurationException
// Java: import org.w3c.dom.Attr
// Java: import org.w3c.dom.Document
// Java: import org.w3c.dom.Element
// Java: import org.w3c.dom.NamedNodeMap
// Java: import org.w3c.dom.Node
// Java: import org.w3c.dom.NodeList
// Java: import org.xml.sax.ErrorHandler
// Java: import org.xml.sax.SAXException
// Java: import org.xml.sax.SAXParseException

use crate::com::hp::hpl::inkml::Ink::Ink;
use crate::com::hp::hpl::inkml::InkMLProcessor::InkMLProcessor;
use crate::com::hp::hpl::inkml::InkMLParser::InkMLParser;
use crate::com::hp::hpl::inkml::Context::Context;
use crate::com::hp::hpl::inkml::Trace::Trace;
use crate::com::hp::hpl::inkml::TraceView::TraceView;
use crate::com::hp::hpl::inkml::TraceGroup::TraceGroup;
use crate::com::hp::hpl::inkml::TraceFormat::TraceFormat;
use crate::com::hp::hpl::inkml::InkSource::InkSource;
use crate::com::hp::hpl::inkml::Canvas::Canvas;
use crate::com::hp::hpl::inkml::CanvasTransform::CanvasTransform;
use crate::com::hp::hpl::inkml::Timestamp::Timestamp;
use crate::com::hp::hpl::inkml::Mapping::Mapping;
use crate::com::hp::hpl::inkml::Brush::Brush;
#[allow(unused_imports)]
use crate::com::hp::hpl::inkml::IBrush::IBrush;
use crate::com::hp::hpl::inkml::Annotation::Annotation;
use crate::com::hp::hpl::inkml::AnnotationXML::AnnotationXML;
use crate::com::hp::hpl::inkml::BrushProperty::BrushProperty;
use crate::com::hp::hpl::inkml::Channel::Channel;
use crate::com::hp::hpl::inkml::ChannelProperties::ChannelProperties;
use crate::com::hp::hpl::inkml::ChannelProperty::ChannelProperty;
use crate::com::hp::hpl::inkml::Definitions::Definitions;

// Java field: String TAG
pub(crate) const TAG: &str = "InkMLDOMParser";

// Java: public class InkMLDOMParser implements InkMLParser, ErrorHandler
pub struct InkMLDOMParser {
    // Java field: Document inkmlDOMDocument
    inkmlDOMDocument: Option<i32>,
    // Java field: InkMLProcessor inkMLProcessor
    inkMLProcessor: Option<InkMLProcessor>,
    // Java field: String VALIDATION_FEATURE_ID
    _validation_feature_id: &'static str,
    // Java field: String SCHEMA_VALIDATION_FEATURE_ID
    _schema_validation_feature_id: &'static str,
    // Java field: String DEFAULT_PARSER_NAME
    _default_parser_name: &'static str,
    // Java field: String sINF
    _s_inf: &'static str,
    // Java field: String sInfinity
    _s_infinity: &'static str,
}

pub(crate) const VALIDATION_FEATURE_ID: &str = "http://xml.org/sax/features/validation";
pub(crate) const SCHEMA_VALIDATION_FEATURE_ID: &str = "http://apache.org/xml/features/validation/schema";
pub(crate) const DEFAULT_PARSER_NAME: &str = "dom.wrappers.Xerces";
pub(crate) const sINF: &str = "INF";
pub(crate) const sInfinity: &str = "Infinity";

// Java: public class InkMLDOMParser implements InkMLParser, ErrorHandler
impl InkMLDOMParser {
    // Java: InkMLDOMParser(InkMLProcessor processor)
    pub fn InkMLDOMParser(processor: InkMLProcessor) -> Self {
      // Java: super();
      // Java: this.inkMLProcessor = processor;
      Self {
          inkmlDOMDocument: None,
          inkMLProcessor: Some(processor),
          _validation_feature_id: VALIDATION_FEATURE_ID,
          _schema_validation_feature_id: SCHEMA_VALIDATION_FEATURE_ID,
          _default_parser_name: DEFAULT_PARSER_NAME,
          _s_inf: sINF,
          _s_infinity: sInfinity,
      }
    }

    // Java: void bindData(Ink ink)
    pub fn bindData(&self, _ink: &mut Ink) {
      // Java: KSLog.i(TAG, "\nTo bind the parsed data to InkML data objects.\n");
      // Java: if (null == this.inkmlDOMDocument)
      if self.inkmlDOMDocument.is_none()
      {
         // Java: KSLog.w(TAG, "No parsed data available for data binding.");
      }
      // Java: else
      else
      {
         // Java: Element rootElmnt = this.inkmlDOMDocument.getDocumentElement();
         // Java: if ("ink".equalsIgnoreCase(rootElmnt.getLocalName()))
         {
            // Java: ink.setDocID(rootElmnt.getAttribute("documentID"));
            // Java: NodeList list = rootElmnt.getChildNodes();
            // Java: for(int i = 0; i < list.getLength(); ++i)
            {
               // Java: Node node = list.item(i);
               // Java: if (node instanceof Element)
               {
                  // Java: Element element = (Element)node;
                  // Java: this.addToInkElementList(element, ink);
               }
            }
         }
         // Java: else if ("inkMLFragment".equalsIgnoreCase(rootElmnt.getLocalName()))
         {
            // Java: NodeList list = rootElmnt.getChildNodes();
            // Java: for(int i = 0; i < list.getLength(); ++i)
            {
               // Java: Node node = list.item(i);
               // Java: if (node instanceof Element)
               {
                  // Java: Element element = (Element)node;
                  // Java: this.addToInkElementList(element, ink);
               }
            }
         }
      }
    }

    // Java: void addToInkElementList(Element element, Ink ink)
    pub fn addToInkElementList(&self, _element: i32, _ink: &mut Ink) {
      // Java: String tagName = element.getLocalName();
      // Java: if (tagName.equals("definitions"))
      {
         // Java: Definitions definitions = ink.getDefinitions();
         // Java: NodeList childElements = element.getChildNodes();
         // Java: this.addToDefinitions(childElements, definitions);
      }
      // Java: else if (tagName.equals("context"))
      {
         // Java: Definitions defs = ink.getDefinitions();
         // Java: Context context = this.getContext(element, defs);
         // Java: if ("".equals(context.getContextRef()))
         {
            // Java: context.setContextRef("#" + ink.getCurrentContext().getId());
         }
         // Java: context.deriveContextualChildrenData(ink.getDefinitions(), ink.getCurrentContext());
         // Java: ArrayList<Ink.contextChangeStatus> ctxChanges = ink.getContextChanges(context);
         // Java: if (0 == ctxChanges.size())
         {
            // Java: return;
         }
         // Java: if ("".equals(context.getId()))
         {
            // Java: String ctxId;
            // Java: for(ctxId = InkMLIDGenerator.getNextIDForContext(); defs.contains(ctxId); ctxId = InkMLIDGenerator.getNextIDForContext())
            {
            }
            // Java: context.setId(ctxId);
         }
         // Java: defs.addToDirectChildrenMap((InkElement)context);
         // Java: this.inkMLProcessor.notifyContextChanged(context, ctxChanges);
         // Java: ink.setCurrentContext(context);
      }
      // Java: else if (tagName.equals("trace"))
      {
         // Java: Trace trace = this.getTrace(element);
         // Java: ink.addTrace(trace);
         // Java: this.inkMLProcessor.notifyTraceReceived(trace);
         // Java: if (!"".equals(trace.getId()))
         {
            // Java: ink.getDefinitions().addToIndirectChildrenMap((InkElement)trace);
         }
      }
      // Java: else if (tagName.equals("traceGroup"))
      {
         // Java: TraceGroup traceGroup = this.getTraceGroup(element, ink.getDefinitions());
         // Java: traceGroup.resolveAssociatedContext(ink.getCurrentContext(), ink.getDefinitions());
         // Java: ink.addTraceGroup(traceGroup);
         // Java: this.inkMLProcessor.notifyTraceGroupReceived(traceGroup);
         // Java: if (!"".equals(traceGroup.getId()))
         {
            // Java: ink.getDefinitions().addToIndirectChildrenMap((InkElement)traceGroup);
         }
         // Java: ArrayList<TraceDataElement> children = traceGroup.getTraceDataList();
         // Java: if (null != children)
         {
            // Java: for(int i = 0; i < children.size(); ++i)
            {
               // Java: TraceDataElement child = (TraceDataElement)children.get(i);
               // Java: if (!"".equals(child.getId()))
               {
                  // Java: ink.getDefinitions().addToIndirectChildrenMap((InkElement)child);
               }
            }
         }
      }
      // Java: else if (tagName.equals("traceView"))
      {
         // Java: TraceView traceView = this.getTraceView(element, ink.getDefinitions());
         // Java: traceView.setAssociatedContext(ink.getCurrentContext());
         // Java: ink.addTraceView(traceView);
         // Java: this.inkMLProcessor.notifyTraceViewReceived(traceView);
         // Java: if (!"".equals(traceView.getId()))
         {
            // Java: ink.getDefinitions().addToDirectChildrenMap((InkElement)traceView);
         }
      }
      // Java: else if (tagName.equals("annotation"))
      {
         // Java: Annotation annotation = this.getAnnotation(element);
         // Java: ink.addAnnotation(annotation);
      }
      // Java: else
      {
         // Java: if (!tagName.equals("annotationXML"))
         {
            // Java: throw new InkMLException("The Element " + tagName + " is not a valid Child Element for InkML <Ink> Element");
         }
         // Java: AnnotationXML aXml = this.getAnnotationXML(element);
         // Java: ink.addAnnotationXML(aXml);
      }
    }

    // Java: AnnotationXML getAnnotationXML(Element element)
    pub fn getAnnotationXML(&self, _element: i32) -> AnnotationXML {
      // Java: AnnotationXML aXml = new AnnotationXML();
      let axml = AnnotationXML::AnnotationXML();
      // Java: NamedNodeMap attributesMap = element.getAttributes();
      // Java: int length = attributesMap.getLength();
      // Java: for(int index = 0; index < length; ++index)
      {
         // Java: Attr attribute = (Attr)attributesMap.item(index);
         // Java: String attributeName = attribute.getName();
         // Java: if ("type".equals(attributeName))
         {
            // Java: aXml.setType(attribute.getValue());
         }
         // Java: else if ("encoding".equals(attributeName))
         {
            // Java: aXml.setEncoding(attribute.getValue());
         }
         // Java: else if ("href".equals(attributeName))
         {
            // Java: String hRef = attribute.getValue();
            // Java: try
            {
               // Java: aXml.setHref(new URI(hRef));
               // Java: if (!"".equals(hRef))
               {
                  // Java: aXml.addAllToPropertyMap(hRef);
               }
            }
            // Java: catch (InkMLException e)
            {
               // Java: throw new InkMLException("Problem in binding 'href' attribute of AnnotationXML data.\nReason: " + e.getMessage());
            }
         }
         // Java: else
         {
            // Java: aXml.addToOtherAttributesMap(attributeName, attribute.getValue());
         }
      }
      // Java: KSLog.w(TAG, "annotationXML received: " + element.toString());
      // Java: NodeList list = element.getChildNodes();
      // Java: int nChildren = list.getLength();
      // Java: if (nChildren > 0)
      {
         // Java: for(int i = 0; i < nChildren; ++i)
         {
            // Java: Node node = list.item(i);
            // Java: if (node instanceof Element)
            {
               // Java: Element childElement = (Element)node;
               // Java: String tagName = childElement.getLocalName();
               // Java: String value = childElement.getFirstChild().getNodeValue();
               // Java: aXml.addToPropertyElementsMap(tagName, value);
               // Java: KSLog.w(TAG, "The property with name = " + tagName + " is added to the propertyElementsMap.");
            }
         }
      }
      // Java: return aXml;
      axml
    }

    // Java: Annotation getAnnotation(Element element)
    pub fn getAnnotation(&self, _element: i32) -> Annotation {
      // Java: Annotation annotation = new Annotation();
      let annotation = Annotation::Annotation();
      // Java: NamedNodeMap attributesMap = element.getAttributes();
      // Java: int length = attributesMap.getLength();
      // Java: for(int index = 0; index < length; ++index)
      {
         // Java: Attr attribute = (Attr)attributesMap.item(index);
         // Java: String attributeName = attribute.getName();
         // Java: if ("type".equals(attributeName))
         {
            // Java: annotation.setType(attribute.getValue());
         }
         // Java: else if ("encoding".equals(attributeName))
         {
            // Java: annotation.setEncoding(attribute.getValue());
         }
         // Java: else
         {
            // Java: annotation.addToOtherAttributesMap(attributeName, attribute.getValue());
         }
      }
      // Java: Node valueNode = element.getFirstChild();
      // Java: if (null != valueNode)
      {
         // Java: annotation.setAnnotationTextValue(valueNode.getNodeValue());
      }
      // Java: return annotation;
      annotation
    }

    // Java: TraceView getTraceView(Element element, Definitions definitions)
    pub fn getTraceView(&self, _element: i32, _definitions: &Definitions) -> TraceView {
      // Java: TraceView traceView = new TraceView();
      let trace_view = TraceView::TraceView();
      // Java: String id = this.getID(element);
      // Java: if (!"".equals(id))
      {
         // Java: traceView.setId(id);
      }
      // Java: String contextRef = element.getAttribute("contextRef");
      // Java: if (!"".equals(contextRef))
      {
         // Java: traceView.setContextRef(contextRef);
      }
      // Java: traceView.resolveContext(definitions);
      // Java: String traceDataRef = element.getAttribute("traceDataRef");
      // Java: if (!"".equals(traceDataRef))
      {
         // Java: traceView.setTraceDataRef(traceDataRef);
         // Java: traceView.setFromAttribute(element.getAttribute("from"));
         // Java: traceView.setToAttribute(element.getAttribute("to"));
         // Java: traceView.setSelectedTree(definitions);
      }
      // Java: else
      {
         // Java: NodeList list = element.getElementsByTagNameNS(element.getNamespaceURI(), "traceView");
         // Java: int nData = list.getLength();
         // Java: if (0 != nData)
         {
            // Java: traceView.initChildViewList();
            // Java: for(int i = 0; i < nData; ++i)
            {
               // Java: Element traceViewElement = (Element)list.item(i);
               // Java: TraceView childTV = this.getTraceView(traceViewElement, definitions);
               // Java: traceView.addToChildTraceViewList(childTV);
            }
            // Java: traceView.processChildren(definitions);
         }
      }
      // Java: return traceView;
      trace_view
    }

    // Java: TraceGroup getTraceGroup(Element element, Definitions definitions)
    pub fn getTraceGroup(&self, _element: i32, _definitions: &Definitions) -> TraceGroup {
      // Java: TraceGroup traceGroup = new TraceGroup();
      let trace_group = TraceGroup::TraceGroup();
      // Java: String id = this.getID(element);
      // Java: String contextref = element.getAttribute("contextRef");
      // Java: String brushRef = element.getAttribute("brushRef");
      // Java: if (!"".equals(id))
      {
         // Java: traceGroup.setId(id);
      }
      // Java: if (!"".equals(contextref))
      {
         // Java: traceGroup.setContextRef(contextref);
      }
      // Java: if (!"".equals(brushRef))
      {
         // Java: traceGroup.setBrushRef(brushRef);
      }
      // Java: NodeList list = element.getChildNodes();
      // Java: for(int i = 0; i < list.getLength(); ++i)
      {
         // Java: Node node = list.item(i);
         // Java: if (node instanceof Element)
         {
            // Java: Element child = (Element)node;
            // Java: String tagName = child.getLocalName();
            // Java: if ("trace".equals(tagName))
            {
               // Java: Trace trace = this.getTrace(child);
               // Java: trace.setParentTraceGroup(traceGroup);
               // Java: traceGroup.addToTraceData(trace);
            }
            // Java: else if ("traceGroup".equals(tagName))
            {
               // Java: TraceGroup childTG = this.getTraceGroup(child, definitions);
               // Java: childTG.setParentTraceGroup(traceGroup);
               // Java: traceGroup.addToTraceData(childTG);
            }
            // Java: else if ("traceView".equals(tagName))
            {
               // Java: TraceView childTV = this.getTraceView(child, definitions);
               // Java: childTV.setParentTraceGroup(traceGroup);
               // Java: traceGroup.addToTraceData(childTV);
            }
            // Java: else if ("annotation".equals(tagName))
            {
               // Java: Annotation annotation = this.getAnnotation(element);
               // Java: traceGroup.addAnnotation(annotation);
            }
            // Java: else if ("annotationXML".equals(tagName))
            {
               // Java: AnnotationXML annotationXml = this.getAnnotationXML(element);
               // Java: traceGroup.addAnnotation(annotationXml);
            }
         }
      }
      // Java: Ink localInk = this.inkMLProcessor.getInk();
      // Java: Context context = localInk.getCurrentContext();
      // Java: traceGroup.resolveAssociatedContext(context, definitions);
      // Java: return traceGroup;
      trace_group
    }

    // Java: Trace getTrace(Element element)
    pub fn getTrace(&self, _element: i32) -> Trace {
      // Java: Trace trace = new Trace();
      let trace = Trace::Trace();
      // Java: NamedNodeMap attrMap = element.getAttributes();
      // Java: int length = attrMap.getLength();
      // Java: for(int i = 0; i < length; ++i)
      {
         // Java: Node attr = attrMap.item(i);
         // Java: trace.setAttribute(attr.getLocalName(), attr.getNodeValue());
      }
      // Java: String traceText = "";
      // Java: NodeList nl = element.getChildNodes();
      // Java: for(int i = 0; i < nl.getLength(); ++i)
      {
         // Java: if (nl.item(i).getNodeType() == 3)
         {
            // Java: traceText = traceText + nl.item(i).getNodeValue();
         }
      }
      // Java: Ink ink = this.inkMLProcessor.getInk();
      // Java: Context currCtx = ink.getCurrentContext();
      // Java: Definitions defs = ink.getDefinitions();
      // Java: trace.resolveAssociatedContext(currCtx, defs);
      // Java: try
      {
         // Java: trace.processTraceElement(traceText, currCtx, defs);
      }
      // Java: catch (NumberFormatException e)
      {
         // Java: KSLog.e("Trace", e.getMessage());
      }
      // Java: return trace;
      trace
    }

    // Java: void addToDefinitions(NodeList elements, Definitions definitions)
    pub fn addToDefinitions(&self, _elements: i32, _definitions: &mut Definitions) {
      // Java: for(int i = 0; i < elements.getLength(); ++i)
      {
         // Java: Node node = elements.item(i);
         // Java: if (node instanceof Element)
         {
            // Java: Element element = (Element)node;
            // Java: String tagName = element.getLocalName();
            // Java: String id = this.getID(element);
            // Java: if ("".equals(id))
            {
               // Java: id = element.getAttribute("xml:id");
               // Java: if ("".equals(id))
               {
                  // Java: throw new InkMLException("Elements within a <definitions> block must have an 'id' attribute. A " + tagName + " element do not have 'id' attribute.");
               }
            }
            // Java: InkElement inkElement = null;
            // Java: if ("brush".equalsIgnoreCase(tagName))
            {
               // Java: inkElement = this.getBrush(element);
            }
            // Java: else if ("traceFormat".equalsIgnoreCase(tagName))
            {
               // Java: inkElement = this.getTraceFormat(element, definitions);
            }
            // Java: else if ("context".equalsIgnoreCase(tagName))
            {
               // Java: Context context = this.getContext(element, definitions);
               // Java: context.resolveImplicitReferenceWithDefaultContext();
               // Java: inkElement = context;
            }
            // Java: else if ("inkSource".equalsIgnoreCase(tagName))
            {
               // Java: inkElement = this.getInkSource(element, definitions);
            }
            // Java: else if ("trace".equalsIgnoreCase(tagName))
            {
               // Java: inkElement = this.getTrace(element);
            }
            // Java: else if ("traceGroup".equalsIgnoreCase(tagName))
            {
               // Java: inkElement = this.getTraceGroup(element, definitions);
            }
            // Java: else if ("traceView".equalsIgnoreCase(tagName))
            {
               // Java: inkElement = this.getTraceView(element, definitions);
            }
            // Java: else if ("canvas".equalsIgnoreCase(tagName))
            {
               // Java: inkElement = this.getCanvas(element, definitions);
            }
            // Java: else if ("canvasTransform".equalsIgnoreCase(tagName))
            {
               // Java: inkElement = this.getCanvasTransform(element);
            }
            // Java: else if ("timestamp".equalsIgnoreCase(tagName))
            {
               // Java: inkElement = this.getTimestamp(element);
            }
            // Java: else
            {
               // Java: if (!"mapping".equalsIgnoreCase(tagName))
               {
                  // Java: throw new InkMLException("Parse Error: The element " + tagName + "should not be a child to <definitions> element.\n");
               }
               // Java: inkElement = this.getMapping(element);
            }
            // Java: definitions.addToDirectChildrenMap(inkElement);
         }
      }
    }

    // Java: Mapping getMapping(Element element)
    pub fn getMapping(&self, _element: i32) -> Mapping {
      // Java: Mapping mapping = new Mapping();
      let mapping = Mapping::Mapping();
      // Java: String type = element.getAttribute("type");
      // Java: if ("".equals(type))
      {
         // Java: type = "unknown";
      }
      // Java: KSLog.w(TAG, "mapping type=" + type);
      // Java: if (!"identity".equalsIgnoreCase(type) && !"unknown".equalsIgnoreCase(type))
      {
         // Java: throw new InkMLException("Feature not implemented: 'Mapping' with 'type' attribute other than identity and unknown");
      }
      // Java: else
      {
         // Java: mapping.setType(type);
         // Java: String id = this.getID(element);
         // Java: if (!id.equals(""))
         {
            // Java: mapping.setId(id);
         }
         // Java: return mapping;
         return mapping;
      }
    }

    // Java: Timestamp getTimestamp(Element element)
    pub fn getTimestamp(&self, _element: i32) -> Timestamp {
      // Java: throw new InkMLException("Feature not implemented. Mapping of type other than identity or unknown");
      Timestamp::Timestamp()
    }

    // Java: CanvasTransform getCanvasTransform(Element element)
    pub fn getCanvasTransform(&self, _element: i32) -> CanvasTransform {
      // Java: CanvasTransformLoader canvasTransform = new CanvasTransformLoader();
      let canvas_transform = CanvasTransform::CanvasTransform();
      // Java: NamedNodeMap attrMap = element.getAttributes();
      // Java: int length = attrMap.getLength();
      // Java: for(int i = 0; i < length; ++i)
      {
         // Java: Node attr = attrMap.item(i);
         // Java: canvasTransform.setAttribute(attr.getLocalName(), attr.getNodeValue());
      }
      // Java: NodeList list = element.getElementsByTagNameNS(element.getNamespaceURI(), "mapping");
      // Java: int nMappingChildren = list.getLength();
      // Java: if (nMappingChildren == 2)
      {
         // Java: canvasTransform.setForwardMapping(this.getMapping((Element)list.item(0)));
         // Java: canvasTransform.setReverseMapping(this.getMapping((Element)list.item(1)));
      }
      // Java: else if (nMappingChildren == 1)
      {
         // Java: canvasTransform.setForwardMapping(this.getMapping((Element)list.item(0)));
      }
      // Java: return canvasTransform;
      canvas_transform
    }

    // Java: Canvas getCanvas(Element element, Definitions definitions)
    pub fn getCanvas(&self, _element: i32, _definitions: &Definitions) -> Canvas {
      // Java: Canvas canvas = new Canvas();
      let canvas = Canvas::Canvas();
      // Java: NamedNodeMap attrMap = element.getAttributes();
      // Java: int length = attrMap.getLength();
      // Java: for(int i = 0; i < length; ++i)
      {
         // Java: Node attr = attrMap.item(i);
         // Java: canvas.setAttribute(attr.getLocalName(), attr.getNodeValue());
      }
      // Java: NodeList list = element.getElementsByTagNameNS(element.getNamespaceURI(), "traceFormat");
      // Java: if (list.getLength() != 0)
      {
         // Java: canvas.setTraceFormat(this.getTraceFormat((Element)list.item(0), definitions));
      }
      // Java: return canvas;
      canvas
    }

    // Java: InkSource getInkSource(Element element, Definitions definitions)
    pub fn getInkSource(&self, _element: i32, _definitions: &Definitions) -> InkSource {
      // Java: InkSource inkSrc = new InkSource();
      let ink_src = InkSource::InkSource();
      // Java: NamedNodeMap attrMap = element.getAttributes();
      // Java: int length = attrMap.getLength();
      // Java: for(int i = 0; i < length; ++i)
      {
         // Java: Node attr = attrMap.item(i);
         // Java: inkSrc.setAttribute(attr.getLocalName(), attr.getNodeValue());
      }
      // Java: NodeList list = element.getElementsByTagNameNS(element.getNamespaceURI(), "traceFormat");
      // Java: if (list.getLength() != 0)
      {
         // Java: inkSrc.setTraceFormat(this.getTraceFormat((Element)list.item(0), definitions));
      }
      // Java: list = element.getElementsByTagNameNS(element.getNamespaceURI(), "sampleRate");
      // Java: if (list.getLength() != 0)
      {
         // Java: inkSrc.setSampleRate(this.getSampleRate((Element)list.item(0), inkSrc));
      }
      // Java: list = element.getElementsByTagNameNS(element.getNamespaceURI(), "latency");
      // Java: if (list.getLength() != 0)
      {
         // Java: inkSrc.setLatency(this.getLatency((Element)list.item(0), inkSrc));
      }
      // Java: list = element.getElementsByTagNameNS(element.getNamespaceURI(), "activeArea");
      // Java: if (list.getLength() != 0)
      {
         // Java: inkSrc.setActiveArea(this.getActiveArea((Element)list.item(0), inkSrc));
      }
      // Java: list = element.getElementsByTagNameNS(element.getNamespaceURI(), "srcProperty");
      // Java: for(int i = 0; i < list.getLength(); ++i)
      {
         // Java: inkSrc.addSourceProperty(this.getSourceProperty((Element)list.item(i), inkSrc));
      }
      // Java: list = element.getElementsByTagNameNS(element.getNamespaceURI(), "channelProperties");
      // Java: if (list.getLength() != 0)
      {
         // Java: inkSrc.setChannelProperties(this.getChannelProperties((Element)list.item(0), inkSrc));
      }
      // Java: return inkSrc;
      ink_src
    }

    // Java: ChannelProperties getChannelProperties(Element element, InkSource inkSrc)
    pub fn getChannelProperties(&self, _element: i32, _ink_src: &InkSource) -> ChannelProperties {
      // Java: NodeList list = element.getElementsByTagNameNS(element.getNamespaceURI(), "channelProperty");
      // Java: int nChnProperty = list.getLength();
      // Java: if (0 == nChnProperty)
      {
         // Java: return null;
      }
      // Java: else
      {
         // Java: ChannelProperties chnProps = new ChannelProperties();
         let chn_props = ChannelProperties::ChannelProperties();
         // Java: for(int i = 0; i < nChnProperty; ++i)
         {
            // Java: ChannelProperty chnProp = this.getChannelProperty((Element)list.item(i), chnProps);
            // Java: chnProps.addChannelProperty(chnProp);
         }
         // Java: return chnProps;
         return chn_props;
      }
    }

    // Java: ChannelProperty getChannelProperty(Element element, ChannelProperties chnProps)
    pub fn getChannelProperty(&self, _element: i32, _chn_props: &ChannelProperties) -> ChannelProperty {
      // Java: ChannelProperty chnProp = null;
      // Java: String channel = element.getAttribute("channel");
      // Java: String name = element.getAttribute("name");
      // Java: String value = element.getAttribute("value");
      // Java: String units = element.getAttribute("units");
      // Java: if (value.equals("INF"))
      {
         // Java: value = "Infinity";
      }
      // Java: if ("".equals(units))
      {
         // Java: chnProp = new ChannelProperty(channel, name, new Double(value));
      }
      // Java: else
      {
         // Java: if ((channel.equals("X") || channel.equals("Y")) && units.equals("cm"))
         {
            // Java: units = "1/cm";
         }
         // Java: chnProp = new ChannelProperty(channel, name, new Double(value), units);
      }
      // Java: return chnProp;
      ChannelProperty::ChannelProperty()
    }

    // Java: Context getContext(Element element, Definitions definitions)
    pub fn getContext(&self, _element: i32, _definitions: &Definitions) -> Context {
      // Java: Context context = new Context();
      let context = Context::Context();
      // Java: NamedNodeMap attrMap = element.getAttributes();
      // Java: int length = attrMap.getLength();
      // Java: for(int i = 0; i < length; ++i)
      {
         // Java: Node attr = attrMap.item(i);
         // Java: context.setAttribute(attr.getLocalName(), attr.getNodeValue());
      }
      // Java: NodeList list = element.getChildNodes();
      // Java: for(int i = 0; i < list.getLength(); ++i)
      {
         // Java: Node node = list.item(i);
         // Java: if (node instanceof Element)
         {
            // Java: Element child = (Element)node;
            // Java: this.addToContextChildrenList(child, context, definitions);
         }
      }
      // Java: return context;
      context
    }

    // Java: void addToContextChildrenList(Element ctxChild, Context context, Definitions definitions)
    pub fn addToContextChildrenList(&self, _ctx_child: i32, _context: &mut Context, _definitions: &Definitions) {
      // Java: String tagName = ctxChild.getLocalName();
      // Java: if (tagName.equals("brush"))
      {
         // Java: IBrush brush = this.getBrush(ctxChild);
         // Java: context.addToContextElementList(brush);
      }
      // Java: else if (tagName.equals("traceFormat"))
      {
         // Java: TraceFormat tf = this.getTraceFormat(ctxChild, definitions);
         // Java: context.addToContextElementList(tf);
      }
      // Java: else if (tagName.equals("canvas"))
      {
         // Java: Canvas canvas = this.getCanvas(ctxChild, definitions);
         // Java: context.addToContextElementList(canvas);
      }
      // Java: else if (tagName.equals("canvasTransform"))
      {
         // Java: CanvasTransform canvasTransform = this.getCanvasTransform(ctxChild);
         // Java: context.addToContextElementList(canvasTransform);
      }
      // Java: else if (tagName.equals("inkSource"))
      {
         // Java: InkSource inkSrc = this.getInkSource(ctxChild, definitions);
         // Java: context.addToContextElementList(inkSrc);
      }
      // Java: else if (tagName.equals("timeStamp"))
      {
         // Java: Timestamp timeStamp = this.getTimestamp(ctxChild);
         // Java: context.addToContextElementList(timeStamp);
      }
    }

    // Java: IBrush getBrush(Element element)
    pub fn getBrush(&self, _element: i32) -> Brush {
      // Java: String id = this.getID(element);
      // Java: Brush brush = new Brush(id);
      let brush = Brush::Brush_with_id(Some(""));
      // Java: String brushRef = element.getAttribute("brushRef");
      // Java: if (!"".equals(brushRef))
      {
         // Java: brush.setBrushRef(brushRef);
      }
      // Java: NodeList list = element.getChildNodes();
      // Java: for(int i = 0; i < list.getLength(); ++i)
      {
         // Java: Node node = list.item(i);
         // Java: if (node instanceof Element)
         {
            // Java: Element childElement = (Element)node;
            // Java: String tagName = childElement.getLocalName();
            // Java: if ("annotationXML".equalsIgnoreCase(tagName))
            {
               // Java: AnnotationXML aXml = this.getAnnotationXML(childElement);
               // Java: brush.setAnnotationXML(aXml);
            }
            // Java: else if ("annotation".equalsIgnoreCase(tagName))
            {
               // Java: Annotation annotation = this.getAnnotation(childElement);
               // Java: brush.setAnnotation(annotation);
            }
            // Java: else if ("brushProperty".equalsIgnoreCase(tagName))
            {
               // Java: BrushProperty brushProperty = this.getBrushProperty(childElement);
               // Java: brush.setBrushProperty(brushProperty.getName(), brushProperty.getValue(), brushProperty.getUnits());
            }
         }
      }
      // Java: return brush;
      brush
    }

    // Java: BrushProperty getBrushProperty(Element element)
    pub fn getBrushProperty(&self, _element: i32) -> BrushProperty {
      // Java: if (element.hasAttribute("name") && element.hasAttribute("value"))
      {
         // Java: String str1 = element.getAttribute("name");
         // Java: String str2 = element.getAttribute("value");
         // Java: BrushProperty localBrushProperty = new BrushProperty(str1, str2);
         let local_brush_property = BrushProperty::BrushProperty_nv("", "");
         // Java: if (element.hasAttribute("units"))
         {
            // Java: localBrushProperty.setUnits(element.getAttribute("units"));
         }
         // Java: return localBrushProperty;
         return local_brush_property;
      }
      // Java: else
      {
         // Java: throw new InkMLException("Brush property requires name and value attributes");
         panic!("Brush property requires name and value attributes");
      }
    }

    // Java: TraceFormat getTraceFormat(Element element, Definitions definitions)
    pub fn getTraceFormat(&self, _element: i32, _definitions: &Definitions) -> TraceFormat {
      // Java: TraceFormat traceFormat = new TraceFormat();
      let trace_format = TraceFormat::TraceFormat();
      // Java: String id = this.getID(element);
      // Java: if (!"".equals(id))
      {
         // Java: traceFormat.setId(this.getID(element));
         // Java: definitions.addToIndirectChildrenMap((InkElement)traceFormat);
      }
      // Java: String href = element.getAttribute("href");
      // Java: if (!"".equals(href))
      {
         // Java: TraceFormat refferedTF = definitions.getTraceFormatRefElement(href);
         // Java: traceFormat.setHref(href);
         // Java: traceFormat.setChannelList(refferedTF.getChannelList());
      }
      // Java: NodeList list = element.getElementsByTagNameNS(element.getNamespaceURI(), "channel");
      // Java: int nChannel = list.getLength();
      // Java: for(int i = 0; i < nChannel; ++i)
      {
         // Java: Element channelElmnt = (Element)list.item(i);
         // Java: Channel ch = this.getChannel(channelElmnt);
         // Java: traceFormat.addChannel(ch);
      }
      // Java: return traceFormat;
      trace_format
    }

    // Java: Channel getChannel(Element channelElement)
    pub fn getChannel(&self, _channel_element: i32) -> Channel {
      // Java: Channel channel = null;
      // Java: String chnName = channelElement.getAttribute("name");
      // Java: if ("".equals(chnName))
      {
         // Java: throw new InkMLException("Channel element must have value for 'name' attribute");
      }
      // Java: else
      {
         // Java: channel = new Channel(chnName);
         let channel = Channel::Channel_name("");
         // Java: if ("intermittentChannels".equalsIgnoreCase(channelElement.getParentNode().getLocalName()))
         {
            // Java: channel.setIntermittent(true);
         }
         // Java: NamedNodeMap attrMap = channelElement.getAttributes();
         // Java: int length = attrMap.getLength();
         // Java: for(int i = 0; i < length; ++i)
         {
            // Java: Node attr = attrMap.item(i);
            // Java: channel.setAttribute(attr.getLocalName(), attr.getNodeValue());
         }
         // Java: return channel;
         return channel;
      }
    }

    // Java: void parseInkMLFile(String inkmlFileName)
    pub fn parseInkMLFile_impl(&mut self, _inkml_file_name: &str) {
      // Java: this.inkmlDOMDocument = this.createDocument(inkmlFileName);
      // Java: this.inkMLProcessor.beginInkSession();
      // Java: this.bindData(this.inkMLProcessor.getInk());
    }

    // Java: Document createDocument(String input)
    pub fn createDocument(&self, _input: &str) -> Option<i32> {
      // Java: Document document = null;
      // Java: File file = new File(input);
      // Java: DocumentBuilderFactory builderFactory = DocumentBuilderFactory.newInstance();
      // Java: try
      {
         // Java: XmlFeatureSecUtil.setFeatureXml(builderFactory);
      }
      // Java: catch (Exception var10)
      {
      }
      // Java: builderFactory.setNamespaceAware(true);
      // Java: try
      {
         // Java: DocumentBuilder builder = builderFactory.newDocumentBuilder();
         // Java: builder.setErrorHandler(this);
         // Java: document = builder.parse(new FileInputStream(file));
      }
      // Java: catch (ParserConfigurationException e)
      {
         // Java: KSLog.e(TAG, "ParserConfigurationException", e);
      }
      // Java: catch (SAXException e)
      {
         // Java: KSLog.e(TAG, "SAXException", e);
      }
      // Java: catch (IOException e)
      {
         // Java: KSLog.e(TAG, "IOException", e);
      }
      // Java: return document;
      None
    }

    // Java: void parseInkMLString(String inkmlStr)
    pub fn parseInkMLString_impl(&mut self, _inkml_str: &str) {
      // Java: this.inkmlDOMDocument = this.createDocument(inkmlStr);
      // Java: this.bindData(this.inkMLProcessor.getInk());
    }

    // Java: void printError(String messageType, SAXParseException exception)
    pub fn printError(&self, _message_type: &str, _exception: i32) {
      // Java: System.err.print("[");
      // Java: System.err.print(messageType);
      // Java: System.err.print("] ");
      // Java: String systemId = exception.getSystemId();
      // Java: if (systemId != null)
      {
         // Java: int index = systemId.lastIndexOf(47);
         // Java: if (index != -1)
         {
            // Java: systemId = systemId.substring(index + 1);
         }
         // Java: System.err.print(systemId);
      }
      // Java: System.err.print(':');
      // Java: System.err.print(exception.getLineNumber());
      // Java: System.err.print(':');
      // Java: System.err.print(exception.getColumnNumber());
      // Java: System.err.print(": ");
      // Java: System.err.print(exception.getMessage());
      // Java: System.err.println();
      // Java: System.err.flush();
    }

    // Java: String getID(Element element)
    pub fn getID(&self, _element: i32) -> String {
      // Java: if (element.hasAttributeNS("http://www.w3.org/XML/1998/namespace", "id"))
      {
         // Java: return element.getAttributeNS("http://www.w3.org/XML/1998/namespace", "id");
      }
      // Java: else if (element.hasAttribute("xml:id"))
      {
         // Java: return element.getAttribute("xml:id");
      }
      // Java: else
      {
         // Java: KSLog.w(TAG, "Resolving ID for element without namespace.");
         // Java: return element.getAttribute("id");
         return String::new();
      }
    }

    // Java: void error(SAXParseException exception)
    pub fn error(&self, _exception: i32) {
      // Java: this.printError("Error", exception);
      // Java: System.exit(-1);
    }

    // Java: void fatalError(SAXParseException exception)
    pub fn fatalError(&self, _exception: i32) {
      // Java: this.printError("FatalError", exception);
      // Java: System.exit(-1);
    }

    // Java: void warning(SAXParseException exception)
    pub fn warning(&self, _exception: i32) {
      // Java: this.printError("Warning", exception);
    }

}

impl InkMLParser for InkMLDOMParser {
    fn parseInkMLFile(&mut self, var1: &str) {
        self.parseInkMLFile_impl(var1);
    }
    fn parseInkMLString(&mut self, var1: &str) {
        self.parseInkMLString_impl(var1);
    }
}
