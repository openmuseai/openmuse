// Java: import java.util.List
// Java: import java.util.ArrayList
// Java: import java.util.Set
use std::collections::HashSet;
// Java: import java.util.Map
use std::collections::HashMap;
// Java: import java.io.IOException
// Java: import java.io.Serializable
// Java: import java.io.ObjectOutputStream
// Java: import java.io.ObjectInputStream
// Java: import java.util.HashMap
// Java: import java.text.MessageFormat
// Java: import java.util.Locale
use super::Localizable::{Localizable, Locale};

// Java field: long serialVersionUID
#[allow(non_upper_case_globals, dead_code)]
pub(crate) const serialVersionUID: i64 = -6024911025449780478;

// Java: public class ExceptionContext implements Serializable
#[allow(non_snake_case)]
pub struct ExceptionContext {
    // Java field: Throwable throwable
    pub(crate) throwable: String,
    // Java field: List<Localizable> msgPatterns
    msgPatterns: Vec<Box<dyn Localizable>>,
    // Java field: List<Object[]> msgArguments
    msgArguments: Vec<Vec<String>>,
    // Java field: Map<String, Object> context
    pub(crate) context: HashMap<String, String>,
}

// Java: public class ExceptionContext implements Serializable
#[allow(non_snake_case)]
impl ExceptionContext {
    // Java: ExceptionContext(final Throwable throwable)
    pub fn new(throwable: String) -> Self {
        // Java: this.throwable = throwable;
        // Java: msgPatterns    = new ArrayList<Localizable>();
        // Java: msgArguments   = new ArrayList<Object[]>();
        // Java: context        = new HashMap<String, Object>();
        ExceptionContext {
            throwable,
            msgPatterns: Vec::new(),
            msgArguments: Vec::new(),
            context: HashMap::new(),
        }
    }

    // Java: Throwable getThrowable()
    pub fn getThrowable(&self) -> &str {
        // Java: return throwable;
        &self.throwable
    }

    // Java: void addMessage()
    pub fn addMessage(&mut self, pattern: Box<dyn Localizable>, arguments: Vec<String>) {
        // Java: msgPatterns.add(pattern);
        self.msgPatterns.push(pattern);
        // Java: msgArguments.add(ArgUtils.flatten(arguments));
        self.msgArguments.push(arguments);
    }

    // Java: void setValue(String key, Object value)
    pub fn setValue(&mut self, key: String, value: String) {
        // Java: context.put(key, value);
        self.context.insert(key, value);
    }

    // Java: Object getValue(String key)
    pub fn getValue(&self, key: &str) -> Option<&String> {
        // Java: return context.get(key);
        self.context.get(key)
    }

    // Java: Set<String> getKeys()
    pub fn getKeys(&self) -> HashSet<&String> {
        // Java: return context.keySet();
        self.context.keys().collect()
    }

    // Java: String getMessage()
    pub fn getMessage(&self) -> String {
        // Java: return getMessage(Locale.US);
        self.getMessage_locale(&"en_US".to_string())
    }

    // Java: String getLocalizedMessage()
    pub fn getLocalizedMessage(&self) -> String {
        // Java: return getMessage(Locale.getDefault());
        self.getMessage_locale(&"en_US".to_string())
    }

    // Java: String getMessage(final Locale locale)
    pub fn getMessage_locale(&self, locale: &Locale) -> String {
        // Java: return buildMessage(locale, ": ");
        self.buildMessage(locale, ": ")
    }

    // Java: String getMessage()
    pub fn getMessage_locale_separator(&self, locale: &Locale, separator: &str) -> String {
        // Java: return buildMessage(locale, separator);
        self.buildMessage(locale, separator)
    }

    // Java: String buildMessage()
    fn buildMessage(&self, locale: &Locale, separator: &str) -> String {
        let mut sb = String::new();
        let len = self.msgPatterns.len();
        for i in 0..len {
            let pat = &self.msgPatterns[i];
            let args = &self.msgArguments[i];
            let mut msg = pat.getLocalizedString(locale);
            for (j, arg) in args.iter().enumerate() {
                let placeholder = format!("{{{}}}", j);
                msg = msg.replace(&placeholder, arg);
            }
            sb.push_str(&msg);
            if i + 1 < len {
                sb.push_str(separator);
            }
        }
        sb
    }

    // Java: void writeObject(ObjectOutputStream out)
    fn writeObject(&self) {
        // no-op: Rust does not use Java serialization
    }

    // Java: void readObject(ObjectInputStream in)
    fn readObject(&mut self) {
        // no-op: Rust does not use Java serialization
    }

    // Java: void serializeMessages(ObjectOutputStream out)
    fn serializeMessages(&self) {
        // no-op: Rust does not use Java serialization
    }

    // Java: void deSerializeMessages(ObjectInputStream in)
    fn deSerializeMessages(&mut self) {
        // no-op: Rust does not use Java serialization
    }

    // Java: void serializeContext(ObjectOutputStream out)
    fn serializeContext(&self) {
        // no-op: Rust does not use Java serialization
    }

    // Java: void deSerializeContext(ObjectInputStream in)
    fn deSerializeContext(&mut self) {
        // no-op: Rust does not use Java serialization
    }

    // Java: String nonSerializableReplacement(Object obj)
    fn nonSerializableReplacement(&self, obj_type: &str) -> String {
        // Java: return "[Object could not be serialized: " + obj.getClass().getName() + "]";
        format!("[Object could not be serialized: {}]", obj_type)
    }
}
