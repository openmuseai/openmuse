// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use starlark::{
    collections::SmallSet,
    values::{ProvidesStaticType, UnpackValue as _, Value, ValueLike},
};

use crate::{
    depset::{Depset, DepsetGen, Order},
    unpack::UnpackDepset,
};

/// An iterator over the elements of a depset, respecting its traversal order.
pub struct DepsetIterator<'a, 'v> {
    stack: Vec<IterState<'a, 'v>>,
    visited_nodes: std::collections::HashSet<usize>,
    visited_elements: SmallSet<Value<'v>>,
    order: Order,
    topo_buffer: Option<Vec<Value<'v>>>,
}

enum IterState<'a, 'v> {
    Direct {
        iter: std::slice::Iter<'a, Value<'v>>,
    },
    Transitive {
        iter: std::slice::Iter<'a, Value<'v>>,
    },
}

impl<'a, 'v> DepsetIterator<'a, 'v> {
    fn push_depset(&mut self, depset: &'a Depset<'v>, order: Order) {
        if !depset.is_empty() {
            match order {
                Order::Preorder => {
                    self.stack.push(IterState::Transitive {
                        iter: depset.transitive().iter(),
                    });
                    self.stack.push(IterState::Direct {
                        iter: depset.direct().iter(),
                    });
                },
                Order::Postorder | Order::Unspecified => {
                    self.stack.push(IterState::Direct {
                        iter: depset.direct().iter(),
                    });
                    self.stack.push(IterState::Transitive {
                        iter: depset.transitive().iter(),
                    });
                },
                Order::Topological => unreachable!(),
            }
        }
    }
}

impl<'a, 'v> Iterator for DepsetIterator<'a, 'v>
where
    'v: 'a,
{
    type Item = Value<'v>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(state) = self.stack.last_mut() {
            match state {
                IterState::Direct { iter } => {
                    if let Some(&val) = iter.next() {
                        let hash = val.get_hashed().expect("Already verified hashable");
                        if self.visited_elements.insert_hashed(hash) {
                            return Some(val);
                        }
                    } else {
                        self.stack.pop();
                    }
                },
                IterState::Transitive { iter } => {
                    if let Some(&child_val) = iter.next() {
                        if let Some(child_depset) =
                            UnpackDepset::unpack_value(child_val).ok().flatten()
                        {
                            let child_depset: &'v Depset<'v> = child_depset.depset();
                            let node_id = std::ptr::from_ref::<Depset<'v>>(child_depset) as usize;
                            if self.visited_nodes.insert(node_id) {
                                self.push_depset(child_depset, self.order);
                            }
                        }
                    } else {
                        self.stack.pop();
                    }
                },
            }
        }

        if self.order == Order::Topological {
            // Safety: Topo order should always have topo buffer set
            return unsafe { self.topo_buffer.as_mut().unwrap_unchecked() }.pop();
        }

        None
    }
}

impl<'v, V: ValueLike<'v>> DepsetGen<V>
where
    V: starlark::coerce::Coerce<Value<'v>>,
    Self: ProvidesStaticType<'v>,
{
    fn iter_ordered<'a>(&'a self, order: Order) -> DepsetIterator<'a, 'v>
    where
        'v: 'a,
    {
        let mut iter = DepsetIterator {
            stack: Vec::new(),
            visited_nodes: std::collections::HashSet::new(),
            visited_elements: Default::default(),
            order,
            // Topological order requires complete collection before yielding any elements.
            // We just do reverse postorder by collecting postorder to a vec and having iter
            // call vec.pop().
            topo_buffer: if order == Order::Topological {
                Some(self.iter_ordered(Order::Postorder).collect())
            } else {
                None
            },
        };
        if order != Order::Topological {
            let depset: &Depset<'v> = starlark::coerce::coerce(self);
            iter.visited_nodes
                .insert(std::ptr::from_ref::<Depset<'v>>(depset) as usize);
            iter.push_depset(depset, order);
        }
        iter
    }

    /// Returns an iterator over the elements in the depset using its configured
    /// order.
    pub fn iter<'a>(&'a self) -> DepsetIterator<'a, 'v>
    where
        'v: 'a,
    {
        self.iter_ordered(self.order())
    }
}

#[cfg(test)]
mod tests {
    use crate::globals::tests::new_assert;

    #[test]
    fn test_depset() {
        let mut a = new_assert();
        a.equivalent(
            "depset(['c'], transitive=[depset(['a', 'b'])], order='preorder').to_list()",
            "['c', 'a', 'b']",
        );
        a.equivalent(
            "depset(['c'], transitive=[depset(['a', 'b'])], order='postorder').to_list()",
            "['a', 'b', 'c']",
        );
        a.equivalent(
            "depset(['c'], transitive=[depset(['a', 'b'])], order='topological').to_list()",
            "['c', 'b', 'a']",
        );
    }

    #[test]
    fn test_depset_complex_graph() {
        let mut a = new_assert();
        a.equivalent(
            r#"
d1 = depset(['a'])
d2 = depset(['b', 'c'], transitive=[d1])
d3 = depset(['d'], transitive=[d1])
depset(['e'], transitive=[d2, d3], order='preorder').to_list()
"#,
            "['e', 'b', 'c', 'a', 'd']",
        );
        a.equivalent(
            r#"
d1 = depset(['a'])
d2 = depset(['b', 'c'], transitive=[d1])
d3 = depset(['d'], transitive=[d1])
depset(['e'], transitive=[d2, d3], order='postorder').to_list()
"#,
            "['a', 'b', 'c', 'd', 'e']",
        );
        a.equivalent(
            r#"
d1 = depset(['a'])
d2 = depset(['b', 'c'], transitive=[d1])
d3 = depset(['d'], transitive=[d1])
depset(['e'], transitive=[d2, d3], order='topological').to_list()
"#,
            "['e', 'd', 'c', 'b', 'a']",
        );
    }
}
