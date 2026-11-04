// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// Java: import cn.wps.base.io.MathArgmentType
use util::base::io::MathArgmentType::*;
// Java: import cn.wps.collections.textrope.ITextPool
use crate::cn::wps::collections::textrope::ITextPool::*;
// Java: import cn.wps.collections.textrope.ITextRope
use crate::cn::wps::collections::textrope::ITextRope::*;
// Java: import cn.wps.io.xwpf.MathIOType
use io_wps_io::cn::wps::io::xwpf::MathIOType::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.SubDocument
use crate::cn::wps::moffice::writer::core::SubDocument::*;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::*;
// Java: import cn.wps.moffice.writer.data.PLCMathEnd
use crate::cn::wps::moffice::writer::data::PLCMathEnd::*;
// Java: import cn.wps.moffice.writer.data.PLCMathFunction
use crate::cn::wps::moffice::writer::data::PLCMathFunction::*;
// Java: import cn.wps.moffice.writer.data.PLCMathStart
use crate::cn::wps::moffice::writer::data::PLCMathStart::*;
// Java: import cn.wps.moffice.writer.data.math.MathObjectProp
use crate::cn::wps::moffice::writer::data::math::MathObjectProp::*;
// Java: import cn.wps.moffice.writer.data.math.MatrixColumn
use crate::cn::wps::moffice::writer::data::math::MatrixColumn::*;
// Java: import cn.wps.moffice.writer.data.math.MatrixProp
use crate::cn::wps::moffice::writer::data::math::MatrixProp::*;
// Java: import cn.wps.moffice.writer.helper.CoreHelper
use crate::cn::wps::moffice::writer::helper::CoreHelper::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ConvertHelper
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.MathImporterHelper
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::MathImporterHelper::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.MathPropsConverter
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::MathPropsConverter::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.MathPropsConverterSimple
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::MathPropsConverterSimple::*;
// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import java.util.Stack
use stubs::java::util::Stack::*;
// Java: import org.xml.sax.Attributes
use kfc_kfc_alpha::kfc::html::reader::lexical::token::Attributes::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
// Java: import org.apache.poi.poifs.filesystem.Entry
use io_poi_compact::org::apache::poi::poifs::filesystem::Entry::*;

// java public class MathImporter
#[derive(Default)]
pub struct MathImporter {
    // java protected Document mDocument;
    pub(crate) mDocument: Option<Document>,
    // java private Stack<Integer> mMathSepaCPStack = null;
    mMathSepaCPStack: Option<Vec<i32>>,
    // java private Stack<PLCMathStart.OMathStartNode> mOMathStartStack = null;
    mOMathStartStack: Option<Vec<PLCMathStart::OMathStartNode>>,
    // java private Stack<PLCMathFunction.MathStartNode> mBeginMathNodeStack = null;
    mBeginMathNodeStack: Option<Vec<PLCMathFunction::MathStartNode>>,
}

// java public class MathImporter
impl MathImporter {
    // java public  MathImporter(Document document)
    pub fn MathImporter(document: &dyn Document) -> Self {
      // java IOAssert.assertNotNull("document should not be null!", document);
      IOAssert::assertNotNull__string_object(&"document should not be null!", Some(&document));
      // java this.mDocument = document;
      let mut this = Self { mDocument: document, mMathSepaCPStack: Vec::new(), mOMathStartStack: Vec::new(), mBeginMathNodeStack: Vec::new() };
      // java this.mMathSepaCPStack = new Stack();
      // java this.mOMathStartStack = new Stack();
      // java this.mBeginMathNodeStack = new Stack();
      this
    }

    // java public void importOMathStart(boolean isInOMathPara, MutablePropertySet oMathParaPr)
    pub fn importOMathStart(&mut self, isInOMathPara: bool, oMathParaPr: &MutablePropertySet) {
      // java PLCMathStart plcOMathStart = this.mDocument.getOMathStarts();
      let mut plcOMathStart: PLCMathStart = self.mDocument.getOMathStarts();
      // java if (null == plcOMathStart)
      if plcOMathStart.is_none() {
        // java this.mDocument.createPLCOMath();
        self.mDocument.createPLCOMath();
        // java plcOMathStart = this.mDocument.getOMathStarts();
        plcOMathStart = self.mDocument.getOMathStarts();
      }
      // java int cp = this.getLengthDocument();
      let mut cp: i32 = self.getLengthDocument();
      // java PLCMathStart.OMathStartNode startNode = plcOMathStart.addOMathStart(cp);
      let mut startNode: PLCMathStart::OMathStartNode = plcOMathStart.addOMathStart(cp);
      // java this.importOMathPara(isInOMathPara, oMathParaPr, startNode);
      self.importOMathPara(isInOMathPara, Some(&oMathParaPr), &mut startNode);
      // java this.mOMathStartStack.add(startNode);
      self.mOMathStartStack.add(startNode);
    }

