use crate::java_runtime::{Rc,RefCell,Weak};
use super::KmoRect::KmoRect;
use std::collections::VecDeque;
type Node<T>=Rc<RefCell<KmoNode<T>>>;
// java class KmoNode
struct KmoNode<T>{data:Option<Rc<T>>,rc:KmoRect,parent:Weak<RefCell<KmoNode<T>>>,children:Vec<Node<T>>}
impl<T> KmoNode<T>{fn new()->Node<T>{Rc::new(RefCell::new(Self{data:None,rc:KmoRect::KmoRect(),parent:Weak::new(),children:vec![]}))}}
// java public class KmoTreeSpatialIndex<KmoDataType> implements KmoSpatialIndex<KmoDataType>
pub struct KmoTreeSpatialIndex<T>{root:RefCell<Node<T>>}
impl<T> KmoTreeSpatialIndex<T>{
// java public KmoTreeSpatialIndex() {
// java this.root.add(new KmoNode());
// java this.root.add(new KmoNode());
// java }
pub fn new()->Self{let root=KmoNode::new();Self::add(&root,KmoNode::new());Self::add(&root,KmoNode::new());Self{root:RefCell::new(root)}}
// java public void add(KmoTreeSpatialIndex<KmoDataType>.KmoNode node) {
// java if (this.children.isEmpty()) {
// java this.rc.copy(node.rc);
// java } else {
// java this.rc.combine(node.rc);
// java }
// java
// java this.children.add(node);
// java node.parent = this;
// java }
fn add(parent:&Node<T>,node:Node<T>){let rc=node.borrow().rc.clone();let mut p=parent.borrow_mut();if p.children.is_empty(){p.rc=rc;}else{p.rc.combine(&rc);}p.children.push(node.clone());node.borrow_mut().parent=Rc::downgrade(parent);}
// java public void rebuildMBR() {
// java if (this.children.isEmpty()) {
// java this.rc.setZero();
// java } else {
// java this.rc.copy(((KmoNode)this.children.get(0)).rc);
// java
// java for(KmoTreeSpatialIndex<KmoDataType>.KmoNode i : this.children) {
// java this.rc.combine(i.rc);
// java }
// java }
// java
// java }
fn rebuild(n:&Node<T>){let children=n.borrow().children.clone();let mut rc=children.first().map_or_else(KmoRect::KmoRect,|c|c.borrow().rc.clone());for c in children{rc.combine(&c.borrow().rc);}n.borrow_mut().rc=rc;}
// java public boolean isLeaf() {
// java if (this.isRoot()) {
// java return false;
// java } else if (this.children.size() == 0) {
// java return true;
// java } else {
// java return ((KmoNode)this.children.get(0)).data != null;
// java }
// java }
fn is_leaf(n:&Node<T>)->bool{let n=n.borrow();if n.parent.upgrade().is_none(){false}else{n.children.first().map_or(true,|c|c.borrow().data.is_some())}}
// java public void lookup(KmoRect rect, Collection<KmoDataType> out) {
// java if (this.root.rc.intersects(rect)) {
// java if (this.root.data != null) {
// java out.add(this.root.data);
// java } else if (this.root.children.size() != 0) {
// java LinkedList<KmoTreeSpatialIndex<KmoDataType>.KmoNode> s = new LinkedList();
// java s.addLast(this.root);
// java
// java while(s.size() != 0) {
// java KmoTreeSpatialIndex<KmoDataType>.KmoNode n = (KmoNode)s.removeFirst();
// java int size = n.children.size();
// java
// java for(int i = 0; i < size; ++i) {
// java KmoTreeSpatialIndex<KmoDataType>.KmoNode c = (KmoNode)n.children.get(i);
// java if (c.rc.intersects(rect)) {
// java if (c.data != null) {
// java out.add(c.data);
// java } else if (c.children.size() != 0) {
// java s.addLast(c);
// java }
// java }
// java }
// java }
// java
// java }
// java }
// java }
pub fn lookup(&self,rect:&KmoRect,out:&mut Vec<Rc<T>>){let root=self.root.borrow().clone();if !root.borrow().rc.intersects(rect){return;}if let Some(d)=root.borrow().data.clone(){out.push(d);return;}let mut q=VecDeque::from([root]);while let Some(n)=q.pop_front(){for c in n.borrow().children.clone(){if c.borrow().rc.intersects(rect){let d=c.borrow().data.clone();if let Some(d)=d{out.push(d);}else if !c.borrow().children.is_empty(){q.push_back(c);}}}}}
// java public KmoDataType lookup_first(KmoRect rect) {
// java if (!this.root.rc.intersects(rect)) {
// java return null;
// java } else if (this.root.data != null) {
// java return this.root.data;
// java } else if (this.root.children.size() == 0) {
// java return null;
// java } else {
// java LinkedList<KmoTreeSpatialIndex<KmoDataType>.KmoNode> s = new LinkedList();
// java s.addLast(this.root);
// java
// java while(s.size() != 0) {
// java KmoTreeSpatialIndex<KmoDataType>.KmoNode n = (KmoNode)s.removeFirst();
// java int size = n.children.size();
// java
// java for(int i = 0; i < size; ++i) {
// java KmoTreeSpatialIndex<KmoDataType>.KmoNode c = (KmoNode)n.children.get(i);
// java if (c.rc.intersects(rect)) {
// java if (c.data != null) {
// java return c.data;
// java }
// java
// java if (c.children.size() != 0) {
// java s.addLast(c);
// java }
// java }
// java }
// java }
// java
// java return null;
// java }
// java }
pub fn lookup_first(&self,rect:&KmoRect)->Option<Rc<T>>{let mut out=vec![];self.lookup(rect,&mut out);out.into_iter().next()}
// java private KmoTreeSpatialIndex<KmoDataType>.KmoNode helper_findLeastEnlargement(KmoTreeSpatialIndex<KmoDataType>.KmoNode node, KmoRect rect) {
// java KmoTreeSpatialIndex<KmoDataType>.KmoNode minNode = null;
// java int minArea = Integer.MAX_VALUE;
// java KmoRect rc = SharedObjects.kmoRectPool.borrowObject();
// java
// java for(KmoTreeSpatialIndex<KmoDataType>.KmoNode i : node.children) {
// java int area = rc.copy(rect).combine(i.rc).area();
// java if (area < minArea) {
// java minArea = area;
// java minNode = i;
// java }
// java }
// java
// java SharedObjects.kmoRectPool.returnObject(rc);
// java return minNode;
// java }
fn least(n:&Node<T>,rect:&KmoRect)->Node<T>{let mut min=None;let mut area=i32::MAX;for c in n.borrow().children.clone(){let mut rc=rect.clone();rc.combine(&c.borrow().rc);let a=rc.area();if a<area{area=a;min=Some(c);}}min.expect("NullPointerException: minNode")}
// java private KmoPair<KmoTreeSpatialIndex<KmoDataType>.KmoNode, KmoTreeSpatialIndex<KmoDataType>.KmoNode> helper_pickSeeds(List<KmoTreeSpatialIndex<KmoDataType>.KmoNode> nodeLst) {
// java int maxD = Integer.MIN_VALUE;
// java KmoTreeSpatialIndex<KmoDataType>.KmoNode resultF = null;
// java KmoTreeSpatialIndex<KmoDataType>.KmoNode resultS = null;
// java KmoRect rcd = SharedObjects.kmoRectPool.borrowObject();
// java int size = nodeLst.size();
// java
// java for(int i = 0; i < size - 1; ++i) {
// java KmoRect rci = ((KmoNode)nodeLst.get(i)).rc;
// java
// java for(int j = i + 1; j < size; ++j) {
// java KmoRect rcj = ((KmoNode)nodeLst.get(j)).rc;
// java rcd.copy(rci).combine(rcj);
// java int d = rcd.area() - (rci.area() + rcj.area());
// java if (d > maxD) {
// java maxD = d;
// java resultF = (KmoNode)nodeLst.get(i);
// java resultS = (KmoNode)nodeLst.get(j);
// java }
// java }
// java }
// java
// java rcd = SharedObjects.kmoRectPool.returnObject(rcd);
// java nodeLst.remove(resultF);
// java nodeLst.remove(resultS);
// java return KmoPair.<KmoTreeSpatialIndex<KmoDataType>.KmoNode, KmoTreeSpatialIndex<KmoDataType>.KmoNode>of(resultF, resultS);
// java }
fn seeds(q:&mut Vec<Node<T>>)->(Node<T>,Node<T>){let(mut max,mut fi,mut si)=(i32::MIN,None,None);for i in 0..q.len()-1{let a=q[i].borrow().rc.clone();for j in i+1..q.len(){let b=q[j].borrow().rc.clone();let mut rc=a.clone();rc.combine(&b);let d=rc.area().wrapping_sub(a.area().wrapping_add(b.area()));if d>max{max=d;fi=Some(i);si=Some(j);}}}let s=q.remove(si.unwrap());let f=q.remove(fi.unwrap());(f,s)}
// java private KmoTreeSpatialIndex<KmoDataType>.KmoNode helper_pickNext(List<KmoTreeSpatialIndex<KmoDataType>.KmoNode> nodeLst, KmoTreeSpatialIndex<KmoDataType>.KmoNode first, KmoTreeSpatialIndex<KmoDataType>.KmoNode second) {
// java int maxD = Integer.MIN_VALUE;
// java KmoTreeSpatialIndex<KmoDataType>.KmoNode maxNode = null;
// java KmoRect rcF = first.rc;
// java KmoRect rcS = second.rc;
// java int areaF = rcF.area();
// java int areaS = rcS.area();
// java KmoRect rc = SharedObjects.kmoRectPool.borrowObject();
// java
// java for(KmoTreeSpatialIndex<KmoDataType>.KmoNode i : nodeLst) {
// java rc.copy(rcF).combine(i.rc);
// java int d1 = rc.area() - areaF;
// java rc.copy(rcS).combine(i.rc);
// java int d2 = rc.area() - areaS;
// java int d = Math.abs(d1 - d2);
// java if (d >= maxD) {
// java maxD = d;
// java maxNode = i;
// java }
// java }
// java
// java rc = SharedObjects.kmoRectPool.returnObject(rc);
// java nodeLst.remove(maxNode);
// java return maxNode;
// java }
fn next(q:&mut Vec<Node<T>>,f:&Node<T>,s:&Node<T>)->Node<T>{let(mut max,mut idx)=(i32::MIN,None);let a=f.borrow().rc.clone();let b=s.borrow().rc.clone();for (i,n) in q.iter().enumerate(){let r=n.borrow().rc.clone();let mut u=a.clone();u.combine(&r);let d1=u.area().wrapping_sub(a.area());u=b.clone();u.combine(&r);let d2=u.area().wrapping_sub(b.area());let d=d1.wrapping_sub(d2).wrapping_abs();if d>=max{max=d;idx=Some(i);}}q.remove(idx.unwrap())}
// java private KmoPair<KmoTreeSpatialIndex<KmoDataType>.KmoNode, KmoTreeSpatialIndex<KmoDataType>.KmoNode> helper_splitNode(KmoTreeSpatialIndex<KmoDataType>.KmoNode node) {
// java KmoPair<KmoTreeSpatialIndex<KmoDataType>.KmoNode, KmoTreeSpatialIndex<KmoDataType>.KmoNode> result = KmoPair.<KmoTreeSpatialIndex<KmoDataType>.KmoNode, KmoTreeSpatialIndex<KmoDataType>.KmoNode>of(node, new KmoNode());
// java List<KmoTreeSpatialIndex<KmoDataType>.KmoNode> Q = new ArrayList(node.children);
// java node.clear();
// java KmoPair<KmoTreeSpatialIndex<KmoDataType>.KmoNode, KmoTreeSpatialIndex<KmoDataType>.KmoNode> pair = this.helper_pickSeeds(Q);
// java ((KmoNode)result.first).add((KmoNode)pair.first);
// java ((KmoNode)result.second).add((KmoNode)pair.second);
// java KmoRect rc = new KmoRect();
// java
// java while(Q.size() > 0) {
// java KmoTreeSpatialIndex<KmoDataType>.KmoNode next = this.helper_pickNext(Q, (KmoNode)result.first, (KmoNode)result.second);
// java int d1 = rc.copy(((KmoNode)result.first).rc).combine(next.rc).area() - ((KmoNode)result.first).rc.area();
// java int d2 = rc.copy(((KmoNode)result.second).rc).combine(next.rc).area() - ((KmoNode)result.second).rc.area();
// java if (d1 > d2) {
// java ((KmoNode)result.first).add(next);
// java } else if (d1 < d2) {
// java ((KmoNode)result.second).add(next);
// java } else if (((KmoNode)result.first).rc.area() < ((KmoNode)result.second).rc.area()) {
// java ((KmoNode)result.first).add(next);
// java } else if (((KmoNode)result.first).rc.area() > ((KmoNode)result.second).rc.area()) {
// java ((KmoNode)result.second).add(next);
// java } else if (((KmoNode)result.first).children.size() < ((KmoNode)result.second).children.size()) {
// java ((KmoNode)result.first).add(next);
// java } else {
// java ((KmoNode)result.second).add(next);
// java }
// java }
// java
// java return result;
// java }
fn split(n:&Node<T>)->(Node<T>,Node<T>){let nn=KmoNode::new();let mut q=std::mem::take(&mut n.borrow_mut().children);Self::rebuild(n);let(f,s)=Self::seeds(&mut q);Self::add(n,f);Self::add(&nn,s);while !q.is_empty(){let next=Self::next(&mut q,n,&nn);let a=n.borrow().rc.clone();let b=nn.borrow().rc.clone();let mut u=a.clone();u.combine(&next.borrow().rc);let d1=u.area().wrapping_sub(a.area());u=b.clone();u.combine(&next.borrow().rc);let d2=u.area().wrapping_sub(b.area());let dest=if d1>d2{n}else if d1<d2{&nn}else if a.area()<b.area(){n}else if a.area()>b.area(){&nn}else if n.borrow().children.len()<nn.borrow().children.len(){n}else{&nn};Self::add(dest,next);}(n.clone(),nn)}
// java private void helper_adjustTree(KmoPair<KmoTreeSpatialIndex<KmoDataType>.KmoNode, KmoTreeSpatialIndex<KmoDataType>.KmoNode> pair) {
// java KmoTreeSpatialIndex<KmoDataType>.KmoNode N = (KmoNode)pair.first;
// java KmoTreeSpatialIndex<KmoDataType>.KmoNode NN = (KmoNode)pair.second;
// java KmoTreeSpatialIndex<KmoDataType>.KmoNode P = N.parent;
// java
// java for(KmoTreeSpatialIndex<KmoDataType>.KmoNode PP = null; !N.isRoot(); PP = null) {
// java if (NN != null) {
// java P.add(NN);
// java if (P.children.size() > 8) {
// java KmoPair<KmoTreeSpatialIndex<KmoDataType>.KmoNode, KmoTreeSpatialIndex<KmoDataType>.KmoNode> parent = this.helper_splitNode(P);
// java P = (KmoNode)parent.first;
// java PP = (KmoNode)parent.second;
// java }
// java }
// java
// java if (PP == null) {
// java P.rebuildMBR();
// java }
// java
// java N = P;
// java NN = PP;
// java P = P.parent;
// java }
// java
// java if (NN != null) {
// java this.root = new KmoNode();
// java this.root.add(N);
// java this.root.add(NN);
// java }
// java
// java }
fn adjust_split(&self,pair:(Node<T>,Node<T>)){let(mut n,nn)=pair;let mut nn=Some(nn);loop{let p=n.borrow().parent.upgrade();let Some(p)=p else{break;};let mut pp=None;if let Some(x)=nn{Self::add(&p,x);if p.borrow().children.len()>8{let (_,x)=Self::split(&p);pp=Some(x);}}if pp.is_none(){Self::rebuild(&p);}n=p;nn=pp;}if let Some(nn)=nn{let root=KmoNode::new();Self::add(&root,n);Self::add(&root,nn);*self.root.borrow_mut()=root;}}
// java public void insert(KmoRect rect, KmoDataType data) {
// java KmoTreeSpatialIndex<KmoDataType>.KmoNode L = this.helper_chooseLeaf(this.root, rect);
// java L.add(new KmoNode(rect, data));
// java if (L.children.size() <= 8) {
// java this.helper_adjustTree(L);
// java } else {
// java this.helper_adjustTree(this.helper_splitNode(L));
// java }
// java
// java }
pub fn insert(&self,rect:&KmoRect,data:Rc<T>){let mut n=self.root.borrow().clone();while !Self::is_leaf(&n){n=Self::least(&n,rect);}let item=KmoNode::new();{let mut i=item.borrow_mut();i.rc=rect.clone();i.data=Some(data);}Self::add(&n,item);if n.borrow().children.len()<=8{let mut p=n.borrow().parent.upgrade();while let Some(parent)=p{Self::rebuild(&parent);p=parent.borrow().parent.upgrade();}}else{self.adjust_split(Self::split(&n));}}
// java private KmoPair<KmoTreeSpatialIndex<KmoDataType>.KmoNode, KmoTreeSpatialIndex<KmoDataType>.KmoNode> helper_findLeaf(KmoTreeSpatialIndex<KmoDataType>.KmoNode node, KmoRect rect, KmoDataType data) {
// java if (node.rc.intersects(rect)) {
// java if (node.isLeaf()) {
// java for(KmoTreeSpatialIndex<KmoDataType>.KmoNode i : node.children) {
// java if (i.rc.intersects(rect) && i.data == data) {
// java return KmoPair.<KmoTreeSpatialIndex<KmoDataType>.KmoNode, KmoTreeSpatialIndex<KmoDataType>.KmoNode>of(node, i);
// java }
// java }
// java } else {
// java for(KmoTreeSpatialIndex<KmoDataType>.KmoNode i : node.children) {
// java KmoPair<KmoTreeSpatialIndex<KmoDataType>.KmoNode, KmoTreeSpatialIndex<KmoDataType>.KmoNode> ret = this.helper_findLeaf(i, rect, data);
// java if (ret != null) {
// java return ret;
// java }
// java }
// java }
// java }
// java
// java return null;
// java }
fn find(n:&Node<T>,rect:&KmoRect,data:&Rc<T>)->Option<(Node<T>,Node<T>)>{if !n.borrow().rc.intersects(rect){return None;}if Self::is_leaf(n){for i in n.borrow().children.clone(){if i.borrow().rc.intersects(rect)&&i.borrow().data.as_ref().is_some_and(|d|Rc::ptr_eq(d,data)){return Some((n.clone(),i));}}}else{for i in n.borrow().children.clone(){if let Some(p)=Self::find(&i,rect,data){return Some(p);}}}None}
// java private void helper_reInsertOrphaned(List<KmoTreeSpatialIndex<KmoDataType>.KmoNode> nodeLst) {
// java for(KmoTreeSpatialIndex<KmoDataType>.KmoNode i : nodeLst) {
// java if (i.data != null) {
// java this.insert(i.rc, i.data);
// java } else {
// java this.helper_reInsertOrphaned(i.children);
// java }
// java }
// java
// java }
fn reinsert(&self,nodes:Vec<Node<T>>){for n in nodes{if let Some(d)=n.borrow().data.clone(){self.insert(&n.borrow().rc,d);}else{self.reinsert(n.borrow().children.clone());}}}
// java private void helper_condenseTree(KmoTreeSpatialIndex<KmoDataType>.KmoNode L) {
// java List<KmoTreeSpatialIndex<KmoDataType>.KmoNode> Q = new ArrayList();
// java KmoTreeSpatialIndex<KmoDataType>.KmoNode N = L;
// java
// java for(KmoTreeSpatialIndex<KmoDataType>.KmoNode P = L.parent; !N.isRoot(); P = P.parent) {
// java if (N.children.size() < 4 && P.isRoot() && P.children.size() > 2) {
// java P.children.remove(N);
// java Q.add(N);
// java }
// java
// java P.rebuildMBR();
// java N = P;
// java }
// java
// java this.helper_reInsertOrphaned(Q);
// java if (this.root.children.size() == 1) {
// java this.root = (KmoNode)this.root.children.get(0);
// java this.root.parent = null;
// java }
// java
// java }
fn condense(&self,mut n:Node<T>){let mut q=vec![];loop{let p=n.borrow().parent.upgrade();let Some(p)=p else{break;};if n.borrow().children.len()<4&&p.borrow().parent.upgrade().is_none()&&p.borrow().children.len()>2{p.borrow_mut().children.retain(|c|!Rc::ptr_eq(c,&n));q.push(n);}Self::rebuild(&p);n=p;}self.reinsert(q);let root=self.root.borrow().clone();if root.borrow().children.len()==1{let next=root.borrow().children[0].clone();next.borrow_mut().parent=Weak::new();*self.root.borrow_mut()=next;}}
// java public void remove(KmoRect rect, KmoDataType node) {
// java KmoPair<KmoTreeSpatialIndex<KmoDataType>.KmoNode, KmoTreeSpatialIndex<KmoDataType>.KmoNode> result = this.helper_findLeaf(this.root, rect, node);
// java if (result != null) {
// java ((KmoNode)result.first).children.remove(result.second);
// java ((KmoNode)result.first).rebuildMBR();
// java this.helper_condenseTree((KmoNode)result.first);
// java }
// java }
pub fn remove(&self,rect:&KmoRect,data:&Rc<T>){let root=self.root.borrow().clone();if let Some((p,n))=Self::find(&root,rect,data){p.borrow_mut().children.retain(|c|!Rc::ptr_eq(c,&n));Self::rebuild(&p);self.condense(p);}}
// java public void clear() {
// java this.root = new KmoNode();
// java this.root.add(new KmoNode());
// java this.root.add(new KmoNode());
// java }
pub fn clear(&self){let root=KmoNode::new();Self::add(&root,KmoNode::new());Self::add(&root,KmoNode::new());*self.root.borrow_mut()=root;}
}
