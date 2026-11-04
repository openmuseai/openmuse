#[allow(unused_imports)]
use super::Context::Context;
#[allow(unused_imports)]
use super::Definitions::Definitions;
#[allow(unused_imports)]
use super::InkMLWriter::InkMLWriter;
#[allow(unused_imports)]
use super::Trace::Trace;
use super::TraceGroup::TraceGroup;

// Java field: String TAG
pub(crate) const TAG: &str = "TraceView";

// Java: public class TraceView implements TraceDataElement, Cloneable
pub struct TraceView {
    // Java field: String id
    pub(crate) id: String,
    // Java field: String contextRef
    pub(crate) contextRef: String,
    // Java field: Context associatedContext
    pub(crate) associatedContext: Option<Context>,
    // Java field: String traceDataRef
    pub(crate) traceDataRef: String,
    // Java field: String from
    pub(crate) from: String,
    // Java field: String to
    pub(crate) to: String,
    // Java field: TraceGroup parentTraceGroup
    pub(crate) parentTraceGroup: Option<Box<TraceGroup>>,
    // Java field: ArrayList<TraceView> childTraceViewList
    pub(crate) childTraceViewList: Vec<TraceView>,
    // Java field: TraceDataElement selectedTree
    pub(crate) selectedTree: Option<Box<TraceGroup>>,
}

// Java: public class TraceView implements TraceDataElement, Cloneable
impl TraceView {
    // Java: TraceView()
    pub fn TraceView() -> Self {
      // Java: super();
      // Java: this.id = "";
      // Java: this.contextRef = "";
      // Java: this.traceDataRef = "";
      // Java: this.from = "";
      // Java: this.to = "";
      // Java: this.childTraceViewList = new ArrayList();
      Self {
          id: String::new(),
          contextRef: String::new(),
          associatedContext: None,
          traceDataRef: String::new(),
          from: String::new(),
          to: String::new(),
          parentTraceGroup: None,
          childTraceViewList: Vec::new(),
          selectedTree: None,
      }
    }

    // Java: TraceView(String traceDef)
    pub fn TraceView_with_def(trace_def: &str) -> Self {
      // Java: this();
      let mut tv = Self::TraceView();
      // Java: this.setTraceDataRef(traceDef);
      tv.setTraceDataRef(trace_def);
      tv
    }

    // Java: void setSelectedTree(Definitions definitions)
    pub fn setSelectedTree_defs(&mut self, _definitions: Option<&Definitions>) {
      // Java: TraceDataElement selectedTree = null;
      let selected_tree: Option<Box<TraceGroup>> = None;
      // Java: if (null != definitions)
      if _definitions.is_some()
      {
         // Java: TraceDataElement traceDataElement = definitions.getTraceDataRefElement(this.traceDataRef);
         // Java: KSLog.i(TAG, "The reffered traceData: " + traceDataElement.getInkElementType() + " - " + traceDataElement.getId());
         // Java: KSLog.i(TAG, "Select from:" + this.from + ", to:" + this.to);
         // Java: selectedTree = traceDataElement.getSelectedTraceDataByRange(this.from, this.to);
      }
      // Java: this.selectedTree = selectedTree;
      self.selectedTree = selected_tree;
    }

    // Java: TraceDataElement getSelectedTraceDataByRange(String from, String to)
    pub fn getSelectedTraceDataByRange(&self, _from: &str, _to: &str) -> Option<i32> {
      // Java: if (this.selectedTree != null)
      if self.selectedTree.is_some()
      {
         // Java: return this.selectedTree.getSelectedTraceDataByRange(from, to);
         return None;
      }
      // Java: else
      else
      {
         // Java: throw new InkMLException("Error: Call to getSelectedTraceDataByRange(String,String) failed.Reason: the traceView selection sub-tree is not available");
         panic!("Error: Call to getSelectedTraceDataByRange(String,String) failed.Reason: the traceView selection sub-tree is not available");
      }
    }

    // Java: TraceDataElement getSelectedTree()
    pub fn getSelectedTree(&self) -> Option<&TraceGroup> {
      // Java: return this.selectedTree;
      self.selectedTree.as_deref()
    }