    // java private void importOMathPara(boolean isInOMathPara, MutablePropertySet oMathParaPr, PLCMathStart.OMathStartNode startNode)
    pub fn importOMathPara(&mut self, isInOMathPara: bool, oMathParaPr: Option<&MutablePropertySet>, startNode: &mut PLCMathStart::OMathStartNode) {
      // java if (isInOMathPara)
      if isInOMathPara {
        // java startNode.createOMathPara();
        startNode.createOMathPara();
        // java if (null != oMathParaPr)
        if let Some(oMathParaPr) = oMathParaPr {
          // java startNode.setOMathParaPr(true);
          startNode.setOMathParaPr(true);
          // java Object jc = oMathParaPr.getValue(62);
          let mut jc: Box<dyn Object> = oMathParaPr.getValue(62);
          // java if (null != jc)
          if jc.is_some() {
            // java startNode.setOMathParaPr((Integer)jc);
            startNode.setOMathParaPr(((jc) as i32));
          }
        }
      }
    }

    // java protected int getLengthDocument()
    pub fn getLengthDocument(&self) -> i32 {
      // java return this.mDocument.getLength();
      return self.mDocument.getLength();
    }

    // java public void importOMathEnd()
    pub fn importOMathEnd(&mut self) {
      // java PLCMathEnd plcOMathEnd = this.mDocument.getOMathEnds();
      let mut plcOMathEnd: PLCMathEnd = self.mDocument.getOMathEnds();
      // java if (null == plcOMathEnd)
      if plcOMathEnd.is_none() {
        // java this.mDocument.createPLCOMath();
        self.mDocument.createPLCOMath();
        // java plcOMathEnd = this.mDocument.getOMathEnds();
        plcOMathEnd = self.mDocument.getOMathEnds();
      }
      // java PLCMathStart plcOMathStart = this.mDocument.getOMathStarts();
      let mut plcOMathStart: PLCMathStart = self.mDocument.getOMathStarts();
      // java int cp = this.getLengthDocument();
      let mut cp: i32 = self.getLengthDocument();
      // java PLCMathEnd.OMathEndNode endNode = plcOMathEnd.addOMathEnd(cp);
      let mut endNode: PLCMathEnd::OMathEndNode = plcOMathEnd.addOMathEnd(cp);
      // java IOAssert.assertFalse("mOMathStartStack should not be empty", this.mOMathStartStack.isEmpty());
      IOAssert::assertFalse__string_boolean(&"mOMathStartStack should not be empty", self.mOMathStartStack.is_empty());
      // java PLCMathStart.OMathStartNode startNode = (PLCMathStart.OMathStartNode)this.mOMathStartStack.pop();
      let mut startNode: PLCMathStart::OMathStartNode = (self.mOMathStartStack.pop()).downcast_ref::<PLCMathStart::OMathStartNode>().unwrap();
      // java if (startNode.getCP() == endNode.getCP())
      if startNode.getCP() == endNode.getCP() {
        // java plcOMathStart.remove(startNode);
        plcOMathStart.remove(startNode);
        // java plcOMathEnd.remove(endNode);
        plcOMathEnd.remove(endNode);
      }
      // java else
      else {
        // java startNode.setEnd(endNode);
        startNode.setEnd(endNode);
        // java endNode.setStart(startNode);
        endNode.setStart(startNode);
      }
    }

