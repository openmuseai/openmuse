#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut)]

use std::collections::{HashMap, HashSet};
use std::path::Path;
use crate::cn::wps::moffice::util::KSLog::KSLog;
use crate::cn::wps::moffice::util::FileUtil::FileUtil;

// Java field: String TAG
const TAG: &str = "";
// Java field: Gson gson
static GSON: std::sync::OnceLock<String> = std::sync::OnceLock::new();
// Java field: Gson gsonNormal
static GSON_NORMAL: std::sync::OnceLock<String> = std::sync::OnceLock::new();

// Java: public class JSONUtil
pub struct JSONUtil;

// Java: public class JSONUtil
impl JSONUtil {
    // Java: JSONUtil()
    pub fn new() -> Self {
        // Java: super();
        Self
    }

    // Java: static Gson getGson()
    pub fn getGson() -> String {
        // Java: if (gson == null)
        // Java: gson = (new GsonBuilder()).excludeFieldsWithoutExposeAnnotation().create();
        // Java: return gson;
        String::new()
    }

    // Java: static Gson getGsonNormal()
    pub fn getGsonNormal() -> String {
        // Java: if (gsonNormal == null)
        // Java: gsonNormal = (new GsonBuilder()).create();
        // Java: return gsonNormal;
        String::new()
    }

    // Java: static public static <T> String toJSONString(T instance)
    pub fn toJSONString(instance: &str) -> String {
        // Java: return getGson().toJson(instance);
        instance.to_string()
    }

    // Java: static private static <T> Collection<T> instanceCollection(String jsonStr)
    fn instanceCollection(json_str: &str) -> Option<Vec<String>> {
        // Java: Collection<T> ts = (Collection)getGson().fromJson(jsonStr, (new TypeToken<Collection<T>>()
        {
        }
        // Java: return ts;
        None
    }

    // Java: static public static <T> T instance(String jsonStr, Class<T> cls)
    pub fn instance(json_str: &str) -> Option<String> {
        // Java: T obj = (T)getGson().fromJson(jsonStr, cls);
        let obj: Option<String> = Some(json_str.to_string());
        // Java: return obj;
        obj
    }

    // Java: static public static <T> void writeObjectCollection(Collection<T> objs, String persistencePath)
    pub fn writeObjectCollection(objs: &[String], persistence_path: &str) {
        // Java: try
        match (|| -> Result<(), Box<dyn std::error::Error>>
        {
            // Java: String str = getGson().toJson(objs, (new TypeToken<Collection<T>>()
            {
            }
            let str_val = format!("{:?}", objs);
            // Java: writeFile(str, persistencePath);
            Self::writeFile(&str_val, persistence_path);
            Ok(())
        })() {
        // Java: catch (Exception e)
        Err(e) =>
        {
            // Java: KSLog.e(TAG, "Exceprion", e);
            KSLog::e(TAG, "Exceprion");
        }
        Ok(_) => {}
        }
    }

    // Java: static void writeFile(String json, String persistencePath)
    pub fn writeFile(json: &str, persistence_path: &str) {
        // Java: try
        match (|| -> Result<(), std::io::Error>
        {
            // Java: File f = new File(persistencePath);
            let f = Path::new(persistence_path);
            // Java: if (!f.exists())
            if !f.exists()
            {
                // Java: FileUtil.newFile(persistencePath);
                FileUtil::newFile(persistence_path);
            }
            // Java: FileUtil.writeToFile(persistencePath, json);
            FileUtil::writeToFile(persistence_path, json);
            Ok(())
        })() {
        // Java: catch (IOException e)
        Err(e) =>
        {
            // Java: e.printStackTrace();
            eprintln!("{}", e);
        }
        Ok(_) => {}
        }
    }

