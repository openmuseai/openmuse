#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.moffice.util.KSLog
use util::cn::wps::moffice::util::KSLog::KSLog;
// Java: import java.util.HashMap
use std::collections::HashMap;
use super::InkMLSerializer::InkMLWriter;

// Java field: String TAG
const TAG: &str = "Channel";

// Java: public enum ChannelType
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChannelType {
    // Java: DECIMAL
    DECIMAL,
    // Java: INTEGER
    INTEGER,
    // Java: BOOLEAN
    BOOLEAN,
}

// Java: public enum ChannelType
impl ChannelType {
    // Java: ChannelType()
    pub fn from_str(s: &str) -> ChannelType {
        match s.to_uppercase().as_str() {
            "INTEGER" => ChannelType::INTEGER,
            "BOOLEAN" => ChannelType::BOOLEAN,
            _ => ChannelType::DECIMAL,
        }
    }
}

impl std::fmt::Display for ChannelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChannelType::DECIMAL => write!(f, "decimal"),
            ChannelType::INTEGER => write!(f, "integer"),
            ChannelType::BOOLEAN => write!(f, "boolean"),
        }
    }
}

// Java: public enum OrientationType
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrientationType {
    // Java: POSITIVE
    POSITIVE,
    // Java: NEGATIVE
    NEGATIVE,
}

// Java: public enum OrientationType
impl OrientationType {
    // Java: OrientationType()
    pub fn from_str(s: &str) -> OrientationType {
        match s {
            "NEGATIVE" => OrientationType::NEGATIVE,
            _ => OrientationType::POSITIVE,
        }
    }
}

impl std::fmt::Display for OrientationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrientationType::POSITIVE => write!(f, "POSITIVE"),
            OrientationType::NEGATIVE => write!(f, "NEGATIVE"),
        }
    }
}

// Java: public class Channel implements InkElement, Cloneable
pub struct Channel {
    // Java field: HashMap<String, String> attributesMap
    pub(crate) attributesMap: HashMap<String, String>,
    // Java field: boolean isIntermittent
    pub(crate) isIntermittent: bool,
}

// Java: public class Channel implements InkElement, Cloneable
impl Channel {
    // Java: Channel()
    pub fn Channel() -> Self {
        // Java: super();
        Self { attributesMap: HashMap::new(), isIntermittent: false }
    }

    // Java: Channel(String name, String id, ChannelType type, String min, String max, String units, OrientationType orientation, URI respectTo)
    pub fn Channel_full(name: &str, id: &str, channel_type: ChannelType, min: &str, max: &str, units: &str, orientation: OrientationType, respect_to: Option<&str>) -> Self {
        // Java: super();
        let mut s = Self { attributesMap: HashMap::new(), isIntermittent: false };
        // Java: this.attributesMap.put("name", name);
        s.attributesMap.insert("name".to_string(), name.to_string());
        // Java: this.attributesMap.put("id", id);
        s.attributesMap.insert("id".to_string(), id.to_string());
        // Java: this.attributesMap.put("type", type.toString());
        s.attributesMap.insert("type".to_string(), channel_type.to_string());
        // Java: this.attributesMap.put("min", min);
        s.attributesMap.insert("min".to_string(), min.to_string());
        // Java: this.attributesMap.put("max", max);
        s.attributesMap.insert("max".to_string(), max.to_string());
        // Java: this.attributesMap.put("units", units);
        s.attributesMap.insert("units".to_string(), units.to_string());
        // Java: this.attributesMap.put("orientation", orientation.toString());
        s.attributesMap.insert("orientation".to_string(), orientation.to_string());
        // Java: if (null != respectTo)
        if let Some(rt) = respect_to {
            // Java: this.attributesMap.put("respectTo", respectTo.toString());
            s.attributesMap.insert("respectTo".to_string(), rt.to_string());
        }
        s
    }

