#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_doc_comments)]

// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::IOAssert;
// Java: import cn.wps.io.xwpf.element_handler.styles.StylePrModel
use io_wps_io::cn::wps::io::xwpf::element_handler::styles::StylePrModel::StylePrModel;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::MutablePropertySet;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.moffice.writer.core.KStyle
use crate::cn::wps::moffice::writer::core::KStyle::{KStyle, TextDocument};
// Java: import cn.wps.moffice.writer.core.KStyles
use crate::cn::wps::moffice::writer::core::KStyles::KStyles;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.helper.DocxReaderHelper
// rust: no-stmt（DocxReaderHelper.setTblStylePrType 未改写 → 971752949 分支 DEFERRED）
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.BuildinStyleSheet
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::BuildinStyleSheet::BuildinStyleSheet;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ConvertHelper
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::{Attributes, ConvertHelper};
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ShareConvertHelper
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ShareConvertHelper::ShareConvertHelper;
// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import java.util.UUID
// rust: Java UUID → uuid crate（word-core Cargo.toml uuid v4）

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::IOStyle::IOStyle;
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::StyleId2Sti::StyleId2Sti;
use uuid::Uuid;

// Java: public class StylesImporter
pub struct StylesImporter {
    // Java field: KStyles mStyles
    pub mStyles: Rc<RefCell<KStyles>>,
    // Java field: StyleId2Sti mStyleId2Sti
    pub(crate) mStyleId2Sti: Rc<RefCell<StyleId2Sti>>,
    // Java field: TextDocument mTextDocument
    mTextDocument: TextDocument,
    // Java field: IOStyle mCurIOStyle
    mCurIOStyle: Option<IOStyle>,
    // Java field: List<IOStyle> mIOStyleList
    mIOStyleList: Vec<IOStyle>,
}

// Java: public class StylesImporter
impl StylesImporter {
    // Java: StylesImporter(KStyles kStyles, StyleId2Sti styleId2Sti)
    pub fn StylesImporter(kStyles: Rc<RefCell<KStyles>>, styleId2Sti: Rc<RefCell<StyleId2Sti>>) -> Self {
        // Java: IOAssert.assertNotNull("kStyles should not be null", kStyles);
        let kstyles_opt = Some(&kStyles);
        IOAssert::assertNotNull_msg("kStyles should not be null", &kstyles_opt);
        // Java: IOAssert.assertNotNull("styleId2Sti should not be null", styleId2Sti);
        let sti_opt = Some(&styleId2Sti);
        IOAssert::assertNotNull_msg("styleId2Sti should not be null", &sti_opt);
        // Java: this.mStyles = kStyles;
        // Java: this.mStyleId2Sti = styleId2Sti;
        // Java: this.mTextDocument = kStyles.getTextDocument();
        let mTextDocument: TextDocument = kStyles.borrow().getTextDocument();
        StylesImporter {
            mStyles: kStyles,
            mStyleId2Sti: styleId2Sti,
            mTextDocument,
            mCurIOStyle: None,
            mIOStyleList: Vec::new(),
        }
    }

    // Java: public void importDocDefaults(MutablePropertySet rPrSet, MutablePropertySet pPrSet)
    // rust: Java null 参数 → Option（rPrSet/pPrSet 可缺席）
    pub fn importDocDefaults(&self, rPrSet: Option<&mut MutablePropertySet>, pPrSet: Option<&mut MutablePropertySet>) {
        // Java: if (null != rPrSet)
        if let Some(r) = rPrSet {
            // Java: this.mStyles.setRPrDefault(rPrSet.freeze());
            self.mStyles.borrow_mut().setRPrDefault(&r.freeze());
        }
        // Java: if (null != pPrSet)
        if let Some(p) = pPrSet {
            // Java: this.mStyles.setPPrDefault(pPrSet.freeze());
            self.mStyles.borrow_mut().setPPrDefault(&p.freeze());
        }
    }

