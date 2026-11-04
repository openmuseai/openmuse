#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.base.assertion.KSAssert
use util::base::assertion::KSAssert::KSAssert;
// Java: import cn.wps.moffice.util.KSLog
use util::moffice::util::KSLog::KSLog;
// Java: import com.hp.hpl.inkml.Brush
use crate::com::hp::hpl::inkml::Brush::Brush;
// Java: import com.hp.hpl.inkml.Channel
use crate::com::hp::hpl::inkml::Channel::Channel;
// Java: import com.hp.hpl.inkml.ChannelProperties
use crate::com::hp::hpl::inkml::ChannelProperties::ChannelProperties;
// Java: import com.hp.hpl.inkml.ChannelProperty
use crate::com::hp::hpl::inkml::ChannelProperty::ChannelProperty;
// Java: import com.hp.hpl.inkml.Context
use crate::com::hp::hpl::inkml::Context::Context;
// Java: import com.hp.hpl.inkml.Definitions
use crate::com::hp::hpl::inkml::Definitions::Definitions;
// Java: import com.hp.hpl.inkml.Ink
use crate::com::hp::hpl::inkml::Ink::Ink;
// Java: import com.hp.hpl.inkml.InkElement
// Java: import com.hp.hpl.inkml.InkMLException
// Java: import com.hp.hpl.inkml.InkSource
use crate::com::hp::hpl::inkml::InkSource::InkSource;
// Java: import com.hp.hpl.inkml.Trace
use crate::com::hp::hpl::inkml::Trace::Trace;
// Java: import com.hp.hpl.inkml.TraceDataList
use crate::com::hp::hpl::inkml::TraceDataList::TraceDataList;
// Java: import com.hp.hpl.inkml.TraceFormat
use crate::com::hp::hpl::inkml::TraceFormat::TraceFormat;
// Java: import com.isf.IsfReaderContext
use crate::com::isf::IsfReaderContext::IsfReaderContext;
// Java: import com.isf.Metric
use crate::com::isf::Metric::Metric;
// Java: import com.isf.Metrics
use crate::com::isf::Metrics::Metrics;
// Java: import com.isf.TagCustomUserIdTag
use crate::com::isf::TagCustomUserIdTag::TagCustomUserIdTag;
// Java: import com.isf.TagDidx
use crate::com::isf::TagDidx::TagDidx;
// Java: import com.isf.TagDrawAttrsBlock
use crate::com::isf::TagDrawAttrsBlock::TagDrawAttrsBlock;
// Java: import com.isf.TagDrawAttrsTable
use crate::com::isf::TagDrawAttrsTable::TagDrawAttrsTable;
// Java: import com.isf.TagMetricBlock
use crate::com::isf::TagMetricBlock::TagMetricBlock;
// Java: import com.isf.TagStroke
use crate::com::isf::TagStroke::TagStroke;
// Java: import com.isf.TagTransform
use crate::com::isf::TagTransform::TagTransform;
// Java: import com.isf.TagTransformIsotropicScale
use crate::com::isf::TagTransformIsotropicScale::TagTransformIsotropicScale;
// Java: import com.isf.TagTransformScaleAndTranslate
use crate::com::isf::TagTransformScaleAndTranslate::TagTransformScaleAndTranslate;
// Java: import com.isf.TaggedStructure
use crate::com::isf::TaggedStructure::TaggedStructure;
// Java: import com.isf.TaggedStructures
use crate::com::isf::TaggedStructures::TaggedStructures;
// Java: import com.isf.utils.BytesReader
use crate::com::isf::utils::BytesReader::BytesReader;
// Java: import com.isf.utils.Multibytecoding
use crate::com::isf::utils::Multibytecoding::Multibytecoding;
// Java: import java.io.InputStream
// Java: import java.util.Iterator

use crate::com::hp::hpl::inkml::Channel::ChannelType;

// Java field: String TAG
const TAG: &str = "IsfReader";

// Java: public class IsfReader
pub struct IsfReader {
    // Java field: BytesReader mBytesReader
    mBytesReader: BytesReader,
    // Java field: IsfReaderContext mContext
    mContext: IsfReaderContext,
}

