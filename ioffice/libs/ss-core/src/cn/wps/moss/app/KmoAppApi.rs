use std::sync::Arc;
use super::KmoApp::{KmoApp,APP};
// java public class KmoAppApi extends KmoApp
pub struct KmoAppApi;
impl KmoAppApi {
    // java public void startup(){super.startup();this.books=new KmoBooksApi();}
    pub fn startup(app:&KmoApp){app.startup();app.setBooks(Some(Arc::new(super::KmoBooksApi::KmoBooksApi::new())));}
    // java if(app==null)app=new KmoAppApi();return app;
    pub fn sharedApp()->Arc<KmoApp>{APP.get_or_init(||Arc::new(KmoApp::new())).clone()}
    // java KmoAppApi newApp=new KmoAppApi();if(app==null)app=newApp;return newApp;
    pub fn sharedApp2()->Arc<KmoApp>{let newApp=Arc::new(KmoApp::new());let _=APP.set(newApp.clone());newApp}
}
