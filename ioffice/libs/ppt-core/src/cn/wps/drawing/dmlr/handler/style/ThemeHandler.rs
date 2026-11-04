use std::cell::RefCell;
use std::rc::Rc;

// Java: import cn.wps.drawing.app.KmoProtocol
// rust: compile-surface receiver is ThemeObj (KmoProtocol.updateProtocol)
// Java: import cn.wps.drawing.data.color.CustomColor
// Java: import cn.wps.drawing.data.style.Theme
// Java: import cn.wps.drawing.dmlr.handler.color.ColorHandlerFactory
use crate::cn::wps::drawing::dmlr::handler::color::ColorHandlerFactory::ColorHandlerFactory;
// Java: import cn.wps.drawing.dmlr.handler.shape.sppr.ShapePropertiesHandler
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::ShapePropertiesHandler::{
    NamedHandler, ShapePropertiesHandler,
};
// Java: import cn.wps.drawing.dmlr.handler.shape.style.ShapeStyleHandler
// rust: parked this function — NamedHandler kind ShapeStyleHandler
// Java: import cn.wps.drawing.dmlr.handler.shape.textshape.text.BodyPropertiesHandler
// Java: import cn.wps.drawing.dmlr.handler.shape.textshape.text.TextListStyleHandler
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::fill::FillNotify::FillNotifyRef;
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
// rust: kfc DefaultElementHandler::new is todo!(); holder is ()
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import cn.wps.drawing.dmlr.handler.style.ColorMapHandler
use crate::cn::wps::drawing::dmlr::handler::style::ColorMapHandler::ColorMapHandler;
// Java: import cn.wps.drawing.dmlr.handler.style.ThemeElementsHandler
// rust: parked — NamedHandler kind ThemeElementsHandler
// Java: import cn.wps.drawing.dmlr.handler.style.ColorSchemeHandler
// rust: parked — NamedHandler kind ColorSchemeHandler
use crate::cn::wps::show::util::layout_shape::{
    CustomColor, DefaultShapeDefinition, ExtraColorScheme, ThemeObj,
};

pub type KmoProtocol = Rc<RefCell<ThemeObj>>;
pub type HandlerCallback = FillNotifyRef;

// java public class ThemeHandler extends DefaultElementHandler
pub struct ThemeHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: (),
    // java private KmoProtocol themeReceiver;
    themeReceiver: KmoProtocol,
    // java private HandlerCallback handlerCallback;
    handlerCallback: HandlerCallback,
    // java private Theme themeData;
    themeData: Rc<RefCell<ThemeObj>>,
}

// java  class ObjectDefaultsHandler extends DefaultElementHandler
pub struct ObjectDefaultsHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: (),
    themeData: Rc<RefCell<ThemeObj>>,
    handlerCallback: HandlerCallback,
}

// java  class DefaultShapeDefinitionHandler extends DefaultElementHandler
pub struct DefaultShapeDefinitionHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: (),
    // java private Theme.DefaultShapeDefinition defaultObj;
    defaultObj: Rc<RefCell<DefaultShapeDefinition>>,
    handlerCallback: HandlerCallback,
}

// java  class CustomColorListHandler extends DefaultElementHandler
pub struct CustomColorListHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: (),
    // java private List<CustomColor> customColors;
    customColors: Option<Vec<Rc<RefCell<CustomColor>>>>,
    // java private CustomColorHandler customColorHandler;
    customColorHandler: Option<CustomColorHandler>,
    themeData: Rc<RefCell<ThemeObj>>,
    handlerCallback: HandlerCallback,
}

// java  class CustomColorHandler extends DefaultElementHandler
#[derive(Clone)]
pub struct CustomColorHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: (),
    // java private CustomColor customColor;
    customColor: Rc<RefCell<CustomColor>>,
    handlerCallback: HandlerCallback,
}

