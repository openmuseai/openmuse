// Java: import cn.wps.base.assertion.KSAssert
use util::base::assertion::KSAssert;
// Java: import cn.wps.moffice.util.KSLog
use util::moffice::util::KSLog;
// Java: import com.hp.hpl.inkml.TraceDataList
use crate::com::hp::hpl::inkml::TraceDataList;
// Java: import com.isf.utils.DeltaTransform
use crate::com::isf::utils::DeltaTransform;
// Java: import com.isf.utils.Huffman
use crate::com::isf::utils::Huffman;
// Java: import com.isf.utils.Multibytecoding
use crate::com::isf::utils::Multibytecoding;
// Java: import java.io.ByteArrayOutputStream
use std::io::Write;
// Java: import java.io.IOException
// (Rust uses Result for IO errors)
// Java: import java.io.OutputStream
use crate::com::isf::writer::BitWriter;
use crate::com::hp::hpl::inkml::Channel;

// Java field: String TAG
const TAG: &str = "TagStrokeWriter";

// Java: public class TagStrokeWriter
pub struct TagStrokeWriter<'a> {
    // Java field: OutputStream mWriter
    mWriter: Vec<u8>,
    // Java field: TraceDataList mTraceDataList
    mTraceDataList: &'a TraceDataList::TraceDataList,
    // Java field: float mScaleX
    mScaleX: f32,
    // Java field: float mScaleY
    mScaleY: f32,
}

// Java: public class TagStrokeWriter
impl<'a> TagStrokeWriter<'a> {
    // Java: TagStrokeWriter(OutputStream writer, TraceDataList trace, float scaleX, float scaleY)
    pub fn TagStrokeWriter(trace: &'a TraceDataList::TraceDataList, scale_x: f32, scale_y: f32) -> Self {
      // Java: super();
      // Java: KSAssert.assertNotNull("writer should not be null!", writer);
      // Java: KSAssert.assertNotNull("trace should not be null!", trace);
      // Java: this.mWriter = writer;
      // Java: this.mTraceDataList = trace;
      // Java: this.mScaleX = scaleX;
      // Java: this.mScaleY = scaleY;
      Self {
          mWriter: Vec::new(),
          mTraceDataList: trace,
          mScaleX: scale_x,
          mScaleY: scale_y,
      }
    }

    // Java: int write()
    pub fn write(&mut self) -> i32 {
      // Java: KSAssert.assertNotNull("mWriter should not be null!", this.mWriter);
      // Java: KSAssert.assertNotNull("mTraceDataList should not be null!", this.mTraceDataList);
      // Java: byte[] compressDataX = this.compressX();
      let compress_data_x = self.compressX();
      // Java: byte[] compressDataY = this.compressY();
      let compress_data_y = self.compressY();
      // Java: byte[] compressDataF = this.compressF();
      let compress_data_f = self.compressF();
      // Java: byte[] bPointCount = Multibytecoding.encodeUInt((long)this.mTraceDataList.size());
      let b_point_count = Multibytecoding::Multibytecoding::encodeUInt(self.mTraceDataList.size() as i64);
      // Java: int streamBytesCount = compressDataX.length + 1 + compressDataY.length + 1 + (compressDataF == null ? 0 : compressDataF.length + 1) + bPointCount.length;
      let stream_bytes_count = compress_data_x.len() + 1 + compress_data_y.len() + 1 + (match &compress_data_f { None => 0, Some(f) => f.len() + 1 }) + b_point_count.len();
      // Java: try
      {
         // Java: this.mWriter.write(10);
         self.mWriter.push(10);
         // Java: this.mWriter.write(Multibytecoding.encodeUInt((long)streamBytesCount));
         self.mWriter.extend_from_slice(&Multibytecoding::Multibytecoding::encodeUInt(stream_bytes_count as i64));
         // Java: this.mWriter.write(bPointCount);
         self.mWriter.extend_from_slice(&b_point_count);
         // Java: this.mWriter.write(131);
         self.mWriter.push(131);
         // Java: this.mWriter.write(compressDataX);
         self.mWriter.extend_from_slice(&compress_data_x);
         // Java: this.mWriter.write(131);
         self.mWriter.push(131);
         // Java: this.mWriter.write(compressDataY);
         self.mWriter.extend_from_slice(&compress_data_y);
         // Java: if (compressDataF != null)
         if let Some(ref f_data) = compress_data_f
         {
            // Java: this.mWriter.write(131);
            self.mWriter.push(131);
            // Java: this.mWriter.write(compressDataF);
            self.mWriter.extend_from_slice(f_data);
         }
         // Java: return streamBytesCount + 1;
         return (stream_bytes_count + 1) as i32;
      }
      // Java: catch (IOException e)
      // (errors won't occur with Vec<u8>)
      {
         // Java: KSLog.e(TAG, "IOException", e);
         // Java: return 0;
         0
      }
    }

