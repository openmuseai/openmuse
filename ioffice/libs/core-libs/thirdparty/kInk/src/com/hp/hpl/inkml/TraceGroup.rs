#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.moffice.util.KSLog
use util::cn::wps::moffice::util::KSLog::KSLog;
// Java: import java.util.ArrayList
use std::collections::HashMap;
// Java: import java.util.HashMap
// Java: import java.util.StringTokenizer

use super::InkMLSerializer::InkMLWriter;
use super::Context::Context;
use super::Definitions::Definitions;
use super::Brush::Brush;
use super::Annotation::Annotation;
use super::AnnotationXML::AnnotationXML;
use super::TraceFormat::TraceFormat;

// Java field: String TAG
const TAG: &str = "TraceGroup";

// Java: public class TraceGroup implements TraceDataElement, Cloneable
pub struct TraceGroup {
    // Java field: String id
    pub(crate) id: String,
    // Java field: String contextRef
    pub(crate) contextRef: String,
    // Java field: String brushRef
    pub(crate) brushRef: String,
    // Java field: ArrayList<TraceDataElement> traceDataList
    pub(crate) traceDataList: Vec<Box<TraceGroup>>,
    // Java field: ArrayList<AnnotationElement> annotations
    pub(crate) annotations: Vec<String>,
    // Java field: Context associatedContext
    pub(crate) associatedContext: Option<Context>,
    // Java field: TraceGroup parentTraceGroup
    pub(crate) parentTraceGroup: Option<Box<TraceGroup>>,
}

// Java: public class TraceGroup implements TraceDataElement, Cloneable
impl TraceGroup {
    // Java: TraceGroup()
    pub fn TraceGroup() -> Self {
        // Java: super();
        Self {
            id: String::new(),
            contextRef: String::new(),
            brushRef: String::new(),
            traceDataList: Vec::new(),
            annotations: Vec::new(),
            associatedContext: None,
            parentTraceGroup: None,
        }
    }

    // Java: Context resolveAssociatedContext(Context currentContext, Definitions definitions)
    pub fn resolveAssociatedContext(&mut self, currentContext: Option<&Context>, _definitions: &Definitions) -> Context {
        // Java: String contextRef = this.getContextRef();
        let contextRef = self.getContextRef();
        // Java: if (!"".equals(contextRef))
        if !contextRef.is_empty() {
            // Java: this.associatedContext = definitions.getContextRefElement(contextRef);
            // Java: return this.associatedContext;
            return self.associatedContext.as_ref().map(|c| c.clone_context()).unwrap_or_else(|| Context::getDefaultContext());
        // Java: else
        } else {
            // Java: if (null != this.parentTraceGroup)
            if let Some(ref parent) = self.parentTraceGroup {
                // Java: Context associatedContext = this.parentTraceGroup.getAssociatedContext();
                let assoc = parent.getAssociatedContext();
                // Java: if (null != associatedContext)
                if let Some(ref ctx) = assoc {
                    // Java: return associatedContext;
                    return ctx.clone_context();
                }
            }
            // Java: return null != currentContext ? currentContext : Context.getDefaultContext();
            currentContext.map(|c| c.clone_context()).unwrap_or_else(|| Context::getDefaultContext())
        }
    }

    // Java: ArrayList<Trace> getTraceList()
    pub fn getTraceList(&self) -> Vec<Box<TraceGroup>> {
        // Java: ArrayList<Trace> ret = new ArrayList();
        let mut ret: Vec<Box<TraceGroup>> = Vec::new();
        // Java: for(TraceDataElement traceData : this.traceDataList)
        for trace_data in &self.traceDataList {
            // Java: if ("TraceGroup".equals(traceData.getInkElementType()))
            if trace_data.getInkElementType() == "TraceGroup" {
                // Java: ret.addAll(((TraceGroup)traceData).getTraceList());
                let sub_list = trace_data.getTraceList();
                for item in sub_list {
                    ret.push(item);
                }
            // Java: else
            } else {
                // Java: ret.add((Trace)traceData);
                ret.push(Box::new(trace_data.clone_tg()));
            }
        }
        // Java: return ret;
        ret
    }

    // Java: void setAssociatedContext(Context associatedContext)
    pub fn setAssociatedContext_ctx(&mut self, associatedContext: Context) {
        // Java: this.associatedContext = associatedContext;
        self.associatedContext = Some(associatedContext);
    }