    // java public void importerMathObjectBegin(MathIOType mathType)
    pub fn importerMathObjectBegin(&mut self, mathType: &MathIOType) {
      // java PLCMathFunction.MathStartNode mathStartNode = this.importMathStartNode('\u001c', PropertySet.empty);
      let mut mathStartNode: PLCMathFunction::MathStartNode = self.importMathStartNode('\u{001c}', &PropertySet::empty());
      // java this.initMathBeginNodeProperty(mathType, mathStartNode);
      self.initMathBeginNodeProperty(&mathType, &mut mathStartNode);
      // java this.mBeginMathNodeStack.push(mathStartNode);
      self.mBeginMathNodeStack.push(mathStartNode);
    }

    // java private PLCMathFunction.MathStartNode importMathStartNode(char placeHolder, PropertySet prop)
    pub fn importMathStartNode(&mut self, placeHolder: char, prop: &PropertySet) -> PLCMathFunction::MathStartNode {
      // java IOAssert.assertNotNull("mDocument should not be null", this.mDocument);
      IOAssert::assertNotNull__string_object(&"mDocument should not be null", Some(&self.mDocument));
      // java PLCMathFunction plcMath = this.getPLCMathFunction();
      let mut plcMath: PLCMathFunction = self.getPLCMathFunction();
      // java int cp = this.appendMathControlChar(placeHolder, prop);
      let mut cp: i32 = self.appendMathControlChar(placeHolder, &prop);
      // java return plcMath.addMathStartNode(cp);
      return plcMath.addMathStartNode(cp);
    }

    // java private void initMathBeginNodeProperty(MathIOType mathType, PLCMathFunction.MathStartNode mathStartNode)
    pub fn initMathBeginNodeProperty(&mut self, mathType: &MathIOType, mathStartNode: &mut PLCMathFunction::MathStartNode) {
      // java MutablePropertySet propSet = new MutablePropertySet(mathStartNode.getPropertySet());
      let mut propSet: MutablePropertySet = MutablePropertySet::new(mathStartNode.getPropertySet());
      // java MathImporterHelper.initMathNodePrObj(mathType, propSet);
      MathImporterHelper::initMathNodePrObj(&mathType, &mut propSet);
      // java propSet.setValue(738, new ArrayList());
      propSet.setValue(738, Vec::new());
      // java if (MathIOType.MATRIX == mathType)
      if MathIOType::MATRIX == mathType {
        // java propSet.setValue(740, new ArrayList());
        propSet.setValue(740, Vec::new());
      }
      // java mathStartNode.setPropertySet(propSet.freeze());
      mathStartNode.setPropertySet(propSet.freeze());
      // java int mathCoreType = MathImporterHelper.convertToMathCoreType(mathType);
      let mut mathCoreType: i32 = MathImporterHelper::convertToMathCoreType(&mathType);
      // java mathStartNode.setType(mathCoreType);
      mathStartNode.setType(mathCoreType);
    }

    // java public void importMathObjectEndSimple(MutablePropertySet runPropSet, MathIOType mathType)
    pub fn importMathObjectEndSimple(&mut self, runPropSet: &MutablePropertySet, mathType: &MathIOType) {
      // java IOAssert.assertNotNull("mathType should not be null", mathType);
      IOAssert::assertNotNull__string_object(&"mathType should not be null", Some(&mathType));
      // java PLCMathFunction.MathStartNode mathStartNode = (PLCMathFunction.MathStartNode)this.mBeginMathNodeStack.pop();
      let mut mathStartNode: PLCMathFunction::MathStartNode = (self.mBeginMathNodeStack.pop()).downcast_ref::<PLCMathFunction::MathStartNode>().unwrap();
      // java IOAssert.assertNotNull("mathStartNode should not be null", mathStartNode);
      IOAssert::assertNotNull__string_object(&"mathStartNode should not be null", Some(&mathStartNode));
      // java this.correctMathObjectData(mathStartNode);
      self.correctMathObjectData(&mathStartNode);
      // java int startCp = mathStartNode.getCP();
      let mut startCp: i32 = mathStartNode.getCP();
      // java this.setMathRunProperty(runPropSet, startCp);
      self.setMathRunProperty(Some(&mut runPropSet), startCp);
      // java this.insertMathEnd(mathType, startCp);
      self.insertMathEnd(&mathType, startCp);
    }

