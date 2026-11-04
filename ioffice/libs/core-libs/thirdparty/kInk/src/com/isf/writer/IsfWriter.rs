// Java: import cn.wps.base.assertion.KSAssert
use util::base::assertion::KSAssert;
// Java: import cn.wps.base.io.TypeConversion
use util::base::io::TypeConversion;
// Java: import cn.wps.moffice.util.KSLog
use util::moffice::util::KSLog;
// Java: import com.hp.hpl.inkml.Brush
use crate::com::hp::hpl::inkml::Brush;
// Java: import com.hp.hpl.inkml.Channel
use crate::com::hp::hpl::inkml::Channel;
// Java: import com.hp.hpl.inkml.Context
use crate::com::hp::hpl::inkml::Context;
// Java: import com.hp.hpl.inkml.Ink
use crate::com::hp::hpl::inkml::Ink;
// Java: import com.hp.hpl.inkml.InkMLException
// (not needed in Rust)
// Java: import com.hp.hpl.inkml.InkSource
use crate::com::hp::hpl::inkml::InkSource;
// Java: import com.hp.hpl.inkml.Trace
use crate::com::hp::hpl::inkml::Trace;
// Java: import com.hp.hpl.inkml.TraceFormat
use crate::com::hp::hpl::inkml::TraceFormat;
// Java: import com.isf.MetricScale
use crate::com::isf::MetricScale;
// Java: import com.isf.utils.Multibytecoding
use crate::com::isf::utils::Multibytecoding;
// Java: import java.io.ByteArrayOutputStream
use std::io::Write;
// Java: import java.io.IOException
// (Rust uses Result for IO errors)
// Java: import java.io.OutputStream
// (Using Box<dyn Write>)
// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import java.util.HashSet
use std::collections::HashSet;
// Java: import java.util.Iterator
// (Rust iterators are built-in)

// Java field: String TAG
const TAG: &str = "IsfWriter";
// Java field: int VERSION
const VERSION: i32 = 0;

// Java: public class IsfWriter
pub struct IsfWriter {
    // Java field: OutputStream mWriter
    mWriter: Vec<u8>,
    // Java field: Ink mInk
    mInk: Ink::Ink,
    // Java field: HashSet<Brush> mBrushWrited
    mBrushWrited: Vec<usize>,
    // Java field: HashMap<Trace, Brush> mBrushContext
    mBrushContext: HashMap<usize, bool>,
}

// Java: public class IsfWriter
impl IsfWriter {
    // Java: IsfWriter(Ink ink, OutputStream os)
    pub fn IsfWriter(ink: Ink::Ink, _os: Vec<u8>) -> Self {
      // Java: super();
      // Java: KSAssert.assertNotNull("ink should not be null!", ink);
      // Java: this.mInk = ink;
      // Java: this.mWriter = os;
      // Java: this.mBrushWrited = new HashSet();
      // Java: this.mBrushContext = new HashMap();
      Self {
          mWriter: _os,
          mInk: ink,
          mBrushWrited: Vec::new(),
          mBrushContext: HashMap::new(),
      }
    }

