// Java: import cn.wps.kfc.ooxfr.OxfrPackage
use super::OpcrPackage::OpcrPackage;
// Java: import cn.wps.kfc.xpxml.hlxml.HLXmlReader
use crate::kfc::xpxml::hlxml::HLXmlReader::HLXmlReader;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementStrHandler
use crate::kfc::xpxml::reader::DefaultElementStrHandler::DefaultElementStrHandler;
// Java: import cn.wps.kfc.xpxml.reader.ElementStrHandler
use crate::kfc::xpxml::reader::ElementStrHandler::ElementStrHandler;
// Java: import cn.wps.moffice.kfs.zip.ZipFile
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.util.ArrayList
// Java: import java.util.Iterator
// Java: import java.lang.Iterable
// Java: import cn.wps.kfc.opcr.OpcrRelationship
use super::OpcrRelationship::OpcrRelationship;
use super::ZipFile::ZipFile;
use super::OpcUtil::OpcUtil;

// java public final class OpcrRelationships extends DefaultElementStrHandler implements Iterable<OpcrRelationship>
#[derive(Default)]
pub struct OpcrRelationships {
    // java DefaultElementStrHandler super_
    pub(crate) super_: DefaultElementStrHandler,
    // java private ArrayList<OpcrRelationship> mRelses = new ArrayList();
    mRelses: Vec<OpcrRelationship>,
    // java private ZipFile mZipFile = null;
    mZipFile: Option<ZipFile>,
    // java private String mParentPartName;
    mParentPartName: String,
}

// java public final class OpcrRelationships extends DefaultElementStrHandler implements Iterable<OpcrRelationship>
impl OpcrRelationships {
    // java public  OpcrRelationships(ZipFile zipFile, String parentPartName)
    pub fn OpcrRelationships(zipFile: &ZipFile, parentPartName: &str) -> Result<Self, std::io::Error> {
      // java super();
      // java this.mZipFile = zipFile;
      let mut this = Self {
        super_: DefaultElementStrHandler::new(),
        mRelses: Vec::new(),
        mZipFile: Some(zipFile.clone()),
        mParentPartName: parentPartName.to_string(),
      };
      // java this.mParentPartName = parentPartName;
      // rust: no-stmt
      // java String relsesName = OpcUtil.getRelsesName(parentPartName);
      let relsesName = OpcUtil::getRelsesName(parentPartName);
      // java InputStream stream = OxfrPackage.getInputStream(zipFile, relsesName);
      let stream = OpcrPackage::getInputStream__name(zipFile, &relsesName)?;
      // java if (stream != null)
      if let Some(bytes) = stream {
        // java HLXmlReader.parser(stream, (ElementStrHandler)this);
        let parsed=std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let handler=RelationshipsHandler{zip:zipFile.clone(),parent:parentPartName.to_string(),rels:parsed.clone()};
        HLXmlReader::parser_str_handler(&mut std::io::Cursor::new(bytes),Box::new(handler))?;
        this.mRelses=std::mem::take(&mut *parsed.borrow_mut());
      }
      Ok(this)
    }

