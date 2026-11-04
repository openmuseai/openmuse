#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::any::Any;

// Java: public class Preconditions
pub struct Preconditions;

// Java: public class Preconditions
impl Preconditions {
    // Java: Preconditions()
    // Java: super();

    // Java: static void checkArgument(boolean expression)
    pub fn checkArgument(expression: bool) {
        // Java: if (!expression)
        if !expression {
            // Java: throw new IllegalArgumentException();
            panic!("IllegalArgumentException");
        }
    }

    // Java: static void checkArgument(boolean expression, Object errorMessage)
    pub fn checkArgument_msg(expression: bool, error_message: &str) {
        // Java: if (!expression)
        if !expression {
            // Java: throw new IllegalArgumentException(String.valueOf(errorMessage));
            panic!("{}", error_message);
        }
    }

    // Java: static void checkArgument(boolean expression, String errorMessageTemplate, Object... errorMessageArgs)
    pub fn checkArgument_fmt(expression: bool, template: &str, args: &[&dyn std::fmt::Display]) {
        // Java: if (!expression)
        if !expression {
            // Java: throw new IllegalArgumentException(format(errorMessageTemplate, errorMessageArgs));
            panic!("{}", Self::format(template, args));
        }
    }

    // Java: static void checkState(boolean expression)
    pub fn checkState(expression: bool) {
        // Java: if (!expression)
        if !expression {
            // Java: throw new IllegalStateException();
            panic!("IllegalStateException");
        }
    }

    // Java: static void checkState(boolean expression, Object errorMessage)
    pub fn checkState_msg(expression: bool, error_message: &str) {
        // Java: if (!expression)
        if !expression {
            // Java: throw new IllegalStateException(String.valueOf(errorMessage));
            panic!("{}", error_message);
        }
    }

    // Java: static void checkState(boolean expression, String errorMessageTemplate, Object... errorMessageArgs)
    pub fn checkState_fmt(expression: bool, template: &str, args: &[&dyn std::fmt::Display]) {
        // Java: if (!expression)
        if !expression {
            // Java: throw new IllegalStateException(format(errorMessageTemplate, errorMessageArgs));
            panic!("{}", Self::format(template, args));
        }
    }

    // Java: static public static <T> T checkNotNull(T reference)
    pub fn checkNotNull<T>(reference: Option<T>) -> T {
        // Java: if (reference == null)
        match reference {
            None => {
                // Java: throw new NullPointerException();
                panic!("NullPointerException");
            }
            // Java: else
            Some(r) => {
                // Java: return reference;
                r
            }
        }
    }

    // Java: static public static <T> T checkNotNull(T reference, Object errorMessage)
    pub fn checkNotNull_msg<T>(reference: Option<T>, error_message: &str) -> T {
        // Java: if (reference == null)
        match reference {
            None => {
                // Java: throw new NullPointerException(String.valueOf(errorMessage));
                panic!("{}", error_message);
            }
            // Java: else
            Some(r) => {
                // Java: return reference;
                r
            }
        }
    }

    // Java: static public static <T> T checkNotNull(T reference, String errorMessageTemplate, Object... errorMessageArgs)
    pub fn checkNotNull_fmt<T>(reference: Option<T>, template: &str, args: &[&dyn std::fmt::Display]) -> T {
        // Java: if (reference == null)
        match reference {
            None => {
                // Java: throw new NullPointerException(format(errorMessageTemplate, errorMessageArgs));
                panic!("{}", Self::format(template, args));
            }
            // Java: else
            Some(r) => {
                // Java: return reference;
                r
            }
        }
    }

    // Java: static int checkElementIndex(int index, int size)
    pub fn checkElementIndex(index: i32, size: i32) -> i32 {
        // Java: return checkElementIndex(index, size, "index");
        Self::checkElementIndex_desc(index, size, "index")
    }

    // Java: static int checkElementIndex(int index, int size, String desc)
    pub fn checkElementIndex_desc(index: i32, size: i32, desc: &str) -> i32 {
        // Java: if (index >= 0 && index < size)
        if index >= 0 && index < size {
            // Java: return index;
            index
        }
        // Java: else
        else {
            // Java: throw new IndexOutOfBoundsException(badElementIndex(index, size, desc));
            panic!("{}", Self::badElementIndex(index, size, desc));
        }
    }

    // Java: static String badElementIndex(int index, int size, String desc)
    fn badElementIndex(index: i32, size: i32, desc: &str) -> String {
        // Java: if (index < 0)
        if index < 0 {
            // Java: return format("%s (%s) must not be negative", desc, index);
            format!("{} ({}) must not be negative", desc, index)
        }
        // Java: else if (size < 0)
        else if size < 0 {
            // Java: throw new IllegalArgumentException("negative size: " + size);
            panic!("negative size: {}", size);
        }
        // Java: else
        else {
            // Java: return format("%s (%s) must be less than size (%s)", desc, index, size);
            format!("{} ({}) must be less than size ({})", desc, index, size)
        }
    }

