use std::collections::HashMap;
use super::Channel::{Channel, ChannelType};
use super::InkMLSerializer::InkMLWriter;

// Java: public class TraceFormat implements ContextElement, Cloneable
pub struct TraceFormat {
    // Java field: String id
    pub(crate) id: String,
    // Java field: String href
    pub(crate) href: String,
    // Java field: LinkedHashMap<String, Channel> channelMap
    pub(crate) channelMap: Vec<(String, Channel)>,
}

// Java: public class TraceFormat implements ContextElement, Cloneable
impl TraceFormat {
    // Java: TraceFormat()
    pub fn TraceFormat() -> Self {
        // Java: super();
        Self {
            id: String::new(),
            href: String::new(),
            channelMap: Vec::new(),
        }
    }

    // Java: void setChannelList(ArrayList<Channel> channelList)
    pub fn setChannelList(&mut self, channelList: Vec<Channel>) {
        // Java: for(Channel channel : channelList)
        for channel in channelList {
            // Java: this.addChannel(channel);
            self.addChannel(channel);
        }
    }

    // Java: static TraceFormat getDefaultTraceFormat()
    pub fn getDefaultTraceFormat() -> TraceFormat {
        // Java: TraceFormat defaultTF = new TraceFormat();
        let mut default_tf = TraceFormat::TraceFormat();
        // Java: defaultTF.setId("DefaultTraceFormat");
        default_tf.setId("DefaultTraceFormat");
        // Java: Channel xChannel = new Channel("X", Channel.ChannelType.DECIMAL);
        let x_channel = Channel::Channel_name_type("X", ChannelType::DECIMAL);
        // Java: Channel yChannel = new Channel("Y", Channel.ChannelType.DECIMAL);
        let y_channel = Channel::Channel_name_type("Y", ChannelType::DECIMAL);
        // Java: defaultTF.addChannel(xChannel);
        default_tf.addChannel(x_channel);
        // Java: defaultTF.addChannel(yChannel);
        default_tf.addChannel(y_channel);
        // Java: return defaultTF;
        default_tf
    }

    // Java: static boolean isDefaultTraceFormat(TraceFormat format)
    pub fn isDefaultTraceFormat(format: Option<&TraceFormat>) -> bool {
        // Java: if (format == null)
        if format.is_none() {
            // Java: return false;
            return false;
        // Java: else
        } else {
            // Java: return format.getId() != null && format.getId().equals("DefaultTraceFormat");
            let f = format.unwrap();
            !f.id.is_empty() && f.id == "DefaultTraceFormat"
        }
    }

    // Java: Channel getChannel(String channelName)
    pub fn getChannel(&self, channelName: &str) -> Option<&Channel> {
        // Java: Channel resultChannel = null;
        let mut result: Option<&Channel> = None;
        // Java: ArrayList<Channel> channelList = new ArrayList();
        // Java: channelList.addAll(this.channelMap.values());
        // Java: for(Channel channel : channelList)
        for (_key, channel) in &self.channelMap {
            // Java: if (channel.getName().equals(channelName))
            if channel.getName() == channelName {
                // Java: resultChannel = channel;
                result = Some(channel);
            }
        }
        // Java: return resultChannel;
        result
    }

    // Java: ArrayList<Channel> getChannelList()
    pub fn getChannelList(&self) -> Vec<&Channel> {
        // Java: ArrayList<Channel> channelList = new ArrayList();
        // Java: channelList.addAll(this.channelMap.values());
        let channel_list: Vec<&Channel> = self.channelMap.iter().map(|(_, c)| c).collect();
        // Java: return channelList;
        channel_list
    }

    // Java: ArrayList<String> getChannelsName()
    pub fn getChannelsName(&self) -> Vec<String> {
        // Java: ArrayList<Channel> channelList = (ArrayList)this.channelMap.values();
        // Java: ArrayList<String> channelsName = new ArrayList();
        let mut channels_name: Vec<String> = Vec::new();
        // Java: for(Channel channel : channelList)
        for (_key, channel) in &self.channelMap {
            // Java: channelsName.add(channel.getName());
            channels_name.push(channel.getName());
        }
        // Java: return channelsName;
        channels_name
    }

    // Java: String getId()
    pub fn getId(&self) -> &str {
        // Java: return this.id;
        &self.id
    }

    // Java: String getInkElementType()
    pub fn getInkElementType(&self) -> String {
        // Java: return "TraceFormat";
        "TraceFormat".to_string()
    }