// Java: public class IsfReader
impl IsfReader {
    // Java: IsfReader(InputStream is)
    pub fn IsfReader_stream(data: Vec<u8>) -> Self {
        // Java: super();
        // Java: KSAssert.assertNotNull("is should not be null!", is);
        // Java: this.mBytesReader = new BytesReader(is);
        // Java: this.mContext = new IsfReaderContext();
        Self {
            mBytesReader: BytesReader::BytesReader_from_stream(Box::new(std::io::Cursor::new(data))),
            mContext: IsfReaderContext::IsfReaderContext(),
        }
    }

    // Java: IsfReader(String path)
    pub fn IsfReader_path(path: &str) -> Self {
        // Java: super();
        // Java: KSAssert.assertNotNull("path should not be null!", path);
        // Java: this.mBytesReader = new BytesReader(path);
        // Java: this.mContext = new IsfReaderContext();
        Self {
            mBytesReader: BytesReader::BytesReader_from_path(path),
            mContext: IsfReaderContext::IsfReaderContext(),
        }
    }

    // Java: Ink read()
    pub fn read(&mut self) -> Ink {
        // Java: KSAssert.assertNotNull("mBytesReader should not be null!", this.mBytesReader);
        // Java: this.mBytesReader.getByte();
        self.mBytesReader.getByte();
        // Java: long nSize = Multibytecoding.decodeUInt(this.mBytesReader);
        let n_size = Multibytecoding::decodeUInt(&mut self.mBytesReader);
        // Java: TaggedStructures tags = null;
        // Java: if (!this.mBytesReader.isEOF())
        let tags = if !self.mBytesReader.isEOF() {
            // Java: tags = this.processTaggedStructure(nSize);
            Some(self.processTaggedStructure(n_size))
        } else {
            None
        };
        // Java: Ink ink = this.toInk(tags);
        let ink = self.toInk(tags);
        // Java: this.mBytesReader.close();
        self.mBytesReader.close();
        // Java: return ink;
        ink
    }

    // Java: TaggedStructures processTaggedStructure(long size)
    pub fn processTaggedStructure(&mut self, size: i64) -> TaggedStructures {
        // Java: TaggedStructures tags = new TaggedStructures(this.mBytesReader, size, this.mContext, this.mContext.mDefaultTaggedStructureFactory);
        use crate::com::isf::DefaultTaggedStructureFactory::DefaultTaggedStructureFactory;
        let factory = DefaultTaggedStructureFactory::DefaultTaggedStructureFactory();
        let tags = TaggedStructures::TaggedStructures(&mut self.mBytesReader, size, factory);
        // Java: return tags;
        tags
    }

