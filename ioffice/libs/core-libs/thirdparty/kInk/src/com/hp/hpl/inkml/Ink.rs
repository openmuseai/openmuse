#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import android.graphics.RectF
use util::graphics::RectF::RectF;
// Java: import cn.wps.graphics.SizeF
use util::graphics::SizeF::SizeF;
// Java: import cn.wps.moffice.util.KSLog
use util::cn::wps::moffice::util::KSLog::KSLog;
// Java: import com.hp.hpl.inkml.impl.HitService
// Java: import com.hp.hpl.inkml.impl.InkPathCache
use crate::com::hp::hpl::inkml::r#impl::InkPathCache::InkPathCache;
// Java: import com.hp.hpl.inkml.impl.InkTool
// Java: import com.hp.hpl.inkml.impl.TraceAdapter
use crate::com::hp::hpl::inkml::r#impl::TraceAdapter::TraceAdapter;
// Java: import java.io.IOException
// Java: import java.util.ArrayList
// Java: import java.util.Collection
// Java: import java.util.Iterator
// Java: import java.util.LinkedHashMap
// Java: import java.util.LinkedList
// Java: import java.util.List

use std::collections::{HashMap, VecDeque};
use super::InkMLSerializer::InkMLWriter;
use super::Context::Context;
use super::Definitions::Definitions;
use super::Annotation::Annotation;
use super::AnnotationXML::AnnotationXML;
use super::Trace::Trace;
use super::TraceGroup::TraceGroup;

// Java: public class Ink implements Cloneable
pub struct Ink {
    // Java field: String TAG
    // Java field: String INKML_NAMESPACE
    // Java field: String XSD_NAMESPACE
    // Java field: LinkedList<TraceDataElement> traceDataList
    pub(crate) traceDataList: VecDeque<Box<dyn InkTraceData>>,
    // Java field: Definitions definitions
    pub(crate) definitions: Definitions,
    // Java field: Context currentContext
    pub(crate) currentContext: Context,
    // Java field: Annotation annotation
    pub(crate) annotation: Option<Annotation>,
    // Java field: AnnotationXML annotationXML
    pub(crate) annotationXML: Option<AnnotationXML>,
    // Java field: String docID
    pub(crate) docID: String,
    // Java field: float maxStrokeWidth
    pub(crate) maxStrokeWidth: f32,
    // Java field: float maxStrokeHeight
    pub(crate) maxStrokeHeight: f32,
    // Java field: RectF inkRect
    pub(crate) inkRect: RectF,
    // Java field: ArrayList<TraceAdapter> traces
    pub(crate) traces: Vec<TraceAdapter>,
    // Java field: boolean tracesValid
    pub(crate) tracesValid: bool,
    // Java field: InkPathCache pathCache
    pub(crate) pathCache: Option<InkPathCache>,
    // Java field: float BRUSH_SCALE_2_POINT
}

pub trait InkTraceData {
    fn getInkElementType(&self) -> String;
    fn toInkML_defs(&self, defs: &Definitions) -> String;
    fn writeXML(&self, writer: &mut InkMLWriter);
}

// Java: public enum contextChangeStatus
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum ContextChangeStatus {
    // Java: contextChangeStatus()
    isBrushChanged,
    isTraceFormatChanged,
    isInkSourceChanged,
    isCanvasChanged,
    isCanvasTransformChanged,
    isTimestampChanged,
    NONE,
}

// Java: public class Ink implements Cloneable
impl Ink {
    // Java: void buildTraces()
    pub fn buildTraces(&mut self) {
      // Java: if (!this.tracesValid)
      if !self.tracesValid {
         // Java: synchronized(this)
         {
            // Java: if (!this.tracesValid)
            {
               // Java: this.tracesValid = true;
               self.tracesValid = true;
               // Java: Iterator<?> iter = null;
               // Java: try
               {
                  // Java: iter = this.getTraceIterator();
               }
               // Java: catch (InkMLException e)
               {
                  // Java: e.printStackTrace();
               }
               // Java: if (null != iter)
               {
                  // Java: TraceAdapter traceImpl;
                  // Java: for(boolean bFirst = true; iter.hasNext(); this.traces.add(traceImpl))
                  {
                     // Java: Trace trace = (Trace)iter.next();
                     // Java: traceImpl = TraceAdapter.fromInkML(trace, this.getCurrentContext());
                     // Java: this.maxStrokeWidth = Math.max(this.maxStrokeWidth, traceImpl.getBrush().getWidth());
                     // Java: this.maxStrokeHeight = Math.max(this.maxStrokeHeight, traceImpl.getBrush().getHeight());
                     // Java: RectF traceRect = traceImpl.getRect();
                     // Java: if (traceRect != null)
                     {
                        // Java: if (bFirst)
                        {
                           // Java: this.inkRect.set(traceRect);
                           // Java: bFirst = false;
                        }
                        // Java: else
                        {
                           // Java: this.inkRect = InkTool.unionRect(this.inkRect, traceImpl.getRect());
                        }
                     }
                  }
               }
            }
         }
      }
    }

