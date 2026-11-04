//! Compile-surface hosts for pl-04 text-box (default crate; app/drawing stay `full`).

use super::layout_shape::KmoContext;

#[derive(Clone, Debug, Default)]
pub struct IndexRange {
    pub start: i32,
    pub end: i32,
}

impl IndexRange {
    pub fn start(&self) -> i32 {
        self.start
    }
    pub fn end(&self) -> i32 {
        self.end
    }
}

#[derive(Clone, Debug, Default)]
pub struct Color {}

#[derive(Clone, Debug, Default)]
pub struct ColorInterpretCtx {}

impl ColorInterpretCtx {
    pub fn setSlide(&mut self, _slide: ()) {}
}

#[derive(Clone, Debug, Default)]
pub struct PatternFill {}

impl PatternFill {
    pub fn hasFgClr(&self) -> bool {
        false
    }
    pub fn hasBgClr(&self) -> bool {
        false
    }
    pub fn hasPrst(&self) -> bool {
        false
    }
    pub fn fgClr(&self) -> Color {
        Color {}
    }
    pub fn bgClr(&self) -> Color {
        Color {}
    }
    pub fn prst(&self) -> i32 {
        0
    }
}

#[derive(Clone, Debug, Default)]
pub struct Shade {}
impl Shade {
    pub fn lin(&self) -> LinAng { LinAng {} }
    pub fn hasPath(&self) -> bool { false }
    pub fn path(&self) -> PathShade { PathShade {} }
}
#[derive(Clone, Debug, Default)]
pub struct LinAng;
impl LinAng {
    pub fn ang(&self) -> i32 { 0 }
}
#[derive(Clone, Debug, Default)]
pub struct PathShade;
impl PathShade {
    pub fn path(&self) -> i32 { 0 }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GradType { LINEAR, SHAPE, CIRCLE, RECT }

#[derive(Clone, Debug, Default)]
pub struct RelativeRect {}
impl RelativeRect {
    pub fn left(&self) -> f32 { 0.0 }
    pub fn top(&self) -> f32 { 0.0 }
    pub fn right(&self) -> f32 { 0.0 }
    pub fn bottom(&self) -> f32 { 0.0 }
}

#[derive(Clone, Debug, Default)]
pub struct GradientFill {}
impl GradientFill {
    pub fn shade(&self) -> Shade { Shade {} }
    pub fn hasShade(&self) -> bool { false }
    pub fn gsLst(&self) -> GsLst { GsLst {} }
    pub fn hasTileRect(&self) -> bool { false }
    pub fn tileRect(&self) -> RelativeRect { RelativeRect {} }
}
pub struct GsLst;
impl GsLst {
    pub fn read(&self, _o: &mut Vec<GradientStop>) {}
}
pub struct GradientStop;
impl GradientStop {
    pub fn color(&self) -> Color { Color {} }
    pub fn pos(&self) -> f32 { 0.0 }
}

#[derive(Clone, Debug, Default)]
pub struct BlipFill {}

impl BlipFill {
    pub fn blip(&self) -> Blip { Blip {} }
    pub fn fillMode(&self) -> FillMode { FillMode {} }
}

#[derive(Clone, Debug, Default)]
pub struct Fill {
    pub type_: i32,
}

impl Fill {
    pub fn blipFill(&self) -> BlipFill { BlipFill::default() }
    pub fn r#type(&self) -> i32 {
        self.type_
    }
    pub fn solidFill(&self) -> Color {
        Color {}
    }
    pub fn pattFill(&self) -> PatternFill {
        PatternFill {}
    }
    pub fn gradFill(&self) -> GradientFill {
        GradientFill {}
    }
}

#[derive(Clone, Debug, Default)]
pub struct Outline {
    pub has_fill: bool,
    pub fill: Fill,
    pub has_width: bool,
    pub width: i32,
    pub has_preset_dash: bool,
    pub preset_dash: i32,
    pub has_compound_type: bool,
    pub compound_type: i32,
    pub has_line_join: bool,
    pub line_join_type: i32,
    pub has_line_cap: bool,
    pub line_cap: i32,
}

