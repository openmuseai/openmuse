#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import android.graphics.RectF
use util::graphics::RectF::RectF;
// Java: import cn.wps.moffice.util.KSLog
use util::cn::wps::moffice::util::KSLog::KSLog;
// Java: import java.text.NumberFormat
// Java: import java.util.ArrayList
use std::collections::HashMap;
// Java: import java.util.HashMap
// Java: import java.util.Iterator
// Java: import java.util.Locale
// Java: import java.util.StringTokenizer
// Java: import java.util.regex.Matcher
// Java: import java.util.regex.Pattern

use super::InkMLSerializer::InkMLWriter;
use super::Context::Context;
use super::Definitions::Definitions;
use super::Brush::Brush;
use super::TraceFormat::TraceFormat;
use super::TraceDataList::TraceDataList;
use super::TraceGroup::TraceGroup;
use super::Channel::ChannelType;
use super::CanvasTransform::CanvasTransform;
use super::Mapping::Mapping;

// Java field: String TAG
const TAG: &str = "Trace";

// Java: public enum TraceType
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum TraceType {
    // Java: TraceType()
    penDown,
    penUp,
    indeterminate,
}

// Java: public enum Continuation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Continuation {
    // Java: Continuation()
    NONE,
    begin,
    middle,
    end,
}

// Java: public enum TracePrefix
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum TracePrefix {
    // Java: TracePrefix()
    NONE,
    velocity,
    acceleration,
}

// Java: public class Trace implements TraceDataElement, Cloneable
pub struct Trace {
    // Java field: HashMap<String, String> attributesMap
    pub(crate) attributesMap: HashMap<String, String>,
    // Java field: TraceDataList traceData
    pub(crate) traceData: TraceDataList,
    // Java field: float mScaleX
    pub(crate) mScaleX: f32,
    // Java field: float mScaleY
    pub(crate) mScaleY: f32,
    // Java field: float mTranslateX
    pub(crate) mTranslateX: f32,
    // Java field: float mTranslateY
    pub(crate) mTranslateY: f32,
    // Java field: Continuation continuation
    continuation: Continuation,
    // Java field: HashMap<String, TracePrefix> lastPrefixMap
    lastPrefixMap: Option<HashMap<String, TracePrefix>>,
    // Java field: Trace priorReferredTrace
    priorReferredTrace: Option<Box<Trace>>,
    // Java field: Context associatedContext
    pub(crate) associatedContext: Option<Context>,
    // Java field: TraceGroup parentTraceGroup
    pub(crate) parentTraceGroup: Option<Box<TraceGroup>>,
}

// Java: public class Trace implements TraceDataElement, Cloneable
impl Trace {
    // Java: Trace()
    pub fn Trace() -> Self {
        // Java: super();
        // Java: this.continuation = Trace.Continuation.NONE;
        // Java: this.priorReferredTrace = null;
        // Java: this.attributesMap = new HashMap();
        // Java: this.traceData = new TraceDataList();
        Self {
            attributesMap: HashMap::new(),
            traceData: TraceDataList::TraceDataList(),
            mScaleX: 0.0,
            mScaleY: 0.0,
            mTranslateX: 0.0,
            mTranslateY: 0.0,
            continuation: Continuation::NONE,
            lastPrefixMap: None,
            priorReferredTrace: None,
            associatedContext: None,
            parentTraceGroup: None,
        }
    }

    // Java: void initTraceDataStructure()
    pub fn initTraceDataStructure(&mut self) {
        // Java: if (null != this.associatedContext)
        if let Some(ref ctx) = self.associatedContext {
            // Java: TraceFormat traceFormat = this.associatedContext.getTraceFormat();
            // Java: this.traceData = this.initTraceDataList(traceFormat);
        }
    }

    // Java: void initTraceDataStructure(TraceFormat format)
    pub fn initTraceDataStructure_fmt(&mut self, format: &TraceFormat) {
        // Java: if (format != null)
        // Java: this.traceData = this.initTraceDataList(format);
    }

    // Java: TraceDataList initTraceDataList(TraceFormat traceFormat)
    pub fn initTraceDataList(&self, _traceFormat: &TraceFormat) -> TraceDataList {
        // Java: int xIndex = 0;
        // Java: int yIndex = 1;
        // Java: int fIndex = -1;
        // Java: Channel.ChannelType xyChannelType = Channel.ChannelType.DECIMAL;
        // Java: ArrayList<Channel> channelList = traceFormat.getChannelList();
        // Java: for(int i = 0; i < channelList.size(); ++i)
        // Java: Channel ch = (Channel)channelList.get(i);
        // Java: if (ch.getName().equals("X"))
        // Java: xIndex = i;
        // Java: xyChannelType = ch.getChannelType();
        // Java: else if (ch.getName().equals("Y"))
        // Java: yIndex = i;
        // Java: else if (ch.getName().equals("F"))
        // Java: fIndex = i;
        // Java: return new TraceDataList(xyChannelType, xIndex, yIndex, fIndex);
        TraceDataList::TraceDataList_full(ChannelType::DECIMAL, 0, 1, -1)
    }

