// Java: public class POIXMLRelation
#[derive(Clone, Debug)]
pub struct POIXMLRelation {
    // Java field: String _type
    pub(crate) _type: Option<String>,
    // Java field: String _relation
    pub(crate) _relation: Option<String>,
    // Java field: String _defaultName
    pub(crate) _defaultName: Option<String>,
    // Java field: Class<? extends POIXMLDocumentPart> _cls
    pub(crate) _cls: Option<String>,
}

// Java: public class POIXMLRelation
impl POIXMLRelation {
    // Java: POIXMLRelation(String type, String rel, String defaultName, Class<? extends POIXMLDocumentPart> cls)
    pub fn new_with_cls(r#type: String, rel: String, default_name: String, cls: Option<String>) -> Self {
        // Java: super();
        POIXMLRelation {
            // Java: this._type = type;
            _type: Some(r#type),
            // Java: this._relation = rel;
            _relation: Some(rel),
            // Java: this._defaultName = defaultName;
            _defaultName: Some(default_name),
            // Java: this._cls = cls;
            _cls: cls,
        }
    }

    // Java: POIXMLRelation()
    pub fn new() -> Self {
        // Java: super();
        POIXMLRelation {
            _type: None,
            _relation: None,
            _defaultName: None,
            _cls: None,
        }
    }

    // Java: POIXMLRelation(String type, String rel, String defaultName)
    pub fn new_with_type(r#type: String, rel: String, default_name: String) -> Self {
        // Java: this(type, rel, defaultName, (Class)null);
        Self::new_with_cls(r#type, rel, default_name, None)
    }

    // Java: String getContentType()
    pub fn getContentType(&self) -> Option<&str> {
        // Java: return this._type;
        self._type.as_deref()
    }

    // Java: String getRelation()
    pub fn getRelation(&self) -> Option<&str> {
        // Java: return this._relation;
        self._relation.as_deref()
    }

    // Java: String getDefaultFileName()
    pub fn getDefaultFileName(&self) -> Option<&str> {
        // Java: return this._defaultName;
        self._defaultName.as_deref()
    }

    // Java: String getFileName(int index)
    pub fn getFileName(&self, index: i32) -> Option<String> {
        // Java: return this._defaultName.indexOf("#") == -1 ? this.getDefaultFileName() : this._defaultName.replace("#", Integer.toString(index));
        self._defaultName.as_ref().map(|name| {
            if !name.contains('#') {
                name.clone()
            } else {
                name.replace('#', &index.to_string())
            }
        })
    }

    // Java: public Class<? extends POIXMLDocumentPart> getRelationClass()
    pub fn getRelationClass(&self) -> Option<&str> {
        // Java: return this._cls;
        self._cls.as_deref()
    }
}