    // Java: static public static <T> void writeObjectArray(T[] objs, String persistencePath)
    pub fn writeObjectArray(objs: &[String], persistence_path: &str) {
        // Java: try
        match (|| -> Result<(), Box<dyn std::error::Error>>
        {
            // Java: String str = getGson().toJson(objs, (new TypeToken<T>()
            {
            }
            let str_val = format!("{:?}", objs);
            // Java: writeFile(str, persistencePath);
            Self::writeFile(&str_val, persistence_path);
            Ok(())
        })() {
        // Java: catch (Exception e)
        Err(e) =>
        {
            // Java: KSLog.e(TAG, "Exception", e);
            KSLog::e(TAG, "Exception");
        }
        Ok(_) => {}
        }
    }

    // Java: static public static <T> void writeObject(T obj, String persistencePath)
    pub fn writeObject(obj: &str, persistence_path: &str) {
        // Java: try
        match (|| -> Result<(), Box<dyn std::error::Error>>
        {
            // Java: String str = getGson().toJson(obj);
            let str_val = obj.to_string();
            // Java: writeFile(str, persistencePath);
            Self::writeFile(&str_val, persistence_path);
            Ok(())
        })() {
        // Java: catch (Exception e)
        Err(e) =>
        {
            // Java: KSLog.e(TAG, "Exception", e);
            KSLog::e(TAG, "Exception");
        }
        Ok(_) => {}
        }
    }

    // Java: static public static <T> Collection<T> readCollection(String persistencePath, Collection<T> collection)
    pub fn readCollection_with_collection(persistence_path: &str, collection: &[String]) -> Option<Vec<String>> {
        // Java: try
        match (|| -> Result<Option<Vec<String>>, Box<dyn std::error::Error>>
        {
            // Java: File f = new File(persistencePath);
            let f = Path::new(persistence_path);
            // Java: if (f.exists())
            if f.exists()
            {
                // Java: String str = FileUtil.readFile(persistencePath);
                let str_val = FileUtil::readFile(persistence_path);
                // Java: Collection<T> object = (Collection)getGson().fromJson(str, (new TypeToken<Collection<T>>()
                {
                }
                let object: Option<Vec<String>> = Some(vec![str_val]);
                // Java: return object;
                return Ok(object);
            }
            Ok(None)
        })() {
        // Java: catch (Exception e)
        Err(e) =>
        {
            // Java: KSLog.e(TAG, "Exception", e);
            KSLog::e(TAG, "Exception");
            // Java: return null;
            None
        }
        Ok(v) => {
            if v.is_some() { return v; }
            // Java: return null;
            None
        }
        }
    }

    // Java: static public static <T> Collection<T> readCollection(String persistencePath)
    pub fn readCollection(persistence_path: &str) -> Option<Vec<String>> {
        // Java: try
        match (|| -> Result<Option<Vec<String>>, Box<dyn std::error::Error>>
        {
            // Java: File f = new File(persistencePath);
            let f = Path::new(persistence_path);
            // Java: if (f.exists())
            if f.exists()
            {
                // Java: String str = FileUtil.readFile(persistencePath);
                let str_val = FileUtil::readFile(persistence_path);
                // Java: Collection<T> object = null;
                let mut object: Option<Vec<String>> = None;
                // Java: object = instanceCollection(str);
                object = Self::instanceCollection(&str_val);
                // Java: return object;
                return Ok(object);
            }
            Ok(None)
        })() {
        // Java: catch (Exception e)
        Err(e) =>
        {
            // Java: KSLog.e(TAG, "Exception", e);
            KSLog::e(TAG, "Exception");
            // Java: return null;
            None
        }
        Ok(v) => {
            if v.is_some() { return v; }
            // Java: return null;
            None
        }
        }
    }