impl Outline {
    pub fn hasFill(&self) -> bool {
        self.has_fill
    }
    pub fn fill(&self) -> Fill {
        self.fill.clone()
    }
    pub fn hasWidth(&self) -> bool {
        self.has_width
    }
    pub fn width(&self) -> i32 {
        self.width
    }
    pub fn hasPresetDash(&self) -> bool {
        self.has_preset_dash
    }
    pub fn presetDash(&self) -> i32 {
        self.preset_dash
    }
    pub fn hasCompoundType(&self) -> bool {
        self.has_compound_type
    }
    pub fn compoundType(&self) -> i32 {
        self.compound_type
    }
    pub fn hasLineJoin(&self) -> bool {
        self.has_line_join
    }
    pub fn lineJoin(&self) -> LineJoin {
        LineJoin {
            type_: self.line_join_type,
        }
    }
    pub fn hasLineCap(&self) -> bool {
        self.has_line_cap
    }
    pub fn lineCap(&self) -> i32 {
        self.line_cap
    }
}

#[derive(Clone, Debug, Default)]
pub struct LineJoin {
    pub type_: i32,
}

impl LineJoin {
    pub fn r#type(&self) -> i32 {
        self.type_
    }
}

#[derive(Clone, Debug, Default)]
pub struct TextCharOptions {
    pub has_bold: bool,
    pub bold: bool,
    pub has_italics: bool,
    pub italics: bool,
    pub has_normalize_h: bool,
    pub normalize_h: bool,
}

impl TextCharOptions {
    pub fn hasBold(&self) -> bool {
        self.has_bold
    }
    pub fn isBold(&self) -> bool {
        self.bold
    }
    pub fn hasItalics(&self) -> bool {
        self.has_italics
    }
    pub fn isItalics(&self) -> bool {
        self.italics
    }
    pub fn hasNormalizeH(&self) -> bool {
        self.has_normalize_h
    }
    pub fn isNormalizeH(&self) -> bool {
        self.normalize_h
    }
}

#[derive(Clone, Debug, Default)]
pub struct TextCharacterProperties {
    pub font_name: String,
    pub font_size_pt: f32,
    pub font_color: i32,
    pub lang: String,
    pub strike: i32,
    pub underline: i32,
    pub options: TextCharOptions,
    pub has_fill: bool,
    pub has_outline: bool,
    pub has_spacing: bool,
    pub has_kern: bool,
    pub has_cap: bool,
    pub cap: i32,
    pub has_baseline: bool,
    pub baseline: f64,
    pub has_symbol_font: bool,
    pub has_east_asian_font: bool,
    pub has_latin_font: bool,
    pub has_complex_script_font: bool,
    pub parent: Option<Box<TextCharacterProperties>>,
    pub scheme_slot: Option<i32>,
}

impl TextCharacterProperties {
    pub fn lang(&self) -> &str {
        &self.lang
    }
    pub fn hasFill(&self) -> bool {
        self.has_fill
    }
    pub fn fill(&self) -> Fill {
        Fill { type_: 5 }
    }
    pub fn hasOutline(&self) -> bool {
        self.has_outline
    }
    pub fn outline(&self) -> Outline {
        Outline::default()
    }
    pub fn hasSpacing(&self) -> bool {
        self.has_spacing
    }
    pub fn spacing(&self) -> i32 {
        0
    }
    pub fn hasKern(&self) -> bool {
        self.has_kern
    }
    pub fn kern(&self) -> i32 {
        0
    }
    pub fn strike(&self) -> i32 {
        self.strike
    }
    pub fn underline(&self) -> i32 {
        self.underline
    }
    pub fn options(&self) -> &TextCharOptions {
        &self.options
    }
    pub fn hasCap(&self) -> bool {
        self.has_cap
    }
    pub fn cap(&self) -> i32 {
        self.cap
    }
    pub fn hasBaseline(&self) -> bool {
        self.has_baseline
    }
    pub fn baseline(&self) -> f64 {
        self.baseline
    }
    pub fn hasSymbolFont(&self) -> bool {
        self.has_symbol_font
    }
    pub fn hasEastAsianFont(&self) -> bool {
        self.has_east_asian_font
    }
    pub fn hasLatinFont(&self) -> bool {
        self.has_latin_font
    }
    pub fn hasComplexScriptFont(&self) -> bool {
        self.has_complex_script_font
    }
    pub fn hasEffect(&self) -> bool { false }
    pub fn effect(&self) -> EffectList { EffectList {} }
    pub fn hasUnderlineFill(&self) -> bool { false }
    pub fn underlineFill(&self) -> Fill { Fill { type_: 5 } }
    pub fn parent(&self) -> Option<TextCharacterProperties> {
        self.parent.as_ref().map(|p| (**p).clone())
    }
    pub fn hasParent(&self) -> bool {
        self.parent.is_some()
    }
}

