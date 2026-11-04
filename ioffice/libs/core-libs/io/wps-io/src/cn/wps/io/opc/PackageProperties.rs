// Java: import cn.wps.io.xwpf.util.Nullable
// Java: import java.util.Date

// Java: public interface PackageProperties
pub trait PackageProperties {
    // Java field: String NAMESPACE_DCTERMS
    const NAMESPACE_DCTERMS: &'static str = "http://purl.org/dc/terms/";
    // Java field: String NAMESPACE_DC
    const NAMESPACE_DC: &'static str = "http://purl.org/dc/elements/1.1/";
    // Java: abstract Nullable<String> getCategoryProperty()
    fn getCategoryProperty(&self) -> Option<String>;

    // Java: abstract void setCategoryProperty(String var1)
    fn setCategoryProperty(&mut self, var1: &str);

    // Java: abstract Nullable<String> getContentStatusProperty()
    fn getContentStatusProperty(&self) -> Option<String>;

    // Java: abstract void setContentStatusProperty(String var1)
    fn setContentStatusProperty(&mut self, var1: &str);

    // Java: abstract Nullable<String> getContentTypeProperty()
    fn getContentTypeProperty(&self) -> Option<String>;

    // Java: abstract void setContentTypeProperty(String var1)
    fn setContentTypeProperty(&mut self, var1: &str);

    // Java: abstract Nullable<Date> getCreatedProperty()
    fn getCreatedProperty(&self) -> Option<i64>;

    // Java: abstract void setCreatedProperty(String var1)
    fn setCreatedProperty_str(&mut self, var1: &str);

    // Java: abstract void setCreatedProperty(Nullable<Date> var1)
    fn setCreatedProperty(&mut self, var1: Option<i64>);

    // Java: abstract Nullable<String> getCreatorProperty()
    fn getCreatorProperty(&self) -> Option<String>;

    // Java: abstract void setCreatorProperty(String var1)
    fn setCreatorProperty(&mut self, var1: &str);

    // Java: abstract Nullable<String> getDescriptionProperty()
    fn getDescriptionProperty(&self) -> Option<String>;

    // Java: abstract void setDescriptionProperty(String var1)
    fn setDescriptionProperty(&mut self, var1: &str);

    // Java: abstract Nullable<String> getIdentifierProperty()
    fn getIdentifierProperty(&self) -> Option<String>;

    // Java: abstract void setIdentifierProperty(String var1)
    fn setIdentifierProperty(&mut self, var1: &str);

    // Java: abstract Nullable<String> getKeywordsProperty()
    fn getKeywordsProperty(&self) -> Option<String>;

    // Java: abstract void setKeywordsProperty(String var1)
    fn setKeywordsProperty(&mut self, var1: &str);

    // Java: abstract Nullable<String> getLanguageProperty()
    fn getLanguageProperty(&self) -> Option<String>;

    // Java: abstract void setLanguageProperty(String var1)
    fn setLanguageProperty(&mut self, var1: &str);

    // Java: abstract Nullable<String> getLastModifiedByProperty()
    fn getLastModifiedByProperty(&self) -> Option<String>;

    // Java: abstract void setLastModifiedByProperty(String var1)
    fn setLastModifiedByProperty(&mut self, var1: &str);

    // Java: abstract Nullable<Date> getLastPrintedProperty()
    fn getLastPrintedProperty(&self) -> Option<i64>;

    // Java: abstract void setLastPrintedProperty(String var1)
    fn setLastPrintedProperty_str(&mut self, var1: &str);

    // Java: abstract void setLastPrintedProperty(Nullable<Date> var1)
    fn setLastPrintedProperty(&mut self, var1: Option<i64>);

    // Java: abstract Nullable<Date> getModifiedProperty()
    fn getModifiedProperty(&self) -> Option<i64>;

    // Java: abstract void setModifiedProperty(String var1)
    fn setModifiedProperty_str(&mut self, var1: &str);

    // Java: abstract void setModifiedProperty(Nullable<Date> var1)
    fn setModifiedProperty(&mut self, var1: Option<i64>);

    // Java: abstract Nullable<String> getRevisionProperty()
    fn getRevisionProperty(&self) -> Option<String>;

    // Java: abstract void setRevisionProperty(String var1)
    fn setRevisionProperty(&mut self, var1: &str);

    // Java: abstract Nullable<String> getSubjectProperty()
    fn getSubjectProperty(&self) -> Option<String>;

    // Java: abstract void setSubjectProperty(String var1)
    fn setSubjectProperty(&mut self, var1: &str);

    // Java: abstract Nullable<String> getTitleProperty()
    fn getTitleProperty(&self) -> Option<String>;

    // Java: abstract void setTitleProperty(String var1)
    fn setTitleProperty(&mut self, var1: &str);

    // Java: abstract Nullable<String> getVersionProperty()
    fn getVersionProperty(&self) -> Option<String>;

    // Java: abstract void setVersionProperty(String var1)
    fn setVersionProperty(&mut self, var1: &str);

}