    // Java: void write()
    pub fn write(&mut self) {
      // Java: KSAssert.assertNotNull("mInk should not be null!", this.mInk);
      // Java: byte[] tagGUIDTable = this.writeGUIDTable();
      let tag_guid_table = self.writeGUIDTable();
      // Java: byte[] tagStrokes = this.writeTagStrokes();
      let tag_strokes = self.writeTagStrokes();
      // Java: byte[] tagDrawAttrsTable = this.writeTagDrawAttrsTable();
      let tag_draw_attrs_table = self.writeTagDrawAttrsTable();
      // Java: byte[] tagHimetricSize = this.writeTagHimetricSize();
      let tag_himetric_size = self.writeTagHimetricSize();
      // Java: byte[] tagMetricBlock = this.writeTagMetricBlock();
      let tag_metric_block = self.writeTagMetricBlock();
      // Java: byte[] tagTagTransformScaleAndTranslate = this.writeTagTransformScaleAndTranslate();
      let tag_transform = self.writeTagTransformScaleAndTranslate();
      // Java: byte[] tagCustomUserId = this.writeCustomUserId();
      let tag_custom_user_id = self.writeCustomUserId();
      // Java: try
      {
         // Java: this.mWriter.write(VERSION);
         let _ = self.mWriter.write(&[VERSION as u8]);
         // Java: this.mWriter.write(Multibytecoding.encodeUInt((long)(tagGUIDTable.length + tagStrokes.length + tagDrawAttrsTable.length + tagHimetricSize.length + tagMetricBlock.length + tagTagTransformScaleAndTranslate.length + tagCustomUserId.length)));
         let total_len = (tag_guid_table.len() + tag_strokes.len() + tag_draw_attrs_table.len() + tag_himetric_size.len() + tag_metric_block.len() + tag_transform.len() + tag_custom_user_id.len()) as i64;
         let _ = self.mWriter.write(&Multibytecoding::Multibytecoding::encodeUInt(total_len));
         // Java: this.mWriter.write(tagGUIDTable);
         let _ = self.mWriter.write(&tag_guid_table);
         // Java: this.mWriter.write(tagHimetricSize);
         let _ = self.mWriter.write(&tag_himetric_size);
         // Java: this.mWriter.write(tagMetricBlock);
         let _ = self.mWriter.write(&tag_metric_block);
         // Java: this.mWriter.write(tagDrawAttrsTable);
         let _ = self.mWriter.write(&tag_draw_attrs_table);
         // Java: this.mWriter.write(tagTagTransformScaleAndTranslate);
         let _ = self.mWriter.write(&tag_transform);
         // Java: this.mWriter.write(tagStrokes);
         let _ = self.mWriter.write(&tag_strokes);
         // Java: this.mWriter.write(tagCustomUserId);
         let _ = self.mWriter.write(&tag_custom_user_id);
      }
      // Java: catch (IOException e)
      // (errors handled inline with let _ =)
      {
         // Java: KSLog.e(TAG, "InkMLException", e);
      }
    }

    // Java: byte[] writeTagMetricBlock()
    pub fn writeTagMetricBlock(&self) -> Vec<u8> {
      // Java: ByteArrayOutputStream bos = new ByteArrayOutputStream();
      let mut bos: Vec<u8> = Vec::new();
      // Java: try
      {
         // Java: Iterator iter = this.mInk.getTraceIterator();
         let traces = self.mInk.getTraceIterator();
         // Java: while(iter.hasNext())
         for trace in &traces
         {
            // Java: Trace trace = (Trace)iter.next();
            // Java: byte[] buff = this.writeMetric(trace.getAssociatedContext());
            let buff = self.writeMetric(trace.getAssociatedContext());
            // Java: if (null != buff)
            if let Some(buff) = buff
            {
               // Java: bos.write(25);
               bos.push(25);
               // Java: bos.write(Multibytecoding.encodeUInt((long)buff.length));
               bos.extend_from_slice(&Multibytecoding::Multibytecoding::encodeUInt(buff.len() as i64));
               // Java: bos.write(buff);
               bos.extend_from_slice(&buff);
               // Java: break;
               break;
            }
         }
      }
      // Java: catch (Exception e)
      {
         // Java: KSLog.e(TAG, "InkMLException", e);
      }
      // Java: return bos.toByteArray();
      bos
    }