    // Java: static int checkPositionIndex(int index, int size)
    pub fn checkPositionIndex(index: i32, size: i32) -> i32 {
        // Java: return checkPositionIndex(index, size, "index");
        Self::checkPositionIndex_desc(index, size, "index")
    }

    // Java: static int checkPositionIndex(int index, int size, String desc)
    pub fn checkPositionIndex_desc(index: i32, size: i32, desc: &str) -> i32 {
        // Java: if (index >= 0 && index <= size)
        if index >= 0 && index <= size {
            // Java: return index;
            index
        }
        // Java: else
        else {
            // Java: throw new IndexOutOfBoundsException(badPositionIndex(index, size, desc));
            panic!("{}", Self::badPositionIndex(index, size, desc));
        }
    }

    // Java: static String badPositionIndex(int index, int size, String desc)
    fn badPositionIndex(index: i32, size: i32, desc: &str) -> String {
        // Java: if (index < 0)
        if index < 0 {
            // Java: return format("%s (%s) must not be negative", desc, index);
            format!("{} ({}) must not be negative", desc, index)
        }
        // Java: else if (size < 0)
        else if size < 0 {
            // Java: throw new IllegalArgumentException("negative size: " + size);
            panic!("negative size: {}", size);
        }
        // Java: else
        else {
            // Java: return format("%s (%s) must not be greater than size (%s)", desc, index, size);
            format!("{} ({}) must not be greater than size ({})", desc, index, size)
        }
    }

    // Java: static void checkPositionIndexes(int start, int end, int size)
    pub fn checkPositionIndexes(start: i32, end: i32, size: i32) {
        // Java: if (start < 0 || end < start || end > size)
        if start < 0 || end < start || end > size {
            // Java: throw new IndexOutOfBoundsException(badPositionIndexes(start, end, size));
            panic!("{}", Self::badPositionIndexes(start, end, size));
        }
    }

    // Java: static String badPositionIndexes(int start, int end, int size)
    fn badPositionIndexes(start: i32, end: i32, size: i32) -> String {
        // Java: if (start >= 0 && start <= size)
        if start >= 0 && start <= size {
            // Java: return end >= 0 && end <= size ? format("end index (%s) must not be less than start index (%s)", end, start) : badPositionIndex(end, size, "end index");
            if end >= 0 && end <= size {
                format!("end index ({}) must not be less than start index ({})", end, start)
            } else {
                Self::badPositionIndex(end, size, "end index")
            }
        }
        // Java: else
        else {
            // Java: return badPositionIndex(start, size, "start index");
            Self::badPositionIndex(start, size, "start index")
        }
    }

    // Java: static String format(String template, Object... args)
    pub fn format(template: &str, args: &[&dyn std::fmt::Display]) -> String {
        // Java: template = String.valueOf(template);
        // Java: StringBuilder builder = new StringBuilder(template.length() + 16 * args.length);
        let mut builder = String::with_capacity(template.len() + 16 * args.len());
        // Java: int templateStart = 0;
        let mut template_start = 0usize;
        // Java: int i;
        let mut i = 0usize;
        // Java: int placeholderStart;
        // Java: for(i = 0; i < args.length; templateStart = placeholderStart + 2)
        while i < args.len() {
            // Java: placeholderStart = template.indexOf("%s", templateStart);
            let placeholder_start = template[template_start..].find("%s").map(|p| p + template_start);
            // Java: if (placeholderStart == -1)
            if placeholder_start.is_none() {
                // Java: break;
                break;
            }
            let placeholder_start = placeholder_start.unwrap();
            // Java: builder.append(template.substring(templateStart, placeholderStart));
            builder.push_str(&template[template_start..placeholder_start]);
            // Java: builder.append(args[i++]);
            builder.push_str(&format!("{}", args[i]));
            i += 1;
            template_start = placeholder_start + 2;
        }
        // Java: builder.append(template.substring(templateStart));
        builder.push_str(&template[template_start..]);
        // Java: if (i < args.length)
        if i < args.len() {
            // Java: builder.append(" [");
            builder.push_str(" [");
            // Java: builder.append(args[i++]);
            builder.push_str(&format!("{}", args[i]));
            i += 1;
            // Java: while(i < args.length)
            while i < args.len() {
                // Java: builder.append(", ");
                builder.push_str(", ");
                // Java: builder.append(args[i++]);
                builder.push_str(&format!("{}", args[i]));
                i += 1;
            }
            // Java: builder.append(']');
            builder.push(']');
        }
        // Java: return builder.toString();
        builder
    }
}