    // Java: static public static <T> Object readObject(String persistencePath, Class<T> cls)
    pub fn readObject(persistence_path: &str) -> Option<String> {
        // Java: try
        match (|| -> Result<Option<String>, Box<dyn std::error::Error>>
        {
            // Java: File f = new File(persistencePath);
            let f = Path::new(persistence_path);
            // Java: if (f.exists())
            if f.exists()
            {
                // Java: String str = FileUtil.readFile(persistencePath);
                let str_val = FileUtil::readFile(persistence_path);
                // Java: Object object = null;
                let mut object: Option<String> = None;
                // Java: object = instance(str, cls);
                object = Self::instance(&str_val);
                // Java: return object;
                return Ok(object);
            }
            Ok(None)
        })() {
        // Java: catch (Exception e)
        Err(e) =>
        {
            // Java: KSLog.e(TAG, "Exception", e);
            KSLog::e(TAG, "Exception");
            // Java: return null;
            None
        }
        Ok(v) => {
            if v.is_some() { return v; }
            // Java: return null;
            None
        }
        }
    }

    // Java: static public static <T> T instanceFromJSONMapObject(Map<String, Object> map, Class<T> cls)
    pub fn instanceFromJSONMapObject(map: &HashMap<String, String>) -> Option<HashMap<String, String>> {
        // Java: T instance = null;
        let mut instance: Option<HashMap<String, String>> = None;
        // Java: try
        match (|| -> Result<(), Box<dyn std::error::Error>>
        {
            // Java: instance = (T)cls.newInstance();
            let mut inst = HashMap::new();
            // Java: Field[] fields = cls.getFields();
            let fields: Vec<&String> = map.keys().collect();
            // Java: Set<String> keySet = map.keySet();
            let key_set: HashSet<&String> = map.keys().collect();
            // Java: for(int j = 0; j < fields.length; ++j)
            for j in 0..fields.len()
            {
                // Java: Field field = fields[j];
                let field = fields[j];
                // Java: if (keySet.contains(field.getName()))
                if key_set.contains(field)
                {
                    // Java: if (!field.isAccessible())
                    {
                        // Java: field.setAccessible(true);
                    }
                    // Java: Object value = map.get(field.getName());
                    let value = map.get(field).cloned().unwrap_or_default();
                    // Java: copyFromValue(instance, value, field);
                    inst.insert(field.clone(), value);
                }
            }
            instance = Some(inst);
            Ok(())
        })() {
        // Java: catch (Exception e)
        Err(e) =>
        {
            // Java: KSLog.e(TAG, "Exception", e);
            KSLog::e(TAG, "Exception");
        }
        Ok(_) => {}
        }
        // Java: return instance;
        instance
    }

    // Java: static void copyFromValue(Object instance, Object value, Field field)
    pub fn copyFromValue(instance: &mut HashMap<String, String>, value: &str, field: &str) {
        // Java: try
        match (|| -> Result<(), Box<dyn std::error::Error>>
        {
            // Java: if (!field.getType().equals(value.getClass()) && !field.getType().isAssignableFrom(value.getClass()))
            {
                // Java: if (value.getClass().equals(Double.class) || value.getClass().equals(Double.TYPE))
                if let Ok(d) = value.parse::<f64>()
                {
                    // Java: Double d = (Double)value;
                    // Java: if (!field.getType().equals(Integer.class) && !field.getType().equals(Integer.TYPE))
                    {
                        // Java: if (!field.getType().equals(Float.class) && !field.getType().equals(Float.TYPE))
                        {
                            // Java: if (field.getType().equals(Long.class) || field.getType().equals(Long.TYPE))
                            {
                                // Java: value = d.longValue();
                                let _long_val = d as i64;
                            }
                        }
                        // Java: else
                        {
                            // Java: value = d.floatValue();
                            let _float_val = d as f32;
                        }
                    }
                    // Java: else
                    {
                        // Java: value = d.intValue();
                        let _int_val = d as i32;
                    }
                    // Java: field.set(instance, value);
                    instance.insert(field.to_string(), value.to_string());
                }
            }
            // Java: else
            {
                // Java: field.set(instance, value);
                instance.insert(field.to_string(), value.to_string());
            }
            Ok(())
        })() {
        // Java: catch (Exception e)
        Err(e) =>
        {
            // Java: KSLog.e(TAG, "Exception", e);
            KSLog::e(TAG, "Exception");
        }
        Ok(_) => {}
        }
    }
}