    // java private void correctMathObjectData(PLCMathFunction.MathStartNode mathStartNode)
    pub fn correctMathObjectData(&mut self, mathStartNode: &PLCMathFunction::MathStartNode) {
      // java this.correctArgSizeProp(mathStartNode);
      self.correctArgSizeProp(&mut mathStartNode);
      // java if (13 == mathStartNode.getType())
      if 13 == mathStartNode.getType() {
        // java this.correctMathMatrixColumn(mathStartNode);
        self.correctMathMatrixColumn(&mathStartNode);
      }
    }

    // java private void correctArgSizeProp(PLCMathFunction.MathStartNode mathStartNode)
    pub fn correctArgSizeProp(&mut self, mathStartNode: &mut PLCMathFunction::MathStartNode) {
      // java PropertySet propSet = mathStartNode.getPropertySet();
      let mut propSet: PropertySet = mathStartNode.getPropertySet();
      // java Object argSzObj = propSet.getValue(738);
      let mut argSzObj: Box<dyn Object> = propSet.getValue(738);
      // java ArrayList<Integer> argSizeList = (ArrayList)argSzObj;
      let mut argSizeList: Vec<i32> = todo!("cast → Vec<()>");
      // java Integer[] argSizeArr = new Integer[argSizeList.size()];
      let mut argSizeArr: Vec<i32> = vec![Default::default(); (argSizeList.len() as i32) as usize];
      // java argSizeList.toArray(argSizeArr);
      argSizeList.toArray(argSizeArr);
      // java MutablePropertySet mdst = new MutablePropertySet(propSet);
      let mut mdst: MutablePropertySet = MutablePropertySet::new(propSet);
      // java mdst.setValue(738, argSizeArr);
      mdst.setValue(738, argSizeArr);
      // java mathStartNode.setPropertySet(mdst.freeze());
      mathStartNode.setPropertySet(mdst.freeze());
    }

    // java private void correctMathMatrixColumn(PLCMathFunction.MathStartNode mathStartNode)
    pub fn correctMathMatrixColumn(&self, mathStartNode: &PLCMathFunction::MathStartNode) {
      // java ArrayList<MatrixColumn> mmcList = this.getMatrixColumns(mathStartNode);
      let mut mmcList: Vec<MatrixColumn> = self.getMatrixColumns(&mathStartNode);
      // java MatrixColumn[] mmcArr = new MatrixColumn[mmcList.size()];
      let mut mmcArr: Vec<MatrixColumn> = vec![Default::default(); (mmcList.len() as i32) as usize];
      // java mmcList.toArray(mmcArr);
      mmcList.toArray(mmcArr);
      // java PropertySet propSet = mathStartNode.getPropertySet();
      let mut propSet: PropertySet = mathStartNode.getPropertySet();
      // java MatrixProp mPr = (MatrixProp)propSet.getValue(737);
      let mut mPr: MatrixProp = (propSet.getValue(737)).downcast_ref::<MatrixProp>().unwrap();
      // java mPr.mcs = mmcArr;
      mPr.mcs = mmcArr;
    }

    // java private void insertMathEnd(MathIOType mathType, int startCp)
    pub fn insertMathEnd(&mut self, mathType: &MathIOType, startCp: i32) {
      // java if (MathImporterHelper.isTripleMathElement(mathType))
      if MathImporterHelper::isTripleMathElement(&mathType) {
        // java this.importTribleMathObjectEnd(startCp);
        self.importTribleMathObjectEnd(startCp);
      }
      // java else
      else {
        // java this.importNormalMathObjectEnd();
        self.importNormalMathObjectEnd();
      }
    }

    // java private void importTribleMathObjectEnd(int startCp)
    pub fn importTribleMathObjectEnd(&mut self, startCp: i32) {
      // java IChpxTable chpxTable = this.mDocument.getChpxTable();
      let mut chpxTable: Box<dyn IChpxTable> = self.mDocument.getChpxTable();
      // java IChpxTable.Entry iEntry = chpxTable.seek(startCp);
      let mut iEntry: Box<dyn Entry> = chpxTable.seek(startCp);
      // java PropertySet prop = iEntry.getProperty();
      let mut prop: PropertySet = iEntry.getProperty();
      // java int cp = this.appendMathControlChar('\u001b', prop);
      let mut cp: i32 = self.appendMathControlChar('\u{001b}', &prop);
      // java this.importMathEndNode(cp, '\u001b');
      self.importMathEndNode(cp, '\u{001b}');
    }