    // Java: ArrayList<Trace> getTraceList()
    pub fn getTraceList(&self) -> Vec<Trace> {
      // Java: if (this.selectedTree == null)
      if self.selectedTree.is_none()
      {
         // Java: throw new InkMLException("Error: Call to TraceView.getTraceList() failed.Reason: the traceView selection sub-tree is not available");
         panic!("Error: Call to TraceView.getTraceList() failed.Reason: the traceView selection sub-tree is not available");
      }
      // Java: else
      // Java: ArrayList<Trace> traceList = new ArrayList();
      let trace_list: Vec<Trace> = Vec::new();
      // Java: if ("TraceGroup".equals(this.selectedTree.getInkElementType()))
      if false
      {
         // Java: traceList.addAll(((TraceGroup)this.selectedTree).getTraceList());
      }
      // Java: else
      else
      {
         // Java: traceList.add((Trace)this.selectedTree);
      }
      // Java: return traceList;
      trace_list
    }

    // Java: String getId()
    pub fn getId(&self) -> String {
      // Java: return this.id == null ? "" : this.id;
      self.id.clone()
    }

    // Java: String getInkElementType()
    pub fn getInkElementType(&self) -> String {
      // Java: return "TraceView";
      "TraceView".to_string()
    }

    // Java: Context getAssociatedContext()
    pub fn getAssociatedContext(&self) -> Option<&Context> {
      // Java: return this.associatedContext;
      self.associatedContext.as_ref()
    }

    // Java: void setAssociatedContext(Context associatedContext)
    pub fn setAssociatedContext(&mut self, associated_context: Option<Context>) {
      // Java: this.associatedContext = associatedContext;
      self.associatedContext = associated_context;
    }

    // Java: void setchildTraceViewList(ArrayList<TraceView> newTraceViewList)
    pub fn setchildTraceViewList(&mut self, new_trace_view_list: Vec<TraceView>) {
      // Java: this.childTraceViewList = newTraceViewList;
      self.childTraceViewList = new_trace_view_list;
    }

    // Java: TraceView getTraceViewAt(int index)
    pub fn getTraceViewAt(&self, index: i32) -> &TraceView {
      // Java: if (index >= 0 && index < this.childTraceViewList.size())
      if index >= 0 && (index as usize) < self.childTraceViewList.len()
      {
         // Java: return (TraceView)this.childTraceViewList.get(index);
         return &self.childTraceViewList[index as usize];
      }
      // Java: else
      else
      {
         // Java: throw new InkMLException("getTraceViewAt(int) called with outofBound index = " + index);
         panic!("getTraceViewAt(int) called with outofBound index = {}", index);
      }
    }

    // Java: void addToChildTraceViewList(TraceView traceView)
    pub fn addToChildTraceViewList(&mut self, trace_view: TraceView) {
      // Java: this.childTraceViewList.add(traceView);
      self.childTraceViewList.push(trace_view);
    }

    // Java: void removeTraceViewAt(int index)
    pub fn removeTraceViewAt(&mut self, index: i32) {
      // Java: if (index >= 0 && index < this.childTraceViewList.size())
      if index >= 0 && (index as usize) < self.childTraceViewList.len()
      {
         // Java: this.childTraceViewList.remove(index);
         self.childTraceViewList.remove(index as usize);
      }
      // Java: else
      else
      {
         // Java: throw new InkMLException("removeTraceViewAt(int) called with outofBound index = " + index);
         panic!("removeTraceViewAt(int) called with outofBound index = {}", index);
      }
    }

    // Java: void removeTraceViewAtFirst(TraceView data)
    pub fn removeTraceViewAtFirst(&mut self, _data: &TraceView) {
      // Java: if (this.childTraceViewList.size() > 0)
      if !self.childTraceViewList.is_empty()
      {
         // Java: int firstIndex = this.childTraceViewList.indexOf(data);
         // Java: this.childTraceViewList.remove(firstIndex);
         self.childTraceViewList.remove(0);
      }
      // Java: else
      else
      {
         // Java: throw new InkMLException("removeTraceViewAt(TraceView) called a empty childTraceViewList");
         panic!("removeTraceViewAt(TraceView) called a empty childTraceViewList");
      }
    }

    // Java: void removeTraceViewAtLast(TraceView data)
    pub fn removeTraceViewAtLast(&mut self, _data: &TraceView) {
      // Java: if (this.childTraceViewList.size() > 0)
      if !self.childTraceViewList.is_empty()
      {
         // Java: int firstIndex = this.childTraceViewList.lastIndexOf(data);
         // Java: this.childTraceViewList.remove(firstIndex);
         let last = self.childTraceViewList.len() - 1;
         self.childTraceViewList.remove(last);
      }
      // Java: else
      else
      {
         // Java: throw new InkMLException("removeTraceViewAt(TraceView) called a empty childTraceViewList");
         panic!("removeTraceViewAt(TraceView) called a empty childTraceViewList");
      }
    }

