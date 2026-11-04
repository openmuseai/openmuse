// Java: import java.io.IOException
// Java: import java.io.OutputStream
// Java: import org.apache.poi.hpsf.ClassID
// Java: import org.apache.poi.hwpf.model.CharArrayPool
// Java: import org.apache.poi.hwpf.model.PageBuffer
// Java: import org.apache.poi.util.LittleEndian

use std::sync::Arc;
use super::Child::Child;
use crate::org::apache::poi::hpsf::ClassID::ClassID;

fn le_get_short(data: &[u8], offset: usize) -> i16 {
    (data[offset] as i16) | ((data[offset + 1] as i16) << 8)
}
fn le_get_ushort(data: &[u8], offset: usize) -> u16 {
    (data[offset] as u16) | ((data[offset + 1] as u16) << 8)
}
fn le_get_int(data: &[u8], offset: usize) -> i32 {
    (data[offset] as i32) | ((data[offset + 1] as i32) << 8) | ((data[offset + 2] as i32) << 16) | ((data[offset + 3] as i32) << 24)
}
fn le_put_short(data: &mut [u8], offset: usize, value: i16) {
    data[offset] = value as u8; data[offset + 1] = (value >> 8) as u8;
}
fn le_put_ushort(data: &mut [u8], offset: usize, value: u16) {
    data[offset] = value as u8; data[offset + 1] = (value >> 8) as u8;
}
fn le_put_int(data: &mut [u8], offset: usize, value: i32) {
    data[offset] = value as u8; data[offset + 1] = (value >> 8) as u8; data[offset + 2] = (value >> 16) as u8; data[offset + 3] = (value >> 24) as u8;
}

// Java: public class Property implements Child
#[derive(Clone)]
pub struct Property {
    // Java field: int _name_size_offset
    // Java field: int _NO_INDEX
    // Java field: int _node_color_offset
    // Java field: int _previous_property_offset
    // Java field: int _next_property_offset
    // Java field: int _child_property_offset
    // Java field: int _storage_clsid_offset
    // Java field: int _start_block_offset
    // Java field: int _size_offset
    // Java field: byte _NODE_BLACK
    // Java field: byte _NODE_RED
    // Java field: int _big_block_minimum_bytes
    // Java field: String _name
    pub(crate) _name: String,
    // Java field: byte _property_type
    pub(crate) _property_type: i8,
    // Java field: byte _node_color
    pub(crate) _node_color: i8,
    // Java field: int _previous_property
    pub(crate) _previous_property: i32,
    // Java field: int _next_property
    pub(crate) _next_property: i32,
    // Java field: int _child_property
    pub(crate) _child_property: i32,
    // Java field: ClassID _storage_clsid
    pub(crate) _storage_clsid: ClassID,
    // Java field: int _start_block
    pub(crate) _start_block: i32,
    // Java field: int _size
    pub(crate) _size: i32,
    // Java field: int _index
    pub(crate) _index: i32,
    // Java field: Child _next_child
    pub(crate) _next_child: Option<Arc<dyn Child>>,
    // Java field: Child _previous_child
    pub(crate) _previous_child: Option<Arc<dyn Child>>,
}

pub const _NO_INDEX: i32 = -1;
pub const _NODE_BLACK: i8 = 1;
pub const _NODE_RED: i8 = 0;

// Java: public class Property implements Child
impl Property {
    // Java: Property()
    pub fn new() -> Self {
        // Java: super();
        // Java: this._property_type = 0;
        // Java: this._node_color = 0;
        // Java: this._previous_property = -1;
        // Java: this._next_property = -1;
        // Java: this._child_property = -1;
        // Java: this._storage_clsid = new ClassID();
        // Java: this._start_block = 0;
        // Java: this._size = 0;
        // Java: this._index = -1;
        // Java: this._name = "";
        Property {
            _name: String::new(),
            _property_type: 0,
            _node_color: 0,
            _previous_property: -1,
            _next_property: -1,
            _child_property: -1,
            _storage_clsid: ClassID::new(),
            _start_block: 0,
            _size: 0,
            _index: -1,
            _next_child: None,
            _previous_child: None,
        }
    }

    // Java: Property(String name, byte propertyType, byte color, int startBlock, int size)
    pub fn new_with_params(name: String, propertyType: i8, color: i8, startBlock: i32, size: i32) -> Self {
        // Java: super();
        // Java: this._property_type = propertyType;
        // Java: this._node_color = color;
        // Java: this._previous_property = -1;
        // Java: this._next_property = -1;
        // Java: this._child_property = -1;
        // Java: this._storage_clsid = new ClassID();
        // Java: this._start_block = startBlock;
        // Java: this._size = size;
        // Java: this._index = -1;
        // Java: this._name = name;
        Property {
            _name: name,
            _property_type: propertyType,
            _node_color: color,
            _previous_property: -1,
            _next_property: -1,
            _child_property: -1,
            _storage_clsid: ClassID::new(),
            _start_block: startBlock,
            _size: size,
            _index: -1,
            _next_child: None,
            _previous_child: None,
        }
    }