    // Java: void resolveAssociatedContext(Context currentContext, Definitions defs)
    pub fn resolveAssociatedContext(&mut self, _currentContext: Option<&Context>, _defs: &Definitions) {
        // Java: String contextRefAttr = this.getContextRef();
        let _contextRefAttr = self.getContextRef();
        // Java: if (!"".equals(contextRefAttr))
        // Java: this.associatedContext = defs.getContextRefElement(contextRefAttr);
        // Java: this.associatedContext.deriveContextualChildrenData(defs, currentContext);
        // Java: else
        // Java: if (null != this.parentTraceGroup)
        // Java: this.associatedContext = this.parentTraceGroup.getAssociatedContext();
        // Java: this.associatedContext = null != this.associatedContext ? this.associatedContext : (null != currentContext ? currentContext : Context.getDefaultContext());
        // Java: this.setContextRef("#" + this.associatedContext.getId());
        // Java: String brushRefAttr = this.getBrushRef();
        // Java: if (!"".equals(brushRefAttr))
        // Java: this.associatedContext.setBrush(defs.getBrushRefElement(brushRefAttr));
        // Java: this.initTraceDataStructure();
    }

    // Java: void setContextRef(String contextRef)
    pub fn setContextRef(&mut self, contextRef: &str) {
        // Java: if (!"".equals(contextRef))
        if !contextRef.is_empty() {
            // Java: this.attributesMap.put("contextRef", contextRef);
            self.attributesMap.insert("contextRef".to_string(), contextRef.to_string());
        }
    }

    // Java: void setBrushRef(String brushRef)
    pub fn setBrushRef(&mut self, brushRef: &str) {
        // Java: if (!"".equals(brushRef))
        if !brushRef.is_empty() {
            // Java: this.attributesMap.put("brushRef", brushRef);
            self.attributesMap.insert("brushRef".to_string(), brushRef.to_string());
        }
    }

    // Java: TracePrefix getLastPrefixOf(String channelName)
    pub fn getLastPrefixOf(&self, channelName: &str) -> TracePrefix {
        // Java: return (TracePrefix)this.lastPrefixMap.get(channelName);
        self.lastPrefixMap.as_ref()
            .and_then(|m| m.get(channelName).copied())
            .unwrap_or(TracePrefix::NONE)
    }