    // Java: String toInkML(Definitions definitions)
    pub fn toInkML_defs(&self, _definitions: Option<&Definitions>) -> String {
      // Java: if (this.selectedTree != null)
      if self.selectedTree.is_some()
      {
         // Java: return this.selectedTree.toInkML();
         return String::new();
      }
      // Java: else
      else
      {
         // Java: KSLog.i(TAG, "TraceView.toInkML method: Could not complete the operation.Reason: The selcted tree is NULL.");
         // Java: return "";
         return String::new();
      }
    }

    // Java: void writeXML(InkMLWriter writer)
    pub fn writeXML(&self, _writer: &mut InkMLWriter) {
      // Java: if (this.selectedTree != null)
      if self.selectedTree.is_some()
      {
         // Java: this.selectedTree.writeXML(writer);
      }
      // Java: else
      else
      {
         // Java: KSLog.i(TAG, "TraceView.toInkML method: Could not complete the operation.Reason: The selcted tree is NULL.");
      }
    }

    // Java: void setId(String id)
    pub fn setId(&mut self, id: &str) {
      // Java: this.id = id;
      self.id = id.to_string();
    }

    // Java: void setContextRef(String contextRef)
    pub fn setContextRef(&mut self, context_ref: &str) {
      // Java: this.contextRef = contextRef;
      self.contextRef = context_ref.to_string();
    }

    // Java: void setSelectedTree(TraceDataElement selectedTree)
    pub fn setSelectedTree_element(&mut self, selected_tree: Option<Box<TraceGroup>>) {
      // Java: this.selectedTree = selectedTree;
      self.selectedTree = selected_tree;
    }

    // Java: void setFromAttribute(String from)
    pub fn setFromAttribute(&mut self, from: &str) {
      // Java: this.from = from;
      self.from = from.to_string();
    }

    // Java: void setToAttribute(String to)
    pub fn setToAttribute(&mut self, to: &str) {
      // Java: this.to = to;
      self.to = to.to_string();
    }

    // Java: void setTraceDataRef(String traceDataRef)
    pub fn setTraceDataRef(&mut self, trace_data_ref: &str) {
      // Java: this.traceDataRef = traceDataRef;
      self.traceDataRef = trace_data_ref.to_string();
    }

    // Java: void resolveContext(Definitions definitions)
    pub fn resolveContext(&mut self, _definitions: Option<&Definitions>) {
      // Java: if (null != definitions && !"".equals(this.contextRef))
      if _definitions.is_some() && !self.contextRef.is_empty()
      {
         // Java: Context context = null;
         let _context: Option<Context> = None;
         // Java: try
         {
            // Java: context = definitions.getContextRefElement(this.contextRef);
         }
         // Java: catch (InkMLException e)
         {
            // Java: KSLog.i(TAG, "Error in TraceView::resolveContext.\nMessage: " + e.getMessage());
         }
         // Java: if (null != context)
         if false
         {
            // Java: this.setAssociatedContext(context);
            // Java: if (this.selectedTree == null)
            if self.selectedTree.is_none()
            {
               // Java: throw new InkMLException("TraceView.resolveContext() failed. Reason, there is no selected sub-tree exist");
               panic!("TraceView.resolveContext() failed.");
            }
            // Java: this.selectedTree.setAssociatedContext(this.getAssociatedContext());
         }
      }
    }

    // Java: void initChildViewList()
    pub fn initChildViewList(&mut self) {
      // Java: this.childTraceViewList = new ArrayList();
      self.childTraceViewList = Vec::new();
    }