    // Java: public void importStyleStart(Attributes attributes)
    pub fn importStyleStart(&mut self, attributes: &dyn Attributes) {
        // Java: IOAssert.assertNotNull("attributes should not be null", attributes);
        // Java: this.mCurIOStyle = new IOStyle();
        let mut cur = IOStyle::default();
        // Java: this.mCurIOStyle.mType = ConvertHelper.getString(attributes, "type");
        cur.mType = ConvertHelper::getString__String(attributes, "type");
        // Java: this.mCurIOStyle.mIsCustom = ConvertHelper.getBoolean(attributes, "customStyle");
        cur.mIsCustom = ConvertHelper::getBoolean(attributes, "customStyle");
        // Java: this.mCurIOStyle.mStyleId = ConvertHelper.getString(attributes, "styleId");
        cur.mStyleId = ConvertHelper::getString__String(attributes, "styleId");
        self.mCurIOStyle = Some(cur);
    }

    // Java: public void importStyleChild(int elementId, Attributes attributes)
    pub fn importStyleChild(&mut self, elementId: i32, attributes: &dyn Attributes) {
        // Java: IOAssert.assertNotNull("attributes should not be null", attributes);
        // Java: IOAssert.assertNotNull("mIOStyle should not be null", this.mCurIOStyle);
        let cur = self.mCurIOStyle.as_mut().expect("mCurIOStyle should not be null");
        // Java: switch (elementId)
        match elementId {
            // Java: case -681243704:
            -681243704 => {
                // Java: this.mCurIOStyle.mQFormat = ConvertHelper.toBoolean4NotNull(attributes);
                cur.mQFormat = ConvertHelper::toBoolean4NotNull(attributes);
            }
            // Java: case -332612366:
            -332612366 => {
                // Java: this.mCurIOStyle.mBasedOn = ConvertHelper.getString(attributes);
                cur.mBasedOn = ConvertHelper::getString(attributes);
            }
            // Java: case 3321850:
            3321850 => {
                // Java: this.mCurIOStyle.mLink = ConvertHelper.getString(attributes);
                cur.mLink = ConvertHelper::getString(attributes);
            }
            // Java: case 3373707:
            3373707 => {
                // Java: this.mCurIOStyle.mName = ConvertHelper.getString(attributes);
                cur.mName = ConvertHelper::getString(attributes);
            }
            // Java: case 3377907:
            3377907 => {
                // Java: this.mCurIOStyle.mNext = ConvertHelper.getString(attributes);
                cur.mNext = ConvertHelper::getString(attributes);
            }
            // Java: case 872749144:
            872749144 => {
                // Java: this.mCurIOStyle.mUiPriority = ShareConvertHelper.getLong4STDecimalNumber(attributes);
                cur.mUiPriority = ShareConvertHelper::getLong4STDecimalNumber(attributes);
            }
            // Java: case 971752949:
            971752949 => {
                /** DEFERRED_NON_FUNCTION
                // Java: DocxReaderHelper.setTblStylePrType(ConvertHelper.getString(attributes, "type"), this.mCurIOStyle);
                **/
                let _ = attributes;
            }
            // Java: default:
            _ => {
            }
        }
    }

    // Java: public void importStyleEnd(StylePrModel stylePrModel)
    pub fn importStyleEnd(&mut self, stylePrModel: &StylePrModel) {
        // Java: IOAssert.assertNotNull("mIOStyle should not be null", this.mCurIOStyle);
        // Java: IOAssert.assertNotNull("stylePropSet should not be null", stylePrModel);
        // Java: this.mCurIOStyle.convertTypeAndId(this.mStyleId2Sti);
        {
            let cur = self.mCurIOStyle.as_mut().expect("mCurIOStyle should not be null");
            let mut sti = self.mStyleId2Sti.borrow_mut();
            cur.convertTypeAndId(&mut sti);
        }
        // Java: this.mCurIOStyle.mCorePrSet.add(getStylePrSet(stylePrModel));
        {
            let cur = self.mCurIOStyle.as_mut().unwrap();
            let mut style_pr_set = Self::getStylePrSet(stylePrModel);
            let frozen = style_pr_set.freeze();
            cur.mCorePrSet.add(&frozen);
        }
        // Java: this.mIOStyleList.add(this.mCurIOStyle);
        let cur = self.mCurIOStyle.take().expect("mCurIOStyle should not be null");
        self.mIOStyleList.push(cur);
    }

