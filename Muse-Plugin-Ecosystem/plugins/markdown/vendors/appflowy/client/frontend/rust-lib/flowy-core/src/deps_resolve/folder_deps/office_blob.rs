//! Generic office blob store. Word keeps `word_blob.rs` so existing
//! `{userData}/word/` paths do not move. Excel / Slides / PDF use this store.

use flowy_error::FlowyError;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Empty PK zip (EOCD only). Valid zip magic before an engine exists.
pub const EMPTY_ZIP: &[u8] = &[
  0x50, 0x4b, 0x05, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

pub const EMPTY_PDF: &[u8] =
  b"%PDF-1.4\n1 0 obj<</Type/Catalog>>endobj\ntrailer<</Root 1 0 R>>\n%%EOF\n";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OfficeBlobKind {
  Excel,
  Slides,
  Pdf,
}

impl OfficeBlobKind {
  pub fn subdir(self) -> &'static str {
    match self {
      Self::Excel => "excel",
      Self::Slides => "slides",
      Self::Pdf => "pdf",
    }
  }

  pub fn extension(self) -> &'static str {
    match self {
      Self::Excel => "xlsx",
      Self::Slides => "pptx",
      Self::Pdf => "pdf",
    }
  }

  pub fn empty_template(self) -> &'static [u8] {
    match self {
      Self::Excel | Self::Slides => EMPTY_ZIP,
      Self::Pdf => EMPTY_PDF,
    }
  }

  pub fn layout(self) -> collab_folder::ViewLayout {
    match self {
      Self::Excel => collab_folder::ViewLayout::Excel,
      Self::Slides => collab_folder::ViewLayout::Slides,
      Self::Pdf => collab_folder::ViewLayout::Pdf,
    }
  }

  pub fn validate(self, bytes: &[u8]) -> Result<(), FlowyError> {
    match self {
      Self::Excel | Self::Slides => {
        if bytes.len() < 4 || &bytes[0..2] != b"PK" {
          return Err(
            FlowyError::invalid_data()
              .with_context(format!("{} blob must be a zip", self.subdir())),
          );
        }
      },
      Self::Pdf => {
        if bytes.len() < 5 || &bytes[0..4] != b"%PDF" {
          return Err(FlowyError::invalid_data().with_context("PDF blob must start with %PDF"));
        }
      },
    }
    Ok(())
  }
}

#[derive(Clone, Debug)]
pub struct OfficeBlobStore {
  root: PathBuf,
  kind: OfficeBlobKind,
}

impl OfficeBlobStore {
  pub fn new(root: PathBuf, kind: OfficeBlobKind) -> Self {
    Self { root, kind }
  }

  pub fn for_user(user_data_dir: &Path, workspace_id: &Uuid, kind: OfficeBlobKind) -> Self {
    Self::new(
      user_data_dir.join(kind.subdir()).join(workspace_id.to_string()),
      kind,
    )
  }

  pub fn revision_of(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
  }

  fn blob_path(&self, view_id: &Uuid) -> PathBuf {
    self.root.join(format!("{}.{}", view_id, self.kind.extension()))
  }

  fn rev_path(&self, view_id: &Uuid) -> PathBuf {
    self.root.join(format!("{}.rev", view_id))
  }

  pub async fn put(&self, view_id: &Uuid, bytes: &[u8]) -> Result<String, FlowyError> {
    self.kind.validate(bytes)?;
    tokio::fs::create_dir_all(&self.root).await.map_err(|e| {
      FlowyError::internal().with_context(format!("create {} blob dir: {e}", self.kind.subdir()))
    })?;
    let tmp = self
      .root
      .join(format!("{}.{}.tmp", view_id, self.kind.extension()));
    tokio::fs::write(&tmp, bytes).await.map_err(|e| {
      FlowyError::internal().with_context(format!("write {} blob tmp: {e}", self.kind.subdir()))
    })?;
    let dest = self.blob_path(view_id);
    tokio::fs::rename(&tmp, &dest).await.map_err(|e| {
      FlowyError::internal().with_context(format!("commit {} blob: {e}", self.kind.subdir()))
    })?;
    let rev = Self::revision_of(bytes);
    tokio::fs::write(self.rev_path(view_id), &rev)
      .await
      .map_err(|e| {
        FlowyError::internal().with_context(format!("write {} rev: {e}", self.kind.subdir()))
      })?;
    Ok(rev)
  }

  pub async fn get(&self, view_id: &Uuid) -> Result<(Vec<u8>, String), FlowyError> {
    let dest = self.blob_path(view_id);
    let bytes = tokio::fs::read(&dest).await.map_err(|_| {
      FlowyError::record_not_found().with_context(format!(
        "{} blob missing: {}",
        self.kind.subdir(),
        view_id
      ))
    })?;
    self.kind.validate(&bytes)?;
    let rev = match tokio::fs::read_to_string(self.rev_path(view_id)).await {
      Ok(value) if !value.is_empty() => value,
      _ => Self::revision_of(&bytes),
    };
    Ok((bytes, rev))
  }

  pub async fn delete(&self, view_id: &Uuid) -> Result<(), FlowyError> {
    let _ = tokio::fs::remove_file(self.blob_path(view_id)).await;
    let _ = tokio::fs::remove_file(self.rev_path(view_id)).await;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::convert::TryFrom;

  fn view(n: u128) -> Uuid {
    Uuid::from_u128(n)
  }

  fn tmp_store(kind: OfficeBlobKind, label: &str) -> OfficeBlobStore {
    let dir = std::env::temp_dir().join(format!(
      "muse-office-blob-{}-{}-{}",
      kind.subdir(),
      label,
      std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    OfficeBlobStore::new(dir, kind)
  }

  #[tokio::test]
  async fn excel_zip_round_trip() {
    let store = tmp_store(OfficeBlobKind::Excel, "round");
    let id = view(1);
    store.put(&id, EMPTY_ZIP).await.unwrap();
    let (bytes, _) = store.get(&id).await.unwrap();
    assert_eq!(bytes, EMPTY_ZIP);
    store.delete(&id).await.unwrap();
    assert!(store.get(&id).await.is_err());
  }

  #[tokio::test]
  async fn pdf_rejects_non_pdf() {
    let store = tmp_store(OfficeBlobKind::Pdf, "bad");
    let err = store.put(&view(2), b"not-a-pdf").await.unwrap_err();
    assert!(err.msg.to_lowercase().contains("pdf"));
  }

  #[test]
  fn layouts_are_reserved() {
    assert_eq!(OfficeBlobKind::Excel.layout() as u8, 10);
    assert_eq!(OfficeBlobKind::Slides.layout() as u8, 11);
    assert_eq!(OfficeBlobKind::Pdf.layout() as u8, 12);
    assert_eq!(
      collab_folder::ViewLayout::try_from(10i64).unwrap(),
      collab_folder::ViewLayout::Excel
    );
    assert_eq!(
      collab_folder::ViewLayout::try_from(11i64).unwrap(),
      collab_folder::ViewLayout::Slides
    );
    assert_eq!(
      collab_folder::ViewLayout::try_from(12i64).unwrap(),
      collab_folder::ViewLayout::Pdf
    );
  }
}
