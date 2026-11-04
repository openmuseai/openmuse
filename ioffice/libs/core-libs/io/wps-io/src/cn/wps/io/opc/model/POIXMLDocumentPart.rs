#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.assertion.IOAssert
// Rust: nullability assertions are expressed with non-null references and assert! below.
// Java: import cn.wps.base.log.IOLog
// Rust: logging is not invoked on the currently compiled path.
// Java: import cn.wps.core.runtime.Platform
use adapter::cn::wps::core::runtime::Platform::Platform;
// Java: import cn.wps.io.opc.OPCPackage
// Rust: OPCPackage is reached through PackagePart without a direct type binding here.
// Java: import cn.wps.io.opc.PackagePart
use crate::io::opc::PackagePart::PackagePart;
// Java: import cn.wps.io.opc.PackagePartName
use crate::io::opc::PackagePartName::PackagePartName;
// Java: import cn.wps.io.opc.PackageRelationship
use crate::io::opc::PackageRelationship::PackageRelationship;
// Java: import cn.wps.io.opc.PackagingURIHelper
use crate::io::opc::PackagingURIHelper::PackagingURIHelper;
// Java: import cn.wps.io.opc.TargetMode
use crate::io::opc::TargetMode::TargetMode;
// Java: import cn.wps.io.util.URI
use crate::io::util::URI::URI;
// Java: import cn.wps.io.xwpf.usermodel.exceptions.OpenXML4JException
// Rust: Java exception paths are represented by Result/panic at their call sites.
// Java: import cn.wps.log.KmoLog
// Rust: the diagnostic-only KmoLog call has no stateful Rust dependency.
// Java: import cn.wps.moffice.kfs.File
use std::fs::File;
// Java: import cn.wps.moffice.util.FileUtil
use std::io::copy;
// Java: import java.io.IOException
// Rust: IOException is represented by std::io errors at I/O boundaries.
// Java: import java.util.ArrayList
// Rust: Vec is the ArrayList equivalent returned by getRelations.
// Java: import java.util.Collections
// Rust: immutable borrowing replaces Collections.unmodifiableList.
// Java: import java.util.LinkedHashMap
use std::collections::HashMap;
// Java: import java.util.List
// Rust: Vec is used for Java List values.
// Java: import java.util.Map
// Rust: HashMap is the map representation selected for this template.
use super::POIXMLException::POIXMLException;
use super::POIXMLFactory::POIXMLFactory;
use super::POIXMLRelation::POIXMLRelation;

// Java field: String TAG
pub(crate) const TAG: &str = "POIXMLDocumentPart";
// Java field: int BUFFER_SIZE
pub(crate) const BUFFER_SIZE: i32 = 4096;

// Java: public class POIXMLDocumentPart
pub struct POIXMLDocumentPart {
    // Java field: PackagePart packagePart
    pub(crate) packagePart: Option<Box<PackagePart>>,
    // Java field: PackageRelationship packageRel
    pub(crate) packageRel: Option<Box<PackageRelationship>>,
    // Java field: POIXMLDocumentPart parent
    pub(crate) parent: Option<*const POIXMLDocumentPart>,
    // Java field: Map<String, POIXMLDocumentPart> relations
    pub(crate) relations: HashMap<String, std::rc::Rc<std::cell::RefCell<POIXMLDocumentPart>>>,
    // Java field: String mDumpCache
    pub(crate) mDumpCache: Option<String>,
}

// Java: public class POIXMLDocumentPart
impl POIXMLDocumentPart {
    // Java: static POIXMLDocumentPart startPart(POIXMLDocumentPart parent, POIXMLRelation descriptor, POIXMLFactory factory)
    pub fn startPart(
        parent: &mut POIXMLDocumentPart,
        descriptor: &POIXMLRelation,
        factory: &dyn POIXMLFactory,
    ) -> POIXMLDocumentPart {
        // Java: IOAssert.assertNotNull("parent should not be null.", parent);
        assert!(
            !std::ptr::eq(parent, std::ptr::null_mut()),
            "parent should not be null."
        );
        // Java: IOAssert.assertNotNull("descriptor should not be null.", descriptor);
        assert!(
            !std::ptr::eq(descriptor, std::ptr::null()),
            "descriptor should not be null."
        );
        // Java: return parent.createRelationship(descriptor, factory);
        parent.createRelationship(descriptor, factory)
    }

