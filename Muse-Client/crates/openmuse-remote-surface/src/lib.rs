//! Strict projection of `openmuse.remote-surface` / `openmuse.remote-control` v1.
//!
//! The Dart package is the Mobile projection. This crate accepts and rejects the
//! same fixtures so a Desktop host cannot drift from that wire contract.

use serde_json::{Map, Value};
use std::collections::BTreeSet;

const MAX_SAFE: i64 = 9_007_199_254_740_991;
const COMPONENTS: &[&str] = &[
    "button",
    "compare",
    "confirmation",
    "diff",
    "document",
    "form",
    "gallery",
    "grid",
    "image",
    "link",
    "list",
    "markdown",
    "progress",
    "status",
    "stepper",
    "text",
    "timeline-basic",
    "video-player",
];
const MODES: &[&str] = &["declarative", "media", "web-interactive", "web-snapshot"];
const EFFECTS: &[&str] = &[
    "external-side-effect",
    "read",
    "workspace-commit",
    "workspace-propose",
];
const FORMATS: &[&str] = &[
    "image/jpeg",
    "image/png",
    "video/hls",
    "video/mp4",
    "video/poster",
];
const EVENT_STATES: &[&str] = &[
    "accepted",
    "cancelled",
    "failed",
    "outcome_unknown",
    "running",
    "succeeded",
];
const FORBIDDEN: &[&str] = &[
    "actor",
    "cookie",
    "darttype",
    "desktopdevice",
    "device",
    "filepath",
    "method",
    "mobiledevice",
    "path",
    "secret",
    "src",
    "uri",
    "url",
    "widget",
];

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum RemoteSurfaceError {
    #[error("{0}")]
    Format(String),
}

pub fn validate_message(kind: &str, value: &Value) -> Result<(), RemoteSurfaceError> {
    match kind {
        "hello" => validate_hello(value),
        "descriptor" => validate_descriptor(value),
        "snapshot" => validate_snapshot(value),
        "request" => validate_request(value),
        "receipt" => validate_receipt(value),
        "event" => validate_event(value),
        _ => Err(error(format!("unknown fixture kind {kind}"))),
    }
}

fn validate_hello(value: &Value) -> Result<(), RemoteSurfaceError> {
    let map = object(value, "hello")?;
    require_keys(
        map,
        &[
            "protocol",
            "protocolMajor",
            "protocolMinor",
            "components",
            "mediaFormats",
            "webSnapshot",
            "webInteractive",
            "maxControlBytes",
        ],
        &[],
    )?;
    protocol(map, "openmuse.remote-surface/hello/v1")?;
    let major = int_field(map, "protocolMajor", 0, 32)?;
    let minor = int_field(map, "protocolMinor", 0, 32)?;
    if major != 1 || minor != 0 {
        return Err(error("incompatible remote surface protocol"));
    }
    let components = string_list(map, "components", 32, false)?;
    if components.is_empty()
        || !unique(&components)
        || components
            .iter()
            .any(|item| !COMPONENTS.contains(&item.as_str()))
    {
        return Err(error("components must be unique known values"));
    }
    let formats = string_list(map, "mediaFormats", 8, false)?;
    if formats.is_empty()
        || !unique(&formats)
        || formats.iter().any(|item| !FORMATS.contains(&item.as_str()))
    {
        return Err(error("mediaFormats must be unique known values"));
    }
    bool_field(map, "webSnapshot")?;
    bool_field(map, "webInteractive")?;
    let budget = int_field(map, "maxControlBytes", 1, MAX_SAFE)?;
    if !(1024..=65536).contains(&budget) {
        return Err(error("maxControlBytes is outside the supported range"));
    }
    Ok(())
}