#[derive(Clone, Debug, Default)]
pub struct TextParaOptions {
    pub has_ea_ln_brk: bool,
    pub ea_ln_brk: bool,
    pub has_latin_ln_brk: bool,
    pub latin_ln_brk: bool,
    pub has_hanging_punct: bool,
    pub hanging_punct: bool,
    pub has_right_to_left: bool,
    pub right_to_left: bool,
    pub has_bu_none: bool,
}

impl TextParaOptions {
    pub fn hasEaLnBrk(&self) -> bool {
        self.has_ea_ln_brk
    }
    pub fn isEaLnBrk(&self) -> bool {
        self.ea_ln_brk
    }
    pub fn hasLatinLnBrk(&self) -> bool {
        self.has_latin_ln_brk
    }
    pub fn isLatinLnBrk(&self) -> bool {
        self.latin_ln_brk
    }
    pub fn hasHangingPunct(&self) -> bool {
        self.has_hanging_punct
    }
    pub fn isHangingPunct(&self) -> bool {
        self.hanging_punct
    }
    pub fn hasRightToLeft(&self) -> bool {
        self.has_right_to_left
    }
    pub fn isRightToLeft(&self) -> bool {
        self.right_to_left
    }
    pub fn hasBuNone(&self) -> bool {
        self.has_bu_none
    }
}

#[derive(Clone, Debug, Default)]
pub struct LnSpc {
    pub has_spc_pct: bool,
    pub spc_pct: f64,
    pub has_spc_pts: bool,
    pub spc_pts: f64,
}

impl LnSpc {
    pub fn hasSpcPct(&self) -> bool {
        self.has_spc_pct
    }
    pub fn spcPct(&self) -> f64 {
        self.spc_pct
    }
    pub fn hasSpcPts(&self) -> bool {
        self.has_spc_pts
    }
    pub fn spcPts(&self) -> f64 {
        self.spc_pts
    }
}

#[derive(Clone, Debug, Default)]
pub struct TextSpacing {}

#[derive(Clone, Debug, Default)]
pub struct Autofit {
    pub type_: i32,
    pub has_font_scale: bool,
    pub font_scale: f64,
    pub has_ln_spc_reduction: bool,
    pub ln_spc_reduction: f64,
}

impl Autofit {
    pub fn r#type(&self) -> i32 {
        self.type_
    }
    pub fn hasFontScale(&self) -> bool {
        self.has_font_scale
    }
    pub fn fontScale(&self) -> f64 {
        self.font_scale
    }
    pub fn hasLnSpcReduction(&self) -> bool {
        self.has_ln_spc_reduction
    }
    pub fn lnSpcReduction(&self) -> f64 {
        self.ln_spc_reduction
    }
}

#[derive(Clone, Debug, Default)]
pub struct PresetTextWarp {
    pub prst: i32,
}

impl PresetTextWarp {
    pub fn prst(&self) -> i32 {
        self.prst
    }
}

#[derive(Clone, Debug, Default)]
pub struct TextBodyProperties {
    pub has_text_wrapping_type: bool,
    pub text_wrapping_type: i32,
    pub has_text_autofit: bool,
    pub text_autofit: Autofit,
    pub has_preset_text_warp: bool,
    pub preset_text_warp: PresetTextWarp,
    pub has_anchor: bool,
    pub anchor: i32,
    pub has_anchor_center: bool,
    pub anchor_center: bool,
    pub has_vertical_text: bool,
    pub vertical_text: i32,
    pub has_left_inset: bool,
    pub left_inset: i32,
    pub has_right_inset: bool,
    pub right_inset: i32,
    pub has_top_inset: bool,
    pub top_inset: i32,
    pub has_bottom_inset: bool,
    pub bottom_inset: i32,
}

