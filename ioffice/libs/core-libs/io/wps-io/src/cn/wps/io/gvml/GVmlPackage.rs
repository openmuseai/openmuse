#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.opc.OPCPackage
// Java: import cn.wps.io.opc.PackageAccess
// Java: import cn.wps.io.opc.PackagePart
// Java: import cn.wps.io.opc.PackagePartCollection
// Java: import cn.wps.io.opc.internal.FileHelper
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidFormatException
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidOperationException
// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.FileOutputStream
// Java: import cn.wps.moffice.kfs.zip.ZipOutputStream

use crate::io::opc::OPCPackage::OPCPackage;

// Java field: String TAG
const TAG: Option<&str> = None;

// Java: public class GVmlPackage extends OPCPackage
pub struct GVmlPackage {
    pub(crate) base: OPCPackage,
}

// Java: public class GVmlPackage extends OPCPackage
impl GVmlPackage {
    // Java: static GVmlPackage open(String path, PackageAccess access)
    pub fn open(path: &str, _access: i32) -> Self {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // Java: if (path != null && !"".equals(path.trim()) && (!(new File(path)).exists() || !(new File(path)).isDirectory()))
        // Java: GVmlPackage pack = new GVmlPackage(path, access);
        // Java: if (pack.partList == null && access != PackageAccess.WRITE)
        // Java: pack.getParts();
        // Java: pack.originalPackagePath = (new File(path)).getAbsolutePath();
        // Java: return pack;
        // Java: else
        // Java: throw new IllegalArgumentException("path");
        */
    }

    // Java: static GVmlPackage create(File file)
    pub fn create(_file: &dyn std::any::Any) -> Self {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // Java: if (file != null && (!file.exists() || !file.isDirectory()))
        // Java: GVmlPackage pkg = new GVmlPackage();
        // Java: pkg.originalPackagePath = file.getAbsolutePath();
        // Java: File dir = FileHelper.getDirectory(file);
        // Java: if (!dir.exists())
        // Java: dir.mkdirs();
        // Java: pkg.tmpPackageFile = File.createTempFile(generateTempFileName(dir), ".tmp", dir);
        // Java: pkg.mZOS = new ZipOutputStream(new FileOutputStream(pkg.tmpPackageFile));
        // Java: pkg.configurePackage();
        // Java: return pkg;
        // Java: else
        // Java: throw new IllegalArgumentException("file");
        */
    }

    // Java: GVmlPackage()
    pub fn new() -> Self {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // Java: super(PackageAccess.WRITE);
        */
    }

    // Java: GVmlPackage(String path, PackageAccess access)
    pub fn new_with_path(_path: &str, _access: i32) -> Self {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // Java: super(path, access);
        */
    }

    // Java: ArrayList<PackagePart> getParts()
    pub fn getParts(&mut self) -> Vec<Box<dyn std::any::Any + Send + Sync>> {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // Java: if (this.partList == null)
        // Java: PackagePart[] parts = this.getPartsImpl();
        // Java: int len = parts.length;
        // Java: this.rawPartList = this.partList;
        // Java: this.partList = new PackagePartCollection();
        // Java: for(int i = 0; i < len; ++i)
        // Java: PackagePart part = parts[i];
        // Java: if (this.partList.containsKey(part.getPartName()))
        // Java: throw new InvalidFormatException("A part with the name '" + part.getPartName() + "' already exist");
        // Java: try
        // Java: this.partList.put(part.getPartName(), part);
        // Java: catch (InvalidOperationException e)
        // Java: throw new InvalidFormatException(e.getMessage());
        // Java: return new ArrayList(this.partList.values());
        */
    }
}
