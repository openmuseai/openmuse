// Java: import cn.wps.io.util.URI
use crate::io::util::URI::*;
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidFormatException
use crate::io::xwpf::usermodel::exceptions::InvalidFormatException::*;
// Java: import cn.wps.io.xwpf.usermodel.exceptions.OpenXML4JException
use crate::io::xwpf::usermodel::exceptions::OpenXML4JException::*;
// Java: import cn.wps.io.opc.PackageRelationship
use crate::io::opc::PackageRelationship::*;
// Java: import cn.wps.io.opc.PackagePartName
use crate::io::opc::PackagePartName::*;
// Java: import cn.wps.io.opc.TargetMode
use crate::io::opc::TargetMode::*;
// Java: import cn.wps.io.opc.PackageRelationshipCollection
use crate::io::opc::PackageRelationshipCollection::*;

// java public interface RelationshipSource
pub trait RelationshipSource {
    // java  PackageRelationship addRelationship(PackagePartName var1, TargetMode var2, String var3)
    fn addRelationship__packagepartname_targetmode_string(&mut self, var1: &PackagePartName, var2: TargetMode, var3: &str) -> PackageRelationship;

    // java  PackageRelationship addRelationship(PackagePartName var1, TargetMode var2, String var3, String var4)
    fn addRelationship__packagepartname_targetmode_string_string(&mut self, var1: &PackagePartName, var2: TargetMode, var3: &str, var4: Option<&str>) -> PackageRelationship;

    // java  PackageRelationship addExternalRelationship(String var1, String var2)
    fn addExternalRelationship__string_string(&mut self, var1: &str, var2: &str) -> PackageRelationship;

    // java  PackageRelationship addExternalRelationship(URI var1, String var2)
    fn addExternalRelationship__uri_string(&mut self, var1: &URI, var2: &str) -> PackageRelationship;

    // java  PackageRelationship addExternalRelationship(String var1, String var2, String var3)
    fn addExternalRelationship__string_string_string(&mut self, var1: &str, var2: &str, var3: Option<&str>) -> PackageRelationship;

    // java  PackageRelationship addExternalRelationship(URI var1, String var2, String var3)
    fn addExternalRelationship__uri_string_string(&mut self, var1: &URI, var2: &str, var3: Option<&str>) -> PackageRelationship;

    // java  void removeRelationship(String var1)
    fn removeRelationship(&mut self, var1: &str);

    // java  PackageRelationshipCollection getRelationships()
    fn getRelationships(&mut self) -> PackageRelationshipCollection;

    // java  PackageRelationship getRelationship(String var1)
    fn getRelationship(&mut self, var1: &str) -> Option<PackageRelationship>;

    // java  PackageRelationshipCollection getRelationshipsByType(String var1)
    fn getRelationshipsByType(&mut self, var1: &str) -> PackageRelationshipCollection;

    // java  boolean hasRelationships()
    fn hasRelationships(&mut self) -> bool;

    // java  boolean isRelationshipExists(PackageRelationship var1)
    fn isRelationshipExists(&mut self, var1: &PackageRelationship) -> bool;

}