    // Java: Context getAssociatedContext()
    pub fn getAssociatedContext(&self) -> Option<&Context> {
        // Java: return this.associatedContext;
        self.associatedContext.as_ref()
    }

    // Java: String getId()
    pub fn getId(&self) -> &str {
        // Java: return this.id == null ? "" : this.id;
        &self.id
    }

    // Java: String getInkElementType()
    pub fn getInkElementType(&self) -> String {
        // Java: return "TraceGroup";
        "TraceGroup".to_string()
    }

    // Java: IBrush getAssociatedBrush()
    pub fn getAssociatedBrush(&self) -> Option<&Brush> {
        // Java: return this.associatedContext.getBrush();
        self.associatedContext.as_ref().and_then(|ctx| ctx.getBrush())
    }

    // Java: TraceDataElement getSelectedTraceDataByRange(String from, String to)
    pub fn getSelectedTraceDataByRange(&self, from: &str, to: &str) -> Result<Box<TraceGroup>, String> {
        // Java: if (from == null)
        // Java: throw new InkMLException("NULL value for the parameter 'from'");
        // Java: else if (to == null)
        // Java: throw new InkMLException("NULL value for the parameter 'to'");
        // Java: else
        // Java: KSLog.w(TAG, "In TG from: " + from + "; to: " + to);
        KSLog::w(TAG, &format!("In TG from: {}; to: {}", from, to));
        // Java: TraceGroup traceDataElement = null;
        // Java: if ("".equals(from) && "".equals(to))
        if from.is_empty() && to.is_empty() {
            // Java: traceDataElement = this;
            return Ok(Box::new(self.clone_tg()));
        // Java: else
        } else {
            // Java: traceDataElement = new TraceGroup();
            let mut traceDataElement = TraceGroup::TraceGroup();
            // Java: traceDataElement.setAssociatedContext(this.associatedContext);
            if let Some(ref ctx) = self.associatedContext {
                traceDataElement.associatedContext = Some(ctx.clone_context());
            }
            // Java: traceDataElement.brushRef = this.brushRef;
            traceDataElement.brushRef = self.brushRef.clone();
            // Java: traceDataElement.contextRef = this.contextRef;
            traceDataElement.contextRef = self.contextRef.clone();
            // Java: if ("".equals(to))
            if to.is_empty() {
                // Java: int[] fromArr = this.parseRange(from);
                let fromArr = self.parseRange(from);
                // Java: if (!this.isRangeDataValid(fromArr))
                if !self.isRangeDataValid(&fromArr) {
                    // Java: throw new InkMLException("The given 'from' RangeString, " + from + " is not valid");
                    return Err(format!("The given 'from' RangeString, {} is not valid", from));
                }
                // Java: if (1 == fromArr.length)
                if fromArr.len() == 1 {
                    // Java: TraceDataElement subtree = this.getTraceDataAt(fromArr[0]);
                    // Java: if ("Trace".equals(subtree.getInkElementType()))
                    // Java: ((Trace)subtree).setParentTraceGroup(this);
                    // Java: else
                    // Java: ((TraceGroup)subtree).setParentTraceGroup(this);
                    // Java: traceDataElement.addToTraceData(subtree);
                // Java: else
                } else {
                    // Java: TraceGroup parentTG = this;
                    // Java: TraceDataElement var15 = ((TraceGroup)this).getTraceDataAt(fromArr[0]);
                    // Java: if ("Trace".equals(var15.getInkElementType()))
                    // Java: Trace data = (Trace)((Trace)var15).getSelectedTraceDataByRange(String.valueOf(fromArr[1]), "");
                    // Java: data.setParentTraceGroup(parentTG);
                    // Java: traceDataElement.addToTraceData(data);
                    // Java: else
                    // Java: String subFrom = from.substring(2);
                    // Java: TraceGroup data = (TraceGroup)((TraceGroup)var15).getSelectedTraceDataByRange(subFrom, "");
                    // Java: data.setParentTraceGroup(parentTG);
                    // Java: traceDataElement.addToTraceData(data);
                }
                // Java: for(int j = fromArr[0]; j < this.traceDataList.size(); ++j)
                // Java: traceDataElement.addToTraceData((TraceDataElement)this.traceDataList.get(j));
            // Java: else if ("".equals(from))
            } else if from.is_empty() {
                // Java: int[] toArr = this.parseRange(to);
                let toArr = self.parseRange(to);
                // Java: if (!this.isRangeDataValid(toArr))
                if !self.isRangeDataValid(&toArr) {
                    // Java: throw new InkMLException("The given 'to' RangeString, " + to + " is not valid");
                    return Err(format!("The given 'to' RangeString, {} is not valid", to));
                }
                // Java: for(int j = 0; j < toArr[0] - 1; ++j)
                // Java: traceDataElement.addToTraceData((TraceDataElement)this.traceDataList.get(j));
                // Java: if (1 == toArr.length)
                if toArr.len() == 1 {
                    // Java: TraceDataElement var17 = this.getTraceDataAt(toArr[0]);
                    // Java: if ("Trace".equals(var17.getInkElementType()))
                    // Java: ((Trace)var17).setParentTraceGroup(this);
                    // Java: else
                    // Java: ((TraceGroup)var17).setParentTraceGroup(this);
                    // Java: traceDataElement.addToTraceData(var17);
                // Java: else
                } else {
                    // Java: TraceGroup var23 = this;
                    // Java: TraceDataElement var18 = ((TraceGroup)this).getTraceDataAt(toArr[0]);
                    // Java: if ("Trace".equals(var18.getInkElementType()))
                    // Java: Trace data = (Trace)((Trace)var18).getSelectedTraceDataByRange("", String.valueOf(toArr[1]));
                    // Java: data.setParentTraceGroup(var23);
                    // Java: traceDataElement.addToTraceData(data);
                    // Java: else
                    // Java: String subTo = to.substring(2);
                    // Java: TraceGroup data = (TraceGroup)((TraceGroup)var18).getSelectedTraceDataByRange("", subTo);
                    // Java: data.setParentTraceGroup(var23);
                    // Java: traceDataElement.addToTraceData(data);
                }
            // Java: else
            } else {
                // Java: int[] fromArr = this.parseRange(from);
                let fromArr = self.parseRange(from);
                // Java: if (!this.isRangeDataValid(fromArr))
                if !self.isRangeDataValid(&fromArr) {
                    // Java: throw new InkMLException("The given 'from' RangeString, " + from + " is not valid");
                    return Err(format!("The given 'from' RangeString, {} is not valid", from));
                }
                // Java: int[] toArr = this.parseRange(to);
                let toArr = self.parseRange(to);
                // Java: if (!this.isRangeDataValid(toArr))
                if !self.isRangeDataValid(&toArr) {
                    // Java: throw new InkMLException("The given 'to' RangeString, " + to + " is not valid");
                    return Err(format!("The given 'to' RangeString, {} is not valid", to));
                }
                // Java: if (1 == fromArr.length)
                if fromArr.len() == 1 {
                    // Java: TraceDataElement var19 = this.getTraceDataAt(fromArr[0]);
                    // Java: if ("Trace".equals(var19.getInkElementType()))
                    // Java: ((Trace)var19).setParentTraceGroup(this);
                    // Java: else
                    // Java: ((TraceGroup)var19).setParentTraceGroup(this);
                    // Java: traceDataElement.addToTraceData(var19);
                // Java: else
                } else {
                    // Java: TraceGroup var24 = this;
                    // Java: TraceDataElement var20 = ((TraceGroup)this).getTraceDataAt(fromArr[0]);
                    // Java: if ("Trace".equals(var20.getInkElementType()))
                    // Java: Trace data = (Trace)((Trace)var20).getSelectedTraceDataByRange(String.valueOf(fromArr[1]), "");
                    // Java: data.setParentTraceGroup(var24);
                    // Java: traceDataElement.addToTraceData(data);
                    // Java: else
                    // Java: String subFrom = from.substring(2);
                    // Java: TraceGroup data = (TraceGroup)((TraceGroup)var20).getSelectedTraceDataByRange(subFrom, "");
                    // Java: data.setParentTraceGroup(var24);
                    // Java: traceDataElement.addToTraceData(data);
                }
                // Java: int difference = toArr[0] - fromArr[0];
                // Java: int nIntermitElmnts = difference - 1;
                // Java: int k = fromArr[0];
                // Java: for(int j = 0; j < nIntermitElmnts; ++j)
                // Java: traceDataElement.addToTraceData((TraceDataElement)this.traceDataList.get(k + j));
                // Java: if (1 == toArr.length)
                if toArr.len() == 1 {
                    // Java: TraceDataElement var21 = this.getTraceDataAt(toArr[0]);
                    // Java: if ("Trace".equals(var21.getInkElementType()))
                    // Java: ((Trace)var21).setParentTraceGroup(this);
                    // Java: else
                    // Java: ((TraceGroup)var21).setParentTraceGroup(this);
                    // Java: traceDataElement.addToTraceData(var21);
                // Java: else
                } else {
                    // Java: TraceGroup var25 = this;
                    // Java: TraceDataElement var22 = ((TraceGroup)this).getTraceDataAt(toArr[0]);
                    // Java: if ("Trace".equals(var22.getInkElementType()))
                    // Java: Trace data = (Trace)((Trace)var22).getSelectedTraceDataByRange("", String.valueOf(toArr[1]));
                    // Java: data.setParentTraceGroup(var25);
                    // Java: traceDataElement.addToTraceData(data);
                    // Java: else
                    // Java: String subTo = to.substring(2);
                    // Java: TraceGroup data = (TraceGroup)((TraceGroup)var22).getSelectedTraceDataByRange("", subTo);
                    // Java: data.setParentTraceGroup(var25);
                    // Java: traceDataElement.addToTraceData(data);
                }
            }
            // Java: return traceDataElement;
            Ok(Box::new(traceDataElement))
        }
    }