    // Java: Ink toInk(TaggedStructures tags)
    pub fn toInk(&self, tags: Option<TaggedStructures>) -> Ink {
        // Java: KSAssert.assertNotNull("tags should not be null!", tags);
        // Java: Ink ink = new Ink();
        let mut ink = Ink::Ink();
        // Java: Context currentContext = ink.getCurrentContext();
        // Java: Trace trace = null;
        // Java: Brush brush = null;
        let mut brush: Option<Brush> = None;
        // Java: Iterator<TaggedStructure> iter = tags.iterator();
        // Java: TagTransformScaleAndTranslate tagTransformScaleAndTranslate = null;
        // Java: TagTransformIsotropicScale tagTransformIsotropicScale = null;
        // Java: TagDrawAttrsTable tagDrawAttrsTable = null;
        // Java: TagTransform tagTransform = null;
        // Java: TagMetricBlock tagMetricBlock = null;
        // Java: while(iter.hasNext())
        if let Some(_tags) = &tags {
            // Java: TaggedStructure tag = (TaggedStructure)iter.next();
            // Java: switch (tag.type())
            // Java: case 2:
            // Java: tagDrawAttrsTable = (TagDrawAttrsTable)tag;
            // Java: if (tagDrawAttrsTable.countTagDrawAttrsBlock() > 0)
            // Java: brush = this.toBrush(ink, tagDrawAttrsTable.item(0), 0);
            // Java: break;
            // Java: case 3:
            // Java: brush = this.toBrush(ink, (TagDrawAttrsBlock)tag, 0);
            // Java: break;
            // Java: case 9:
            // Java: long index = ((TagDidx)tag).mIndex;
            // Java: brush = this.toBrush(ink, tagDrawAttrsTable.item((int)index), (int)index);
            // Java: break;
            // Java: case 10:
            // Java: trace = this.toTrace(currentContext, brush, (TagStroke)tag, tagTransformScaleAndTranslate, tagTransformIsotropicScale, tagTransform, tagMetricBlock);
            // Java: ink.addTrace(trace);
            // Java: ink.getDefinitions().addToDirectChildrenMap((InkElement)trace.getAssociatedContext());
            // Java: break;
            // Java: case 16:
            // Java: tagTransform = (TagTransform)tag;
            // Java: break;
            // Java: case 17:
            // Java: tagTransformIsotropicScale = (TagTransformIsotropicScale)tag;
            // Java: break;
            // Java: case 21:
            // Java: tagTransformScaleAndTranslate = (TagTransformScaleAndTranslate)tag;
            // Java: break;
            // Java: case 25:
            // Java: tagMetricBlock = (TagMetricBlock)tag;
            // Java: break;
            // Java: case 104:
            // Java: TagCustomUserIdTag customUserIdTag = (TagCustomUserIdTag)tag;
            // Java: if (null != customUserIdTag)
            // Java: String userId = customUserIdTag.getUserId();
            // Java: ink.setDocID(userId);
        }
        // Java: return ink;
        ink
    }

    // Java: Brush toBrush(Ink ink, TagDrawAttrsBlock src, int index)
    pub fn toBrush(&self, ink: &mut Ink, src: &TagDrawAttrsBlock, index: i32) -> Brush {
        // Java: KSAssert.assertNotNull("ink should not be null!", ink);
        // Java: KSAssert.assertNotNull("src should not be null!", src);
        // Java: Brush brush = new Brush();
        let mut brush = Brush::Brush();
        // Java: brush.setId("mybrush" + index);
        brush.setId(&format!("mybrush{}", index));
        // Java: brush.setBrushProperty("shape", "round");
        brush.setBrushProperty_2("shape", "round");
        // Java: brush.setBrushProperty("type", "regular");
        brush.setBrushProperty_2("type", "regular");
        // Java: brush.setBrushProperty("fitToCurve", "1");
        brush.setBrushProperty_2("fitToCurve", "1");
        // Java: float width = (float)src.penWidth() / 1000.0F;
        let width = src.penWidth() as f32 / 1000.0;
        // Java: float height = (float)src.penHeight() / 1000.0F;
        let height = src.penHeight() as f32 / 1000.0;
        // Java: brush.setBrushProperty("width", String.valueOf(width), "cm");
        brush.setBrushProperty_2("width", &format!("{} cm", width));
        // Java: brush.setBrushProperty("height", String.valueOf(height), "cm");
        brush.setBrushProperty_2("height", &format!("{} cm", height));
        // Java: brush.setBrushProperty("color", "#" + coreColor2HexColor(src.penColor()));
        brush.setBrushProperty_2("color", &format!("#{}", Self::coreColor2HexColor(src.penColor())));
        // Java: brush.setBrushProperty("transparency", String.valueOf(src.penTransparency()));
        brush.setBrushProperty_2("transparency", &src.penTransparency().to_string());
        // Java: if (src.isRetangleTip())
        if src.isRetangleTip() {
            // Java: brush.setBrushProperty("tip", "rectangle");
            brush.setBrushProperty_2("tip", "rectangle");
        }
        // Java: if (src.isHighLighter())
        if src.isHighLighter() {
            // Java: brush.setBrushProperty("rasterOp", "maskPen");
            brush.setBrushProperty_2("rasterOp", "maskPen");
        }
        // Java: Definitions definitions = ink.getDefinitions();
        // Java: definitions.addToDirectChildrenMap((InkElement)brush);
        // Java: return brush;
        brush
    }