    // Java: byte[] compressX()
    pub fn compressX(&self) -> Vec<u8> {
      // Java: int size = this.mTraceDataList.size();
      let size = self.mTraceDataList.size();
      // Java: int xindex = this.mTraceDataList.getXIndex();
      let xindex = self.mTraceDataList.getXIndex();
      // Java: int[] x = new int[size];
      let mut x = vec![0i32; size];
      // Java: for(int i = 0; i < size; ++i)
      for i in 0..size
      {
         // Java: Integer value = (int)((float)this.toInt(i, xindex) / this.mScaleX);
         let value = self.toInt(i, xindex);
         // Java: if (null != value)
         if let Some(v) = value
         {
            // Java: x[i] = value;
            x[i] = (v as f32 / self.mScaleX) as i32;
         }
      }
      // Java: return this.compress(x);
      self.compress(&mut x)
    }

    // Java: byte[] compressY()
    pub fn compressY(&self) -> Vec<u8> {
      // Java: int size = this.mTraceDataList.size();
      let size = self.mTraceDataList.size();
      // Java: int yindex = this.mTraceDataList.getYIndex();
      let yindex = self.mTraceDataList.getYIndex();
      // Java: int[] y = new int[size];
      let mut y = vec![0i32; size];
      // Java: for(int i = 0; i < size; ++i)
      for i in 0..size
      {
         // Java: Integer value = (int)((float)this.toInt(i, yindex) / this.mScaleY);
         let value = self.toInt(i, yindex);
         // Java: if (null != value)
         if let Some(v) = value
         {
            // Java: y[i] = value;
            y[i] = (v as f32 / self.mScaleY) as i32;
         }
      }
      // Java: return this.compress(y);
      self.compress(&mut y)
    }

    // Java: byte[] compressF()
    pub fn compressF(&self) -> Option<Vec<u8>> {
      // Java: int size = this.mTraceDataList.size();
      let size = self.mTraceDataList.size();
      // Java: int fIndex = this.mTraceDataList.getFIndex();
      let f_index = self.mTraceDataList.getFIndex();
      // Java: if (fIndex != -1)
      if f_index != -1
      {
         // Java: int[] f = new int[size];
         let mut f = vec![0i32; size];
         // Java: for(int i = 0; i < size; ++i)
         for i in 0..size
         {
            // Java: Integer value = this.toInt(i, fIndex);
            let value = self.toInt(i, f_index);
            // Java: if (null != value)
            if let Some(v) = value
            {
               // Java: f[i] = value;
               f[i] = v;
            }
         }
         // Java: return this.compress(f);
         Some(self.compress(&mut f))
      }
      // Java: else
      else
      {
         // Java: return null;
         None
      }
    }

    // Java: byte[] compress(int[] in)
    pub fn compress(&self, data: &mut [i32]) -> Vec<u8> {
      // Java: KSAssert.assertNotNull("in should not be null!", in);
      // Java: ByteArrayOutputStream out = new ByteArrayOutputStream();
      // Java: BitWriter writer = new BitWriter(out);
      let mut writer = BitWriter::BitWriter::BitWriter();
      // Java: DeltaTransform.transform(in);
      DeltaTransform::DeltaTransform::transform(data);
      // Java: Huffman.deflate(writer, 3, in);
      Huffman::Huffman::deflate(&mut writer, 3, data);
      // Java: writer.close();
      writer.close();
      // Java: return out.toByteArray();
      writer.get_output().to_vec()
    }

    // Java: Integer toInt(int i, int index)
    pub fn toInt(&self, i: usize, index: i32) -> Option<i32> {
      // Java: KSAssert.assertNotNull("mTraceDataList should not be null!", this.mTraceDataList);
      // Java: Object[] traceData = this.mTraceDataList.get(i);
      let trace_data = self.mTraceDataList.get(i);
      // Java: switch (this.mTraceDataList.getXYChannleType())
      match self.mTraceDataList.getXYChannleType()
      {
         // Java: case DECIMAL:
         Channel::ChannelType::DECIMAL |
         // Java: case INTEGER:
         Channel::ChannelType::INTEGER =>
         {
            // Java: Float value = (Float)traceData[index];
            if (index as usize) < trace_data.len() {
                let value = trace_data[index as usize];
                // Java: return value.intValue();
                Some(value as i32)
            } else {
                None
            }
         }
         // Java: default:
         _ =>
         {
            // Java: KSAssert.fail("TODO");
            // Java: return null;
            None
         }
      }
    }

}
