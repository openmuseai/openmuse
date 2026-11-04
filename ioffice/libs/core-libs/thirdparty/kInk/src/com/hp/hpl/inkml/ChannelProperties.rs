use super::ChannelProperty::ChannelProperty;
use super::InkMLSerializer::InkMLWriter;

// Java: public class ChannelProperties implements Cloneable
pub struct ChannelProperties {
    // Java field: ArrayList<ChannelProperty> ChannelPropertyList
    pub(crate) ChannelPropertyList: Option<Vec<ChannelProperty>>,
}

// Java: public class ChannelProperties implements Cloneable
impl ChannelProperties {
    // Java: ChannelProperties()
    pub fn ChannelProperties() -> Self {
        // Java: super();
        Self {
            ChannelPropertyList: Some(Vec::new()),
        }
    }

    // Java: void addChannelProperty(ChannelProperty property)
    pub fn addChannelProperty(&mut self, property: ChannelProperty) {
        // Java: if (null == this.ChannelPropertyList)
        if self.ChannelPropertyList.is_none() {
            // Java: this.ChannelPropertyList = new ArrayList();
            self.ChannelPropertyList = Some(Vec::new());
        }
        // Java: this.ChannelPropertyList.add(property);
        self.ChannelPropertyList.as_mut().unwrap().push(property);
    }

    // Java: void removeChannelProperty(String channelName, String propertyName)
    pub fn removeChannelProperty(&mut self, channelName: &str, propertyName: &str) {
        // Java: if (null != this.ChannelPropertyList)
        if self.ChannelPropertyList.is_some() {
            // Java: this.ChannelPropertyList = new ArrayList();
            // Java: Iterator<ChannelProperty> itr = this.ChannelPropertyList.iterator();
            // Java: while(itr.hasNext())
            if let Some(ref mut list) = self.ChannelPropertyList {
                list.retain(|cp| {
                    // Java: ChannelProperty cp = (ChannelProperty)itr.next();
                    // Java: if (channelName.equalsIgnoreCase(cp.getChannel()) && propertyName.equalsIgnoreCase(cp.getName()))
                    if channelName.eq_ignore_ascii_case(cp.getChannel()) && propertyName.eq_ignore_ascii_case(cp.getName()) {
                        // Java: itr.remove();
                        false
                    } else {
                        true
                    }
                });
            }
        }
    }

    // Java: void setChannelProperty(String channelName, String propertyName, double propertyValue)
    pub fn setChannelProperty(&mut self, channelName: &str, propertyName: &str, propertyValue: f64) {
        // Java: if (null != this.ChannelPropertyList)
        if self.ChannelPropertyList.is_some() {
            // Java: this.ChannelPropertyList = new ArrayList();
            // Java: for(ChannelProperty cp : this.ChannelPropertyList)
            if let Some(ref mut list) = self.ChannelPropertyList {
                for cp in list.iter_mut() {
                    // Java: if (channelName.equalsIgnoreCase(cp.getChannel()) && propertyName.equalsIgnoreCase(cp.getName()))
                    if channelName.eq_ignore_ascii_case(cp.getChannel()) && propertyName.eq_ignore_ascii_case(cp.getName()) {
                        // Java: cp.setValue(propertyValue);
                        cp.setValue(propertyValue);
                    }
                }
            }
        }
    }

    // Java: ChannelProperty getChannelPropertyByChannel(String channel)
    pub fn getChannelPropertyByChannel(&self, channel: &str) -> Option<&ChannelProperty> {
        // Java: int size = this.ChannelPropertyList.size();
        if let Some(ref list) = self.ChannelPropertyList {
            let size = list.len();
            // Java: for(int i = 0; i < size; ++i)
            for i in 0..size {
                // Java: if (channel.equals(((ChannelProperty)this.ChannelPropertyList.get(i)).channel))
                if channel == list[i].channel {
                    // Java: return (ChannelProperty)this.ChannelPropertyList.get(i);
                    return Some(&list[i]);
                }
            }
        }
        // Java: return null;
        None
    }

    // Java: void writeXML(InkMLWriter writer)
    pub fn writeXML(&self, _writer: &mut InkMLWriter) {
        // Java: if (0 != this.ChannelPropertyList.size())
        if let Some(ref list) = self.ChannelPropertyList {
            if 0 != list.len() {
                // Java: writer.writeStartTag("channelProperties", (HashMap)null);
                // Java: writer.incrementTagLevel();
                // Java: for(ChannelProperty cp : this.ChannelPropertyList)
                for cp in list {
                    // Java: cp.writeXML(writer);
                    cp.writeXML(_writer);
                }
                // Java: writer.decrementTagLevel();
                // Java: writer.writeEndTag("channelProperties");
            }
        }
    }

    // Java: String toInkML()
    pub fn toInkML(&self) -> String {
        // Java: String element = "<channelProperties>";
        let mut element = "<channelProperties>".to_string();
        // Java: String intermittentChannelsElement = null;
        // Java: for(ChannelProperty cp : this.ChannelPropertyList)
        if let Some(ref list) = self.ChannelPropertyList {
            for cp in list {
                // Java: element = element + cp.toInkML();
                element += &cp.toInkML();
            }
        }
        // Java: element = element + "</channelProperties>";
        element += "</channelProperties>";
        // Java: return element;
        element
    }

    // Java: ChannelProperties clone()
    pub fn clone_cp(&self) -> ChannelProperties {
        // Java: ChannelProperties ret = new ChannelProperties();
        let mut ret = ChannelProperties::ChannelProperties();
        // Java: if (null == this.ChannelPropertyList)
        if self.ChannelPropertyList.is_none() {
            // Java: return ret;
            return ret;
        // Java: else
        } else {
            // Java: ret.ChannelPropertyList = new ArrayList();
            ret.ChannelPropertyList = Some(Vec::new());
            // Java: int size = this.ChannelPropertyList.size();
            let list = self.ChannelPropertyList.as_ref().unwrap();
            let size = list.len();
            // Java: for(int i = 0; i < size; ++i)
            for i in 0..size {
                // Java: ChannelProperty e = (ChannelProperty)this.ChannelPropertyList.get(i);
                let e = &list[i];
                // Java: ret.ChannelPropertyList.add(e.clone());
                ret.ChannelPropertyList.as_mut().unwrap().push(e.clone_cp());
            }
            // Java: return ret;
            ret
        }
    }
}
