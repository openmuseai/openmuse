// java public final class KmoConstraintCheck
pub struct KmoConstraintCheck {}

fn java_value_of(value: Option<&str>) -> String {
    match value {
        None => "null".to_string(),
        Some(text) => text.to_string(),
    }
}

fn index_of_from(template: &str, needle: &str, from: i32) -> i32 {
    if from < 0 {
        return -1;
    }
    let from_us = from as usize;
    if from_us > template.len() {
        return -1;
    }
    match template[from_us..].find(needle) {
        Some(pos) => (from_us + pos) as i32,
        None => -1,
    }
}

// java public final class KmoConstraintCheck
impl KmoConstraintCheck {
    // java private  KmoConstraintCheck()
    #[allow(dead_code)]
    fn KmoConstraintCheck() -> Self {
        Self {}
    }

    // java public static void checkArgument(boolean expression)
    pub fn checkArgument(expression: bool) {
        // java if (!expression)
        if !expression {
            // java throw new IllegalArgumentException();
            panic!("IllegalArgumentException");
        }
    }

    // java public static void checkArgument(boolean expression, Object errorMessage)
    pub fn checkArgument__boolean_object(expression: bool, errorMessage: Option<&str>) {
        // java if (!expression)
        if !expression {
            // java throw new IllegalArgumentException(String.valueOf(errorMessage));
            panic!(
                "IllegalArgumentException: {}",
                java_value_of(errorMessage)
            );
        }
    }

    // java public static void checkArgument(boolean expression, String errorMessageTemplate, Object... errorMessageArgs)
    pub fn checkArgument__boolean_string_(
        expression: bool,
        errorMessageTemplate: &str,
        errorMessageArgs: &[String],
    ) {
        // java if (!expression)
        if !expression {
            // java throw new IllegalArgumentException(format(errorMessageTemplate, errorMessageArgs));
            panic!(
                "IllegalArgumentException: {}",
                Self::format(Some(errorMessageTemplate), errorMessageArgs)
            );
        }
    }

    // java public static void checkState(boolean expression)
    pub fn checkState(expression: bool) {
        // java if (!expression)
        if !expression {
            // java throw new IllegalStateException();
            panic!("IllegalStateException");
        }
    }

    // java public static void checkState(boolean expression, Object errorMessage)
    pub fn checkState__boolean_object(expression: bool, errorMessage: Option<&str>) {
        // java if (!expression)
        if !expression {
            // java throw new IllegalStateException(String.valueOf(errorMessage));
            panic!("IllegalStateException: {}", java_value_of(errorMessage));
        }
    }

    // java public static void checkState(boolean expression, String errorMessageTemplate, Object... errorMessageArgs)
    pub fn checkState__boolean_string_(
        expression: bool,
        errorMessageTemplate: &str,
        errorMessageArgs: &[String],
    ) {
        // java if (!expression)
        if !expression {
            // java throw new IllegalStateException(format(errorMessageTemplate, errorMessageArgs));
            panic!(
                "IllegalStateException: {}",
                Self::format(Some(errorMessageTemplate), errorMessageArgs)
            );
        }
    }

    // java public static T checkNotNull(T reference)
    pub fn checkNotNull<T>(reference: Option<T>) -> T {
        // java if (reference == null)
        if reference.is_none() {
            // java throw new NullPointerException();
            panic!("NullPointerException");
        }
        // java else
        else {
            // java return reference;
            reference.unwrap()
        }
    }

    // java public static T checkNotNull(T reference, Object errorMessage)
    pub fn checkNotNull__t_object<T>(reference: Option<T>, errorMessage: Option<&str>) -> T {
        // java if (reference == null)
        if reference.is_none() {
            // java throw new NullPointerException(String.valueOf(errorMessage));
            panic!("NullPointerException: {}", java_value_of(errorMessage));
        }
        // java else
        else {
            // java return reference;
            reference.unwrap()
        }
    }

    // java public static T checkNotNull(T reference, String errorMessageTemplate, Object... errorMessageArgs)
    pub fn checkNotNull__t_string_<T>(
        reference: Option<T>,
        errorMessageTemplate: &str,
        errorMessageArgs: &[String],
    ) -> T {
        // java if (reference == null)
        if reference.is_none() {
            // java throw new NullPointerException(format(errorMessageTemplate, errorMessageArgs));
            panic!(
                "NullPointerException: {}",
                Self::format(Some(errorMessageTemplate), errorMessageArgs)
            );
        }
        // java else
        else {
            // java return reference;
            reference.unwrap()
        }
    }

    // java public static int checkElementIndex(int index, int size)
    pub fn checkElementIndex__int_int(index: i32, size: i32) -> i32 {
        // java return checkElementIndex(index, size, "index");
        Self::checkElementIndex__int_int_string(index, size, "index")
    }

    // java public static int checkElementIndex(int index, int size, String desc)
    pub fn checkElementIndex__int_int_string(index: i32, size: i32, desc: &str) -> i32 {
        // java if (index >= 0 && index < size)
        if index >= 0 && index < size {
            // java return index;
            return index;
        }
        // java else
        else {
            // java throw new IndexOutOfBoundsException(badElementIndex(index, size, desc));
            panic!(
                "IndexOutOfBoundsException: {}",
                Self::badElementIndex(index, size, desc)
            );
        }
    }

