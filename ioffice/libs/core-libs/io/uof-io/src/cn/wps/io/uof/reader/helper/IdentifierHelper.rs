// Java: public class IdentifierHelper
pub struct IdentifierHelper {
}

// Java: public class IdentifierHelper
impl IdentifierHelper {
    // Java: IdentifierHelper()
    pub fn IdentifierHelper() -> Self {
      // Java: super();
      IdentifierHelper {}
    }

    // Java: static Integer convertSsStringIdentifier(String ssString)
    pub fn convertSsStringIdentifier(ssString: Option<&str>) -> Option<i32> {
      // Java: if (ssString == null)
      if ssString.is_none()
      {
         // Java: return null;
         return None;
      }
      // Java: else
      else
      {
         let s = ssString.unwrap();
         // Java: try
         {
            // Java: return Integer.parseInt(ssString.substring(8));
            if s.len() >= 8 {
                return s[8..].parse::<i32>().ok();
            }
         }
         // Java: catch (NumberFormatException var2)
         {
            // Java: return null;
            return None;
         }
      }
    }

    // Java: static Integer convertSsStyleIdentifier(String ssStyle)
    pub fn convertSsStyleIdentifier(ssStyle: Option<&str>) -> Option<i32> {
      // Java: if (ssStyle == null)
      if ssStyle.is_none()
      {
         // Java: return null;
         return None;
      }
      // Java: else
      else
      {
         let s = ssStyle.unwrap();
         // Java: try
         {
            // Java: return Integer.parseInt(ssStyle.substring(7));
            if s.len() >= 7 {
                return s[7..].parse::<i32>().ok();
            }
         }
         // Java: catch (NumberFormatException var2)
         {
            // Java: return null;
            return None;
         }
      }
    }

    // Java: static Integer convertGeLinkIdentifier(String ssStyle)
    pub fn convertGeLinkIdentifier(ssStyle: Option<&str>) -> Option<i32> {
      // Java: if (ssStyle == null)
      if ssStyle.is_none()
      {
         // Java: return null;
         return None;
      }
      // Java: else
      else
      {
         let s = ssStyle.unwrap();
         // Java: try
         {
            // Java: return Integer.parseInt(ssStyle.substring(6));
            if s.len() >= 6 {
                return s[6..].parse::<i32>().ok();
            }
         }
         // Java: catch (NumberFormatException var2)
         {
            // Java: return null;
            return None;
         }
      }
    }

}