    // Java: Channel(String name, ChannelType type)
    pub fn Channel_name_type(name: &str, channel_type: ChannelType) -> Self {
        // Java: super();
        let mut s = Self { attributesMap: HashMap::new(), isIntermittent: false };
        // Java: this.attributesMap.put("name", name);
        s.attributesMap.insert("name".to_string(), name.to_string());
        // Java: this.attributesMap.put("type", type.toString());
        s.attributesMap.insert("type".to_string(), channel_type.to_string());
        // Java: this.attributesMap.put("orientation", Channel.OrientationType.POSITIVE.toString());
        s.attributesMap.insert("orientation".to_string(), OrientationType::POSITIVE.to_string());
        s
    }

    // Java: Channel(String name)
    pub fn Channel_name(name: &str) -> Self {
        // Java: super();
        let mut s = Self { attributesMap: HashMap::new(), isIntermittent: false };
        // Java: this.attributesMap.put("name", name);
        s.attributesMap.insert("name".to_string(), name.to_string());
        s
    }

    // Java: boolean isIntermittent()
    pub fn isIntermittent(&self) -> bool {
        // Java: return this.isIntermittent;
        self.isIntermittent
    }

    // Java: String getName()
    pub fn getName(&self) -> String {
        // Java: String value = (String)this.attributesMap.get("name");
        let value = self.attributesMap.get("name");
        // Java: return null == value ? "" : value;
        value.cloned().unwrap_or_default()
    }

    // Java: ChannelType getChannelType()
    pub fn getChannelType(&self) -> ChannelType {
        // Java: String value = (String)this.attributesMap.get("type");
        let value = self.attributesMap.get("type");
        // Java: if (null != value)
        if let Some(v) = value {
            // Java: ChannelType type = Channel.ChannelType.valueOf(value.toUpperCase());
            let ct = ChannelType::from_str(v);
            // Java: return type;
            ct
        // Java: else
        } else {
            // Java: return Channel.ChannelType.DECIMAL;
            ChannelType::DECIMAL
        }
    }

    // Java: String getUnits()
    pub fn getUnits(&self) -> String {
        // Java: String value = (String)this.attributesMap.get("units");
        let value = self.attributesMap.get("units");
        // Java: return null == value ? "" : value;
        value.cloned().unwrap_or_default()
    }

    // Java: String getDefaultValue()
    pub fn getDefaultValue(&self) -> String {
        // Java: String value = (String)this.attributesMap.get("default");
        let value = self.attributesMap.get("default");
        // Java: if (null == value)
        if value.is_none() {
            // Java: return this.getChannelType() != Channel.ChannelType.DECIMAL && this.getChannelType() != Channel.ChannelType.INTEGER ? "F" : "0";
            if self.getChannelType() != ChannelType::DECIMAL && self.getChannelType() != ChannelType::INTEGER {
                "F".to_string()
            } else {
                "0".to_string()
            }
        // Java: else
        } else {
            // Java: return value;
            value.unwrap().clone()
        }
    }

    // Java: String getId()
    pub fn getId(&self) -> String {
        // Java: String value = (String)this.attributesMap.get("id");
        let value = self.attributesMap.get("id");
        // Java: return null == value ? "" : value;
        value.cloned().unwrap_or_default()
    }

    // Java: OrientationType getOrientation()
    pub fn getOrientation(&self) -> OrientationType {
        // Java: String value = (String)this.attributesMap.get("orientation");
        let value = self.attributesMap.get("orientation");
        // Java: if (null != value)
        if let Some(v) = value {
            // Java: try
            {
                // Java: OrientationType type = Channel.OrientationType.valueOf(value);
                let ot = OrientationType::from_str(v);
                // Java: return type;
                return ot;
            }
            // Java: catch (Exception var3)
            {
                // Java: KSLog.w(TAG, "The 'orientation' attribute of channel " + this.getName() + " is invalid. Given value is " + value);
            }
        }
        // Java: return Channel.OrientationType.POSITIVE;
        OrientationType::POSITIVE
    }

