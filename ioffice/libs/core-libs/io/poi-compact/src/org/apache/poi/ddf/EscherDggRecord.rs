// Java: import java.io.IOException
// Java: import java.util.ArrayList
// Java: import java.util.Arrays
// Java: import java.util.Collections
// Java: import java.util.Comparator
// Java: import java.util.List
// Java: import org.apache.poi.poifs.common.IBlockDocumentInputStream
use crate::org::apache::poi::poifs::common::IBlockDocumentInputStream::IBlockDocumentInputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.HexDump
// Java: import org.apache.poi.util.LittleEndian
// Java: import org.apache.poi.util.RecordFormatException
use super::EscherRecord::EscherRecord;
use super::EscherRecordFactory::EscherRecordFactory;
use super::EscherSerializationListener::EscherSerializationListener;

fn le_put_short(data: &mut [u8], offset: i32, value: i16) {
    let off = offset as usize;
    data[off] = value as u8;
    data[off + 1] = (value >> 8) as u8;
}

fn le_put_int(data: &mut [u8], offset: i32, value: i32) {
    let off = offset as usize;
    data[off] = value as u8;
    data[off + 1] = (value >> 8) as u8;
    data[off + 2] = (value >> 16) as u8;
    data[off + 3] = (value >> 24) as u8;
}

// Java field: short RECORD_ID
pub(crate) const RECORD_ID: i16 = -4090;
// Java field: String RECORD_DESCRIPTION
pub(crate) const RECORD_DESCRIPTION: &str = "MsofbtDgg";

// Java: public class EscherDggRecord extends EscherRecord
pub struct EscherDggRecord {
    pub(crate) base: EscherRecord,
    // Java field: int field_1_shapeIdMax
    pub(crate) field_1_shapeIdMax: i32,
    // Java field: int field_3_numShapesSaved
    pub(crate) field_3_numShapesSaved: i32,
    // Java field: int field_4_drawingsSaved
    pub(crate) field_4_drawingsSaved: i32,
    // Java field: FileIdCluster[] field_5_fileIdClusters
    pub(crate) field_5_fileIdClusters: Vec<FileIdCluster>,
    // Java field: int maxDgId
    pub(crate) maxDgId: i32,
}

// Java: public class FileIdCluster
pub struct FileIdCluster {
    // Java field: int field_1_drawingGroupId
    pub(crate) field_1_drawingGroupId: i32,
    // Java field: int field_2_numShapeIdsUsed
    pub(crate) field_2_numShapeIdsUsed: i32,
}

// Java: public class EscherDggRecord extends EscherRecord
impl EscherDggRecord {
    // Java: EscherDggRecord()
    pub fn new() -> Self {
      // Java: super();
      EscherDggRecord {
          base: EscherRecord::new(),
          field_1_shapeIdMax: 0,
          field_3_numShapesSaved: 0,
          field_4_drawingsSaved: 0,
          field_5_fileIdClusters: Vec::new(),
          maxDgId: 0,
      }
    }

    pub fn getOptions(&self) -> i16 { self.base.getOptions() }
    pub fn getRemainingBytes(&self) -> i32 { self.base.getRemainingBytes() }