    // Java: byte[] writeMetric(Context context)
    pub fn writeMetric(&self, context: Option<&Context::Context>) -> Option<Vec<u8>> {
      // Java: if (null == context)
      let context = match context {
          None =>
      {
         // Java: return null;
         return None;
      }
          Some(c) => c,
      };
      // Java: else
      {
         // Java: ByteArrayOutputStream bos = new ByteArrayOutputStream();
         let mut bos: Vec<u8> = Vec::new();
         // Java: InkSource inkSource = context.getInkSource();
         let ink_source = context.getInkSource();
         // Java: if (null == inkSource)
         let ink_source = match ink_source {
             None =>
         {
            // Java: return null;
            return None;
         }
             Some(s) => s,
         };
         // Java: else
         {
            // Java: TraceFormat format = inkSource.getTraceFormat();
            let format = ink_source.getTraceFormat();
            // Java: if (null == format)
            let format = match format {
                None =>
            {
               // Java: return null;
               return None;
            }
                Some(ref f) => f,
            };
            // Java: else
            {
               // Java: Channel channel = format.getChannel("F");
               let channel = format.getChannel("F");
               // Java: if (null == channel)
               let channel = match channel {
                   None =>
               {
                  // Java: return null;
                  return None;
               }
                   Some(c) => c,
               };
               // Java: else
               {
                  // Java: String maxStr = channel.getMax();
                  let max_str = channel.getMax();
                  // Java: if (null == maxStr)
                  if max_str.is_empty()
                  {
                     // Java: return null;
                     return None;
                  }
                  // Java: else
                  {
                     // Java: Long max = TypeConversion.string2Long(maxStr);
                     let max = TypeConversion::TypeConversion::string2Long(&max_str);
                     // Java: if (null == max)
                     let max = match max {
                         None =>
                     {
                        // Java: return null;
                        return None;
                     }
                         Some(v) => v,
                     };
                     // Java: else
                     {
                        // Java: bos.write(Multibytecoding.encodeUInt(56L));
                        bos.extend_from_slice(&Multibytecoding::Multibytecoding::encodeUInt(56));
                        // Java: byte[] buff = this.writeMetricContent(max);
                        let buff = self.writeMetricContent(max);
                        // Java: bos.write(Multibytecoding.encodeUInt((long)buff.length));
                        bos.extend_from_slice(&Multibytecoding::Multibytecoding::encodeUInt(buff.len() as i64));
                        // Java: bos.write(buff);
                        bos.extend_from_slice(&buff);
                        // Java: return bos.toByteArray();
                        return Some(bos);
                     }
                  }
               }
            }
         }
      }
    }

    // Java: byte[] writeMetricContent(long max)
    pub fn writeMetricContent(&self, max: i64) -> Vec<u8> {
      // Java: ByteArrayOutputStream bos = new ByteArrayOutputStream();
      let mut bos: Vec<u8> = Vec::new();
      // Java: bos.write(Multibytecoding.encodeInt(0L));
      bos.extend_from_slice(&Multibytecoding::Multibytecoding::encodeInt(0));
      // Java: bos.write(Multibytecoding.encodeInt(max));
      bos.extend_from_slice(&Multibytecoding::Multibytecoding::encodeInt(max));
      // Java: bos.write((byte)MetricScale.UNIT_DEFAULT);
      bos.push(MetricScale::UNIT_DEFAULT as u8);
      // Java: bos.write(Multibytecoding.encodeFloat(0.0F));
      bos.extend_from_slice(&Multibytecoding::Multibytecoding::encodeFloat(0.0));
      // Java: return bos.toByteArray();
      bos
    }

    // Java: byte[] writeTagEmpty(int tagName)
    pub fn writeTagEmpty(&self, tag_name: i32) -> Vec<u8> {
      // Java: byte[] buff = new byte[]{(byte)tagName};
      let buff = vec![tag_name as u8];
      // Java: return buff;
      buff
    }

    // Java: byte[] writeTagStrokeDescBlock()
    pub fn writeTagStrokeDescBlock(&self) -> Vec<u8> {
      // Java: ByteArrayOutputStream bos = new ByteArrayOutputStream();
      let mut bos: Vec<u8> = Vec::new();
      // Java: try
      {
         // Java: bos.write(5);
         bos.push(5);
         // Java: bos.write(Multibytecoding.encodeUInt(1L));
         bos.extend_from_slice(&Multibytecoding::Multibytecoding::encodeUInt(1));
         // Java: bos.write(this.writeTagEmpty(56));
         bos.extend_from_slice(&self.writeTagEmpty(56));
      }
      // Java: catch (IOException e)
      {
         // Java: KSLog.e(TAG, "IOException", e);
      }
      // Java: return bos.toByteArray();
      bos
    }

    // Java: byte[] writeTagHimetricSize()
    pub fn writeTagHimetricSize(&self) -> Vec<u8> {
      // Java: KSAssert.assertNotNull("mInk should not be null!", this.mInk);
      // Java: byte[] buff = new byte[]{29, 4, -28, 2, -104, 4};
      let buff: Vec<u8> = vec![29, 4, 0xE4u8.wrapping_neg().wrapping_add(0xE4), 2, 0x98u8.wrapping_neg().wrapping_add(0x98), 4];
      // Java: return buff;
      vec![29u8, 4, 228u8 as u8, 2, 152u8 as u8, 4]
    }

