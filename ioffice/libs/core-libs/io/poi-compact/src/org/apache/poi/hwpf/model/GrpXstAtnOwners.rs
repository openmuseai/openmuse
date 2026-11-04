// Java: import java.io.IOException
// Java: import java.util.ArrayList
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
use super::io::HWPFOutputStream::HWPFOutputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
use super::Xst::Xst;
use std::io::Write;

// Java: public class GrpXstAtnOwners
pub struct GrpXstAtnOwners {
    // Java field: int numberOfBytes
    numberOfBytes: i32,
    // Java field: ArrayList<Xst> owners
    pub(crate) owners: Vec<Xst>,
}

// Java: public class GrpXstAtnOwners
impl GrpXstAtnOwners {
    // Java: GrpXstAtnOwners()
    pub fn new() -> Self {
      // Java: super();
      GrpXstAtnOwners { numberOfBytes: 0, owners: Vec::new() }
    }

    // Java: GrpXstAtnOwners(DocumentInputStream tableStream, int offset, int size)
    pub fn new_from_stream(table_stream: &mut DocumentInputStream, mut offset: i32, mut size: i32) -> Self {
      // Java: super();
      let mut holder = Self::new();
      // Java: this.numberOfBytes = size;
      holder.numberOfBytes = size;
      // Java: while(size > 0)
      while size > 0
      {
         // Java: Xst xst = new Xst(tableStream, offset);
         let xst = Xst::new_from_stream(table_stream, offset);
         // Java: int len = xst.getNumberOfChars() * 2 + 2;
         let len = xst.getNumberOfChars() * 2 + 2;
         // Java: offset += len;
         offset += len;
         // Java: size -= len;
         size -= len;
         // Java: this.owners.add(xst);
         holder.owners.push(xst);
      }
      holder
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
      // Java: return this.owners.size();
      self.owners.len() as i32
    }

    // Java: void addAtnOwner(Xst xst)
    pub fn addAtnOwner(&mut self, xst: Xst) {
      // Java: this.owners.add(xst);
      // Java: this.numberOfBytes += xst.getNumberOfChars() * 2 + 2;
      self.numberOfBytes += xst.getNumberOfChars() * 2 + 2;
      self.owners.push(xst);
    }

    // Java: Xst getAtnOwner(int index)
    pub fn getAtnOwner(&self, index: i32) -> Option<&Xst> {
      // Java: return index >= 0 && index < this.owners.size() ? (Xst)this.owners.get(index) : null;
      if index >= 0 && (index as usize) < self.owners.len() {
          Some(&self.owners[index as usize])
      } else {
          None
      }
    }

    // Java: void writeTo(HWPFOutputStream tableStream)
    pub fn writeTo(&self, table_stream: &mut HWPFOutputStream) {
      // Java: if (this.getSize() > 0)
      if self.getSize() > 0
      {
         // Java: int offset = 0;
         let mut offset: i32 = 0;
         // Java: int count = this.getSize();
         let count = self.getSize();
         // Java: byte[] buf = new byte[this.numberOfBytes];
         let mut buf = vec![0u8; self.numberOfBytes as usize];
         // Java: for(int i = 0; i < count; ++i)
         for i in 0..count
         {
            // Java: Xst xst = (Xst)this.owners.get(i);
            let xst = &self.owners[i as usize];
            // Java: xst.serialize(buf, offset);
            xst.serialize(&mut buf, offset);
            // Java: offset += xst.getNumberOfChars() * 2 + 2;
            offset += xst.getNumberOfChars() * 2 + 2;
         }
         // Java: tableStream.write(buf);
         let _ = table_stream.write(&buf);
      }
    }

}