fn validate_descriptor(value: &Value) -> Result<(), RemoteSurfaceError> {
    let map = object(value, "descriptor")?;
    require_keys(
        map,
        &[
            "protocol",
            "pluginId",
            "surfaceId",
            "title",
            "workspaceRef",
            "modes",
            "requiredCapabilities",
            "readPermissions",
            "actions",
        ],
        &[],
    )?;
    protocol(map, "openmuse.remote-surface/descriptor/v1")?;
    plugin_id(string_field(map, "pluginId", 160)?)?;
    surface_id(string_field(map, "surfaceId", 64)?)?;
    string_field(map, "title", 80)?;
    opaque(string_field(map, "workspaceRef", 129)?, 129)?;
    let modes = string_list(map, "modes", 4, false)?;
    if modes.is_empty()
        || !unique(&modes)
        || modes.iter().any(|mode| !MODES.contains(&mode.as_str()))
    {
        return Err(error("modes must be unique known values"));
    }
    let capabilities = string_list(map, "requiredCapabilities", 18, true)?;
    if !unique(&capabilities)
        || capabilities
            .iter()
            .any(|item| !COMPONENTS.contains(&item.as_str()))
    {
        return Err(error(
            "requiredCapabilities must be unique known components",
        ));
    }
    permissions(map, "readPermissions", false)?;
    let actions = array(map, "actions")?;
    if actions.len() > 32 {
        return Err(error("actions must be unique and bounded"));
    }
    let mut ids = BTreeSet::new();
    for action in actions {
        let id = validate_action(action)?;
        if !ids.insert(id) {
            return Err(error("actions must be unique and bounded"));
        }
    }
    Ok(())
}

fn validate_action(value: &Value) -> Result<String, RemoteSurfaceError> {
    let map = object(value, "action")?;
    require_keys(
        map,
        &["id", "effect", "requiredPermissions", "inputSchema"],
        &[],
    )?;
    let id = action_id(string_field(map, "id", 81)?)?.to_string();
    let effect = string_field(map, "effect", 32)?;
    if !EFFECTS.contains(&effect) {
        return Err(error("action effect is unsupported"));
    }
    permissions(map, "requiredPermissions", false)?;
    validate_schema(object(
        map.get("inputSchema").unwrap_or(&Value::Null),
        "inputSchema",
    )?)?;
    Ok(id)
}

fn validate_schema(schema: &Map<String, Value>) -> Result<(), RemoteSurfaceError> {
    require_keys(
        schema,
        &["type", "additionalProperties", "required", "properties"],
        &[],
    )?;
    if schema.get("type").and_then(Value::as_str) != Some("object")
        || schema.get("additionalProperties").and_then(Value::as_bool) != Some(false)
    {
        return Err(error("inputSchema must be a closed object"));
    }
    let required = string_list(schema, "required", 16, true)?;
    let properties = object(
        schema.get("properties").unwrap_or(&Value::Null),
        "properties",
    )?;
    if properties.len() > 16 {
        return Err(error("inputSchema has too many fields"));
    }
    for name in properties.keys() {
        field_id(name)?;
    }
    for field in &required {
        field_id(field)?;
        if !properties.contains_key(field) {
            return Err(error(format!(
                "inputSchema.required contains unknown field {field}"
            )));
        }
    }
    for (_name, property) in properties {
        let property = object(property, "inputSchema.properties")?;
        let kind = string_field(property, "type", 16)?;
        match kind {
            "string" => {
                require_keys(property, &["type"], &["minLength", "maxLength"])?;
                let min = optional_int(property, "minLength", 0, 2000)?.unwrap_or(0);
                let max = optional_int(property, "maxLength", 0, 2000)?.unwrap_or(2000);
                if min > max {
                    return Err(error("inputSchema string bounds are inverted"));
                }
            }
            "integer" => {
                require_keys(property, &["type"], &["minimum", "maximum"])?;
                let minimum = optional_int(property, "minimum", -MAX_SAFE, MAX_SAFE)?;
                let maximum = optional_int(property, "maximum", -MAX_SAFE, MAX_SAFE)?;
                if minimum.zip(maximum).is_some_and(|(min, max)| min > max) {
                    return Err(error("inputSchema integer bounds are inverted"));
                }
            }
            "boolean" => require_keys(property, &["type"], &[])?,
            _ => return Err(error("inputSchema property type is unsupported")),
        }
    }
    Ok(())
}