    // Java: Property(int index, byte[] array, int offset)
    pub fn new_from_array(index: i32, array: &[u8], offset: i32) -> Self {
        // Java: super();
        let off = offset as usize;
        // Java: int name_size = LittleEndian.getShort(array, offset + 64);
        let name_size = le_get_short(array, off + 64) as i32;
        // Java: this._property_type = LittleEndian.getByte(array, offset + 66);
        let _property_type = array[off + 66] as i8;
        // Java: this._node_color = LittleEndian.getByte(array, offset + 67);
        let _node_color = array[off + 67] as i8;
        // Java: this._previous_property = LittleEndian.getInt(array, offset + 68);
        let _previous_property = le_get_int(array, off + 68);
        // Java: this._next_property = LittleEndian.getInt(array, offset + 72);
        let _next_property = le_get_int(array, off + 72);
        // Java: this._child_property = LittleEndian.getInt(array, offset + 76);
        let _child_property = le_get_int(array, off + 76);
        // Java: this._storage_clsid = new ClassID(array, offset + 80);
        let _storage_clsid = ClassID::new_from_bytes(array, offset + 80);
        // Java: this._start_block = LittleEndian.getInt(array, offset + 116);
        let _start_block = le_get_int(array, off + 116);
        // Java: this._size = LittleEndian.getInt(array, offset + 120);
        let _size = le_get_int(array, off + 120);
        // Java: this._index = index;
        let _index = index;
        // Java: int name_length = name_size / 2 - 1;
        let name_length = name_size / 2 - 1;
        let _name;
        // Java: if (name_length < 1)
        if name_length < 1 {
            // Java: this._name = "";
            _name = String::new();
        }
        // Java: else
        else {
            // Java: char[] char_array = CharArrayPool.sInstance.obtain(name_length);
            let nl = name_length as usize;
            let mut char_array = vec![0u16; nl];
            // Java: for(int i = 0; i < name_length; ++i)
            for i in 0..nl {
                // Java: char_array[i] = (char)LittleEndian.getUShort(array, offset + (i << 1));
                char_array[i] = le_get_ushort(array, off + (i << 1));
            }
            // Java: this._name = new String(char_array, 0, name_length);
            _name = String::from_utf16_lossy(&char_array[..nl]);
            // Java: CharArrayPool.sInstance.give(char_array);
        }
        // Java: this._next_child = null;
        // Java: this._previous_child = null;
        Property {
            _name,
            _property_type,
            _node_color,
            _previous_property,
            _next_property,
            _child_property,
            _storage_clsid,
            _start_block,
            _size,
            _index,
            _next_child: None,
            _previous_child: None,
        }
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: this._name = null;
        self._name = String::new();
        // Java: this._storage_clsid = null;
        // Java: this._next_child = null;
        self._next_child = None;
        // Java: this._previous_child = null;
        self._previous_child = None;
    }

    // Java: void writeData(OutputStream stream)
    pub fn writeData(&self, stream: &mut dyn std::io::Write) -> std::io::Result<()> {
        // Java: PageBuffer pb = PageBuffer.Pool.InstanceSmall.obtain();
        // Java: byte[] buff = pb.getByteArray();
        let mut buff = vec![0u8; 64];
        // Java: PageBuffer.fillBy0(pb);
        // Java: String name = null == this._name ? "" : this._name;
        let name = &self._name;
        // Java: int n = name.length();
        let n = name.chars().count();
        // Java: if (n != 0)
        if n != 0 {
            // Java: int offset = 0;
            let mut offset: usize = 0;
            // Java: char[] chars = name.toCharArray();
            let chars: Vec<u16> = name.encode_utf16().collect();
            // Java: for(int i = 0; i < n; ++i)
            for i in 0..chars.len() {
                // Java: LittleEndian.putUShort(buff, offset, chars[i]);
                le_put_ushort(&mut buff, offset, chars[i]);
                // Java: offset += 2;
                offset += 2;
            }
        }
        // Java: stream.write(buff);
        stream.write_all(&buff)?;
        // Java: PageBuffer.fillBy0(pb);
        buff.fill(0);
        // Java: int offset = -64;
        let base: i32 = -64;
        // Java: LittleEndian.putShort(buff, offset + 64, (short)((n + 1) * 2));
        le_put_short(&mut buff, (base + 64) as usize, ((n + 1) * 2) as i16);
        // Java: LittleEndian.putByte(buff, offset + 66, this._property_type);
        buff[(base + 66) as usize] = self._property_type as u8;
        // Java: LittleEndian.putByte(buff, offset + 67, this._node_color);
        buff[(base + 67) as usize] = self._node_color as u8;
        // Java: LittleEndian.putInt(buff, offset + 68, this._previous_property);
        le_put_int(&mut buff, (base + 68) as usize, self._previous_property);
        // Java: LittleEndian.putInt(buff, offset + 72, this._next_property);
        le_put_int(&mut buff, (base + 72) as usize, self._next_property);
        // Java: LittleEndian.putInt(buff, offset + 76, this._child_property);
        le_put_int(&mut buff, (base + 76) as usize, self._child_property);
        // Java: this._storage_clsid.write(buff, offset + 80);
        self._storage_clsid.write(&mut buff, (base + 80) as i32);
        // Java: LittleEndian.putInt(buff, offset + 116, this._start_block);
        le_put_int(&mut buff, (base + 116) as usize, self._start_block);
        // Java: LittleEndian.putInt(buff, offset + 120, this._size);
        le_put_int(&mut buff, (base + 120) as usize, self._size);
        // Java: stream.write(buff);
        stream.write_all(&buff)?;
        // Java: PageBuffer.Pool.InstanceSmall.recycle(pb);
        Ok(())
    }

