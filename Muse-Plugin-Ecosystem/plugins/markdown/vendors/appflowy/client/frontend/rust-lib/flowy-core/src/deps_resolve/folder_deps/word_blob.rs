//! Local Word `.docx` blob store. Not a collab document.
//!
//! Layout: `{root}/{view_id}.docx` plus `{view_id}.rev` (hex sha256).

use flowy_error::FlowyError;
use sha2::{Digest, Sha256};
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};
use uuid::Uuid;
use zip::ZipArchive;

pub const WORD_EMPTY_DOCX: &[u8] = include_bytes!("word_empty.docx");

#[derive(Clone, Debug)]
pub struct WordBlobStore {
  root: PathBuf,
}

impl WordBlobStore {
  pub fn new(root: PathBuf) -> Self {
    Self { root }
  }

  pub fn for_user(user_data_dir: &Path, workspace_id: &Uuid) -> Self {
    Self::new(user_data_dir.join("word").join(workspace_id.to_string()))
  }

  pub fn revision_of(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
  }

  pub fn validate_docx(bytes: &[u8]) -> Result<(), FlowyError> {
    if bytes.len() < 4 || &bytes[0..2] != b"PK" {
      return Err(FlowyError::invalid_data().with_context("Word blob must be a zip/docx"));
    }
    Ok(())
  }

  fn docx_path(&self, view_id: &Uuid) -> PathBuf {
    self.root.join(format!("{}.docx", view_id))
  }

  fn rev_path(&self, view_id: &Uuid) -> PathBuf {
    self.root.join(format!("{}.rev", view_id))
  }

  pub async fn put(&self, view_id: &Uuid, bytes: &[u8]) -> Result<String, FlowyError> {
    Self::validate_docx(bytes)?;
    tokio::fs::create_dir_all(&self.root).await.map_err(|e| {
      FlowyError::internal().with_context(format!("create word blob dir: {e}"))
    })?;
    let tmp = self.root.join(format!("{}.docx.tmp", view_id));
    tokio::fs::write(&tmp, bytes).await.map_err(|e| {
      FlowyError::internal().with_context(format!("write word blob tmp: {e}"))
    })?;
    let dest = self.docx_path(view_id);
    tokio::fs::rename(&tmp, &dest).await.map_err(|e| {
      FlowyError::internal().with_context(format!("commit word blob: {e}"))
    })?;
    let rev = Self::revision_of(bytes);
    tokio::fs::write(self.rev_path(view_id), &rev)
      .await
      .map_err(|e| FlowyError::internal().with_context(format!("write word rev: {e}")))?;
    Ok(rev)
  }

  pub async fn get(&self, view_id: &Uuid) -> Result<(Vec<u8>, String), FlowyError> {
    let dest = self.docx_path(view_id);
    let bytes = tokio::fs::read(&dest).await.map_err(|_| {
      FlowyError::record_not_found().with_context(format!("word blob missing: {}", view_id))
    })?;
    Self::validate_docx(&bytes)?;
    let rev = match tokio::fs::read_to_string(self.rev_path(view_id)).await {
      Ok(value) if !value.is_empty() => value,
      _ => Self::revision_of(&bytes),
    };
    Ok((bytes, rev))
  }

  pub async fn delete(&self, view_id: &Uuid) -> Result<(), FlowyError> {
    let _ = tokio::fs::remove_file(self.docx_path(view_id)).await;
    let _ = tokio::fs::remove_file(self.rev_path(view_id)).await;
    Ok(())
  }
}

pub fn plain_text_from_docx(bytes: &[u8]) -> String {
  let mut archive = match ZipArchive::new(Cursor::new(bytes)) {
    Ok(archive) => archive,
    Err(_) => return String::new(),
  };
  let mut file = match archive.by_name("word/document.xml") {
    Ok(file) => file,
    Err(_) => return String::new(),
  };
  let mut xml = String::new();
  if file.read_to_string(&mut xml).is_err() {
    return String::new();
  }
  extract_w_t(&xml)
}

fn extract_w_t(xml: &str) -> String {
  let mut out = String::new();
  let mut rest = xml;
  while let Some(start) = rest.find("<w:t") {
    rest = &rest[start + 4..];
    let Some(gt) = rest.find('>') else { break };
    rest = &rest[gt + 1..];
    let Some(end) = rest.find("</w:t>") else { break };
    out.push_str(&xml_unescape(&rest[..end]));
    rest = &rest[end + 6..];
  }
  out
}

fn xml_unescape(value: &str) -> String {
  value
    .replace("&lt;", "<")
    .replace("&gt;", ">")
    .replace("&quot;", "\"")
    .replace("&apos;", "'")
    .replace("&amp;", "&")
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::convert::TryFrom;

  fn view(n: u128) -> Uuid {
    Uuid::from_u128(n)
  }

  fn tmp_store(label: &str) -> WordBlobStore {
    let dir = std::env::temp_dir().join(format!(
      "muse-word-blob-{}-{}",
      label,
      std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    WordBlobStore::new(dir)
  }

  #[tokio::test]
  async fn put_get_delete_round_trip() {
    let store = tmp_store("round");
    let id = view(1);
    let rev = store.put(&id, WORD_EMPTY_DOCX).await.unwrap();
    assert_eq!(rev.len(), 64);
    let (bytes, got) = store.get(&id).await.unwrap();
    assert_eq!(bytes, WORD_EMPTY_DOCX);
    assert_eq!(got, rev);
    store.delete(&id).await.unwrap();
    assert!(store.get(&id).await.is_err());
    assert!(!store.docx_path(&id).exists());
  }

  #[test]
  fn empty_template_plain_text() {
    assert!(plain_text_from_docx(WORD_EMPTY_DOCX).is_empty());
  }

  #[tokio::test]
  async fn reject_non_zip() {
    let store = tmp_store("bad");
    let id = view(2);
    let err = store.put(&id, b"not-a-docx").await.unwrap_err();
    assert!(!store.docx_path(&id).exists());
    assert!(err.msg.to_lowercase().contains("zip"));
  }

  #[tokio::test]
  async fn duplicate_copies_bytes() {
    let store = tmp_store("dup");
    let src = view(3);
    let dst = view(4);
    store.put(&src, WORD_EMPTY_DOCX).await.unwrap();
    let (bytes, _) = store.get(&src).await.unwrap();
    store.put(&dst, &bytes).await.unwrap();
    let (copy, _) = store.get(&dst).await.unwrap();
    assert_eq!(copy, bytes);
    assert_ne!(src, dst);
  }

  #[test]
  fn try_from_layout_9_is_word() {
    assert_eq!(
      collab_folder::ViewLayout::try_from(9i64).unwrap(),
      collab_folder::ViewLayout::Word
    );
    assert!(collab_folder::ViewLayout::try_from(5i64).is_err());
    assert!(collab_folder::ViewLayout::Word.is_office_blob());
    assert_eq!(
      collab_folder::ViewLayout::try_from(10i64).unwrap(),
      collab_folder::ViewLayout::Excel
    );
  }
}