    // Java: void onEdit()
    pub fn onEdit(&mut self) {
      // Java: this.tracesValid = false;
      self.tracesValid = false;
      // Java: this.traces.clear();
      self.traces.clear();
      // Java: if (this.inkRect != null)
      {
         // Java: this.inkRect.setEmpty();
         self.inkRect = RectF::default();
      }
    }

    // Java: ArrayList<TraceAdapter> getTraces()
    pub fn getTraces(&self) -> &Vec<TraceAdapter> {
      // Java: return this.traces;
      &self.traces
    }

    // Java: Ink()
    pub fn Ink() -> Self {
      // Java: super();
      Self {
          traceDataList: VecDeque::new(),
          definitions: Definitions::Definitions(),
          currentContext: Context::getDefaultContext(),
          annotation: None,
          annotationXML: None,
          docID: String::new(),
          maxStrokeWidth: -1.0,
          maxStrokeHeight: 1.0,
          inkRect: RectF::default(),
          traces: Vec::new(),
          tracesValid: false,
          pathCache: None,
      }
    }

    // Java: ArrayList<contextChangeStatus> getContextChanges(Context context)
    pub fn getContextChanges(&self, context: Option<&Context>) -> Vec<ContextChangeStatus> {
      // Java: ArrayList<contextChangeStatus> ctxStatus = new ArrayList();
      let mut ctx_status = Vec::new();
      // Java: if (null == context)
      if context.is_none() {
         // Java: return ctxStatus;
         return ctx_status;
      }
      // Java: else
      {
         // Java: IBrush brush = context.getBrush();
         // Java: TraceFormat traceFormat = context.getTraceFormat();
         // Java: InkSource inkSource = context.getInkSource();
         // Java: Canvas canvas = context.getCanvas();
         // Java: CanvasTransform canvasTransform = context.getCanvasTransform();
         // Java: Timestamp timestamp = context.getTimestamp();
         // Java: if (brush != null && !this.currentContext.getBrush().equals(brush))
         {
            // Java: ctxStatus.add(Ink.contextChangeStatus.isBrushChanged);
         }
         // Java: if (traceFormat != null && !this.currentContext.getTraceFormat().equals2(traceFormat))
         {
            // Java: ctxStatus.add(Ink.contextChangeStatus.isTraceFormatChanged);
         }
         // Java: if (inkSource != null && !this.currentContext.getInkSource().equals2(inkSource))
         {
            // Java: ctxStatus.add(Ink.contextChangeStatus.isInkSourceChanged);
         }
         // Java: if (canvas != null && !this.currentContext.getCanvas().equals2(canvas))
         {
            // Java: ctxStatus.add(Ink.contextChangeStatus.isCanvasChanged);
         }
         // Java: if (canvasTransform != null && !this.currentContext.getCanvasTransform().equals2(canvasTransform))
         {
            // Java: ctxStatus.add(Ink.contextChangeStatus.isCanvasTransformChanged);
         }
         // Java: if (timestamp != null && !this.currentContext.getTimestamp().equals(timestamp))
         {
            // Java: ctxStatus.add(Ink.contextChangeStatus.isTimestampChanged);
         }
         // Java: return ctxStatus;
         return ctx_status;
      }
    }

    // Java: void addToTraceDataList(TraceDataElement traceData)
    pub fn addToTraceDataList(&mut self, traceData: Box<dyn InkTraceData>) {
      // Java: this.traceDataList.add(traceData);
      self.traceDataList.push_back(traceData);
    }