    // java private void importNormalMathObjectEnd()
    pub fn importNormalMathObjectEnd(&mut self) {
      // java int cp = this.getLengthDocument() - 1;
      let mut cp: i32 = self.getLengthDocument() - 1;
      // java this.convertControlChar(cp, '\u001b');
      self.convertControlChar(cp, '\u{001b}');
      // java PLCMathFunction plcMath = this.getPLCMathFunction();
      let mut plcMath: PLCMathFunction = self.getPLCMathFunction();
      // java PLCMathFunction.MathStartNode node = plcMath.findMathStartNodeByCP(cp);
      let mut node: PLCMathFunction::MathStartNode = plcMath.findMathStartNodeByCP(cp);
      // java if (node != null)
      if node.is_some() {
        // java plcMath.remove(node);
        plcMath.remove(node);
      }
      // java this.importMathEndNode(cp, '\u001b');
      self.importMathEndNode(cp, '\u{001b}');
    }

    // java private PLCMathFunction.MathNode importMathEndNode(int cp, char mathChar)
    pub fn importMathEndNode(&self, cp: i32, mathChar: char) -> PLCMathFunction::MathNode {
      // java IOAssert.assertNotNull("mDocument should not be null", this.mDocument);
      IOAssert::assertNotNull__string_object(&"mDocument should not be null", Some(&self.mDocument));
      // java PLCMathFunction plcMath = this.getPLCMathFunction();
      let mut plcMath: PLCMathFunction = self.getPLCMathFunction();
      // java return plcMath.addMathEndNode(cp);
      return plcMath.addMathEndNode(cp);
    }

    // java public void importMathElementsProp(MathIOType mathType, int eleId, Attributes attrs)
    pub fn importMathElementsProp(&mut self, mathType: &MathIOType, eleId: i32, attrs: &Attributes) {
      // java switch (eleId)
      match eleId {
        // java case 3478:
        3478 => {
          // java this.importMathMatrixColStart();
          self.importMathMatrixColStart();
        }
        // java case 3344751:
        3344751 => {
          // java this.convertMatrixColumnJC(attrs);
          self.convertMatrixColumnJC(&attrs);
        }
        // java case 93079389:
        93079389 => {
          // java this.updateMathArgSize(attrs);
          self.updateMathArgSize(&attrs);
        }
        // java case 94851343:
        94851343 => {
          // java this.convertMatrixCountProperty(attrs);
          self.convertMatrixCountProperty(&attrs);
        }
        // java default:
        _ => {
          // java this.convertMathNodeProperty(mathType, eleId, attrs);
          self.convertMathNodeProperty(&mathType, eleId, &attrs);
        }
      }
    }

    // java private void convertMatrixColumnJC(Attributes attrs)
    pub fn convertMatrixColumnJC(&self, attrs: &Attributes) {
      // java MatrixColumn latestMMC = this.getLatestMatrixColumn();
      let mut latestMMC: MatrixColumn = self.getLatestMatrixColumn();
      // java MathPropsConverterSimple.convertMathMcJc(attrs, latestMMC);
      MathPropsConverterSimple::convertMathMcJc(&attrs, &mut latestMMC);
    }

    // java private void convertMatrixCountProperty(Attributes attrs)
    pub fn convertMatrixCountProperty(&self, attrs: &Attributes) {
      // java MatrixColumn latestMMC = this.getLatestMatrixColumn();
      let mut latestMMC: MatrixColumn = self.getLatestMatrixColumn();
      // java MathPropsConverterSimple.convertMathCount(attrs, latestMMC);
      MathPropsConverterSimple::convertMathCount(&attrs, &mut latestMMC);
    }

    // java private MatrixColumn getLatestMatrixColumn()
    pub fn getLatestMatrixColumn(&mut self) -> MatrixColumn {
      // java List<MatrixColumn> mmcList = this.getMatrixColumns((PLCMathFunction.MathStartNode)this.mBeginMathNodeStack.peek());
      let mut mmcList: Vec<MatrixColumn> = self.getMatrixColumns(&(self.mBeginMathNodeStack.peek()).downcast_ref::<PLCMathFunction::MathStartNode>().unwrap());
      // java int size = mmcList.size();
      let mut size: i32 = (mmcList.len() as i32);
      // java return (MatrixColumn)mmcList.get(size - 1);
      return (mmcList.get(size - 1)).downcast_ref::<MatrixColumn>().unwrap();
    }

