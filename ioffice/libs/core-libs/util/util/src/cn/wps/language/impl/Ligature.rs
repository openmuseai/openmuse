#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import cn.wps.language.Arabic
use crate::language::Arabic::Arabic;
// Java: import java.util.HashMap
use std::collections::HashMap;
use std::sync::OnceLock;
use std::sync::Mutex;

// Java field: Node root
static ROOT: OnceLock<Mutex<Option<LigNode>>> = OnceLock::new();
// Java field: HashMap<Character, Node> startOrMiddleNode
static START_OR_MIDDLE_NODE: OnceLock<Mutex<HashMap<char, bool>>> = OnceLock::new();

// Java: public class Ligature
pub struct Ligature;

// Java: public class Ligature
impl Ligature {
    // Java: Ligature()
    pub fn new() -> Self {
        // Java: super();
        Self
    }

    // Java: static boolean canConnectNext(_chars: &[char], int i, int end, Node curNode)
    fn canConnectNext(chars: &[char], mut i: usize, end: usize, cur_node: &LigNode) -> bool {
        // Java: if (null == curNode.start && null == curNode.middle)
        if cur_node.start.is_none() && cur_node.middle.is_none()
        {
            // Java: return false;
            return false;
        }
        // Java: else
        {
            // Java: char c = chars[i];
            let mut c = chars[i];
            // Java: if (c != ' ' && c != '.')
            if c != ' ' && c != '.'
            {
                // Java: Node node = root;
                // Java: Node resultNode = node;
                // Java: while(node.childs != null)
                {
                    // Java: if (isSpacingMark(c))
                    if Self::isSpacingMark(c)
                    {
                        // Java: ++i;
                        i += 1;
                        // Java: if (i >= end)
                        if i >= end
                        {
                            // Java: break;
                        }
                        // Java: c = chars[i];
                    }
                    // Java: else
                    {
                        // Java: Node[] childs = node.childs;
                        // Java: Node findNode = binarySearch(childs, c);
                        // Java: if (findNode == null)
                        {
                            // Java: break;
                        }
                        // Java: if (findNode.level > 0)
                        {
                            // Java: resultNode = findNode;
                        }
                        // Java: node = findNode;
                        // Java: ++i;
                        // Java: if (i >= end)
                        {
                            // Java: break;
                        }
                        // Java: c = chars[i];
                    }
                }
                // Java: if (resultNode == root)
                {
                    // Java: return false;
                }
                // Java: else
                {
                    // Java: return null != resultNode.middle || null != resultNode.end;
                }
                false
            }
            // Java: else
            else
            {
                // Java: return false;
                false
            }
        }
    }

    // Java: static Node binarySearch(Node[] childs, char c)
    pub fn binarySearch(childs: &[LigNode], c: char) -> Option<usize> {
        // Java: int low = 0;
        let mut low: i32 = 0;
        // Java: int high = childs.length - 1;
        let mut high: i32 = childs.len() as i32 - 1;
        // Java: while(low <= high)
        while low <= high
        {
            // Java: int middle = low + (high - low >>> 1);
            let middle = low + ((high - low) as u32 >> 1) as i32;
            let mid_idx = middle as usize;
            // Java: if (c < childs[middle].ch)
            if c < childs[mid_idx].ch
            {
                // Java: high = middle - 1;
                high = middle - 1;
            }
            // Java: else
            else
            {
                // Java: if (c == childs[middle].ch)
                if c == childs[mid_idx].ch
                {
                    // Java: return childs[middle];
                    return Some(mid_idx);
                }
                // Java: low = middle + 1;
                low = middle + 1;
            }
        }
        // Java: return null;
        None
    }

