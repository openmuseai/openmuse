// Java: public class Xml07Context
// The part stack and PartContext graph stay off this compile surface.
// importBlip's xml07 branch still records the CoBlipFill identity Java constructs:
// shape id, document type, target type 0, and the embed rId.
#[derive(Default, Clone, Debug)]
pub struct Xml07Context {
    // Java: Stack<PartInfo> mPartInfos. Empty until a real xml07 part push exists.
    part_owners: Vec<String>,
    // Java: collectRid2CoreObject(info, new CoBlipFill(shapeId, docType, 0))
    blip_core_objects: Vec<Xml07BlipCore>,
}

// Java: CoBlipFill(shapeId, docType, targetType) plus ClRefInfo.mRid.
#[derive(Clone, Debug)]
pub struct Xml07BlipCore {
    pub mRid: Option<String>,
    pub mShapeId: i32,
    pub mDocType: i32,
    pub mTargetType: i32,
    pub mOwnerPart: String,
}

impl Xml07Context {
    // Java: public PartInfo peekPartInfo()
    pub fn peekPartInfo(&self) -> Option<&str> {
        // Java: return this.mPartInfos.size() > 0 ? this.mPartInfos.peek() : null;
        self.part_owners.last().map(|owner| owner.as_str())
    }

    // Java: public void collectRid2CoreObject(ClRefInfo info, ICoreObject co)
    pub fn collectRid2CoreObject(&mut self, mRid: Option<String>, shape_id: i32, doc_type: i32, target_type: i32) {
        // Java: PartInfo partInfo = this.peekPartInfo();
        // Java: String ownerPart = partInfo.getOwnerPart();
        let owner_part = self.peekPartInfo().expect("partInfo").to_string();
        // Java: if (null != partInfo && null != ownerPart && null != info && null != co)
        if mRid.is_none() {
            return;
        }
        // Java: co.setOwnerPart(ownerPart); partCxt.putRid2CoreObject(info, co);
        self.blip_core_objects.push(Xml07BlipCore {
            mRid,
            mShapeId: shape_id,
            mDocType: doc_type,
            mTargetType: target_type,
            mOwnerPart: owner_part,
        });
    }
}
