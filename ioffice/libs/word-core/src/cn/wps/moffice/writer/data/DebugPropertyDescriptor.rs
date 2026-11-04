// Java: import cn.wps.moffice.property.IPropertySetDescription
use util::moffice::property::IPropertySetDescription::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import java.lang.reflect.Field
use stubs::java::lang::reflect::Field::*;
// Java: import java.util.HashMap
// Java: import java.lang.Class
use stubs::java::lang::Class::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
use std::collections::{HashMap};

// java private static HashMap<Integer, String> _property_names;
static _property_names: std::sync::OnceLock<Option<HashMap<i32, String>>> =
                std::sync::OnceLock::new();
// java private static HashMap<Integer, String> _shape_property_names;
static _shape_property_names: std::sync::OnceLock<Option<HashMap<i32, String>>> =
                std::sync::OnceLock::new();

// java public class DebugPropertyDescriptor implements IPropertySetDescription
#[derive(Default)]
pub struct DebugPropertyDescriptor {
}

// java public interface IPropertySetDescription
impl IPropertySetDescription for DebugPropertyDescriptor {
    // java public String getDescription(PropertySet p)
    fn getDescription(&self, property_set: &dyn std::any::Any) -> String {
      // java return PropertySet_toString(p);
      let p = property_set
          .downcast_ref::<PropertySet>()
          .expect("IPropertySetDescription.getDescription expects PropertySet");
      return Self::PropertySet_toString(p);
    }

}

// java public class DebugPropertyDescriptor implements IPropertySetDescription
impl DebugPropertyDescriptor {
    // java private  DebugPropertyDescriptor()
    pub fn DebugPropertyDescriptor() -> Self {
      Self {}
    }

    // java public static synchronized void init()
    pub fn init() {
    }

    // java static void fill_property_names(Class<?> pClass, HashMap<Integer, String> pMap)
    pub fn fill_property_names(pClass: &(dyn std::any::Any + Send + Sync), pMap: &mut HashMap<i32, String>) {
      // java Field[] fields = pClass.getDeclaredFields();
      let mut fields: Vec<Box<dyn std::any::Any + Send + Sync>> = pClass.getDeclaredFields();
          // java for(Field field : fields)
          for field in fields.iter() {
            // java String name = field.getName();
            let mut name: String = field.getName();
            // java if (!name.endsWith("_begin") && !name.endsWith("_end"))
            if !name.endsWith("_begin") && !name.endsWith("_end") {
              // java Object pValue = field.get(pClass);
              let mut pValue: Box<dyn Object> = field.get(pClass);
              // java if (pValue instanceof Integer)
              if pValue.downcast_ref::<i32>().is_some() {
                // java int value = (Integer)pValue;
                let mut value: i32 = ((pValue) as i32);
                // java pMap.put(value, name);
                pMap.put(value, name);
              }
            }
          }
          // java Exception e
          todo!();
            // java e.printStackTrace();
            e.printStackTrace();
    }

    // java static String property_to_string(PropertySet prop, HashMap<Integer, String> pMap)
    pub fn property_to_string(prop: &PropertySet, pMap: &HashMap<i32, String>) -> String {
      // java StringBuilder buf = new StringBuilder();
      let mut buf: String = StringBuilder::new();
      // java buf.append(prop.getClass().getSimpleName());
      buf.append(prop.getClass().getSimpleName());
      // java if (prop.isEmpty())
      if prop.isEmpty() {
        // java buf.append("\n{}");
        buf.append("\n{}");
        // java return buf.toString();
        return buf.toString();
      }
      // java else
      else {
        // java buf.append("\n{\n");
        buf.append("\n{\n");
        // java int n = prop.size();
        let mut n: i32 = prop.size();
        // java int[] ids = new int[n];
        let mut ids: Vec<i32> = vec![Default::default(); (n) as usize];
        // java Object[] values = new Object[n];
        let mut values: Box<dyn Object> = vec![Default::default(); (n) as usize];
        // java prop.getAll(ids, values);
        prop.getAll(Some(&mut ids), Some(&mut values));
        let mut i: i32 = 0;
        // java for(int i = 0; i < n; ++i)
        while i < n {
          // java String name = (String)pMap.get(ids[i]);
          let mut name: String = pMap.get(ids[(i) as usize]).to_string();
          // java if (name == null)
          if name.is_none() {
            // java name = "unknown";
            name = "unknown";
          }
          // java buf.append("\t" + name + " = " + values[i] + "\n");
          buf.append("\t" + name + " = " + values[(i) as usize] + "\n");
        }
        // java buf.append("}");
        buf.append("}");
        // java return buf.toString();
        return buf.toString();
      }
    }

    // java public static synchronized String PropertySet_toString(PropertySet p)
    pub fn PropertySet_toString(p: &PropertySet) -> String {
      // java return "";
      return "";
    }

}