    // Java: String getMin()
    pub fn getMin(&self) -> String {
        // Java: String value = (String)this.attributesMap.get("min");
        let value = self.attributesMap.get("min");
        // Java: return null == value ? "" : value;
        value.cloned().unwrap_or_default()
    }

    // Java: String getMax()
    pub fn getMax(&self) -> String {
        // Java: String value = (String)this.attributesMap.get("max");
        let value = self.attributesMap.get("max");
        // Java: return null == value ? "" : value;
        value.cloned().unwrap_or_default()
    }

    // Java: String getInkElementType()
    pub fn getInkElementType(&self) -> String {
        // Java: return "Channel";
        "Channel".to_string()
    }

    // Java: String getRespectTo()
    pub fn getRespectTo(&self) -> String {
        // Java: String value = (String)this.attributesMap.get("respectTo");
        let value = self.attributesMap.get("respectTo");
        // Java: return null == value ? "" : value;
        value.cloned().unwrap_or_default()
    }

    // Java: void setRespectTo(URI respectTo)
    pub fn setRespectTo_uri(&mut self, respectTo: &str) {
        // Java: if (null != respectTo)
        if !respectTo.is_empty() {
            // Java: this.attributesMap.put("respectTo", respectTo.getURIString());
            self.attributesMap.insert("respectTo".to_string(), respectTo.to_string());
        }
    }

    // Java: void setRespectTo(String respectTo)
    pub fn setRespectTo(&mut self, respectTo: &str) {
        // Java: if (null != respectTo && !"".equals(respectTo))
        if !respectTo.is_empty() {
            // Java: this.attributesMap.put("respectTo", respectTo);
            self.attributesMap.insert("respectTo".to_string(), respectTo.to_string());
        }
    }

    // Java: void setChannelType(ChannelType channelType)
    pub fn setChannelType_enum(&mut self, channelType: ChannelType) {
        // Java: if (null != channelType)
        {
            // Java: this.attributesMap.put("type", channelType.toString());
            self.attributesMap.insert("type".to_string(), channelType.to_string());
        }
    }

    // Java: void setChannelType(String channelType)
    pub fn setChannelType(&mut self, channelType: &str) {
        // Java: if (null != channelType && !"".equals(channelType))
        if !channelType.is_empty() {
            // Java: this.attributesMap.put("type", channelType.toUpperCase());
            self.attributesMap.insert("type".to_string(), channelType.to_uppercase());
        }
    }

    // Java: void setDefaultValue(Object defaultValue)
    pub fn setDefaultValue_obj(&mut self, defaultValue: &str) {
        // Java: if (null != defaultValue)
        if !defaultValue.is_empty() {
            // Java: this.attributesMap.put("default", defaultValue.toString());
            self.attributesMap.insert("default".to_string(), defaultValue.to_string());
        }
    }

    // Java: void setDefaultValue(String defaultValue)
    pub fn setDefaultValue(&mut self, defaultValue: &str) {
        // Java: if (null != defaultValue && !"".equals(defaultValue))
        if !defaultValue.is_empty() {
            // Java: this.attributesMap.put("default", defaultValue);
            self.attributesMap.insert("default".to_string(), defaultValue.to_string());
        }
    }

    // Java: void setId(String id)
    pub fn setId(&mut self, id: &str) {
        // Java: if (null != id && !"".equals(id))
        if !id.is_empty() {
            // Java: this.attributesMap.put("id", id);
            self.attributesMap.insert("id".to_string(), id.to_string());
        }
    }

    // Java: void setIntermittent(boolean isIntermittent)
    pub fn setIntermittent(&mut self, is_intermittent: bool) {
        // Java: this.isIntermittent = isIntermittent;
        self.isIntermittent = is_intermittent;
    }

    // Java: void setMax(String max)
    pub fn setMax(&mut self, max: &str) {
        // Java: if (null != max && !"".equals(max))
        if !max.is_empty() {
            // Java: this.attributesMap.put("max", max);
            self.attributesMap.insert("max".to_string(), max.to_string());
        }
    }