    // Java: byte[] writeTagDrawAttrsTable()
    pub fn writeTagDrawAttrsTable(&mut self) -> Vec<u8> {
      // Java: KSAssert.assertNotNull("mInk should not be null!", this.mInk);
      // Java: byte[] content = this.writeTagDrawAttrsBlocks();
      let content = self.writeTagDrawAttrsBlocks();
      // Java: ByteArrayOutputStream bos = new ByteArrayOutputStream();
      let mut bos: Vec<u8> = Vec::new();
      // Java: try
      {
         // Java: bos.write(2);
         bos.push(2);
         // Java: bos.write(Multibytecoding.encodeUInt((long)content.length));
         bos.extend_from_slice(&Multibytecoding::Multibytecoding::encodeUInt(content.len() as i64));
         // Java: bos.write(content);
         bos.extend_from_slice(&content);
      }
      // Java: catch (IOException e)
      {
         // Java: KSLog.e(TAG, "IOException", e);
      }
      // Java: return bos.toByteArray();
      bos
    }

    // Java: byte[] writeTagDrawAttrsBlocks()
    pub fn writeTagDrawAttrsBlocks(&mut self) -> Vec<u8> {
      // Java: KSAssert.assertNotNull("mInk should not be null!", this.mInk);
      // Java: ByteArrayOutputStream bos = new ByteArrayOutputStream();
      let mut bos: Vec<u8> = Vec::new();
      // Java: try
      {
         // Java: Iterator iter = this.mInk.getTraceIterator();
         let traces = self.mInk.getTraceIterator();
         // Java: while(iter.hasNext())
         for (idx, trace) in traces.iter().enumerate()
         {
            // Java: Trace trace = (Trace)iter.next();
            // Java: Brush brush = (Brush)trace.getAssociatedBrush();
            let _brush = trace.getAssociatedBrush();
            // Java: if (this.mBrushWrited.contains(brush))
            if self.mBrushWrited.contains(&idx)
            {
               // Java: this.mBrushContext.put(trace, null);
               self.mBrushContext.insert(idx, false);
            }
            // Java: else
            else
            {
               // Java: this.mBrushContext.put(trace, brush);
               self.mBrushContext.insert(idx, true);
               // Java: this.mBrushWrited.add(brush);
               self.mBrushWrited.push(idx);
               // Java: byte[] content = this.writeTagDrawAttrsBlock(brush);
               let content = self.writeTagDrawAttrsBlock(trace.getAssociatedBrush());
               // Java: try
               {
                  // Java: bos.write(Multibytecoding.encodeUInt((long)content.length));
                  bos.extend_from_slice(&Multibytecoding::Multibytecoding::encodeUInt(content.len() as i64));
                  // Java: bos.write(content);
                  bos.extend_from_slice(&content);
               }
               // Java: catch (IOException e1)
               {
                  // Java: KSLog.e(TAG, "IOException", e1);
               }
            }
         }
      }
      // Java: catch (InkMLException e)
      {
         // Java: KSLog.e(TAG, "InkMLException", e);
      }
      // Java: return bos.toByteArray();
      bos
    }

