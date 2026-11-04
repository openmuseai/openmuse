use std::cell::RefCell;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

use crate::cn::wps::drawing::dmlr::handler::color::ColorHandlerFactory::ColorHandlerFactory;
use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::BodyPropertiesHandler::BodyPropertiesHandler;
use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::text_model_surface::TextBodyProperties;
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// rust: ★2B-③-2 —— 由影子 `layout_shape::FillObj` 改为**真身** `data::fill::Fill`
//   （文本 run 的 `solidFill` 颜色写回真模型）。
use crate::cn::wps::drawing::data::fill::Fill::Fill;
use crate::cn::wps::show::util::layout_textbox::{
    KmoTextOperator, TextCharOptions, TextCharacterProperties, TextPara, TextParagraphProperties,
    TextParaOptions, TextRun,
};

#[derive(Clone, Debug, Default)]
pub struct TextRunSurf {
    pub text: String,
    pub sz_hundredths: Option<i32>,
    pub bold: bool,
    pub font_color: Option<i32>,
    pub scheme: Option<i32>,
}

#[derive(Clone, Debug, Default)]
pub struct TextParagraphSurf {
    pub runs: Vec<TextRunSurf>,
}

#[derive(Clone, Debug, Default)]
pub struct TextBodySurf {
    pub paras: Vec<TextParagraphSurf>,
    pub vertical_text: Option<i32>,
}

pub struct RunSolidFillHandler {
    fill: Rc<RefCell<Fill>>,
    run: Rc<RefCell<TextRunSurf>>,
}

impl ElementHandler for RunSolidFillHandler {
    fn kind(&self) -> &'static str {
        "SolidFillHandler"
    }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        ColorHandlerFactory::createColorHandler(
            subElement,
            // rust: ★2B-③-2 —— 真 `Fill` 无扁平 `color` 字段；经 `solidFill()` 取真 `Color`
            //   的模式作为采集初值。
            // rust: ★2B-③-2 —— 真 `Fill` 的 `Color` 经 `solidFill_mut()` 取得。
            Rc::new(RefCell::new({
                let mut f = self.fill.borrow_mut();
                let c = f.solidFill_mut();
                crate::cn::wps::show::util::layout_shape::ColorSlot {
                    mode: Some(c.mode()),
                    srgb: None,
                    scheme: None,
                }
            })),
            Some(self.fill.clone()),
            None,
            1114371,
        )
    }
    fn endElement(&mut self, _element: i32) {
        // rust: ★2B-③-2 —— 真 `Fill` 经 `solidFill()` 取真 `Color`，
        //   `srgb` 为 `RGB` 子对象（键 1/2/3 = r/g/b），`scheme` 为枚举值。
        let mut f = self.fill.borrow_mut();
        let c = f.solidFill_mut();
        if c.hasSrgb() {
            let rgb = c.srgb();
            let packed = (0xFF000000u32
                | ((rgb.r() as u32) << 16)
                | ((rgb.g() as u32) << 8)
                | (rgb.b() as u32)) as i32;
            self.run.borrow_mut().font_color = Some(packed);
        }
        if c.hasScheme() {
            self.run.borrow_mut().scheme = Some(c.scheme());
        }
    }
}

pub struct RegularTextHandler {
    run: Rc<RefCell<TextRunSurf>>,
}

impl ElementHandler for RegularTextHandler {
    fn kind(&self) -> &'static str {
        "RegularTextHandler"
    }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
    fn characters(&mut self, chars: &str) {
        self.run.borrow_mut().text.push_str(chars);
    }
}

pub struct RunPropHandler {
    run: Rc<RefCell<TextRunSurf>>,
}

impl ElementHandler for RunPropHandler {
    fn kind(&self) -> &'static str {
        "TextCharacterPropertiesHandler"
    }
    fn startElement(&mut self, _element: i32, attrs: &XmlRoAttrList) {
        if let Some(sz) = attrs.get_by_name(1114589) {
            self.run.borrow_mut().sz_hundredths = Some(sz.to_int());
        }
        if let Some(b) = attrs.get_by_name(1114195) {
            self.run.borrow_mut().bold = b.to_boolean();
        }
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: TextCharacterPropertiesHandler.enterSubElement — solidFill 1114371
        match subElement {
            1114371 => {
                let fill = Rc::new(RefCell::new(Fill::create()));
                Some(Box::new(RunSolidFillHandler {
                    fill,
                    run: self.run.clone(),
                }))
            }
            _ => None,
        }
    }
    fn endElement(&mut self, _element: i32) {}
}

pub struct TextRunsHandler {
    para: Rc<RefCell<TextParagraphSurf>>,
    run: Rc<RefCell<TextRunSurf>>,
}

impl ElementHandler for TextRunsHandler {
    fn kind(&self) -> &'static str {
        "TextRunsHandler"
    }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        match subElement {
            1114214 => Some(Box::new(RegularTextHandler {
                run: self.run.clone(),
            })),
            1114603 => Some(Box::new(RunPropHandler {
                run: self.run.clone(),
            })),
            _ => None,
        }
    }
    fn endElement(&mut self, _element: i32) {
        self.para.borrow_mut().runs.push(self.run.borrow().clone());
    }
}

