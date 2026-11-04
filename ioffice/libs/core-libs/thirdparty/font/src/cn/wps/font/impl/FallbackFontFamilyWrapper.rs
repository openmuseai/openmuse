// Java: import cn.wps.font.IFontFamily
use crate::cn::wps::font::IFontFamily::IFontFamily;
// Java: import cn.wps.font.ITypeface
use crate::cn::wps::font::ITypeface::ITypeface;
// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import java.util.Iterator
use std::sync::Arc;
use std::sync::Mutex;

// Java: public class FallbackFontFamilyWrapper implements IFontFamily
pub struct FallbackFontFamilyWrapper {
    // Java field: IFontFamily mOriginFontFamily
    pub(crate) mOriginFontFamily: Arc<dyn IFontFamily>,
    // Java field: HashMap<Integer, ITypeface> mDerivedTypefaces
    mDerivedTypefaces: Mutex<HashMap<i32, Arc<dyn ITypeface>>>,
}

// Java: public class FallbackFontFamilyWrapper implements IFontFamily
impl FallbackFontFamilyWrapper {
    // Java: FallbackFontFamilyWrapper(IFontFamily orgFontFamily)
    pub fn new(org_font_family: Arc<dyn IFontFamily>) -> Self {
        // Java: super();
        // Java: this.mOriginFontFamily = orgFontFamily;
        // Java: this.mDerivedTypefaces = new HashMap();
        FallbackFontFamilyWrapper {
            mOriginFontFamily: org_font_family,
            mDerivedTypefaces: Mutex::new(HashMap::new()),
        }
    }

    // Java: ITypeface getTypeface(int style)
    pub fn getTypeface_impl(&self, style: i32) -> Option<Arc<dyn ITypeface>> {
        // Java: ITypeface derivedTypeface = (ITypeface)this.mDerivedTypefaces.get(style);
        let map = self.mDerivedTypefaces.lock().unwrap();
        let existing = map.get(&style).cloned();
        drop(map);
        // Java: if (null != derivedTypeface)
        if let Some(derived) = existing {
            // Java: return derivedTypeface;
            return Some(derived);
        }
        // Java: else
        {
            // Java: ITypeface orgTypeface = this.mOriginFontFamily.getTypeface(style);
            let org_typeface = self.mOriginFontFamily.getTypeface(style);
            // Java: derivedTypeface = orgTypeface.deriveFallbackTypeface(this);
            // Java: this.mDerivedTypefaces.put(style, derivedTypeface);
            // Java: return derivedTypeface;
            if let Some(org) = org_typeface {
                let derived = org.deriveFallbackTypeface(Arc::new(FallbackFontFamilyWrapper::new(self.mOriginFontFamily.clone())));
                if let Some(ref d) = derived {
                    let mut map = self.mDerivedTypefaces.lock().unwrap();
                    map.insert(style, d.clone());
                }
                derived
            } else {
                None
            }
        }
    }

    // Java: Iterator<ITypeface> iterator()
    pub fn iterator(&self) -> Vec<Arc<dyn ITypeface>> {
        // Java: final Iterator<ITypeface> it = this.mOriginFontFamily.iterator();
        // Java: return new Iterator<ITypeface>()
        {
            // Java: private ITypeface mLastIterateTypeface;
            // Java: public boolean hasNext()
            {
                // Java: return it.hasNext();
            }
            // Java: public ITypeface next()
            {
                // Java: ITypeface typeface = (ITypeface)it.next();
                // Java: int style = typeface.getStyle();
                // Java: return FallbackFontFamilyWrapper.this.getTypeface(style);
            }
            // Java: public void remove()
            {
                // Java: if (null != this.mLastIterateTypeface)
                {
                    // Java: int style = this.mLastIterateTypeface.getStyle();
                    // Java: FallbackFontFamilyWrapper.this.mDerivedTypefaces.remove(style);
                }
            }
        }
        Vec::new()
    }
}

impl IFontFamily for FallbackFontFamilyWrapper {
    // Java: String getName()
    fn getName(&self) -> String {
        // Java: return this.mOriginFontFamily.getName();
        self.mOriginFontFamily.getName()
    }

    // Java: int getId()
    fn getId(&self) -> i32 {
        // Java: return this.mOriginFontFamily.getId();
        self.mOriginFontFamily.getId()
    }

    fn getTypeface(&self, style: i32) -> Option<Arc<dyn ITypeface>> {
        self.getTypeface_impl(style)
    }

    // Java: boolean isSystemFont()
    fn isSystemFont(&self) -> bool {
        // Java: return this.mOriginFontFamily.isSystemFont();
        self.mOriginFontFamily.isSystemFont()
    }

    // Java: String[] getFontFiles(int style)
    fn getFontFiles(&self, style: i32) -> Vec<String> {
        // Java: return this.mOriginFontFamily.getFontFiles(style);
        self.mOriginFontFamily.getFontFiles(style)
    }

    // Java: String getFontFile(int style)
    fn getFontFile(&self, style: i32) -> String {
        // Java: return this.mOriginFontFamily.getFontFile(style);
        self.mOriginFontFamily.getFontFile(style)
    }

    // Java: String[] getAllFontFiles()
    fn getAllFontFiles(&self) -> Vec<String> {
        // Java: return this.mOriginFontFamily.getAllFontFiles();
        self.mOriginFontFamily.getAllFontFiles()
    }

    // Java: boolean isOnlineFont()
    fn isOnlineFont(&self) -> bool {
        // Java: return this.mOriginFontFamily.isOnlineFont();
        self.mOriginFontFamily.isOnlineFont()
    }

    // Java: boolean fontFileExist()
    fn fontFileExist(&self) -> bool {
        // Java: return this.mOriginFontFamily.fontFileExist();
        self.mOriginFontFamily.fontFileExist()
    }

    // Java: void setOnlineFont(boolean isOnlineFont)
    fn setOnlineFont(&self, _is_online_font: bool) {
    }

    // Java: long getNativeFreeTypeFontPointer(int style)
    fn getNativeFreeTypeFontPointer(&self, _style: i32) -> i64 {
        // Java: return 0L;
        0
    }

    // Java: ITypeface addTypefaceFromFile(int style, String file)
    fn addTypefaceFromFile(&self, _style: i32, _file: String) -> Option<Arc<dyn ITypeface>> {
        // Java: return null;
        None
    }

    // Java: void addFontFile(int style, String fileName)
    fn addFontFile(&self, _style: i32, _file_name: String) {
    }

    // Java: boolean isBuildinHeader()
    fn isBuildinHeader(&self) -> bool {
        // Java: return false;
        false
    }

    // Java: IFontFamily getSourceFont()
    fn getSourceFont(&self) -> Option<Arc<dyn IFontFamily>> {
        // Java: return null;
        None
    }

    // Java: void closeNativePtr()
    fn closeNativePtr(&self) {
    }
}