    // Java: byte[] writeTagDrawAttrsBlock(Brush srcBrush)
    pub fn writeTagDrawAttrsBlock(&self, src_brush: Option<&Brush::Brush>) -> Vec<u8> {
      // Java: ByteArrayOutputStream bos = new ByteArrayOutputStream();
      let mut bos: Vec<u8> = Vec::new();
      // Java: try
      if let Some(_src_brush) = src_brush
      {
         // Java: com.hp.hpl.inkml.impl.Brush brush = com.hp.hpl.inkml.impl.Brush.fromInkML(srcBrush);
         let brush = crate::com::hp::hpl::inkml::r#impl::Brush::Brush::Brush();
         // Java: bos.write(Multibytecoding.encodeUInt(72L));
         bos.extend_from_slice(&Multibytecoding::Multibytecoding::encodeUInt(72));
         // Java: bos.write(Multibytecoding.encodeUInt(17L));
         bos.extend_from_slice(&Multibytecoding::Multibytecoding::encodeUInt(17));
         // Java: bos.write(Multibytecoding.encodeUInt(68L));
         bos.extend_from_slice(&Multibytecoding::Multibytecoding::encodeUInt(68));
         // Java: long color = (long)brush.getColor();
         let mut color: i64 = brush.getColor() as i64;
         // Java: color &= 16777215L;
         color &= 16777215;
         // Java: bos.write(Multibytecoding.encodeUInt((long)toBGR((int)color)));
         bos.extend_from_slice(&Multibytecoding::Multibytecoding::encodeUInt(Self::toBGR(color as i32) as i64));
         // Java: bos.write(Multibytecoding.encodeUInt(69L));
         bos.extend_from_slice(&Multibytecoding::Multibytecoding::encodeUInt(69));
         // Java: int width = (int)(brush.getWidth() * 1000.0F);
         let width = (brush.getWidth() * 1000.0) as i32;
         // Java: bos.write(Multibytecoding.encodeUInt((long)width));
         bos.extend_from_slice(&Multibytecoding::Multibytecoding::encodeUInt(width as i64));
         // Java: if (com.hp.hpl.inkml.impl.Brush.Shape.rectangle == brush.getShape())
         if crate::com::hp::hpl::inkml::r#impl::Brush::Shape::rectangle == brush.getShape()
         {
            // Java: bos.write(Multibytecoding.encodeUInt(71L));
            bos.extend_from_slice(&Multibytecoding::Multibytecoding::encodeUInt(71));
            // Java: bos.write(Multibytecoding.encodeUInt(1L));
            bos.extend_from_slice(&Multibytecoding::Multibytecoding::encodeUInt(1));
         }
         // Java: if (com.hp.hpl.inkml.impl.Brush.RasterOp.maskPen == brush.getRasterOp())
         if crate::com::hp::hpl::inkml::r#impl::Brush::RasterOp::maskPen == brush.getRasterOp()
         {
            // Java: bos.write(Multibytecoding.encodeUInt(87L));
            bos.extend_from_slice(&Multibytecoding::Multibytecoding::encodeUInt(87));
            // Java: bos.write(Multibytecoding.encodeUInt(9L));
            bos.extend_from_slice(&Multibytecoding::Multibytecoding::encodeUInt(9));
            // Java: byte[] buffer = new byte[]{0, 0, 0};
            let buffer: Vec<u8> = vec![0, 0, 0];
            // Java: bos.write(buffer);
            bos.extend_from_slice(&buffer);
         }
         // Java: bos.write(Multibytecoding.encodeUInt(70L));
         bos.extend_from_slice(&Multibytecoding::Multibytecoding::encodeUInt(70));
         // Java: int height = (int)(brush.getHeight() * 1000.0F);
         let height = (brush.getHeight() * 1000.0) as i32;
         // Java: bos.write(Multibytecoding.encodeUInt((long)height));
         bos.extend_from_slice(&Multibytecoding::Multibytecoding::encodeUInt(height as i64));
         // Java: bos.write(Multibytecoding.encodeUInt(80L));
         bos.extend_from_slice(&Multibytecoding::Multibytecoding::encodeUInt(80));
         // Java: bos.write(Multibytecoding.encodeUInt((long)(255 - brush.getTransparency())));
         bos.extend_from_slice(&Multibytecoding::Multibytecoding::encodeUInt((255 - brush.getTransparency()) as i64));
      }
      // Java: catch (IOException e)
      {
         // Java: KSLog.e(TAG, "IOException", e);
      }
      // Java: return bos.toByteArray();
      bos
    }

    // Java: static int toBGR(int color)
    pub fn toBGR(color: i32) -> i32 {
      // Java: int tmpColor = (color & 16711680) >> 16;
      let mut tmp_color = (color & 0x00FF0000) >> 16;
      // Java: tmpColor |= color & '\uff00';
      tmp_color |= color & 0x0000FF00u32 as i32;
      // Java: tmpColor |= (color & 255) << 16;
      tmp_color |= (color & 255) << 16;
      // Java: return tmpColor;
      tmp_color
    }