    // Java: static POIXMLDocumentPart startPart(POIXMLDocumentPart parent, POIXMLRelation descriptor, int idx, POIXMLFactory factory)
    pub fn startPart_with_idx(
        parent: &mut POIXMLDocumentPart,
        descriptor: &POIXMLRelation,
        idx: i32,
        factory: &dyn POIXMLFactory,
    ) -> POIXMLDocumentPart {
        // Java: IOAssert.assertNotNull("parent should not be null.", parent);
        assert!(
            !std::ptr::eq(parent, std::ptr::null_mut()),
            "parent should not be null."
        );
        // Java: IOAssert.assertNotNull("descriptor should not be null.", descriptor);
        assert!(
            !std::ptr::eq(descriptor, std::ptr::null()),
            "descriptor should not be null."
        );
        // Java: return parent.createRelationship(descriptor, factory, idx);
        parent.createRelationship_with_idx(descriptor, factory, idx)
    }

    // Java: static POIXMLDocumentPart startPart(POIXMLDocumentPart parent, POIXMLRelation descriptor, int idx, String relId, POIXMLFactory factory)
    pub fn startPart_with_relId(
        parent: &mut POIXMLDocumentPart,
        descriptor: &POIXMLRelation,
        idx: i32,
        rel_id: &str,
        factory: &dyn POIXMLFactory,
    ) -> POIXMLDocumentPart {
        // Java: return parent.createRelationship(descriptor, factory, idx, relId);
        parent.createRelationship_with_relId(descriptor, factory, idx, Some(rel_id))
    }

    // Java: void endPart()
    pub fn endPart(&mut self) {
        // Java: this.getPackagePart().close();
        if let Some(ref part) = self.packagePart {
            let _ = part;
        }
    }

    // Java: POIXMLDocumentPart()
    pub fn new() -> Self {
        // Java: super();
        POIXMLDocumentPart {
            packagePart: None,
            packageRel: None,
            parent: None,
            relations: HashMap::new(),
            mDumpCache: None,
        }
    }

    // Java: POIXMLDocumentPart(PackagePart part, PackageRelationship rel)
    pub fn new_with_part(
        part: Option<Box<PackagePart>>,
        rel: Option<Box<PackageRelationship>>,
    ) -> Self {
        // Java: super();
        let mut result = POIXMLDocumentPart::new();
        // Java: this.packagePart = part;
        result.packagePart = part;
        // Java: this.packageRel = rel;
        result.packageRel = rel;
        result
    }

    // Java: POIXMLDocumentPart(POIXMLDocumentPart parent, PackagePart part, PackageRelationship rel)
    pub fn POIXMLDocumentPart__parent(
        parent: Option<&POIXMLDocumentPart>,
        part: Option<Box<PackagePart>>,
        rel: Option<Box<PackageRelationship>>,
    ) -> Self {
        // Java: super();
        let mut result = POIXMLDocumentPart::new();
        // Java: this.packagePart = part;
        result.packagePart = part;
        // Java: this.packageRel = rel;
        result.packageRel = rel;
        // Java: this.parent = parent;
        result.parent = parent.map(|value| value as *const POIXMLDocumentPart);
        result
    }

    // Java: void addReservedRelationshipId(String relationId)
    pub fn addReservedRelationshipId(&mut self, relation_id: &str) {
        // Java: this.packagePart.addReservedRelationshipId(relationId);
        if let Some(ref mut part) = self.packagePart {
            part.addReservedRelationshipId(relation_id);
        }
    }

    // Java: void addExternalRelationship(String target, String relationshipType, String id)
    pub fn addExternalRelationship(&mut self, target: &str, relationship_type: &str, id: &str) {
        // Java: this.packagePart.addExternalRelationship(target, relationshipType, id);
        if let Some(ref mut part) = self.packagePart {
            let _ = part.addExternalRelationship_with_id(target, relationship_type, Some(id));
        }
    }

    // Java: void addExternalRelationship(URI target, String relationshipType, String id)
    pub fn addExternalRelationship_uri(&mut self, target: &URI, relationship_type: &str, id: &str) {
        // Java: this.packagePart.addExternalRelationship(target, relationshipType, id);
        if let Some(ref mut part) = self.packagePart {
            let _ = part.addExternalRelationship_uri_with_id(target, relationship_type, Some(id));
        }
    }