impl TextBodyProperties {
    pub fn hasTextWrappingType(&self) -> bool {
        self.has_text_wrapping_type
    }
    pub fn textWrappingType(&self) -> i32 {
        self.text_wrapping_type
    }
    pub fn hasTextAutofit(&self) -> bool {
        self.has_text_autofit
    }
    pub fn textAutofit(&self) -> Autofit {
        self.text_autofit.clone()
    }
    pub fn hasPresetTextWarp(&self) -> bool {
        self.has_preset_text_warp
    }
    pub fn presetTextWarp(&self) -> PresetTextWarp {
        self.preset_text_warp.clone()
    }
    pub fn hasAnchor(&self) -> bool {
        self.has_anchor
    }
    pub fn anchor(&self) -> i32 {
        self.anchor
    }
    pub fn hasAnchorCenter(&self) -> bool {
        self.has_anchor_center
    }
    pub fn isAnchorCenter(&self) -> bool {
        self.anchor_center
    }
    pub fn hasVerticalText(&self) -> bool {
        self.has_vertical_text
    }
    pub fn verticalText(&self) -> i32 {
        self.vertical_text
    }
    pub fn hasLeftInset(&self) -> bool {
        self.has_left_inset
    }
    pub fn leftInset(&self) -> i32 {
        self.left_inset
    }
    pub fn hasRightInset(&self) -> bool {
        self.has_right_inset
    }
    pub fn rightInset(&self) -> i32 {
        self.right_inset
    }
    pub fn hasTopInset(&self) -> bool {
        self.has_top_inset
    }
    pub fn topInset(&self) -> i32 {
        self.top_inset
    }
    pub fn hasBottomInset(&self) -> bool {
        self.has_bottom_inset
    }
    pub fn bottomInset(&self) -> i32 {
        self.bottom_inset
    }
}

#[derive(Clone, Debug, Default)]
pub struct TextParagraphProperties {
    pub lvl: i32,
    pub alignment: i32,
    pub has_margin_left: bool,
    pub margin_left: i32,
    pub has_def_tab_size: bool,
    pub def_tab_size: i32,
    pub has_indent: bool,
    pub indent: i32,
    pub has_spc_bef: bool,
    pub has_spc_aft: bool,
    pub has_ln_spc: bool,
    pub ln_spc: LnSpc,
    pub options: TextParaOptions,
    pub has_bu_char: bool,
    pub has_bu_blip: bool,
    pub has_bu_auton_num: bool,
    pub parent: Option<Box<TextParagraphProperties>>,
}

impl TextParagraphProperties {
    pub fn lvl(&self) -> i32 {
        self.lvl
    }
    pub fn alignment(&self) -> i32 {
        self.alignment
    }
    pub fn hasMarginLeft(&self) -> bool {
        self.has_margin_left
    }
    pub fn marginLeft(&self) -> i32 {
        self.margin_left
    }
    pub fn hasDefTabSize(&self) -> bool {
        self.has_def_tab_size
    }
    pub fn defTabSize(&self) -> i32 {
        self.def_tab_size
    }
    pub fn tabStopLst(&self) -> Option<TabStopList> {
        None
    }
    pub fn hasIndent(&self) -> bool {
        self.has_indent
    }
    pub fn indent(&self) -> i32 {
        self.indent
    }
    pub fn hasSpcBef(&self) -> bool {
        self.has_spc_bef
    }
    pub fn spcBef(&self) -> TextSpacing {
        TextSpacing {}
    }
    pub fn hasSpcAft(&self) -> bool {
        self.has_spc_aft
    }
    pub fn spcAft(&self) -> TextSpacing {
        TextSpacing {}
    }
    pub fn options(&self) -> &TextParaOptions {
        &self.options
    }
    pub fn hasLnSpc(&self) -> bool {
        self.has_ln_spc
    }
    pub fn lnSpc(&self) -> &LnSpc {
        &self.ln_spc
    }
    pub fn hasBuChar(&self) -> bool {
        self.has_bu_char
    }
    pub fn hasBuBlip(&self) -> bool {
        self.has_bu_blip
    }
    pub fn hasBuAutonNum(&self) -> bool {
        self.has_bu_auton_num
    }
    pub fn parent(&self) -> Option<TextParagraphProperties> {
        self.parent.as_ref().map(|p| (**p).clone())
    }
    pub fn hasParent(&self) -> bool {
        self.parent.is_some()
    }
    pub fn hasOptions(&self) -> bool {
        true
    }
    pub fn save(&self) -> TextParagraphProperties {
        self.clone()
    }
    pub fn open(saved: TextParagraphProperties) -> TextParagraphProperties {
        saved
    }
    pub fn follow(&mut self, parent: Option<TextParagraphProperties>) {
        if let Some(p) = parent {
            self.parent = Some(Box::new(p));
        }
    }
    pub fn unFollow(&mut self) {
        self.parent = None;
    }
    pub fn buAutonNum(&self) -> BuAutonNum {
        BuAutonNum { type_: 0 }
    }
}

