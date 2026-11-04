// java package cn.wps.moss.xlsxr.util;

// java import cn.wps.kfc.ooxfr.OxfXlRelations;
// java import cn.wps.kfc.ooxfr.OxfrPackage;
// java import cn.wps.kfc.opcr.OpcrPackagePart;
// java import cn.wps.kfc.opcr.OpcrRelationship;
// java import cn.wps.kfc.opcr.OpcrRelationships;
// java import cn.wps.moffice.kfs.File;
// java import cn.wps.moffice.kfs.RandomAccessFile;
// java import cn.wps.moss.io.MemoryToolKit;
// java import cn.wps.moss.io.except.FileNotSupportException;
// java import cn.wps.moss.io.except.XlsbException;
use std::sync::atomic::{AtomicBool, Ordering};

use kfc_kfc_alpha::kfc::ooxfr::OxfrPackage::OxfrPackage;
use kfc_kfc_alpha::kfc::opcr::OpcrPackagePart::OpcrPackagePart;
use kfc_kfc_alpha::kfc::opcr::OpcrRelationship::OpcrRelationship;
use kfc_kfc_alpha::kfc::opcr::OpcrRelationships::OpcrRelationships;

// java public final class XlsxrPackage
pub struct XlsxrPackage;

// java private static boolean bLargestFile = false;
static bLargestFile: AtomicBool = AtomicBool::new(false);

// java public final class XlsxrPackage
impl XlsxrPackage {
    // java public static void initLargestFile(String path, RandomAccessFile raf, OxfrPackage pack)
    pub fn initLargestFile() {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (null != path || null != raf)
        // java long fileSize = 0L
        // java if (null != raf)
        // java fileSize = raf.length()
        // java else
        // java fileSize = (new File(path)).length()
        // java long maxMemory = Runtime.getRuntime().maxMemory()
        // java int packMaxMemory = 0
        // java try {
        // java packMaxMemory = pack.getMaxMemory()
        // java } catch (IllegalArgumentException var10)
        // java if (fileSize >= 5242880L || (long)packMaxMemory >= maxMemory)
        // java bLargestFile = true
        // java if (bLargestFile && maxMemory >= 268435456L && MemoryToolKit.isLargeMemoryDevice())
        // java bLargestFile = false
        // java if (bLargestFile)
        // java long xmlSize = fileSize - (long)pack.getMediasSize()
        // java if (xmlSize < 2097152L)
        // java bLargestFile = false
        */
    }

    // java public static boolean isLargestFile()
    pub fn isLargestFile() -> bool {
        // java return bLargestFile;
        bLargestFile.load(Ordering::Relaxed)
    }

    // java public static void setLargestFile()
    pub fn setLargestFile() {
        // java bLargestFile = true;
        bLargestFile.store(true, Ordering::Relaxed);
    }

    // java public static boolean isXlsb(OpcrPackagePart bookPart)
    pub fn isXlsb(book_part: &OpcrPackagePart) -> bool {
        // java return bookPart.getPartName().endsWith(".bin");
        book_part.getPartName().ends_with(".bin")
    }

    // java public static OpcrPackagePart getBookPart(OxfrPackage pack)
    pub fn getBookPart(pack: &mut OxfrPackage) -> &mut OpcrPackagePart {
        // java OpcrPackagePart bookPart = pack.getOfficeDocument();
        // java if (bookPart == null)
        if pack.getOfficeDocument_mut().is_none() {
            // java throw new FileNotSupportException("It is not a OOXML file.");
            panic!("FileNotSupportException: It is not a OOXML file.");
        }
        // java else if (bookPart.getPartName().endsWith(".bin"))
        if pack
            .getOfficeDocument_mut()
            .unwrap()
            .getPartName()
            .ends_with(".bin")
        {
            // java throw new XlsbException("It is a .xlsb file.");
            panic!("XlsbException: It is a .xlsb file.");
        }
        // java return bookPart;
        pack.getOfficeDocument_mut().unwrap()
    }

