use kfc_kfc_alpha::kfc::{ooxfr::OxfXlRelations::{self, OxfXlRelations as X}, ooxfw::OxfwPackage::OxfwPackage, opcw::{OpcwPackagePart::OpcwPackagePart, OpcwRelationships::OpcwRelationships, native::{PartRef, RelationRef, ZipSink, ZipEntry, IListener, DeflateDataCallBack}}};

// java public final class XlsxwPackage {
pub struct XlsxwPackage;
impl XlsxwPackage {
    // java public static OpcwPackagePart createBookPart(OxfwPackage pack, boolean bTemplate, boolean bMacroEnabled) {
    pub fn createBookPart(pack: &mut OxfwPackage, bTemplate: bool, bMacroEnabled: bool) -> PartRef {
        // java OpcRelation rel = null;
        let rel: RelationRef;
        // java if (bTemplate && bMacroEnabled) { rel = OxfXlRelations.MACRO_TEMPLATE_WORKBOOK;
        if bTemplate && bMacroEnabled { rel = RelationRef::Static(&*OxfXlRelations::MACRO_TEMPLATE_WORKBOOK); }
        // java } else if (bTemplate) { rel = OxfXlRelations.TEMPLATE_WORKBOOK;
        else if bTemplate { rel = RelationRef::Static(&*OxfXlRelations::TEMPLATE_WORKBOOK); }
        // java } else if (bMacroEnabled) { rel = OxfXlRelations.MACROS_WORKBOOK;
        else if bMacroEnabled { rel = RelationRef::Static(&*OxfXlRelations::MACROS_WORKBOOK); }
        // java } else { rel = OxfXlRelations.WORKBOOK; }
        else { rel = RelationRef::Static(&*OxfXlRelations::WORKBOOK); }
        // java return pack.createOfficeDocument(rel);
        pack.createOfficeDocument(rel)
    }

