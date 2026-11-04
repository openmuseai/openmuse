#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.moffice.util.KSLog
use util::cn::wps::moffice::util::KSLog::KSLog;
// Java: import java.util.ArrayList
// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import java.util.Iterator

use super::InkMLSerializer::InkMLWriter;
use super::TraceFormat::TraceFormat;
use super::ChannelProperties::ChannelProperties;
use super::ChannelProperty::ChannelProperty;
use super::URI::URI;

// Java field: String TAG
const TAG: &str = "InkSource";

// Java: public class SampleRate implements Cloneable
pub struct SampleRate {
    // Java field: boolean isUniform
    pub(crate) isUniform: bool,
    // Java field: double value
    pub(crate) value: f64,
}

// Java: public class SampleRate implements Cloneable
impl SampleRate {
    // Java: SampleRate(double value)
    pub fn SampleRate(value: f64) -> Self {
        // Java: super();
        // Java: this.value = value;
        Self { isUniform: true, value }
    }

    // Java: SampleRate(double value, boolean isUniform)
    pub fn SampleRate_full(value: f64, isUniform: bool) -> Self {
        // Java: super();
        // Java: this.value = value;
        // Java: this.isUniform = isUniform;
        Self { isUniform, value }
    }

    // Java: boolean isUniform()
    pub fn isUniform(&self) -> bool {
        // Java: return this.isUniform;
        self.isUniform
    }

    // Java: void setUniform(boolean isUniform)
    pub fn setUniform(&mut self, isUniform: bool) {
        // Java: this.isUniform = isUniform;
        self.isUniform = isUniform;
    }

    // Java: double getValue()
    pub fn getValue(&self) -> f64 {
        // Java: return this.value;
        self.value
    }

    // Java: void setValue(double value)
    pub fn setValue(&mut self, value: f64) {
        // Java: this.value = value;
        self.value = value;
    }

    // Java: void writeXML(InkMLWriter writer)
    pub fn writeXML(&self, writer: &mut InkMLWriter) {
        // Java: HashMap<String, String> attr = new HashMap();
        // Java: attr.put("uniform", String.valueOf(this.isUniform));
        // Java: attr.put("value", String.valueOf(this.value));
        // Java: writer.writeEmptyStartTag("sampleRate", attr);
    }

    // Java: SampleRate clone()
    pub fn clone_sr(&self) -> SampleRate {
        // Java: return InkSource.this.new SampleRate(this.value, this.isUniform);
        SampleRate::SampleRate_full(self.value, self.isUniform)
    }
}

// Java: public class Latency implements Cloneable
pub struct Latency {
    // Java field: double value
    pub(crate) value: f64,
}

// Java: public class Latency implements Cloneable
impl Latency {
    // Java: Latency(double value)
    pub fn Latency(value: f64) -> Self {
        // Java: super();
        // Java: this.value = value;
        Self { value }
    }

    // Java: double getValue()
    pub fn getValue(&self) -> f64 {
        // Java: return this.value;
        self.value
    }

    // Java: void setValue(double value)
    pub fn setValue(&mut self, value: f64) {
        // Java: this.value = value;
        self.value = value;
    }

    // Java: void writeXML(InkMLWriter writer)
    pub fn writeXML(&self, writer: &mut InkMLWriter) {
        // Java: HashMap<String, String> attr = new HashMap();
        // Java: attr.put("value", String.valueOf(this.value));
        // Java: writer.writeEmptyStartTag("latency", attr);
    }

    // Java: Latency clone()
    pub fn clone_latency(&self) -> Latency {
        // Java: Latency ret = InkSource.this.new Latency(this.value);
        // Java: return ret;
        Latency::Latency(self.value)
    }
}

// Java: public class ActiveArea implements Cloneable
pub struct ActiveArea {
    // Java field: String size
    pub(crate) size: String,
    // Java field: double hegiht
    pub(crate) hegiht: f64,
    // Java field: double width
    pub(crate) width: f64,
    // Java field: String units
    pub(crate) units: String,
}

