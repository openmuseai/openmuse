use std::collections::HashMap;

pub fn parse_xml(_xml: &str) -> String {
    _xml.to_string()
}

pub fn get_tag_attributes(xml: &str, tag_name: &str) -> HashMap<String, String> {
    let mut attrs = HashMap::new();
    let search = format!("<{}", tag_name);
    if let Some(start) = xml.find(&search) {
        let rest = &xml[start + search.len()..];
        if let Some(end) = rest.find('>') {
            let attr_str = &rest[..end];
            parse_attributes(attr_str, &mut attrs);
        }
    }
    attrs
}

pub fn get_tag_children(xml: &str, tag_name: &str) -> Vec<(String, HashMap<String, String>)> {
    let mut children = Vec::new();
    let open_tag = format!("<{}", tag_name);
    if let Some(start) = xml.find(&open_tag) {
        let rest = &xml[start..];
        let close_tag = format!("</{}>", tag_name);
        let inner = if let Some(end) = rest.find(&close_tag) {
            if let Some(gt) = rest.find('>') {
                &rest[gt + 1..end]
            } else {
                ""
            }
        } else {
            ""
        };
        let mut pos = 0;
        while pos < inner.len() {
            if let Some(lt) = inner[pos..].find('<') {
                let tag_start = pos + lt;
                if tag_start + 1 >= inner.len() || inner.as_bytes()[tag_start + 1] == b'/' {
                    pos = tag_start + 1;
                    continue;
                }
                let after_lt = &inner[tag_start + 1..];
                let name_end = after_lt.find(|c: char| c.is_whitespace() || c == '>' || c == '/').unwrap_or(after_lt.len());
                let name = after_lt[..name_end].to_string();
                let mut attrs = HashMap::new();
                if let Some(gt) = after_lt.find('>') {
                    let attr_str = &after_lt[name_end..gt];
                    parse_attributes(attr_str, &mut attrs);
                    pos = tag_start + 1 + gt + 1;
                } else {
                    pos = tag_start + 1 + name_end;
                }
                children.push((name, attrs));
            } else {
                break;
            }
        }
    }
    children
}

fn parse_attributes(s: &str, attrs: &mut HashMap<String, String>) {
    let mut i = 0;
    let bytes = s.as_bytes();
    while i < bytes.len() {
        while i < bytes.len() && (bytes[i] as char).is_whitespace() {
            i += 1;
        }
        if i >= bytes.len() || bytes[i] == b'/' {
            break;
        }
        let name_start = i;
        while i < bytes.len() && bytes[i] != b'=' && !(bytes[i] as char).is_whitespace() {
            i += 1;
        }
        let name = &s[name_start..i];
        while i < bytes.len() && (bytes[i] == b'=' || (bytes[i] as char).is_whitespace()) {
            i += 1;
        }
        if i >= bytes.len() {
            break;
        }
        let quote = bytes[i];
        if quote != b'"' && quote != b'\'' {
            continue;
        }
        i += 1;
        let val_start = i;
        while i < bytes.len() && bytes[i] != quote {
            i += 1;
        }
        let value = &s[val_start..i];
        if i < bytes.len() {
            i += 1;
        }
        attrs.insert(name.to_string(), value.to_string());
    }
}
