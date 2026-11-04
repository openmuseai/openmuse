/*
 * Copyright (c) 1997, 2023, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * This code is free software; you can redistribute it and/or modify itA
 * under the terms of the GNU General Public License version 2 only, as
 * published by the Free Software Foundation.  Oracle designates this
 * particular file as subject to the "Classpath" exception as provided
 * by Oracle in the LICENSE file that accompanied this code.
 *
 * This code is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
 * version 2 for more details (a copy is included in the LICENSE file that
 * accompanied this code).
 *
 * You should have received a copy of the GNU General Public License version
 * 2 along with this work; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 * Please contact Oracle, 500 Oracle Parkway, Redwood Shores, CA 94065 USA
 * or visit www.oracle.com if you need additional information or have any
 * questions.
 */
//! Native java.util.HashMap<String,V> put/entrySet closure used by content types.
//! Android 36 java/util/HashMap.java: list tail insertion, TreeNode links,
//! treeify/putTreeVal, red-black insertion, moveRootToFront, resize/split.
use std::cmp::Ordering;
struct Node<V>{hash:i32,key:String,value:V,parent:Option<usize>,left:Option<usize>,right:Option<usize>,red:bool}
impl<V> Node<V>{fn new(hash:i32,key:String,value:V)->Self{Self{hash,key,value,parent:None,left:None,right:None,red:false}}}
struct Bin<V>{nodes:Vec<Node<V>>,order:Vec<usize>,root:Option<usize>,tree:bool}
impl<V> Default for Bin<V>{fn default()->Self{Self{nodes:vec![],order:vec![],root:None,tree:false}}}
pub struct StringMap<V>{table:Vec<Bin<V>>,size:usize}
impl<V> Default for StringMap<V>{fn default()->Self{Self{table:vec![],size:0}}}
impl<V> StringMap<V>{
    // java HashMap.get(Object key): return (e = getNode(key)) == null ? null : e.value;
    pub fn get(&self,key:&str)->Option<&V>{if self.table.is_empty(){return None;}let hash=Self::hash(key);let bin=&self.table[hash as u32 as usize&(self.table.len()-1)];bin.nodes.iter().find(|n|n.hash==hash&&n.key==key).map(|n|&n.value)}
    // java public int size() { return size; }
    pub fn size(&self)->usize{self.size}
    // java public boolean isEmpty() { return size == 0; }
    pub fn isEmpty(&self)->bool{self.size==0}
    // java static final int hash(Object key) { int h; return key == null ? 0 : (h = key.hashCode()) ^ (h >>> 16); }
    fn hash(key:&str)->i32{let mut h=0i32;for c in key.encode_utf16(){h=h.wrapping_mul(31).wrapping_add(c as i32);}h^((h as u32>>16)as i32)}
    // java public V put(K key, V value) { return putVal(hash(key), key, value, false, true); }
    pub fn put(&mut self,key:String,value:V)->Option<V>{
        // java if ((tab = table) == null || (n = tab.length) == 0) n = (tab = resize()).length;
        if self.table.is_empty(){self.resize();}
        let hash=Self::hash(&key);let n=self.table.len();let index=hash as u32 as usize&(n-1);let bin=&mut self.table[index];
        // java if (e != null) { V oldValue = e.value; if (!onlyIfAbsent || oldValue == null) e.value = value; afterNodeAccess(e); return oldValue; }
        if let Some(node)=bin.nodes.iter_mut().find(|node|node.hash==hash&&node.key==key){return Some(std::mem::replace(&mut node.value,value));}
        let oldCount=bin.nodes.len();let x=oldCount;bin.nodes.push(Node::new(hash,key,value));
        // java else if (p instanceof TreeNode) e = ((TreeNode<K,V>)p).putTreeVal(this, tab, hash, key, value);
        if bin.tree{bin.putTreeVal(x);}
        // java p.next = newNode(hash, key, value, null); if (binCount >= TREEIFY_THRESHOLD - 1) treeifyBin(tab, hash);
        else{bin.order.push(x);if oldCount>=8{if n<64{self.resize();}else{self.table[index].treeify();}}}
        // java ++modCount; if (++size > threshold) resize(); afterNodeInsertion(evict); return null;
        self.size+=1;if self.size>self.table.len()*3/4{self.resize();}None
    }
    // java HashIterator.nextNode walks table indices, then the Node.next chain.
    pub fn entrySet(&self)->impl Iterator<Item=(&str,&V)>{self.table.iter().flat_map(|bin|bin.order.iter().map(move|i|{let node=&bin.nodes[*i];(node.key.as_str(),&node.value)}))}
    // java final Node<K,V>[] resize() {
    fn resize(&mut self){
        // java if (oldCap > 0) newCap = oldCap << 1; else newCap = DEFAULT_INITIAL_CAPACITY;
        let n=self.table.len();let capacity=if n==0{16}else{n*2};let mut next:Vec<Bin<V>>=(0..capacity).map(|_|Bin::default()).collect();
        // java for (int j = 0; j < oldCap; ++j) {
        for (j,bin)in std::mem::take(&mut self.table).into_iter().enumerate(){
            let lo=bin.order.iter().filter(|i|bin.nodes[**i].hash as u32 as usize&n==0).count();let hi=bin.order.len()-lo;
            // java single node and unsplit TreeNode: retain the same next/tree links.
            if hi==0{next[j]=bin;continue;}if lo==0{next[j+n]=bin;continue;}
            let tree=bin.tree;let mut nodes:Vec<_>=bin.nodes.into_iter().map(Some).collect();
            // java list split and TreeNode.split: relink into lo/hi lists, preserving order.
            for id in bin.order{let node=nodes[id].take().unwrap();let index=if node.hash as u32 as usize&n==0{j}else{j+n};let b=&mut next[index];b.order.push(b.nodes.len());b.nodes.push(Node::new(node.hash,node.key,node.value));}
            // java if (lc <= UNTREEIFY_THRESHOLD) tab[index] = loHead.untreeify(map); else { tab[index] = loHead; if (hiHead != null) loHead.treeify(tab); }
            if tree{if lo>6{next[j].treeify();}if hi>6{next[j+n].treeify();}}
        }
        self.table=next;
    }
}
impl<V> Bin<V>{
    // java compare String hash, then compareComparables(String.class,k,pk).
    fn dir(&self,x:usize,p:usize)->Ordering{self.nodes[x].hash.cmp(&self.nodes[p].hash).then_with(||self.nodes[x].key.encode_utf16().cmp(self.nodes[p].key.encode_utf16()))}
    // java TreeNode.moveRootToFront: unlink root from prev/next and place before first.
    fn moveRootToFront(&mut self,root:usize){let i=self.order.iter().position(|i|*i==root).unwrap();if i!=0{self.order.remove(i);self.order.insert(0,root);}self.root=Some(root);}
    // java final void treeify(Node<K,V>[] tab) {
    fn treeify(&mut self){
        // java TreeNode<K,V> root = null; for (TreeNode<K,V> x = this, next; x != null; x = next) {
        let mut root=None;let order=self.order.clone();
        for x in order{
            // java next = (TreeNode<K,V>)x.next; x.left = x.right = null;
            self.nodes[x].left=None;self.nodes[x].right=None;
            // java if (root == null) { x.parent = null; x.red = false; root = x; }
            if root.is_none(){self.nodes[x].parent=None;self.nodes[x].red=false;root=Some(x);}
            // java else { for (TreeNode<K,V> p = root;;) {
            else{let mut p=root.unwrap();loop{
                // java int dir; compare hash, then String.compareTo (distinct keys cannot compare equal).
                let dir=self.dir(x,p);let next=if dir!=Ordering::Greater{self.nodes[p].left}else{self.nodes[p].right};
                // java TreeNode<K,V> xp = p; if ((p = (dir <= 0) ? p.left : p.right) == null) {
                if let Some(next)=next{p=next;}else{
                    // java x.parent = xp; if (dir <= 0) xp.left = x; else xp.right = x; root = balanceInsertion(root, x); break;
                    self.nodes[x].parent=Some(p);if dir!=Ordering::Greater{self.nodes[p].left=Some(x);}else{self.nodes[p].right=Some(x);}root=Some(self.balanceInsertion(root.unwrap(),x));break;
                }
            }}
        }
        // java moveRootToFront(tab, root);
        self.tree=true;if let Some(root)=root{self.moveRootToFront(root);}
    }
    // java final TreeNode<K,V> putTreeVal(HashMap<K,V> map, Node<K,V>[] tab, int h, K k, V v) {
    fn putTreeVal(&mut self,x:usize){
        // java TreeNode<K,V> root = (parent != null) ? root() : this; for (TreeNode<K,V> p = root;;) {
        let root=self.root.unwrap();let mut p=root;loop{
            // java String is comparable; putVal already found/replaced any equal key above.
            let dir=self.dir(x,p);let next=if dir!=Ordering::Greater{self.nodes[p].left}else{self.nodes[p].right};
            // java TreeNode<K,V> xp = p; if ((p = (dir <= 0) ? p.left : p.right) == null) {
            if let Some(next)=next{p=next;}else{
                // java Node<K,V> xpn = xp.next; TreeNode<K,V> x = map.newTreeNode(h,k,v,xpn); if (dir <= 0) xp.left = x; else xp.right = x;
                if dir!=Ordering::Greater{self.nodes[p].left=Some(x);}else{self.nodes[p].right=Some(x);}
                // java xp.next = x; x.parent = x.prev = xp; if (xpn != null) ((TreeNode<K,V>)xpn).prev = x;
                self.nodes[x].parent=Some(p);let pos=self.order.iter().position(|id|*id==p).unwrap();self.order.insert(pos+1,x);
                // java moveRootToFront(tab, balanceInsertion(root, x)); return null;
                let root=self.balanceInsertion(root,x);self.moveRootToFront(root);break;
            }
        }
    }
    // java static TreeNode<K,V> rotateLeft(TreeNode<K,V> root, TreeNode<K,V> p) {
    fn rotateLeft(&mut self,mut root:usize,p:usize)->usize{
        // java if (p != null && (r = p.right) != null) {
        if let Some(r)=self.nodes[p].right{
            // java if ((rl = p.right = r.left) != null) rl.parent = p;
            self.nodes[p].right=self.nodes[r].left;if let Some(rl)=self.nodes[p].right{self.nodes[rl].parent=Some(p);}
            // java if ((pp = r.parent = p.parent) == null) (root = r).red = false; else if (pp.left == p) pp.left = r; else pp.right = r;
            self.nodes[r].parent=self.nodes[p].parent;if let Some(pp)=self.nodes[r].parent{if self.nodes[pp].left==Some(p){self.nodes[pp].left=Some(r);}else{self.nodes[pp].right=Some(r);}}else{root=r;self.nodes[r].red=false;}
            // java r.left = p; p.parent = r;
            self.nodes[r].left=Some(p);self.nodes[p].parent=Some(r);
        }
        // java return root;
        root
    }
    // java static TreeNode<K,V> rotateRight(TreeNode<K,V> root, TreeNode<K,V> p) {
    fn rotateRight(&mut self,mut root:usize,p:usize)->usize{
        // java if (p != null && (l = p.left) != null) {
        if let Some(l)=self.nodes[p].left{
            // java if ((lr = p.left = l.right) != null) lr.parent = p;
            self.nodes[p].left=self.nodes[l].right;if let Some(lr)=self.nodes[p].left{self.nodes[lr].parent=Some(p);}
            // java if ((pp = l.parent = p.parent) == null) (root = l).red = false; else if (pp.right == p) pp.right = l; else pp.left = l;
            self.nodes[l].parent=self.nodes[p].parent;if let Some(pp)=self.nodes[l].parent{if self.nodes[pp].right==Some(p){self.nodes[pp].right=Some(l);}else{self.nodes[pp].left=Some(l);}}else{root=l;self.nodes[l].red=false;}
            // java l.right = p; p.parent = l;
            self.nodes[l].right=Some(p);self.nodes[p].parent=Some(l);
        }
        // java return root;
        root
    }
    // java static TreeNode<K,V> balanceInsertion(TreeNode<K,V> root, TreeNode<K,V> x) {
    fn balanceInsertion(&mut self,mut root:usize,mut x:usize)->usize{
        // java x.red = true; for (TreeNode<K,V> xp, xpp, xppl, xppr;;) {
        self.nodes[x].red=true;loop{
            // java if ((xp = x.parent) == null) { x.red = false; return x; }
            let Some(parent)=self.nodes[x].parent else{self.nodes[x].red=false;return x;};
            // java else if (!xp.red || (xpp = xp.parent) == null) return root;
            if !self.nodes[parent].red||self.nodes[parent].parent.is_none(){return root;}
            let mut xp=Some(parent);let mut xpp=self.nodes[parent].parent;let grand=xpp.unwrap();let xppl=self.nodes[grand].left;
            // java if (xp == (xppl = xpp.left)) {
            if xp==xppl{
                // java if ((xppr = xpp.right) != null && xppr.red) { xppr.red=false; xp.red=false; xpp.red=true; x=xpp; }
                if let Some(xppr)=self.nodes[grand].right.filter(|p|self.nodes[*p].red){self.nodes[xppr].red=false;self.nodes[parent].red=false;self.nodes[grand].red=true;x=grand;}
                // java else {
                else{
                    // java if (x == xp.right) { root=rotateLeft(root,x=xp); xpp=(xp=x.parent)==null?null:xp.parent; }
                    if Some(x)==self.nodes[parent].right{x=parent;root=self.rotateLeft(root,x);xp=self.nodes[x].parent;xpp=xp.and_then(|p|self.nodes[p].parent);}
                    // java if (xp != null) { xp.red=false; if (xpp != null) { xpp.red=true; root=rotateRight(root,xpp); } }
                    if let Some(xp)=xp{self.nodes[xp].red=false;if let Some(xpp)=xpp{self.nodes[xpp].red=true;root=self.rotateRight(root,xpp);}}
                }
            // java } else {
            }else{
                // java if (xppl != null && xppl.red) { xppl.red=false; xp.red=false; xpp.red=true; x=xpp; }
                if let Some(xppl)=xppl.filter(|p|self.nodes[*p].red){self.nodes[xppl].red=false;self.nodes[parent].red=false;self.nodes[grand].red=true;x=grand;}
                // java else {
                else{
                    // java if (x == xp.left) { root=rotateRight(root,x=xp); xpp=(xp=x.parent)==null?null:xp.parent; }
                    if Some(x)==self.nodes[parent].left{x=parent;root=self.rotateRight(root,x);xp=self.nodes[x].parent;xpp=xp.and_then(|p|self.nodes[p].parent);}
                    // java if (xp != null) { xp.red=false; if (xpp != null) { xpp.red=true; root=rotateLeft(root,xpp); } }
                    if let Some(xp)=xp{self.nodes[xp].red=false;if let Some(xpp)=xpp{self.nodes[xpp].red=true;root=self.rotateLeft(root,xpp);}}
                }
            }
        }
    }
}