#[derive(Clone, Debug, Default)]
pub struct TabStopList {}

impl TabStopList {
    pub fn read(&self, _out: &mut Vec<TextTabStop>) {}
}

#[derive(Clone, Debug, Default)]
pub struct TextTabStop {
    pub pos: i32,
}

impl TextTabStop {
    pub fn pos(&self) -> i32 {
        self.pos
    }
}

#[derive(Clone, Debug, Default)]
pub struct BuAutonNum {
    pub type_: i32,
}

impl BuAutonNum {
    pub fn r#type(&self) -> i32 {
        self.type_
    }
}

#[derive(Clone, Debug, Default)]
pub struct TextRun {
    pub text: String,
    pub start: i32,
    pub end: i32,
    pub ch_prop: TextCharacterProperties,
}

#[derive(Clone, Debug, Default)]
pub struct TextPara {
    pub runs: Vec<TextRun>,
    pub para_prop: TextParagraphProperties,
}

#[derive(Clone, Debug, Default)]
pub struct KmoTextBody {
    pub protocol: i32,
}

impl KmoTextBody {
    pub fn protocol(&self) -> i32 {
        self.protocol
    }
    pub fn updateProtocol(&mut self, protocol: i32) {
        self.protocol = protocol;
    }
}

#[derive(Clone, Debug, Default)]
pub struct KmoTextOperator {
    pub paras: Vec<TextPara>,
    pub para_i: usize,
    pub run_i: usize,
    pub ph_type: i32,
    pub body_pr: TextBodyProperties,
    pub has_paragraph_list: bool,
    pub text_body: KmoTextBody,
    pub saved: bool,
}

impl KmoTextOperator {
    pub fn setContext(&mut self, _ctx: Option<&KmoContext>) {}
    pub fn phType(&self) -> i32 {
        self.ph_type
    }
    pub fn seek(&mut self, level: i32, _flag: bool) {
        if level == 0 {
            self.para_i = 0;
            self.run_i = 0;
        }
    }
    pub fn next(&mut self, level: i32, _in_para: bool) -> bool {
        if level == 1 {
            self.run_i = self.run_i.saturating_add(1);
            return self
                .paras
                .get(self.para_i)
                .map(|p| self.run_i < p.runs.len())
                .unwrap_or(false);
        }
        self.para_i = self.para_i.saturating_add(1);
        self.run_i = 0;
        self.para_i < self.paras.len()
    }
    pub fn range(&self, _level: i32) -> IndexRange {
        self.current_run()
            .map(|r| IndexRange {
                start: r.start,
                end: r.end,
            })
            .unwrap_or_default()
    }
    pub fn text(&self, _level: i32) -> String {
        self.current_run().map(|r| r.text.clone()).unwrap_or_default()
    }
    pub fn text__int(&self, level: i32) -> String {
        self.text(level)
    }
    pub fn format(&self, level: i32, _deep: bool) -> Option<KoAr> {
        if level == 0 {
            return self
                .paras
                .get(self.para_i)
                .map(|p| KoAr::Para(p.para_prop.clone()));
        }
        self.current_run()
            .map(|r| KoAr::Run(r.ch_prop.clone()))
    }
    pub fn format__int_boolean(&self, level: i32, deep: bool) -> Option<KoAr> {
        self.format(level, deep)
    }
    pub fn textBodyProperties(&self, _deep: bool) -> TextBodyProperties {
        self.body_pr.clone()
    }
    pub fn hasParagraphList(&self) -> bool {
        self.has_paragraph_list
    }
    pub fn getTextBody(&mut self) -> &mut KmoTextBody {
        &mut self.text_body
    }
    pub fn save(&mut self) {
        self.saved = true;
    }
    pub fn resetDateFieldTextRuns(&self, _flag: bool) -> bool {
        false
    }
    pub fn resetSlideNumFieldTextRuns(&self, _n: i32, _flag: bool) -> bool {
        false
    }
    fn current_run(&self) -> Option<&TextRun> {
        self.paras.get(self.para_i)?.runs.get(self.run_i)
    }
}