    // java private static String badElementIndex(int index, int size, String desc)
    pub fn badElementIndex(index: i32, size: i32, desc: &str) -> String {
        // java if (index < 0)
        if index < 0 {
            // java return format("%s (%s) must not be negative", desc, index);
            return Self::format(
                Some("%s (%s) must not be negative"),
                &[desc.to_string(), index.to_string()],
            );
        }
        // java else if (size < 0)
        else if size < 0 {
            // java throw new IllegalArgumentException("negative size: " + size);
            panic!("IllegalArgumentException: negative size: {size}");
        }
        // java else
        else {
            // java return format("%s (%s) must be less than size (%s)", desc, index, size);
            Self::format(
                Some("%s (%s) must be less than size (%s)"),
                &[desc.to_string(), index.to_string(), size.to_string()],
            )
        }
    }

    // java public static int checkPositionIndex(int index, int size)
    pub fn checkPositionIndex__int_int(index: i32, size: i32) -> i32 {
        // java return checkPositionIndex(index, size, "index");
        Self::checkPositionIndex__int_int_string(index, size, "index")
    }

    // java public static int checkPositionIndex(int index, int size, String desc)
    pub fn checkPositionIndex__int_int_string(index: i32, size: i32, desc: &str) -> i32 {
        // java if (index >= 0 && index <= size)
        if index >= 0 && index <= size {
            // java return index;
            return index;
        }
        // java else
        else {
            // java throw new IndexOutOfBoundsException(badPositionIndex(index, size, desc));
            panic!(
                "IndexOutOfBoundsException: {}",
                Self::badPositionIndex(index, size, desc)
            );
        }
    }

    // java private static String badPositionIndex(int index, int size, String desc)
    pub fn badPositionIndex(index: i32, size: i32, desc: &str) -> String {
        // java if (index < 0)
        if index < 0 {
            // java return format("%s (%s) must not be negative", desc, index);
            return Self::format(
                Some("%s (%s) must not be negative"),
                &[desc.to_string(), index.to_string()],
            );
        }
        // java else if (size < 0)
        else if size < 0 {
            // java throw new IllegalArgumentException("negative size: " + size);
            panic!("IllegalArgumentException: negative size: {size}");
        }
        // java else
        else {
            // java return format("%s (%s) must not be greater than size (%s)", desc, index, size);
            Self::format(
                Some("%s (%s) must not be greater than size (%s)"),
                &[desc.to_string(), index.to_string(), size.to_string()],
            )
        }
    }

    // java public static void checkPositionIndexes(int start, int end, int size)
    pub fn checkPositionIndexes(start: i32, end: i32, size: i32) {
        // java if (start < 0 || end < start || end > size)
        if start < 0 || end < start || end > size {
            // java throw new IndexOutOfBoundsException(badPositionIndexes(start, end, size));
            panic!(
                "IndexOutOfBoundsException: {}",
                Self::badPositionIndexes(start, end, size)
            );
        }
    }

    // java private static String badPositionIndexes(int start, int end, int size)
    pub fn badPositionIndexes(start: i32, end: i32, size: i32) -> String {
        // java if (start >= 0 && start <= size)
        if start >= 0 && start <= size {
            // java return end >= 0 && end <= size ? format("end index (%s) must not be less than start index (%s)", end, start) : badPositionIndex(end, size, "end index");
            if end >= 0 && end <= size {
                Self::format(
                    Some("end index (%s) must not be less than start index (%s)"),
                    &[end.to_string(), start.to_string()],
                )
            } else {
                Self::badPositionIndex(end, size, "end index")
            }
        }
        // java else
        else {
            // java return badPositionIndex(start, size, "start index");
            Self::badPositionIndex(start, size, "start index")
        }
    }

    // java static String format(String template, Object... args)
    pub fn format(template: Option<&str>, args: &[String]) -> String {
        // java template = String.valueOf(template);
        let template = java_value_of(template);
        // java StringBuilder builder = new StringBuilder(template.length() + 16 * args.length);
        let mut builder = String::with_capacity(template.len() + 16 * args.len());
        // java int templateStart = 0;
        let mut templateStart: i32 = 0;
        // java int i;
        let mut i: i32;
        // java int placeholderStart;
        let mut placeholderStart: i32;
        i = 0;
        // java for(i = 0; i < args.length; templateStart = placeholderStart + 2)
        while i < args.len() as i32 {
            // java placeholderStart = template.indexOf("%s", templateStart);
            placeholderStart = index_of_from(&template, "%s", templateStart);
            // java if (placeholderStart == -1)
            if placeholderStart == -1 {
                // java break;
                break;
            }
            // java builder.append(template.substring(templateStart, placeholderStart));
            builder.push_str(&template[templateStart as usize..placeholderStart as usize]);
            // java builder.append(args[i++]);
            let arg_index = i;
            i += 1;
            builder.push_str(&args[arg_index as usize]);
            templateStart = placeholderStart + 2;
        }
        // java builder.append(template.substring(templateStart));
        builder.push_str(&template[templateStart as usize..]);
        // java if (i < args.length)
        if i < args.len() as i32 {
            // java builder.append(" [");
            builder.push_str(" [");
            // java builder.append(args[i++]);
            let arg_index = i;
            i += 1;
            builder.push_str(&args[arg_index as usize]);
            // java while(i < args.length)
            while i < args.len() as i32 {
                // java builder.append(", ");
                builder.push_str(", ");
                // java builder.append(args[i++]);
                let arg_index = i;
                i += 1;
                builder.push_str(&args[arg_index as usize]);
            }
            // java builder.append(']');
            builder.push(']');
        }
        // java return builder.toString();
        builder
    }
}