    // Java: void addToTraceData(TraceDataElement subtree)
    pub fn addToTraceData(&mut self, subtree: TraceGroup) {
        // Java: if (null == this.traceDataList)
        // Java: this.traceDataList = new ArrayList();
        // Java: if ("Trace".equals(subtree.getInkElementType()))
        // Java: ((Trace)subtree).setParentTraceGroup(this);
        // Java: else if ("TraceGroup".equals(subtree.getInkElementType()))
        // Java: ((TraceGroup)subtree).setParentTraceGroup(this);
        // Java: this.traceDataList.add(subtree);
        self.traceDataList.push(Box::new(subtree));
    }

    // Java: ArrayList<AnnotationElement> getAnnotation()
    pub fn getAnnotation(&self) -> &Vec<String> {
        // Java: return this.annotations;
        &self.annotations
    }

    // Java: void addAnnotation(AnnotationElement annotation)
    pub fn addAnnotation(&mut self, annotation: &str) {
        // Java: if (annotation != null)
        if !annotation.is_empty() {
            // Java: this.annotations.add(annotation);
            self.annotations.push(annotation.to_string());
        }
    }

    // Java: void removeTraceDataAt(int index)
    pub fn removeTraceDataAt(&mut self, index: i32) {
        // Java: this.traceDataList.remove(index);
        if (index as usize) < self.traceDataList.len() {
            self.traceDataList.remove(index as usize);
        }
    }