fn validate_snapshot(value: &Value) -> Result<(), RemoteSurfaceError> {
    let map = object(value, "snapshot")?;
    require_keys(
        map,
        &[
            "protocol",
            "pluginId",
            "surfaceId",
            "surfaceSessionRef",
            "generation",
            "stateRevision",
            "mode",
            "nodes",
        ],
        &[],
    )?;
    protocol(map, "openmuse.remote-surface/snapshot/v1")?;
    plugin_id(string_field(map, "pluginId", 160)?)?;
    surface_id(string_field(map, "surfaceId", 64)?)?;
    opaque(string_field(map, "surfaceSessionRef", 129)?, 129)?;
    int_field(map, "generation", 1, MAX_SAFE)?;
    opaque(string_field(map, "stateRevision", 129)?, 129)?;
    let mode = string_field(map, "mode", 32)?;
    if !MODES.contains(&mode) {
        return Err(error("mode is unsupported"));
    }
    let mut walk = Walk::default();
    for node in array(map, "nodes")? {
        parse_node(node, 1, &mut walk)?;
    }
    Ok(())
}

#[derive(Default)]
struct Walk {
    ids: BTreeSet<String>,
    count: usize,
}

fn parse_node(value: &Value, depth: usize, walk: &mut Walk) -> Result<(), RemoteSurfaceError> {
    if depth > 6 {
        return Err(error("component tree is too deep"));
    }
    let map = object(value, "node")?;
    require_keys(
        map,
        &["nodeId", "type", "required", "props", "children"],
        &[],
    )?;
    let node_id = node_id(string_field(map, "nodeId", 64)?)?.to_string();
    let node_type = component_type(string_field(map, "type", 33)?)?;
    let required = bool_field(map, "required")?;
    if required && !COMPONENTS.contains(&node_type) {
        return Err(error("required component is not in the v1 vocabulary"));
    }
    parse_props(
        node_type,
        object(map.get("props").unwrap_or(&Value::Null), "props")?,
    )?;
    let children = array(map, "children")?;
    if children.len() > 32 {
        return Err(error("node has too many children"));
    }
    walk.count += 1;
    if walk.count > 64 {
        return Err(error("component tree has too many nodes"));
    }
    if !walk.ids.insert(node_id) {
        return Err(error("duplicate nodeId"));
    }
    for child in children {
        parse_node(child, depth + 1, walk)?;
    }
    Ok(())
}

fn parse_props(node_type: &str, props: &Map<String, Value>) -> Result<(), RemoteSurfaceError> {
    if !COMPONENTS.contains(&node_type) {
        return opaque_props(props);
    }
    match node_type {
        "text" | "markdown" => {
            require_keys(props, &["text"], &[])?;
            string_field(props, "text", 2000)?;
        }
        "image" => parse_image(props)?,
        "gallery" => {
            require_keys(props, &["items"], &[])?;
            let items = array(props, "items")?;
            if items.is_empty() || items.len() > 12 {
                return Err(error("gallery items are out of range"));
            }
            for item in items {
                parse_image(object(item, "gallery item")?)?;
            }
        }
        "video-player" => {
            require_keys(props, &["posterHandle", "durationMs"], &[])?;
            media_handle(string_field(props, "posterHandle", 128)?)?;
            int_field(props, "durationMs", 0, MAX_SAFE)?;
        }
        "document" => {
            require_keys(props, &["label", "mediaHandle"], &[])?;
            string_field(props, "label", 200)?;
            media_handle(string_field(props, "mediaHandle", 128)?)?;
        }
        "link" => {
            require_keys(props, &["label"], &[])?;
            string_field(props, "label", 200)?;
        }
        "list" | "grid" => {
            require_keys(props, &["items"], &[])?;
            let items = array(props, "items")?;
            if items.len() > 32 {
                return Err(error("items exceed 32"));
            }
            for item in items {
                id_label(object(item, "item")?)?;
            }
        }
        "form" => parse_form(props)?,
        "stepper" => parse_steps(props)?,
        "progress" => {
            require_keys(props, &["label", "value"], &[])?;
            string_field(props, "label", 200)?;
            int_field(props, "value", 0, 100)?;
        }
        "status" => {
            require_keys(props, &["label", "state"], &[])?;
            string_field(props, "label", 200)?;
            let state = string_field(props, "state", 32)?;
            if !status_state(state) {
                return Err(error("status state is invalid"));
            }
        }
        "diff" | "compare" => {
            require_keys(props, &["before", "after"], &[])?;
            string_field(props, "before", 2000)?;
            string_field(props, "after", 2000)?;
        }
        "timeline-basic" => parse_timeline(props)?,
        "button" => parse_button(props, false)?,
        "confirmation" => parse_button(props, true)?,
        _ => return Err(error(format!("unsupported component {node_type}"))),
    }
    Ok(())
}