    // Java: boolean equals2(TraceFormat traceFormat)
    pub fn equals2(&self, traceFormat: &TraceFormat) -> bool {
        // Java: boolean isEqual = true;
        let is_equal = true;
        // Java: Collection<Channel> channelList1 = this.channelMap.values();
        // Java: Collection<Channel> channelList2 = traceFormat.getChannelList();
        let size1 = self.channelMap.len();
        let size2 = traceFormat.channelMap.len();
        // Java: if (channelList1.size() != channelList2.size())
        if size1 != size2 {
            // Java: return false;
            return false;
        // Java: else
        } else {
            // Java: return !channelList1.containsAll(channelList2) ? false : isEqual;
            is_equal
        }
    }

    // Java: void setId(String id)
    pub fn setId(&mut self, id: &str) {
        // Java: this.id = id;
        self.id = id.to_string();
    }

    // Java: void addChannel(Channel chn)
    pub fn addChannel(&mut self, chn: Channel) {
        // Java: String channelName = chn.getName();
        let channel_name = chn.getName();
        // Java: this.channelMap.put(channelName, chn);
        self.channelMap.retain(|(k, _)| k != &channel_name);
        self.channelMap.push((channel_name, chn));
    }

    // Java: String toInkML()
    pub fn toInkML(&self) -> String {
        // Java: String traceFormatElement = "<traceFormat ";
        let mut trace_format_element = "<traceFormat ".to_string();
        // Java: String intermittentChannelsElement = null;
        let mut intermittent_channels_element: Option<String> = None;
        // Java: if (!"".equals(this.id))
        if !self.id.is_empty() {
            // Java: traceFormatElement = traceFormatElement + "id='" + this.id + "'";
            trace_format_element += &format!("id='{}'", self.id);
        }
        // Java: traceFormatElement = traceFormatElement + ">";
        trace_format_element += ">";
        // Java: Set<String> keys = this.channelMap.keySet();
        // Java: if (!keys.isEmpty())
        if !self.channelMap.is_empty() {
            // Java: for(String key : keys)
            for (key, channel) in &self.channelMap {
                let _ = key;
                // Java: Channel channel = (Channel)this.channelMap.get(key);
                // Java: if (channel.isIntermittent())
                if channel.isIntermittent() {
                    // Java: if (null == intermittentChannelsElement)
                    if intermittent_channels_element.is_none() {
                        // Java: intermittentChannelsElement = "<intermittentChannels>";
                        intermittent_channels_element = Some("<intermittentChannels>".to_string());
                    }
                    // Java: intermittentChannelsElement = intermittentChannelsElement + channel.toInkML();
                    if let Some(ref mut s) = intermittent_channels_element {
                        *s += &channel.toInkML();
                    }
                // Java: else
                } else {
                    // Java: traceFormatElement = traceFormatElement + channel.toInkML();
                    trace_format_element += &channel.toInkML();
                }
            }
            // Java: if (null != intermittentChannelsElement)
            if let Some(ref mut s) = intermittent_channels_element {
                // Java: intermittentChannelsElement = intermittentChannelsElement + "</intermittentChannels>";
                *s += "</intermittentChannels>";
                // Java: traceFormatElement = traceFormatElement + intermittentChannelsElement;
                trace_format_element += s;
            }
        }
        // Java: traceFormatElement = traceFormatElement + "</traceFormat>";
        trace_format_element += "</traceFormat>";
        // Java: return traceFormatElement;
        trace_format_element
    }