    // Java: Iterator getTraceIterator()
    pub fn getTraceIterator(&self) -> Vec<&Trace> {
      // Java: ArrayList<Trace> traceList = new ArrayList();
      let trace_list: Vec<&Trace> = Vec::new();
      // Java: if (this.traceDataList != null)
      {
         // Java: for(InkElement object : this.traceDataList)
         {
            // Java: String inkElmntType = object.getInkElementType();
            // Java: if ("Trace".equals(inkElmntType))
            {
               // Java: traceList.add((Trace)object);
            }
            // Java: if ("TraceGroup".equals(inkElmntType))
            {
               // Java: traceList.addAll(((TraceGroup)object).getTraceList());
            }
            // Java: if ("TraceView".equals(inkElmntType))
            {
               // Java: traceList.addAll(((TraceView)object).getTraceList());
            }
         }
      }
      // Java: return traceList.iterator();
      trace_list
    }

    // Java: Definitions getDefinitions()
    pub fn getDefinitions(&self) -> &Definitions {
      // Java: return this.definitions;
      &self.definitions
    }

    // Java: String getDocID()
    pub fn getDocID(&self) -> &str {
      // Java: return this.docID;
      &self.docID
    }

    // Java: void addTrace(Trace trace)
    pub fn addTrace(&mut self, trace: Box<dyn InkTraceData>) {
      // Java: this.addToTraceDataList(trace);
      self.traceDataList.push_back(trace);
    }

    // Java: void addTraceGroup(TraceGroup traceGroup)
    pub fn addTraceGroup(&mut self, traceGroup: Box<dyn InkTraceData>) {
      // Java: this.addToTraceDataList(traceGroup);
      self.traceDataList.push_back(traceGroup);
    }

    // Java: void addTraceView(TraceView traceView)
    pub fn addTraceView(&mut self, traceView: Box<dyn InkTraceData>) {
      // Java: this.addToTraceDataList(traceView);
      self.traceDataList.push_back(traceView);
    }

    // Java: void clearTraceDataList()
    pub fn clearTraceDataList(&mut self) {
      // Java: this.traceDataList.clear();
      self.traceDataList.clear();
    }

    // Java: boolean containsInTraceDataList(TraceDataElement traceData)
    pub fn containsInTraceDataList(&self, _traceData: &dyn InkTraceData) -> bool {
      // Java: return this.traceDataList.contains(traceData);
      false
    }

    // Java: boolean containsAllInTraceDataList(Collection<TraceDataElement> traceData)
    pub fn containsAllInTraceDataList(&self, _traceData: &[Box<dyn InkTraceData>]) -> bool {
      // Java: return this.traceDataList.containsAll(traceData);
      false
    }

    // Java: boolean isTraceDataListEmpty()
    pub fn isTraceDataListEmpty(&self) -> bool {
      // Java: return this.traceDataList.isEmpty();
      self.traceDataList.is_empty()
    }

    // Java: InkElement removeFromTraceDataList(int index)
    pub fn removeFromTraceDataList_index(&mut self, index: usize) -> Option<Box<dyn InkTraceData>> {
      // Java: return (InkElement)this.traceDataList.remove(index);
      self.traceDataList.remove(index)
    }

    // Java: InkElement removeLastFromTraceDataList()
    pub fn removeLastFromTraceDataList(&mut self) -> Option<Box<dyn InkTraceData>> {
      // Java: return (InkElement)this.traceDataList.removeLast();
      self.traceDataList.pop_back()
    }

    // Java: boolean removeFromTraceDataList(TraceDataElement traceData)
    pub fn removeFromTraceDataList(&mut self, _traceData: &dyn InkTraceData) -> bool {
      // Java: return this.traceDataList.remove(traceData);
      false
    }

    // Java: boolean removeAllTraceData(Collection<TraceDataElement> traceDatas)
    pub fn removeAllTraceData(&mut self, _traceDatas: &[Box<dyn InkTraceData>]) -> bool {
      // Java: return this.traceDataList.removeAll(traceDatas);
      false
    }

    // Java: boolean retainAllTraceData(Collection<TraceDataElement> traceDatas)
    pub fn retainAllTraceData(&mut self, _traceDatas: &[Box<dyn InkTraceData>]) -> bool {
      // Java: return this.traceDataList.retainAll(traceDatas);
      false
    }

    // Java: Context getCurrentContext()
    pub fn getCurrentContext(&self) -> &Context {
      // Java: return this.currentContext;
      &self.currentContext
    }

    // Java: void setDocID(String id)
    pub fn setDocID(&mut self, id: &str) {
      // Java: this.docID = id;
      self.docID = id.to_string();
    }