fn parse_image(props: &Map<String, Value>) -> Result<(), RemoteSurfaceError> {
    require_keys(props, &["mediaHandle"], &["alt"])?;
    media_handle(string_field(props, "mediaHandle", 128)?)?;
    if props.contains_key("alt") {
        string_field(props, "alt", 200)?;
    }
    Ok(())
}

fn parse_form(props: &Map<String, Value>) -> Result<(), RemoteSurfaceError> {
    require_keys(props, &["fields"], &[])?;
    let fields = array(props, "fields")?;
    if fields.is_empty() || fields.len() > 16 {
        return Err(error("form fields are out of range"));
    }
    let mut ids = BTreeSet::new();
    for field in fields {
        let id = parse_field(field)?;
        if !ids.insert(id) {
            return Err(error("form field ids must be unique"));
        }
    }
    Ok(())
}

fn parse_field(value: &Value) -> Result<String, RemoteSurfaceError> {
    let map = object(value, "field")?;
    let kind = string_field(map, "kind", 16)?;
    let id = field_id(string_field(map, "id", 65)?)?.to_string();
    string_field(map, "label", 200)?;
    match kind {
        "text" => {
            require_keys(map, &["id", "kind", "label"], &["value"])?;
            if map.contains_key("value") {
                string_field(map, "value", 2000)?;
            }
        }
        "date" => {
            require_keys(map, &["id", "kind", "label"], &["value"])?;
            if map.contains_key("value") {
                let date = string_field(map, "value", 10)?;
                if !calendar_date(date) {
                    return Err(error("date value is invalid"));
                }
            }
        }
        "switch" => {
            require_keys(map, &["id", "kind", "label"], &["value"])?;
            if map.contains_key("value") {
                bool_field(map, "value")?;
            }
        }
        "choice" => {
            require_keys(map, &["id", "kind", "label", "options"], &["value"])?;
            let options = string_list(map, "options", 12, false)?;
            if map.contains_key("value") {
                let selected = string_field(map, "value", 2000)?;
                if !options.iter().any(|option| option == selected) {
                    return Err(error("choice value is not an option"));
                }
            }
        }
        _ => return Err(error("form field kind is unsupported")),
    }
    Ok(id)
}

fn parse_steps(props: &Map<String, Value>) -> Result<(), RemoteSurfaceError> {
    require_keys(props, &["steps"], &[])?;
    let steps = array(props, "steps")?;
    if steps.is_empty() || steps.len() > 12 {
        return Err(error("steps are out of range"));
    }
    for step in steps {
        let map = object(step, "step")?;
        require_keys(map, &["id", "label", "state"], &[])?;
        field_id(string_field(map, "id", 65)?)?;
        string_field(map, "label", 200)?;
        let state = string_field(map, "state", 16)?;
        if !matches!(state, "pending" | "current" | "done") {
            return Err(error("step state is unsupported"));
        }
    }
    Ok(())
}

