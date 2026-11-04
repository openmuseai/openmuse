// Java: import cn.wps.moffice.drawing.Shape
use drawing::cn::wps::moffice::drawing::Shape::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.KRange
use crate::cn::wps::moffice::writer::core::KRange::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import cn.wps.moffice.writer.core.io.IOListener
use crate::cn::wps::moffice::writer::core::io::IOListener::*;
// Java: import cn.wps.moffice.writer.core.shape.KShapeRange
use crate::cn::wps::moffice::writer::core::shape::KShapeRange::*;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::*;
// Java: import cn.wps.moffice.writer.data.table.TableInfo
use crate::cn::wps::moffice::writer::data::table::TableInfo::*;
// Java: import java.util.Vector
use stubs::java::util::Vector::*;
// Java: import cn.wps.moffice.writer.insert.ShapeIdMapping
use crate::cn::wps::moffice::writer::insert::ShapeIdMapping::*;

// java public class InsertEnv
#[derive(Default)]
pub struct InsertEnv {
    // java public TextDocument mDestTextDocument;
    pub(crate) mDestTextDocument: Option<TextDocument>,
    // java public TextDocument mSrcTextDocument;
    pub(crate) mSrcTextDocument: Option<TextDocument>,
    // java public Document mDestDocument;
    pub(crate) mDestDocument: Option<Document>,
    // java public Document mSrcDocument;
    pub(crate) mSrcDocument: Option<Document>,
    // java public KRange mInsertRange;
    pub(crate) mInsertRange: Option<KRange>,
    // java public KShapeRange mShapeRange;
    pub(crate) mShapeRange: Option<KShapeRange>,
    // java public IChpxTable.Entry mChpx;
    pub(crate) mChpx: Option<IChpxTable::Entry>,
    // java public int mFieldDepth;
    pub(crate) mFieldDepth: i32,
    // java public String mInsertFile;
    pub(crate) mInsertFile: Option<String>,
    // java public int mTableDepth;
    pub(crate) mTableDepth: i32,
    // java public TableInfo mTableInfo;
    pub(crate) mTableInfo: Option<TableInfo>,
    // java public boolean mJustText;
    pub(crate) mJustText: bool,
    // java public ShapeIdMapping mShapeIDMapping;
    pub(crate) mShapeIDMapping: Option<ShapeIdMapping>,
    // java public Vector<Shape> mInsertShaps;
    pub(crate) mInsertShaps: Option<Vec<Shape>>,
    // java public String mInsertFilePassword;
    pub(crate) mInsertFilePassword: Option<String>,
    // java public IOListener mIoListener;
    pub(crate) mIoListener: Option<Box<dyn IOListener>>,
}

// java public class InsertEnv
impl InsertEnv {
    // java public  InsertEnv()
    pub fn new() -> Self {
      // java this.init();
      this.init();
    }

    // java public  InsertEnv(InsertEnv env)
    pub fn new__insertenv(env: &InsertEnv) -> Self {
      // java this.init();
      this.init();
      // java this.mDestTextDocument = env.mDestTextDocument;
      let mut this = Self { mDestTextDocument: env.mDestTextDocument, mSrcTextDocument: env.mSrcTextDocument, mDestDocument: Default::default(), mSrcDocument: Default::default(), mInsertRange: Default::default(), mShapeRange: Default::default(), mChpx: Default::default(), mFieldDepth: Default::default(), mInsertFile: Default::default(), mTableDepth: Default::default(), mTableInfo: Default::default(), mJustText: Default::default(), mShapeIDMapping: Default::default(), mInsertShaps: Default::default(), mInsertFilePassword: Default::default(), mIoListener: Default::default() };
      // java this.mSrcTextDocument = env.mSrcTextDocument;
      this
    }

    // java private void init()
    pub fn init(&mut self) {
      // java this.mFieldDepth = 0;
      self.mFieldDepth = 0;
      // java this.mJustText = false;
      self.mJustText = false;
      // java this.mTableDepth = 0;
      self.mTableDepth = 0;
      // java this.mShapeIDMapping = new ShapeIdMapping();
      self.mShapeIDMapping = ShapeIdMapping::new();
      // java this.mInsertShaps = new Vector();
      self.mInsertShaps = Vec::new();
      // java this.mTableInfo = null;
      self.mTableInfo = None;
    }

}