    // Java: void addAnnotation(Annotation annotation)
    pub fn addAnnotation(&mut self, annotation: Annotation) {
      // Java: this.annotation = annotation;
      self.annotation = Some(annotation);
    }

    // Java: void addAnnotationXML(AnnotationXML aXml)
    pub fn addAnnotationXML(&mut self, aXml: AnnotationXML) {
      // Java: this.annotationXML = aXml;
      self.annotationXML = Some(aXml);
    }

    // Java: LinkedList<TraceDataElement> getTraceDataList()
    pub fn getTraceDataList(&self) -> &VecDeque<Box<dyn InkTraceData>> {
      // Java: return this.traceDataList;
      &self.traceDataList
    }

    // Java: Annotation getAnnotation()
    pub fn getAnnotation(&self) -> Option<&Annotation> {
      // Java: return this.annotation;
      self.annotation.as_ref()
    }

    // Java: void setAnnotation(Annotation annotation)
    pub fn setAnnotation(&mut self, annotation: Option<Annotation>) {
      // Java: this.annotation = annotation;
      self.annotation = annotation;
    }

    // Java: AnnotationXML getAnnotationXML()
    pub fn getAnnotationXML(&self) -> Option<&AnnotationXML> {
      // Java: return this.annotationXML;
      self.annotationXML.as_ref()
    }

    // Java: void setAnnotationXML(AnnotationXML annotationXML)
    pub fn setAnnotationXML(&mut self, annotationXML: Option<AnnotationXML>) {
      // Java: this.annotationXML = annotationXML;
      self.annotationXML = annotationXML;
    }

    // Java: void setTraceDataList(LinkedList<TraceDataElement> traceDataList)
    pub fn setTraceDataList(&mut self, traceDataList: VecDeque<Box<dyn InkTraceData>>) {
      // Java: this.traceDataList = traceDataList;
      self.traceDataList = traceDataList;
    }

    // Java: void setCurrentContext(Context context)
    pub fn setCurrentContext(&mut self, context: Context) {
      // Java: this.currentContext = context;
      self.currentContext = context;
    }

    // Java: void writeXML(InkMLWriter writer)
    pub fn writeXML(&self, writer: &mut InkMLWriter) {
      // Java: if (writer == null)
      // Java: KSLog.w(TAG, "Ink:writeXML, InkMLWriter object not available (null)!!!");
      // Java: throw new InkMLException("Ink:writeXML, InkMLWriter object not available (null)!!!");
      // Java: else
      {
         // Java: LinkedHashMap<String, String> attrMap = new LinkedHashMap();
         let mut attr_map = HashMap::new();
         // Java: if (!"".equals(this.docID))
         if !self.docID.is_empty() {
            // Java: attrMap.put("documentID", this.docID);
            attr_map.insert("documentID".to_string(), self.docID.clone());
         }
         // Java: attrMap.put("xmlns", "http://www.w3.org/2003/InkML");
         attr_map.insert("xmlns".to_string(), "http://www.w3.org/2003/InkML".to_string());
         // Java: writer.writeStartTag("ink", attrMap);
         writer.writeStartTag("ink", Some(&attr_map));
         // Java: writer.incrementTagLevel();
         writer.incrementTagLevel();
         // Java: this.definitions.writeXML(writer);
         self.definitions.writeXML(writer);
         // Java: for(TraceDataElement data : this.traceDataList)
         for data in &self.traceDataList {
            // Java: data.writeXML(writer);
            data.writeXML(writer);
         }
         // Java: writer.decrementTagLevel();
         writer.decrementTagLevel();
         // Java: writer.writeEndTag("ink");
         writer.writeEndTag("ink");
      }
    }

    // Java: String toInkML()
    pub fn toInkML(&self) -> String {
      // Java: StringBuffer xml = new StringBuffer();
      let mut xml = String::new();
      // Java: LinkedHashMap<String, String> attrMap = new LinkedHashMap();
      // Java: if (null != this.docID && !"".equals(this.docID))
      if !self.docID.is_empty() {
         // Java: xml.append("<ink xmlns=\"http://www.w3.org/2003/InkML\" documentID=\"" + this.docID + "\">");
         xml.push_str(&format!("<ink xmlns=\"http://www.w3.org/2003/InkML\" documentID=\"{}\">", self.docID));
         // Java: attrMap.put("documentID", this.docID);
      }
      // Java: else
      else {
         // Java: xml.append("<ink xmlns=\"http://www.w3.org/2003/InkML\">");
         xml.push_str("<ink xmlns=\"http://www.w3.org/2003/InkML\">");
      }
      // Java: xml.append(this.definitions.toInkML());
      xml.push_str(&self.definitions.toInkML());
      // Java: for(TraceDataElement data : this.traceDataList)
      for data in &self.traceDataList {
         // Java: xml.append(data.toInkML(this.definitions));
         xml.push_str(&data.toInkML_defs(&self.definitions));
      }
      // Java: xml.append("</ink>");
      xml.push_str("</ink>");
      // Java: return xml.toString();
      xml
    }