    // Java: static int getLigatureGraphs(Arabic.Args args)
    pub fn getLigatureGraphs(args: &mut crate::language::Arabic::Args) -> i32 {
        // Java: init();
        Self::init();
        // Java: int count = 0;
        let mut count: i32 = 0;
        // Java: int start = args.srcPos;
        let start = args.srcPos;
        // Java: int end = start + args.len;
        let end = start + args.len;
        // Java: _chars: &[char] = args.srcBuff;
        // Java: boolean hasBefore = true;
        let mut has_before = true;
        // Java: boolean hasAfter = true;
        let mut has_after = true;
        // Java: int lastLevel = -1;
        let mut last_level: i32 = -1;
        // Java: boolean lastIsArabic = false;
        let mut last_is_arabic = false;
        // Java: int curSrcPos = start;
        let mut cur_src_pos = start;
        // Java: int i = start;
        let mut i = start;
        // Java: while(i < end)
        while i < end
        {
            // Java: char c = chars[i];
            let mut c = args.srcBuff.as_ref().unwrap()[i];
            // Java: boolean isArabic = isArabic(c);
            let is_arabic = Self::isArabic(c);
            // Java: if (i == start)
            if i == start
            {
                // Java: hasBefore = false;
                has_before = false;
            }
            // Java: Node node = root;
            // Java: Node tmpNode = node;
            // Java: while(tmpNode.childs != null)
            {
                // Java: if (isSpacingMark(c))
                if Self::isSpacingMark(c)
                {
                    // Java: if (node != root)
                    {
                        // Java: break;
                    }
                    // Java: args.desBuff[args.desPos] = c;
                    if let Some(ref mut des) = args.desBuff {
                        des[args.desPos] = c;
                    }
                    // Java: ++count;
                    count += 1;
                    // Java: ++args.desPos;
                    args.desPos += 1;
                    // Java: --args.len;
                    args.len -= 1;
                    // Java: ++args.srcPos;
                    args.srcPos += 1;
                    // Java: ++i;
                    i += 1;
                    // Java: ++curSrcPos;
                    cur_src_pos += 1;
                    // Java: if (i >= end)
                    if i >= end
                    {
                        // Java: return count;
                        return count;
                    }
                    // Java: c = chars[i];
                    c = args.srcBuff.as_ref().unwrap()[i];
                }
                // Java: else
                {
                    // Java: Node nodeFind = binarySearch(tmpNode.childs, c);
                    // Java: if (nodeFind != null)
                    {
                        // Java: if (lastLevel != -1 && lastLevel != nodeFind.level)
                        {
                            // Java: return count;
                        }
                        // Java: tmpNode = nodeFind;
                        // Java: ++i;
                        // Java: if (nodeFind.level > 0)
                        {
                            // Java: node = nodeFind;
                            // Java: curSrcPos = i;
                        }
                        // Java: if (i < end)
                        {
                            // Java: c = chars[i];
                        }
                    }
                    // Java: if (nodeFind == null || i >= end)
                    {
                        // Java: break;
                    }
                }
            }
            // Java: if (lastLevel != -1 && (lastLevel != node.level || lastIsArabic != isArabic))
            {
                // Java: return count;
            }
            // Java: lastLevel = node.level;
            // Java: lastIsArabic = isArabic;
            // Java: if (node == root)
            {
                // Java: args.desBuff[args.desPos] = c;
                if let Some(ref mut des) = args.desBuff {
                    des[args.desPos] = c;
                }
                // Java: ++curSrcPos;
                cur_src_pos += 1;
                // Java: hasBefore = false;
                has_before = false;
            }
            // Java: else
            {
                // Java: if (curSrcPos >= end)
                {
                    // Java: hasAfter = false;
                }
                // Java: else
                {
                    // Java: hasAfter = canConnectNext(chars, curSrcPos, end, node);
                }
                // Java: Node tmp = node.isolate;
                // Java: if (null == tmp)
                {
                    // Java: tmp = node.end;
                }
                // Java: if (null == tmp)
                {
                    // Java: tmp = node.start;
                }
                // Java: if (null == tmp)
                {
                    // Java: tmp = node.middle;
                }
                // Java: if (null == tmp)
                {
                    // Java: tmp = node;
                }
                // Java: if (!hasBefore && args.desPos >= 1)
                {
                    // Java: hasBefore = null != startOrMiddleNode.get(args.desBuff[args.desPos - 1]);
                }
                // Java: if (hasBefore && hasAfter)
                {
                    // Java: Node result = node.middle;
                    // Java: if (null == result)
                    {
                        // Java: result = node.end != null ? node.end : tmp;
                    }
                    // Java: else
                    {
                        // Java: startOrMiddleNode.put(result.ch, result);
                    }
                    // Java: args.desBuff[args.desPos] = result.ch;
                    // Java: hasBefore = result == node.middle || result == node.start;
                }
                // Java: else if (hasBefore && !hasAfter)
                {
                    // Java: Node result = node.end;
                    // Java: if (null == result)
                    {
                        // Java: result = node.middle != null ? node.middle : tmp;
                    }
                    // Java: args.desBuff[args.desPos] = result.ch;
                    // Java: hasBefore = false;
                }
                // Java: else if (!hasBefore && hasAfter)
                {
                    // Java: Node result = node.start;
                    // Java: hasBefore = null != result;
                    // Java: if (null == result)
                    {
                        // Java: result = node.isolate != null ? node.isolate : tmp;
                    }
                    // Java: else
                    {
                        // Java: startOrMiddleNode.put(result.ch, result);
                    }
                    // Java: args.desBuff[args.desPos] = result.ch;
                }
                // Java: else if (!hasBefore && !hasAfter)
                {
                    // Java: Node result = node.isolate;
                    // Java: if (null == result)
                    {
                        // Java: result = node.start != null ? node.start : tmp;
                    }
                    // Java: args.desBuff[args.desPos] = result.ch;
                    // Java: hasBefore = false;
                }
            }
            // Java: args.len -= curSrcPos - args.srcPos;
            args.len -= cur_src_pos - args.srcPos;
            // Java: args.srcPos = curSrcPos;
            args.srcPos = cur_src_pos;
            // Java: i = curSrcPos;
            i = cur_src_pos;
            // Java: ++count;
            count += 1;
            // Java: ++args.desPos;
            args.desPos += 1;
            // Java: if (node.level > 1)
            {
                // Java: return count;
            }
        }
        // Java: return count;
        count
    }