    // Java: byte[] writeTagTransformScaleAndTranslate()
    pub fn writeTagTransformScaleAndTranslate(&self) -> Vec<u8> {
      // Java: ByteArrayOutputStream bos = new ByteArrayOutputStream();
      let mut bos: Vec<u8> = Vec::new();
      // Java: try
      {
         // Java: Iterator iter = this.mInk.getTraceIterator();
         let traces = self.mInk.getTraceIterator();
         // Java: Trace lastTrace = null;
         let mut _last_trace: Option<&Trace::Trace> = None;
         // Java: while(iter.hasNext())
         for trace in &traces
         {
            // Java: Trace trace = (Trace)iter.next();
            // Java: if (null == lastTrace || !lastTrace.equalsTransform(trace))
            if _last_trace.is_none() || !_last_trace.unwrap().equalsTransform(trace)
            {
               // Java: this.writeTagTransformScaleAndTranslate(bos, trace);
               Self::writeTagTransformScaleAndTranslate_impl(&mut bos, trace);
               // Java: break;
               break;
            }
         }
      }
      // Java: catch (InkMLException e)
      {
         // Java: KSLog.e(TAG, "InkMLException", e);
      }
      // Java: return bos.toByteArray();
      bos
    }

    // Java: void writeTagTransformScaleAndTranslate(ByteArrayOutputStream bos, Trace trace)
    fn writeTagTransformScaleAndTranslate_impl(bos: &mut Vec<u8>, trace: &Trace::Trace) {
      // Java: KSAssert.assertNotNull("bos should not be null!", bos);
      // Java: KSAssert.assertNotNull("trace should not be null!", trace);
      // Java: try
      {
         // Java: bos.write(21);
         bos.push(21);
         // Java: bos.write(Multibytecoding.encodeFloat(1.0F));
         bos.extend_from_slice(&Multibytecoding::Multibytecoding::encodeFloat(1.0));
         // Java: bos.write(Multibytecoding.encodeFloat(1.0F));
         bos.extend_from_slice(&Multibytecoding::Multibytecoding::encodeFloat(1.0));
         // Java: bos.write(Multibytecoding.encodeFloat(trace.mTranslateX));
         bos.extend_from_slice(&Multibytecoding::Multibytecoding::encodeFloat(trace.mTranslateX));
         // Java: bos.write(Multibytecoding.encodeFloat(trace.mTranslateY));
         bos.extend_from_slice(&Multibytecoding::Multibytecoding::encodeFloat(trace.mTranslateY));
      }
      // Java: catch (IOException e)
      {
         // Java: KSLog.e(TAG, "IOException", e);
      }
    }

    // Java: void writeTagDidx(ByteArrayOutputStream bos, int index)
    fn writeTagDidx(bos: &mut Vec<u8>, index: i32) {
      // Java: try
      {
         // Java: bos.write(9);
         bos.push(9);
         // Java: bos.write(Multibytecoding.encodeUInt((long)index));
         bos.extend_from_slice(&Multibytecoding::Multibytecoding::encodeUInt(index as i64));
      }
      // Java: catch (IOException e)
      {
         // Java: KSLog.e(TAG, "IOException", e);
      }
    }

    // Java: byte[] writeTagStrokes()
    pub fn writeTagStrokes(&self) -> Vec<u8> {
      // Java: KSAssert.assertNotNull("mInk should not be null!", this.mInk);
      // Java: ByteArrayOutputStream bos = new ByteArrayOutputStream();
      let mut bos: Vec<u8> = Vec::new();
      // Java: int currentIndex = 0;
      let mut current_index: i32 = 0;
      // Java: try
      {
         // Java: for(Iterator iter = this.mInk.getTraceIterator(); iter.hasNext(); ++currentIndex)
         let traces = self.mInk.getTraceIterator();
         for (idx, trace) in traces.iter().enumerate()
         {
            // Java: Trace trace = (Trace)iter.next();
            // Java: if (null != this.mBrushContext.get(trace))
            if self.mBrushContext.get(&idx) == Some(&true)
            {
               // Java: if (0 != currentIndex)
               if 0 != current_index
               {
                  // Java: this.writeTagDidx(bos, currentIndex);
                  Self::writeTagDidx(&mut bos, current_index);
               }
               // Java: ++currentIndex;
               current_index += 1;
            }
            // Java: if (trace.getTraceData().getFIndex() != -1)
            if trace.getTraceData().getFIndex() != -1
            {
               // Java: bos.write(this.writeTagStrokeDescBlock());
               bos.extend_from_slice(&self.writeTagStrokeDescBlock());
            }
            // Java: TagStrokeWriter writer = new TagStrokeWriter(bos, trace.getTraceData(), 1.0F, 1.0F);
            // TODO: TagStrokeWriter not yet available
            // Java: writer.write();
            current_index += 1;
         }
      }
      // Java: catch (InkMLException e)
      {
         // Java: KSLog.e(TAG, "InkMLException", e);
      }
      // Java: catch (IOException e)
      {
         // Java: KSLog.e(TAG, "IOException", e);
      }
      // Java: return bos.toByteArray();
      bos
    }