    // Java: String toInkMLForPDFSign()
    pub fn toInkMLForPDFSign(&self) -> String {
      // Java: StringBuffer xml = new StringBuffer();
      let mut xml = String::new();
      // Java: xml.append("<ink xmlns=\"http://www.w3.org/2003/InkML\" documentID=\"" + this.docID + "\">");
      xml.push_str(&format!("<ink xmlns=\"http://www.w3.org/2003/InkML\" documentID=\"{}\">", self.docID));
      // Java: xml.append(this.definitions.toInkML());
      xml.push_str(&self.definitions.toInkML());
      // Java: for(TraceDataElement data : this.traceDataList)
      for data in &self.traceDataList {
         // Java: xml.append(data.toInkML(this.definitions));
         xml.push_str(&data.toInkML_defs(&self.definitions));
      }
      // Java: xml.append("</ink>");
      xml.push_str("</ink>");
      // Java: return xml.toString();
      xml
    }

    // Java: Ink clone()
    pub fn clone_ink(&self) -> Ink {
      // Java: return this.clone((List)null);
      self.clone_with_del(None)
    }

    // Java: Ink clone(List<Integer> delTraces)
    pub fn clone_with_del(&self, delTraces: Option<&Vec<i32>>) -> Ink {
      // Java: Ink ret = new Ink();
      let mut ret = Ink::Ink();
      // Java: ret.traceDataList = this.cloneTraceDataList(this.traceDataList, delTraces);
      // Java: if (null != this.definitions)
      {
         // Java: ret.definitions = this.definitions.clone();
         ret.definitions = self.definitions.clone_defs();
      }
      // Java: if (null != this.annotation)
      {
         // Java: ret.annotation = this.annotation.clone();
      }
      // Java: if (null != this.annotationXML)
      {
         // Java: ret.annotationXML = this.annotationXML.clone();
      }
      // Java: if (null != this.currentContext)
      {
         // Java: ret.currentContext = this.currentContext.clone();
      }
      // Java: if (null != this.docID)
      {
         // Java: ret.docID = new String(this.docID);
         ret.docID = self.docID.clone();
      }
      // Java: return ret;
      ret
    }

    // Java: LinkedList<TraceDataElement> cloneTraceDataList(LinkedList<TraceDataElement> src, List<Integer> delTraces)
    fn cloneTraceDataList(&self, src: &VecDeque<Box<dyn InkTraceData>>, delTraces: Option<&Vec<i32>>) -> VecDeque<Box<dyn InkTraceData>> {
      // Java: if (null == src)
      {
         // Java: return null;
      }
      // Java: else
      {
         // Java: LinkedList<TraceDataElement> ret = new LinkedList();
         let ret: VecDeque<Box<dyn InkTraceData>> = VecDeque::new();
         // Java: int size = src.size();
         let size = src.len();
         // Java: for(int i = 0; i < size; ++i)
         for i in 0..size {
            // Java: if (delTraces == null || !delTraces.contains(i))
            if delTraces.is_none() || !delTraces.unwrap().contains(&(i as i32)) {
               // Java: TraceDataElement e = (TraceDataElement)src.get(i);
               // Java: if (e instanceof TraceGroup)
               {
                  // Java: TraceGroup traceGroup = (TraceGroup)e;
                  // Java: ret.add(traceGroup.clone());
               }
               // Java: else if (e instanceof Trace)
               {
                  // Java: Trace trace = (Trace)e;
                  // Java: ret.add(trace.clone());
               }
               // Java: else if (e instanceof TraceView)
               {
                  // Java: TraceView traceView = (TraceView)e;
                  // Java: ret.add(traceView.clone());
               }
            }
         }
         // Java: return ret;
         ret
      }
    }