    // Java: private static MutablePropertySet getStylePrSet(StylePrModel stylePrModel)
    fn getStylePrSet(stylePrModel: &StylePrModel) -> MutablePropertySet {
        // Java: MutablePropertySet stylePrSet = new MutablePropertySet();
        let mut stylePrSet = MutablePropertySet::new();
        // Java: stylePrSet.add(stylePrModel.mRprBaseSet);
        if let Some(ps) = Self::to_frozen(&stylePrModel.mRprBaseSet) {
            stylePrSet.add(&ps);
        }
        // Java: stylePrSet.add(stylePrModel.mPPrGeneralSet);
        if let Some(ps) = Self::to_frozen(&stylePrModel.mPPrGeneralSet) {
            stylePrSet.add(&ps);
        }
        // Java: stylePrSet.add(correctTcPrSet(stylePrModel));
        if let Some(mut ps) = Self::correctTcPrSet(stylePrModel) {
            let frozen = ps.freeze();
            stylePrSet.add(&frozen);
        }
        // Java: stylePrSet.add(stylePrModel.mTblPrBaseSet);
        if let Some(ps) = Self::to_frozen(&stylePrModel.mTblPrBaseSet) {
            stylePrSet.add(&ps);
        }
        // Java: return stylePrSet;
        stylePrSet
    }

    /// Box<dyn Any> → freeze 后的 PropertySet（Java null 字段 → None）
    fn to_frozen(b: &Option<Box<dyn Any + Send + Sync>>) -> Option<PropertySet> {
        match b {
            Some(x) => match x.downcast_ref::<MutablePropertySet>() {
                Some(mps) => {
                    let mut c = mps.clone_mps();
                    Some(c.freeze())
                }
                None => None,
            },
            None => None,
        }
    }

    // Java: private static MutablePropertySet correctTcPrSet(StylePrModel stylePrModel)
    fn correctTcPrSet(stylePrModel: &StylePrModel) -> Option<MutablePropertySet> {
        // Java: MutablePropertySet tcPrSet = stylePrModel.mTcPrSet;
        let mut tcPrSet: Option<MutablePropertySet> = match &stylePrModel.mTcPrSet {
            Some(b) => match b.downcast_ref::<MutablePropertySet>() {
                Some(mps) => Some(mps.clone_mps()),
                None => None,
            },
            None => None,
        };
        // Java: if (null != tcPrSet)
        if let Some(ref mut t) = tcPrSet {
            // Java: tcPrSet.remove(346);
            t.remove(346);
            // Java: tcPrSet.remove(347);
            t.remove(347);
            // Java: tcPrSet.remove(348);
            t.remove(348);
            // Java: tcPrSet.remove(349);
            t.remove(349);
            // Java: tcPrSet.remove(350);
            t.remove(350);
            // Java: tcPrSet.remove(351);
            t.remove(351);
            // Java: correctCellPaddingStype(tcPrSet);
            Self::correctCellPaddingStype(t);
        }
        // Java: return tcPrSet;
        tcPrSet
    }

    // Java: private static void correctCellPaddingStype(MutablePropertySet dst)
    fn correctCellPaddingStype(_dst: &mut MutablePropertySet) {
        /** DEFERRED_NON_FUNCTION
        // Java: ArrayList<CellMargin> listCellMargin = (ArrayList)dst.getValue(355);
        // Java: if (null != listCellMargin && listCellMargin.size() != 0)
        // Java: CellMargin[] cellMargins = new CellMargin[listCellMargin.size()];
        // Java: listCellMargin.toArray(cellMargins);
        // Java: dst.setValue(355, cellMargins);
        （355 的 List→CellMargin[] 数组化延后：PropertyValue 对数组/值类型的承载待 wave0/值类型接线后启用）
        **/
        let _ = _dst;
    }

    // Java: public void importStylesEnd()
    pub fn importStylesEnd(&mut self) {
        // Java: int size = this.mIOStyleList.size();
        let size = self.mIOStyleList.len();
        // Java: for (int i = 0; i < size; ++i)
        let mut i = 0;
        while i < size {
            // Java: IOStyle ioStyle = (IOStyle)this.mIOStyleList.get(i);
            let ioStyle = std::mem::take(self.mIOStyleList.get_mut(i).expect("ioStyle"));
            // Java: KStyle kStyle = this.createKStyle(ioStyle);
            let mut kStyle = Self::createKStyle(&self.mTextDocument, &mut *self.mStyleId2Sti.borrow_mut(), &ioStyle);
            // Java: int link = toSti(ioStyle.mLink, this.mStyleId2Sti);
            let link = Self::toSti(ioStyle.mLink.as_deref(), &*self.mStyleId2Sti.borrow());
            // Java: kStyle.setLink(link);
            kStyle.setLink(link);
            // Java: if (null != ioStyle.mQFormat)
            if let Some(qFormat) = ioStyle.mQFormat {
                // Java: kStyle.setQFormat(ioStyle.mQFormat);
                kStyle.setQFormat(qFormat);
            }
            // Java: if (null != ioStyle.mUiPriority)
            if let Some(uiPriority) = ioStyle.mUiPriority {
                // Java: kStyle.setUiPriority(ioStyle.mUiPriority);
                kStyle.setUiPriority(uiPriority);
            }
            // Java: this.mStyles.add(kStyle);
            self.mStyles.borrow_mut().add(&mut kStyle);
            // Java: ioStyle = null;
            *self.mIOStyleList.get_mut(i).expect("ioStyle") = ioStyle;
            i += 1;
        }
    }