    // Java: void setMin(String min)
    pub fn setMin(&mut self, min: &str) {
        // Java: if (null != min && !"".equals(min))
        if !min.is_empty() {
            // Java: this.attributesMap.put("min", min);
            self.attributesMap.insert("min".to_string(), min.to_string());
        }
    }

    // Java: void setName(String name)
    pub fn setName(&mut self, name: &str) {
        // Java: if (null != name && !"".equals(name))
        if !name.is_empty() {
            // Java: this.attributesMap.put("name", name);
            self.attributesMap.insert("name".to_string(), name.to_string());
        }
    }

    // Java: void setOrientation(OrientationType orientation)
    pub fn setOrientation(&mut self, orientation: OrientationType) {
        // Java: if (null != orientation)
        {
            // Java: this.attributesMap.put("orientation", orientation.toString());
            self.attributesMap.insert("orientation".to_string(), orientation.to_string());
        }
    }

    // Java: void setUnits(String units)
    pub fn setUnits(&mut self, units: &str) {
        // Java: if (null != units && !"".equals(units))
        if !units.is_empty() {
            // Java: this.attributesMap.put("units", units);
            self.attributesMap.insert("units".to_string(), units.to_string());
        }
    }

    // Java: boolean equals2(Channel channel)
    pub fn equals2(&self, channel: Option<&Channel>) -> bool {
        // Java: if (channel == null)
        if channel.is_none() {
            // Java: return false;
            return false;
        // Java: else
        } else {
            let channel = channel.unwrap();
            // Java: boolean isEqual = true;
            let is_equal = true;
            // Java: if (!this.getName().equalsIgnoreCase(channel.getName()))
            if !self.getName().eq_ignore_ascii_case(&channel.getName()) {
                // Java: return false;
                return false;
            // Java: else if (this.getChannelType() != channel.getChannelType())
            } else if self.getChannelType() != channel.getChannelType() {
                // Java: return false;
                return false;
            // Java: else if (this.getOrientation() != channel.getOrientation())
            } else if self.getOrientation() != channel.getOrientation() {
                // Java: return false;
                return false;
            // Java: else if (this.isIntermittent() != channel.isIntermittent())
            } else if self.isIntermittent() != channel.isIntermittent() {
                // Java: return false;
                return false;
            // Java: else if (!this.getRespectTo().equals(channel.getRespectTo()))
            } else if self.getRespectTo() != channel.getRespectTo() {
                // Java: return false;
                return false;
            // Java: else if (!this.getDefaultValue().equals(channel.getDefaultValue()))
            } else if self.getDefaultValue() != channel.getDefaultValue() {
                // Java: return false;
                return false;
            // Java: else if (!this.getMin().equals(channel.getMin()))
            } else if self.getMin() != channel.getMin() {
                // Java: return false;
                return false;
            // Java: else if (!this.getMax().equals(channel.getMax()))
            } else if self.getMax() != channel.getMax() {
                // Java: return false;
                return false;
            // Java: else
            } else {
                // Java: return !this.getUnits().equals(channel.getUnits()) ? false : isEqual;
                if self.getUnits() != channel.getUnits() { false } else { is_equal }
            }
        }
    }