// java  class ColorSchemeListHandler extends DefaultElementHandler
pub struct ColorSchemeListHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: (),
    // java private List<Theme.ExtraColorScheme> extraColorSchemes;
    extraColorSchemes: Option<Vec<Rc<RefCell<ExtraColorScheme>>>>,
    // java private ColorSchemeAndMappingHandler colorSchemeAndMappingHandler;
    colorSchemeAndMappingHandler: Option<ColorSchemeAndMappingHandler>,
    themeData: Rc<RefCell<ThemeObj>>,
    handlerCallback: HandlerCallback,
}

// java  class ColorSchemeAndMappingHandler extends DefaultElementHandler
#[derive(Clone)]
pub struct ColorSchemeAndMappingHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: (),
    // java private Theme.ExtraColorScheme extraColorScheme;
    extraColorScheme: Rc<RefCell<ExtraColorScheme>>,
    handlerCallback: HandlerCallback,
}

// java public class ThemeHandler extends DefaultElementHandler
impl ThemeHandler {
    // java public  ThemeHandler(KmoProtocol themeReceiver, HandlerCallback handlerCallback)
    pub fn ThemeHandler(themeReceiver: KmoProtocol, handlerCallback: HandlerCallback) -> Self {
        // java this.themeReceiver = themeReceiver;
        // java this.handlerCallback = handlerCallback;
        // java this.themeData = Theme.create();
        Self {
            super_: (),
            themeReceiver,
            handlerCallback,
            themeData: Rc::new(RefCell::new(ThemeObj::create())),
        }
    }

    pub fn themeData(&self) -> Rc<RefCell<ThemeObj>> {
        self.themeData.clone()
    }
}

impl ElementHandler for ThemeHandler {
    fn kind(&self) -> &'static str {
        "ThemeHandler"
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // java XmlRoAttr tmp;
        let mut tmp;
        // java if ((tmp = attrsNotCopy.getByName(1114136)) != null)
        tmp = attrsNotCopy.get_by_name(1114136);
        if tmp.is_some() {
            // java this.themeData.putName(tmp.toString());
            self.themeData.borrow_mut().putName(tmp.unwrap().to_string());
        }
    }

    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // java switch (subElement)
        match subElement {
            // java case 1114459:
            1114459 => {
                // java return new ColorMapHandler(this.themeData.colorMapping());
                Some(Box::new(ColorMapHandler::ColorMapHandler(
                    self.themeData.borrow().colorMapping(),
                )))
            }
            // java case 1114461:
            1114461 => {
                // java return new ThemeElementsHandler(this.themeData, this.handlerCallback);
                Some(Box::new(NamedHandler {
                    kind: "ThemeElementsHandler",
                }))
            }
            // java case 1114462:
            1114462 => {
                // java return new ObjectDefaultsHandler();
                Some(Box::new(ObjectDefaultsHandler {
                    super_: (),
                    themeData: self.themeData.clone(),
                    handlerCallback: self.handlerCallback.clone(),
                }))
            }
            // java case 1114463:
            1114463 => {
                // java return new ColorSchemeListHandler();
                Some(Box::new(ColorSchemeListHandler {
                    super_: (),
                    extraColorSchemes: None,
                    colorSchemeAndMappingHandler: None,
                    themeData: self.themeData.clone(),
                    handlerCallback: self.handlerCallback.clone(),
                }))
            }
            // java case 1114464:
            1114464 => {
                // java return new CustomColorListHandler();
                Some(Box::new(CustomColorListHandler {
                    super_: (),
                    customColors: None,
                    customColorHandler: None,
                    themeData: self.themeData.clone(),
                    handlerCallback: self.handlerCallback.clone(),
                }))
            }
            // java case 1114460:
            // java default:
            1114460 | _ => {
                // java return null;
                None
            }
        }
    }

    // java public void endElement(int element)
    fn endElement(&mut self, _element: i32) {
        // java this.themeReceiver.updateProtocol(this.themeData.save());
        self.themeReceiver
            .borrow_mut()
            .updateProtocol(self.themeData.borrow().save());
    }
}