    // java private void convertMathNodeProperty(MathIOType mathType, int eleId, Attributes attrs)
    pub fn convertMathNodeProperty(&mut self, mathType: &MathIOType, eleId: i32, attrs: &Attributes) {
      // java PLCMathFunction.MathStartNode mathStartNode = (PLCMathFunction.MathStartNode)this.mBeginMathNodeStack.peek();
      let mut mathStartNode: PLCMathFunction::MathStartNode = (self.mBeginMathNodeStack.peek()).downcast_ref::<PLCMathFunction::MathStartNode>().unwrap();
      // java Object mathPrObj = this.getMathObjectProp(mathStartNode);
      let mut mathPrObj: Box<dyn Object> = self.getMathObjectProp(&mathStartNode);
      // java if (null != mathPrObj)
      if mathPrObj.is_some() {
        // java MathImporterHelper.convertMathNodeProperty(mathType, eleId, attrs, mathPrObj);
        MathImporterHelper::convertMathNodeProperty(&mathType, eleId, &attrs, &mathPrObj);
      }
    }

    // java private void importMathMatrixColStart()
    pub fn importMathMatrixColStart(&mut self) {
      // java PLCMathFunction.MathStartNode mathStartNode = (PLCMathFunction.MathStartNode)this.mBeginMathNodeStack.peek();
      let mut mathStartNode: PLCMathFunction::MathStartNode = (self.mBeginMathNodeStack.peek()).downcast_ref::<PLCMathFunction::MathStartNode>().unwrap();
      // java ArrayList<MatrixColumn> mmcList = this.getMatrixColumns(mathStartNode);
      let mut mmcList: Vec<MatrixColumn> = self.getMatrixColumns(&mathStartNode);
      // java mmcList.add(new MatrixColumn());
      mmcList.add(MatrixColumn::new());
    }

    // java private ArrayList<MatrixColumn> getMatrixColumns(PLCMathFunction.MathStartNode mathStartNode)
    pub fn getMatrixColumns(&self, mathStartNode: &PLCMathFunction::MathStartNode) -> Vec<MatrixColumn> {
      // java PropertySet propSet = mathStartNode.getPropertySet();
      let mut propSet: PropertySet = mathStartNode.getPropertySet();
      // java Object mmcObj = propSet.getValue(740);
      let mut mmcObj: Box<dyn Object> = propSet.getValue(740);
      // java ArrayList<MatrixColumn> mmcList = (ArrayList)mmcObj;
      let mut mmcList: Vec<MatrixColumn> = todo!("cast → Vec<()>");
      // java IOAssert.assertNotNull("mmcList should not be null", mmcList);
      IOAssert::assertNotNull__string_object(&"mmcList should not be null", Some(&mmcList));
      // java return mmcList;
      return mmcList;
    }

    // java private MathObjectProp getMathObjectProp(PLCMathFunction.MathStartNode mathStartNode)
    pub fn getMathObjectProp(&self, mathStartNode: &PLCMathFunction::MathStartNode) -> MathObjectProp {
      // java PropertySet propSet = mathStartNode.getPropertySet();
      let mut propSet: PropertySet = mathStartNode.getPropertySet();
      // java return (MathObjectProp)propSet.getValue(737);
      return (propSet.getValue(737)).downcast_ref::<MathObjectProp>().unwrap();
    }

    // java public void importMathArgmentStart(MathIOType mathIOType, MathArgmentType argmentType)
    pub fn importMathArgmentStart(&mut self, mathIOType: &MathIOType, argmentType: &MathArgmentType) {
      // java if (MathImporterHelper.isTripleMathElement(mathIOType))
      if MathImporterHelper::isTripleMathElement(&mathIOType) {
        // java int cp = this.importMathSeparator(mathIOType, argmentType);
        let mut cp: i32 = self.importMathSeparator(&mathIOType, &argmentType);
        // java this.mMathSepaCPStack.add(cp);
        self.mMathSepaCPStack.add(cp);
      }
      // java this.addDefaultArgSize();
      self.addDefaultArgSize();
    }

