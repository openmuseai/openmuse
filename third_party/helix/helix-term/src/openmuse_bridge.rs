//! Optional, authenticated Host event channel. The PTY remains rendering-only.
//! Never connect to an address supplied by an untrusted workspace file.

use helix_view::Editor;
use serde_json::json;
use std::{
    io::{Read, Write},
    net::{IpAddr, SocketAddr, TcpStream},
    path::Path,
    thread,
    time::Duration,
};
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct OpenMuseCommand {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub revision: i32,
}

impl OpenMuseCommand {
    fn parse(bytes: &[u8]) -> Option<Self> {
        let message: serde_json::Value = serde_json::from_slice(bytes).ok()?;
        if message.get("version")?.as_u64()? != 1 || message.get("type")?.as_str()? != "command" {
            return None;
        }
        let id = message.get("id")?.as_u64()?;
        let name = message.get("command")?.as_str()?.to_owned();
        let path = message.get("path")?.as_str()?.to_owned();
        let revision = i32::try_from(message.get("revision")?.as_i64()?).ok()?;
        if id == 0 || name.len() > 32 || path.len() > 4096 || !Path::new(&path).is_absolute() {
            return None;
        }
        Some(Self {
            id,
            name,
            path,
            revision,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ResourceState {
    path: Option<String>,
    dirty: bool,
    revision: i32,
}

impl ResourceState {
    fn current(editor: &Editor) -> Self {
        let doc = current_ref!(editor).1;
        Self {
            path: doc.path().map(|path| path.to_string_lossy().into_owned()),
            dirty: doc.is_modified(),
            revision: doc.version(),
        }
    }
}

pub struct OpenMuseBridge {
    stream: TcpStream,
    previous: Option<ResourceState>,
}

impl OpenMuseBridge {
    pub fn from_environment(commands: mpsc::Sender<OpenMuseCommand>) -> Option<Self> {
        let address = std::env::var("OPENMUSE_HELIX_CONTROL_ADDR").ok()?;
        let token = std::env::var("OPENMUSE_HELIX_CONTROL_TOKEN").ok()?;
        if token.len() < 32 || token.len() > 128 || !token.is_ascii() {
            return None;
        }
        let address: SocketAddr = address.parse().ok()?;
        if address.ip() != IpAddr::V4(std::net::Ipv4Addr::LOCALHOST) {
            return None;
        }
        let stream = TcpStream::connect_timeout(&address, Duration::from_millis(200)).ok()?;
        stream
            .set_write_timeout(Some(Duration::from_millis(20)))
            .ok()?;
        let mut bridge = Self {
            stream,
            previous: None,
        };
        bridge
            .send(json!({
                "version": 1,
                "type": "hello",
                "token": token,
                "pid": std::process::id(),
            }))
            .ok()?;
        let mut reader = bridge.stream.try_clone().ok()?;
        thread::spawn(move || {
            let mut chunk = [0u8; 4096];
            let mut pending = Vec::new();
            while let Ok(length) = reader.read(&mut chunk) {
                if length == 0 {
                    break;
                }
                for &byte in &chunk[..length] {
                    if byte == b'\n' {
                        if let Some(command) = OpenMuseCommand::parse(&pending) {
                            if commands.blocking_send(command).is_err() {
                                return;
                            }
                        }
                        pending.clear();
                    } else {
                        pending.push(byte);
                        if pending.len() > 16384 {
                            return;
                        }
                    }
                }
            }
        });
        Some(bridge)
    }

    fn send(&mut self, value: serde_json::Value) -> std::io::Result<()> {
        let mut bytes = serde_json::to_vec(&value).map_err(std::io::Error::other)?;
        bytes.push(b'\n');
        self.stream.write_all(&bytes)
    }

    pub fn publish_state(&mut self, editor: &Editor) -> std::io::Result<()> {
        let current = ResourceState::current(editor);
        if self.previous.as_ref() == Some(&current) {
            return Ok(());
        }
        self.send(json!({
            "version": 1,
            "type": "state",
            "path": current.path,
            "dirty": current.dirty,
            "revision": current.revision,
        }))?;
        self.previous = Some(current);
        Ok(())
    }

    pub fn publish_save(&mut self, path: &std::path::Path, revision: usize) -> std::io::Result<()> {
        self.send(json!({
            "version": 1,
            "type": "saved",
            "path": path.to_string_lossy(),
            "revision": revision,
        }))
    }

    pub fn publish_result(
        &mut self,
        id: u64,
        result: Result<(), &'static str>,
        editor: &Editor,
    ) -> std::io::Result<()> {
        let state = ResourceState::current(editor);
        self.send(json!({
            "version": 1,
            "type": "result",
            "id": id,
            "ok": result.is_ok(),
            "error": result.err(),
            "path": state.path,
            "revision": state.revision,
            "dirty": state.dirty,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_state_deduplicates_all_fields() {
        let first = ResourceState {
            path: Some("/a.rs".to_owned()),
            dirty: false,
            revision: 1,
        };
        assert_eq!(first, first.clone());
        assert_ne!(
            first,
            ResourceState {
                dirty: true,
                ..first.clone()
            }
        );
        assert_ne!(
            first,
            ResourceState {
                revision: 2,
                ..first.clone()
            }
        );
        assert_ne!(
            first,
            ResourceState {
                path: Some("/b.rs".to_owned()),
                ..first
            }
        );
    }

    #[test]
    fn command_parser_requires_version_path_and_revision() {
        let path = std::env::current_dir().unwrap().join("a.rs");
        let valid = serde_json::to_vec(&json!({
            "version": 1,
            "type": "command",
            "id": 7,
            "command": "save",
            "path": path,
            "revision": 2,
        }))
        .unwrap();
        let command = OpenMuseCommand::parse(&valid).unwrap();
        assert_eq!(command.id, 7);
        assert_eq!(command.name, "save");
        assert_eq!(command.revision, 2);
        assert_eq!(command.path, path.to_string_lossy());
        assert!(OpenMuseCommand::parse(br#"{"version":2,"type":"command","id":7,"command":"save","path":"/a.rs","revision":2}"#).is_none());
        assert!(OpenMuseCommand::parse(
            br#"{"version":1,"type":"command","id":7,"command":"save","path":"a.rs","revision":2}"#
        )
        .is_none());
    }
}
