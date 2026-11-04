#![allow(non_snake_case)]

use crate::cn::wps::io::opt::AttributesImpl::AttributesImpl;

/// Rust boundary for Java `org.xml.sax.Attributes` passed through the DML importer.
/// The product SAX reader supplies `AttributesImpl`; the Java return values of
/// `getIndex` and `getType` are deliberately preserved by that implementation.
pub trait SaxAttributes {
    fn getLength(&self) -> usize;
    fn getURI(&self, index: usize) -> Option<String>;
    fn getLocalName(&self, index: usize) -> Option<String>;
    fn getQName(&self, index: usize) -> Option<String>;
    fn getType_by_index(&self, index: usize) -> Option<String>;
    fn getValue_by_index(&self, index: usize) -> Option<String>;
    fn getIndex_by_uri(&self, uri: &str, localName: &str) -> i32;
    fn getIndex_by_qname(&self, qName: &str) -> i32;
    fn getType_by_uri(&self, uri: &str, localName: &str) -> Option<String>;
    fn getType_by_qname(&self, qName: &str) -> Option<String>;
    fn getValue_by_uri(&self, uri: &str, localName: &str) -> Option<String>;
    fn getValue_by_qname(&self, qName: &str) -> Option<String>;

    // Java passes the concrete SAX AttributesImpl. Recover it when the caller
    // only has this trait, so theme tags keep the same attribute object.
    fn as_attributes_impl(&self) -> Option<&AttributesImpl> {
        None
    }
}

impl SaxAttributes for AttributesImpl {
    fn getLength(&self) -> usize { AttributesImpl::getLength(self) }
    fn getURI(&self, index: usize) -> Option<String> { AttributesImpl::getURI(self, index) }
    fn getLocalName(&self, index: usize) -> Option<String> { AttributesImpl::getLocalName(self, index) }
    fn getQName(&self, index: usize) -> Option<String> { AttributesImpl::getQName(self, index) }
    fn getType_by_index(&self, index: usize) -> Option<String> { AttributesImpl::getType_by_index(self, index) }
    fn getValue_by_index(&self, index: usize) -> Option<String> { AttributesImpl::getValue_by_index(self, index) }
    fn getIndex_by_uri(&self, uri: &str, localName: &str) -> i32 { AttributesImpl::getIndex_by_uri(self, uri, localName) }
    fn getIndex_by_qname(&self, qName: &str) -> i32 { AttributesImpl::getIndex_by_qname(self, qName) }
    fn getType_by_uri(&self, uri: &str, localName: &str) -> Option<String> { AttributesImpl::getType_by_uri(self, uri, localName) }
    fn getType_by_qname(&self, qName: &str) -> Option<String> { AttributesImpl::getType_by_qname(self, qName) }
    fn getValue_by_uri(&self, uri: &str, localName: &str) -> Option<String> { AttributesImpl::getValue_by_uri(self, uri, localName) }
    fn getValue_by_qname(&self, qName: &str) -> Option<String> { AttributesImpl::getValue_by_qname(self, qName) }
    fn as_attributes_impl(&self) -> Option<&AttributesImpl> { Some(self) }
}