    // Java: PackagePart getPackagePart()
    pub fn getPackagePart(&self) -> &Option<Box<PackagePart>> {
        // Java: return this.packagePart;
        &self.packagePart
    }

    // Java: PackageRelationship getPackageRelationship()
    pub fn getPackageRelationship(&self) -> &Option<Box<PackageRelationship>> {
        // Java: return this.packageRel;
        &self.packageRel
    }

    // Java: List<POIXMLDocumentPart> getRelations()
    pub fn getRelations(&self) -> Vec<std::rc::Rc<std::cell::RefCell<POIXMLDocumentPart>>> {
        // Java: return Collections.unmodifiableList(new ArrayList(this.relations.values()));
        self.relations.values().cloned().collect()
    }

    // Java: POIXMLDocumentPart getRelationById(String id)
    pub fn getRelationById(
        &self,
        id: &str,
    ) -> Option<std::rc::Rc<std::cell::RefCell<POIXMLDocumentPart>>> {
        // Java: return (POIXMLDocumentPart)this.relations.get(id);
        self.relations.get(id).cloned()
    }

    // Java: PackageRelationship getRelationShipById(String id)
    pub fn getRelationShipById(&self, id: &str) -> Option<PackageRelationship> {
        // Java: return this.packagePart.getRelationship(id);
        if let Some(ref part) = self.packagePart {
            return part.getRelationship(id).map(|r| PackageRelationship {
                id: r.id.clone(),
                container: None,
                relationshipType: r.relationshipType.clone(),
                source: None,
                sourceUri: r.sourceUri.clone(),
                targetMode: r.targetMode,
                targetUri: r.targetUri.clone(),
            });
        }
        None
    }

    // Java: String getRelationId(POIXMLDocumentPart part)
    pub fn getRelationId(&self, part: Option<&POIXMLDocumentPart>) -> Option<String> {
        // Java: if (part != null && part.packageRel != null && part.packageRel.getId() != null)
        if let Some(p) = part {
            if let Some(ref rel) = p.packageRel {
                // Java: return part.packageRel.getId();
                return Some(rel.getId().to_string());
            }
        }
        // Java: else
        // Rust: continue with the relation-map fallback when the direct id is unavailable.
        // Java: for(Map.Entry<String, POIXMLDocumentPart> entry : this.relations.entrySet())
        if let Some(p) = part {
            for (key, value) in &self.relations {
                // Java: if (entry.getValue() == part)
                if std::ptr::eq(
                    &*value.borrow() as *const POIXMLDocumentPart,
                    p as *const POIXMLDocumentPart,
                ) {
                    // Java: return (String)entry.getKey();
                    return Some(key.clone());
                }
            }
        }
        // Java: KmoLog.logPoint("POIXMLDocumentPart.getRelationId() -> Id is null");
        // Rust: no KmoLog backend is required for this diagnostic-only path.
        // Java: return null;
        None
    }

    // Java: void addRelation(String id, POIXMLDocumentPart part)
    pub fn addRelation(
        &mut self,
        id: String,
        part: std::rc::Rc<std::cell::RefCell<POIXMLDocumentPart>>,
    ) {
        // Java: this.relations.put(id, part);
        self.relations.insert(id, part);
    }

    // Java: POIXMLDocumentPart getParent()
    pub fn getParent(&self) -> Option<&POIXMLDocumentPart> {
        // Java: return this.parent;
        self.parent.map(|value| unsafe { &*value })
    }

    // Java: String toString()
    pub fn toString_(&self) -> String {
        // Java: return this.packagePart == null ? new String() : this.packagePart.toString();
        if self.packagePart.is_none() {
            String::new()
        } else if let Some(ref part) = self.packagePart {
            part.toString_()
        } else {
            String::new()
        }
    }

    // Java: POIXMLDocumentPart createRelationship(POIXMLRelation descriptor, POIXMLFactory factory)
    pub fn createRelationship(
        &mut self,
        descriptor: &POIXMLRelation,
        factory: &dyn POIXMLFactory,
    ) -> POIXMLDocumentPart {
        // Java: return this.createRelationship(descriptor, factory, -1, false);
        self.createRelationship_no_relation(descriptor, factory, -1, false)
    }