    // Java: private KStyle createKStyle(IOStyle ioStyle)
    // rust: 依赖的 mStyleId2Sti/mTextDocument 以参数传入（Promise 保持 Java 语义）
    fn createKStyle(textDocument: &TextDocument, styleId2Sti: &mut StyleId2Sti, ioStyle: &IOStyle) -> KStyle {
        // Java: int id = toSti(ioStyle.rectifyStyleId(), this.mStyleId2Sti);
        let id = Self::toSti(Some(&ioStyle.rectifyStyleId()), styleId2Sti);
        // Java: int base = toBaseSti(ioStyle.mBasedOn, this.mStyleId2Sti, id);
        let base = Self::toBaseSti(ioStyle.mBasedOn.as_deref(), styleId2Sti, id);
        // Java: int next = toNextsSti(ioStyle.mNext, id, this.mStyleId2Sti);
        let next = Self::toNextsSti(ioStyle.mNext.as_deref(), id, styleId2Sti);
        // Java: String name = toDisplayName(ioStyle.mName, id);
        let name = Self::toDisplayName(ioStyle.mName.as_deref(), id);
        // Java: PropertySet prSet = ioStyle.mCorePrSet.freeze();
        let mut core_pr_set = ioStyle.mCorePrSet.clone_mps();
        let prSet = core_pr_set.freeze();
        // Java: return new KStyle(this.mTextDocument, id, name, base, next, ioStyle.mCoreType, prSet);
        KStyle::new__textdocument_int_string_int_int_int_propertyset(textDocument, id, &name, base, next, ioStyle.mCoreType, &prSet)
    }

    // Java: private static String toDisplayName(String name, int sti)
    fn toDisplayName(name: Option<&str>, sti: i32) -> String {
        // Java: if (null != name && BuildinStyleSheet.isReservedName(name))
        if let Some(n) = name {
            if BuildinStyleSheet::isReservedName(n) {
                // Java: return BuildinStyleSheet.isReservedIndex(sti) ? name : name + "_" + UUID.randomUUID().toString();
                if BuildinStyleSheet::isReservedIndex(sti) {
                    return n.to_string();
                } else {
                    return format!("{}_{}", n, Uuid::new_v4());
                }
            }
        }
        // Java: else
        // Java: return name;
        // rust: Java 允许 name=null 传给 KStyle；Rust name: &str 用空串等价（KStyle 下游按空处理）
        name.unwrap_or_default().to_string()
    }

    // Java: private static int toBaseSti(String base, StyleId2Sti styleId2Sti, int coreSti)
    fn toBaseSti(base: Option<&str>, styleId2Sti: &StyleId2Sti, coreSti: i32) -> i32 {
        // Java: int baseSti = toSti(base, styleId2Sti);
        let mut baseSti = Self::toSti(base, styleId2Sti);
        // Java: baseSti = baseSti == coreSti ? 4095 : baseSti;
        if baseSti == coreSti {
            baseSti = 4095;
        }
        // Java: return 0 == coreSti ? 4095 : baseSti;
        if coreSti == 0 {
            4095
        } else {
            baseSti
        }
    }

    // Java: private static int toSti(String styleId, StyleId2Sti styleId2Sti)
    fn toSti(styleId: Option<&str>, styleId2Sti: &StyleId2Sti) -> i32 {
        // Java: if (null == styleId)
        if styleId.is_none() {
            // Java: return 4095;
            return 4095;
        }
        // Java: else
        // Java: Integer sti = styleId2Sti.getSti(styleId);
        let sti: Option<i32> = styleId2Sti.getSti(styleId.unwrap());
        // Java: return null == sti ? 4095 : sti;
        sti.unwrap_or(4095)
    }