// Java: public class ActiveArea implements Cloneable
impl ActiveArea {
    // Java: ActiveArea()
    pub fn ActiveArea() -> Self {
        // Java: super();
        Self { size: "unknown".to_string(), hegiht: -1.0, width: -1.0, units: "unknown".to_string() }
    }

    // Java: double getHegiht()
    pub fn getHegiht(&self) -> f64 {
        // Java: return this.hegiht;
        self.hegiht
    }

    // Java: void setHegiht(double hegiht)
    pub fn setHegiht(&mut self, hegiht: f64) {
        // Java: this.hegiht = hegiht;
        self.hegiht = hegiht;
    }

    // Java: String getSize()
    pub fn getSize(&self) -> &str {
        // Java: return this.size;
        &self.size
    }

    // Java: void setSize(String size)
    pub fn setSize(&mut self, size: &str) {
        // Java: this.size = size;
        self.size = size.to_string();
    }

    // Java: String getUnits()
    pub fn getUnits(&self) -> &str {
        // Java: return this.units;
        &self.units
    }

    // Java: void setUnits(String units)
    pub fn setUnits(&mut self, units: &str) {
        // Java: this.units = units;
        self.units = units.to_string();
    }

    // Java: double getWidth()
    pub fn getWidth(&self) -> f64 {
        // Java: return this.width;
        self.width
    }

    // Java: void setWidth(double width)
    pub fn setWidth(&mut self, width: f64) {
        // Java: this.width = width;
        self.width = width;
    }

    // Java: void writeXML(InkMLWriter writer)
    pub fn writeXML(&self, writer: &mut InkMLWriter) {
        // Java: HashMap<String, String> attributes = new HashMap();
        // Java: if (!this.size.equals("unknown"))
        // Java: attributes.put("size", this.size);
        // Java: if ((double)-1.0F != this.hegiht)
        // Java: attributes.put("hegiht", String.valueOf(this.hegiht));
        // Java: if ((double)-1.0F != this.width)
        // Java: attributes.put("width", String.valueOf(this.width));
        // Java: if (!this.units.equals("unknown"))
        // Java: attributes.put("units", this.units);
        // Java: writer.writeEmptyStartTag("activeArea", attributes);
    }

    // Java: ActiveArea clone()
    pub fn clone_aa(&self) -> ActiveArea {
        // Java: ActiveArea ret = InkSource.this.new ActiveArea();
        let mut ret = ActiveArea::ActiveArea();
        // Java: ret.hegiht = this.hegiht;
        ret.hegiht = self.hegiht;
        // Java: if (null != this.size)
        {
            // Java: ret.size = new String(this.size);
            ret.size = self.size.clone();
        }
        // Java: if (null != this.units)
        {
            // Java: ret.units = new String(this.units);
            ret.units = self.units.clone();
        }
        // Java: ret.width = this.width;
        ret.width = self.width;
        // Java: return ret;
        ret
    }
}

// Java: public class SourceProperty implements Cloneable
pub struct SourceProperty {
    // Java field: String name
    pub(crate) name: String,
    // Java field: double value
    pub(crate) value: f64,
    // Java field: String units
    pub(crate) units: String,
}

// Java: public class SourceProperty implements Cloneable
impl SourceProperty {
    // Java: SourceProperty()
    pub fn SourceProperty() -> Self {
        // Java: super();
        // Java: this.units = "";
        Self { name: String::new(), value: 0.0, units: String::new() }
    }

    // Java: SourceProperty(String name, double value, String units)
    pub fn SourceProperty_full(name: &str, value: f64, units: &str) -> Self {
        // Java: super();
        // Java: this.units = "";
        // Java: this.name = name;
        // Java: this.value = value;
        // Java: this.units = units;
        Self { name: name.to_string(), value, units: units.to_string() }
    }

    // Java: SourceProperty(String name, double value)
    pub fn SourceProperty_nv(name: &str, value: f64) -> Self {
        // Java: this(name, value, "unknown");
        Self::SourceProperty_full(name, value, "unknown")
    }

    // Java: String getName()
    pub fn getName(&self) -> &str {
        // Java: return this.name;
        &self.name
    }

