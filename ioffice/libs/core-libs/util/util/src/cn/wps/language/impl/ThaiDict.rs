#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::any::Any;
use std::io::BufRead;
use std::sync::Mutex;
use std::sync::OnceLock;

// Java: import cn.wps.kfc.impl.map.WIntObjectHashMap
// Java: import cn.wps.moffice.util.KFSFileLogAdapter
// Java: import java.io.BufferedReader
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.InputStreamReader

// Java field: String TAG
pub(crate) const TAG: &str = "";

// Java field: DictNode msRootNode
static MS_ROOT_NODE: OnceLock<Mutex<DictNode>> = OnceLock::new();

fn get_root_node() -> &'static Mutex<DictNode> {
    MS_ROOT_NODE.get_or_init(|| Mutex::new(DictNode::new()))
}

// Java: public class ThaiDict
pub struct ThaiDict;

// Java: public class DictNode
pub struct DictNode {
    // Java field: boolean isEnd
    pub isEnd: bool,
    // Java field: WIntObjectHashMap<DictNode> subNodes
    pub subNodes: Option<std::collections::HashMap<i32, Box<DictNode>>>,
}

// Java: public class ThaiDict
impl ThaiDict {
    // Java: ThaiDict()
    // Java: super();

    // Java: static boolean init(InputStream inputStream)
    pub fn init(_inputStream: &dyn Any) -> bool {
        // Java: synchronized(msRootNode)
        {
            let root = get_root_node().lock().unwrap();
            // Java: return null != msRootNode.subNodes ? true : initForOrgDictFile(inputStream);
            if root.subNodes.is_some() {
                true
            } else {
                drop(root);
                Self::initForOrgDictFile(_inputStream)
            }
        }
    }

    // Java: static boolean isThai(char ch)
    pub fn isThai(ch: char) -> bool {
        // Java: return ch >> 8 == 14;
        (ch as u32) >> 8 == 14
    }

    // Java: static boolean initForOrgDictFile(InputStream inputStream)
    pub fn initForOrgDictFile(_inputStream: &dyn Any) -> bool {
        // Java: InputStreamReader inputStreamReader = new InputStreamReader(inputStream);
        // Java: BufferedReader reader = new BufferedReader(inputStreamReader);
        // Java: boolean rs = true;
        let rs = true;
        // Java: try
        {
            // Java: String line;
            // Java: try
            {
                // Java: while(null != (line = reader.readLine()))
                {
                    // Java: line = line.trim();
                    // Java: int charIndex = 0;
                    // Java: char ch = line.charAt(charIndex);
                    // Java: int max = line.length();
                    // Java: if (isThai(ch))
                    {
                        // Java: DictNode curParentNode;
                        // Java: DictNode curNode;
                        // Java: for(curParentNode = msRootNode; charIndex < max; curParentNode = curNode)
                        {
                            // Java: ch = line.charAt(charIndex);
                            // Java: if (null == curParentNode.subNodes)
                            {
                                // Java: curParentNode.subNodes = new WIntObjectHashMap<DictNode>();
                            }
                            // Java: curNode = null;
                            // Java: if (curParentNode.subNodes.containsKey(ch))
                            {
                                // Java: curNode = curParentNode.subNodes.get(ch);
                            }
                            // Java: else
                            {
                                // Java: curNode = new DictNode();
                                // Java: curParentNode.subNodes.put(ch, curNode);
                            }
                            // Java: ++charIndex;
                        }
                        // Java: curParentNode.isEnd = true;
                    }
                }
            }
            // Java: catch (Exception e)
            {
                // Java: rs = false;
                // Java: KFSFileLogAdapter.e(TAG, "init dict error", e);
                // Java: msRootNode.subNodes = null;
            }
        }
        // Java: finally
        {
            // Java: try
            {
                // Java: reader.close();
            }
            // Java: catch (IOException e)
            {
                // Java: KFSFileLogAdapter.e(TAG, "close dict reader error", e);
            }
            // Java: try
            {
                // Java: inputStreamReader.close();
            }
            // Java: catch (IOException e)
            {
                // Java: KFSFileLogAdapter.e(TAG, "close dict inputStreamReader error", e);
            }
        }
        // Java: return rs;
        rs
    }

    // Java: static int getWordLen(_chars: &[char], int start, int max)
    pub fn getWordLen(chars: &[char], start: i32, max: i32) -> i32 {
        // Java: synchronized(msRootNode)
        {
            // Java: DictNode curDictNode = msRootNode;
            // Java: int index = start;
            let mut _index = start;
            // Java: for(int lastVaildMax = start; index < max; ++index)
            let mut _lastVaildMax = start;
            while _index < max {
                // Java: char ch = chars[index];
                // Java: if (null == curDictNode.subNodes || !curDictNode.subNodes.containsKey(ch))
                {
                    // Java: return Math.max(lastVaildMax + 1 - start, 1);
                }
                // Java: curDictNode = curDictNode.subNodes.get(ch);
                // Java: if (curDictNode.isEnd)
                {
                    // Java: lastVaildMax = index;
                    _lastVaildMax = _index;
                }
                _index += 1;
            }
            // Java: return Math.max(max - start, 1);
            (max - start).max(1)
        }
    }
}

// Java: public class DictNode
impl DictNode {
    pub fn new() -> Self {
        DictNode {
            isEnd: false,
            subNodes: None,
        }
    }
}