    // Java: private static int toNextsSti(String nextId, int sti, StyleId2Sti styleId2Sti)
    fn toNextsSti(nextId: Option<&str>, sti: i32, styleId2Sti: &StyleId2Sti) -> i32 {
        // Java: if (null == nextId)
        if nextId.is_none() {
            // Java: return sti;
            return sti;
        }
        // Java: else
        // Java: Integer nextStyleSti = styleId2Sti.getSti(nextId);
        let nextStyleSti: Option<i32> = styleId2Sti.getSti(nextId.unwrap());
        // Java: return null == nextStyleSti ? sti : nextStyleSti;
        match nextStyleSti {
            Some(s) => s,
            None => sti,
        }
    }

    // Java: public void importTblStylePrEnd(StylePrModel stylePrModel)
    pub fn importTblStylePrEnd(&self, stylePrModel: Option<&StylePrModel>) {
        /** DEFERRED_NON_FUNCTION
        // Java: IOAssert.assertNotNull("stylePrModel should not be null", stylePrModel);
        // Java: setConditionalCharacterFormatting(stylePrModel.mRprBaseSet, this.mCurIOStyle);
        // Java: setConditionalParagraphFormatting(stylePrModel.mPPrGeneralSet, this.mCurIOStyle);
        // Java: setTableStyleProperties(stylePrModel, this.mCurIOStyle);
        （表样式条件格式 CFC/CPF/TSP：依赖 Conditional 与 TableStyleProperties 值类型，MUST 可延后）
        **/
        let _ = stylePrModel;
    }

    // Java: private static void setConditionalCharacterFormatting(MutablePropertySet rPrSet, IOStyle ioStyle)
    fn setConditionalCharacterFormatting(_rPrSet: Option<&PropertySet>, _ioStyle: &mut IOStyle) {
        /** DEFERRED_NON_FUNCTION
        // Java: ConditionalCharacterFormatting ccf = new ConditionalCharacterFormatting();
        // Java: if (null != rPrSet) ccf.setProperty(rPrSet.freeze());
        // Java: ioStyle.mCorePrSet.setValue(ioStyle.mTblStyleRPrId, ccf);
        **/
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: private static void setConditionalParagraphFormatting(MutablePropertySet pPrSet, IOStyle ioStyle)
    fn setConditionalParagraphFormatting(_pPrSet: Option<&PropertySet>, _ioStyle: &mut IOStyle) {
        /** DEFERRED_NON_FUNCTION
        // Java: ConditionalParagraphFormatting cpf = new ConditionalParagraphFormatting();
        // Java: if (null != pPrSet) cpf.setProperty(pPrSet.freeze());
        // Java: ioStyle.mCorePrSet.setValue(ioStyle.mTblStylePPrId, cpf);
        **/
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: private static void setTableStyleProperties(StylePrModel stylePrModel, IOStyle ioStyle)
    fn setTableStyleProperties(_stylePrModel: &StylePrModel, _ioStyle: &mut IOStyle) {
        /** DEFERRED_NON_FUNCTION
        // Java: TableStyleProperties tsp = new TableStyleProperties();
        // Java: MutablePropertySet tspSet = new MutablePropertySet();
        // Java: setTableStyleProp(stylePrModel, tspSet);
        // Java: tsp.setProperty(tspSet.freeze());
        // Java: ioStyle.mCorePrSet.setValue(ioStyle.mTblStyleTblPrId, tsp);
        **/
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: private static void setTableStyleProp(StylePrModel stylePrModel, MutablePropertySet tspSet)
    fn setTableStyleProp(_stylePrModel: &StylePrModel, _tspSet: &mut MutablePropertySet) {
        /** DEFERRED_NON_FUNCTION
        // Java: tspSet.add(stylePrModel.mTblPrBaseSet);
        // Java: MutablePropertySet tcPrSet = stylePrModel.mTcPrSet;
        // Java: if (null != tcPrSet)
        // Java: tcPrSet.remove(309);
        // Java: correctCellPaddingStype(tcPrSet);
        // Java: tspSet.add(tcPrSet);
        **/
        todo!("DEFERRED_NON_FUNCTION");
    }
}