    // Java: void setName(String name)
    pub fn setName(&mut self, name: &str) {
        // Java: this.name = name;
        self.name = name.to_string();
    }

    // Java: String getUnits()
    pub fn getUnits(&self) -> &str {
        // Java: return this.units;
        &self.units
    }

    // Java: void setUnits(String units)
    pub fn setUnits(&mut self, units: &str) {
        // Java: this.units = units;
        self.units = units.to_string();
    }

    // Java: double getValue()
    pub fn getValue(&self) -> f64 {
        // Java: return this.value;
        self.value
    }

    // Java: void setValue(double value)
    pub fn setValue(&mut self, value: f64) {
        // Java: this.value = value;
        self.value = value;
    }

    // Java: void writeXML(InkMLWriter writer)
    pub fn writeXML(&self, writer: &mut InkMLWriter) {
        // Java: HashMap<String, String> attr = new HashMap();
        // Java: attr.put("name", this.name);
        // Java: attr.put("value", String.valueOf(this.value));
        // Java: if (!"".equals(this.units))
        // Java: attr.put("units", this.units);
        // Java: writer.writeEmptyStartTag("sourceProperty", attr);
    }

    // Java: SourceProperty clone()
    pub fn clone_sp(&self) -> SourceProperty {
        // Java: SourceProperty ret = InkSource.this.new SourceProperty();
        let mut ret = SourceProperty::SourceProperty();
        // Java: if (null != this.name)
        {
            // Java: ret.name = new String(this.name);
            ret.name = self.name.clone();
        }
        // Java: ret.value = this.value;
        ret.value = self.value;
        // Java: if (null != this.units)
        {
            // Java: ret.units = this.units;
            ret.units = self.units.clone();
        }
        // Java: return ret;
        ret
    }
}

// Java: public class InkSource implements ContextElement, Cloneable
pub struct InkSource {
    // Java field: HashMap<String, String> attributesMap
    pub(crate) attributesMap: HashMap<String, String>,
    // Java field: TraceFormat traceFormat
    pub(crate) traceFormat: Option<TraceFormat>,
    // Java field: SampleRate sampleRate
    pub(crate) sampleRate: Option<SampleRate>,
    // Java field: ActiveArea activeArea
    pub(crate) activeArea: Option<ActiveArea>,
    // Java field: ArrayList<SourceProperty> sourcePropertyList
    pub(crate) sourcePropertyList: Option<Vec<SourceProperty>>,
    // Java field: ChannelProperties channelProperties
    pub(crate) channelProperties: Option<ChannelProperties>,
    // Java field: Latency latency
    pub(crate) latency: Option<Latency>,
}

// Java: public class InkSource implements ContextElement, Cloneable
impl InkSource {
    // Java: InkSource()
    pub fn InkSource() -> Self {
        // Java: super();
        // Java: this.attributesMap = new HashMap();
        // Java: this.traceFormat = TraceFormat.getDefaultTraceFormat();
        Self {
            attributesMap: HashMap::new(),
            traceFormat: Some(TraceFormat::getDefaultTraceFormat()),
            sampleRate: None,
            activeArea: None,
            sourcePropertyList: None,
            channelProperties: None,
            latency: None,
        }
    }

    // Java: InkSource(TraceFormat tf)
    pub fn InkSource_tf(tf: TraceFormat) -> Self {
        // Java: super();
        // Java: this.traceFormat = tf;
        Self {
            attributesMap: HashMap::new(),
            traceFormat: Some(tf),
            sampleRate: None,
            activeArea: None,
            sourcePropertyList: None,
            channelProperties: None,
            latency: None,
        }
    }

    // Java: static InkSource getDefaultInkSource()
    pub fn getDefaultInkSource() -> InkSource {
        // Java: InkSource defaultInkSource = new InkSource();
        let mut defaultInkSource = InkSource::InkSource();
        // Java: defaultInkSource.setId("DefaultInkSource");
        defaultInkSource.setId("DefaultInkSource");
        // Java: return defaultInkSource;
        defaultInkSource
    }

