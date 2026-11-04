// Java: import cn.wps.base.assertion.KSAssert
// Java: import java.io.FileNotFoundException
// Java: import java.io.IOException
// Java: import java.util.ArrayList

use super::KOLEObjectBase::KOLEObjectBase;
use super::KDirEntry::KDirEntry;

// Java: public class KOLEStorage extends KOLEObjectBase implements OLEConstants, IOLEStorage
pub struct KOLEStorage {
    // Java field: KOLEStorage mParent
    pub(crate) mParent: Option<Box<dyn std::any::Any>>,
    // Java field: DirectoryManager mDirectoryManager
    mDirectoryManager: Option<Box<dyn std::any::Any>>,
    // Java field: KDirEntry mEntry
    pub(crate) mEntry: Option<KDirEntry>,
    pub(crate) base: KOLEObjectBase,
}

// Java: public class KOLEStorage extends KOLEObjectBase implements OLEConstants, IOLEStorage
impl KOLEStorage {
    // Java: KOLEDocument getDocument()
    pub fn getDocument(&self) -> Option<&Box<dyn std::any::Any>> {
      // Java: return this.mDocument;
      todo!()
    }

    // Java: KOLEStorage getParent()
    pub fn getParent(&self) -> Option<&Box<dyn std::any::Any>> {
      // Java: return this.mParent;
      self.mParent.as_ref()
    }

    // Java: KDirEntry getEntry()
    pub fn getEntry(&self) -> Option<&KDirEntry> {
      // Java: return this.mEntry;
      self.mEntry.as_ref()
    }

    // Java: DirectoryManager getDirectoryManager()
    pub fn getDirectoryManager(&self) -> Option<&Box<dyn std::any::Any>> {
      // Java: return this.mDirectoryManager;
      self.mDirectoryManager.as_ref()
    }

    // Java: void close()
    pub fn close(&mut self) {
    }

    // Java: IOLEStream openStream(String szName)
    pub fn openStream(&self, _szName: &str) -> Option<Box<dyn std::any::Any>> {
      // Java: KDirEntry pEntry = this.findChild(szName);

      // Java: if (pEntry != null && pEntry.getType() == 2)

      {
         // Java: KOLEStream pStream = new KOLEStream(this, pEntry);

         // Java: return pStream;

      }
      // Java: else

      {
         // Java: throw new FileNotFoundException("Can't find the specified stream: " + szName);

      }
      todo!()
    }

    // Java: IOLEStorage openStorage(String szName)
    pub fn openStorage(&self, _szName: &str) -> Option<Box<dyn std::any::Any>> {
      // Java: KDirEntry pEntry = this.findChild(szName);

      // Java: if (pEntry != null && pEntry.getType() == 1)

      {
         // Java: KOLEStorage pStorage = new KOLEStorage(this.mDocument, this, pEntry);

         // Java: return pStorage;

      }
      // Java: else

      {
         // Java: throw new FileNotFoundException("Can't find the specified storage: " + szName);

      }
      todo!()
    }

    // Java: IOLEStorage createStorage(String szName)
    pub fn createStorage(&mut self, _szName: &str) -> Option<Box<dyn std::any::Any>> {
      // Java: KDirEntry pEntry = this.findChild(szName);

      // Java: if (pEntry != null && pEntry.getType() != 1)

      {
         // Java: throw new OLEException("An entry which is not storage already exists: " + pEntry.getName());

      }
      // Java: else

      {
         // Java: if (pEntry == null)

         {
            // Java: pEntry = this.mDirectoryManager.allocEntry();

            // Java: pEntry.setType(1);

            // Java: pEntry.setName(szName);

            // Java: this.addChild(pEntry);

         }
         // Java: else

         {
            // Java: pEntry.setName(szName);

         }
         // Java: KOLEStorage pStorage = new KOLEStorage(this.mDocument, this, pEntry);

         // Java: return pStorage;

      }
      todo!()
    }