    // java private void addDefaultArgSize()
    pub fn addDefaultArgSize(&self) {
      // java int defaultArgSize = 0;
      let mut defaultArgSize: i32 = 0;
      // java ArrayList<Integer> argSizes = this.getArgumentSizeList();
      let mut argSizes: Vec<i32> = self.getArgumentSizeList();
      // java argSizes.add(defaultArgSize);
      argSizes.add(defaultArgSize);
    }

    // java public void importMathArgmentEnd(MutablePropertySet propSet, MathArgmentType argType, MathIOType mathIOType)
    pub fn importMathArgmentEnd(&mut self, propSet: &MutablePropertySet, argType: &MathArgmentType, mathIOType: &MathIOType) {
      // java Integer cp = this.getMathArgRunCp(argType, mathIOType);
      let mut cp: i32 = self.getMathArgRunCp(&argType, &mathIOType);
      // java this.setMathRunProperty(propSet, cp);
      self.setMathRunProperty(Some(&mut propSet), cp);
    }

    // java private int getMathArgRunCp(MathArgmentType argType, MathIOType mathIOType)
    pub fn getMathArgRunCp(&mut self, argType: &MathArgmentType, mathIOType: &MathIOType) -> i32 {
      // java return MathImporterHelper.isTripleMathElement(mathIOType) ? (Integer)this.mMathSepaCPStack.pop() : this.importMathSeparator(mathIOType, argType);
      return (if MathImporterHelper::isTripleMathElement(&mathIOType) { ((self.mMathSepaCPStack.pop()) as i32) } else { self.importMathSeparator(&mathIOType, &argType) });
    }

    // java public void importMatrixRowEnd()
    pub fn importMatrixRowEnd(&mut self) {
      // java int latestMatrixColCp = this.getLengthDocument() - 1;
      let mut latestMatrixColCp: i32 = self.getLengthDocument() - 1;
      // java this.convertControlChar(latestMatrixColCp, '@');
      self.convertControlChar(latestMatrixColCp, '@');
    }

    // java private void convertControlChar(int cp, char ch)
    pub fn convertControlChar(&self, cp: i32, ch: char) {
      // java ITextRope textRope = this.mDocument.getTextRope();
      let mut textRope: Box<dyn ITextRope> = self.mDocument.getTextRope();
      // java int fc = textRope.cp2fc(cp);
      let mut fc: i32 = textRope.cp2fc(cp);
      // java ITextPool textPool = textRope.getTextPool();
      let mut textPool: Box<dyn ITextPool> = textRope.getTextPool();
      // java IOAssert.assertNotNull("textPool should not be null", textPool);
      IOAssert::assertNotNull__string_object(&"textPool should not be null", Some(&textPool));
      // java textPool.set(fc, ch);
      textPool.set(fc, ch);
    }

    // java private int importMathSeparator(MathIOType mathType, MathArgmentType argmentType)
    pub fn importMathSeparator(&mut self, mathType: &MathIOType, argmentType: &MathArgmentType) -> i32 {
      // java char placeHolder = MathImporterHelper.getSepaPlaceHolder(mathType, argmentType);
      let mut placeHolder: char = MathImporterHelper::getSepaPlaceHolder(&mathType, &argmentType);
      // java return this.appendMathControlChar(placeHolder, PropertySet.empty);
      return self.appendMathControlChar(placeHolder, &PropertySet::empty());
    }

    // java private PLCMathFunction getPLCMathFunction()
    pub fn getPLCMathFunction(&mut self) -> PLCMathFunction {
      // java PLCMathFunction plcMath = this.mDocument.getPLCMathFunction();
      let mut plcMath: PLCMathFunction = self.mDocument.getPLCMathFunction();
      // java if (null == plcMath)
      if plcMath.is_none() {
        // java this.mDocument.createPLCMath();
        self.mDocument.createPLCMath();
        // java plcMath = this.mDocument.getPLCMathFunction();
        plcMath = self.mDocument.getPLCMathFunction();
      }
      // java return plcMath;
      return plcMath;
    }