    // Java: void setAttribute(String attrName, String attrValue)
    pub fn setAttribute(&mut self, attrName: &str, attrValue: &str) {
        // Java: this.attributesMap.put(attrName, attrValue);
        self.attributesMap.insert(attrName.to_string(), attrValue.to_string());
    }

    // Java: void setId(String id)
    pub fn setId(&mut self, id: &str) {
        // Java: this.attributesMap.put("id", id);
        self.attributesMap.insert("id".to_string(), id.to_string());
    }

    // Java: String getId()
    pub fn getId(&self) -> String {
        // Java: return (String)this.attributesMap.get("id");
        self.attributesMap.get("id").cloned().unwrap_or_default()
    }

    // Java: String getInkElementType()
    pub fn getInkElementType(&self) -> String {
        // Java: return "InkSource";
        "InkSource".to_string()
    }

    // Java: void removeSourceProperty(String propertyName)
    fn removeSourceProperty(&mut self, propertyName: &str) {
        // Java: if (null != this.sourcePropertyList)
        if let Some(ref mut list) = self.sourcePropertyList {
            // Java: this.sourcePropertyList = new ArrayList();
            // Java: Iterator<SourceProperty> itr = this.sourcePropertyList.iterator();
            // Java: while(itr.hasNext())
            // Java: SourceProperty sp = (SourceProperty)itr.next();
            // Java: if (propertyName.equalsIgnoreCase(sp.getName()))
            // Java: itr.remove();
            list.retain(|sp| !propertyName.eq_ignore_ascii_case(sp.getName()));
        }
    }

    // Java: void setSourceProperty(String propertyName, double propertyValue)
    fn setSourceProperty(&mut self, propertyName: &str, propertyValue: f64) {
        // Java: if (null != this.sourcePropertyList)
        if let Some(ref mut list) = self.sourcePropertyList {
            // Java: this.sourcePropertyList = new ArrayList();
            // Java: Iterator<SourceProperty> itr = this.sourcePropertyList.iterator();
            // Java: boolean done = false;
            let mut done = false;
            // Java: while(itr.hasNext())
            for sp in list.iter_mut() {
                // Java: SourceProperty sp = (SourceProperty)itr.next();
                // Java: if (propertyName.equalsIgnoreCase(sp.getName()))
                if propertyName.eq_ignore_ascii_case(sp.getName()) {
                    // Java: sp.setValue(propertyValue);
                    sp.setValue(propertyValue);
                    // Java: done = true;
                    done = true;
                }
            }
            // Java: if (!done)
            if !done {
                // Java: this.sourcePropertyList.add(new SourceProperty(propertyName, propertyValue));
                list.push(SourceProperty::SourceProperty_nv(propertyName, propertyValue));
            }
        }
    }

    // Java: boolean equals2(InkSource inkSource)
    pub fn equals2(&self, inkSource: Option<&InkSource>) -> bool {
        // Java: if (inkSource == null)
        if inkSource.is_none() {
            // Java: return false;
            return false;
        // Java: else
        } else {
            // Java: KSLog.w(TAG, "The equals method implementtaion is not provided.\nIt returns 'true' for any two not null objects.");
            KSLog::w(TAG, "The equals method implementtaion is not provided.\nIt returns 'true' for any two not null objects.");
            // Java: return true;
            true
        }
    }

