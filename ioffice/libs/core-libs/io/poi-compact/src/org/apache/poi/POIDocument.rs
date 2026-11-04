// Java: import cn.wps.base.log.IOLog
// Java: import java.io.ByteArrayInputStream
// Java: import java.io.ByteArrayOutputStream
// Java: import java.io.IOException
// Java: import java.io.OutputStream
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import org.apache.poi.hpsf.DocumentSummaryInformation
use crate::org::apache::poi::hpsf::DocumentSummaryInformation::DocumentSummaryInformation;
// Java: import org.apache.poi.hpsf.HPSFException
// Java: import org.apache.poi.hpsf.MutablePropertySet
// Java: import org.apache.poi.hpsf.PropertySet
use crate::org::apache::poi::hpsf::PropertySet::PropertySet;
// Java: import org.apache.poi.hpsf.PropertySetFactory
use crate::org::apache::poi::hpsf::PropertySetFactory;
// Java: import org.apache.poi.hpsf.SummaryInformation
use crate::org::apache::poi::hpsf::SummaryInformation::SummaryInformation;
// Java: import org.apache.poi.hpsf.WritingNotSupportedException
// Java: import org.apache.poi.poifs.filesystem.DirectoryEntry
// Java: import org.apache.poi.poifs.filesystem.DirectoryNode
use crate::org::apache::poi::poifs::filesystem::DirectoryNode::DirectoryNode;
// Java: import org.apache.poi.poifs.filesystem.DocumentEntry
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
// Java: import org.apache.poi.poifs.filesystem.Entry
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
use crate::org::apache::poi::poifs::filesystem::POIFSFileSystem::POIFSFileSystem;

// Java field: String TAG
const _TAG: Option<&str> = None;

// Java: public class POIDocument
pub struct POIDocument {
    // Java field: SummaryInformation sInf
    pub(crate) sInf: Option<SummaryInformation>,
    // Java field: DocumentSummaryInformation dsInf
    pub(crate) dsInf: Option<DocumentSummaryInformation>,
    // Java field: DirectoryNode directory
    pub(crate) directory: Option<DirectoryNode>,
    // Java field: boolean initialized
    pub(crate) initialized: bool,
}

// Java: public class POIDocument
impl POIDocument {
    // Java: POIDocument(DirectoryNode dir)
    pub fn new(dir: DirectoryNode) -> Self {
        // Java: super();
        POIDocument {
            sInf: None,
            dsInf: None,
            // Java: this.initialized = false;
            initialized: false,
            // Java: this.directory = dir;
            directory: Some(dir),
        }
    }

    // Java: POIDocument(DirectoryNode dir, POIFSFileSystem fs)
    pub fn new_with_fs(dir: DirectoryNode, _fs: &POIFSFileSystem) -> Self {
        // Java: super();
        POIDocument {
            sInf: None,
            dsInf: None,
            // Java: this.initialized = false;
            initialized: false,
            // Java: this.directory = dir;
            directory: Some(dir),
        }
    }

    // Java: POIDocument(POIFSFileSystem fs)
    pub fn new_from_fs(_fs: &POIFSFileSystem) -> Self {
        // Java: this(fs.getRoot());
        POIDocument {
            sInf: None,
            dsInf: None,
            initialized: false,
            directory: None,
        }
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: if (null != this.directory)
        if self.directory.is_some() {
            // Java: POIFSFileSystem fs = this.directory.getFileSystem();
            // Java: if (null != fs)
            {
                // Java: fs.dispose();
            }
            // Java: this.directory.dispose();
            // Java: this.directory = null;
            self.directory = None;
        }
    }

    // Java: DocumentSummaryInformation getDocumentSummaryInformation()
    pub fn getDocumentSummaryInformation(&mut self) -> Option<&DocumentSummaryInformation> {
        // Java: if (!this.initialized)
        if !self.initialized {
            // Java: this.readProperties();
            self.readProperties();
        }
        // Java: return this.dsInf;
        self.dsInf.as_ref()
    }

    // Java: SummaryInformation getSummaryInformation()
    pub fn getSummaryInformation(&mut self) -> Option<&SummaryInformation> {
        // Java: if (!this.initialized)
        if !self.initialized {
            // Java: this.readProperties();
            self.readProperties();
        }
        // Java: return this.sInf;
        self.sInf.as_ref()
    }

    // Java: void createInformationProperties()
    pub fn createInformationProperties(&mut self) {
        // Java: if (!this.initialized)
        if !self.initialized {
            // Java: this.readProperties();
            self.readProperties();
        }
        // Java: if (this.sInf == null)
        if self.sInf.is_none() {
            // Java: this.sInf = PropertySetFactory.newSummaryInformation();
            let ps = PropertySet::new();
            self.sInf = Some(SummaryInformation::new(&ps));
        }
        // Java: if (this.dsInf == null)
        if self.dsInf.is_none() {
            // Java: this.dsInf = PropertySetFactory.newDocumentSummaryInformation();
            let ps = PropertySet::new();
            self.dsInf = Some(DocumentSummaryInformation::new(&ps));
        }
    }

    // Java: void readProperties()
    pub fn readProperties(&mut self) {
        // Java: PropertySet ps = this.getPropertySet("\u0005DocumentSummaryInformation");
        // Java: if (ps != null && ps instanceof DocumentSummaryInformation)
        {
            // Java: this.dsInf = (DocumentSummaryInformation)ps;
        }
        // Java: else if (ps != null)
        {
        }
        // Java: ps = this.getPropertySet("\u0005SummaryInformation");
        // Java: if (ps instanceof SummaryInformation)
        {
            // Java: this.sInf = (SummaryInformation)ps;
        }
        // Java: else if (ps != null)
        {
        }
        // Java: this.initialized = true;
        self.initialized = true;
    }