    // Java: void processChildren(Definitions definitions)
    pub fn processChildren(&mut self, _definitions: Option<&Definitions>) {
      // Java: this.selectedTree = new TraceGroup();
      self.selectedTree = Some(Box::new(TraceGroup::TraceGroup()));
      // Java: Context traceViewAssoContext = null;
      let _trace_view_asso_context: Option<Context> = None;
      // Java: if (null != definitions && !"".equals(this.contextRef))
      if _definitions.is_some() && !self.contextRef.is_empty()
      {
         // Java: try
         {
            // Java: traceViewAssoContext = definitions.getContextRefElement(this.contextRef);
            // Java: this.selectedTree.setAssociatedContext(traceViewAssoContext);
         }
         // Java: catch (InkMLException e)
         {
            // Java: KSLog.i(TAG, "Error in TraceView::processChildren.\nMessage: " + e.getMessage());
         }
      }
      // Java: for(int i = 0; i < this.childTraceViewList.size(); ++i)
      for _i in 0..self.childTraceViewList.len()
      {
         // Java: TraceDataElement selectedSubtree = ((TraceView)this.childTraceViewList.get(i)).getSelectedTree();
         // Java: if (null != traceViewAssoContext)
         if false
         {
            // Java: selectedSubtree.setAssociatedContext(traceViewAssoContext);
         }
         // Java: ((TraceGroup)this.selectedTree).addToTraceData(selectedSubtree);
      }
    }

    // Java: void printSelectedTree()
    pub fn printSelectedTree(&self) {
      // Java: if (null == this.selectedTree)
      if self.selectedTree.is_none()
      {
         // Java: KSLog.i(TAG, "SelectedTree is NULL");
      }
      // Java: else if ("Trace".equals(this.selectedTree.getInkElementType()))
      else if false
      {
         // Java: ((Trace)this.selectedTree).printTrace();
      }
      // Java: else
      else
      {
         // Java: ((TraceGroup)this.selectedTree).printTraceGroup();
      }
    }

    // Java: void setParentTraceGroup(TraceGroup paramTraceGroup)
    pub fn setParentTraceGroup(&mut self, param_trace_group: Option<Box<TraceGroup>>) {
      // Java: this.parentTraceGroup = paramTraceGroup;
      self.parentTraceGroup = param_trace_group;
    }

    // Java: TraceGroup getParentTraceGroup()
    pub fn getParentTraceGroup(&self) -> Option<&TraceGroup> {
      // Java: return this.parentTraceGroup;
      self.parentTraceGroup.as_deref()
    }

    // Java: String toInkML()
    pub fn toInkML(&self) -> Option<String> {
      // Java: return null;
      None
    }

    // Java: TraceView clone()
    pub fn clone_tv(&self) -> TraceView {
      // Java: TraceView ret = new TraceView();
      let mut ret = TraceView::TraceView();
      // Java: if (null != this.id)
      {
         // Java: ret.id = new String(this.id);
         ret.id = self.id.clone();
      }
      // Java: if (null != this.from)
      {
         // Java: ret.from = new String(this.from);
         ret.from = self.from.clone();
      }
      // Java: if (null != this.contextRef)
      {
         // Java: ret.contextRef = new String(this.contextRef);
         ret.contextRef = self.contextRef.clone();
      }
      // Java: if (null != this.traceDataRef)
      {
         // Java: ret.traceDataRef = new String(this.traceDataRef);
         ret.traceDataRef = self.traceDataRef.clone();
      }
      // Java: if (null != this.to)
      {
         // Java: ret.to = new String(this.to);
         ret.to = self.to.clone();
      }
      // Java: if (null != this.associatedContext)
      {
         // Java: ret.associatedContext = this.associatedContext.clone();
      }
      // Java: ret.childTraceViewList = this.cloneChildTraceViewList();
      ret.childTraceViewList = self.cloneChildTraceViewList();
      // Java: if (null != this.parentTraceGroup)
      {
         // Java: ret.parentTraceGroup = this.parentTraceGroup.clone();
      }
      // Java: return ret;
      ret
    }

    // Java: ArrayList<TraceView> cloneChildTraceViewList()
    fn cloneChildTraceViewList(&self) -> Vec<TraceView> {
      // Java: if (null == this.childTraceViewList)
      if self.childTraceViewList.is_empty() {
         // Java: return null;
         return Vec::new();
      }
      // Java: else
      // Java: ArrayList<TraceView> ret = new ArrayList();
      let mut ret = Vec::new();
      // Java: int size = this.childTraceViewList.size();
      let size = self.childTraceViewList.len();
      // Java: for(int i = 0; i < size; ++i)
      for i in 0..size {
         // Java: TraceView tv = (TraceView)this.childTraceViewList.get(i);
         let tv = &self.childTraceViewList[i];
         // Java: ret.add(tv.clone());
         ret.push(tv.clone_tv());
      }
      // Java: return ret;
      ret
    }

}