    // java public static OutputStream getOutputStream(OpcwPackagePart part) {
    pub fn getOutputStream(part: &mut OpcwPackagePart) -> Option<ZipSink> {
        // java try { return part.getOutputStream(); } catch (IOException e) { throw new RuntimeException(e); }
        part.getOutputStream().unwrap_or_else(|e| crate::java_runtime::RuntimeException::RuntimeException::throw(e))
    }
    // java public static OutputStream getOutputStream(OpcwPackagePart part, ZipEntry entry, ZipOutputStream.IListener listener, ZipOutputStream.DeflateDataCallBack deflateDataCallBack) {
    pub fn getOutputStream__4(part: &mut OpcwPackagePart, entry: Option<&dyn ZipEntry>, listener: Option<Box<dyn IListener>>, deflateDataCallBack: Option<Box<dyn DeflateDataCallBack>>) -> Option<ZipSink> {
        // java try { return part.getOutputStream(entry, listener, deflateDataCallBack); } catch (IOException e) { throw new RuntimeException(e); }
        part.getOutputStream__3(entry, listener, deflateDataCallBack).unwrap_or_else(|e| crate::java_runtime::RuntimeException::RuntimeException::throw(e))
    }
    // java public static OutputStream getOutputStream(OpcwPackagePart part, ZipEntry entry) { return getOutputStream(part, entry, (ZipOutputStream.IListener)null, (ZipOutputStream.DeflateDataCallBack)null); }
    pub fn getOutputStream__2(part: &mut OpcwPackagePart, entry: Option<&dyn ZipEntry>) -> Option<ZipSink> {
        Self::getOutputStream__4(part, entry, None, None)
    }
    // java public static OutputStream getOutputStream(OpcwPackagePart part, ZipEntry entry, ZipOutputStream.IListener listener) { return getOutputStream(part, entry, listener, (ZipOutputStream.DeflateDataCallBack)null); }
    pub fn getOutputStream__3(part: &mut OpcwPackagePart, entry: Option<&dyn ZipEntry>, listener: Option<Box<dyn IListener>>) -> Option<ZipSink> {
        Self::getOutputStream__4(part, entry, listener, None)
    }
    // java public static OpcwPackagePart createCellImagesPart(OpcwRelationships relses) {
    pub fn createCellImagesPart(relses: &mut OpcwRelationships) -> PartRef {
        // java OpcwRelationship rels = relses.addRelationship(OxfXlRelations.CELL_IMAGES);
        let rels = relses.addRelationship(RelationRef::Static(&*OxfXlRelations::CELL_IMAGES));
        // java return rels.createPart();
        let part = rels.borrow_mut().createPart(); part
    }
    // java public static OpcwPackagePart createStylesPart(OpcwRelationships relses) {
    pub fn createStylesPart(relses: &mut OpcwRelationships) -> PartRef {
        // java OpcwRelationship rels = relses.addRelationship(OxfXlRelations.STYLES);
        let rels = relses.addRelationship(RelationRef::Static(&*OxfXlRelations::STYLES));
        // java return rels.createPart();
        let part = rels.borrow_mut().createPart(); part
    }
    // java public static OpcwPackagePart createStringsPart(OpcwRelationships relses) {
    pub fn createStringsPart(relses: &mut OpcwRelationships) -> PartRef {
        // java OpcwRelationship rels = relses.addRelationship(OxfXlRelations.SHARED_STRINGS);
        let rels = relses.addRelationship(RelationRef::Static(&*OxfXlRelations::SHARED_STRINGS));
        // java return rels.createPart();
        let part = rels.borrow_mut().createPart(); part
    }
    // java public static OpcwPackagePart createThemePart(OpcwRelationships relses) {
    pub fn createThemePart(relses: &mut OpcwRelationships) -> PartRef {
        // java OpcwRelationship rels = relses.addRelationship(OxfXlRelations.theme(), 1);
        let rels = relses.addRelationship__2(X::theme(), 1);
        // java return rels.createPart();
        let part = rels.borrow_mut().createPart(); part
    }
    // java public static OpcwPackagePart createThemeOverridePart(OpcwRelationships relses, int relIndex) {
    pub fn createThemeOverridePart(relses: &mut OpcwRelationships, relIndex: i32) -> PartRef {
        // java OpcwRelationship rels = relses.addRelationship(OxfXlRelations.themeOverride(), relIndex);
        let rels = relses.addRelationship__2(X::themeOverride(), relIndex);
        // java return rels.createPart();
        let part = rels.borrow_mut().createPart(); part
    }
    // java public static OpcwPackagePart createItems(OpcwRelationships relses, int relIndex) {
    pub fn createItems(relses: &mut OpcwRelationships, relIndex: i32) -> PartRef {
        // java OpcwRelationship rels = relses.addRelationship(OxfXlRelations.ITEM, relIndex);
        let rels = relses.addRelationship__2(RelationRef::Static(&*OxfXlRelations::ITEM), relIndex);
        // java return rels.createPart();
        let part = rels.borrow_mut().createPart(); part
    }
    // java public static OpcwPackagePart createItemProps(OpcwRelationships relses, int relIndex) {
    pub fn createItemProps(relses: &mut OpcwRelationships, relIndex: i32) -> PartRef {
        // java OpcwRelationship rels = relses.addRelationship(OxfXlRelations.ITEMPROPS, relIndex);
        let rels = relses.addRelationship__2(RelationRef::Static(&*OxfXlRelations::ITEMPROPS), relIndex);
        // java return rels.createPart();
        let part = rels.borrow_mut().createPart(); part
    }
    // java public static OpcwPackagePart createWebExtension(OpcwRelationships relses, int relIndex) {
    pub fn createWebExtension(relses: &mut OpcwRelationships, relIndex: i32) -> PartRef {
        // java OpcwRelationship rels = relses.addRelationship(OxfXlRelations.WEBEXTENSION, relIndex);
        let rels = relses.addRelationship__2(RelationRef::Static(&*OxfXlRelations::WEBEXTENSION), relIndex);
        // java return rels.createPart();
        let part = rels.borrow_mut().createPart(); part
    }
    // java public static OpcwPackagePart createPivotTable(OpcwRelationships relses, int relIndex) {
    pub fn createPivotTable(relses: &mut OpcwRelationships, relIndex: i32) -> PartRef {
        // java OpcwRelationship rels = relses.addRelationship(OxfXlRelations.PIVOT_TABLE, relIndex);
        let rels = relses.addRelationship__2(RelationRef::Static(&*OxfXlRelations::PIVOT_TABLE), relIndex);
        // java return rels.createPart();
        let part = rels.borrow_mut().createPart(); part
    }
    // java public static OpcRelation getRelation(int type) {
    pub fn getRelation(r#type: i32) -> RelationRef {
        // java switch (type) {
        match r#type {
            // java case 2: return OxfXlRelations.imageEmf();
            2 => X::imageEmf(),
            // java case 3: return OxfXlRelations.imageWmf();
            3 => X::imageWmf(),
            // java case 4: return OxfXlRelations.imagePict();
            4 => X::imagePict(),
            // java case 5: return OxfXlRelations.imageJpeg();
            5 => X::imageJpeg(),
            // java case 6: return OxfXlRelations.imagePng();
            6 => X::imagePng(),
            // java case 7: return OxfXlRelations.imageBmp();
            7 => X::imageBmp(),
            // java case 8: case 9: case 10: case 11: case 12: case 13: case 14: case 15: case 16: case 17: case 18: default: return OxfXlRelations.imageJpeg();
            8..=18 => X::imageJpeg(),
            // java case 19: return OxfXlRelations.wdp();
            19 => X::wdp(),
            // java default: return OxfXlRelations.imageJpeg();
            _ => X::imageJpeg(),
        }
    }
    // The original PictureData overload and isGif remain outside this method slice.
    // Their EscherBlipRecord/FileDataStorage/slim-tool dependencies must retain Java identity and side effects before being connected.
}