    // Java: void removeTraceDataAtFirst(TraceDataElement data)
    pub fn removeTraceDataAtFirst(&mut self, _data: &TraceGroup) {
        // Java: int firstIndex = this.traceDataList.indexOf(data);
        // Java: this.traceDataList.remove(firstIndex);
    }

    // Java: void removeTraceDataAtLast(TraceDataElement data)
    pub fn removeTraceDataAtLast(&mut self, _data: &TraceGroup) {
        // Java: int lastIndex = this.traceDataList.lastIndexOf(data);
        // Java: this.traceDataList.remove(lastIndex);
    }

    // Java: int[] parseRange(String rangeStr)
    pub fn parseRange(&self, rangeStr: &str) -> Vec<i32> {
        // Java: int[] rangeArr = null;
        // Java: StringTokenizer tokens = new StringTokenizer(rangeStr, ":");
        // Java: int length = tokens.countTokens();
        // Java: rangeArr = new int[length];
        let mut rangeArr: Vec<i32> = Vec::new();
        // Java: for(int i = 0; tokens.hasMoreTokens(); ++i)
        for token in rangeStr.split(':') {
            // Java: rangeArr[i] = Integer.parseInt(tokens.nextToken());
            if let Ok(v) = token.parse::<i32>() {
                rangeArr.push(v);
            }
        }
        // Java: return rangeArr;
        rangeArr
    }

