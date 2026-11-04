// Java: import java.io.IOException
// Java: import java.util.ArrayList
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
use crate::org::apache::poi::hwpf::model::io::HWPFFileSystem::HWPFOutputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;

use super::ATRDPost::ATRDPost;

// Java: public class AtrdExtra
pub struct AtrdExtra {
    // Java field: int numberOfBytes
    numberOfBytes: i32,
    // Java field: ArrayList<ATRDPost> atrdPostList
    pub atrdPostList: Vec<ATRDPost>,
}

// Java: public class AtrdExtra
impl AtrdExtra {
    // Java: AtrdExtra()
    pub fn new() -> Self {
        // Java: super();
        AtrdExtra {
            numberOfBytes: 0,
            atrdPostList: Vec::new(),
        }
    }

    // Java: AtrdExtra(DocumentInputStream buf, int offset, int size)
    pub fn new_from_stream(buf: &mut DocumentInputStream, mut offset: i32, mut size: i32) -> Self {
        // Java: super();
        // Java: this.numberOfBytes = size;
        let number_of_bytes = size;
        let mut atrd_post_list = Vec::new();
        // Java: while(size > 0)
        while size > 0 {
            // Java: ATRDPost atrdPost = new ATRDPost(buf, offset);
            let atrd_post = ATRDPost::new_from_stream(buf, offset);
            // Java: offset += 18;
            offset += 18;
            // Java: size -= 18;
            size -= 18;
            // Java: this.atrdPostList.add(atrdPost);
            atrd_post_list.push(atrd_post);
        }
        AtrdExtra {
            numberOfBytes: number_of_bytes,
            atrdPostList: atrd_post_list,
        }
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
        // Java: return this.atrdPostList.size();
        self.atrdPostList.len() as i32
    }

    // Java: void addATRDPost(ATRDPost atrdPost)
    pub fn addATRDPost(&mut self, atrd_post: ATRDPost) {
        // Java: this.atrdPostList.add(atrdPost);
        self.atrdPostList.push(atrd_post);
        // Java: this.numberOfBytes += 18;
        self.numberOfBytes += 18;
    }

    // Java: ATRDPost getATRDPost(int index)
    pub fn getATRDPost(&self, index: i32) -> Option<&ATRDPost> {
        // Java: return index >= 0 && index < this.atrdPostList.size() ? (ATRDPost)this.atrdPostList.get(index) : null;
        if index >= 0 && (index as usize) < self.atrdPostList.len() {
            Some(&self.atrdPostList[index as usize])
        } else {
            None
        }
    }

    // Java: void writeTo(HWPFOutputStream tableStream)
    pub fn writeTo(&self, table_stream: &mut HWPFOutputStream) {
        // Java: if (this.getSize() > 0)
        if self.getSize() > 0 {
            // Java: int offset = 0;
            let mut offset: usize = 0;
            // Java: int count = this.getSize();
            let count = self.getSize();
            // Java: byte[] buf = new byte[this.numberOfBytes];
            let mut buf = vec![0u8; self.numberOfBytes as usize];
            // Java: for(int i = 0; i < count; ++i)
            for i in 0..count as usize {
                // Java: ATRDPost atrdPost = (ATRDPost)this.atrdPostList.get(i);
                let atrd_post = &self.atrdPostList[i];
                // Java: atrdPost.serialize(buf, offset);
                atrd_post.serialize(&mut buf, offset);
                // Java: offset += 18;
                offset += 18;
            }
            // Java: tableStream.write(buf);
            table_stream.write(&buf);
        }
    }
}