pub struct ParagraphHandler {
    pub para: Rc<RefCell<TextParagraphSurf>>,
}

impl ParagraphHandler {
    pub fn ParagraphHandler(para: Rc<RefCell<TextParagraphSurf>>) -> Self {
        Self { para }
    }
}

impl ElementHandler for ParagraphHandler {
    fn kind(&self) -> &'static str {
        "ParagraphHandler"
    }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        match subElement {
            1114193 | 1114618 | 1114619 => {
                let run = Rc::new(RefCell::new(TextRunSurf::default()));
                Some(Box::new(TextRunsHandler {
                    para: self.para.clone(),
                    run,
                }))
            }
            _ => None,
        }
    }
    fn endElement(&mut self, _element: i32) {}
}

pub struct TextBodyHandler {
    body: Rc<RefCell<TextBodySurf>>,
}

impl TextBodyHandler {
    pub fn TextBodyHandler(body: Rc<RefCell<TextBodySurf>>) -> Self {
        Self { body }
    }
    pub fn to_operator(body: &TextBodySurf) -> KmoTextOperator {
        let paras = body
            .paras
            .iter()
            .map(|p| {
                let mut start = 0i32;
                let runs = p
                    .runs
                    .iter()
                    .filter(|r| !r.text.is_empty())
                    .map(|r| {
                        let n = r.text.chars().count() as i32;
                        let run = TextRun {
                            text: r.text.clone(),
                            start,
                            end: start + n,
                            ch_prop: TextCharacterProperties {
                                font_name: "Arial".into(),
                                font_size_pt: r.sz_hundredths.map(|s| s as f32 / 100.0).unwrap_or(18.0),
                                font_color: r.font_color.unwrap_or(0xFF000000u32 as i32),
                                scheme_slot: r.scheme,
                                options: TextCharOptions {
                                    has_bold: true,
                                    bold: r.bold,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                        };
                        start += n;
                        run
                    })
                    .collect();
                TextPara {
                    para_prop: TextParagraphProperties {
                        options: TextParaOptions {
                            has_bu_none: true,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    runs,
                }
            })
            .collect();
        let mut op = KmoTextOperator {
            paras,
            has_paragraph_list: true,
            ph_type: 1,
            ..Default::default()
        };
        if let Some(v) = body.vertical_text {
            op.body_pr.has_vertical_text = true;
            op.body_pr.vertical_text = v;
        }
        op
    }
}

impl ElementHandler for TextBodyHandler {
    fn kind(&self) -> &'static str {
        "TextBodyHandler"
    }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        match subElement {
            1114562 => {
                let para = Rc::new(RefCell::new(TextParagraphSurf::default()));
                self.body.borrow_mut().paras.push(TextParagraphSurf::default());
                Some(Box::new(ParaBridge {
                    body: self.body.clone(),
                    inner: ParagraphHandler::ParagraphHandler(para),
                }))
            }
            1114448 => {
                let props = Rc::new(RefCell::new(TextBodyProperties::default()));
                Some(Box::new(BodyPrBridge {
                    body: self.body.clone(),
                    props: props.clone(),
                    inner: BodyPropertiesHandler::BodyPropertiesHandler(props, None),
                }))
            }
            1114449 => None,
            _ => None,
        }
    }
    fn endElement(&mut self, _element: i32) {}
}

struct BodyPrBridge {
    body: Rc<RefCell<TextBodySurf>>,
    props: Rc<RefCell<TextBodyProperties>>,
    inner: BodyPropertiesHandler,
}

impl ElementHandler for BodyPrBridge {
    fn kind(&self) -> &'static str {
        "BodyPropertiesHandler"
    }
    fn startElement(&mut self, e: i32, a: &XmlRoAttrList) {
        self.inner.startElement(e, a);
    }
    fn enterSubElement(&mut self, s: i32) -> Option<Box<dyn ElementHandler>> {
        self.inner.enterSubElement(s)
    }
    fn endElement(&mut self, e: i32) {
        self.inner.endElement(e);
        self.body.borrow_mut().vertical_text = self.props.borrow().verticalText;
    }
}

struct ParaBridge {
    body: Rc<RefCell<TextBodySurf>>,
    inner: ParagraphHandler,
}

impl ElementHandler for ParaBridge {
    fn kind(&self) -> &'static str {
        "ParagraphHandler"
    }
    fn startElement(&mut self, e: i32, a: &XmlRoAttrList) {
        self.inner.startElement(e, a);
    }
    fn enterSubElement(&mut self, s: i32) -> Option<Box<dyn ElementHandler>> {
        self.inner.enterSubElement(s)
    }
    fn endElement(&mut self, e: i32) {
        self.inner.endElement(e);
        if let Some(last) = self.body.borrow_mut().paras.last_mut() {
            *last = self.inner.para.borrow().clone();
        }
    }
    fn characters(&mut self, c: &str) {
        self.inner.characters(c);
    }
}
