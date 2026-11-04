use std::sync::{Arc,Mutex};
use super::GlobalEnv::GlobalEnv;
use crate::cn::wps::kfc::numfmt::i18n::{KmoLang::KmoLang,KmoLocaleChanged::KmoLocaleChanged};
// java public class NumFmtLocaleHandler implements KmoLocaleChanged
pub struct NumFmtLocaleHandler;
static NOTIFIERS:Mutex<Vec<Arc<dyn KmoLocaleChanged+Send+Sync>>>=Mutex::new(Vec::new());
impl NumFmtLocaleHandler {
    pub fn new()->Self{Self}
    // java if(!mLocaleNotifers.contains(listener))mLocaleNotifers.add(listener);
    pub fn addKmoLocaleChanged(listener:Arc<dyn KmoLocaleChanged+Send+Sync>){let mut list=NOTIFIERS.lock().unwrap();if !list.iter().any(|x|Arc::ptr_eq(x,&listener)){list.push(listener);}}
    // java mLocaleNotifers.remove(listener);
    pub fn removeKmoLocaleChanged(listener:&Arc<dyn KmoLocaleChanged+Send+Sync>){let mut list=NOTIFIERS.lock().unwrap();if let Some(i)=list.iter().position(|x|Arc::ptr_eq(x,listener)){list.remove(i);}}
    pub fn localeChanged_impl(&self){
        // java short lcid=KmoLang.getSystemLCID();if(GlobalEnv.getDefaultLCID()!=0)GlobalEnv.setLCID(lcid);
        let lcid=KmoLang::getSystemLCID();if GlobalEnv::getDefaultLCID()!=0{GlobalEnv::setLCID(lcid);}
        // java for(int i=0,len=mLocaleNotifers.size();i<len;++i)mLocaleNotifers.get(i).localeChanged();
        let listeners=NOTIFIERS.lock().unwrap().clone();for listener in listeners{listener.localeChanged();}
        // java System.out.println("numfmt locale changed code :"+Integer.toHexString(lcid));
        eprintln!("numfmt locale changed code :{lcid:x}");
    }
}
impl KmoLocaleChanged for NumFmtLocaleHandler {fn localeChanged(&self){self.localeChanged_impl();}}