    // Java: void processAndStoreChannelValue(Object[] traceFragment, int channelIndex, Object[] lastFragment, String channelName, String channelValue, Channel.ChannelType channelType, int traceIndex, HashMap<String, Object> variableValueMap, Object previousDoubleDiffVal)
    pub fn processAndStoreChannelValue(&mut self, _traceFragment: &mut Vec<f32>, _channelIndex: usize, _lastFragment: &[f32], _channelName: &str, _channelValue: &str, _channelType: ChannelType, _traceIndex: i32, _variableValueMap: &mut HashMap<String, f32>, _previousDoubleDiffVal: f32) {
        // Java: int mCharPos;
        // Java: Object chnVal;
        // Java: if ((mCharPos = channelValue.indexOf(33)) >= 0)
        // Java: if (channelType == Channel.ChannelType.BOOLEAN)
        // Java: throw new InkMLException("The prefix <!> is NOT applicable to BOOLEAN channel.");
        // Java: chnVal = this.getChannelValueObject(channelValue.substring(mCharPos + 1), channelType);
        // Java: else if ((mCharPos = channelValue.indexOf(39)) >= 0)
        // Java: if (channelType == Channel.ChannelType.BOOLEAN)
        // Java: throw new InkMLException("The prefix <'> is NOT applicable to BOOLEAN channel.");
        // Java: if (this.lastPrefixMap.get(channelName) != Trace.TracePrefix.NONE || this.continuation == Trace.Continuation.begin || 0 == traceIndex)
        // Java: throw new InkMLException("The condition that the the prefix <'> must be preceded with an explicit value is violated.\nProblem is in the channelValue " + channelValue + " (channel name = " + channelName + ").");
        // Java: this.lastPrefixMap.put(channelName, Trace.TracePrefix.velocity);
        // Java: Object previousValue = lastFragment[channelIndex];
        // Java: Object singleDiffVal = this.getChannelValueObject(channelValue.substring(mCharPos + 1), channelType);
        // Java: variableValueMap.put(channelName, singleDiffVal);
        // Java: chnVal = null;
        // Java: if (channelType == Channel.ChannelType.DECIMAL || channelType == Channel.ChannelType.INTEGER)
        // Java: chnVal = (Float)previousValue + (Float)singleDiffVal;
        // Java: else if ((mCharPos = channelValue.indexOf(34)) >= 0)
        // Java: if (channelType == Channel.ChannelType.BOOLEAN)
        // Java: throw new InkMLException("The prefix <\"> is NOT applicable to BOOLEAN channel.");
        // Java: if (this.lastPrefixMap.get(channelName) != Trace.TracePrefix.velocity || this.continuation == Trace.Continuation.begin)
        // Java: throw new InkMLException("The condition that the the prefix <\"> must be preceded with a single difference is violated.\nProblem is in the channelValue " + channelValue + " (channel name = " + channelName + ").");
        // Java: this.lastPrefixMap.put(channelName, Trace.TracePrefix.acceleration);
        // Java: Object doubleDiffVal = this.getChannelValueObject(channelValue.substring(mCharPos + 1), channelType);
        // Java: Object currentVariableValue = variableValueMap.get(channelName);
        // Java: Object newVariableValue = null;
        // Java: if (channelType == Channel.ChannelType.DECIMAL || channelType == Channel.ChannelType.INTEGER)
        // Java: newVariableValue = (Float)doubleDiffVal + (Float)currentVariableValue;
        // Java: variableValueMap.put(channelName, newVariableValue);
        // Java: Object previousValue = lastFragment[channelIndex];
        // Java: chnVal = null;
        // Java: if (channelType == Channel.ChannelType.DECIMAL || channelType == Channel.ChannelType.INTEGER)
        // Java: chnVal = (Float)previousValue + (Float)newVariableValue;
        // Java: else
        // Java: if (channelValue.indexOf(63) >= 0)
        // Java: return;
        // Java: if (channelValue.indexOf(42) >= 0)
        // Java: Channel channel = this.getAssociatedContext().getTraceFormat().getChannel(channelName);
        // Java: if (channel.isIntermittent())
        // Java: Object previousValue = lastFragment[channelIndex];
        // Java: int i = this.traceData.size() - 2;
        // Java: while(null == previousValue)
        // Java: previousValue = ((Object[])this.traceData.get(i))[channelIndex];
        // Java: --i;
        // Java: if (i < 0)
        // Java: break;
        // Java: if (null == previousValue)
        // Java: previousValue = channel.getDefaultValue();
        // Java: if (channelType != Channel.ChannelType.DECIMAL && channelType != Channel.ChannelType.INTEGER)
        // Java: if (channelType == Channel.ChannelType.BOOLEAN)
        // Java: if ("F".equalsIgnoreCase((String)previousValue))
        // Java: previousValue = new Boolean("false");
        // Java: else if ("T".equalsIgnoreCase((String)previousValue))
        // Java: previousValue = new Boolean("true");
        // Java: else
        // Java: previousValue = new Float((String)previousValue);
        // Java: this.validateRange(previousValue.toString(), channelName);
        // Java: traceFragment[channelIndex] = previousValue;
        // Java: return;
        // Java: if (channelType == Channel.ChannelType.BOOLEAN)
        // Java: Object previousValue = lastFragment[channelIndex];
        // Java: if (this.lastPrefixMap.get(channelName) == Trace.TracePrefix.velocity)
        // Java: Object singleDiffVal = variableValueMap.get(channelName);
        // Java: Object previousValue = lastFragment[channelIndex];
        // Java: chnVal = null;
        // Java: if (channelType == Channel.ChannelType.DECIMAL || channelType == Channel.ChannelType.INTEGER)
        // Java: chnVal = (Float)previousValue + (Float)singleDiffVal;
        // Java: else if (this.lastPrefixMap.get(channelName) == Trace.TracePrefix.acceleration)
        // Java: Object currentVariableValue = variableValueMap.get(channelName);
        // Java: Object newVariableValue = null;
        // Java: if (channelType == Channel.ChannelType.DECIMAL || channelType == Channel.ChannelType.INTEGER)
        // Java: newVariableValue = (Float)previousDoubleDiffVal + (Float)currentVariableValue;
        // Java: variableValueMap.put(channelName, newVariableValue);
        // Java: Object previousValue = lastFragment[channelIndex];
        // Java: chnVal = null;
        // Java: if (channelType == Channel.ChannelType.DECIMAL || channelType == Channel.ChannelType.INTEGER)
        // Java: chnVal = (Float)previousValue + (Float)newVariableValue;
        // Java: else
        // Java: Object previousValue = lastFragment[channelIndex];
        // Java: chnVal = previousValue;
        // Java: else
        // Java: if (channelType == Channel.ChannelType.BOOLEAN)
        // Java: chnVal = null;
        // Java: if ("F".equalsIgnoreCase(channelValue))
        // Java: chnVal = new Boolean("false");
        // Java: else if ("T".equalsIgnoreCase(channelValue))
        // Java: chnVal = new Boolean("true");
        // Java: traceFragment[channelIndex] = chnVal;
        // Java: return;
        // Java: if (this.lastPrefixMap.get(channelName) == Trace.TracePrefix.velocity)
        // Java: Object previousValue = lastFragment[channelIndex];
        // Java: Object singleDiffVal = null;
        // Java: if (channelType == Channel.ChannelType.DECIMAL || channelType == Channel.ChannelType.INTEGER)
        // Java: singleDiffVal = new Float(channelValue);
        // Java: variableValueMap.put(channelName, singleDiffVal);
        // Java: chnVal = null;
        // Java: if (channelType == Channel.ChannelType.DECIMAL || channelType == Channel.ChannelType.INTEGER)
        // Java: chnVal = (Float)previousValue + (Float)singleDiffVal;
        // Java: else if (this.lastPrefixMap.get(channelName) == Trace.TracePrefix.acceleration)
        // Java: Object doubleDiffVal = null;
        // Java: if (channelType == Channel.ChannelType.DECIMAL || channelType == Channel.ChannelType.INTEGER)
        // Java: doubleDiffVal = new Float(channelValue);
        // Java: Object currentVariableValue = variableValueMap.get(channelName);
        // Java: Object newVariableValue = null;
        // Java: if (channelType == Channel.ChannelType.DECIMAL || channelType == Channel.ChannelType.INTEGER)
        // Java: newVariableValue = (Float)doubleDiffVal + (Float)currentVariableValue;
        // Java: variableValueMap.put(channelName, newVariableValue);
        // Java: Object previousValue = lastFragment[channelIndex];
        // Java: chnVal = null;
        // Java: if (channelType == Channel.ChannelType.DECIMAL || channelType == Channel.ChannelType.INTEGER)
        // Java: chnVal = (Float)previousValue + (Float)newVariableValue;
        // Java: else
        // Java: chnVal = null;
        // Java: if (channelType == Channel.ChannelType.DECIMAL || channelType == Channel.ChannelType.INTEGER)
        // Java: chnVal = new Float(channelValue);
        // Java: chnVal = null == chnVal ? 0 : chnVal;
        // Java: traceFragment[channelIndex] = chnVal;
    }