    // Java: String toInkML()
    pub fn toInkML(&self) -> String {
        // Java: String inkSourceResult = "<inkSource ";
        let mut inkSourceResult = "<inkSource ".to_string();
        // Java: String id = this.getId();
        let id = self.getId();
        // Java: if (null != id && !id.equals(""))
        if !id.is_empty() {
            // Java: inkSourceResult = inkSourceResult + "xml:id='" + id + "' ";
            inkSourceResult = inkSourceResult + "xml:id='" + &id + "' ";
        }
        // Java: String manufacturer = this.getManufacturer();
        let manufacturer = self.getManufacturer();
        // Java: if (null != manufacturer && !manufacturer.equals(""))
        if !manufacturer.is_empty() {
            // Java: inkSourceResult = inkSourceResult + "manufacturer='" + manufacturer + "' ";
            inkSourceResult = inkSourceResult + "manufacturer='" + &manufacturer + "' ";
        }
        // Java: String model = this.getModel();
        let model = self.getModel();
        // Java: if (null != model && !model.equals(""))
        if !model.is_empty() {
            // Java: inkSourceResult = inkSourceResult + "model='" + model + "' ";
            inkSourceResult = inkSourceResult + "model='" + &model + "' ";
        }
        // Java: String serialNo = this.getSerialNo();
        let serialNo = self.getSerialNo();
        // Java: if (null != serialNo && !serialNo.equals(""))
        if !serialNo.is_empty() {
            // Java: inkSourceResult = inkSourceResult + "serialNo='" + serialNo + "' ";
            inkSourceResult = inkSourceResult + "serialNo='" + &serialNo + "' ";
        }
        // Java: String specificationRef = this.getSpecificationRef().getURIString();
        let specificationRef = self.getSpecificationRef();
        // Java: if (null != specificationRef && !specificationRef.equals(""))
        if !specificationRef.is_empty() {
            // Java: inkSourceResult = inkSourceResult + "specificationRef='" + specificationRef + "' ";
            inkSourceResult = inkSourceResult + "specificationRef='" + &specificationRef + "' ";
        }
        // Java: String description = this.getDescription();
        let description = self.getDescription();
        // Java: if (null != description && !description.equals(""))
        if !description.is_empty() {
            // Java: inkSourceResult = inkSourceResult + "description='" + description + "' ";
            inkSourceResult = inkSourceResult + "description='" + &description + "' ";
        }
        // Java: inkSourceResult = inkSourceResult + ">";
        inkSourceResult = inkSourceResult + ">";
        // Java: if (this.traceFormat != null)
        if let Some(ref tf) = self.traceFormat {
            // Java: inkSourceResult = inkSourceResult + this.traceFormat.toInkML();
            inkSourceResult = inkSourceResult + &tf.toInkML();
        }
        // Java: if (this.channelProperties != null)
        if let Some(ref cp) = self.channelProperties {
            // Java: inkSourceResult = inkSourceResult + this.channelProperties.toInkML();
            inkSourceResult = inkSourceResult + &cp.toInkML();
        }
        // Java: inkSourceResult = inkSourceResult + "</inkSource>";
        inkSourceResult = inkSourceResult + "</inkSource>";
        // Java: return inkSourceResult;
        inkSourceResult
    }

    // Java: TraceFormat getTraceFormat()
    pub fn getTraceFormat(&self) -> &Option<TraceFormat> {
        // Java: return this.traceFormat;
        &self.traceFormat
    }

    // Java: void setTraceFormat(TraceFormat traceFormat)
    pub fn setTraceFormat(&mut self, traceFormat: TraceFormat) {
        // Java: this.traceFormat = traceFormat;
        self.traceFormat = Some(traceFormat);
    }

    // Java: ActiveArea getActiveArea()
    pub fn getActiveArea(&self) -> &Option<ActiveArea> {
        // Java: return this.activeArea;
        &self.activeArea
    }

    // Java: void setActiveArea(ActiveArea activeArea)
    pub fn setActiveArea(&mut self, activeArea: ActiveArea) {
        // Java: this.activeArea = activeArea;
        self.activeArea = Some(activeArea);
    }

    // Java: String getDescription()
    pub fn getDescription(&self) -> String {
        // Java: return (String)this.attributesMap.get("description");
        self.attributesMap.get("description").cloned().unwrap_or_default()
    }

    // Java: void setDescription(String description)
    pub fn setDescription(&mut self, description: &str) {
        // Java: this.attributesMap.put("description", description);
        self.attributesMap.insert("description".to_string(), description.to_string());
    }

    // Java: String getManufacturer()
    pub fn getManufacturer(&self) -> String {
        // Java: return (String)this.attributesMap.get("manufacturer");
        self.attributesMap.get("manufacturer").cloned().unwrap_or_default()
    }