    // Java: int fillFields(DocumentInputStream data, int offset, EscherRecordFactory recordFactory)
    pub fn fillFields_stream(&mut self, data: &mut DocumentInputStream, offset: i32, _record_factory: &dyn EscherRecordFactory) -> i32 {
      // Java: int bytesRemaining = this.readHeader(data, offset);
      let mut bytes_remaining = self.base.readHeader_stream(data, offset);
      // Java: int pos = offset + 8;
      let pos = offset + 8;
      // Java: int size = 0;
      let mut size = 0;
      // Java: data.seek((long)pos);
      data.seek(pos as i64);
      // Java: this.field_1_shapeIdMax = data.readInt();
      self.field_1_shapeIdMax = data.readInt();
      // Java: size += 4;
      size += 4;
      // Java: data.readInt();
      data.readInt();
      // Java: size += 4;
      size += 4;
      // Java: this.field_3_numShapesSaved = data.readInt();
      self.field_3_numShapesSaved = data.readInt();
      // Java: size += 4;
      size += 4;
      // Java: this.field_4_drawingsSaved = data.readInt();
      self.field_4_drawingsSaved = data.readInt();
      // Java: size += 4;
      size += 4;
      // Java: this.field_5_fileIdClusters = new FileIdCluster[(bytesRemaining - size) / 8];
      let cluster_count = ((bytes_remaining - size) / 8) as usize;
      self.field_5_fileIdClusters = Vec::with_capacity(cluster_count);
      // Java: for(int i = 0; i < this.field_5_fileIdClusters.length; ++i)
        for _i in 0..cluster_count
      {
         // Java: this.field_5_fileIdClusters[i] = new FileIdCluster(data.readInt(), data.readInt());
         let cluster = FileIdCluster::new(data.readInt(), data.readInt());
         // Java: this.maxDgId = Math.max(this.maxDgId, this.field_5_fileIdClusters[i].getDrawingGroupId());
         self.maxDgId = self.maxDgId.max(cluster.getDrawingGroupId());
         self.field_5_fileIdClusters.push(cluster);
         // Java: size += 8;
         size += 8;
      }
      // Java: bytesRemaining -= size;
      bytes_remaining -= size;
      // Java: if (bytesRemaining != 0)
        if bytes_remaining != 0
      {
         // Java: throw new RecordFormatException("Expecting no remaining data but got " + bytesRemaining + " byte(s).");
         panic!("RecordFormatException: Expecting no remaining data but got {} byte(s).", bytes_remaining);
      }
      // Java: else
        else
      {
         // Java: return 8 + size + bytesRemaining;
         return 8 + size + bytes_remaining;
      }
    }

    // Java: int fillFields(IBlockDocumentInputStream data, int offset, EscherRecordFactory recordFactory, String pictureDirectory, String extension)
    pub fn fillFields_block(&mut self, data: &mut dyn IBlockDocumentInputStream, _offset: i32, _record_factory: &dyn EscherRecordFactory, _picture_directory: &str, _extension: &str) -> i32 {
      // Java: int bytesRemaining = this.getRemainingBytes();
      let mut bytes_remaining = self.base.getRemainingBytes();
      // Java: int size = 0;
      let mut size = 0;
      // Java: this.field_1_shapeIdMax = data.readInt();
      self.field_1_shapeIdMax = data.readInt();
      // Java: size += 4;
      size += 4;
      // Java: data.readInt();
      data.readInt();
      // Java: size += 4;
      size += 4;
      // Java: this.field_3_numShapesSaved = data.readInt();
      self.field_3_numShapesSaved = data.readInt();
      // Java: size += 4;
      size += 4;
      // Java: this.field_4_drawingsSaved = data.readInt();
      self.field_4_drawingsSaved = data.readInt();
      // Java: size += 4;
      size += 4;
      // Java: this.field_5_fileIdClusters = new FileIdCluster[(bytesRemaining - size) / 8];
      let cluster_count = ((bytes_remaining - size) / 8) as usize;
      self.field_5_fileIdClusters = Vec::with_capacity(cluster_count);
      // Java: for(int i = 0; i < this.field_5_fileIdClusters.length; ++i)
        for _i in 0..cluster_count
      {
         // Java: this.field_5_fileIdClusters[i] = new FileIdCluster(data.readInt(), data.readInt());
         let cluster = FileIdCluster::new(data.readInt(), data.readInt());
         // Java: this.maxDgId = Math.max(this.maxDgId, this.field_5_fileIdClusters[i].getDrawingGroupId());
         self.maxDgId = self.maxDgId.max(cluster.getDrawingGroupId());
         self.field_5_fileIdClusters.push(cluster);
         // Java: size += 8;
         size += 8;
      }
      // Java: bytesRemaining -= size;
      bytes_remaining -= size;
      // Java: if (bytesRemaining != 0)
        if bytes_remaining != 0
      {
         // Java: throw new RecordFormatException("Expecting no remaining data but got " + bytesRemaining + " byte(s).");
         panic!("RecordFormatException: Expecting no remaining data but got {} byte(s).", bytes_remaining);
      }
      // Java: else
        else
      {
         // Java: return 8 + size + bytesRemaining;
         return 8 + size + bytes_remaining;
      }
    }

