use super::super::NativeTypeface::NativeTypeface;
use super::TypefaceFactory::TypefaceFactory;
use crate::java_runtime::{Rc, RefCell};
// java class FontFamilyImpl implements IFontFamily
pub struct FontFamilyImpl {
    pub mFamilyName: String,
    pub mIsSystemFont: bool,
    pub mId: i32,
    pub mTypefaces: RefCell<Vec<Option<NativeTypeface>>>,
    pub mFontFiles: RefCell<[Option<String>; 4]>,
}
impl FontFamilyImpl {
    // java public FontFamilyImpl(String familyName, boolean isSystemFont, int id) {
    // java super();
    // java this.mFamilyName = familyName;
    // java this.mIsSystemFont = isSystemFont;
    // java this.mId = id;
    // java }
    pub fn new(name: String, system: bool, id: i32) -> Self {
        Self {
            mFamilyName: name,
            mIsSystemFont: system,
            mId: id,
            mTypefaces: RefCell::new(vec![]),
            mFontFiles: RefCell::new([None, None, None, None]),
        }
    }
    // java public void addTypeface(int style, ITypeface face) {
    // java if (face == null) {
    // java throw new IllegalArgumentException("Invalid typeface");
    // java } else {
    // java for(int i = this.mTypefaces.size(); i < style + 1; ++i) {
    // java this.mTypefaces.add(null);
    // java }
    // java
    // java this.mTypefaces.set(style, face);
    // java }
    // java }
    pub fn addTypeface(&self, style: i32, face: NativeTypeface) {
        let mut faces = self.mTypefaces.borrow_mut();
        while faces.len() < (style + 1) as usize {
            faces.push(None);
        }
        faces[style as usize] = Some(face);
    }
    // java public ITypeface getTypeface(int style) {
    // java if (style < 0) {
    // java return null;
    // java } else {
    // java ITypeface face = null;
    // java face = this.createTypeface(style);
    // java if (face == null) {
    // java face = FontHost.getFontTable().getDefault().getTypeface(style);
    // java }
    // java
    // java return face;
    // java }
    // java }
    pub fn getTypeface(&self, style: i32) -> Option<NativeTypeface> {
        if style < 0 {
            None
        } else {
            self.createTypeface(style).or_else(|| {
                let table = super::super::FontHost::FontHost::getFontTable().borrow();
                let default = table.getDefault();
                assert_ne!(default.mId, self.mId, "No default native font face");
                default.getTypeface(style)
            })
        }
    }
    // java public ITypeface createTypeface(int style) {
    // java if (style < 0) {
    // java return null;
    // java } else {
    // java ITypeface face = null;
    // java if (style < this.mTypefaces.size()) {
    // java face = (ITypeface)this.mTypefaces.get(style);
    // java }
    // java
    // java if (face == null) {
    // java if (this.mIsBuildinHeader) {
    // java if (null != this.mSourceFontFamily) {
    // java ITypeface typeFace = this.mSourceFontFamily.getTypeface(style);
    // java face = TypefaceFactory.createFromSourceFace(this, typeFace, style);
    // java ((AbstractTypeface)face).setNeedBold(typeFace.needBold());
    // java ((AbstractTypeface)face).setNeedItalic(typeFace.needItalic());
    // java } else {
    // java face = TypefaceFactory.createTypeface(this, 0, (String)null);
    // java ((AbstractTypeface)face).setNeedBold((style & 1) == 1);
    // java ((AbstractTypeface)face).setNeedItalic((style & 2) == 2);
    // java }
    // java } else {
    // java String fileName = null;
    // java FontFiles paths = this.mFontFiles;
    // java if (null != (fileName = paths.getFontFilePath(style))) {
    // java face = TypefaceFactory.createTypeface(this, style, fileName);
    // java } else if (null != (fileName = paths.getFontFilePath(0))) {
    // java face = TypefaceFactory.createTypeface(this, style, fileName);
    // java ((AbstractTypeface)face).setNeedBold((style & 1) == 1);
    // java ((AbstractTypeface)face).setNeedItalic((style & 2) == 2);
    // java } else if (null != (fileName = paths.getFontFilePath(1))) {
    // java face = TypefaceFactory.createTypeface(this, style, fileName);
    // java ((AbstractTypeface)face).setNeedItalic((style & 2) == 2);
    // java } else if (null != (fileName = paths.getFontFilePath(2))) {
    // java face = TypefaceFactory.createTypeface(this, style, fileName);
    // java ((AbstractTypeface)face).setNeedBold((style & 1) == 1);
    // java } else if (null != (fileName = paths.getFontFilePath(3))) {
    // java face = TypefaceFactory.createTypeface(this, style, fileName);
    // java }
    // java }
    // java
    // java if (face != null) {
    // java this.addTypeface(style, face);
    // java }
    // java }
    // java
    // java return face;
    // java }
    // java }
    pub fn createTypeface(&self, style: i32) -> Option<NativeTypeface> {
        if style < 0 {
            return None;
        }
        if let Some(Some(face)) = self.mTypefaces.borrow().get(style as usize) {
            return Some(face.clone());
        }
        let files = self.mFontFiles.borrow();
        let file = files
            .get(style as usize)
            .and_then(|v| v.as_deref())
            .or(files[0].as_deref())
            .or(files[1].as_deref())
            .or(files[2].as_deref())
            .or(files[3].as_deref());
        let face = file.map(|file| TypefaceFactory::createTypeface(style, Some(file)));
        if let Some(face) = &face {
            self.addTypeface(style, face.clone());
        }
        face
    }
}