    // Java: void setManufacturer(String manufacturer)
    pub fn setManufacturer(&mut self, manufacturer: &str) {
        // Java: this.attributesMap.put("manufacturer", manufacturer);
        self.attributesMap.insert("manufacturer".to_string(), manufacturer.to_string());
    }

    // Java: String getModel()
    pub fn getModel(&self) -> String {
        // Java: return (String)this.attributesMap.get("model");
        self.attributesMap.get("model").cloned().unwrap_or_default()
    }

    // Java: void setModel(String model)
    pub fn setModel(&mut self, model: &str) {
        // Java: this.attributesMap.put("model", model);
        self.attributesMap.insert("model".to_string(), model.to_string());
    }

    // Java: SampleRate getSampleRate()
    pub fn getSampleRate(&self) -> &Option<SampleRate> {
        // Java: return this.sampleRate;
        &self.sampleRate
    }

    // Java: void setSampleRate(SampleRate sampleRate)
    pub fn setSampleRate(&mut self, sampleRate: SampleRate) {
        // Java: this.sampleRate = sampleRate;
        self.sampleRate = Some(sampleRate);
    }

    // Java: String getSerialNo()
    pub fn getSerialNo(&self) -> String {
        // Java: return (String)this.attributesMap.get("serialNo");
        self.attributesMap.get("serialNo").cloned().unwrap_or_default()
    }

    // Java: void setSerialNo(String serialNo)
    pub fn setSerialNo(&mut self, serialNo: &str) {
        // Java: this.attributesMap.put("serialNo", serialNo);
        self.attributesMap.insert("serialNo".to_string(), serialNo.to_string());
    }

    // Java: URI getSpecificationRef()
    pub fn getSpecificationRef(&self) -> String {
        // Java: return new URI((String)this.attributesMap.get("specificationRef"));
        self.attributesMap.get("specificationRef").cloned().unwrap_or_default()
    }

    // Java: void setSpecificationRef(URI specificationRef)
    pub fn setSpecificationRef(&mut self, specificationRef: &str) {
        // Java: this.attributesMap.put("specificationRef", specificationRef.toString());
        self.attributesMap.insert("specificationRef".to_string(), specificationRef.to_string());
    }

    // Java: void setChannelProperties(ChannelProperties chnProbs)
    pub fn setChannelProperties(&mut self, chnProbs: ChannelProperties) {
        // Java: this.channelProperties = chnProbs;
        self.channelProperties = Some(chnProbs);
    }

    // Java: void addToChannelProperties(ChannelProperty chnProperty)
    pub fn addToChannelProperties(&mut self, chnProperty: ChannelProperty) {
        // Java: this.channelProperties.addChannelProperty(chnProperty);
        if let Some(ref mut cp) = self.channelProperties {
            cp.addChannelProperty(chnProperty);
        }
    }

    // Java: ChannelProperties getChannelProperties()
    pub fn getChannelProperties(&self) -> &Option<ChannelProperties> {
        // Java: return this.channelProperties;
        &self.channelProperties
    }

    // Java: void writeXML(InkMLWriter writer)
    pub fn writeXML(&self, writer: &mut InkMLWriter) {
        // Java: writer.writeStartTag("inkSource", this.attributesMap);
        writer.writeStartTag("inkSource", Some(&self.attributesMap));
        // Java: writer.incrementTagLevel();
        writer.incrementTagLevel();
        // Java: if (null != this.traceFormat)
        if let Some(ref tf) = self.traceFormat {
            // Java: this.traceFormat.writeXML(writer);
            tf.writeXML(writer);
        }
        // Java: if (null != this.sampleRate)
        if let Some(ref sr) = self.sampleRate {
            // Java: this.sampleRate.writeXML(writer);
            sr.writeXML(writer);
        }
        // Java: if (null != this.activeArea)
        if let Some(ref aa) = self.activeArea {
            // Java: this.activeArea.writeXML(writer);
            aa.writeXML(writer);
        }
        // Java: if (null != this.sourcePropertyList)
        if let Some(ref spl) = self.sourcePropertyList {
            // Java: for(SourceProperty sp : this.sourcePropertyList)
            for sp in spl {
                // Java: if (null != sp)
                {
                    // Java: sp.writeXML(writer);
                    sp.writeXML(writer);
                }
            }
        }
        // Java: if (null != this.channelProperties)
        if let Some(ref cp) = self.channelProperties {
            // Java: this.channelProperties.writeXML(writer);
            cp.writeXML(writer);
        }
        // Java: writer.decrementTagLevel();
        writer.decrementTagLevel();
        // Java: writer.writeEndTag("inkSource");
        writer.writeEndTag("inkSource");
    }