// java  class ObjectDefaultsHandler extends DefaultElementHandler
impl ElementHandler for ObjectDefaultsHandler {
    fn kind(&self) -> &'static str {
        "ObjectDefaultsHandler"
    }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // java switch (subElement)
        match subElement {
            // java case 1114450:
            1114450 => {
                // java return new DefaultShapeDefinitionHandler(ThemeHandler.this.themeData.shapeDef());
                Some(Box::new(DefaultShapeDefinitionHandler {
                    super_: (),
                    defaultObj: self.themeData.borrow().shapeDef(),
                    handlerCallback: self.handlerCallback.clone(),
                }))
            }
            // java case 1114451:
            1114451 => {
                // java return new DefaultShapeDefinitionHandler(ThemeHandler.this.themeData.lineDef());
                Some(Box::new(DefaultShapeDefinitionHandler {
                    super_: (),
                    defaultObj: self.themeData.borrow().lineDef(),
                    handlerCallback: self.handlerCallback.clone(),
                }))
            }
            // java case 1114452:
            1114452 => {
                // java return new DefaultShapeDefinitionHandler(ThemeHandler.this.themeData.textDef());
                Some(Box::new(DefaultShapeDefinitionHandler {
                    super_: (),
                    defaultObj: self.themeData.borrow().textDef(),
                    handlerCallback: self.handlerCallback.clone(),
                }))
            }
            // java default:
            _ => {
                // java return null;
                None
            }
        }
    }
    fn endElement(&mut self, _element: i32) {}
}

// java  class DefaultShapeDefinitionHandler extends DefaultElementHandler
impl DefaultShapeDefinitionHandler {
    // java public  DefaultShapeDefinitionHandler(Theme.DefaultShapeDefinition defaultObj)
    pub fn DefaultShapeDefinitionHandler(
        defaultObj: Rc<RefCell<DefaultShapeDefinition>>,
        handlerCallback: HandlerCallback,
    ) -> Self {
        // java this.defaultObj = defaultObj;
        Self {
            super_: (),
            defaultObj,
            handlerCallback,
        }
    }
}

impl ElementHandler for DefaultShapeDefinitionHandler {
    fn kind(&self) -> &'static str {
        "DefaultShapeDefinitionHandler"
    }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // java switch (subElement)
        match subElement {
            // java case 1114448:
            1114448 => {
                // java return new BodyPropertiesHandler(this.defaultObj.bodyPr(), ThemeHandler.this.handlerCallback);
                let _ = self.defaultObj.borrow().bodyPr();
                let _ = &self.handlerCallback;
                Some(Box::new(NamedHandler {
                    kind: "BodyPropertiesHandler",
                }))
            }
            // java case 1114449:
            1114449 => {
                // java return new TextListStyleHandler(this.defaultObj.textListStyle(), ThemeHandler.this.handlerCallback);
                let _ = self.defaultObj.borrow().textListStyle();
                Some(Box::new(NamedHandler {
                    kind: "TextListStyleHandler",
                }))
            }
            // java case 3211578:
            3211578 => {
                // java return new ShapePropertiesHandler(this.defaultObj.shapePr(), ThemeHandler.this.handlerCallback);
                Some(Box::new(ShapePropertiesHandler::ShapePropertiesHandler(
                    Rc::new(RefCell::new(
                        crate::cn::wps::drawing::data::shape::ShapePr::ShapePr::create(),
                    )),
                )))
            }
            // java case 3211579:
            3211579 => {
                // java return new ShapeStyleHandler(this.defaultObj.shapeStyle(), ThemeHandler.this.handlerCallback);
                let _ = self.defaultObj.borrow().shapeStyle();
                Some(Box::new(NamedHandler {
                    kind: "ShapeStyleHandler",
                }))
            }
            // java default:
            _ => {
                // java return null;
                None
            }
        }
    }
    fn endElement(&mut self, _element: i32) {}
}