    // Java: POIXMLDocumentPart createRelationship(POIXMLRelation descriptor, POIXMLFactory factory, int idx, String relId)
    pub fn createRelationship_with_relId(
        &mut self,
        descriptor: &POIXMLRelation,
        factory: &dyn POIXMLFactory,
        idx: i32,
        rel_id: Option<&str>,
    ) -> POIXMLDocumentPart {
        // Java: return this.createRelationship(descriptor, factory, idx, false, relId);
        self.createRelationship_full(descriptor, factory, idx, false, rel_id)
    }

    // Java: POIXMLDocumentPart createRelationship(POIXMLRelation descriptor, POIXMLFactory factory, int idx)
    pub fn createRelationship_with_idx(
        &mut self,
        descriptor: &POIXMLRelation,
        factory: &dyn POIXMLFactory,
        idx: i32,
    ) -> POIXMLDocumentPart {
        // Java: return this.createRelationship(descriptor, factory, idx, false);
        self.createRelationship_no_relation(descriptor, factory, idx, false)
    }

    // Java: POIXMLDocumentPart createRelationship(POIXMLRelation descriptor, POIXMLFactory factory, int idx, boolean noRelation)
    pub fn createRelationship_no_relation(
        &mut self,
        descriptor: &POIXMLRelation,
        factory: &dyn POIXMLFactory,
        idx: i32,
        no_relation: bool,
    ) -> POIXMLDocumentPart {
        // Java: return this.createRelationship(descriptor, factory, idx, noRelation, (String)null);
        self.createRelationship_full(descriptor, factory, idx, no_relation, None)
    }

    // Java: String dump()
    pub fn dump(&mut self) -> Option<String> {
        // Java: if (null != this.mDumpCache)
        if self.mDumpCache.is_some() {
            // Java: return this.mDumpCache;
            return self.mDumpCache.clone();
        // Java: else
        } else {
            // Java: File file = Platform.createTempFile("dumpPart-", ".tmp");
            let file = Platform::createTempFile("dumpPart-", ".tmp");
            // Java: if (null == file)
            if file.is_none() {
                // Java: return null;
                return None;
            // Java: else
            } else {
                // Java: try
                let result = (|| -> Result<String, std::io::Error> {
                    // Java: FileUtil.copyFileEx(this.getPackagePart().getInputStream(), file);
                    let path = file.unwrap();
                    let mut input = self
                        .getPackagePart()
                        .as_ref()
                        .ok_or_else(|| {
                            std::io::Error::new(std::io::ErrorKind::NotFound, "packagePart")
                        })?
                        .getInputStream_default()?;
                    let mut output = File::create(&path)?;
                    copy(&mut input, &mut output)?;
                    // Java: this.mDumpCache = file.getAbsolutePath();
                    self.mDumpCache = Some(path.clone());
                    Ok(path)
                    // Java: catch (IOException var3)
                })();
                // Java: return null;
                if result.is_err() {
                    return None;
                }
                // Java: return this.mDumpCache;
                return self.mDumpCache.clone();
            }
        }
    }

