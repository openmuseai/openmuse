use crate::java_runtime::{Rc,Weak,RefCell,Date};
use crate::cn::wps::moss::app::KmoBook::KmoBook;
use kfc_kfc_alpha::kfc::opcw::StringMap::StringMap;
use io_poi_compact::org::apache::poi::hpsf::Property::{Property,PropertyValue};
use util::cn::wps::moffice::util::UUIDUtil::UUIDUtil;
// java public final class KmoAppSettings {
pub struct KmoAppSettings {
    // java private Map<String, Property> mCustomProps = new HashMap();
    mCustomProps:Rc<RefCell<StringMap<Option<Rc<Property>>>>>,
    // java private KmoBook book;
    book:Weak<KmoBook>,
    // java private boolean isIcvUpdatedFromNew;
    isIcvUpdatedFromNew:bool,
    // java private String mAppName = null;
    mAppName:Option<String>,
    // java private String mAppVersion = null;
    mAppVersion:Option<String>,
    // java private String mManager = null;
    mManager:Option<String>,
    // java private String mCompany = null;
    mCompany:Option<String>,
    // java private String mHyperlinkBase = null;
    mHyperlinkBase:Option<String>,
    // java private byte[] mHyperlinkBaseXls = null;
    mHyperlinkBaseXls:Option<Rc<RefCell<Vec<u8>>>>,
    // java private boolean bScale = false;
    bScale:bool,
    // java private boolean bLinkDirty = false;
    bLinkDirty:bool,
    // java private String mTitle = null;
    mTitle:Option<String>,
    // java private String mSubject = null;
    mSubject:Option<String>,
    // java private String mCreator = null;
    mCreator:Option<String>,
    // java private String mKeywords = null;
    mKeywords:Option<String>,
    // java private String mDescription = null;
    mDescription:Option<String>,
    // java private String mLastModifiedBy = null;
    mLastModifiedBy:Option<String>,
    // java private Date mCreated = null;
    mCreated:Option<Rc<Date>>,
    // java private Date mModified = null;
    mModified:Option<Rc<Date>>,
    // java private String mCategory = null;
    mCategory:Option<String>,
    // java private String mContentStatus = null;
    mContentStatus:Option<String>,
    // java private String mContentType = null;
    mContentType:Option<String>,
    // java private int mSecurity = -1;
    mSecurity:i32,
}
impl KmoAppSettings {
    // java public static final String APPLICATION_NAME = "Kingsoft Office";
    pub const APPLICATION_NAME:&str="Kingsoft Office";
    // java public static final String READINGLAYOUT = "KSOReadingLayout";
    pub const READINGLAYOUT:&str="KSOReadingLayout";
    // java private static final String ICV = "ICV";
    const ICV:&str="ICV";
    // java private static final String TEMPLATE_ICV = "KSOTemplateUUID";
    const TEMPLATE_ICV:&str="KSOTemplateUUID";
    // java public static final String ICV_NEW_SUFFIX = "21";
    pub const ICV_NEW_SUFFIX:&str="21";
    // java public static final String ICV_SAVE_SUFFIX = "22";
    pub const ICV_SAVE_SUFFIX:&str="22";
    // java public static final String ICV_SAVE_AS_SUFFIX = "23";
    pub const ICV_SAVE_AS_SUFFIX:&str="23";
    // java public static final String LINK_BASE = "_PID_LINKBASE";
    pub const LINK_BASE:&str="_PID_LINKBASE";
    // java public KmoAppSettings(KmoBook book) {
    pub fn KmoAppSettings(book:Rc<KmoBook>)->Self {
        // java this.book = book;
        Self{mCustomProps:Rc::new(RefCell::new(StringMap::default())),book:Rc::downgrade(&book),isIcvUpdatedFromNew:false,mAppName:None,mAppVersion:None,mManager:None,mCompany:None,mHyperlinkBase:None,mHyperlinkBaseXls:None,bScale:false,bLinkDirty:false,mTitle:None,mSubject:None,mCreator:None,mKeywords:None,mDescription:None,mLastModifiedBy:None,mCreated:None,mModified:None,mCategory:None,mContentStatus:None,mContentType:None,mSecurity:-1}
    }
    // java public Map<String, Property> getCustomProps() { return this.mCustomProps; }
    pub fn getCustomProps(&self)->Rc<RefCell<StringMap<Option<Rc<Property>>>>>{self.mCustomProps.clone()}
    // java public void setReadingLayout(boolean b) {
    pub fn setReadingLayout(&mut self,b:bool){
        // java this.mCustomProps.put("KSOReadingLayout", new Property(-1L, 11L, b));
        self.mCustomProps.borrow_mut().put(Self::READINGLAYOUT.into(),Some(Rc::new(Property::new(-1,11,PropertyValue::Bool(b)))));
        // java this.book.setSilentSaveWithoutDirty(true);
        self.book.upgrade().expect("KmoBook").setSilentSaveWithoutDirty(true);
    }
    // java public boolean getReadingLayout() {
    pub fn getReadingLayout(&self)->bool{
        // java Property prop = (Property)this.mCustomProps.get("KSOReadingLayout");
        let props=self.mCustomProps.borrow();let prop=props.get(Self::READINGLAYOUT).and_then(Option::as_ref);
        // java return prop == null ? false : (Boolean)prop.getValue();
        match prop {None=>false,Some(p)=>match p.getValue(){PropertyValue::Bool(b)=>*b,PropertyValue::None=>panic!("NullPointerException"),_=>panic!("ClassCastException Boolean")}}
    }
    // java public void createIcvFromNew() {
    pub fn createIcvFromNew(&mut self){
        // java this.updateIcv("21"); this.isIcvUpdatedFromNew = true;
        self.updateIcv__suffix(Some(Self::ICV_NEW_SUFFIX));self.isIcvUpdatedFromNew=true;
    }
    // java public void updateIcv(boolean isSavedAs) {
    pub fn updateIcv(&mut self,isSavedAs:bool){
        // java if (isSavedAs || !this.hasIcv()) { this.updateIcv(isSavedAs ? "23" : "22"); }
        if isSavedAs||!self.hasIcv(){self.updateIcv__suffix(Some(if isSavedAs{Self::ICV_SAVE_AS_SUFFIX}else{Self::ICV_SAVE_SUFFIX}));}
    }
    // java public void updateIcv(String suffix) {
    pub fn updateIcv__suffix(&mut self,suffix:Option<&str>){
        // java if (this.isIcvUpdatedFromNew) { this.isIcvUpdatedFromNew = false; } else { this.addIcv(suffix); }
        if self.isIcvUpdatedFromNew{self.isIcvUpdatedFromNew=false;}else{self.addIcv__suffix(suffix);}
    }
    // java public boolean hasIcv() { Property prop = this.getIcv(); return null != prop; }
    pub fn hasIcv(&self)->bool{let prop=self.getIcv();prop.is_some()}
    // java public Property getIcv() { return (Property)this.mCustomProps.get("ICV"); }
    pub fn getIcv(&self)->Option<Rc<Property>>{self.mCustomProps.borrow().get(Self::ICV).cloned().flatten()}
    // java public void addIcv() { this.addIcv((String)null); }
    pub fn addIcv(&mut self){self.addIcv__suffix(None);}
    // java public void addIcv(String suffix) {
    pub fn addIcv__suffix(&mut self,suffix:Option<&str>){
        // java String icv = UUIDUtil.generate32ID();
        let mut icv=UUIDUtil::generate32ID();
        // java if (null != suffix && suffix.length() != 0) { icv = icv + "_" + suffix; }
        if let Some(suffix)=suffix.filter(|s|!s.is_empty()){icv.push('_');icv.push_str(suffix);}
        // java this.mCustomProps.put("ICV", new Property(-1L, 31L, icv));
        self.mCustomProps.borrow_mut().put(Self::ICV.into(),Some(Rc::new(Property::new(-1,31,PropertyValue::Str(icv)))));
    }
    // java public Property getTemplateIcv() { return (Property)this.mCustomProps.get("KSOTemplateUUID"); }
    pub fn getTemplateIcv(&self)->Option<Rc<Property>>{self.mCustomProps.borrow().get(Self::TEMPLATE_ICV).cloned().flatten()}
    // java public void setAppName(String appName) { this.mAppName = appName; }
    pub fn setAppName(&mut self,appName:Option<String>){self.mAppName=appName;}
    // java public String getAppName() { return this.mAppName; }
    pub fn getAppName(&self)->Option<String>{self.mAppName.clone()}
    // java public void setAppVersion(String appVersion) { this.mAppVersion = appVersion; }
    pub fn setAppVersion(&mut self,appVersion:Option<String>){self.mAppVersion=appVersion;}
    // java public String getAppVersion() { return this.mAppVersion; }
    pub fn getAppVersion(&self)->Option<String>{self.mAppVersion.clone()}
    // java public void setHyperlinkBase(String hyperlink) { this.mHyperlinkBase = hyperlink; }
    pub fn setHyperlinkBase(&mut self,hyperlink:Option<String>){self.mHyperlinkBase=hyperlink;}
    // java public String getHyperlinkBase() { return this.mHyperlinkBase; }
    pub fn getHyperlinkBase(&self)->Option<String>{self.mHyperlinkBase.clone()}
    // java public void setHyperlinkBaseXls(byte[] bytes) { this.mHyperlinkBaseXls = bytes; }
    pub fn setHyperlinkBaseXls(&mut self,bytes:Option<Rc<RefCell<Vec<u8>>>>){self.mHyperlinkBaseXls=bytes;}
    // java public byte[] getHyperlinkBaseXls() { return this.mHyperlinkBaseXls; }
    pub fn getHyperlinkBaseXls(&self)->Option<Rc<RefCell<Vec<u8>>>>{self.mHyperlinkBaseXls.clone()}
    // java public void setManager(String manager) { this.mManager = manager; }
    pub fn setManager(&mut self,manager:Option<String>){self.mManager=manager;}
    // java public String getManager() { return this.mManager; }
    pub fn getManager(&self)->Option<String>{self.mManager.clone()}
    // java public void setCompany(String company) { this.mCompany = company; }
    pub fn setCompany(&mut self,company:Option<String>){self.mCompany=company;}
    // java public String getCompany() { return this.mCompany; }
    pub fn getCompany(&self)->Option<String>{self.mCompany.clone()}
    // java public void setScale(boolean b) { this.bScale = b; }
    pub fn setScale(&mut self,b:bool){self.bScale=b;}
    // java public boolean getScale() { return this.bScale; }
    pub fn getScale(&self)->bool{self.bScale}
    // java public void setLinkDirty(boolean b) { this.bLinkDirty = b; }
    pub fn setLinkDirty(&mut self,b:bool){self.bLinkDirty=b;}
    // java public boolean getLinkDirty() { return this.bLinkDirty; }
    pub fn getLinkDirty(&self)->bool{self.bLinkDirty}
    // java public void setCreator(String creator) { this.mCreator = creator; }
    pub fn setCreator(&mut self,creator:Option<String>){self.mCreator=creator;}
    // java public String getCreator() { return this.mCreator; }
    pub fn getCreator(&self)->Option<String>{self.mCreator.clone()}
    // java public void setCreated(Date created) { this.mCreated = created; }
    pub fn setCreated(&mut self,created:Option<Rc<Date>>){self.mCreated=created;}
    // java public Date getCreated() { return this.mCreated; }
    pub fn getCreated(&self)->Option<Rc<Date>>{self.mCreated.clone()}
    // java public void setLastModifiedBy(String modify) { this.mLastModifiedBy = modify; }
    pub fn setLastModifiedBy(&mut self,modify:Option<String>){self.mLastModifiedBy=modify;}
    // java public String getLastModifiedBy() { return this.mLastModifiedBy; }
    pub fn getLastModifiedBy(&self)->Option<String>{self.mLastModifiedBy.clone()}
    // java public void setModified(Date modified) { this.mModified = modified; }
    pub fn setModified(&mut self,modified:Option<Rc<Date>>){self.mModified=modified;}
    // java public Date getModified() { return this.mModified; }
    pub fn getModified(&self)->Option<Rc<Date>>{self.mModified.clone()}
    // java public void setTitle(String title) { this.mTitle = title; }
    pub fn setTitle(&mut self,title:Option<String>){self.mTitle=title;}
    // java public String getTitle() { return this.mTitle; }
    pub fn getTitle(&self)->Option<String>{self.mTitle.clone()}
    // java public void setKeywords(String keywords) { this.mKeywords = keywords; }
    pub fn setKeywords(&mut self,keywords:Option<String>){self.mKeywords=keywords;}
    // java public String getKeywords() { return this.mKeywords; }
    pub fn getKeywords(&self)->Option<String>{self.mKeywords.clone()}
    // java public void setCategory(String category) { this.mCategory = category; }
    pub fn setCategory(&mut self,category:Option<String>){self.mCategory=category;}
    // java public String getCategory() { return this.mCategory; }
    pub fn getCategory(&self)->Option<String>{self.mCategory.clone()}
    // java public void setDescription(String description) { this.mDescription = description; }
    pub fn setDescription(&mut self,description:Option<String>){self.mDescription=description;}
    // java public String getDescription() { return this.mDescription; }
    pub fn getDescription(&self)->Option<String>{self.mDescription.clone()}
    // java public void setContentStatus(String status) { this.mContentStatus = status; }
    pub fn setContentStatus(&mut self,status:Option<String>){self.mContentStatus=status;}
    // java public String getContentStatus() { return this.mContentStatus; }
    pub fn getContentStatus(&self)->Option<String>{self.mContentStatus.clone()}
    // java public void setContentType(String type) { this.mContentType = type; }
    pub fn setContentType(&mut self,r#type:Option<String>){self.mContentType=r#type;}
    // java public String getContentType() { return this.mContentType; }
    pub fn getContentType(&self)->Option<String>{self.mContentType.clone()}
    // java public void setSubject(String subject) { this.mSubject = subject; }
    pub fn setSubject(&mut self,subject:Option<String>){self.mSubject=subject;}
    // java public String getSubject() { return this.mSubject; }
    pub fn getSubject(&self)->Option<String>{self.mSubject.clone()}
    // java public void setSecurity(int security) { this.mSecurity = security; }
    pub fn setSecurity(&mut self,security:i32){self.mSecurity=security;}
    // java public int getSecurity() { return this.mSecurity; }
    pub fn getSecurity(&self)->i32{self.mSecurity}
    // java public void updateSettings() {
    pub fn updateSettings(&mut self){
        // java this.mAppName = "Kingsoft Office";
        self.mAppName=Some(Self::APPLICATION_NAME.into());
        // java if (this.mCreated == null) { this.mCreated = new Date(); }
        if self.mCreated.is_none(){self.mCreated=Some(Rc::new(Date::new()));}
        // java this.mModified = new Date();
        self.mModified=Some(Rc::new(Date::new()));
    }
}