    // java private void setMathRunProperty(MutablePropertySet propSet, int cp)
    pub fn setMathRunProperty(&self, propSet: Option<&mut MutablePropertySet>, cp: i32) {
      // java if (null != propSet && !propSet.isEmpty())
      if let Some(propSet) = propSet {
        if !propSet.isEmpty() {
          // java IChpxTable chpxTable = this.mDocument.getChpxTable();
          let mut chpxTable: Box<dyn IChpxTable> = self.mDocument.getChpxTable();
          // java IChpxTable.Entry chpxEntry = chpxTable.seek(cp);
          let mut chpxEntry: Box<dyn Entry> = chpxTable.seek(cp);
          // java propSet.add(chpxEntry.getProperty());
          propSet.add__mutablepropertyset(Some(&chpxEntry.getProperty()));
          // java chpxEntry.setProperty(propSet.freeze());
          chpxEntry.setProperty(propSet.freeze());
        }
      }
    }

    // java private int appendMathControlChar(char controlChar, PropertySet prop)
    pub fn appendMathControlChar(&mut self, controlChar: char, prop: &PropertySet) -> i32 {
      // java IOAssert.assertNotNull("mDocument should not be null", this.mDocument);
      IOAssert::assertNotNull__string_object(&"mDocument should not be null", Some(&self.mDocument));
      // java MutablePropertySet props = new MutablePropertySet(prop);
      let mut props: MutablePropertySet = MutablePropertySet::new(prop);
      // java props.setBoolean(104, true);
      props.setBoolean(104, true);
      // java return this.appendText(controlChar, props.freeze());
      return self.appendText(controlChar, &props.freeze());
    }

    // java protected int appendText(char ch, PropertySet props)
    pub fn appendText(&self, ch: char, props: &PropertySet) -> i32 {
      // java return CoreHelper.appendText((SubDocument)this.mDocument, ch, props);
      return CoreHelper::appendText__subdocument_char_propertyset(&(self.mDocument).downcast_ref::<SubDocument>().unwrap(), ch, &props);
    }

    // java private void updateMathArgSize(Attributes attributes)
    pub fn updateMathArgSize(&mut self, attributes: &Attributes) {
      // java Integer argSize = ConvertHelper.toInteger4Math(attributes);
      let mut argSize: i32 = ConvertHelper::toInteger4Math__attributes(&attributes);
      // java this.updateLatestMathArgSize(argSize);
      self.updateLatestMathArgSize(argSize);
    }

    // java private void updateLatestMathArgSize(Integer argSize)
    pub fn updateLatestMathArgSize(&self, argSize: i32) {
      // java if (null != argSize)
      if true {
        // java int coreArgSz = MathPropsConverter.convertOMathArgPrSz(argSize);
        let mut coreArgSz: i32 = MathPropsConverter::convertOMathArgPrSz(argSize);
        // java ArrayList<Integer> argSizes = this.getArgumentSizeList();
        let mut argSizes: Vec<i32> = self.getArgumentSizeList();
        // java int lastIndex = argSizes.size() - 1;
        let mut lastIndex: i32 = (argSizes.len() as i32) - 1;
        // java argSizes.set(lastIndex, coreArgSz);
        argSizes.set(lastIndex, coreArgSz);
      }
    }

    // java private ArrayList<Integer> getArgumentSizeList()
    pub fn getArgumentSizeList(&mut self) -> Vec<i32> {
      // java PLCMathFunction.MathStartNode mathStartNode = (PLCMathFunction.MathStartNode)this.mBeginMathNodeStack.peek();
      let mut mathStartNode: PLCMathFunction::MathStartNode = (self.mBeginMathNodeStack.peek()).downcast_ref::<PLCMathFunction::MathStartNode>().unwrap();
      // java PropertySet propSet = mathStartNode.getPropertySet();
      let mut propSet: PropertySet = mathStartNode.getPropertySet();
      // java Object argSzObj = propSet.getValue(738);
      let mut argSzObj: Box<dyn Object> = propSet.getValue(738);
      // java ArrayList<Integer> argSizes = (ArrayList)argSzObj;
      let mut argSizes: Vec<i32> = todo!("cast → Vec<()>");
      // java IOAssert.assertNotNull("argSizes should not be null", argSizes);
      IOAssert::assertNotNull__string_object(&"argSizes should not be null", Some(&argSizes));
      // java return argSizes;
      return argSizes;
    }

}
