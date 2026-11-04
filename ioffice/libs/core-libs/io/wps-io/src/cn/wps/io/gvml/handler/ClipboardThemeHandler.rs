#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.io.gvml.IGVmlImporter
// Java: import cn.wps.io.simple.ElementHandlerSimple
// Java: import cn.wps.io.xwpf.IDmlCommonImporter
// Java: import cn.wps.io.xwpf.element_handler.XmlSimpleNodeElementHandler
// Java: import cn.wps.io.xwpf.element_handler.dml.ClrMapHandler
// Java: import cn.wps.io.xwpf.element_handler.dml.ThemeElementsHandler

// Java: public class ClipboardThemeHandler extends XmlSimpleNodeElementHandler
pub struct ClipboardThemeHandler {
    // Java field: ThemeElementsHandler mThemeElementsHandler
    mThemeElementsHandler: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: ClrMapHandler mClrMapHandler
    mClrMapHandler: Option<Box<dyn std::any::Any + Send + Sync>>,
}

// Java: public class ClipboardThemeHandler extends XmlSimpleNodeElementHandler
impl ClipboardThemeHandler {
    // Java: ClipboardThemeHandler(IGVmlImporter importer)
    pub fn new(_importer: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super((IDmlCommonImporter)importer);
        ClipboardThemeHandler {
            mThemeElementsHandler: None,
            mClrMapHandler: None,
        }
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&mut self, element_id: i32, _namespace_uri: &str) -> Option<&Box<dyn std::any::Any + Send + Sync>> {
        // Java: switch (elementId)
        match element_id {
            // Java: case -1357467789:
            -1357467789_i32 => {
                // Java: return this.getClrMapHandler();
                self.getClrMapHandler()
            }
            // Java: case 2003091936:
            2003091936 => {
                // Java: return this.getThemeElementsHandler();
                self.getThemeElementsHandler()
            }
            // Java: default:
            _ => {
                // Java: return null;
                None
            }
        }
    }

    // Java: ElementHandlerSimple getThemeElementsHandler()
    fn getThemeElementsHandler(&mut self) -> Option<&Box<dyn std::any::Any + Send + Sync>> {
        // Java: if (null == this.mThemeElementsHandler)
        if self.mThemeElementsHandler.is_none() {
            // Java: this.mThemeElementsHandler = new ThemeElementsHandler((IGVmlImporter)this.mDmlImporter);
            todo!()
        }
        // Java: return this.mThemeElementsHandler;
        self.mThemeElementsHandler.as_ref()
    }

    // Java: ElementHandlerSimple getClrMapHandler()
    fn getClrMapHandler(&mut self) -> Option<&Box<dyn std::any::Any + Send + Sync>> {
        // Java: if (null == this.mClrMapHandler)
        if self.mClrMapHandler.is_none() {
            // Java: this.mClrMapHandler = new ClrMapHandler((IGVmlImporter)this.mDmlImporter);
            todo!()
        }
        // Java: return this.mClrMapHandler;
        self.mClrMapHandler.as_ref()
    }
}