    // Java: IOLEStream createStream(String szName)
    pub fn createStream(&mut self, _szName: &str) -> Option<Box<dyn std::any::Any>> {
      // Java: KDirEntry pEntry = this.findChild(szName);

      // Java: if (pEntry != null && pEntry.getType() != 2)

      {
         // Java: throw new OLEException("An entry which is not stream already exists: " + pEntry.getName());

      }
      // Java: else

      {
         // Java: if (pEntry == null)

         {
            // Java: pEntry = this.mDirectoryManager.allocEntry();

            // Java: pEntry.setType(2);

            // Java: pEntry.setName(szName);

            // Java: this.addChild(pEntry);

         }
         // Java: else

         {
            // Java: pEntry.setName(szName);

         }
         // Java: KOLEStream pStream = new KOLEStream(this, pEntry);

         // Java: return pStream;

      }
      todo!()
    }

    // Java: void renameElement(String szOleName, String szNewName)
    pub fn renameElement(&mut self, _szOleName: &str, _szNewName: &str) {
      // Java: KDirEntry pEntry = this.findChild(szNewName);

      // Java: if (pEntry != null)

      {
         // Java: throw new OLEException("The new name is not available: " + szNewName);

      }
      // Java: else

      {
         // Java: pEntry = this.findChild(szOleName);

         // Java: if (pEntry == null)

         {
            // Java: throw new FileNotFoundException("The specified entry is not found: " + szOleName);

         }
         // Java: else

         {
            // Java: pEntry.setName(szNewName);

         }
      }
    }

    // Java: static void get_tree(DirectoryManager pDirectoryManager, KDirEntry root, ArrayList<KDirEntry> children)
    pub fn get_tree(_pDirectoryManager: &dyn std::any::Any, _root: Option<&KDirEntry>, _children: &mut Vec<Box<dyn std::any::Any>>) {
      // Java: if (root != null)

      {
         // Java: children.add(root);

         // Java: get_tree(pDirectoryManager, root.getLeft(), children);

         // Java: get_tree(pDirectoryManager, root.getRight(), children);

      }
    }

    // Java: void getChildren(ArrayList<KDirEntry> children)
    pub fn getChildren(&self, _children: &mut Vec<Box<dyn std::any::Any>>) {
      // Java: KDirEntry pEntry = this.getEntry();

      // Java: get_tree(this.getDirectoryManager(), pEntry.getChildren(), children);

    }

    // Java: int getElementCount()
    pub fn getElementCount(&self) -> i32 {
      // Java: ArrayList<KDirEntry> children = new ArrayList();

      // Java: this.getChildren(children);

      // Java: return children.size();

      todo!()
    }

    // Java: void getElements(int index, int count, OLEEntry[] buf)
    pub fn getElements(&self, _index: i32, _count: i32, _buf: &mut [Box<dyn std::any::Any>]) {
      // Java: ArrayList<KDirEntry> children = new ArrayList();

      // Java: this.getChildren(children);

      // Java: while(index < children.size())

      {
         // Java: KDirEntry pEntry = (KDirEntry)children.get(index);

         // Java: setEntry(buf[index], pEntry);

         // Java: ++index;

      }
    }

    // Java: static void wstring_toupper(char[] p)
    pub fn wstring_toupper(_p: &mut [char]) {
      // Java: int length = p.length;

      // Java: for(int i = 0; i < length; ++i)

      {
         // Java: char c = p[i];

         // Java: if ('a' <= c && c <= 'z')

         {
            // Java: p[i] = (char)(c - 97 + 65);

         }
      }
    }

    // Java: static int wcscmp(char[] a, char[] b, int length)
    pub fn wcscmp(_a: &[char], _b: &[char], _length: i32) -> i32 {
      // Java: for(int i = 0; i < length; ++i)

      {
         // Java: if (a[i] != b[i])

         {
            // Java: return a[i] - b[i];

         }
      }
      // Java: return 0;

      todo!()
    }

