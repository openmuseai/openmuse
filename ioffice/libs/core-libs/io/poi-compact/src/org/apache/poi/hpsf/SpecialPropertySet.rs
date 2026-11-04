// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.OutputStream
// Java: import java.util.List
// Java: import org.apache.poi.hpsf.wellknown.PropertyIDMap
// Java: import org.apache.poi.poifs.filesystem.DirectoryEntry

use std::any::Any;
use super::MutablePropertySet::MutablePropertySet;
use super::PropertySet::{PropertySet, Section};
use super::ClassID::ClassID;

// Java: public class SpecialPropertySet extends MutablePropertySet
pub struct SpecialPropertySet {
    pub(crate) base: MutablePropertySet,
    // Java field: MutablePropertySet delegate
    pub(crate) delegate: MutablePropertySet,
}

// Java: public class SpecialPropertySet extends MutablePropertySet
impl SpecialPropertySet {
    // Java: abstract PropertyIDMap getPropertySetIDMap()

    // Java: SpecialPropertySet(PropertySet ps)
    pub fn new(ps: &PropertySet) -> Self {
        // Java: super();
        // Java: this.delegate = new MutablePropertySet(ps);
        SpecialPropertySet {
            base: MutablePropertySet::new(),
            delegate: MutablePropertySet::new_from_ps(ps),
        }
    }

    // Java: SpecialPropertySet(MutablePropertySet ps)
    pub fn new_from_mutable(ps: MutablePropertySet) -> Self {
        // Java: super();
        // Java: this.delegate = ps;
        SpecialPropertySet {
            base: MutablePropertySet::new(),
            delegate: ps,
        }
    }

    // Java: int getByteOrder()
    pub fn getByteOrder(&self) -> i32 {
        // Java: return this.delegate.getByteOrder();
        self.delegate.base.getByteOrder()
    }

    // Java: int getFormat()
    pub fn getFormat(&self) -> i32 {
        // Java: return this.delegate.getFormat();
        self.delegate.base.getFormat()
    }

    // Java: int getOSVersion()
    pub fn getOSVersion(&self) -> i32 {
        // Java: return this.delegate.getOSVersion();
        self.delegate.base.getOSVersion()
    }

    // Java: ClassID getClassID()
    pub fn getClassID(&self) -> &ClassID {
        // Java: return this.delegate.getClassID();
        self.delegate.base.getClassID()
    }

    // Java: int getSectionCount()
    pub fn getSectionCount(&self) -> i32 {
        // Java: return this.delegate.getSectionCount();
        self.delegate.base.getSectionCount()
    }

    // Java: List getSections()
    pub fn getSections(&self) -> &Vec<Section> {
        // Java: return this.delegate.getSections();
        self.delegate.base.getSections()
    }

    // Java: boolean isSummaryInformation()
    pub fn isSummaryInformation(&self) -> bool {
        // Java: return this.delegate.isSummaryInformation();
        self.delegate.base.isSummaryInformation()
    }

    // Java: boolean isDocumentSummaryInformation()
    pub fn isDocumentSummaryInformation(&self) -> bool {
        // Java: return this.delegate.isDocumentSummaryInformation();
        self.delegate.base.isDocumentSummaryInformation()
    }

    // Java: Section getFirstSection()
    pub fn getFirstSection(&self) -> &Section {
        // Java: return this.delegate.getFirstSection();
        self.delegate.base.getFirstSection()
    }

    // Java: void addSection(Section section)
    pub fn addSection(&mut self, section: Section) {
        // Java: this.delegate.addSection(section);
        self.delegate.addSection(section);
    }

    // Java: void clearSections()
    pub fn clearSections(&mut self) {
        // Java: this.delegate.clearSections();
        self.delegate.clearSections();
    }

    // Java: void setByteOrder(int byteOrder)
    pub fn setByteOrder(&mut self, byteOrder: i32) {
        // Java: this.delegate.setByteOrder(byteOrder);
        self.delegate.setByteOrder(byteOrder);
    }

    // Java: void setClassID(ClassID classID)
    pub fn setClassID(&mut self, classID: ClassID) {
        // Java: this.delegate.setClassID(classID);
        self.delegate.setClassID(classID);
    }

    // Java: void setFormat(int format)
    pub fn setFormat(&mut self, format: i32) {
        // Java: this.delegate.setFormat(format);
        self.delegate.setFormat(format);
    }

    // Java: void setOSVersion(int osVersion)
    pub fn setOSVersion(&mut self, osVersion: i32) {
        // Java: this.delegate.setOSVersion(osVersion);
        self.delegate.setOSVersion(osVersion);
    }

    // Java: InputStream toInputStream()
    pub fn toInputStream(&self) -> std::io::Result<Vec<u8>> {
        // Java: return this.delegate.toInputStream();
        self.delegate.toInputStream()
    }

    // Java: void write(DirectoryEntry dir, String name)
    pub fn write_to_dir(&self, dir: &mut dyn Any, name: &str) -> std::io::Result<()> {
        // Java: this.delegate.write(dir, name);
        self.delegate.write_to_dir(dir, name)
    }

    // Java: void write(OutputStream out)
    pub fn write(&self, out: &mut dyn std::io::Write) -> std::io::Result<()> {
        // Java: this.delegate.write(out);
        self.delegate.write(out)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, _o: Option<&SpecialPropertySet>) -> bool {
        // Java: return this.delegate.equals(o);
        false
    }

    // Java: Property[] getProperties()
    pub fn getProperties(&self) -> Vec<Box<dyn Any>> {
        // Java: return this.delegate.getProperties();
        Vec::new()
    }

    // Java: Object getProperty(int id)
    pub fn getProperty(&self, _id: i32) -> Option<Box<dyn Any>> {
        // Java: return this.delegate.getProperty(id);
        None
    }

    // Java: boolean getPropertyBooleanValue(int id)
    pub fn getPropertyBooleanValue(&self, _id: i32) -> bool {
        // Java: return this.delegate.getPropertyBooleanValue(id);
        false
    }

    // Java: int getPropertyIntValue(int id)
    pub fn getPropertyIntValue(&self, _id: i32) -> i32 {
        // Java: return this.delegate.getPropertyIntValue(id);
        0
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return this.delegate.hashCode();
        0
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return this.delegate.toString();
        String::from("SpecialPropertySet")
    }

    // Java: boolean wasNull()
    pub fn wasNull(&self) -> bool {
        // Java: return this.delegate.wasNull();
        false
    }
}