    // Java: void calOutRect(cn.wps.graphics.RectF outRectF)
    pub fn calOutRect(&mut self, outRectF: &mut RectF) {
      // Java: this.buildTraces();
      self.buildTraces();
      // Java: float expandX = this.maxStrokeWidth / 2.0F;
      let mut expand_x = self.maxStrokeWidth / 2.0;
      // Java: float expandY = this.maxStrokeHeight / 2.0F;
      let mut expand_y = self.maxStrokeHeight / 2.0;
      // Java: if (outRectF.width() != 0.0F && this.inkRect.width() != 0.0F)
      let out_w = outRectF.right - outRectF.left;
      let ink_w = self.inkRect.right - self.inkRect.left;
      if out_w != 0.0 && ink_w != 0.0 {
         // Java: expandX *= outRectF.width() / this.inkRect.width();
         expand_x *= out_w / ink_w;
      }
      // Java: if (outRectF.height() != 0.0F && this.inkRect.height() != 0.0F)
      let out_h = outRectF.bottom - outRectF.top;
      let ink_h = self.inkRect.bottom - self.inkRect.top;
      if out_h != 0.0 && ink_h != 0.0 {
         // Java: expandY *= outRectF.height() / this.inkRect.height();
         expand_y *= out_h / ink_h;
      }
      // Java: outRectF.left -= expandX;
      outRectF.left -= expand_x;
      // Java: outRectF.top -= expandY;
      outRectF.top -= expand_y;
      // Java: outRectF.right += expandX;
      outRectF.right += expand_x;
      // Java: outRectF.bottom += expandY;
      outRectF.bottom += expand_y;
    }

    // Java: void calOutRectPound(cn.wps.graphics.RectF outRectF)
    pub fn calOutRectPound(&mut self, outRectF: &mut RectF) {
      // Java: this.buildTraces();
      self.buildTraces();
      // Java: float expandX = this.maxStrokeWidth / 2.0F * 0.037795275F;
      let expand_x = self.maxStrokeWidth / 2.0 * 0.037795275;
      // Java: float expandY = this.maxStrokeHeight / 2.0F * 0.037795275F;
      let expand_y = self.maxStrokeHeight / 2.0 * 0.037795275;
      // Java: outRectF.left -= expandX;
      outRectF.left -= expand_x;
      // Java: outRectF.top -= expandY;
      outRectF.top -= expand_y;
      // Java: outRectF.right += expandX;
      outRectF.right += expand_x;
      // Java: outRectF.bottom += expandY;
      outRectF.bottom += expand_y;
    }

    // Java: void calInRectPound(cn.wps.graphics.RectF inRectF)
    pub fn calInRectPound(&mut self, inRectF: &mut RectF) {
      // Java: this.buildTraces();
      self.buildTraces();
      // Java: float expandX = this.maxStrokeWidth / 2.0F * 0.037795275F;
      let expand_x = self.maxStrokeWidth / 2.0 * 0.037795275;
      // Java: float expandY = this.maxStrokeHeight / 2.0F * 0.037795275F;
      let expand_y = self.maxStrokeHeight / 2.0 * 0.037795275;
      // Java: inRectF.left += expandX;
      inRectF.left += expand_x;
      // Java: inRectF.top += expandY;
      inRectF.top += expand_y;
      // Java: inRectF.right -= expandX;
      inRectF.right -= expand_x;
      // Java: inRectF.bottom -= expandY;
      inRectF.bottom -= expand_y;
    }

    // Java: RectF getInkRect()
    pub fn getInkRect(&self) -> &RectF {
      // Java: this.buildTraces();
      // Java: return this.inkRect;
      &self.inkRect
    }

    // Java: void setInkRect(RectF rect)
    pub fn setInkRect(&mut self, rect: RectF) {
      // Java: if (this.inkRect != null)
      {
         // Java: this.inkRect.set(rect);
         self.inkRect = rect;
      }
      // Java: else
      {
         // Java: this.inkRect = new RectF(rect);
      }
    }

    // Java: void setMaxStrokeWidth(float maxStrokeWidth)
    pub fn setMaxStrokeWidth(&mut self, maxStrokeWidth: f32) {
      // Java: this.maxStrokeWidth = maxStrokeWidth;
      self.maxStrokeWidth = maxStrokeWidth;
    }

    // Java: void setMaxStrokeHeight(float maxStrokeHeight)
    pub fn setMaxStrokeHeight(&mut self, maxStrokeHeight: f32) {
      // Java: this.maxStrokeHeight = maxStrokeHeight;
      self.maxStrokeHeight = maxStrokeHeight;
    }