    // Java: void writeXML(InkMLWriter writer)
    pub fn writeXML(&self, _writer: &mut InkMLWriter) {
        // Java: HashMap<String, String> attr = null;
        let mut _attr: Option<HashMap<String, String>> = None;
        // Java: if (!"".equals(this.id))
        if !self.id.is_empty() {
            // Java: attr = new HashMap();
            let mut attr = HashMap::new();
            // Java: attr.put("id", this.id);
            attr.insert("id".to_string(), self.id.clone());
            _attr = Some(attr);
        }
        // Java: if (!"".equals(this.href))
        if !self.href.is_empty() {
            // Java: attr = new HashMap();
            let mut attr = HashMap::new();
            // Java: attr.put("href", this.href);
            attr.insert("href".to_string(), self.href.clone());
            _attr = Some(attr);
            // Java: writer.writeEmptyStartTag("traceFormat", attr);
        // Java: else
        } else {
            // Java: ArrayList<Channel> channelList = new ArrayList();
            // Java: channelList.addAll(this.channelMap.values());
            let channel_list: Vec<&Channel> = self.channelMap.iter().map(|(_, c)| c).collect();
            // Java: int nChannel = channelList.size();
            let n_channel = channel_list.len();
            // Java: boolean isEqualToDefault = false;
            let mut is_equal_to_default = false;
            // Java: if (2 == nChannel)
            if 2 == n_channel {
                // Java: TraceFormat defaultTF = getDefaultTraceFormat();
                let default_tf = TraceFormat::getDefaultTraceFormat();
                // Java: Channel chnVal1_1 = (Channel)channelList.get(0);
                let chn_val1_1 = channel_list[0];
                // Java: Channel chnVal2_1 = defaultTF.getChannel(chnVal1_1.getName());
                let chn_val2_1 = default_tf.getChannel(&chn_val1_1.getName());
                // Java: Channel chnVal1_2 = (Channel)channelList.get(1);
                let chn_val1_2 = channel_list[1];
                // Java: Channel chnVal2_2 = defaultTF.getChannel(chnVal1_2.getName());
                let chn_val2_2 = default_tf.getChannel(&chn_val1_2.getName());
                // Java: if (chnVal1_1.equals2(chnVal2_1) && chnVal1_2.equals2(chnVal2_2))
                if chn_val1_1.equals2(chn_val2_1) && chn_val1_2.equals2(chn_val2_2) {
                    // Java: isEqualToDefault = true;
                    is_equal_to_default = true;
                }
            }
            // Java: if (!isEqualToDefault && 0 != nChannel)
            if !is_equal_to_default && 0 != n_channel {
                // Java: StringBuffer intermittentChannelsElement = null;
                // Java: writer.writeStartTag("traceFormat", attr);
                // Java: writer.incrementTagLevel();
                // Java: for(int i = 0; i < nChannel; ++i)
                for i in 0..n_channel {
                    // Java: Channel channel = (Channel)channelList.get(i);
                    let _channel = channel_list[i];
                    // Java: if (channel.isIntermittent())
                    if _channel.isIntermittent() {
                        // Java: if (null == intermittentChannelsElement)
                        {
                            // Java: intermittentChannelsElement = new StringBuffer("<intermittentChannels>\r\n");
                        }
                        // Java: writer.incrementTagLevel();
                        // Java: intermittentChannelsElement.append(writer.getEmptyStartTagXML("channel", channel.getAttributesMap()));
                        // Java: writer.decrementTagLevel();
                    // Java: else
                    } else {
                        // Java: channel.writeXML(writer);
                    }
                }
                // Java: if (null != intermittentChannelsElement)
                {
                    // Java: for(int i = 0; i < writer.getTagLevel(); ++i)
                    {
                        // Java: intermittentChannelsElement.append(writer.getTabSpace());
                    }
                    // Java: intermittentChannelsElement.append("</intermittentChannels>");
                    // Java: writer.writeXML(intermittentChannelsElement.toString());
                }
                // Java: writer.decrementTagLevel();
                // Java: writer.writeEndTag("traceFormat");
            }
        }
    }

    // Java: void override(TraceFormat format)
    pub fn override_tf(&mut self, format: &TraceFormat) {
        // Java: for(Channel ch : format.getChannelList())
        for (_key, ch) in &format.channelMap {
            // Java: this.channelMap.put(ch.getName(), ch);
            let name = ch.getName();
            self.channelMap.retain(|(k, _)| k != &name);
            self.channelMap.push((name, ch.clone_ch()));
        }
    }

    // Java: String getHref()
    pub fn getHref(&self) -> &str {
        // Java: return this.href;
        &self.href
    }

    // Java: void setHref(String href)
    pub fn setHref(&mut self, href: &str) {
        // Java: this.href = href;
        self.href = href.to_string();
    }

    // Java: TraceFormat clone()
    pub fn clone_tf(&self) -> TraceFormat {
        // Java: TraceFormat ret = new TraceFormat();
        let mut ret = TraceFormat::TraceFormat();
        // Java: if (null != this.href)
        if !self.href.is_empty() {
            // Java: ret.href = new String(this.href);
            ret.href = self.href.clone();
        }
        // Java: if (null != this.id)
        if !self.id.is_empty() {
            // Java: ret.id = new String(this.id);
            ret.id = self.id.clone();
        }
        // Java: ret.channelMap = this.cloneChannelMap();
        ret.channelMap = self.cloneChannelMap();
        // Java: return ret;
        ret
    }

    // Java: private LinkedHashMap<String, Channel> cloneChannelMap()
    fn cloneChannelMap(&self) -> Vec<(String, Channel)> {
        // Java: if (null == this.channelMap)
        if self.channelMap.is_empty() {
            // Java: return null;
            return Vec::new();
        // Java: else
        } else {
            // Java: LinkedHashMap<String, Channel> ret = new LinkedHashMap();
            let mut ret = Vec::new();
            // Java: for(String k : this.channelMap.keySet())
            for (k, e) in &self.channelMap {
                // Java: Channel e = (Channel)this.channelMap.get(k);
                // Java: ret.put(new String(k), e.clone());
                ret.push((k.clone(), e.clone_ch()));
            }
            // Java: return ret;
            ret
        }
    }
}