    // java public  OpcrRelationships(InputStream inputStream, String parentPartName)
    pub fn OpcrRelationships__inputstream(inputStream: Option<&dyn std::io::Read>, parentPartName: &str) -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java super();
      // java this.mParentPartName = parentPartName;
      let mut this = Self { super_: DefaultElementStrHandler::new(), mRelses: Default::default(), mZipFile: Default::default(), mParentPartName: parentPartName };
      // java HLXmlReader.parser(inputStream, (ElementStrHandler)this);
      HLXmlReader::parser__inputstream_elementstrhandler(inputStream, todo!("cast → Box<dyn ElementStrHandler>"));
      this
      */
    }

    // java public int size()
    pub fn size(&self) -> i32 {
      // java return this.mRelses.size();
      return (self.mRelses.len() as i32);
    }

    // java public OpcrRelationship getRelationship(int index)
    pub fn getRelationship(&self, index: i32) -> &OpcrRelationship {
      // java return (OpcrRelationship)this.mRelses.get(index);
      &self.mRelses[index as usize]
    }

    pub fn getRelationship_mut(&mut self, index: i32) -> &mut OpcrRelationship {
      &mut self.mRelses[index as usize]
    }

    // java public OpcrRelationship getRelationship(String rid)
    pub fn getRelationship__rid(&self, rid: &str) -> Option<&OpcrRelationship> {
      // java for (OpcrRelationship rels : this.mRelses)
      for rels in self.mRelses.iter() {
        // java if (rels.getId().equals(rid))
        if rels.getId().as_deref().expect("NullPointerException: relationship Id") == rid {
          // java return rels;
          return Some(rels);
        }
      }
      // java return null;
      return None;
    }

    // Native mutable reference view of the same Java getRelationship(String).
    pub fn getRelationship__rid_mut(&mut self,rid:Option<&str>)->Option<&mut OpcrRelationship>{
      for rels in self.mRelses.iter_mut(){if Some(rels.getId().as_deref().expect("NullPointerException: relationship Id"))==rid{return Some(rels);}}
      None
    }

    // java public OpcrRelationship getRelationshipByType(String type)
    pub fn getRelationshipByType(&self, r#type: &str) -> Option<&OpcrRelationship> {
      // java for (OpcrRelationship rels : this.mRelses)
      for rels in self.mRelses.iter() {
        // java if (rels.getType().equals(type))
        if rels.getType().as_deref().expect("NullPointerException: relationship Type") == r#type {
          // java return rels;
          return Some(rels);
        }
      }
      // java return null;
      return None;
    }

    // java public Iterator<OpcrRelationship> iterator()
    pub fn iterator(&self) -> std::slice::Iter<'_, OpcrRelationship> {
      // java return this.mRelses.iterator();
      self.mRelses.iter()
    }

    // java public ElementStrHandler enterSubElement(String subLabel)
    pub fn enterSubElement(&mut self, subLabel: &str) -> Option<&mut OpcrRelationship> {
      // java if (subLabel.endsWith("Relationship"))
      if subLabel.ends_with("Relationship") {
        // java OpcrRelationship rels = new OpcrRelationship(this.mZipFile, this, this.mParentPartName);
        let zip = self.mZipFile.clone().unwrap_or_default();
        let parent = self.mParentPartName.clone();
        let rels = OpcrRelationship::with_parent(&zip, &parent);
        // java this.mRelses.add(rels);
        self.mRelses.push(rels);
        // java return rels;
        return self.mRelses.last_mut();
      }
      // java else
      else {
        // java return null;
        return None;
      }
    }

}

// rust: Handler references share the original collection while the XML stack owns each child handler.
type Parsed=std::rc::Rc<std::cell::RefCell<Vec<OpcrRelationship>>>;
struct RelationshipsHandler{zip:ZipFile,parent:String,rels:Parsed}
impl ElementStrHandler for RelationshipsHandler{fn start_element(&mut self,_:&str){}fn end_element(&mut self,_:&str){}fn add_attr(&mut self,_:&str,_:&str){}fn characters(&mut self,_:&str){}
// java if(subLabel.endsWith("Relationship")){OpcrRelationship rels=new OpcrRelationship(this.mZipFile,this,this.mParentPartName);this.mRelses.add(rels);return rels;}else return null;
fn enter_sub_element(&mut self,s:&str)->Option<Box<dyn ElementStrHandler>>{if !s.ends_with("Relationship"){return None;}let index=self.rels.borrow().len();self.rels.borrow_mut().push(OpcrRelationship::with_parent(&self.zip,&self.parent));Some(Box::new(RelationshipHandler{rels:self.rels.clone(),index}))}}
struct RelationshipHandler{rels:Parsed,index:usize}impl ElementStrHandler for RelationshipHandler{fn start_element(&mut self,_:&str){}fn end_element(&mut self,_:&str){}fn add_attr(&mut self,n:&str,v:&str){self.rels.borrow_mut()[self.index].addAttr(n,v);}fn characters(&mut self,_:&str){}fn enter_sub_element(&mut self,_:&str)->Option<Box<dyn ElementStrHandler>>{None}}

impl std::ops::Deref for OpcrRelationships {
    type Target = DefaultElementStrHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for OpcrRelationships {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