// java  class CustomColorListHandler extends DefaultElementHandler
impl ElementHandler for CustomColorListHandler {
    fn kind(&self) -> &'static str {
        "CustomColorListHandler"
    }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // java ElementHandler handler = null;
        let mut handler: Option<Box<dyn ElementHandler>> = None;
        // java switch (subElement)
        match subElement {
            // java case 1114139:
            1114139 => {
                // java if (this.customColors == null)
                if self.customColors.is_none() {
                    // java this.customColors = new ArrayList();
                    self.customColors = Some(Vec::new());
                }
                // java CustomColor customColor = CustomColor.create();
                let customColor = CustomColor::create();
                // java this.customColors.add(customColor);
                self.customColors
                    .as_mut()
                    .unwrap()
                    .push(customColor.clone());
                // java if (this.customColorHandler == null)
                if self.customColorHandler.is_none() {
                    // java this.customColorHandler = new CustomColorHandler(customColor);
                    self.customColorHandler = Some(CustomColorHandler::CustomColorHandler(
                        customColor,
                        self.handlerCallback.clone(),
                    ));
                } else {
                    // java this.customColorHandler.init(customColor);
                    self.customColorHandler
                        .as_mut()
                        .unwrap()
                        .init(customColor);
                }
                // java handler = this.customColorHandler;
                handler = Some(Box::new(self.customColorHandler.as_ref().unwrap().clone()));
            }
            // java default:
            _ => {
                // rust: no-stmt
            }
        }
        // java return handler;
        handler
    }

    // java public void endElement(int element)
    fn endElement(&mut self, _element: i32) {
        // java if (this.customColors != null)
        if let Some(customColors) = self.customColors.as_ref() {
            // java for (CustomColor customColor : this.customColors)
            for customColor in customColors.iter() {
                // java customColor.save();
                customColor.borrow_mut().save();
            }
            // java ThemeHandler.this.themeData.customColorLst().write(this.customColors);
            self.themeData
                .borrow_mut()
                .customColorLst()
                .write(customColors.clone());
        }
    }
}

// java  class CustomColorHandler extends DefaultElementHandler
impl CustomColorHandler {
    // java public  CustomColorHandler(CustomColor customColor)
    pub fn CustomColorHandler(
        customColor: Rc<RefCell<CustomColor>>,
        handlerCallback: HandlerCallback,
    ) -> Self {
        let mut this = Self {
            super_: (),
            customColor: customColor.clone(),
            handlerCallback,
        };
        // java this.init(customColor);
        this.init(customColor);
        this
    }

    // java public void init(CustomColor customColor)
    pub fn init(&mut self, customColor: Rc<RefCell<CustomColor>>) {
        // java this.customColor = customColor;
        self.customColor = customColor;
    }
}

impl ElementHandler for CustomColorHandler {
    fn kind(&self) -> &'static str {
        "CustomColorHandler"
    }
    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // java XmlRoAttr tmp = null;
        let mut tmp = None;
        // java if ((tmp = attrsNotCopy.getByName(1114136)) != null)
        tmp = attrsNotCopy.get_by_name(1114136);
        if tmp.is_some() {
            // java this.customColor.putName(tmp.toString());
            self.customColor.borrow_mut().putName(tmp.unwrap().to_string());
        }
    }

    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // java return ColorHandlerFactory.createColorHandler(subElement, this.customColor.color(), ThemeHandler.this.handlerCallback, 1114139);
        ColorHandlerFactory::createColorHandler(
            subElement,
            self.customColor.borrow().color(),
            None,
            self.handlerCallback.clone(),
            1114139,
        )
    }
    fn endElement(&mut self, _element: i32) {}
}