    // Java: int hitLine(cn.wps.graphics.RectF srcRect, float pointA_x, float pointA_y, float pointB_x, float pointB_y, float deviation)
    pub fn hitLine(&mut self, srcRect: &RectF, mut pointA_x: f32, mut pointA_y: f32, mut pointB_x: f32, mut pointB_y: f32, deviation: f32) -> i32 {
      // Java: cn.wps.graphics.RectF outRect = new cn.wps.graphics.RectF(srcRect);
      let mut out_rect = RectF { left: srcRect.left, top: srcRect.top, right: srcRect.right, bottom: srcRect.bottom };
      // Java: this.calOutRectPound(outRect);
      self.calOutRectPound(&mut out_rect);
      // Java: if ((!(pointA_x < outRect.left) || !(pointB_x < outRect.left)) && (!(pointA_x > outRect.right) || !(pointB_x > outRect.right)) && (!(pointA_y < outRect.top) || !(pointB_y < outRect.top)) && (!(pointA_y > outRect.bottom) || !(pointB_y > outRect.bottom)))
      if (pointA_x >= out_rect.left || pointB_x >= out_rect.left) && (pointA_x <= out_rect.right || pointB_x <= out_rect.right) && (pointA_y >= out_rect.top || pointB_y >= out_rect.top) && (pointA_y <= out_rect.bottom || pointB_y <= out_rect.bottom) {
         // Java: RectF inkRect = this.getInkRect();
         let ink_rect = RectF { left: self.inkRect.left, top: self.inkRect.top, right: self.inkRect.right, bottom: self.inkRect.bottom };
         // Java: SizeF scale = calScaleXY(inkRect, srcRect.width(), srcRect.height());
         let src_w = srcRect.right - srcRect.left;
         let src_h = srcRect.bottom - srcRect.top;
         let scale = Ink::calScaleXY(&ink_rect, src_w, src_h);
         // Java: float scaleX = scale.width;
         let scale_x = scale.width;
         // Java: float scaleY = scale.height;
         let scale_y = scale.height;
         // Java: pointA_x = pointA_x - srcRect.left + inkRect.left * scaleX;
         pointA_x = pointA_x - srcRect.left + ink_rect.left * scale_x;
         // Java: pointA_y = pointA_y - srcRect.top + inkRect.top * scaleY;
         pointA_y = pointA_y - srcRect.top + ink_rect.top * scale_y;
         // Java: pointB_x = pointB_x - srcRect.left + inkRect.left * scaleX;
         pointB_x = pointB_x - srcRect.left + ink_rect.left * scale_x;
         // Java: pointB_y = pointB_y - srcRect.top + inkRect.top * scaleY;
         pointB_y = pointB_y - srcRect.top + ink_rect.top * scale_y;
         // Java: return HitService.hit(this.getTraces(), pointA_x, pointA_y, pointB_x, pointB_y, scaleX, scaleY, deviation);
         -1
      }
      // Java: else
      else {
         // Java: return -1;
         -1
      }
    }

    // Java: static SizeF calScaleXY(RectF inkRect, float drawingWidth, float drawingHeight)
    pub fn calScaleXY(ink_rect: &RectF, drawingWidth: f32, drawingHeight: f32) -> SizeF {
      // Java: SizeF size = new SizeF();
      let mut size = SizeF { width: 0.0, height: 0.0 };
      let rect_w = ink_rect.right - ink_rect.left;
      let rect_h = ink_rect.bottom - ink_rect.top;
      // Java: if (!inkRect.isEmpty())
      if rect_w != 0.0 && rect_h != 0.0 {
         // Java: size.width = drawingWidth / inkRect.width();
         size.width = drawingWidth / rect_w;
         // Java: size.height = drawingHeight / inkRect.height();
         size.height = drawingHeight / rect_h;
      }
      // Java: else if (inkRect.width() == 0.0F && inkRect.height() == 0.0F)
      else if rect_w == 0.0 && rect_h == 0.0 {
         // Java: size.width = 0.037795275F;
         size.width = 0.037795275;
         // Java: size.height = 0.037795275F;
         size.height = 0.037795275;
      }
      // Java: else if (inkRect.width() == 0.0F)
      else if rect_w == 0.0 {
         // Java: size.height = drawingHeight / inkRect.height();
         size.height = drawingHeight / rect_h;
         // Java: size.width = size.height;
         size.width = size.height;
      }
      // Java: else if (inkRect.height() == 0.0F)
      else if rect_h == 0.0 {
         // Java: size.width = drawingWidth / inkRect.width();
         size.width = drawingWidth / rect_w;
         // Java: size.height = size.width;
         size.height = size.width;
      }
      // Java: return size;
      size
    }