fn parse_timeline(props: &Map<String, Value>) -> Result<(), RemoteSurfaceError> {
    require_keys(props, &["clips"], &[])?;
    let clips = array(props, "clips")?;
    if clips.is_empty() || clips.len() > 32 {
        return Err(error("clips are out of range"));
    }
    for clip in clips {
        let map = object(clip, "clip")?;
        require_keys(map, &["id", "label", "startMs", "endMs"], &[])?;
        field_id(string_field(map, "id", 65)?)?;
        string_field(map, "label", 200)?;
        let start = int_field(map, "startMs", 0, MAX_SAFE)?;
        let end = int_field(map, "endMs", 0, MAX_SAFE)?;
        if end < start {
            return Err(error("clip bounds are inverted"));
        }
    }
    Ok(())
}

fn parse_button(props: &Map<String, Value>, confirmation: bool) -> Result<(), RemoteSurfaceError> {
    if confirmation {
        require_keys(
            props,
            &["label", "actionId", "prompt"],
            &["input", "inputFromFields"],
        )?;
        string_field(props, "prompt", 500)?;
    } else {
        require_keys(props, &["label", "actionId"], &["input", "inputFromFields"])?;
    }
    string_field(props, "label", 80)?;
    action_id(string_field(props, "actionId", 81)?)?;
    if props.contains_key("inputFromFields") {
        for field in string_list(props, "inputFromFields", 16, false)? {
            field_id(&field)?;
        }
    }
    if let Some(input) = props.get("input") {
        flat_input(object(input, "input")?)?;
    }
    Ok(())
}

fn flat_input(input: &Map<String, Value>) -> Result<(), RemoteSurfaceError> {
    if input.len() > 16 {
        return Err(error("button input has too many fields"));
    }
    for (key, value) in input {
        field_id(key)?;
        match value {
            Value::String(text) => dangerous(text, "input", 2000)?,
            Value::Number(number) => {
                let Some(integer) = number.as_i64() else {
                    return Err(error("input integer is out of range"));
                };
                if !(-MAX_SAFE..=MAX_SAFE).contains(&integer) {
                    return Err(error("input integer is out of range"));
                }
            }
            Value::Bool(_) => {}
            _ => return Err(error("button input values must be flat JSON scalars")),
        }
    }
    Ok(())
}

fn opaque_props(props: &Map<String, Value>) -> Result<(), RemoteSurfaceError> {
    if props.len() > 8 {
        return Err(error("unknown component has too many props"));
    }
    for (key, value) in props {
        field_id(key)?;
        match value {
            Value::String(text) => dangerous(text, "props", 200)?,
            Value::Bool(_) => {}
            Value::Number(number) => {
                let Some(integer) = number.as_i64() else {
                    return Err(error("unknown component props must be short scalars"));
                };
                if !(0..=MAX_SAFE).contains(&integer) {
                    return Err(error("unknown component props must be short scalars"));
                }
            }
            _ => return Err(error("unknown component props must be short scalars")),
        }
    }
    Ok(())
}

fn id_label(map: &Map<String, Value>) -> Result<(), RemoteSurfaceError> {
    require_keys(map, &["id", "label"], &[])?;
    field_id(string_field(map, "id", 65)?)?;
    string_field(map, "label", 200)?;
    Ok(())
}

fn validate_request(value: &Value) -> Result<(), RemoteSurfaceError> {
    let map = object(value, "request")?;
    require_keys(
        map,
        &[
            "protocol",
            "requestId",
            "surfaceSessionRef",
            "generation",
            "actionId",
            "input",
            "expectedStateRevision",
            "idempotencyKey",
            "deadlineMs",
        ],
        &[],
    )?;
    protocol(map, "openmuse.remote-control/request/v1")?;
    opaque(string_field(map, "requestId", 129)?, 129)?;
    opaque(string_field(map, "surfaceSessionRef", 129)?, 129)?;
    int_field(map, "generation", 1, MAX_SAFE)?;
    action_id(string_field(map, "actionId", 81)?)?;
    flat_input(object(map.get("input").unwrap_or(&Value::Null), "input")?)?;
    opaque(string_field(map, "expectedStateRevision", 129)?, 129)?;
    opaque(string_field(map, "idempotencyKey", 129)?, 129)?;
    int_field(map, "deadlineMs", 1, 120_000)?;
    Ok(())
}

