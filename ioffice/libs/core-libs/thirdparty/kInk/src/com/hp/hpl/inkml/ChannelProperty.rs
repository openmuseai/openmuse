use std::collections::HashMap;
use super::InkMLSerializer::InkMLWriter;

// Java: public class ChannelProperty implements Cloneable
#[derive(Clone)]
pub struct ChannelProperty {
    // Java field: String channel
    pub(crate) channel: String,
    // Java field: String name
    pub(crate) name: String,
    // Java field: double value
    pub(crate) value: f64,
    // Java field: String units
    pub(crate) units: Option<String>,
}

// Java: public class ChannelProperty implements Cloneable
impl ChannelProperty {
    // Java: ChannelProperty()
    pub fn ChannelProperty() -> Self {
        // Java: super();
        Self {
            channel: String::new(),
            name: String::new(),
            value: 0.0,
            units: None,
        }
    }

    // Java: ChannelProperty(String channelName, String propertyName, double value)
    pub fn ChannelProperty_3(channelName: &str, propertyName: &str, value: f64) -> Self {
        // Java: this(channelName, propertyName, value, "unknown");
        Self::ChannelProperty_4(channelName, propertyName, value, "unknown")
    }

    // Java: ChannelProperty(String channelName, String propertyName, double value, String units)
    pub fn ChannelProperty_4(channelName: &str, propertyName: &str, value: f64, units: &str) -> Self {
        // Java: super();
        // Java: this.channel = channelName;
        // Java: this.name = propertyName;
        // Java: this.value = value;
        // Java: this.units = units;
        Self {
            channel: channelName.to_string(),
            name: propertyName.to_string(),
            value,
            units: Some(units.to_string()),
        }
    }

    // Java: String getChannel()
    pub fn getChannel(&self) -> &str {
        // Java: return this.channel;
        &self.channel
    }

    // Java: void setChannel(String channel)
    pub fn setChannel(&mut self, channel: &str) {
        // Java: this.channel = channel;
        self.channel = channel.to_string();
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
    pub fn getUnits(&self) -> Option<&str> {
        // Java: return this.units;
        self.units.as_deref()
    }

    // Java: void setUnits(String units)
    pub fn setUnits(&mut self, units: &str) {
        // Java: this.units = units;
        self.units = Some(units.to_string());
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
    pub fn writeXML(&self, _writer: &mut InkMLWriter) {
        // Java: HashMap<String, String> attr = new HashMap();
        let mut attr: HashMap<String, String> = HashMap::new();
        // Java: attr.put("channel", this.channel);
        attr.insert("channel".to_string(), self.channel.clone());
        // Java: attr.put("name", this.name);
        attr.insert("name".to_string(), self.name.clone());
        // Java: attr.put("value", String.valueOf(this.value));
        attr.insert("value".to_string(), self.value.to_string());
        // Java: if (!"".equals(this.units))
        if let Some(ref units) = self.units {
            // Java: attr.put("units", this.units);
            attr.insert("units".to_string(), units.clone());
        }
        // Java: writer.writeEmptyStartTag("channelProperty", attr);
    }

    // Java: String toInkML()
    pub fn toInkML(&self) -> String {
        // Java: return this.units == null ? String.format("<channelProperty channel=\"%s\" name=\"%s\" value=\"%s\" />", this.channel, this.name, this.value) : String.format("<channelProperty channel=\"%s\" name=\"%s\" value=\"%s\" units=\"%s\" />", this.channel, this.name, this.value, this.units);
        if self.units.is_none() {
            format!("<channelProperty channel=\"{}\" name=\"{}\" value=\"{}\" />", self.channel, self.name, self.value)
        } else {
            format!("<channelProperty channel=\"{}\" name=\"{}\" value=\"{}\" units=\"{}\" />", self.channel, self.name, self.value, self.units.as_ref().unwrap())
        }
    }

    // Java: boolean isResolution()
    pub fn isResolution(&self) -> bool {
        // Java: return "resolution".equals(this.name);
        self.name == "resolution"
    }

    // Java: ChannelProperty clone()
    pub fn clone_cp(&self) -> ChannelProperty {
        // Java: ChannelProperty ret = new ChannelProperty();
        let mut ret = ChannelProperty::ChannelProperty();
        // Java: if (null != this.channel)
        if !self.channel.is_empty() {
            // Java: ret.channel = new String(this.channel);
            ret.channel = self.channel.clone();
        }
        // Java: if (null != this.name)
        if !self.name.is_empty() {
            // Java: ret.name = new String(this.name);
            ret.name = self.name.clone();
        }
        // Java: if (null != this.units)
        if self.units.is_some() {
            // Java: ret.units = new String(this.units);
            ret.units = self.units.clone();
        }
        // Java: ret.value = this.value;
        ret.value = self.value;
        // Java: return ret;
        ret
    }
}