    // Java: boolean isRangeDataValid(int[] rangeArr)
    pub fn isRangeDataValid(&self, rangeArr: &[i32]) -> bool {
        // Java: if (rangeArr[0] >= 1 && rangeArr[0] <= this.traceDataList.size())
        if !rangeArr.is_empty() && rangeArr[0] >= 1 && (rangeArr[0] as usize) <= self.traceDataList.len() {
            // Java: boolean isValid = true;
            let mut isValid = true;
            // Java: int length = rangeArr.length;
            let length = rangeArr.len();
            // Java: TraceDataElement temp = this;
            // Java: for(int i = 0; i < length; ++i)
            for i in 0..length {
                // Java: temp = ((TraceGroup)temp).getTraceDataAt(rangeArr[i]);
                // Java: if ("Trace".equals(temp.getInkElementType()))
                // Java: if (length - i > 2)
                if length - i > 2 {
                    // Java: isValid = false;
                    isValid = false;
                }
                // Java: break;
                break;
            }
            // Java: return isValid;
            isValid
        // Java: else
        } else {
            // Java: return false;
            false
        }
    }

    // Java: TraceDataElement getTraceDataAt(int index)
    pub fn getTraceDataAt(&self, index: i32) -> Result<&TraceGroup, String> {
        // Java: --index;
        let idx = index - 1;
        // Java: if (index >= 0 && index <= this.traceDataList.size())
        if idx >= 0 && (idx as usize) < self.traceDataList.len() {
            // Java: return (TraceDataElement)this.traceDataList.get(index);
            Ok(&self.traceDataList[idx as usize])
        // Java: else
        } else {
            // Java: throw new InkMLException("The indexOutofBound exception in getting TraceData");
            Err("The indexOutofBound exception in getting TraceData".to_string())
        }
    }

    // Java: void setTraceData(ArrayList<TraceDataElement> newTraceData)
    pub fn setTraceData(&mut self, newTraceData: Vec<Box<TraceGroup>>) {
        // Java: this.traceDataList = newTraceData;
        self.traceDataList = newTraceData;
    }

    // Java: TraceFormat getAssociatedTraceFormat()
    pub fn getAssociatedTraceFormat(&self) -> Option<&TraceFormat> {
        // Java: return this.associatedContext.getTraceFormat();
        self.associatedContext.as_ref().and_then(|ctx| ctx.getTraceFormat())
    }

    // Java: void setAssociatedBrush(IBrush brush)
    pub fn setAssociatedBrush(&mut self, brush: Brush) {
        // Java: this.associatedContext.setBrush(brush);
        if let Some(ref mut ctx) = self.associatedContext {
            ctx.setBrush(brush);
        }
    }

    // Java: TraceGroup getParentTraceGroup()
    pub fn getParentTraceGroup(&self) -> Option<&TraceGroup> {
        // Java: return this.parentTraceGroup;
        self.parentTraceGroup.as_deref()
    }

    // Java: void setParentTraceGroup(TraceGroup parentTraceGroup)
    pub fn setParentTraceGroup(&mut self, parentTraceGroup: TraceGroup) {
        // Java: this.parentTraceGroup = parentTraceGroup;
        self.parentTraceGroup = Some(Box::new(parentTraceGroup));
    }

    // Java: String toInkML(Definitions definitions)
    pub fn toInkML_defs(&self, _definitions: &Definitions) -> String {
        // Java: StringBuffer traceGroupStrBuff = new StringBuffer("<traceGroup");
        let mut buf = String::from("<traceGroup");
        // Java: if (!"".equals(this.id))
        if !self.id.is_empty() {
            // Java: traceGroupStrBuff.append(" id=\"" + this.id + "\"");
            buf += &format!(" id=\"{}\"", self.id);
        }
        // Java: if (!"".equals(this.contextRef))
        if !self.contextRef.is_empty() {
            // Java: traceGroupStrBuff.append(" contextRef=\"" + this.contextRef + "\"");
            buf += &format!(" contextRef=\"{}\"", self.contextRef);
        }
        // Java: if (!"".equals(this.brushRef))
        if !self.brushRef.is_empty() {
            // Java: traceGroupStrBuff.append(" brushRef=\"" + this.brushRef + "\"");
            buf += &format!(" brushRef=\"{}\"", self.brushRef);
        }
        // Java: int size = this.traceDataList.size();
        let size = self.traceDataList.len();
        // Java: if (size != 0)
        if size != 0 {
            // Java: traceGroupStrBuff.append(">");
            buf += ">";
            // Java: for(TraceDataElement child : this.traceDataList)
            for child in &self.traceDataList {
                // Java: traceGroupStrBuff.append(child.toInkML(definitions));
                buf += &child.toInkML_defs(_definitions);
            }
            // Java: traceGroupStrBuff.append("</traceGroup>");
            buf += "</traceGroup>";
        // Java: else
        } else {
            // Java: traceGroupStrBuff.append(" />");
            buf += " />";
        }
        // Java: return traceGroupStrBuff.toString();
        buf
    }

