// Java: cn.wps.moffice.writer.layout.base.line.TabHandler
#![allow(dead_code, non_snake_case)]
use crate::cn::wps::moffice::writer::data::value::TxTabStops::TxTabStops;
use crate::cn::wps::moffice::writer::layout::base::env::{DocumentEnv::DocumentEnv, LayoutState::LayoutState, TBD::TBD, ViewEnv::ViewEnv};
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
use crate::cn::wps::moffice::writer::layout::base::line::SubLineEnv::SubLineEnv;

// Java keeps references to these objects; the pointers alias the owners during synchronous line flow.
pub struct TabHandler {
    state: *mut LayoutState, subline: *mut SubLineEnv, doc: *mut DocumentEnv, view: *mut ViewEnv,
    cur_pos: i32, cur_jc: i32, user_tbd: bool, in_buffer: bool,
    cur_run: *mut Graphs, max_buffer_width: i32, rollback: bool,
}
impl Default for TabHandler { fn default() -> Self { Self::new() } }
impl TabHandler {
    pub fn new() -> Self { Self { state: std::ptr::null_mut(), subline: std::ptr::null_mut(), doc: std::ptr::null_mut(), view: std::ptr::null_mut(), cur_pos:0,cur_jc:0,user_tbd:false,in_buffer:false,cur_run:std::ptr::null_mut(),max_buffer_width:0,rollback:false } }
    // java public TabHandler(DocumentEnv docEnv, ViewEnv viewEnv)
    pub fn with_env(doc:&mut DocumentEnv,view:&mut ViewEnv)->Self {let mut s=Self::new();s.doc=doc;s.view=view;s}
    pub fn reuseClean(&mut self)->bool {self.state=std::ptr::null_mut();self.subline=std::ptr::null_mut();self.cur_run=std::ptr::null_mut();true}
    pub fn reuseInit(&mut self){self.cur_pos=0;self.cur_jc=0;self.user_tbd=false;self.in_buffer=false;self.max_buffer_width=0;}
    pub fn dispose(&mut self){self.reuseClean();self.doc=std::ptr::null_mut();self.view=std::ptr::null_mut();}
    // java public void setEnv(LayoutState env)
    pub fn setEnv(&mut self,state:&mut LayoutState){self.state=state;}
    // java public void setSubLineEnv(SubLineEnv subLineEnv)
    pub fn setSubLineEnv(&mut self,env:&mut SubLineEnv){self.subline=env;self.in_buffer=false;}
    pub fn needRollBackTab(&self)->bool{self.rollback}
    // java final void insertTbd(KList<TBD> tabList, TBD tbd)
    fn insertTbd(stops:&mut Vec<TBD>,tbd:TBD){let i=stops.partition_point(|x|x.tabStopPos<=tbd.tabStopPos);stops.insert(i,tbd);}
    // java final void removeTbd(KList<TBD> tabList, int pos)
    fn removeTbd(stops:&mut Vec<TBD>,pos:i32){if let Some(i)=stops.iter().position(|x|x.tabStopPos==pos){stops.remove(i);}}
    // java final void initUserTabs()
    fn initUserTabs(&mut self){
        let para=&mut unsafe{&mut *self.state}.paraEnv;
        if para.hasInitTBDs{return;}
        para.hasInitTBDs=true;self.cur_run=std::ptr::null_mut();
        let Some(props)=para.curParaPropEnv.as_ref() else{return;};
        let stops=para.tabStops.get_or_insert_with(Vec::new);
        // java TxTabStops txTabStops = (TxTabStops)paraEnv.curParaPropEnv.paraProperty.getValue(236)
        if let Some(tx)=props.paraProperty.as_ref().and_then(|p|p.getValue(236)).and_then(|v|v.as_ref().as_any().downcast_ref::<TxTabStops>()){
            for i in 0..tx.itbdAddMax.max(0) as usize {
                if let (Some(pos),Some(Some(value)))=(tx.rgdxaAdd.as_ref().and_then(|v|v.get(i)),tx.rgtbdAdd.as_ref().and_then(|v|v.get(i))){
                    let mut tbd=unsafe{&mut *self.doc}.pools.obtainTBD();tbd.tabStopPos=*pos;tbd.tabJC=value.tabJustify;tbd.tabFontCharType=value.frontCharType;Self::insertTbd(stops,tbd);
                }
            }
            for i in 0..tx.itbdDelMax.max(0) as usize{if let Some(pos)=tx.rgdxaDel.as_ref().and_then(|v|v.get(i)){Self::removeTbd(stops,*pos);}}
        }
        // java if (0 > firstLineIndent && !mDocEnv.noTabHangInd)
        if props.firstLineIndent<0 && !unsafe{&*self.doc}.noTabHangInd{let mut tbd=unsafe{&mut *self.doc}.pools.obtainTBD();tbd.tabJC=8;tbd.tabStopPos=props.leftIndent;Self::insertTbd(stops,tbd);}
    }
    // java final void getCurTbd(Graphs run)
    fn getCurTbd(&mut self,run:&mut Graphs){
        let env=unsafe{&*self.subline};let state=unsafe{&*self.state};let doc=unsafe{&*self.doc};
        let offset=env.layoutLeft-env.left;let tab_pos=run.getLeft()+offset;self.user_tbd=false;
        if let Some(stops)=state.paraEnv.tabStops.as_ref(){for (i,tbd) in stops.iter().enumerate(){
            if tbd.tabJC==4 || (run.isAutoNum() && doc.noNumTabHandInd && tbd.tabJC==8 && i+1!=stops.len()){continue;}
            if tab_pos<tbd.tabStopPos{self.cur_pos=tbd.tabStopPos-offset+run.getBdrWidth();self.cur_jc=tbd.tabJC;run.runType=match tbd.tabFontCharType{1=>21,2=>22,3=>23,4=>24,5=>25,_=>20};self.user_tbd=true;return;}
        }}
        // java pos = (int)(Math.floor(1.0F * tabPos / defTab) + 1.0F) * defTab
        let pos=if doc.defTab==0{tab_pos}else{((tab_pos as f32/doc.defTab as f32).floor() as i32+1)*doc.defTab};
        self.cur_jc=0;self.cur_pos=pos-offset;
    }
    // java public final boolean onAddTab(Graphs run)
    pub fn onAddTab(&mut self,run:&mut Graphs)->bool{
        self.rollback=false;self.initUserTabs();if self.in_buffer{self.commit();}self.getCurTbd(run);
        let env=unsafe{&*self.subline};let width=env.layoutRight-env.layoutLeft;let run_pos=run.getLeft();let page_mode=unsafe{&*self.view}.getViewMode()==0;
        if std::env::var("WORD_PARITY_AUTONUM_TRACE_CP").ok().and_then(|v|v.parse::<i32>().ok())==Some(unsafe{&*self.state}.cp){eprintln!("tab cp={} graph={} run_pos={} left={} layout_left={} right={} def={} cur={} jc={} user={} auto={} stops={:?}",unsafe{&*self.state}.cp,run.graphIndex,run_pos,env.left,env.layoutLeft,env.layoutRight,unsafe{&*self.doc}.defTab,self.cur_pos,self.cur_jc,self.user_tbd,run.isAutoNum(),unsafe{&*self.state}.paraEnv.tabStops.as_ref().map(|x|x.iter().map(|v|(v.tabStopPos,v.tabJC)).collect::<Vec<_>>()));}
        if self.user_tbd && page_mode && run_pos>width && !run.isAutoNum(){if matches!(self.cur_jc,0|6|8){run.setRight(self.cur_pos);}self.in_buffer=true;self.max_buffer_width=31680;self.cur_run=run;return true;}
        if self.cur_pos>width && (!self.user_tbd || !page_mode) && !run.isAutoNum(){run.mWidth=(width.min(self.cur_pos)-run_pos).max(0);return false;}
        self.cur_run=run;run.mWidth=0;
        match self.cur_jc{0|6|8=>{run.setRight(self.cur_pos);self.in_buffer=false;},1=>{self.max_buffer_width=(self.cur_pos-run_pos+self.cur_pos).min(width);self.in_buffer=true;},2=>{self.max_buffer_width=self.cur_pos;self.in_buffer=true;},3=>{self.max_buffer_width=width;self.in_buffer=true;},_=>{}}
        true
    }
    // java public int getRealTabWidth(Graphs run, int offset)
    pub fn getRealTabWidth(&mut self,run:&Graphs,offset:i32)->i32{self.initUserTabs();let state=unsafe{&*self.state};let doc=unsafe{&*self.doc};
        if let Some(stops)=state.paraEnv.tabStops.as_ref(){for (i,tbd) in stops.iter().enumerate(){if tbd.tabJC!=4 && offset<tbd.tabStopPos && !(run.isAutoNum() && doc.noNumTabHandInd && tbd.tabJC==8 && i+1!=stops.len()){return tbd.tabStopPos-offset;}}}
        if doc.defTab==0{0}else{((offset/doc.defTab)+1)*doc.defTab-offset}
    }
    // java public final boolean onAddRun(Graphs run, int compress)
    pub fn onAddRun(&mut self,run:&mut Graphs,compress:i32)->bool{if !self.in_buffer{return false;}if run.getRight()-compress>self.max_buffer_width || (!self.cur_run.is_null() && run.graphIndex<unsafe{(*self.cur_run).graphIndex}){self.in_buffer=false;}self.in_buffer}
    // java public final void commit()
    pub fn commit(&mut self){if !self.in_buffer{return;}self.in_buffer=false;let state=unsafe{&mut *self.state};let env=unsafe{&*self.subline};let tab=self.cur_run;if tab.is_null(){return;}
        let width=match (self.cur_jc,env.alignRightGraphs){(1,Some(right))=>{let half=(unsafe{(*right).getRight()}-unsafe{(*tab).getLeft()})/2;if self.cur_pos+half>env.layoutRight{self.cur_pos-unsafe{(*right).getRight()}}else{(self.cur_pos-half-unsafe{(*tab).getLeft()}).max(0)}},(2|3,Some(right))=>self.cur_pos-unsafe{(*right).getRight()},(_,Some(_))=>unsafe{(*tab).width()},(_,None)=>self.cur_pos-unsafe{(*tab).getLeft()}};
        if env.alignRightGraphs.is_some() && width<=0{return;}unsafe{(*tab).mWidth=width;}
        let mut pos=unsafe{(*tab).getRight()};let mut node=state.graphList.getNext(tab);while !node.is_null(){unsafe{(*node).offsetXTo(pos);pos=(*node).getRight();}if Some(node)==env.lastGraphNode{break;}node=state.graphList.getNext(node);}
    }
}