// java  class ColorSchemeListHandler extends DefaultElementHandler
impl ElementHandler for ColorSchemeListHandler {
    fn kind(&self) -> &'static str {
        "ColorSchemeListHandler"
    }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // java ElementHandler handler = null;
        let mut handler: Option<Box<dyn ElementHandler>> = None;
        // java switch (subElement)
        match subElement {
            // java case 1114460:
            1114460 => {
                // java if (this.extraColorSchemes == null)
                if self.extraColorSchemes.is_none() {
                    // java this.extraColorSchemes = new ArrayList();
                    self.extraColorSchemes = Some(Vec::new());
                }
                // java Theme.ExtraColorScheme extraColorScheme = Theme.ExtraColorScheme.create();
                let extraColorScheme = ExtraColorScheme::create();
                // java this.extraColorSchemes.add(extraColorScheme);
                self.extraColorSchemes
                    .as_mut()
                    .unwrap()
                    .push(extraColorScheme.clone());
                // java if (this.colorSchemeAndMappingHandler == null)
                if self.colorSchemeAndMappingHandler.is_none() {
                    // java this.colorSchemeAndMappingHandler = new ColorSchemeAndMappingHandler(extraColorScheme);
                    self.colorSchemeAndMappingHandler =
                        Some(ColorSchemeAndMappingHandler::ColorSchemeAndMappingHandler(
                            extraColorScheme,
                            self.handlerCallback.clone(),
                        ));
                } else {
                    // java this.colorSchemeAndMappingHandler.init(extraColorScheme);
                    self.colorSchemeAndMappingHandler
                        .as_mut()
                        .unwrap()
                        .init(extraColorScheme);
                }
                // java handler = this.colorSchemeAndMappingHandler;
                handler = Some(Box::new(
                    self.colorSchemeAndMappingHandler.as_ref().unwrap().clone(),
                ));
            }
            // java default:
            _ => {
                // rust: no-stmt
            }
        }
        handler
    }

    // java public void endElement(int element)
    fn endElement(&mut self, _element: i32) {
        // java if (this.extraColorSchemes != null)
        if let Some(extraColorSchemes) = self.extraColorSchemes.as_ref() {
            // java for (Theme.ExtraColorScheme extraColorScheme : this.extraColorSchemes)
            for extraColorScheme in extraColorSchemes.iter() {
                // java extraColorScheme.save();
                extraColorScheme.borrow_mut().save();
            }
            // java ThemeHandler.this.themeData.extraColorSchemeList().write(this.extraColorSchemes);
            self.themeData
                .borrow_mut()
                .extraColorSchemeList()
                .write(extraColorSchemes.clone());
        }
    }
}

// java  class ColorSchemeAndMappingHandler extends DefaultElementHandler
impl ColorSchemeAndMappingHandler {
    // java public  ColorSchemeAndMappingHandler(Theme.ExtraColorScheme extraColorScheme)
    pub fn ColorSchemeAndMappingHandler(
        extraColorScheme: Rc<RefCell<ExtraColorScheme>>,
        handlerCallback: HandlerCallback,
    ) -> Self {
        // java this.extraColorScheme = extraColorScheme;
        Self {
            super_: (),
            extraColorScheme,
            handlerCallback,
        }
    }

    // java public void init(Theme.ExtraColorScheme extraColorScheme)
    pub fn init(&mut self, extraColorScheme: Rc<RefCell<ExtraColorScheme>>) {
        // java this.extraColorScheme = extraColorScheme;
        self.extraColorScheme = extraColorScheme;
    }
}

impl ElementHandler for ColorSchemeAndMappingHandler {
    fn kind(&self) -> &'static str {
        "ColorSchemeAndMappingHandler"
    }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // java switch (subElement)
        match subElement {
            // java case 1114154:
            1114154 => {
                // java return new ColorSchemeHandler(this.extraColorScheme.colorScheme(), ThemeHandler.this.handlerCallback);
                let _ = &self.handlerCallback;
                Some(Box::new(NamedHandler {
                    kind: "ColorSchemeHandler",
                }))
            }
            // java case 1114459:
            1114459 => {
                // java return new ColorMapHandler(this.extraColorScheme.colorMapping());
                Some(Box::new(ColorMapHandler::ColorMapHandler(
                    self.extraColorScheme.borrow().colorMapping(),
                )))
            }
            // java default:
            _ => {
                // java return null;
                None
            }
        }
    }
    fn endElement(&mut self, _element: i32) {}
}
