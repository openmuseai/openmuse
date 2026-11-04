#![allow(non_snake_case, dead_code, unused_variables, unused_mut)]

use std::io::{self, Write};
use super::Node::Node;
use crate::ui::ILabelProvider::ILabelProvider;

// Java: public class TreeUtil
pub struct TreeUtil;

// Java: public class TreeUtil
impl TreeUtil {
    // Java: TreeUtil()
    pub fn new() -> Self {
        // Java: super();
        Self
    }

    // Java: static boolean isAllElementsNull(List<Node> list)
    fn isAllElementsNull(list: &[*mut Node]) -> bool {
        // Java: for(Object object : list)
        for &object in list {
            // Java: if (object != null)
            if !object.is_null() {
                // Java: return false;
                return false;
            }
        }
        // Java: return true;
        true
    }

    // Java: static int maxLevel(Node node)
    fn maxLevel(node: *mut Node) -> i32 {
        // Java: return node == null ? 0 : Math.max(maxLevel(node.left), maxLevel(node.right)) + 1;
        if node.is_null() {
            0
        } else {
            unsafe {
                std::cmp::max(Self::maxLevel((*node).left), Self::maxLevel((*node).right)) + 1
            }
        }
    }

    // Java: static <T extends Node> void printTree(Node root, ILabelProvider<T> labels, Writer out)
    pub fn printTree(root: *mut Node, labels: &dyn ILabelProvider<*mut Node>, out: &mut dyn Write) {
        // Java: if (root != null)
        if !root.is_null() {
            // Java: int maxLevel = maxLevel(root);
            let max_level = Self::maxLevel(root);
            // Java: try
            {
                // Java: printTreeInternal(Collections.singletonList(root), 1, maxLevel, labels, out);
                Self::printTreeInternal(&vec![root], 1, max_level, labels, out);
                // Java: out.flush();
                let _ = out.flush();
            }
            // Java: catch (IOException var5)
        }
    }

    // Java: static <T extends Node> void printTreeInternal(List<Node> nodes, int level, int maxLevel, ILabelProvider<T> labels, Writer out)
    fn printTreeInternal(nodes: &[*mut Node], level: i32, max_level: i32, labels: &dyn ILabelProvider<*mut Node>, out: &mut dyn Write) {
        // Java: if (!nodes.isEmpty() && !isAllElementsNull(nodes))
        if !nodes.is_empty() && !Self::isAllElementsNull(nodes) {
            // Java: int floor = maxLevel - level;
            let floor = max_level - level;
            // Java: int endgeLines = (int)Math.pow((double)2.0F, (double)Math.max(floor - 1, 0));
            let endge_lines = 2_i32.pow(std::cmp::max(floor - 1, 0) as u32);
            // Java: int firstSpaces = (int)Math.pow((double)2.0F, (double)floor) - 1;
            let first_spaces = 2_i32.pow(floor as u32) - 1;
            // Java: int betweenSpaces = (int)Math.pow((double)2.0F, (double)(floor + 1)) - 1;
            let between_spaces = 2_i32.pow((floor + 1) as u32) - 1;
            // Java: printWhitespaces(firstSpaces, out);
            Self::printWhitespaces(first_spaces, out);
            // Java: List<Node> newNodes = new ArrayList();
            let mut new_nodes: Vec<*mut Node> = Vec::new();
            // Java: for(Node node : nodes)
            for &node in nodes {
                // Java: if (node != null)
                if !node.is_null() {
                    // Java: String tag = labels.getText((T)node);
                    let tag = labels.getText(&node);
                    // Java: out.write(tag);
                    let _ = out.write_all(tag.as_bytes());
                    // Java: newNodes.add(node.left);
                    unsafe { new_nodes.push((*node).left); }
                    // Java: newNodes.add(node.right);
                    unsafe { new_nodes.push((*node).right); }
                // Java: else
                } else {
                    // Java: newNodes.add(null);
                    new_nodes.push(std::ptr::null_mut());
                    // Java: newNodes.add(null);
                    new_nodes.push(std::ptr::null_mut());
                    // Java: out.write(" ");
                    let _ = out.write_all(b" ");
                }
                // Java: printWhitespaces(betweenSpaces, out);
                Self::printWhitespaces(between_spaces, out);
            }
            // Java: out.write(10);
            let _ = out.write_all(b"\n");
            // Java: for(int i = 1; i <= endgeLines; ++i)
            for i in 1..=endge_lines {
                // Java: for(int j = 0; j < nodes.size(); ++j)
                for j in 0..nodes.len() {
                    // Java: printWhitespaces(firstSpaces - i, out);
                    Self::printWhitespaces(first_spaces - i, out);
                    // Java: if (nodes.get(j) == null)
                    if nodes[j].is_null() {
                        // Java: printWhitespaces(endgeLines + endgeLines + i + 1, out);
                        Self::printWhitespaces(endge_lines + endge_lines + i + 1, out);
                    // Java: else
                    } else {
                        // Java: if (((Node)nodes.get(j)).left != null)
                        unsafe {
                            if !(*nodes[j]).left.is_null() {
                                // Java: out.write(47);
                                let _ = out.write_all(b"/");
                            // Java: else
                            } else {
                                // Java: printWhitespaces(1, out);
                                Self::printWhitespaces(1, out);
                            }
                        }
                        // Java: printWhitespaces(i + i - 1, out);
                        Self::printWhitespaces(i + i - 1, out);
                        // Java: if (((Node)nodes.get(j)).right != null)
                        unsafe {
                            if !(*nodes[j]).right.is_null() {
                                // Java: out.write(92);
                                let _ = out.write_all(b"\\");
                            // Java: else
                            } else {
                                // Java: printWhitespaces(1, out);
                                Self::printWhitespaces(1, out);
                            }
                        }
                        // Java: printWhitespaces(endgeLines + endgeLines - i, out);
                        Self::printWhitespaces(endge_lines + endge_lines - i, out);
                    }
                }
                // Java: out.write(10);
                let _ = out.write_all(b"\n");
            }
            // Java: printTreeInternal(newNodes, level + 1, maxLevel, labels, out);
            Self::printTreeInternal(&new_nodes, level + 1, max_level, labels, out);
        }
    }

    // Java: static void printWhitespaces(int count, Writer out)
    fn printWhitespaces(mut count: i32, out: &mut dyn Write) {
        // Java: while(count-- > 0)
        while count > 0 {
            count -= 1;
            // Java: out.write(32);
            let _ = out.write_all(b" ");
        }
    }
}
