// Java: import cn.wps.base.assertion.KSAssert
// Java: import com.isf.utils.BytesReader
// Java: import com.isf.utils.Multibytecoding
use super::utils::BytesReader::BytesReader;
use super::utils::Multibytecoding::Multibytecoding;
use super::TaggedStructure::TaggedStructure;
use super::Metrics::Metrics;
use super::Metric::Metric;
use super::StrokeInfo::StrokeInfo;
use super::IsfReaderContext::IsfReaderContext;

// Java: public class TagMetricBlock extends TaggedStructure
pub struct TagMetricBlock {
    // Java field: Metrics mMetrics
    mMetrics: Metrics,
    pub(crate) base: TaggedStructure,
}

// Java: public class TagMetricBlock extends TaggedStructure
impl TagMetricBlock {
    // Java: TagMetricBlock(BytesReader reader, int tagName, IsfReaderContext context)
    pub fn TagMetricBlock(reader: &mut BytesReader, tagName: i32, context: &mut IsfReaderContext) -> Self {
        // Java: super(reader, tagName, context);
        let base = TaggedStructure::TaggedStructure(reader, tagName);
        // Java: this.process();
        let mut s = Self {
            mMetrics: Metrics::Metrics(),
            base,
        };
        s.process(reader, context);
        s
    }

    // Java: Metrics getMetrics()
    pub fn getMetrics(&self) -> &Metrics {
        // Java: return this.mMetrics;
        &self.mMetrics
    }

    // Java: void process()
    fn process(&mut self, reader: &mut BytesReader, context: &mut IsfReaderContext) {
        // Java: KSAssert.assertNotNull("mReader should not be null!", this.mReader);
        // Java: this.mMetrics = new Metrics();
        self.mMetrics = Metrics::Metrics();
        // Java: int end = (int)((long)this.mReader.pos() + this.mSize);
        let end = (reader.pos() as i64 + self.base.mSize) as i32;
        // Java: while(this.mReader.pos() < end)
        while reader.pos() < end {
            // Java: int property = (int)Multibytecoding.decodeUInt(this.mReader);
            let property = Multibytecoding::decodeUInt(reader) as i32;
            // Java: long size = Multibytecoding.decodeUInt(this.mReader);
            let size = Multibytecoding::decodeUInt(reader);
            // Java: if (size < 7L)
            if size < 7 {
                // Java: for(int i = 0; (long)i < size; ++i)
                for _i in 0..size {
                    // Java: this.mReader.getByte();
                    reader.getByte();
                }
            // Java: else
            } else {
                // Java: Metric metric = new Metric();
                let mut metric = Metric::Metric();
                // Java: metric.mType = property;
                metric.mType = property;
                // Java: metric.mMin = Multibytecoding.decodeInt(this.mReader);
                metric.mMin = Multibytecoding::decodeInt(reader);
                // Java: metric.mMax = Multibytecoding.decodeInt(this.mReader);
                metric.mMax = Multibytecoding::decodeInt(reader);
                // Java: metric.mUnits = (byte)this.mReader.getByte();
                metric.mUnits = reader.getByte() as i32;
                // Java: metric.mResolution = Multibytecoding.decodeFloat(this.mReader);
                metric.mResolution = Multibytecoding::decodeFloat(reader);
                // Java: switch (property)
                match property {
                    // Java: case 56:
                    56 => {
                        // Java: if (null == this.mContext.mCurrentStrokeInfo)
                        if context.mCurrentStrokeInfo.is_none() {
                            // Java: this.mContext.mCurrentStrokeInfo = new StrokeInfo();
                            context.mCurrentStrokeInfo = Some(StrokeInfo::StrokeInfo());
                        }
                        // Java: this.mContext.mCurrentStrokeInfo.mHasPressureData = true;
                        if let Some(ref mut si) = context.mCurrentStrokeInfo {
                            si.mHasPressureData = true;
                        }
                        // Java: case 50:
                        // Java: case 51:
                        // Java: this.mMetrics.mItems.put(property, metric);
                        self.mMetrics.mItems.insert(property, metric);
                        // Java: break;
                    }
                    // Java: case 50:
                    // Java: case 51:
                    50 | 51 => {
                        // Java: this.mMetrics.mItems.put(property, metric);
                        self.mMetrics.mItems.insert(property, metric);
                        // Java: break;
                    }
                    // Java: default:
                    _ => {
                        // Java: for(int i = 0; (long)i < size; ++i)
                        for _i in 0..size {
                            // Java: this.mReader.getByte();
                            reader.getByte();
                        }
                    }
                }
            }
        }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: KSAssert.assertNotNull("mMetrics should not be null!", this.mMetrics);
        // Java: StringBuilder sb = new StringBuilder();
        let mut sb = String::new();
        // Java: sb.append("TAG_METRIC_BLOCK\n");
        sb.push_str("TAG_METRIC_BLOCK\n");
        // Java: sb.append(this.mMetrics.toString());
        sb.push_str(&self.mMetrics.toString());
        // Java: return sb.toString();
        sb
    }

    // Java: int type()
    pub fn r#type(&self) -> i32 {
        // Java: return 25;
        25
    }
}