    // Java: void addSourceProperty(SourceProperty sourceProperty)
    pub fn addSourceProperty(&mut self, sourceProperty: SourceProperty) {
        // Java: if (null == this.sourcePropertyList)
        if self.sourcePropertyList.is_none() {
            // Java: this.sourcePropertyList = new ArrayList();
            self.sourcePropertyList = Some(Vec::new());
        }
        // Java: this.sourcePropertyList.add(sourceProperty);
        self.sourcePropertyList.as_mut().unwrap().push(sourceProperty);
    }

    // Java: void setLatency(Latency latency)
    pub fn setLatency(&mut self, latency: Latency) {
        // Java: this.latency = latency;
        self.latency = Some(latency);
    }

    // Java: Latency getLatency()
    pub fn getLatency(&self) -> &Option<Latency> {
        // Java: return this.latency;
        &self.latency
    }

    // Java: InkSource clone()
    pub fn clone_is(&self) -> InkSource {
        // Java: InkSource ret = new InkSource();
        let mut ret = InkSource::InkSource();
        // Java: if (null != this.activeArea)
        if let Some(ref aa) = self.activeArea {
            // Java: ret.activeArea = this.activeArea.clone();
            ret.activeArea = Some(aa.clone_aa());
        }
        // Java: ret.attributesMap = this.cloneAttributesMap();
        ret.attributesMap = self.cloneAttributesMap();
        // Java: if (null != this.channelProperties)
        if let Some(ref cp) = self.channelProperties {
            // Java: ret.channelProperties = this.channelProperties.clone();
            ret.channelProperties = Some(cp.clone_cp());
        }
        // Java: if (null != this.latency)
        if let Some(ref l) = self.latency {
            // Java: ret.latency = this.latency.clone();
            ret.latency = Some(l.clone_latency());
        }
        // Java: if (null != this.sampleRate)
        if let Some(ref sr) = self.sampleRate {
            // Java: ret.sampleRate = this.sampleRate.clone();
            ret.sampleRate = Some(sr.clone_sr());
        }
        // Java: ret.sourcePropertyList = this.cloneSroucePropertyList();
        ret.sourcePropertyList = self.cloneSroucePropertyList();
        // Java: if (null != this.traceFormat)
        if let Some(ref tf) = self.traceFormat {
            // Java: ret.traceFormat = this.traceFormat.clone();
            ret.traceFormat = Some(tf.clone_tf());
        }
        // Java: return ret;
        ret
    }

    // Java: private ArrayList<SourceProperty> cloneSroucePropertyList()
    fn cloneSroucePropertyList(&self) -> Option<Vec<SourceProperty>> {
        // Java: if (null == this.sourcePropertyList)
        if self.sourcePropertyList.is_none() {
            // Java: return null;
            return None;
        // Java: else
        } else {
            // Java: ArrayList<SourceProperty> ret = new ArrayList();
            let mut ret = Vec::new();
            // Java: int size = this.sourcePropertyList.size();
            let list = self.sourcePropertyList.as_ref().unwrap();
            // Java: for(int i = 0; i < size; ++i)
            for sp in list {
                // Java: SourceProperty e = (SourceProperty)this.sourcePropertyList.get(i);
                // Java: ret.add(e.clone());
                ret.push(sp.clone_sp());
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
            for (k, v) in &self.attributesMap {
                // Java: ret.put(new String(k), (String)this.attributesMap.get(k));
                ret.insert(k.clone(), v.clone());
            }
            // Java: return ret;
            ret
        }
    }
}