    // Java: Object getChannelValueObject(String channelValue, Channel.ChannelType channelType)
    pub fn getChannelValueObject(&self, channelValue: &str, channelType: ChannelType) -> f32 {
        // Java: Object chnVal = null;
        // Java: if (channelType != Channel.ChannelType.DECIMAL && channelType != Channel.ChannelType.INTEGER)
        // Java: if (channelType == Channel.ChannelType.BOOLEAN)
        // Java: chnVal = new Boolean(channelValue);
        // Java: else
        // Java: chnVal = new Float(channelValue);
        // Java: return chnVal;
        channelValue.parse::<f32>().unwrap_or(0.0)
    }

    // Java: TraceType getType()
    pub fn getType(&self) -> TraceType {
        // Java: String typeStr = (String)this.attributesMap.get("type");
        let typeStr = self.attributesMap.get("type");
        // Java: return null == typeStr ? Trace.TraceType.penDown : Trace.TraceType.valueOf(typeStr);
        match typeStr.map(|s| s.as_str()) {
            Some("penUp") => TraceType::penUp,
            Some("indeterminate") => TraceType::indeterminate,
            _ => TraceType::penDown,
        }
    }

    // Java: void setType(TraceType type)
    pub fn setType(&mut self, r#type: TraceType) {
        // Java: String typeStr = String.valueOf(type);
        // Java: this.attributesMap.put("type", typeStr);
        self.attributesMap.insert("type".to_string(), format!("{:?}", r#type));
    }

    // Java: Continuation getContinuation()
    pub fn getContinuation(&self) -> Continuation {
        // Java: String continuStr = (String)this.attributesMap.get("continuation");
        let continuStr = self.attributesMap.get("continuation");
        // Java: return null == continuStr ? Trace.Continuation.NONE : Trace.Continuation.valueOf(continuStr);
        match continuStr.map(|s| s.as_str()) {
            Some("begin") => Continuation::begin,
            Some("middle") => Continuation::middle,
            Some("end") => Continuation::end,
            _ => Continuation::NONE,
        }
    }

    // Java: void setContinuation(Continuation continuation)
    pub fn setContinuation(&mut self, continuation: Option<Continuation>) {
        // Java: if (continuation == null)
        if continuation.is_none() {
            // Java: this.attributesMap.put("continuation", null);
            self.attributesMap.remove("continuation");
        // Java: else
        } else {
            // Java: String continuStr = String.valueOf(continuation);
            // Java: this.attributesMap.put("continuation", continuStr);
            if let Some(c) = continuation {
                self.attributesMap.insert("continuation".to_string(), format!("{:?}", c));
            }
        }
    }

    // Java: String getContextRef()
    pub fn getContextRef(&self) -> String {
        // Java: String contextRef = (String)this.attributesMap.get("contextRef");
        // Java: return null == contextRef ? "" : contextRef;
        self.attributesMap.get("contextRef").cloned().unwrap_or_default()
    }

    // Java: String getBrushRef()
    pub fn getBrushRef(&self) -> String {
        // Java: String brushRef = (String)this.attributesMap.get("brushRef");
        // Java: return null == brushRef ? "" : brushRef;
        self.attributesMap.get("brushRef").cloned().unwrap_or_default()
    }

    // Java: long getDuration()
    pub fn getDuration(&self) -> i64 {
        // Java: String durationStr = (String)this.attributesMap.get("duration");
        let durationStr = self.attributesMap.get("duration");
        // Java: if (null == durationStr)
        if durationStr.is_none() {
            // Java: return 0L;
            0
        // Java: else
        } else {
            // Java: long duration = Long.parseLong(durationStr);
            // Java: return duration;
            durationStr.unwrap().parse::<i64>().unwrap_or(0)
        }
    }

    // Java: String getId()
    pub fn getId(&self) -> String {
        // Java: String id = (String)this.attributesMap.get("id");
        let mut id = self.attributesMap.get("id").cloned();
        // Java: if (null == id)
        if id.is_none() {
            // Java: id = (String)this.attributesMap.get("xml:id");
            id = self.attributesMap.get("xml:id").cloned();
            // Java: if (null == id)
            if id.is_none() {
                // Java: return "";
                return String::new();
            }
        }
        // Java: return id;
        id.unwrap_or_default()
    }

    // Java: String getPriorRef()
    pub fn getPriorRef(&self) -> String {
        // Java: String prioRef = (String)this.attributesMap.get("priorRef");
        // Java: return null == prioRef ? "" : prioRef;
        self.attributesMap.get("priorRef").cloned().unwrap_or_default()
    }

    // Java: float getTimeOffset()
    pub fn getTimeOffset(&self) -> f32 {
        // Java: String tOffsetStr = (String)this.attributesMap.get("timeOffset");
        let tOffsetStr = self.attributesMap.get("timeOffset");
        // Java: if (null == tOffsetStr)
        if tOffsetStr.is_none() {
            // Java: return 0.0F;
            0.0
        // Java: else
        } else {
            // Java: float timeOffset = Float.parseFloat(tOffsetStr);
            // Java: return timeOffset;
            tOffsetStr.unwrap().parse::<f32>().unwrap_or(0.0)
        }
    }

    // Java: IBrush getAssociatedBrush()
    pub fn getAssociatedBrush(&self) -> Option<&Brush> {
        // Java: return this.associatedContext.getBrush();
        self.associatedContext.as_ref().and_then(|ctx| ctx.getBrush())
    }

    // Java: void setAssociatedBrush(IBrush brush)
    pub fn setAssociatedBrush(&mut self, brush: Brush) {
        // Java: this.associatedContext.setBrush(brush);
        if let Some(ref mut ctx) = self.associatedContext {
            ctx.setBrush(brush);
        }
    }

    // Java: TraceFormat getAssociatedTraceFormat()
    pub fn getAssociatedTraceFormat(&self) -> Option<&TraceFormat> {
        // Java: return this.associatedContext.getTraceFormat();
        self.associatedContext.as_ref().and_then(|ctx| ctx.getTraceFormat())
    }

    // Java: String[] getTraceDataAsString()
    pub fn getTraceDataAsString(&self) -> Vec<String> {
        // Java: StringBuffer traceDataBuffer = new StringBuffer();
        // Java: int lineSize = 75;
        // Java: int channelSize = this.getAssociatedTraceFormat().getChannelList().size();
        // Java: Iterator<Object[]> iterator = this.traceData.iterator();
        // Java: while(true)
        // Java: Object[] traceFragment = iterator.next();
        // Java: for(int i = 0; i < channelSize; ++i)
        // Java: String value = String.valueOf(traceFragment[i]);
        // Java: if ("null".equals(value))
        // Java: value = "?";
        // Java: else if ("false".equalsIgnoreCase(value))
        // Java: value = "F";
        // Java: else if ("true".equalsIgnoreCase(value))
        // Java: value = "T";
        // Java: traceDataBuffer.append(" " + value);
        // Java: if (!iterator.hasNext())
        // Java: int dataLength = traceDataBuffer.length();
        // Java: int q = dataLength / 75;
        // Java: String[] traceDataStr = new String[q + 1];
        // Java: int i = 0;
        // Java: for(int start = 0; i < q + 1; ++i)
        // Java: if (start + 75 > dataLength - 1)
        // Java: traceDataStr[i] = traceDataBuffer.substring(start);
        // Java: else
        // Java: int commaIndex = traceDataBuffer.substring(start + 75, dataLength - 1).indexOf(44);
        // Java: traceDataStr[i] = traceDataBuffer.substring(start, start + 75 + commaIndex + 1);
        // Java: start = start + 75 + commaIndex + 1;
        // Java: return traceDataStr;
        // Java: traceDataBuffer.append(',');
        Vec::new()
    }

    // Java: String getInkElementType()
    pub fn getInkElementType(&self) -> String {
        // Java: return "Trace";
        "Trace".to_string()
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

    // Java: TraceDataElement getSelectedTraceDataByRange(String from, String to)
    pub fn getSelectedTraceDataByRange(&self, _from: &str, _to: &str) -> Result<Box<Trace>, String> {
        // Java: throw new InkMLException("Not supported getSelectedTraceDataByRange Yet");
        Err("Not supported getSelectedTraceDataByRange Yet".to_string())
    }

    // Java: Context getAssociatedContext()
    pub fn getAssociatedContext(&self) -> Option<&Context> {
        // Java: return this.associatedContext;
        self.associatedContext.as_ref()
    }

    // Java: boolean isRangeDataValid(int rangeIndex)
    pub fn isRangeDataValid(&self, rangeIndex: i32) -> bool {
        // Java: String aChannelName = ((Channel)this.getAssociatedTraceFormat().getChannelList().get(0)).getName();
        // Java: int nSamples = this.traceData.size();
        let nSamples = self.traceData.size() as i32;
        // Java: return rangeIndex >= 1 && rangeIndex <= nSamples;
        rangeIndex >= 1 && rangeIndex <= nSamples
    }

    // Java: void setAssociatedContext(Context associatedContext)
    pub fn setAssociatedContext(&mut self, associatedContext: Option<Context>) {
        // Java: if (null == associatedContext)
        if associatedContext.is_none() {
            // Java: KSLog.i(TAG, "Trace::setAssociatedContext, the given parameter context is null");
            KSLog::i(TAG, "Trace::setAssociatedContext, the given parameter context is null");
        // Java: else
        } else {
            // Java: this.associatedContext = new Context(associatedContext);
            self.associatedContext = associatedContext;
            // Java: this.initTraceDataStructure();
            self.initTraceDataStructure();
        }
    }

    // Java: String replaceString(String inputStr, String patternStr, String replacementStr)
    pub fn replaceString(&self, inputStr: &str, patternStr: &str, replacementStr: &str) -> String {
        // Java: Pattern pattern = Pattern.compile(patternStr);
        // Java: Matcher matcher = pattern.matcher(inputStr);
        // Java: String output = matcher.replaceAll(replacementStr);
        // Java: return output;
        inputStr.replace(patternStr, replacementStr)
    }

    // Java: String toString()
    pub fn to_string(&self) -> String {
        // Java: String id = this.getId();
        let id = self.getId();
        // Java: if (id == null)
        // Java: id = "";
        // Java: String biodata = "\n\tTrace - objectid = " + this.hashCode() + "\n\tId = " + id + "\n\tbrushRef = " + this.getBrushRef() + "\n\tcontextRef = " + this.getContextRef();
        let mut biodata = format!("\n\tTrace\n\tId = {}\n\tbrushRef = {}\n\tcontextRef = {}", id, self.getBrushRef(), self.getContextRef());
        // Java: if (null != this.associatedContext)
        if let Some(ref ctx) = self.associatedContext {
            // Java: IBrush brush = this.associatedContext.getBrush();
            // Java: if (null != brush)
            // Java: biodata = biodata + "\n\t associatedBrush = " + brush.getId() + " - " + brush;
            // Java: biodata = biodata + "\n\t associatedContext = " + this.associatedContext.getId() + " - " + this.associatedContext;
            biodata += &format!("\n\t associatedContext = {}", ctx.getId());
        }
        // Java: biodata = biodata + "\n";
        biodata += "\n";
        // Java: return biodata;
        biodata
    }

    // Java: String toInkML(Definitions definitions)
    pub fn toInkML_defs(&self, _definitions: &Definitions) -> String {
        // Java: StringBuilder strInkML = new StringBuilder("<trace");
        // Java: String id = this.getId();
        // Java: if (!"".equals(id))
        // Java: strInkML.append(" id=\"" + id + "\"");
        // Java: float timeOffset = this.getTimeOffset();
        // Java: if (-1.0F != timeOffset)
        // Java: strInkML.append(" timeOffset=\"" + timeOffset + "\"");
        // Java: String brushRef = this.getBrushRef();
        // Java: if (brushRef != null && brushRef.length() > 0)
        // Java: strInkML.append(" brushRef=\"" + brushRef + "\"");
        // Java: String contextRef = this.getContextRef();
        // Java: if (contextRef != null && contextRef.length() > 0)
        // Java: strInkML.append(" contextRef=\"" + contextRef + "\"");
        // Java: strInkML.append(">");
        // Java: NumberFormat nf = NumberFormat.getInstance(Locale.US);
        // Java: nf.setGroupingUsed(false);
        // Java: int channelSize = ((Object[])this.traceData.get(0)).length;
        // Java: Iterator<Object[]> iterator = this.traceData.iterator();
        // Java: while(true)
        // Java: Object[] traceFragment = iterator.next();
        // Java: for(int i = 0; i < channelSize; ++i)
        // Java: strInkML.append(" " + nf.format(traceFragment[i]));
        // Java: if (!iterator.hasNext())
        // Java: strInkML.append("</trace>");
        // Java: return strInkML.toString();
        // Java: strInkML.append(",");
        String::from("<trace />")
    }

    // Java: TraceDataList getTraceData()
    pub fn getTraceData(&self) -> &TraceDataList {
        // Java: return this.traceData;
        &self.traceData
    }

    // Java: void setTraceData(TraceDataList list)
    pub fn setTraceData(&mut self, list: TraceDataList) {
        // Java: this.traceData = list;
        self.traceData = list;
    }

    // Java: void setTimeOffset(long timeOffset)
    pub fn setTimeOffset(&mut self, timeOffset: i64) {
        // Java: this.attributesMap.put("timeOffset", String.valueOf(timeOffset));
        self.attributesMap.insert("timeOffset".to_string(), timeOffset.to_string());
    }

    // Java: void setId(String id)
    pub fn setId(&mut self, id: &str) {
        // Java: this.attributesMap.put("id", id);
        self.attributesMap.insert("id".to_string(), id.to_string());
    }

    // Java: void setAttribute(String attrName, String attrValue)
    pub fn setAttribute(&mut self, attrName: &str, attrValue: &str) {
        // Java: this.attributesMap.put(attrName, attrValue);
        self.attributesMap.insert(attrName.to_string(), attrValue.to_string());
    }

    // Java: void processTraceElement(String traceDataStr, Context currentContext, Definitions definitions)
    pub fn processTraceElement(&mut self, _traceDataStr: &str, _currentContext: Option<&Context>, _definitions: &Definitions) {
        // Java: if (null == this.associatedContext)
        // Java: this.resolveAssociatedContext(currentContext, definitions);
        // Java: String ctxRef = (String)this.attributesMap.get("contextRef");
        // Java: if (null == ctxRef)
        // Java: if (null != this.parentTraceGroup)
        // Java: ctxRef = this.parentTraceGroup.getContextRef();
        // Java: else
        // Java: ctxRef = "#" + currentContext.getId();
        // Java: this.attributesMap.put("contextRef", ctxRef);
        // Java: String continution = "";
        // Java: Continuation continuationAttr = Trace.Continuation.NONE;
        // Java: try
        // Java: continution = (String)this.attributesMap.get("continution");
        // Java: if (null != continution)
        // Java: continuationAttr = Trace.Continuation.valueOf(continution);
        // Java: catch (IllegalArgumentException var25)
        // Java: KSLog.i(TAG, "The value = " + continution + "is illegeal for the 'continution' attribute of trace element.");
        // Java: String priorRefStr = (String)this.attributesMap.get("priorRef");
        // Java: if (null == priorRefStr)
        // Java: if (Trace.Continuation.middle == continuationAttr || Trace.Continuation.end == continuationAttr)
        // Java: throw new InkMLException("Problem in the definition of trace:" + this.toString() + "The continuation attribute is either 'middle'  or 'end', but no value is given for 'priorRef' attribute.");
        // Java: else if (Trace.Continuation.begin == continuationAttr)
        // Java: KSLog.i(TAG, "Problem in the definition of tarce:" + this.toString() + "The continuation attribute is either 'begin' , but a value is given for 'priorRef' attribute. The priorRef value is ignored.");
        // Java: HashMap<String, Object> variableValueMap = new HashMap();
        // Java: if (!"".equals(traceDataStr))
        // Java: KSLog.i(TAG, "Trace data read from XML Doc: " + traceDataStr);
        // Java: traceDataStr = traceDataStr.trim();
        // Java: traceDataStr = traceDataStr.replace('\n', ' ');
        // Java: traceDataStr = this.replaceString(traceDataStr, "'-", " '-");
        // Java: traceDataStr = this.replaceString(traceDataStr, "\"-", " \"-");
        // Java: traceDataStr = this.replaceString(traceDataStr, "-", " -");
        // Java: traceDataStr = this.replaceString(traceDataStr, "' -", "'-");
        // Java: traceDataStr = this.replaceString(traceDataStr, "\" -", "\"-");
        // Java: traceDataStr = this.replaceString(traceDataStr, "[e|E] -", "e-");
        // Java: traceDataStr = this.replaceString(traceDataStr, "'", " '");
        // Java: traceDataStr = this.replaceString(traceDataStr, "\"", " \"");
        // Java: traceDataStr = this.replaceString(traceDataStr, "\\*", " \\* ");
        // Java: traceDataStr = this.replaceString(traceDataStr, "!", " !");
        // Java: KSLog.i(TAG, "Trace data after inserting required spaces for clarity: " + traceDataStr);
        // Java: StringTokenizer st = new StringTokenizer(traceDataStr, ",");
        // Java: int nTraceSamples = st.countTokens();
        // Java: if (0 != nTraceSamples)
        // Java: TraceFormat tf = this.associatedContext.getTraceFormat();
        // Java: ArrayList<Channel> channelList = tf.getChannelList();
        // Java: int nChannel = channelList.size();
        // Java: String[] channelNames = new String[nChannel];
        // Java: Channel.ChannelType[] channelTypes = new Channel.ChannelType[nChannel];
        // Java: this.lastPrefixMap = new HashMap();
        // Java: for(int i = 0; i < nChannel; ++i)
        // Java: Channel chn = (Channel)channelList.get(i);
        // Java: String channelName = chn.getName();
        // Java: Channel.ChannelType type = chn.getChannelType();
        // Java: channelNames[i] = channelName;
        // Java: channelTypes[i] = type;
        // Java: if (null != this.priorReferredTrace)
        // Java: this.lastPrefixMap.put(channelName, this.priorReferredTrace.getLastPrefixOf(channelName));
        // Java: else
        // Java: this.lastPrefixMap.put(channelName, Trace.TracePrefix.NONE);
        // Java: if (type == Channel.ChannelType.DECIMAL || type == Channel.ChannelType.INTEGER)
        // Java: variableValueMap.put(channelName, new Float(0.0F));
        // Java: Object[] lastFragment = null;
        // Java: for(int index = 0; st.hasMoreTokens(); ++index)
        // Java: Object[] traceFragment = new Object[nChannel];
        // Java: String traceSampleData = st.nextToken().trim();
        // Java: StringTokenizer token = new StringTokenizer(traceSampleData, " ");
        // Java: for(int valueIndex = 0; valueIndex < nChannel; ++valueIndex)
        // Java: String channelValue = null;
        // Java: if (!token.hasMoreTokens())
        // Java: Channel chn = (Channel)channelList.get(valueIndex);
        // Java: if (!chn.isIntermittent())
        // Java: throw new InkMLException("Error: Value must be given to all regular Channels.\nLocation: Value is not given for Channel " + chn.getName() + " in sample point index " + index + " [range starts from 0].");
        // Java: if (0 == index)
        // Java: channelValue = chn.getDefaultValue();
        // Java: else
        // Java: channelValue = "*";
        // Java: else
        // Java: channelValue = token.nextToken().trim();
        // Java: Object previousDoubleDiffVal = 0;
        // Java: int channelIndex = valueIndex % nChannel;
        // Java: this.processAndStoreChannelValue(traceFragment, channelIndex, lastFragment, channelNames[channelIndex], channelValue, channelTypes[channelIndex], index, variableValueMap, previousDoubleDiffVal);
        // Java: this.traceData.add(traceFragment);
        // Java: lastFragment = traceFragment;
        // Java: if (!"".equals(this.getId()))
        // Java: definitions.addToIndirectChildrenMap((InkElement)this);
    }

    // Java: void validateRange(String channelValue, String channelName)
    pub fn validateRange(&self, _channelValue: &str, _channelName: &str) {
    }

    // Java: public HashMap<String, String> getAttributesMap()
    pub fn getAttributesMap(&self) -> &HashMap<String, String> {
        // Java: return this.attributesMap;
        &self.attributesMap
    }

    // Java: void writeXML(InkMLWriter writer)
    pub fn writeXML(&self, writer: &mut InkMLWriter) {
        // Java: writer.writeStartTag("trace", this.attributesMap);
        writer.writeStartTag("trace", Some(&self.attributesMap));
        // Java: writer.incrementTagLevel();
        writer.incrementTagLevel();
        // Java: String[] traceDataStr = this.getTraceDataAsString();
        let traceDataStr = self.getTraceDataAsString();
        // Java: for(int i = 0; i < traceDataStr.length; ++i)
        for s in &traceDataStr {
            // Java: writer.writeCharacters(traceDataStr[i]);
        }
        // Java: writer.decrementTagLevel();
        writer.decrementTagLevel();
        // Java: writer.writeEndTag("trace");
        writer.writeEndTag("trace");
    }

    // Java: void printTrace()
    pub fn printTrace(&self) {
        // Java: KSLog.i(TAG, "<trace>");
        KSLog::i(TAG, "<trace>");
        // Java: String[] arr = this.getTraceDataAsString();
        let arr = self.getTraceDataAsString();
        // Java: for(int i = 0; i < arr.length; ++i)
        for s in &arr {
            // Java: KSLog.i(TAG, " " + arr[i]);
            KSLog::i(TAG, &format!(" {}", s));
        }
        // Java: KSLog.i(TAG, "</trace>");
        KSLog::i(TAG, "</trace>");
    }

    // Java: Trace clone()
    pub fn clone_trace(&self) -> Trace {
        // Java: Trace ret = new Trace();
        let mut ret = Trace::Trace();
        // Java: if (null != this.associatedContext)
        if let Some(ref ctx) = self.associatedContext {
            // Java: ret.associatedContext = this.associatedContext.clone();
            ret.associatedContext = Some(ctx.clone_context());
        }
        // Java: ret.attributesMap = this.cloneAttributesMap();
        ret.attributesMap = self.cloneAttributesMap();
        // Java: ret.continuation = this.continuation;
        ret.continuation = self.continuation;
        // Java: ret.lastPrefixMap = this.cloneLastPrefixMap();
        ret.lastPrefixMap = self.cloneLastPrefixMap();
        // Java: if (null != this.parentTraceGroup)
        if let Some(ref ptg) = self.parentTraceGroup {
            // Java: ret.parentTraceGroup = this.parentTraceGroup.clone();
            ret.parentTraceGroup = Some(Box::new(ptg.clone_tg()));
        }
        // Java: if (null != this.priorReferredTrace)
        if let Some(ref prt) = self.priorReferredTrace {
            // Java: ret.priorReferredTrace = this.priorReferredTrace.clone();
            ret.priorReferredTrace = Some(Box::new(prt.clone_trace()));
        }
        // Java: if (null != this.traceData)
        // Java: ret.traceData = this.traceData.clone();
        ret.traceData = self.traceData.clone_tdl();
        // Java: ret.mScaleX = this.mScaleX;
        ret.mScaleX = self.mScaleX;
        // Java: ret.mScaleY = this.mScaleY;
        ret.mScaleY = self.mScaleY;
        // Java: ret.mTranslateX = this.mTranslateX;
        ret.mTranslateX = self.mTranslateX;
        // Java: ret.mTranslateY = this.mTranslateY;
        ret.mTranslateY = self.mTranslateY;
        // Java: return ret;
        ret
    }

    // Java: private HashMap<String, TracePrefix> cloneLastPrefixMap()
    fn cloneLastPrefixMap(&self) -> Option<HashMap<String, TracePrefix>> {
        // Java: if (null == this.lastPrefixMap)
        if self.lastPrefixMap.is_none() {
            // Java: return null;
            return None;
        // Java: else
        } else {
            // Java: HashMap<String, TracePrefix> ret = new HashMap();
            let mut ret = HashMap::new();
            // Java: for(String k : this.lastPrefixMap.keySet())
            if let Some(ref map) = self.lastPrefixMap {
                for (k, e) in map {
                    // Java: TracePrefix e = (TracePrefix)this.lastPrefixMap.get(k);
                    // Java: ret.put(new String(k), e);
                    ret.insert(k.clone(), *e);
                }
            }
            // Java: return ret;
            Some(ret)
        }
    }

    // Java: private HashMap<String, String> cloneAttributesMap()
    fn cloneAttributesMap(&self) -> HashMap<String, String> {
        // Java: if (null == this.attributesMap)
        if self.attributesMap.is_empty() {
            // Java: return null;
            return HashMap::new();
        // Java: else
        } else {
            // Java: HashMap<String, String> ret = new HashMap();
            let mut ret = HashMap::new();
            // Java: for(String k : this.attributesMap.keySet())
            for (k, e) in &self.attributesMap {
                // Java: String e = (String)this.attributesMap.get(k);
                // Java: ret.put(new String(k), e);
                ret.insert(k.clone(), e.clone());
            }
            // Java: return ret;
            ret
        }
    }

    // Java: void transformByCanvasTransform(CanvasTransform paramCanvasTransform, boolean paramBoolean)
    pub fn transformByCanvasTransform(&mut self, paramCanvasTransform: &CanvasTransform, paramBoolean: bool) {
        // Java: Mapping localMapping = null;
        // Java: if (paramBoolean)
        if paramBoolean {
            // Java: localMapping = paramCanvasTransform.getForwardMapping();
        // Java: else
        } else {
            // Java: localMapping = paramCanvasTransform.getReverseMapping();
        }
    }

    // Java: ChannelProperties getChannelProperties()
    pub fn getChannelProperties(&self) -> Option<()> {
        // Java: if (this.getAssociatedContext() == null)
        if self.associatedContext.is_none() {
            // Java: return null;
            None
        // Java: else
        } else {
            // Java: return this.getAssociatedContext().getInkSource() == null ? null : this.getAssociatedContext().getInkSource().getChannelProperties();
            None
        }
    }

    // Java: RectF getRect()
    pub fn getRect(&self) -> RectF {
        // Java: return this.traceData.getRect();
        let (l, t, r, b) = self.traceData.getRect();
        RectF { left: l, top: t, right: r, bottom: b }
    }

    // Java: String toInkML()
    pub fn toInkML(&self) -> String {
        // Java: return null;
        String::new()
    }

    // Java: RectF getRectAfterTransform()
    pub fn getRectAfterTransform(&self) -> RectF {
        // Java: RectF rect = this.getRect();
        let mut rect = self.getRect();
        // Java: if (this.mScaleX != 0.0F && this.mScaleY != 0.0F)
        if self.mScaleX != 0.0 && self.mScaleY != 0.0 {
            // Java: rect.left /= this.mScaleX;
            rect.left /= self.mScaleX;
            // Java: rect.right /= this.mScaleX;
            rect.right /= self.mScaleX;
            // Java: rect.top /= this.mScaleY;
            rect.top /= self.mScaleY;
            // Java: rect.bottom /= this.mScaleY;
            rect.bottom /= self.mScaleY;
        }
        // Java: return rect;
        rect
    }

    // Java: boolean equalsTransform(Trace trace)
    pub fn equalsTransform(&self, trace: &Trace) -> bool {
        // Java: return this.mScaleX == trace.mScaleX && this.mScaleY == trace.mScaleY && this.mTranslateX == trace.mTranslateX && this.mTranslateY == trace.mTranslateY;
        self.mScaleX == trace.mScaleX && self.mScaleY == trace.mScaleY && self.mTranslateX == trace.mTranslateX && self.mTranslateY == trace.mTranslateY
    }
}
