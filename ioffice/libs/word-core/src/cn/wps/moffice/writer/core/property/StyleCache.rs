#![allow(non_snake_case)]

use crate::cn::wps::moffice::writer::core::Document::SoftTextDocument;
use crate::cn::wps::moffice::writer::data::DefaultParagraphProperties::DefaultParagraphProperties;
use crate::cn::wps::moffice::writer::data::DefaultRunProperties::DefaultRunProperties;
use util::moffice::property::PropertySet::PropertySet;

// java class StyleCache
pub struct StyleCache<'a> {
    // java private WeakReference<TextDocument> mDocument;
    // Rust borrows the document for this cache's lifetime. The future factory
    // must still implement Java's weak-key invalidation before caching it.
    mDocument: &'a dyn SoftTextDocument,
    // java private PropertySet mDefaultPap;
    mDefaultPap: PropertySet,
    // java private PropertySet mDefaultChp;
    mDefaultChp: PropertySet,
}

impl<'a> StyleCache<'a> {
    // java public StyleCache(TextDocument document)
    pub fn StyleCache(document: &'a dyn SoftTextDocument) -> Self {
        let mut cache = Self {
            mDocument: document,
            mDefaultPap: PropertySet::empty(),
            mDefaultChp: PropertySet::empty(),
        };
        cache.reset(document);
        cache
    }

    // java public void reset(TextDocument document)
    pub fn reset(&mut self, document: &'a dyn SoftTextDocument) {
        // java this.mDocument = new WeakReference(document);
        self.mDocument = document;
        // java KStyles styles = document.getStyles();
        let styles = document.getStyles().expect("StyleCache: document styles");
        // java this.mDefaultPap = PropertySet.compose(DefaultParagraphProperties.getProperties(), styles.getPPrDefault(), 2);
        self.mDefaultPap = PropertySet::compose(
            &DefaultParagraphProperties::getProperties(), &styles.getPPrDefault(), 2,
        );
        // java this.mDefaultChp = PropertySet.compose(DefaultRunProperties.getProperties(), styles.getRPrDefault(), 2);
        self.mDefaultChp = PropertySet::compose(
            &DefaultRunProperties::getProperties(), &styles.getRPrDefault(), 2,
        );
    }

    // java private TextDocument getDocument()
    fn getDocument(&self) -> &dyn SoftTextDocument {
        self.mDocument
    }

    // java @Deprecated public PropertySet getStyleFormat(int sti)
    pub fn getStyleFormat(&self, sti: i32) -> PropertySet {
        // java KStyle style = this.getDocument().getStyles().getStyleById(sti);
        let mut style = self.getDocument().getStyles()
            .expect("StyleCache: document styles")
            .getStyleById__int(sti).expect("StyleCache: style");
        // java KSAssert.assertNotNull(style);
        // java return style.getFinalProperty();
        style.getFinalProperty().expect("StyleCache: final property")
    }

    // java public PropertySet getRunStyleFormat(int sti)
    pub fn getRunStyleFormat(&self, sti: i32) -> PropertySet {
        // java KStyle style = this.getDocument().getStyles().getStyleById(sti);
        let mut style = self.getDocument().getStyles()
            .expect("StyleCache: document styles")
            .getStyleById__int(sti).expect("StyleCache: style");
        // java return style.getType() == 2 ? style.getFinalProperty() : PropertySet.empty;
        if style.getType() == 2 {
            style.getFinalProperty().expect("StyleCache: final run property")
        } else {
            PropertySet::empty()
        }
    }

    // java public PropertySet getParagraphStyleFormat(int sti)
    pub fn getParagraphStyleFormat(&self, sti: i32) -> PropertySet {
        // java KStyle style = this.getDocument().getStyles().getStyleById(sti);
        let style = self.getDocument().getStyles()
            .expect("StyleCache: document styles")
            .getStyleById__int(sti);
        // java return null != style && style.getType() == 1 ? style.getFinalProperty() : PropertySet.empty;
        if let Some(mut style) = style.filter(|style| style.getType() == 1) {
            style.getFinalProperty().expect("StyleCache: final paragraph property")
        } else {
            PropertySet::empty()
        }
    }

    // java public PropertySet getDefaultPap()
    pub fn getDefaultPap(&self) -> PropertySet { self.mDefaultPap.clone() }

    // java public PropertySet getDefaultChp()
    pub fn getDefaultChp(&self) -> PropertySet { self.mDefaultChp.clone() }
}
