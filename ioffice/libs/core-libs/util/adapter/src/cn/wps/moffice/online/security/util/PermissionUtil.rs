// Java: import cn.wps.moffice.online.security.data.SecurityRightData
use crate::cn::wps::moffice::online::security::data::SecurityRightData::SecurityRightData;

// Java field: int P_READ
pub(crate) const P_READ: i32 = 1;
// Java field: int P_WRITE
pub(crate) const P_WRITE: i32 = 2;
// Java field: int P_SAVEAS
pub(crate) const P_SAVEAS: i32 = 4;
// Java field: int P_COPY
pub(crate) const P_COPY: i32 = 8;
// Java field: int P_PRINT
pub(crate) const P_PRINT: i32 = 16;
// Java field: int P_AUTHORIZE
pub(crate) const P_AUTHORIZE: i32 = 32;
// Java field: int FULL_CONTROL
pub(crate) const FULL_CONTROL: i32 = 1;
// Java field: int AUTHORIZE
pub(crate) const AUTHORIZE: i32 = 2;
// Java field: int READ_ONLY
pub(crate) const READ_ONLY: i32 = 3;
// Java field: int EDIT
pub(crate) const EDIT: i32 = 4;
// Java field: int SAVEAS
pub(crate) const SAVEAS: i32 = 5;
// Java field: int COPY
pub(crate) const COPY: i32 = 6;
// Java field: int PRINT
pub(crate) const PRINT: i32 = 7;
// Java field: int PRINT_SCREEN
pub(crate) const PRINT_SCREEN: i32 = 8;

// Java: public class PermissionUtil
pub struct PermissionUtil {
}

// Java: public class PermissionUtil
impl PermissionUtil {
    // Java: static int parsePermission(SecurityRightData rights)
    pub fn parsePermission(rights: Option<&SecurityRightData>) -> i32 {
      // Java: if (null == rights)
      if rights.is_none() {
         // Java: return 0;
         return 0;
      }
      // Java: else
      else {
         // Java: int permission = 0;
         let mut permission = 0;
         // Java: for(String operationId : rights.operationIds)
         let rights = rights.unwrap();
         for operationId in &rights.operationIds {
            // Java: permission |= toPermission(Integer.parseInt(operationId));
            if let Ok(id) = operationId.parse::<i32>() {
                permission |= Self::toPermission(id);
            }
         }
         // Java: return permission;
         permission
      }
    }

    // Java: static int toPermission(int operationID)
    pub fn toPermission(operationID: i32) -> i32 {
      // Java: switch (operationID)
      match operationID {
         // Java: case 1:
         1 => {
            // Java: return 31;
            31
         }
         // Java: case 2:
         2 => {
            // Java: return 33;
            33
         }
         // Java: case 3:
         3 => {
            // Java: return 1;
            1
         }
         // Java: case 4:
         4 => {
            // Java: return 3;
            3
         }
         // Java: case 5:
         5 => {
            // Java: return 5;
            5
         }
         // Java: case 6:
         6 => {
            // Java: return 9;
            9
         }
         // Java: case 7:
         7 => {
            // Java: return 17;
            17
         }
         // Java: default:
         _ => {
            // Java: return 0;
            0
         }
      }
    }

    // Java: PermissionUtil()
    pub fn new() -> Self {
      // Java: super();
      PermissionUtil {}
    }

}