    // Java: int serialize(int offset, byte[] data, EscherSerializationListener listener)
    pub fn serialize_with_listener(&mut self, offset: i32, data: &mut [u8], listener: &mut dyn EscherSerializationListener) -> i32 {
      // Java: listener.beforeRecordSerialize(offset, this.getRecordId(), this);
      listener.beforeRecordSerialize(offset, self.getRecordId(), &self.base);
      // Java: LittleEndian.putShort(data, offset, this.getOptions());
      le_put_short(data, offset, self.getOptions());
      // Java: int pos = offset + 2;
      let mut pos = offset + 2;
      // Java: LittleEndian.putShort(data, pos, this.getRecordId());
      le_put_short(data, pos, self.getRecordId());
      // Java: pos += 2;
      pos += 2;
      // Java: int remainingBytes = this.getRecordSize() - 8;
      let remaining_bytes = self.getRecordSize() - 8;
      // Java: LittleEndian.putInt(data, pos, remainingBytes);
      le_put_int(data, pos, remaining_bytes);
      // Java: pos += 4;
      pos += 4;
      // Java: LittleEndian.putInt(data, pos, this.field_1_shapeIdMax);
      le_put_int(data, pos, self.field_1_shapeIdMax);
      // Java: pos += 4;
      pos += 4;
      // Java: LittleEndian.putInt(data, pos, this.getNumIdClusters());
      le_put_int(data, pos, self.getNumIdClusters());
      // Java: pos += 4;
      pos += 4;
      // Java: LittleEndian.putInt(data, pos, this.field_3_numShapesSaved);
      le_put_int(data, pos, self.field_3_numShapesSaved);
      // Java: pos += 4;
      pos += 4;
      // Java: LittleEndian.putInt(data, pos, this.field_4_drawingsSaved);
      le_put_int(data, pos, self.field_4_drawingsSaved);
      // Java: pos += 4;
      pos += 4;
      // Java: for(int i = 0; i < this.field_5_fileIdClusters.length; ++i)
        for i in 0..self.field_5_fileIdClusters.len()
      {
         // Java: LittleEndian.putInt(data, pos, this.field_5_fileIdClusters[i].field_1_drawingGroupId);
         le_put_int(data, pos, self.field_5_fileIdClusters[i].field_1_drawingGroupId);
         // Java: pos += 4;
         pos += 4;
         // Java: LittleEndian.putInt(data, pos, this.field_5_fileIdClusters[i].field_2_numShapeIdsUsed);
         le_put_int(data, pos, self.field_5_fileIdClusters[i].field_2_numShapeIdsUsed);
         // Java: pos += 4;
         pos += 4;
      }
      // Java: listener.afterRecordSerialize(pos, this.getRecordId(), this.getRecordSize(), this);
      let record_size = self.getRecordSize();
      listener.afterRecordSerialize(pos, self.getRecordId(), record_size, &self.base);
      // Java: return this.getRecordSize();
      record_size
    }

    // Java: int getRecordSize()
    pub fn getRecordSize(&self) -> i32 {
      // Java: return 24 + 8 * this.field_5_fileIdClusters.length;
      24 + 8 * self.field_5_fileIdClusters.len() as i32
    }

    // Java: short getRecordId()
    pub fn getRecordId(&self) -> i16 {
      // Java: return -4090;
      -4090
    }

