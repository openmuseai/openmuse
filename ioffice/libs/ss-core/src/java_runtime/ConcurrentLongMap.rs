/*
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * This code is free software; you can redistribute it and/or modify it
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
//! java.util.concurrent.ConcurrentHashMap<Long,V> under the enclosing valuePos
//! monitor. Native ownership adapter: preserve bucket/next traversal, including
//! resize's lastRun split and TreeBin.first ordering. ET never owns this order.
#[derive(Clone)]struct Node<V>{hash:u32,key:i64,value:V}
#[derive(Clone)]struct Bin<V>{nodes:Vec<Node<V>>,tree:bool}
impl<V> Default for Bin<V>{fn default()->Self{Self{nodes:vec![],tree:false}}}
#[derive(Clone)]pub struct ConcurrentLongMap<V>{table:Vec<Bin<V>>,count:usize}
impl<V> Default for ConcurrentLongMap<V>{fn default()->Self{Self{table:vec![],count:0}}}
impl<V:Clone> ConcurrentLongMap<V>{
    // java spread(Long.hashCode(key)): (h ^ (h >>> 16)) & HASH_BITS;
    fn spread(key:i64)->u32{let h=(key as u64^(key as u64>>32))as u32;(h^(h>>16))&0x7fffffff}
    // java get: traverse the matching bin, compare hash and Long.equals.
    pub fn get(&self,key:&i64)->Option<&V>{if self.table.is_empty(){return None;}let hash=Self::spread(*key);self.table[hash as usize&(self.table.len()-1)].nodes.iter().find(|n|n.hash==hash&&n.key==*key).map(|n|&n.value)}
    // java putVal(key,value,false), serialized by the original enclosing collector monitor.
    pub fn insert(&mut self,key:i64,value:V)->Option<V>{
        // java if (tab == null || (n = tab.length) == 0) tab = initTable();
        if self.table.is_empty(){self.table=(0..16).map(|_|Bin::default()).collect();}
        let hash=Self::spread(key);let n=self.table.len();let index=hash as usize&(n-1);let bin=&mut self.table[index];
        // java existing node: oldVal = e.val; if (!onlyIfAbsent) e.val = value;
        if let Some(node)=bin.nodes.iter_mut().find(|v|v.hash==hash&&v.key==key){return Some(std::mem::replace(&mut node.value,value));}
        let oldCount=bin.nodes.len();let tree=bin.tree;
        // java TreeBin.putTreeVal: first = x = new TreeNode(h,k,v,f,xp); list putVal: pred.next = new Node(hash,key,value,null);
        let node=Node{hash,key,value};if tree{bin.nodes.insert(0,node);}else{bin.nodes.push(node);}
        // java if (binCount >= TREEIFY_THRESHOLD) treeifyBin(tab,i);
        if !tree&&oldCount>=8{
            // java if (n < MIN_TREEIFY_CAPACITY) tryPresize(n << 1);
            if n<64{let size=n<<1;let c=(size+(size>>1)+1).next_power_of_two();while self.table.len()-self.table.len()/4<c{self.transfer();}}
            // java new TreeBin(hd): first is the original next chain, independent of search-tree rotations.
            else{self.table[index].tree=true;}
        }
        // java addCount(1L,binCount): resize once count reaches sizeCtl (three quarters of capacity).
        self.count+=1;while self.count>=self.table.len()-self.table.len()/4{self.transfer();}None
    }
    fn transfer(&mut self){
        let n=self.table.len();let old=std::mem::take(&mut self.table);let mut next:Vec<Bin<V>>=(0..n*2).map(|_|Bin::default()).collect();
        for (i,bin) in old.into_iter().enumerate(){
            if bin.tree{
                // java TreeBin split: preserve first/next order in each side; untreeify for <= UNTREEIFY_THRESHOLD.
                for node in bin.nodes{let index=if node.hash as usize&n==0{i}else{i+n};next[index].nodes.push(node);}
                next[i].tree=next[i].nodes.len()>6;next[i+n].tree=next[i+n].nodes.len()>6;
            }else if !bin.nodes.is_empty(){
                // java int runBit = fh & n; Node lastRun = f; for (p=f.next;p!=null;p=p.next) { int b=p.hash&n; if (b!=runBit) { runBit=b;lastRun=p; } }
                let mut runBit=bin.nodes[0].hash as usize&n;let mut lastRun=0;
                for p in 1..bin.nodes.len(){let b=bin.nodes[p].hash as usize&n;if b!=runBit{runBit=b;lastRun=p;}}
                let mut nodes=bin.nodes;let suffix=nodes.split_off(lastRun);
                // java if (runBit == 0) { ln=lastRun;hn=null; } else { hn=lastRun;ln=null; }
                let target=if runBit==0{i}else{i+n};next[target].nodes=suffix;
                // java for (p=f;p!=lastRun;p=p.next) { if ((ph&n)==0) ln=new Node(ph,pk,pv,ln); else hn=new Node(ph,pk,pv,hn); }
                for node in nodes{let index=if node.hash as usize&n==0{i}else{i+n};next[index].nodes.insert(0,node);}
            }
        }
        self.table=next;
    }
    pub fn clear(&mut self){for bin in &mut self.table{bin.nodes.clear();bin.tree=false;}self.count=0;}
    pub fn len(&self)->usize{self.count}
}
impl<V> IntoIterator for ConcurrentLongMap<V>{type Item=(i64,V);type IntoIter=std::vec::IntoIter<(i64,V)>;
    // java Traverser.advance: table indices ascending, each bin via next (TreeBin.first).
    fn into_iter(self)->Self::IntoIter{self.table.into_iter().flat_map(|bin|bin.nodes.into_iter().map(|n|(n.key,n.value))).collect::<Vec<_>>().into_iter()}}
// rust: compatibility with existing native callers' map indexing, same get().
impl<V:Clone> std::ops::Index<&i64> for ConcurrentLongMap<V>{type Output=V;fn index(&self,key:&i64)->&V{self.get(key).expect("missing ConcurrentHashMap key")}}