    // Java: static String coreColor2HexColor(int color)
    pub fn coreColor2HexColor(color: i32) -> String {
        // Java: int tmpColor = (color & 255) << 16;
        let mut tmp_color = (color & 255) << 16;
        // Java: tmpColor |= color & '\uff00';
        tmp_color |= color & 0xff00;
        // Java: tmpColor |= (color & 16711680) >> 16;
        tmp_color |= (color & 16711680) >> 16;
        // Java: StringBuilder ret = new StringBuilder(String.format("%06x", tmpColor));
        // Java: return ret.toString();
        format!("{:06x}", tmp_color)
    }

    // Java: Trace toTrace(Context currentContext, Brush brush, TagStroke src, TagTransformScaleAndTranslate tagTransformScaleAndTranslate, TagTransformIsotropicScale tagTransformIsotropicScale, TagTransform tagTransform, TagMetricBlock tagMetricBlock)
    pub fn toTrace(&self, current_context: &Context, brush: Option<&Brush>, src: &TagStroke, tag_transform_sat: Option<&TagTransformScaleAndTranslate>, tag_transform_is: Option<&TagTransformIsotropicScale>, tag_transform: Option<&TagTransform>, tag_metric_block: Option<&TagMetricBlock>) -> Trace {
        // Java: KSAssert.assertNotNull("currentContext should not be null!", currentContext);
        // Java: KSAssert.assertNotNull("brush should not be null!", brush);
        // Java: KSAssert.assertNotNull("src should not be null!", src);
        // Java: Trace trace = new Trace();
        let mut trace = Trace::Trace();
        // Java: trace.setAssociatedContext(currentContext);
        // Java: Context associContext = trace.getAssociatedContext();
        // Java: associContext.setId("mycontext");
        // Java: trace.setContextRef("#mycontext");
        trace.setContextRef("#mycontext");
        // Java: trace.setBrushRef("#" + brush.getId());
        // Java: trace.setAssociatedBrush(brush);
        // Java: float scaleX = 1.0F;
        let mut scale_x: f32 = 1.0;
        // Java: float scaleY = 1.0F;
        let mut scale_y: f32 = 1.0;
        // Java: float translateX = 0.0F;
        let mut translate_x: f32 = 0.0;
        // Java: float translateY = 0.0F;
        let mut translate_y: f32 = 0.0;
        // Java: if (null != tagTransformScaleAndTranslate)
        if let Some(sat) = tag_transform_sat {
            // Java: scaleX = tagTransformScaleAndTranslate.scaleX();
            scale_x = sat.scaleX();
            // Java: scaleY = tagTransformScaleAndTranslate.scaleY();
            scale_y = sat.scaleY();
            // Java: translateX = tagTransformScaleAndTranslate.dX();
            translate_x = sat.dX();
            // Java: translateY = tagTransformScaleAndTranslate.dY();
            translate_y = sat.dY();
        }
        // Java: if (null != tagTransformIsotropicScale)
        if let Some(tis) = tag_transform_is {
            // Java: scaleX = tagTransformIsotropicScale.getScaleAmount();
            scale_x = tis.getScaleAmount();
            // Java: scaleY = tagTransformIsotropicScale.getScaleAmount();
            scale_y = tis.getScaleAmount();
        }
        // Java: if (null != tagTransform)
        if let Some(tt) = tag_transform {
            // Java: scaleX = tagTransform.scaleX();
            scale_x = tt.scaleX();
            // Java: scaleY = tagTransform.scaleY();
            scale_y = tt.scaleY();
        }
        // Java: trace.mScaleX = scaleX;
        trace.mScaleX = scale_x;
        // Java: trace.mScaleY = scaleY;
        trace.mScaleY = scale_y;
        // Java: trace.mTranslateX = translateX;
        trace.mTranslateX = translate_x;
        // Java: trace.mTranslateY = translateY;
        trace.mTranslateY = translate_y;
        // Java: TraceDataList list = trace.getTraceData();
        // Java: list.setXYChannelType(Channel.ChannelType.DECIMAL);
        // Java: int count = src.count();
        let count = src.count();
        // Java: int fragmentSize = src.hasPressureData() ? 3 : 2;
        let fragment_size = if src.hasPressureData() { 3 } else { 2 };
        // Java: for(int i = 0; i < count; ++i)
        for i in 0..count {
            // Java: Float[] fragment = new Float[fragmentSize];
            let mut fragment = vec![0.0f32; fragment_size];
            // Java: fragment[0] = (float)src.item_x(i) * scaleX;
            fragment[0] = src.item_x(i as i32) as f32 * scale_x;
            // Java: fragment[1] = (float)src.item_y(i) * scaleY;
            fragment[1] = src.item_y(i as i32) as f32 * scale_y;
            // Java: if (src.hasPressureData())
            if src.hasPressureData() {
                // Java: fragment[2] = (float)src.item_f(i) * 1.0F;
                fragment[2] = src.item_f(i as i32) as f32 * 1.0;
            }
            // Java: list.add((Object[])fragment);
        }
        // Java: if (src.hasPressureData())
        if src.hasPressureData() {
            // Java: trace.getTraceData().setFIndex(2);
        }
        // Java: Context associatedContext = trace.getAssociatedContext();
        // Java: ChannelProperties channelProperties = new ChannelProperties();
        // Java: channelProperties.addChannelProperty(new ChannelProperty("X", "resolution", 28.36041, "1/cm"));
        // Java: channelProperties.addChannelProperty(new ChannelProperty("Y", "resolution", 28.34646, "1/cm"));
        // Java: if (src.hasPressureData())
        // Java: channelProperties.addChannelProperty(new ChannelProperty("F", "resolution", (double)1.0F, "1/dev"));
        // Java: InkSource inkSource = new InkSource();
        // Java: inkSource.setChannelProperties(channelProperties);
        // Java: associatedContext.setInkSource(inkSource);
        // Java: inkSource.setId("myinksource");
        // Java: TraceFormat traceFormat = new TraceFormat();
        // Java: Channel channelX = new Channel("X");
        // Java: try
        // Java: channelX.setAttribute("type", "decimal");
        // Java: channelX.setAttribute("max", "1920");
        // Java: channelX.setAttribute("units", "cm");
        // Java: traceFormat.addChannel(channelX);
        // Java: Channel channelY = new Channel("Y");
        // Java: channelY.setAttribute("type", "decimal");
        // Java: channelY.setAttribute("max", "1080");
        // Java: channelY.setAttribute("units", "cm");
        // Java: traceFormat.addChannel(channelY);
        // Java: if (src.hasPressureData())
        // Java: this.supplyFChannel(traceFormat, tagMetricBlock);
        // Java: catch (InkMLException e)
        // Java: KSLog.e(TAG, "InkMLException", e);
        // Java: inkSource.setTraceFormat(traceFormat);
        // Java: return trace;
        trace
    }

    // Java: void supplyFChannel(TraceFormat traceFormat, TagMetricBlock tagMetricBlock)
    pub fn supplyFChannel(&self, _trace_format: &mut TraceFormat, tag_metric_block: Option<&TagMetricBlock>) {
        // Java: Metrics metrics = null != tagMetricBlock ? tagMetricBlock.getMetrics() : null;
        // Java: Metric metric = null != metrics ? (Metric)metrics.mItems.get(56) : null;
        // Java: if (null == metric)
        // Java: Channel channelF = new Channel("F");
        // Java: channelF.setAttribute("type", "decimal");
        // Java: channelF.setAttribute("max", "32767");
        // Java: channelF.setAttribute("units", "dev");
        // Java: traceFormat.addChannel(channelF);
        // Java: else
        // Java: String max = Long.toString(metric.mMax);
        // Java: Channel channelF = new Channel("F");
        // Java: channelF.setAttribute("type", "decimal");
        // Java: channelF.setAttribute("max", max);
        // Java: channelF.setAttribute("units", "dev");
        // Java: traceFormat.addChannel(channelF);
    }
}