    // Java: void setStartBlock(int startBlock)
    pub fn setStartBlock(&mut self, startBlock: i32) {
        // Java: this._start_block = startBlock;
        self._start_block = startBlock;
    }

    // Java: int getStartBlock()
    pub fn getStartBlock(&self) -> i32 {
        // Java: return this._start_block;
        self._start_block
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
        // Java: return this._size;
        self._size
    }

    // Java: boolean shouldUseSmallBlocks()
    pub fn shouldUseSmallBlocks(&self) -> bool {
        // Java: return this._size < 4096;
        self._size < 4096
    }

    // Java: static boolean isSmall(int length)
    pub fn isSmall(length: i32) -> bool {
        // Java: return length < 4096;
        length < 4096
    }

    // Java: String getName()
    pub fn getName(&self) -> &str {
        // Java: return this._name;
        &self._name
    }

    // Java: abstract boolean isDirectory()

    // Java: ClassID getStorageClsid()
    pub fn getStorageClsid(&self) -> &ClassID {
        // Java: return this._storage_clsid;
        &self._storage_clsid
    }

    // Java: void setName(String name)
    pub fn setName(&mut self, name: String) {
        // Java: this._name = name;
        self._name = name;
    }

    // Java: void setStorageClsid(ClassID clsidStorage)
    pub fn setStorageClsid(&mut self, clsidStorage: ClassID) {
        // Java: this._storage_clsid = clsidStorage;
        self._storage_clsid = clsidStorage;
    }

    // Java: void setPropertyType(byte propertyType)
    pub fn setPropertyType(&mut self, propertyType: i8) {
        // Java: this._property_type = propertyType;
        self._property_type = propertyType;
    }

    // Java: void setNodeColor(byte nodeColor)
    pub fn setNodeColor(&mut self, nodeColor: i8) {
        // Java: this._node_color = nodeColor;
        self._node_color = nodeColor;
    }

    // Java: void setChildProperty(int child)
    pub fn setChildProperty(&mut self, child: i32) {
        // Java: this._child_property = child;
        self._child_property = child;
    }

    // Java: int getChildIndex()
    pub fn getChildIndex(&self) -> i32 {
        // Java: return this._child_property;
        self._child_property
    }

    // Java: void setSize(int size)
    pub fn setSize(&mut self, size: i32) {
        // Java: this._size = size;
        self._size = size;
    }

    // Java: void setIndex(int index)
    pub fn setIndex(&mut self, index: i32) {
        // Java: this._index = index;
        self._index = index;
    }

    // Java: int getIndex()
    pub fn getIndex(&self) -> i32 {
        // Java: return this._index;
        self._index
    }

    // Java: abstract void preWrite()

    // Java: int getNextChildIndex()
    pub fn getNextChildIndex(&self) -> i32 {
        // Java: return this._next_property;
        self._next_property
    }

    // Java: int getPreviousChildIndex()
    pub fn getPreviousChildIndex(&self) -> i32 {
        // Java: return this._previous_property;
        self._previous_property
    }

    // Java: static boolean isValidIndex(int index)
    pub fn isValidIndex(index: i32) -> bool {
        // Java: return index != -1;
        index != -1
    }

    // Java: Child getNextChild()
    pub fn getNextChild_impl(&self) -> Option<Arc<dyn Child>> {
        // Java: return this._next_child;
        self._next_child.clone()
    }

    // Java: Child getPreviousChild()
    pub fn getPreviousChild_impl(&self) -> Option<Arc<dyn Child>> {
        // Java: return this._previous_child;
        self._previous_child.clone()
    }

    // Java: void setNextChild(Child child)
    pub fn setNextChild_impl(&mut self, child: Option<Arc<dyn Child>>) {
        // Java: this._next_child = child;
        self._next_child = child;
        // Java: this._next_property = child == null ? -1 : ((Property)child).getIndex();
        // Note: in Rust we cannot downcast easily, so this needs external coordination
    }

    // Java: void setPreviousChild(Child child)
    pub fn setPreviousChild_impl(&mut self, child: Option<Arc<dyn Child>>) {
        // Java: this._previous_child = child;
        self._previous_child = child;
        // Java: this._previous_property = child == null ? -1 : ((Property)child).getIndex();
    }
}