fn validate_receipt(value: &Value) -> Result<(), RemoteSurfaceError> {
    let map = object(value, "receipt")?;
    let status = string_field(map, "status", 16)?;
    if !matches!(status, "accepted" | "denied" | "conflict" | "unsupported") {
        return Err(error("receipt status is unsupported"));
    }
    let accepted = status == "accepted";
    let mut required = vec!["protocol", "requestId", "idempotencyKey", "status"];
    if accepted {
        required.extend_from_slice(&["stateRevision", "decisionRef"]);
    } else {
        required.push("errorCode");
    }
    let optional: &[&str] = if accepted { &["jobRef"] } else { &[] };
    require_keys(map, &required, optional)?;
    protocol(map, "openmuse.remote-control/receipt/v1")?;
    opaque(string_field(map, "requestId", 129)?, 129)?;
    opaque(string_field(map, "idempotencyKey", 129)?, 129)?;
    if accepted {
        opaque(string_field(map, "stateRevision", 129)?, 129)?;
        opaque(string_field(map, "decisionRef", 129)?, 129)?;
        if map.contains_key("jobRef") {
            opaque(string_field(map, "jobRef", 129)?, 129)?;
        }
    } else {
        error_code(string_field(map, "errorCode", 64)?)?;
        if map.contains_key("jobRef") {
            return Err(error("a rejected receipt cannot carry a job"));
        }
    }
    Ok(())
}

fn validate_event(value: &Value) -> Result<(), RemoteSurfaceError> {
    let map = object(value, "event")?;
    require_keys(
        map,
        &[
            "protocol",
            "surfaceSessionRef",
            "generation",
            "seq",
            "state",
            "occurredAtMs",
            "stateRevision",
        ],
        &["jobRef"],
    )?;
    protocol(map, "openmuse.remote-control/event/v1")?;
    opaque(string_field(map, "surfaceSessionRef", 129)?, 129)?;
    int_field(map, "generation", 1, MAX_SAFE)?;
    int_field(map, "seq", 1, MAX_SAFE)?;
    let state = string_field(map, "state", 32)?;
    if !EVENT_STATES.contains(&state) {
        return Err(error("event state is unsupported"));
    }
    int_field(map, "occurredAtMs", 0, MAX_SAFE)?;
    opaque(string_field(map, "stateRevision", 129)?, 129)?;
    if map.contains_key("jobRef") {
        opaque(string_field(map, "jobRef", 129)?, 129)?;
    }
    Ok(())
}

fn object<'a>(value: &'a Value, field: &str) -> Result<&'a Map<String, Value>, RemoteSurfaceError> {
    value
        .as_object()
        .ok_or_else(|| error(format!("{field} must be an object")))
}

fn array<'a>(
    map: &'a Map<String, Value>,
    field: &str,
) -> Result<&'a Vec<Value>, RemoteSurfaceError> {
    map.get(field)
        .and_then(Value::as_array)
        .ok_or_else(|| error(format!("{field} must be an array")))
}

fn require_keys(
    map: &Map<String, Value>,
    required: &[&str],
    optional: &[&str],
) -> Result<(), RemoteSurfaceError> {
    for key in required {
        if !map.contains_key(*key) {
            return Err(error(format!("missing fields: {key}")));
        }
    }
    for key in map.keys() {
        if FORBIDDEN.contains(&key.to_ascii_lowercase().as_str()) {
            return Err(error(format!("{key} is not allowed on this contract")));
        }
        if !required.contains(&key.as_str()) && !optional.contains(&key.as_str()) {
            return Err(error(format!("unknown fields: {key}")));
        }
    }
    Ok(())
}