    // Java: String getContextRef()
    pub fn getContextRef(&self) -> String {
        // Java: return this.contextRef;
        self.contextRef.clone()
    }

    // Java: void writeXML(InkMLWriter writer)
    pub fn writeXML(&self, writer: &mut InkMLWriter) {
        // Java: HashMap<String, String> attrs = new HashMap();
        let mut attrs: HashMap<String, String> = HashMap::new();
        // Java: if (!"".equals(this.id))
        if !self.id.is_empty() {
            // Java: attrs.put("id", this.id);
            attrs.insert("id".to_string(), self.id.clone());
        }
        // Java: if (!"".equals(this.contextRef))
        if !self.contextRef.is_empty() {
            // Java: attrs.put("contextRef", this.contextRef);
            attrs.insert("contextRef".to_string(), self.contextRef.clone());
        }
        // Java: if (!"".equals(this.brushRef))
        if !self.brushRef.is_empty() {
            // Java: attrs.put("brushRef", this.brushRef);
            attrs.insert("brushRef".to_string(), self.brushRef.clone());
        }
        // Java: if (0 == attrs.size())
        // Java: attrs = null;
        let attrs_opt = if attrs.is_empty() { None } else { Some(&attrs) };
        // Java: int size = this.traceDataList.size();
        let size = self.traceDataList.len();
        // Java: if (size != 0)
        if size != 0 {
            // Java: writer.writeStartTag("traceGroup", attrs);
            writer.writeStartTag("traceGroup", attrs_opt.map(|a| a as &HashMap<String, String>));
            // Java: writer.incrementTagLevel();
            writer.incrementTagLevel();
            // Java: for(TraceDataElement child : this.traceDataList)
            for child in &self.traceDataList {
                // Java: child.writeXML(writer);
                child.writeXML(writer);
            }
            // Java: writer.decrementTagLevel();
            writer.decrementTagLevel();
            // Java: writer.writeEndTag("traceGroup");
            writer.writeEndTag("traceGroup");
        // Java: else
        } else {
            // Java: writer.writeEmptyStartTag("traceGroup", attrs);
            writer.writeEmptyStartTag("traceGroup", attrs_opt.map(|a| a as &HashMap<String, String>));
        }
    }

    // Java: String getBrushRef()
    pub fn getBrushRef(&self) -> String {
        // Java: return this.brushRef;
        self.brushRef.clone()
    }

    // Java: void setBrushRef(String brushRef)
    pub fn setBrushRef(&mut self, brushRef: &str) {
        // Java: this.brushRef = brushRef;
        self.brushRef = brushRef.to_string();
    }

    // Java: void setContextRef(String contextRef)
    pub fn setContextRef(&mut self, contextRef: &str) {
        // Java: this.contextRef = contextRef;
        self.contextRef = contextRef.to_string();
    }

    // Java: void setId(String id)
    pub fn setId(&mut self, id: &str) {
        // Java: this.id = id;
        self.id = id.to_string();
    }

    // Java: ArrayList<TraceDataElement> getTraceDataList()
    pub fn getTraceDataList(&self) -> &Vec<Box<TraceGroup>> {
        // Java: return this.traceDataList;
        &self.traceDataList
    }

    // Java: void printTraceGroup()
    pub fn printTraceGroup(&self) {
        // Java: ArrayList<TraceDataElement> list = this.getTraceDataList();
        let list = self.getTraceDataList();
        // Java: if (null != list)
        if !list.is_empty() {
            // Java: KSLog.w(TAG, "<traceGroup>");
            KSLog::w(TAG, "<traceGroup>");
            // Java: for(TraceDataElement td : list)
            for td in list {
                // Java: if ("Trace".equals(td.getInkElementType()))
                if td.getInkElementType() == "Trace" {
                    // Java: ((Trace)td).printTrace();
                // Java: else
                } else {
                    // Java: ((TraceGroup)td).printTraceGroup();
                    td.printTraceGroup();
                }
            }
            // Java: KSLog.w(TAG, "</traceGroup>");
            KSLog::w(TAG, "</traceGroup>");
        }
    }

