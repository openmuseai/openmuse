use std::{collections::HashMap,sync::{Arc,Mutex,LazyLock}};
use super::{ResourceLoader::ResourceLoader,ResourceDef::ResourceDef,IResourceParser::IResourceParser};
use crate::cn::wps::kfc::numfmt::global::{GlobalEnv::GlobalEnv,LocaleResID::LocaleResID};
// java private static final HashMap<Short,LocaleResCache> mapLocaleRes=new HashMap();
static MAP:LazyLock<Mutex<HashMap<i16,Arc<LocaleResCache>>>>=LazyLock::new(||Mutex::new(HashMap::new()));
pub struct LocaleResAccessor;
// java public static class ResourceItem
#[derive(Clone)]
pub struct ResourceItem {pub IsArray:bool,pub NAME:String,pub mContent:Option<Vec<String>>}
impl ResourceItem {pub fn isEmpty(&self)->bool {self.mContent.as_ref().is_none_or(|a|a.is_empty())}}
// java public static class LocaleResCache
pub struct LocaleResCache {pub mSuccessed:bool,mResName:String,mItems:Vec<ResourceItem>}
impl LocaleResAccessor {
    pub fn clear(){MAP.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clear();}
    // java get requested cache, then GlobalEnv.matchLCID, then 1033 when missing or unsuccessful.
    pub fn getLocaleResource(localeCode:i16)->Arc<LocaleResCache>{
        let mut map=MAP.lock().unwrap_or_else(std::sync::PoisonError::into_inner);let mut resInfo=map.get(&localeCode).cloned();
        if resInfo.is_none()&&localeCode>0 {let cache=Arc::new(LocaleResCache::new(localeCode));map.insert(localeCode,cache.clone());resInfo=Some(cache);}
        if resInfo.as_ref().is_none_or(|x|!x.mSuccessed){let supported=GlobalEnv::matchLCID(localeCode);resInfo=Some(map.entry(supported).or_insert_with(||Arc::new(LocaleResCache::new(supported))).clone());}
        if resInfo.as_ref().is_none_or(|x|!x.mSuccessed){resInfo=Some(map.entry(1033).or_insert_with(||Arc::new(LocaleResCache::new(1033))).clone());}
        resInfo.unwrap()
    }
}
impl LocaleResCache {
    // java this.mResName=ResourceDef.getLocaleResName(localeCode);this.reload();
    pub fn new(localeCode:i16)->Self{let mut cache=Self{mSuccessed:false,mResName:ResourceDef::getLocaleResName(localeCode),mItems:Vec::new()};cache.reload();cache}
    pub fn getItem(&self,id:i32)->&ResourceItem{&self.mItems[id as usize]}
    pub fn getString(&self,id:i32)->String{self.getItem(id).mContent.as_ref().expect("resource item")[0].clone()}
    pub fn getArray(&self,id:i32)->Vec<String>{self.getItem(id).mContent.clone().unwrap_or_default()}
    pub fn isLoaded(&self)->bool{self.mSuccessed}
    fn reload(&mut self){
        // java IResourceParser parser=ResourceLoader.getSaxParser(this.mResName);if(parser!=null){this.init();for(ResourceItem item:this.mItems){...}parser.destroy();this.mSuccessed=true;}
        if let Some(mut parser)=ResourceLoader::getSaxParser(&self.mResName){self.mItems=(0..LocaleResID::getIDCount()).map(|id|{let name=LocaleResID::getName(id).unwrap();let isArray=LocaleResID::isArray(id);let content=if isArray{parser.getArray(name)}else{vec![parser.getItem(name)]};ResourceItem{IsArray:isArray,NAME:name.into(),mContent:Some(content)}}).collect();parser.destroy();self.mSuccessed=true;}
    }
}