    // java public static OpcrRelationship getChildRelationshipByType(OpcrPackagePart part, String relsType)
    pub fn getChildRelationshipByType<'a>(
        part: Option<&'a mut OpcrPackagePart>,
        rels_type: &str,
    ) -> Result<Option<&'a OpcrRelationship>, std::io::Error> {
        // java if (part == null)
        let Some(part) = part else {
            // java return null;
            return Ok(None);
        };
        // java OpcrRelationships relses = part.getRelationships();
        let Some(relses) = part.getRelationships()? else {
            // java return relses == null ? null : relses.getRelationshipByType(relsType);
            return Ok(None);
        };
        // java return relses == null ? null : relses.getRelationshipByType(relsType);
        Ok(relses.getRelationshipByType(rels_type))
    }

    // java public static OpcrPackagePart getChildPartByType(OpcrPackagePart part, String relsType)
    pub fn getChildPartByType<'a>(
        part: Option<&'a mut OpcrPackagePart>,
        rels_type: &str,
    ) -> Result<Option<&'a OpcrPackagePart>, std::io::Error> {
        // java if (part == null)
        let Some(part) = part else {
            // java return null;
            return Ok(None);
        };
        // java OpcrRelationships relses = part.getRelationships();
        let Some(relses) = part.getRelationships_mut()? else {
            // java if (relses == null)
            // java return null;
            return Ok(None);
        };
        // java OpcrRelationship rels = relses.getRelationshipByType(relsType);
        let found = Self::index_by_type(relses, rels_type);
        // java return rels == null ? null : rels.getPackagePart();
        let Some(index) = found else {
            return Ok(None);
        };
        Ok(relses.getRelationship_mut(index).getPackagePart())
    }

    // java public static OpcrRelationship getChildRelationshipByRid(OpcrPackagePart part, String rid)
    pub fn getChildRelationshipByRid<'a>(
        part: Option<&'a mut OpcrPackagePart>,
        rid: &str,
    ) -> Result<Option<&'a OpcrRelationship>, std::io::Error> {
        // java if (part == null)
        let Some(part) = part else {
            // java return null;
            return Ok(None);
        };
        // java OpcrRelationships relses = part.getRelationships();
        let Some(relses) = part.getRelationships()? else {
            // java return relses == null ? null : relses.getRelationship(rid);
            return Ok(None);
        };
        // java return relses == null ? null : relses.getRelationship(rid);
        Ok(relses.getRelationship__rid(rid))
    }

    // java public static OpcrPackagePart getChildPartByRid(OpcrPackagePart part, String rid)
    pub fn getChildPartByRid<'a>(
        part: Option<&'a mut OpcrPackagePart>,
        rid: &str,
    ) -> Result<Option<&'a OpcrPackagePart>, std::io::Error> {
        // java if (part == null)
        let Some(part) = part else {
            // java return null;
            return Ok(None);
        };
        // java OpcrRelationships relses = part.getRelationships();
        let Some(relses) = part.getRelationships_mut()? else {
            // java if (relses == null)
            // java return null;
            return Ok(None);
        };
        // java OpcrRelationship rels = relses.getRelationship(rid);
        let found = Self::index_by_rid(relses, rid);
        // java return null == rels ? null : rels.getPackagePart();
        let Some(index) = found else {
            return Ok(None);
        };
        Ok(relses.getRelationship_mut(index).getPackagePart())
    }

    // java public static List<String> getRidsByType(OpcrPackagePart bookPart, String type)
    pub fn getRidsByType(
        book_part: &mut OpcrPackagePart,
        type_: &str,
    ) -> Result<Vec<Option<String>>, std::io::Error> {
        // java List<String> ridsList = new ArrayList();
        let mut ridsList = Vec::new();
        // java OpcrRelationships relses = bookPart.getRelationships();
        let relses = book_part
            .getRelationships()?
            .unwrap_or_else(|| panic!("NullPointerException"));
        // java int i = 0;
        let mut i = 0;
        // java for(int size = relses.size(); i < size; ++i)
        let size = relses.size();
        while i < size {
            // java OpcrRelationship rels = relses.getRelationship(i);
            let rels = relses.getRelationship(i);
            // java if (rels.getType().equals(type))
            let rel_type = rels
                .getType()
                .unwrap_or_else(|| panic!("NullPointerException"));
            if rel_type == type_ {
                // java ridsList.add(rels.getId());
                ridsList.push(rels.getId());
            }
            i += 1;
        }
        // java return ridsList;
        Ok(ridsList)
    }

    // java public static Map<String, String> getHyperTargetMap(OpcrPackagePart sheetPart)
    pub fn getHyperTargetMap() {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java OpcrRelationships relses = sheetPart.getRelationships()
        // java if (relses == null)
        // java return null
        // java HashMap<String, String> map = new HashMap()
        // java int i = 0
        // java for(int size = relses.size(); i < size; ++i)
        // java OpcrRelationship rels = relses.getRelationship(i)
        // java String type = rels.getType()
        // java if (OxfXlRelations.HYPERLINK.getRelsType().equals(type))
        // java String value = rels.getTarget()
        // java if (value.indexOf(92) != -1)
        // java value = value.replaceAll("\\\\", "/")
        // java map.put(rels.getId(), value)
        // java return map
        */
    }

    // java public static HashMap<String, OpcrPackagePart> getChildPartsMapByType(OpcrPackagePart drawPart, String relsType)
    pub fn getChildPartsMapByType() {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java OpcrRelationships relses = drawPart.getRelationships()
        // java if (relsType != null && relses != null && relses.size() != 0)
        // java HashMap<String, OpcrPackagePart> map = null
        // java int i = 0
        // java for(int count = relses.size(); i < count; ++i)
        // java OpcrRelationship rels = relses.getRelationship(i)
        // java if (relsType.equals(rels.getType()))
        // java if (map == null)
        // java map = new HashMap()
        // java map.put(rels.getId(), rels.getPackagePart())
        // java return map
        // java else
        // java return null
        */
    }

    // java public static HashMap<String, OpcrPackagePart> getDiagramsPart(OpcrPackagePart drawPart)
    pub fn getDiagramsPart() {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java OpcrRelationships relses = drawPart.getRelationships()
        // java if (relses != null && relses.size() != 0)
        // java HashMap<String, OpcrPackagePart> map = null
        // java int i = 0
        // java for(int count = relses.size(); i < count; ++i)
        // java OpcrRelationship rels = relses.getRelationship(i)
        // java String type = rels.getType()
        // java if (isDiagramsRelation(type))
        // java if (map == null)
        // java map = new HashMap()
        // java map.put(rels.getId(), rels.getPackagePart())
        // java return map
        // java else
        // java return null
        */
    }

    fn index_by_type(relses: &OpcrRelationships, rels_type: &str) -> Option<i32> {
        let size = relses.size();
        let mut i = 0;
        while i < size {
            if relses.getRelationship(i).getType().as_deref().expect("NullPointerException: relationship Type") == rels_type {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    fn index_by_rid(relses: &OpcrRelationships, rid: &str) -> Option<i32> {
        let size = relses.size();
        let mut i = 0;
        while i < size {
            if relses.getRelationship(i).getId().as_deref().expect("NullPointerException: relationship Id") == rid {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    // java private static boolean isDiagramsRelation(String type)
    fn isDiagramsRelation() {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return type.equals(OxfXlRelations.diagramQuickStyle().getRelsType()) || type.equals(OxfXlRelations.diagramLayout().getRelsType()) || type.equals(OxfXlRelations.diagramDrawing().getRelsType()) || type.equals(OxfXlRelations.diagramData().getRelsType()) || type.equals(OxfXlRelations.diagramColors().getRelsType())
        */
    }
}
