// Java: import java.io.IOException
// Java: import java.util.ArrayList
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian

pub struct GenericPropertyNode {
    pub(crate) start: i32,
    pub(crate) end: i32,
    pub(crate) buf: Vec<u8>,
    pub(crate) offset: i32,
}

impl GenericPropertyNode {
    pub fn new(start: i32, end: i32, buf: Vec<u8>, offset: i32) -> Self {
        GenericPropertyNode { start, end, buf, offset }
    }
    pub fn getStart(&self) -> i32 { self.start }
    pub fn getEnd(&self) -> i32 { self.end }
    pub fn getBytes(&self) -> &[u8] { &self.buf }
    pub fn getOffset(&self) -> i32 { self.offset }
}

fn le_get_int(data: &[u8], offset: i32) -> i32 {
    let o = offset as usize;
    let b0 = data[o] as i32 & 0xFF;
    let b1 = data[o + 1] as i32 & 0xFF;
    let b2 = data[o + 2] as i32 & 0xFF;
    let b3 = data[o + 3] as i32 & 0xFF;
    (b3 << 24) | (b2 << 16) | (b1 << 8) | b0
}

fn le_put_int(data: &mut [u8], offset: i32, value: i32) {
    let o = offset as usize;
    data[o] = value as u8;
    data[o + 1] = (value >> 8) as u8;
    data[o + 2] = (value >> 16) as u8;
    data[o + 3] = (value >> 24) as u8;
}

// Java: public class PlexOfCps
pub struct PlexOfCps {
    // Java field: int _count
    pub(crate) _count: i32,
    // Java field: int _offset
    _offset: i32,
    // Java field: int _sizeOfStruct
    _sizeOfStruct: i32,
    // Java field: DocumentInputStream _stream
    _stream: Option<DocumentInputStream>,
    // Java field: ArrayList<GenericPropertyNode> _props
    _props: Vec<GenericPropertyNode>,
    // Java field: byte[] _buffer
    _buffer: Option<Vec<u8>>,
}

// Java: public class PlexOfCps
impl PlexOfCps {
    // Java: PlexOfCps(int sizeOfStruct)
    pub fn new(size_of_struct: i32) -> Self {
      // Java: super();
      // Java: this._props = new ArrayList();
      // Java: this._sizeOfStruct = sizeOfStruct;
      PlexOfCps {
          _count: 0,
          _offset: 0,
          _sizeOfStruct: size_of_struct,
          _stream: None,
          _props: Vec::new(),
          _buffer: None,
      }
    }

    // Java: PlexOfCps(DocumentInputStream inputStream, int start, int size, int sizeOfStruct)
    pub fn new_from_stream(input_stream: DocumentInputStream, start: i32, size: i32, size_of_struct: i32) -> Self {
      // Java: super();
      // Java: this._count = (size - 4) / (4 + sizeOfStruct);
      let count = (size - 4) / (4 + size_of_struct);
      // Java: this._offset = start;
      // Java: this._sizeOfStruct = sizeOfStruct;
      // Java: this._stream = inputStream;
      // Java: this._props = null;
      PlexOfCps {
          _count: count,
          _offset: start,
          _sizeOfStruct: size_of_struct,
          _stream: Some(input_stream),
          _props: Vec::new(),
          _buffer: None,
      }
    }

    // Java: GenericPropertyNode getProperty(int index)
    pub fn getProperty(&mut self, index: i32) -> GenericPropertyNode {
      // Java: if (this._buffer == null)
        if self._buffer.is_none()
      {
         // Java: int size = (this._count + 1) * 4 + this._count * this._sizeOfStruct;
         let size = (self._count + 1) * 4 + self._count * self._sizeOfStruct;
         // Java: this._buffer = new byte[size];
         self._buffer = Some(vec![0u8; size as usize]);
         // Java: this._stream.seek((long)this._offset);
         if let Some(ref mut stream) = self._stream {
             stream.seek(self._offset as i64);
         // Java: this._stream.read(this._buffer, 0, size);
             if let Some(ref mut buf) = self._buffer {
                 stream.read_off(buf, 0, size);
             }
         }
      }
      // Java: int offset = index * 4;
      let offset = index * 4;
      // Java: int start = LittleEndian.getInt(this._buffer, offset);
      let buf = self._buffer.as_ref().unwrap();
      let start = le_get_int(buf, offset);
      // Java: int end = LittleEndian.getInt(this._buffer, offset + 4);
      let end = le_get_int(buf, offset + 4);
      // Java: return new GenericPropertyNode(start, end, this._buffer, this.getStructOffset(index));
      GenericPropertyNode::new(start, end, buf.clone(), self.getStructOffset(index))
    }

    // Java: void addProperty(GenericPropertyNode node)
    pub fn addProperty(&mut self, node: GenericPropertyNode) {
      // Java: this._props.add(node);
      self._props.push(node);
    }

    // Java: byte[] toByteArray()
    pub fn toByteArray(&self) -> Vec<u8> {
      // Java: int size = this._props.size();
      let size = self._props.len() as i32;
      // Java: int cpBufSize = (size + 1) * 4;
      let cp_buf_size = (size + 1) * 4;
      // Java: int structBufSize = this._sizeOfStruct * size;
      let struct_buf_size = self._sizeOfStruct * size;
      // Java: int bufSize = cpBufSize + structBufSize;
      let buf_size = cp_buf_size + struct_buf_size;
      // Java: byte[] buf = new byte[bufSize];
      let mut buf = vec![0u8; buf_size as usize];
      // Java: GenericPropertyNode node = null;
      let mut last_node: Option<&GenericPropertyNode> = None;
      // Java: for(int x = 0; x < size; ++x)
        for x in 0..size
      {
         // Java: node = (GenericPropertyNode)this._props.get(x);
         let node = &self._props[x as usize];
         last_node = Some(node);
         // Java: LittleEndian.putInt(buf, 4 * x, node.getStart());
         le_put_int(&mut buf, 4 * x, node.getStart());
         // Java: System.arraycopy(node.getBytes(), node.getOffset(), buf, cpBufSize + x * this._sizeOfStruct, this._sizeOfStruct);
         let src = node.getBytes();
         let src_off = node.getOffset() as usize;
         let dst_off = (cp_buf_size + x * self._sizeOfStruct) as usize;
         let len = self._sizeOfStruct as usize;
         buf[dst_off..dst_off + len].copy_from_slice(&src[src_off..src_off + len]);
      }
      // Java: LittleEndian.putInt(buf, 4 * size, node.getEnd());
      if let Some(node) = last_node {
          le_put_int(&mut buf, 4 * size, node.getEnd());
      }
      // Java: return buf;
      buf
    }

    // Java: int length()
    pub fn length(&self) -> i32 {
      // Java: return this._count;
      self._count
    }

    // Java: int getStructOffset(int index)
    pub fn getStructOffset(&self, index: i32) -> i32 {
      // Java: return 4 * (this._count + 1) + this._sizeOfStruct * index;
      4 * (self._count + 1) + self._sizeOfStruct * index
    }

}