    // Java: String getRecordName()
    pub fn getRecordName(&self) -> String {
      // Java: return "Dgg";
      "Dgg".to_string()
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: StringBuffer field_5_string = new StringBuffer();
      let mut field_5_string = String::new();
      // Java: if (this.field_5_fileIdClusters != null)
        if !self.field_5_fileIdClusters.is_empty()
      {
         // Java: for(int i = 0; i < this.field_5_fileIdClusters.length; ++i)
            for i in 0..self.field_5_fileIdClusters.len()
         {
            // Java: field_5_string.append("  DrawingGroupId").append(i + 1).append(": ");
            field_5_string.push_str(&format!("  DrawingGroupId{}: ", i + 1));
            // Java: field_5_string.append(this.field_5_fileIdClusters[i].field_1_drawingGroupId);
            field_5_string.push_str(&self.field_5_fileIdClusters[i].field_1_drawingGroupId.to_string());
            // Java: field_5_string.append('\n');
            field_5_string.push('\n');
            // Java: field_5_string.append("  NumShapeIdsUsed").append(i + 1).append(": ");
            field_5_string.push_str(&format!("  NumShapeIdsUsed{}: ", i + 1));
            // Java: field_5_string.append(this.field_5_fileIdClusters[i].field_2_numShapeIdsUsed);
            field_5_string.push_str(&self.field_5_fileIdClusters[i].field_2_numShapeIdsUsed.to_string());
            // Java: field_5_string.append('\n');
            field_5_string.push('\n');
         }
      }
      // Java: return this.getClass().getName() + ":" + '\n' + "  RecordId: 0x" + HexDump.toHex((short)-4090) + '\n' + "  Options: 0x" + HexDump.toHex(this.getOptions()) + '\n' + "  ShapeIdMax: " + this.field_1_shapeIdMax + '\n' + "  NumIdClusters: " + this.getNumIdClusters() + '\n' + "  NumShapesSaved: " + this.field_3_numShapesSaved + '\n' + "  DrawingsSaved: " + this.field_4_drawingsSaved + '\n' + "" + field_5_string.toString();
      format!("EscherDggRecord:\n\
          \x20 RecordId: 0x{:04x}\n\
          \x20 Options: 0x{:04x}\n\
          \x20 ShapeIdMax: {}\n\
          \x20 NumIdClusters: {}\n\
          \x20 NumShapesSaved: {}\n\
          \x20 DrawingsSaved: {}\n\
          {}",
          (-4090i16) as u16,
          self.getOptions() as u16,
          self.field_1_shapeIdMax,
          self.getNumIdClusters(),
          self.field_3_numShapesSaved,
          self.field_4_drawingsSaved,
          field_5_string)
    }

    // Java: int getShapeIdMax()
    pub fn getShapeIdMax(&self) -> i32 {
      // Java: return this.field_1_shapeIdMax;
      self.field_1_shapeIdMax
    }

    // Java: void setShapeIdMax(int shapeIdMax)
    pub fn setShapeIdMax(&mut self, shape_id_max: i32) {
      // Java: this.field_1_shapeIdMax = shapeIdMax;
      self.field_1_shapeIdMax = shape_id_max;
    }

    // Java: int getNumIdClusters()
    pub fn getNumIdClusters(&self) -> i32 {
      // Java: return this.field_5_fileIdClusters == null ? 0 : this.field_5_fileIdClusters.length + 1;
      if self.field_5_fileIdClusters.is_empty() { 0 } else { self.field_5_fileIdClusters.len() as i32 + 1 }
    }

    // Java: int getNumShapesSaved()
    pub fn getNumShapesSaved(&self) -> i32 {
      // Java: return this.field_3_numShapesSaved;
      self.field_3_numShapesSaved
    }

    // Java: void setNumShapesSaved(int numShapesSaved)
    pub fn setNumShapesSaved(&mut self, num_shapes_saved: i32) {
      // Java: this.field_3_numShapesSaved = numShapesSaved;
      self.field_3_numShapesSaved = num_shapes_saved;
    }