fn protocol(map: &Map<String, Value>, expected: &str) -> Result<(), RemoteSurfaceError> {
    if map.get("protocol").and_then(Value::as_str) != Some(expected) {
        return Err(error(format!("protocol must be {expected}")));
    }
    Ok(())
}

fn string_field<'a>(
    map: &'a Map<String, Value>,
    field: &str,
    max: usize,
) -> Result<&'a str, RemoteSurfaceError> {
    let value = map
        .get(field)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| error(format!("{field} must be a non-empty string")))?;
    dangerous(value, field, max)?;
    Ok(value)
}

fn string_list(
    map: &Map<String, Value>,
    field: &str,
    max: usize,
    allow_empty: bool,
) -> Result<Vec<String>, RemoteSurfaceError> {
    let values = array(map, field)?;
    if values.is_empty() && !allow_empty {
        return Err(error(format!("{field} must not be empty")));
    }
    if values.len() > max {
        return Err(error(format!("{field} exceeds {max} items")));
    }
    let mut result = Vec::with_capacity(values.len());
    for value in values {
        let text = value
            .as_str()
            .filter(|item| !item.is_empty())
            .ok_or_else(|| error(format!("{field} must contain non-empty strings")))?;
        dangerous(text, field, 2000)?;
        result.push(text.to_string());
    }
    if !unique(&result) {
        return Err(error(format!("{field} must contain unique values")));
    }
    Ok(result)
}

fn permissions(
    map: &Map<String, Value>,
    field: &str,
    allow_empty: bool,
) -> Result<(), RemoteSurfaceError> {
    let values = string_list(map, field, 16, allow_empty)?;
    if values.is_empty() && !allow_empty {
        return Err(error(format!("{field} must not be empty")));
    }
    for value in values {
        permission(&value)?;
    }
    Ok(())
}

fn int_field(
    map: &Map<String, Value>,
    field: &str,
    min: i64,
    max: i64,
) -> Result<i64, RemoteSurfaceError> {
    let value = map
        .get(field)
        .and_then(Value::as_i64)
        .ok_or_else(|| error(format!("{field} must be an integer from {min} to {max}")))?;
    if value < min || value > max {
        return Err(error(format!(
            "{field} must be an integer from {min} to {max}"
        )));
    }
    Ok(value)
}

fn optional_int(
    map: &Map<String, Value>,
    field: &str,
    min: i64,
    max: i64,
) -> Result<Option<i64>, RemoteSurfaceError> {
    if !map.contains_key(field) {
        return Ok(None);
    }
    int_field(map, field, min, max).map(Some)
}

fn bool_field(map: &Map<String, Value>, field: &str) -> Result<bool, RemoteSurfaceError> {
    map.get(field)
        .and_then(Value::as_bool)
        .ok_or_else(|| error(format!("{field} must be a boolean")))
}

fn dangerous(value: &str, field: &str, max: usize) -> Result<(), RemoteSurfaceError> {
    if value.chars().count() > max {
        return Err(error(format!("{field} exceeds {max} characters")));
    }
    if value
        .chars()
        .any(|ch| (ch as u32) < 0x20 && ch != '\n' && ch != '\t')
    {
        return Err(error(format!("{field} contains a control character")));
    }
    let lower = value.to_ascii_lowercase();
    if lower.contains("://") || lower.contains("localhost") || lower.contains("127.0.0.1") {
        return Err(error(format!("{field} contains a forbidden locator")));
    }
    Ok(())
}

fn plugin_id(value: &str) -> Result<&str, RemoteSurfaceError> {
    let mut parts = value.split('.');
    let Some(first) = parts.next() else {
        return Err(error("pluginId has an invalid identifier"));
    };
    if !dns_segment(first) || !parts.all(dns_segment) || !value.contains('.') {
        return Err(error("pluginId has an invalid identifier"));
    }
    Ok(value)
}