    // Java: ArrayList<Integer> hitLineEx(cn.wps.graphics.RectF srcRect, float pointA_x, float pointA_y, float pointB_x, float pointB_y, float deviation)
    pub fn hitLineEx(&mut self, srcRect: &RectF, mut pointA_x: f32, mut pointA_y: f32, mut pointB_x: f32, mut pointB_y: f32, mut deviation: f32) -> Option<Vec<i32>> {
      // Java: cn.wps.graphics.RectF outRect = new cn.wps.graphics.RectF(srcRect);
      let mut out_rect = RectF { left: srcRect.left, top: srcRect.top, right: srcRect.right, bottom: srcRect.bottom };
      // Java: this.calOutRect(outRect);
      self.calOutRect(&mut out_rect);
      // Java: if ((!(pointA_x < outRect.left) || !(pointB_x < outRect.left)) && (!(pointA_x > outRect.right) || !(pointB_x > outRect.right)) && (!(pointA_y < outRect.top) || !(pointB_y < outRect.top)) && (!(pointA_y > outRect.bottom) || !(pointB_y > outRect.bottom)))
      if (pointA_x >= out_rect.left || pointB_x >= out_rect.left) && (pointA_x <= out_rect.right || pointB_x <= out_rect.right) && (pointA_y >= out_rect.top || pointB_y >= out_rect.top) && (pointA_y <= out_rect.bottom || pointB_y <= out_rect.bottom) {
         // Java: RectF inkRect = this.getInkRect();
         let ink_rect = RectF { left: self.inkRect.left, top: self.inkRect.top, right: self.inkRect.right, bottom: self.inkRect.bottom };
         // Java: float scaleX = 1.0F;
         let mut scale_x: f32 = 1.0;
         // Java: if (inkRect.width() > 0.0F)
         let ink_w = ink_rect.right - ink_rect.left;
         let ink_h = ink_rect.bottom - ink_rect.top;
         let src_w = srcRect.right - srcRect.left;
         let src_h = srcRect.bottom - srcRect.top;
         if ink_w > 0.0 {
            // Java: scaleX = inkRect.width() / srcRect.width();
            scale_x = ink_w / src_w;
         }
         // Java: float scaleY = 1.0F;
         let mut scale_y: f32 = 1.0;
         // Java: if (inkRect.height() > 0.0F)
         if ink_h > 0.0 {
            // Java: scaleY = inkRect.height() / srcRect.height();
            scale_y = ink_h / src_h;
         }
         // Java: pointA_x = (pointA_x - srcRect.left) * scaleX + inkRect.left;
         pointA_x = (pointA_x - srcRect.left) * scale_x + ink_rect.left;
         // Java: pointA_y = (pointA_y - srcRect.top) * scaleY + inkRect.top;
         pointA_y = (pointA_y - srcRect.top) * scale_y + ink_rect.top;
         // Java: pointB_x = (pointB_x - srcRect.left) * scaleX + inkRect.left;
         pointB_x = (pointB_x - srcRect.left) * scale_x + ink_rect.left;
         // Java: pointB_y = (pointB_y - srcRect.top) * scaleY + inkRect.top;
         pointB_y = (pointB_y - srcRect.top) * scale_y + ink_rect.top;
         // Java: deviation = deviation * (scaleX + scaleY) / 2.0F;
         deviation = deviation * (scale_x + scale_y) / 2.0;
         // Java: return HitService.hitEx(this.getTraces(), pointA_x, pointA_y, pointB_x, pointB_y, 1.0F, 1.0F, deviation);
         None
      }
      // Java: else
      else {
         // Java: return null;
         None
      }
    }

    // Java: InkPathCache getPathCache()
    pub fn getPathCache(&self) -> Option<&InkPathCache> {
      // Java: return this.pathCache;
      self.pathCache.as_ref()
    }

    // Java: void setPathCache(InkPathCache pathCache)
    pub fn setPathCache(&mut self, pathCache: Option<InkPathCache>) {
      // Java: this.pathCache = pathCache;
      self.pathCache = pathCache;
    }

}