    // Java: int getDrawingsSaved()
    pub fn getDrawingsSaved(&self) -> i32 {
      // Java: return this.field_4_drawingsSaved;
      self.field_4_drawingsSaved
    }

    // Java: void setDrawingsSaved(int drawingsSaved)
    pub fn setDrawingsSaved(&mut self, drawings_saved: i32) {
      // Java: this.field_4_drawingsSaved = drawingsSaved;
      self.field_4_drawingsSaved = drawings_saved;
    }

    // Java: int getMaxDrawingGroupId()
    pub fn getMaxDrawingGroupId(&self) -> i32 {
      // Java: return this.maxDgId;
      self.maxDgId
    }

    // Java: void setMaxDrawingGroupId(int id)
    pub fn setMaxDrawingGroupId(&mut self, id: i32) {
      // Java: this.maxDgId = id;
      self.maxDgId = id;
    }

    // Java: FileIdCluster[] getFileIdClusters()
    pub fn getFileIdClusters(&self) -> &[FileIdCluster] {
      // Java: return this.field_5_fileIdClusters;
      &self.field_5_fileIdClusters
    }

    // Java: void setFileIdClusters(FileIdCluster[] fileIdClusters)
    pub fn setFileIdClusters(&mut self, file_id_clusters: Vec<FileIdCluster>) {
      // Java: this.field_5_fileIdClusters = fileIdClusters;
      self.field_5_fileIdClusters = file_id_clusters;
    }

    // Java: void addCluster(int dgId, int numShapedUsed)
    pub fn addCluster(&mut self, dg_id: i32, num_shaped_used: i32) {
      // Java: this.addCluster(dgId, numShapedUsed, true);
      self.addCluster_sort(dg_id, num_shaped_used, true);
    }

    // Java: void addCluster(int dgId, int numShapedUsed, boolean sort)
    pub fn addCluster_sort(&mut self, dg_id: i32, num_shaped_used: i32, sort: bool) {
      // Java: List<FileIdCluster> clusters = new ArrayList(Arrays.asList(this.field_5_fileIdClusters));
      // Java: clusters.add(new FileIdCluster(dgId, numShapedUsed));
      self.field_5_fileIdClusters.push(FileIdCluster::new(dg_id, num_shaped_used));
      // Java: if (sort)
        if sort
      {
         // Java: Collections.sort(clusters, MY_COMP);
         self.field_5_fileIdClusters.sort_by(|a, b| a.field_1_drawingGroupId.cmp(&b.field_1_drawingGroupId));
      }
      // Java: this.maxDgId = Math.min(this.maxDgId, dgId);
      self.maxDgId = self.maxDgId.min(dg_id);
      // Java: this.field_5_fileIdClusters = (FileIdCluster[])clusters.toArray(new FileIdCluster[clusters.size()]);
    }

}

// Java: public class FileIdCluster
impl FileIdCluster {
    // Java: FileIdCluster(int drawingGroupId, int numShapeIdsUsed)
    pub fn new(drawing_group_id: i32, num_shape_ids_used: i32) -> Self {
         // Java: super();
         // Java: this.field_1_drawingGroupId = drawingGroupId;
         // Java: this.field_2_numShapeIdsUsed = numShapeIdsUsed;
         FileIdCluster {
             field_1_drawingGroupId: drawing_group_id,
             field_2_numShapeIdsUsed: num_shape_ids_used,
         }
    }

    // Java: int getDrawingGroupId()
    pub fn getDrawingGroupId(&self) -> i32 {
         // Java: return this.field_1_drawingGroupId;
         self.field_1_drawingGroupId
    }

    // Java: int getNumShapeIdsUsed()
    pub fn getNumShapeIdsUsed(&self) -> i32 {
         // Java: return this.field_2_numShapeIdsUsed;
         self.field_2_numShapeIdsUsed
    }

    // Java: void incrementShapeId()
    pub fn incrementShapeId(&mut self) {
         // Java: ++this.field_2_numShapeIdsUsed;
         self.field_2_numShapeIdsUsed += 1;
    }

}