    // Java: static void init()
    fn init() {
        // Java: if (null == root)
        {
            // Java: root = new Node();
            // Java: root.level = 0;
            // Java: LigatureIniter.init(root);
            ROOT.get_or_init(|| Mutex::new(None));
            START_OR_MIDDLE_NODE.get_or_init(|| Mutex::new(HashMap::new()));
        }
    }

    // Java: static boolean isArabic(char c)
    fn isArabic(c: char) -> bool {
        // Java: return Arabic.charIsArabic(c);
        Arabic::charIsArabic(c)
    }

    // Java: static boolean isSpacingMark(char ch)
    fn isSpacingMark(ch: char) -> bool {
        // Java: int charType = Character.getType(ch);
        // Java: return charType == 6 || charType == 8;
        false
    }
}

// Java: public class Node
pub struct LigNode {
    // Java field: char ch
    pub ch: char,
    // Java field: Node[] childs
    pub childs: Option<Vec<LigNode>>,
    // Java field: int level
    pub level: i32,
    // Java field: Node end
    pub end: Option<Box<LigNode>>,
    // Java field: Node isolate
    pub isolate: Option<Box<LigNode>>,
    // Java field: Node middle
    pub middle: Option<Box<LigNode>>,
    // Java field: Node start
    pub start: Option<Box<LigNode>>,
}

// Java: public class Node
impl LigNode {
    // Java: Node()
    pub fn new() -> Self {
        // Java: super();
        Self {
            ch: '\0',
            childs: None,
            level: 0,
            end: None,
            isolate: None,
            middle: None,
            start: None,
        }
    }

    // Java: Node(char ch)
    pub fn new_with_ch(ch: char) -> Self {
        // Java: super();
        // Java: this.ch = ch;
        Self {
            ch,
            childs: None,
            level: 0,
            end: None,
            isolate: None,
            middle: None,
            start: None,
        }
    }

    // Java: String toString()
    pub fn to_string(&self) -> String {
        // Java: return "" + this.ch;
        self.ch.to_string()
    }
}