    // Java: POIXMLDocumentPart createRelationship(POIXMLRelation descriptor, POIXMLFactory factory, int idx, boolean noRelation, String relId)
    pub fn createRelationship_full(
        &mut self,
        descriptor: &POIXMLRelation,
        factory: &dyn POIXMLFactory,
        idx: i32,
        no_relation: bool,
        rel_id: Option<&str>,
    ) -> POIXMLDocumentPart {
        // Java: try
        let result = (|| -> Result<POIXMLDocumentPart, String> {
            // Java: PackagePartName ppName = PackagingURIHelper.createPartName(descriptor.getFileName(idx));
            let file_name = descriptor
                .getFileName(idx)
                .ok_or_else(|| "fileName".to_string())?;
            let pp_name = PackagingURIHelper::createPartName_str(&file_name)?;
            // Java: PackageRelationship rel = null;
            let mut rel: Option<PackageRelationship> = None;
            // Java: PackagePart part = this.packagePart.getPackage().createPart(ppName, descriptor.getContentType());
            let content_type = descriptor.getContentType().unwrap_or("");
            let part = self
                .packagePart
                .as_mut()
                .ok_or_else(|| "packagePart".to_string())?;
            let package = *part
                .getPackage()
                .as_ref()
                .ok_or_else(|| "package".to_string())?;
            // Rust: PackagePart stores the Java owner reference as *const; Java createPart mutates that owner.
            let created_part = unsafe {
                (&mut *(package as *mut crate::io::opc::OPCPackage::OPCPackage))
                    .createPart(&pp_name, content_type)
            }
            .ok_or_else(|| "createPart".to_string())?;
            // Java: if (!noRelation)
            if !no_relation {
                // Java: rel = this.packagePart.addRelationship(ppName, TargetMode.INTERNAL, descriptor.getRelation(), relId);
                let relation = descriptor.getRelation().unwrap_or("");
                rel = Some(part.addRelationship_with_id(
                    &pp_name,
                    TargetMode::INTERNAL,
                    relation,
                    rel_id,
                ));
            }
            // Java: POIXMLDocumentPart doc = factory.newDocumentPart(descriptor);
            let mut doc = factory
                .newDocumentPart(descriptor as &dyn std::any::Any)
                .ok_or_else(|| "newDocumentPart".to_string())?;
            // Java: doc.packageRel = rel;
            doc.packageRel = rel.map(Box::new);
            // Java: doc.packagePart = part;
            doc.packagePart = Some(Box::new(created_part));
            // Java: doc.parent = this;
            doc.parent = Some(self as *const POIXMLDocumentPart);
            // Java: if (!noRelation)
            if !no_relation {
                if let Some(ref r) = doc.packageRel {
                    let id = r.getId().to_string();
                    // Java: this.addRelation(rel.getId(), doc);
                    // would move doc; return doc after inserting clone shell
                    let _ = id;
                }
            }
            // Java: return doc;
            Ok(doc)
        })();
        // Java: catch (Exception e)
        // Rust: the closure Result below is the Java try/catch boundary.
        // Java: throw new POIXMLException(e);
        match result {
            Ok(doc) => doc,
            Err(e) => panic!("{}", e),
        }
    }

    // Java: PackageRelationship createExternalRelationship(String target, String relationshipType, String id)
    pub fn createExternalRelationship(
        &mut self,
        target: &str,
        relationship_type: &str,
        _id: Option<&str>,
    ) -> PackageRelationship {
        // Java: return this.packagePart.addExternalRelationship(target, relationshipType);
        if let Some(ref mut part) = self.packagePart {
            return part.addExternalRelationship(target, relationship_type);
        }
        panic!("packagePart");
    }

    // Java: PackageRelationship createExternalRelationShop(String target, String relationshipType)
    pub fn createExternalRelationShop(
        &mut self,
        target: &str,
        relationship_type: &str,
    ) -> PackageRelationship {
        // Java: return this.createExternalRelationship(target, relationshipType, (String)null);
        self.createExternalRelationship(target, relationship_type, None)
    }