    // Java: String toInkML()
    pub fn toInkML(&self) -> String {
        // Java: String channelElement = "<channel ";
        let mut channelElement = "<channel ".to_string();
        // Java: String name = this.getName();
        let name = self.getName();
        // Java: if (!"".equals(name))
        if !name.is_empty() {
            // Java: channelElement = channelElement + "name='" + name + "' ";
            channelElement = channelElement + "name='" + &name + "' ";
        }
        // Java: String id = this.getId();
        let id = self.getId();
        // Java: if (!"".equals(id))
        if !id.is_empty() {
            // Java: channelElement = channelElement + "id='" + id + "' ";
            channelElement = channelElement + "id='" + &id + "' ";
        }
        // Java: String min = this.getMin();
        let min = self.getMin();
        // Java: if (!"".equals(min))
        if !min.is_empty() {
            // Java: channelElement = channelElement + "min='" + min + "' ";
            channelElement = channelElement + "min='" + &min + "' ";
        }
        // Java: String max = this.getMax();
        let max = self.getMax();
        // Java: if (!"".equals(max))
        if !max.is_empty() {
            // Java: channelElement = channelElement + "max='" + max + "' ";
            channelElement = channelElement + "max='" + &max + "' ";
        }
        // Java: String units = this.getUnits();
        let units = self.getUnits();
        // Java: if (!"".equals(units))
        if !units.is_empty() {
            // Java: channelElement = channelElement + "units='" + units + "' ";
            channelElement = channelElement + "units='" + &units + "' ";
        }
        // Java: String respectTo = this.getRespectTo();
        let respectTo = self.getRespectTo();
        // Java: if (!"".endsWith(respectTo))
        if !respectTo.is_empty() {
            // Java: channelElement = channelElement + "respectTo='" + respectTo + "' ";
            channelElement = channelElement + "respectTo='" + &respectTo + "' ";
        }
        // Java: String defaultValue = this.getDefaultValue();
        let defaultValue = self.getDefaultValue();
        // Java: if (!"".equals(defaultValue))
        if !defaultValue.is_empty() {
            // Java: channelElement = channelElement + "defaultValue='" + defaultValue + "' ";
            channelElement = channelElement + "defaultValue='" + &defaultValue + "' ";
        }
        // Java: ChannelType type = this.getChannelType();
        let ct = self.getChannelType();
        // Java: if (null != type)
        {
            // Java: channelElement = channelElement + "type='" + type.toString().toLowerCase() + "' ";
            channelElement = channelElement + "type='" + &ct.to_string() + "' ";
        }
        // Java: channelElement = channelElement + "/>";
        channelElement = channelElement + "/>";
        // Java: return channelElement;
        channelElement
    }

    // Java: void setAttribute(String attrName, String attrValue)
    pub fn setAttribute(&mut self, attrName: &str, attrValue: &str) -> Result<(), String> {
        // Java: KSLog.i(TAG, "adding Channel attribute " + attrName + " = " + attrValue);
        KSLog::i(TAG, &format!("adding Channel attribute {} = {}", attrName, attrValue));
        // Java: if (attrName.equals("type"))
        if attrName == "type" {
            // Java: try
            {
                // Java: Channel.ChannelType.valueOf(attrValue.toUpperCase());
                ChannelType::from_str(attrValue);
            }
            // Java: catch (Exception var4)
            {
                // Java: throw new InkMLException("The type attribute of channel " + this.getName() + " is invalid. Given value is " + attrValue);
            }
        }
        // Java: this.attributesMap.put(attrName, attrValue);
        self.attributesMap.insert(attrName.to_string(), attrValue.to_string());
        Ok(())
    }

    // Java: void writeXML(InkMLWriter writer)
    pub fn writeXML(&self, writer: &mut InkMLWriter) {
        // Java: writer.writeEmptyStartTag("channel", this.attributesMap);
        writer.writeStartTag("channel", Some(&self.attributesMap));
    }

    // Java: HashMap<String, String> getAttributesMap()
    pub fn getAttributesMap(&self) -> &HashMap<String, String> {
        // Java: return this.attributesMap;
        &self.attributesMap
    }

    // Java: Channel clone()
    pub fn clone_ch(&self) -> Channel {
        // Java: Channel ret = new Channel();
        let mut ret = Channel::Channel();
        // Java: if (null == this.attributesMap)
        if self.attributesMap.is_empty() {
            // Java: return ret;
            return ret;
        // Java: else
        } else {
            // Java: for(String k : this.attributesMap.keySet())
            for (k, v) in &self.attributesMap {
                // Java: String e = (String)this.attributesMap.get(k);
                // Java: ret.attributesMap.put(new String(k), new String(e));
                ret.attributesMap.insert(k.clone(), v.clone());
            }
            // Java: return ret;
            ret
        }
    }
}