fn dns_segment(value: &str) -> bool {
    let mut chars = value.chars();
    matches!(chars.next(), Some(ch) if ch.is_ascii_lowercase())
        && chars.all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-')
}

fn surface_id(value: &str) -> Result<&str, RemoteSurfaceError> {
    if (1..=64).contains(&value.len()) && dns_segment(value) {
        Ok(value)
    } else {
        Err(error("surfaceId has an invalid identifier"))
    }
}

fn action_id(value: &str) -> Result<&str, RemoteSurfaceError> {
    let mut chars = value.chars();
    let ok = matches!(chars.next(), Some(ch) if ch.is_ascii_lowercase())
        && chars.all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '.' || ch == '-')
        && (1..=81).contains(&value.len());
    if ok {
        Ok(value)
    } else {
        Err(error("action id has an invalid identifier"))
    }
}

fn opaque(value: &str, max: usize) -> Result<&str, RemoteSurfaceError> {
    let mut chars = value.chars();
    let ok = matches!(chars.next(), Some(ch) if ch.is_ascii_alphanumeric())
        && chars.all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | ':' | '-'))
        && (1..=max).contains(&value.len());
    if ok {
        Ok(value)
    } else {
        Err(error("identifier is invalid"))
    }
}

fn node_id(value: &str) -> Result<&str, RemoteSurfaceError> {
    let mut chars = value.chars();
    let ok = matches!(chars.next(), Some(ch) if ch.is_ascii_alphabetic())
        && chars.all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | '-'))
        && (1..=64).contains(&value.len());
    if ok {
        Ok(value)
    } else {
        Err(error("nodeId has an invalid identifier"))
    }
}

fn media_handle(value: &str) -> Result<&str, RemoteSurfaceError> {
    opaque(value, 128).map_err(|_| error("mediaHandle has an invalid identifier"))
}

fn permission(value: &str) -> Result<&str, RemoteSurfaceError> {
    let mut chars = value.chars();
    let ok = matches!(chars.next(), Some(ch) if ch.is_ascii_lowercase())
        && chars.all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '.' || ch == '-')
        && (1..=81).contains(&value.len());
    if ok {
        Ok(value)
    } else {
        Err(error("permission has an invalid identifier"))
    }
}

fn error_code(value: &str) -> Result<&str, RemoteSurfaceError> {
    let mut chars = value.chars();
    let ok = matches!(chars.next(), Some(ch) if ch.is_ascii_uppercase())
        && chars.clone().count() >= 1
        && chars.all(|ch| ch.is_ascii_uppercase() || ch.is_ascii_digit() || ch == '_')
        && (2..=64).contains(&value.len());
    if ok {
        Ok(value)
    } else {
        Err(error("errorCode has an invalid identifier"))
    }
}

fn field_id(value: &str) -> Result<&str, RemoteSurfaceError> {
    let mut chars = value.chars();
    let ok = matches!(chars.next(), Some(ch) if ch.is_ascii_alphabetic())
        && chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
        && (1..=65).contains(&value.len());
    if ok {
        Ok(value)
    } else {
        Err(error("field id has an invalid identifier"))
    }
}

fn component_type(value: &str) -> Result<&str, RemoteSurfaceError> {
    if (1..=33).contains(&value.len()) && dns_segment(value) {
        Ok(value)
    } else {
        Err(error("type has an invalid identifier"))
    }
}

fn status_state(value: &str) -> bool {
    (1..=32).contains(&value.len())
        && value
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_' || ch == '-')
}

fn calendar_date(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(index, byte)| index == 4 || index == 7 || byte.is_ascii_digit())
}

fn unique(values: &[String]) -> bool {
    values.iter().collect::<BTreeSet<_>>().len() == values.len()
}

fn error(message: impl Into<String>) -> RemoteSurfaceError {
    RemoteSurfaceError::Format(message.into())
}
