#![allow(non_snake_case)]

// Java: import cn.wps.base.assertion.Assert
use core_libs_util::cn::wps::base::assertion::Assert::Assert;
// Java: import java.util.HashMap
// Java: import java.util.Map
use std::collections::HashMap;
use super::RIDNode::RIDNode;

// Java: public class DrawingRIDMap
pub struct DrawingRIDMap {
    // Java field: Map<String, RIDNode> mIdFilePathMap
    pub(crate) mIdFilePathMap: HashMap<String, RIDNode>,
}

// Java: public class DrawingRIDMap
impl DrawingRIDMap {
    // Java: DrawingRIDMap()
    pub fn new() -> Self {
        // Java: super();
        DrawingRIDMap {
            mIdFilePathMap: HashMap::new(),
        }
    }

    // Java: public Map<String, RIDNode> getMap()
    pub fn getMap(&mut self) -> &mut HashMap<String, RIDNode> {
        // Java: return this.mIdFilePathMap;
        &mut self.mIdFilePathMap
    }

    // Java: void add(Map<String, RIDNode> map)
    pub fn add_map(&mut self, map: HashMap<String, RIDNode>) {
        // Java: Assert.assertNotNull("map should not be null", map);
        Assert::assertNotNull_msg("map should not be null", &Some(&map));
        // Java: this.mIdFilePathMap.putAll(map);
        self.mIdFilePathMap.extend(map);
    }

    // Java: void add(String relId, RIDNode node)
    pub fn add(&mut self, relId: String, node: RIDNode) {
        // Java: this.mIdFilePathMap.put(relId, node);
        self.mIdFilePathMap.insert(relId, node);
    }

    // Java: RelationshipCollection getRelationshipCollection(String relId)
    pub fn getRelationshipCollection(&self, relId: &str) -> Option<()> {
        // Java: return ((RIDNode)this.mIdFilePathMap.get(relId)).relationshipCollection;
        todo!()
    }

    // Java: String getPath(String relId)
    pub fn getPath(&self, relId: &str) -> Option<&str> {
        // Java: return ((RIDNode)this.mIdFilePathMap.get(relId)).path;
        todo!()
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: for(RIDNode node : this.mIdFilePathMap.values())
        for (_key, node) in self.mIdFilePathMap.iter_mut() {
            // Java: node.dispose();
            node.dispose();
        }
        // Java: this.mIdFilePathMap.clear();
        self.mIdFilePathMap.clear();
    }
}