    // Java: String toInkML()
    pub fn toInkML(&self) -> String {
        // Java: return null;
        String::new()
    }

    // Java: TraceGroup clone()
    pub fn clone_tg(&self) -> TraceGroup {
        // Java: TraceGroup ret = new TraceGroup();
        let mut ret = TraceGroup::TraceGroup();
        // Java: if (null != this.id)
        if !self.id.is_empty() {
            // Java: ret.id = new String(this.id);
            ret.id = self.id.clone();
        }
        // Java: if (null != this.contextRef)
        if !self.contextRef.is_empty() {
            // Java: ret.contextRef = new String(this.contextRef);
            ret.contextRef = self.contextRef.clone();
        }
        // Java: if (null != this.brushRef)
        if !self.brushRef.is_empty() {
            // Java: ret.brushRef = new String(this.brushRef);
            ret.brushRef = self.brushRef.clone();
        }
        // Java: if (null != this.parentTraceGroup)
        if let Some(ref ptg) = self.parentTraceGroup {
            // Java: ret.parentTraceGroup = this.parentTraceGroup.clone();
            ret.parentTraceGroup = Some(Box::new(ptg.clone_tg()));
        }
        // Java: if (null != this.associatedContext)
        if let Some(ref ctx) = self.associatedContext {
            // Java: ret.associatedContext = this.associatedContext.clone();
            ret.associatedContext = Some(ctx.clone_context());
        }
        // Java: ret.annotations = this.cloneAnnotations();
        ret.annotations = self.cloneAnnotations();
        // Java: ret.traceDataList = this.cloneTraceDataList();
        ret.traceDataList = self.cloneTraceDataList();
        // Java: return ret;
        ret
    }

    // Java: ArrayList<TraceDataElement> cloneTraceDataList()
    pub fn cloneTraceDataList(&self) -> Vec<Box<TraceGroup>> {
        // Java: if (null == this.traceDataList)
        if self.traceDataList.is_empty() {
            // Java: return null;
            return Vec::new();
        // Java: else
        } else {
            // Java: ArrayList<TraceDataElement> ret = new ArrayList();
            let mut ret: Vec<Box<TraceGroup>> = Vec::new();
            // Java: int size = this.traceDataList.size();
            let size = self.traceDataList.len();
            // Java: for(int i = 0; i < size; ++i)
            for i in 0..size {
                // Java: TraceDataElement e = (TraceDataElement)this.traceDataList.get(i);
                let e = &self.traceDataList[i];
                // Java: if (e instanceof TraceGroup)
                // Java: TraceGroup traceGroup = (TraceGroup)e;
                // Java: ret.add(traceGroup.clone());
                // Java: else if (e instanceof Trace)
                // Java: Trace trace = (Trace)e;
                // Java: ret.add(trace.clone());
                // Java: else if (e instanceof TraceView)
                // Java: TraceView traceView = (TraceView)e;
                // Java: ret.add(traceView.clone());
                ret.push(Box::new(e.clone_tg()));
            }
            // Java: return ret;
            ret
        }
    }

    // Java: ArrayList<AnnotationElement> cloneAnnotations()
    pub fn cloneAnnotations(&self) -> Vec<String> {
        // Java: if (null == this.annotations)
        if self.annotations.is_empty() {
            // Java: return null;
            return Vec::new();
        // Java: else
        } else {
            // Java: ArrayList<AnnotationElement> ret = new ArrayList();
            let mut ret: Vec<String> = Vec::new();
            // Java: int size = this.annotations.size();
            let size = self.annotations.len();
            // Java: for(int i = 0; i < size; ++i)
            for i in 0..size {
                // Java: AnnotationElement e = (AnnotationElement)this.annotations.get(i);
                let e = &self.annotations[i];
                // Java: if (e instanceof Annotation)
                // Java: Annotation a = (Annotation)e;
                // Java: ret.add(a.clone());
                // Java: else if (e instanceof AnnotationXML)
                // Java: AnnotationXML aXml = (AnnotationXML)e;
                // Java: ret.add(aXml.clone());
                ret.push(e.clone());
            }
            // Java: return ret;
            ret
        }
    }
}
