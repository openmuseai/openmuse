// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.UnsupportedEncodingException
use std::io::{self, Read};
// Java: import org.apache.poi.hpsf.wellknown.SectionIDMap
use super::wellknown::SectionIDMap;
use super::PropertySet::PropertySet;
use super::SummaryInformation::SummaryInformation;
use super::DocumentSummaryInformation::DocumentSummaryInformation;
use super::MutablePropertySet::MutablePropertySet;

// Java: public class PropertySetFactory
pub struct PropertySetFactory {
}

// Java: public class PropertySetFactory
impl PropertySetFactory {
    // Java: PropertySetFactory()
    pub fn new() -> Self {
        // Java: super();
        PropertySetFactory {}
    }

    // Java: static PropertySet create(InputStream stream)
    pub fn create(stream: &mut dyn Read) -> io::Result<PropertySet> {
        // Java: PropertySet ps = new PropertySet(stream);
        let ps = PropertySet::new_from_stream(stream)?;
        // Java: try
        let result: io::Result<PropertySet> = (|| {
            // Java: if (ps.isSummaryInformation())
            if ps.isSummaryInformation() {
                // Java: return new SummaryInformation(ps);
                return Ok(SummaryInformation::new_from_property_set(ps)?.into());
            }
            // Java: else
            else {
                // Java: return (PropertySet)(ps.isDocumentSummaryInformation() ? new DocumentSummaryInformation(ps) : ps);
                if ps.isDocumentSummaryInformation() {
                    return Ok(DocumentSummaryInformation::new_from_property_set(ps)?.into());
                } else {
                    return Ok(ps);
                }
            }
        })();
        // Java: catch (UnexpectedPropertySetTypeException ex)
        match result {
            Ok(v) => Ok(v),
            Err(e) => {
                // Java: throw new IOException(ex.toString());
                return Err(io::Error::new(io::ErrorKind::Other, e.to_string()));
            }
        }
    }

    // Java: static SummaryInformation newSummaryInformation()
    pub fn newSummaryInformation() -> SummaryInformation {
        // Java: MutablePropertySet ps = new MutablePropertySet();
        let mut ps = MutablePropertySet::new();
        // Java: MutableSection s = (MutableSection)ps.getFirstSection();
        let s = ps.getFirstSectionMut();
        // Java: s.setFormatID(SectionIDMap.SUMMARY_INFORMATION_ID);
        s.setFormatID(SectionIDMap::SUMMARY_INFORMATION_ID.to_vec());
        // Java: try
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // Java: return new SummaryInformation(ps);
            SummaryInformation::new_from_mutable(ps)
        }));
        // Java: catch (UnexpectedPropertySetTypeException ex)
        match result {
            Ok(v) => v,
            // Java: throw new HPSFRuntimeException(ex);
            Err(_) => panic!("HPSFRuntimeException: UnexpectedPropertySetTypeException"),
        }
    }

    // Java: static DocumentSummaryInformation newDocumentSummaryInformation()
    pub fn newDocumentSummaryInformation() -> DocumentSummaryInformation {
        // Java: MutablePropertySet ps = new MutablePropertySet();
        let mut ps = MutablePropertySet::new();
        // Java: MutableSection s = (MutableSection)ps.getFirstSection();
        let s = ps.getFirstSectionMut();
        // Java: s.setFormatID(SectionIDMap.DOCUMENT_SUMMARY_INFORMATION_ID[0]);
        s.setFormatID(SectionIDMap::DOCUMENT_SUMMARY_INFORMATION_ID[0].to_vec());
        // Java: try
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // Java: return new DocumentSummaryInformation(ps);
            DocumentSummaryInformation::new_from_mutable(ps)
        }));
        // Java: catch (UnexpectedPropertySetTypeException ex)
        match result {
            Ok(v) => v,
            // Java: throw new HPSFRuntimeException(ex);
            Err(_) => panic!("HPSFRuntimeException: UnexpectedPropertySetTypeException"),
        }
    }
}
