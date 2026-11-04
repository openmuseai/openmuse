use util::org::apache::poi::util::LittleEndianOutput::LittleEndianOutput;
// Native superclass vtable for StandardRecord's stream serializer. The original
// byte-array and RecordInputStream overloads require their buffer/reader graph
// and are not represented by empty methods or a second record format.
pub trait StandardRecord{
    // java protected abstract int getDataSize();
    fn getDataSize(&self)->i32;
    // java Record.getSid and protected abstract void serializeData(LittleEndianOutput var1);
    fn getSid(&self)->i16;
    fn serializeData(&self,out:&mut dyn LittleEndianOutput);
    // java Record.reSizeRecord(){try{this.reSizeRecordImpl();}catch(RuntimeException var2){}}
    fn reSizeRecord(&self){if let Err(e)=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||self.reSizeRecordImpl())){let s=e.downcast_ref::<String>().map(String::as_str).or_else(||e.downcast_ref::<&str>().copied()).unwrap_or("");if !s.starts_with("RuntimeException"){std::panic::resume_unwind(e);}}}
    // java Record.reSizeRecordImpl() { }
    fn reSizeRecordImpl(&self){}
    // java public final int getRecordSize(){return 4+this.getDataSize();}
    fn getRecordSize(&self)->i32{4i32.wrapping_add(self.getDataSize())}
    // java public int serialize(LittleEndianOutput out){this.reSizeRecord();out.writeShort(this.getSid());int datasize=this.getDataSize();if(datasize>8224){out.writeShort(8224);this.serializeBigData(out);return 4+datasize;}else{out.writeShort(datasize);this.serializeData(out);return 4+datasize;}}
    fn serialize(&self,out:&mut dyn LittleEndianOutput)->i32{self.reSizeRecord();out.writeShort(self.getSid()as i32);let datasize=self.getDataSize();if datasize>8224{out.writeShort(8224);self.serializeBigData(out);4i32.wrapping_add(datasize)}else{out.writeShort(datasize);self.serializeData(out);4i32.wrapping_add(datasize)}}
    // java protected void serializeBigData(LittleEndianOutput out){this.serializeData(out);}
    fn serializeBigData(&self,out:&mut dyn LittleEndianOutput){self.serializeData(out);}
}