    // Java: static int compare_entry_name(String a, int aLen, String b, int bLen)
    pub fn compare_entry_name(_a: &str, _aLen: i32, _b: &str, _bLen: i32) -> i32 {
      // Java: if (aLen != bLen)

      {
         // Java: return aLen - bLen;

      }
      // Java: else

      {
         // Java: char[] namea = new char[aLen];

         // Java: char[] nameb = new char[bLen];

         // Java: a.getChars(0, aLen, namea, 0);

         // Java: b.getChars(0, bLen, nameb, 0);

         // Java: wstring_toupper(namea);

         // Java: wstring_toupper(nameb);

         // Java: return wcscmp(namea, nameb, aLen);

      }
      todo!()
    }

    // Java: static int compare_entry(KDirEntry a, KDirEntry b)
    pub fn compare_entry(_a: &KDirEntry, _b: &KDirEntry) -> i32 {
      // Java: return compare_entry_name(a.getName(), a.getNameLength(), b.getName(), b.getNameLength());

      todo!()
    }

    // Java: KDirEntry findChild(String szName)
    pub fn findChild(&self, _szName: &str) -> Option<&KDirEntry> {
      // Java: KDirEntry pMyEntry = this.mEntry;

      // Java: KDirEntry x = pMyEntry.getChildren();

      // Java: int nameLen = szName.length();

      // Java: while(x != null)

      {
         // Java: int cmp = compare_entry_name(szName, nameLen, x.getName(), x.getNameLength());

         // Java: if (cmp == 0)

         {
            // Java: return x;

         }
         // Java: if (cmp < 0)

         {
            // Java: x = x.getLeft();

         }
         // Java: else

         {
            // Java: x = x.getRight();

         }
      }
      // Java: return null;

      todo!()
    }

    // Java: void addChild(KDirEntry pChildEntry)
    pub fn addChild(&mut self, _pChildEntry: &mut KDirEntry) {
      // Java: KDirEntry pMyEntry = this.mEntry;

      // Java: pChildEntry.setColor(1);

      // Java: if (pMyEntry.getChildren() == null)

      {
         // Java: pMyEntry.setChildren(pChildEntry);

      }
      // Java: else

      {
         // Java: KDirEntry z = pChildEntry;

         // Java: KDirEntry y = null;

         // Java: KDirEntry x = pMyEntry.getChildren();

         // Java: while(x != null)

         {
            // Java: y = x;

            // Java: if (compare_entry(z, x) < 0)

            {
               // Java: x = x.getLeft();

            }
            // Java: else

            {
               // Java: x = x.getRight();

            }
         }
         // Java: if (compare_entry(z, y) < 0)

         {
            // Java: y.setLeft(z);

         }
         // Java: else

         {
            // Java: y.setRight(z);

         }
      }
    }

    // Java: byte[] getStorageClsid()
    pub fn getStorageClsid(&self) -> Option<&Vec<u8>> {
      // Java: return this.mEntry.getStorageClsid();
      todo!()
    }

    // Java: void setStorageClsid(byte[] clsid)
    pub fn setStorageClsid(&mut self, _clsid: Vec<u8>) {
      // Java: this.mEntry.setStorageClsid(clsid);

    }

    // Java: boolean destroyElement(String name)
    pub fn destroyElement(&mut self, _name: &str) -> bool {
      // Java: KSAssert.fail("fail");

      // Java: return false;

      todo!()
    }

}

impl super::OLEConstants::OLEConstants for KOLEStorage {
}

impl super::IOLEStorage::IOLEStorage for KOLEStorage {
    fn openStream(&self, var1: &str) -> Box<dyn std::any::Any> { todo!() }
    fn openStorage(&self, var1: &str) -> Box<dyn std::any::Any> { todo!() }
    fn createStream(&mut self, var1: &str) -> Box<dyn std::any::Any> { todo!() }
    fn createStorage(&mut self, var1: &str) -> Box<dyn std::any::Any> { todo!() }
    fn renameElement(&mut self, var1: &str, var2: &str) {}
    fn getElementCount(&self) -> i32 { todo!() }
    fn getElements(&self, var1: i32, var2: i32, var3: &mut [Box<dyn std::any::Any>]) {}
    fn getStorageClsid(&self) -> Vec<u8> { todo!() }
    fn setStorageClsid(&mut self, var1: &[u8]) {}
    fn destroyElement(&mut self, var1: &str) -> bool { todo!() }
}