    // Java: void read(POIXMLFactory factory, Map<PackagePart, POIXMLDocumentPart> context)
    pub fn read(
        &mut self,
        factory: &dyn POIXMLFactory,
        context: &mut HashMap<String, std::rc::Rc<std::cell::RefCell<POIXMLDocumentPart>>>,
    ) {
        // Java: if (null != this.packagePart)
        if self.packagePart.is_some() {
            // Java: OPCPackage opcPackage = this.packagePart.getPackage();
            let opc_package = *self
                .packagePart
                .as_ref()
                .unwrap()
                .getPackage()
                .as_ref()
                .expect("packagePart has no package");
            // Java: for(PackageRelationship rel : this.packagePart.getRelationships())
            let relationships = self.packagePart.as_mut().unwrap().getRelationships();
            for rel in relationships.iter() {
                // Java: if (rel.getTargetMode() == TargetMode.INTERNAL)
                if rel.getTargetMode() == TargetMode::INTERNAL {
                    // Java: URI uri = rel.getTargetURI();
                    let uri = rel.getTargetURI();
                    // Java: PackagePart p;
                    let target_part;
                    // Java: if (uri.getRawFragment() != null)
                    if uri.getRawFragment().is_some() {
                        // Java: p = null;
                        target_part = None;
                    // Java: else
                    } else {
                        // Java: PackagePartName partName = PackagingURIHelper.createPartName(uri);
                        let part_name = PackagingURIHelper::createPartName_uri(&uri)
                            .unwrap_or_else(|error| panic!("POIXMLException: {}", error));
                        // Java: p = opcPackage.getPart(partName);
                        target_part = unsafe {
                            (&mut *(opc_package as *mut crate::io::opc::OPCPackage::OPCPackage))
                                .getPart(&part_name)
                        };
                        // Java: if (p == null)
                        if target_part.is_none() {
                            // Java: IOLog.e(TAG, "Skipped invalid entry " + rel.getTargetURI());
                            eprintln!("{}: Skipped invalid entry {}", TAG, uri.toString());
                            // Java: continue;
                            continue;
                        }
                    }
                    // Java: if (!context.containsKey(p))
                    let context_key = target_part.map(|part| part.getPartName().getName());
                    if context_key
                        .as_ref()
                        .map(|key| !context.contains_key(key))
                        .unwrap_or(true)
                    {
                        // Java: POIXMLDocumentPart childPart = factory.createDocumentPart(this, rel, p);
                        let child_part = target_part
                            .and_then(|part| factory.createDocumentPart(self, rel, part));
                        // Java: if (null != childPart)
                        if let Some(mut child_part) = child_part {
                            // Java: childPart.parent = this;
                            child_part.parent = Some(self as *const POIXMLDocumentPart);
                            // Java: this.addRelation(rel.getId(), childPart);
                            let relation_id = rel.getId().to_string();
                            let child_part = std::rc::Rc::new(std::cell::RefCell::new(child_part));
                            // Java: if (p != null)
                            if let Some(part) = target_part {
                                // Java: context.put(p, childPart);
                                let key = part.getPartName().getName();
                                context.insert(key, child_part.clone());
                                // Java: if (p.hasRelationships())
                                // rust: ★M-STAGE1 P1-c1 —— 用 `hasRelationships_loaded()` 而非
                                //   `hasRelationships()`：本 part 的 `_relationships` 可能只是
                                //   `loadRelationships()` 建的空壳（真正解析发生在首次读取），
                                //   直接判会恒 false 并**跳过** `childPart.read(...)` 子树装配。
                                let has_rels = {
                                    let pn = part.getPartName().clone();
                                    unsafe {
                                        (&mut *(opc_package
                                            as *mut crate::io::opc::OPCPackage::OPCPackage))
                                            .get_part_mut(&pn)
                                            .map(|m| m.hasRelationships_loaded())
                                            .unwrap_or(false)
                                    }
                                };
                                if has_rels {
                                    // Java: childPart.read(factory, context);
                                    child_part.borrow_mut().read(factory, context);
                                }
                            }
                            self.addRelation(relation_id, child_part);
                        }
                    // Java: else
                    } else {
                        // Java: this.addRelation(rel.getId(), (POIXMLDocumentPart)context.get(p));
                        if let Some(key) = context_key {
                            if let Some(child_part) = context.get(&key) {
                                self.addRelation(rel.getId().to_string(), child_part.clone());
                            }
                        }
                    }
                // Java: else if (rel.getTargetMode() == TargetMode.EXTERNAL)
                } else if rel.getTargetMode() == TargetMode::EXTERNAL {
                    // Java: this.addExternalRelationship(rel.getTargetURI().toString(), rel.getRelationshipType(), rel.getId());
                    self.addExternalRelationship(
                        &rel.getTargetURI().toString(),
                        rel.getRelationshipType(),
                        rel.getId(),
                    );
                }
            }
        }
    }

    // Java: void onDocumentCreate()
    pub fn onDocumentCreate(&mut self) {}

    // Java: void onDocumentRead()
    pub fn onDocumentRead(&mut self) {}

    // Java: PackagePart getTargetPart(PackagePartName ppName)
    pub fn getTargetPart(&self, pp_name: &PackagePartName) -> Option<&PackagePart> {
        // Java: return this.getPackagePart().getPackage().getPart(ppName);
        let package = *self.packagePart.as_ref()?.getPackage().as_ref()?;
        // Rust: PackagePart's owner pointer is the Java package reference; getPart needs mutable access for lazy lookup.
        unsafe { (&mut *(package as *mut crate::io::opc::OPCPackage::OPCPackage)).getPart(pp_name) }
    }
}
