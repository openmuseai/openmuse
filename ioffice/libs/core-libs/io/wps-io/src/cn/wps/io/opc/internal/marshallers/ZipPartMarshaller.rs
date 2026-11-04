#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.io.dom.Document
// Java: import cn.wps.io.dom.Element
// Java: import cn.wps.io.dom.tree.DefaultDocument
// Java: import cn.wps.io.opc.PackagePart
// Java: import cn.wps.io.opc.PackagePartName
// Java: import cn.wps.io.opc.PackageRelationship
// Java: import cn.wps.io.opc.PackageRelationshipCollection
// Java: import cn.wps.io.opc.PackagingURIHelper
// Java: import cn.wps.io.opc.StreamHelper
// Java: import cn.wps.io.opc.TargetMode
// Java: import cn.wps.io.opc.internal.PartMarshaller
// Java: import cn.wps.io.opc.internal.ZipHelper
// Java: import cn.wps.io.util.URI
// Java: import cn.wps.io.util.WpsUriUtils
// Java: import cn.wps.io.xwpf.usermodel.exceptions.OpenXML4JException
// Java: import cn.wps.moffice.kfs.zip.ZipEntry
// Java: import cn.wps.moffice.kfs.zip.ZipOutputStream
// Java: import java.io.IOException
// Java: import java.io.OutputStream

// Java: public class ZipPartMarshaller implements PartMarshaller
pub struct ZipPartMarshaller {
}

// Java: public class ZipPartMarshaller implements PartMarshaller
impl ZipPartMarshaller {
    // Java: ZipPartMarshaller()
    pub fn new() -> Self {
        // Java: super();
        ZipPartMarshaller {}
    }

    // Java: boolean marshall(PackagePart part, OutputStream os)
    pub fn marshall(&self, _part: &dyn std::any::Any, _os: &mut dyn std::io::Write) -> Result<bool, Box<dyn std::error::Error>> {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (!(os instanceof ZipOutputStream)) {
        // java IOAssert.fail("Unexpected class " + os.getClass().getName());
        // java throw new OpenXML4JException("ZipOutputStream expected !");
        // java else {
        // java ZipOutputStream zos = (ZipOutputStream)os;
        // java if (part.hasRelationships()) {
        // java PackagePartName relationshipPartName = PackagingURIHelper.getRelationshipPartName(part.getPartName());
        // java marshallRelationshipPart(part.getRelationships(), relationshipPartName, zos);
        // java }
        // java return true;
        // java }
        */
    }

    // Java: void finish()
    pub fn finish(&self) {}

    // Java: static boolean marshallRelationshipPart(PackageRelationshipCollection rels, PackagePartName relPartName, ZipOutputStream zos)
    pub fn marshallRelationshipPart(_rels: &dyn std::any::Any, _rel_part_name: &dyn std::any::Any, _zos: &mut dyn std::any::Any) -> bool {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java Document xmlOutDoc = new DefaultDocument();
        // java Element root = xmlOutDoc.addElement("http://schemas.openxmlformats.org/package/2006/relationships", "", "Relationships");
        // java URI sourcePartURI = PackagingURIHelper.getSourcePartUriFromRelationshipPartUri(relPartName.getURI());
        // java for(PackageRelationship rel : rels) {
        // java Element relElem = root.addElement("Relationship");
        // java relElem.addAttribute("Id", rel.getId());
        // java relElem.addAttribute("Type", rel.getRelationshipType());
        // java URI uri = rel.getTargetURI();
        // java String targetValue;
        // java if (rel.getTargetMode() == TargetMode.EXTERNAL) {
        // java targetValue = uri.toString();
        // java relElem.addAttribute("TargetMode", "External");
        // java else {
        // java URI targetURI = rel.getTargetURI();
        // java targetValue = PackagingURIHelper.relativizeURI(sourcePartURI, targetURI, true).getPath();
        // java if (targetURI.getRawFragment() != null) {
        // java targetValue = targetValue + "#" + targetURI.getRawFragment();
        // java }
        // java }
        // java if (targetValue.contains("'")) {
        // java targetValue = targetValue.replaceFirst("%23'", "'");
        // java targetValue = targetValue.replaceAll("%20", " ");
        // java }
        // java }
        // java relElem.addAttribute("Target", targetValue);
        // java }
        // java xmlOutDoc.normalize();
        // java ZipEntry ctEntry = new ZipEntry(ZipHelper.getZipURIFromOPCName(WpsUriUtils.toASCIIString2(relPartName.getURI().toString())).getPath());
        // java try {
        // java zos.putNextEntry(ctEntry);
        // java if (!StreamHelper.saveXmlInStream(xmlOutDoc, zos)) {
        // java root.detachRecursively();
        // java return false;
        // java }
        // java zos.closeEntry();
        // java } catch (IOException e) {
        // java IOLog.e(TAG, "Cannot create zip entry " + relPartName, e);
        // java root.detachRecursively();
        // java return false;
        // java }
        // java root.detachRecursively();
        // java return true;
        */
    }
}