    // Java: PropertySet getPropertySet(String setName)
    pub fn getPropertySet(&self, _setName: &str) -> Option<PropertySet> {
        // Java: if (this.directory == null)
        if self.directory.is_none() {
            // Java: return null;
            return None;
        }
        // Java: else
        else {
            // Java: DocumentInputStream dis;
            // Java: try
            // Java: dis = this.directory.createDocumentInputStream(this.directory.getEntry(setName));
            // Java: catch (IOException ie)
            // Java: IOLog.e(TAG, "Error getting property set with name " + setName + "\n" + ie);
            // Java: return null;
            // Java: try
            // Java: PropertySet set = PropertySetFactory.create(dis);
            // Java: PropertySet var4 = set;
            // Java: return var4;
            // Java: catch (IOException ie)
            // Java: IOLog.e(TAG, "Error creating property set with name " + setName + "\n" + ie);
            // Java: catch (HPSFException he)
            // Java: IOLog.e(TAG, "Error creating property set with name " + setName + "\n" + he);
            // Java: finally
            // Java: if (null != dis)
            // Java: dis.close();
            // Java: DocumentInputStream var14 = null;
            // Java: return null;
            None
        }
    }

    // Java: void writeProperties(POIFSFileSystem outFS)
    pub fn writeProperties_fs(&mut self, outFS: &mut POIFSFileSystem) -> std::io::Result<()> {
        // Java: this.writeProperties(outFS, (List)null);
        self.writeProperties_with_entries(outFS, None)
    }

    // Java: void writeProperties(POIFSFileSystem outFS, List<String> writtenEntries)
    pub fn writeProperties_with_entries(&mut self, _outFS: &mut POIFSFileSystem, _writtenEntries: Option<&mut Vec<String>>) -> std::io::Result<()> {
        // Java: SummaryInformation si = this.getSummaryInformation();
        // Java: if (si != null)
        {
            // Java: this.writePropertySet("\u0005SummaryInformation", si, outFS);
            // Java: if (writtenEntries != null)
            {
                // Java: writtenEntries.add("\u0005SummaryInformation");
            }
        }
        // Java: DocumentSummaryInformation dsi = this.getDocumentSummaryInformation();
        // Java: if (dsi != null)
        {
            // Java: this.writePropertySet("\u0005DocumentSummaryInformation", dsi, outFS);
            // Java: if (writtenEntries != null)
            {
                // Java: writtenEntries.add("\u0005DocumentSummaryInformation");
            }
        }
        Ok(())
    }

    // Java: void writePropertySet(String name, PropertySet set, POIFSFileSystem outFS)
    pub fn writePropertySet(&self, _name: &str, _set: &PropertySet, _outFS: &mut POIFSFileSystem) -> std::io::Result<()> {
        // Java: try
        {
            // Java: MutablePropertySet mSet = new MutablePropertySet(set);
            // Java: ByteArrayOutputStream bOut = new ByteArrayOutputStream();
            // Java: mSet.write(bOut);
            // Java: byte[] data = bOut.toByteArray();
            // Java: ByteArrayInputStream bIn = new ByteArrayInputStream(data);
            // Java: outFS.createDocument(bIn, name);
        }
        // Java: catch (WritingNotSupportedException var8)
        {
            // Java: System.err.println("Couldn't write property set with name " + name + " as not supported by HPSF yet");
            eprintln!("Couldn't write property set with name {} as not supported by HPSF yet", _name);
        }
        Ok(())
    }

    // Java: abstract void write(OutputStream var1)

    // Java: void copyNodes(POIFSFileSystem source, POIFSFileSystem target, List<String> excepts)
    pub fn copyNodes(&self, _source: &POIFSFileSystem, _target: &mut POIFSFileSystem, _excepts: &[String]) -> std::io::Result<()> {
        // Java: DirectoryEntry root = source.getRoot();
        // Java: DirectoryEntry newRoot = target.getRoot();
        // Java: Iterator<Entry> entries = root.getEntries();
        // Java: while(entries.hasNext())
        {
            // Java: Entry entry = (Entry)entries.next();
            // Java: if (!excepts.contains(entry.getName()))
            {
                // Java: this.copyNodeRecursively(entry, newRoot);
            }
        }
        Ok(())
    }

    // Java: void copyNodeRecursively(Entry entry, DirectoryEntry target)
    fn copyNodeRecursively(&self, _entry_name: &str, _is_dir: bool) -> std::io::Result<()> {
        // Java: DirectoryEntry newTarget = null;
        // Java: if (entry.isDirectoryEntry())
        if _is_dir {
            // Java: newTarget = target.createDirectory(entry.getName());
            // Java: Iterator<Entry> entries = ((DirectoryEntry)entry).getEntries();
            // Java: while(entries.hasNext())
            {
                // Java: this.copyNodeRecursively((Entry)entries.next(), newTarget);
            }
        }
        // Java: else
        else {
            // Java: DocumentEntry dentry = (DocumentEntry)entry;
            // Java: DocumentInputStream dstream = new DocumentInputStream(dentry);
            // Java: target.createDocument(dentry.getName(), dstream, true);
            // Java: dstream.close();
        }
        Ok(())
    }
}