    // Java: byte[] writeGUIDTable()
    pub fn writeGUIDTable(&self) -> Vec<u8> {
      // Java: if (!this.hasCustomTags())
      if !self.hasCustomTags()
      {
         // Java: return new byte[0];
         return Vec::new();
      }
      // Java: else
      {
         // Java: ByteArrayOutputStream bos = new ByteArrayOutputStream();
         let mut bos: Vec<u8> = Vec::new();
         // Java: if (this.hasCustomUserId())
         if self.hasCustomUserId()
         {
            // Java: bos.write(1);
            bos.push(1);
            // Java: byte[] guidSize = new byte[]{32, 0, 104, 12, 0, 0, 0, 0, 0, -64, 0, 0, 0, 0, 0, 0, 70, 88, -49, 84, -118, -26, -105, -59, 79, -113, 6, -8, -70, -46, -31, -101, 34};
            let guid_size: Vec<u8> = vec![32, 0, 104, 12, 0, 0, 0, 0, 0, 192, 0, 0, 0, 0, 0, 0, 70, 88, 207, 84, 138, 230, 151, 197, 79, 143, 6, 248, 186, 210, 225, 155, 34];
            // Java: try
            {
               // Java: bos.write(guidSize);
               bos.extend_from_slice(&guid_size);
            }
            // Java: catch (IOException e)
            {
               // Java: e.printStackTrace();
            }
         }
         // Java: return bos.toByteArray();
         bos
      }
    }

    // Java: byte[] writeCustomUserId()
    pub fn writeCustomUserId(&self) -> Vec<u8> {
      // Java: if (!this.hasCustomUserId())
      if !self.hasCustomUserId()
      {
         // Java: return new byte[0];
         return Vec::new();
      }
      // Java: else
      {
         // Java: ByteArrayOutputStream bos = new ByteArrayOutputStream();
         let mut bos: Vec<u8> = Vec::new();
         // Java: String userId = this.mInk.getDocID();
         let user_id = self.mInk.getDocID();
         // Java: if (null != userId && userId.length() > 0)
         if !user_id.is_empty()
         {
            // Java: try
            {
               // Java: bos.write(Multibytecoding.encodeUInt(104L));
               bos.extend_from_slice(&Multibytecoding::Multibytecoding::encodeUInt(104));
               // Java: byte[] bytes = userId.getBytes();
               let bytes = user_id.as_bytes();
               // Java: int streamBytesCount = bytes.length;
               let stream_bytes_count = bytes.len();
               // Java: bos.write(streamBytesCount);
               bos.push(stream_bytes_count as u8);
               // Java: bos.write(131);
               bos.push(131);
               // Java: bos.write(bytes);
               bos.extend_from_slice(bytes);
            }
            // Java: catch (IOException e)
            {
               // Java: KSLog.e(TAG, "IOException", e);
            }
         }
         // Java: return bos.toByteArray();
         bos
      }
    }

    // Java: boolean hasCustomTags()
    pub fn hasCustomTags(&self) -> bool {
      // Java: return this.hasCustomUserId();
      self.hasCustomUserId()
    }

    // Java: boolean hasCustomUserId()
    pub fn hasCustomUserId(&self) -> bool {
      // Java: String userId = this.mInk.getDocID();
      let user_id = self.mInk.getDocID();
      // Java: return null != userId && userId.length() > 0;
      !user_id.is_empty()
    }

}