#[derive(Clone, Debug)]
pub enum KoAr {
    Para(TextParagraphProperties),
    Run(TextCharacterProperties),
}

impl KoAr {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}

pub struct FontHelper;

impl FontHelper {
    pub fn getFontColor(runProp: &TextCharacterProperties, _ctx: &ColorInterpretCtx) -> i32 {
        runProp.font_color
    }
    pub fn getFontColor__textcharacterproperties_colorinterpretctx(
        runProp: &TextCharacterProperties,
        ctx: ColorInterpretCtx,
    ) -> i32 {
        Self::getFontColor(runProp, &ctx)
    }
    pub fn getFontSize(runProp: &TextCharacterProperties) -> f32 {
        runProp.font_size_pt
    }
    pub fn getCoreFontName(
        _ch: char,
        prop: &TextCharacterProperties,
        _phType: i32,
        _ctx: Option<&KmoContext>,
    ) -> String {
        if prop.font_name.is_empty() {
            "Arial".to_string()
        } else {
            prop.font_name.clone()
        }
    }
    pub fn isSplitCharacter(ch: char) -> bool {
        ch == '\u{0}' || ch == '\u{fffc}'
    }
}

pub struct SymbolConverter;

impl SymbolConverter {
    pub fn convertToSymbol__string_string(text: String, _font: String) -> String {
        text
    }
    pub fn convertToSymbol__stringbuffer_int_int_string(
        buf: &str,
        _a: i32,
        _b: i32,
        _font: String,
    ) -> String {
        buf.to_string()
    }
}

pub struct ColorInterpreter;

impl ColorInterpreter {
    pub fn getHexRGBAValue(_c: Color, _ctx: ColorInterpretCtx) -> i32 {
        -16777216
    }
}

pub trait Bitmap {}
pub struct Blip;
pub struct FileDataStorage;
pub struct ColorTransform;
#[derive(Clone, Debug, Default)]
pub struct Effect;
pub struct EffectList;
pub struct InnerShadowEffect;
pub struct OuterShadowEffect;
pub struct PresetShadowEffect;
pub struct ReflectionEffect;
pub struct FillMode;
pub struct Hyperlink;
pub struct TextFont;
pub struct ISlide;
pub struct KmoMediaLib;
pub struct KmoMediaData;
impl KmoMediaData {
    pub fn getData(&self) -> FileDataStorage { FileDataStorage }
}
impl FileDataStorage {
    pub fn getFile(&self) -> FilePath { FilePath }
}
pub struct FilePath;
impl FilePath {
    pub fn getAbsolutePath(&self) -> String { String::new() }
}
impl KmoMediaLib {
    pub fn getMediaData(&self, _id: i32) -> KmoMediaData { KmoMediaData }
}
pub struct TileInfo;
pub struct AutoNumTransform;
pub struct PicFormatConvertUtils;
pub struct PaintFontHelper;
pub struct ITypeface;
pub struct TextUtils;

impl Blip {
    pub fn hasEmbedMediaID(&self) -> bool { false }
    pub fn embedMediaID(&self) -> i32 { -1 }
    pub fn hasLinkMediaID(&self) -> bool { false }
    pub fn linkMediaID(&self) -> i32 { -1 }
    pub fn effects(&self) -> EffectList { EffectList {} }
}
impl EffectList {
    pub fn read(&self, _out: &mut Vec<Effect>) {}
}
impl FillMode {
    pub fn r#type(&self) -> i32 { 0 }
    pub fn tile(&self) -> Option<TileInfo> { None }
}
impl PaintFontHelper {
    pub fn getITypeface(_n: String, _b: bool, _i: bool) -> ITypeface { ITypeface }
}
impl ITypeface {
    pub fn getFamily(&self) -> ITypeface { ITypeface }
    pub fn getName(&self) -> String { String::new() }
}

impl TextUtils {
    pub fn equals(_a: String, _b: String) -> bool {
        true
    }
}
