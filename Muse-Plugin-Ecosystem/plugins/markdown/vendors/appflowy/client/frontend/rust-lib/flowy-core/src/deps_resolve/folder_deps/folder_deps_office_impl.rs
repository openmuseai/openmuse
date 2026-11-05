use crate::deps_resolve::folder_deps::office_blob::{OfficeBlobKind, OfficeBlobStore};
use bytes::Bytes;
use collab::entity::EncodedCollab;
use collab_folder::ViewLayout;
use flowy_error::FlowyError;
use flowy_folder::entities::CreateViewParams;
use flowy_folder::share::ImportType;
use flowy_folder::view_operation::{FolderOperationHandler, ImportedData, ViewData};
use flowy_user::services::authenticate_user::AuthenticateUser;
use lib_infra::async_trait::async_trait;
use std::sync::Weak;
use uuid::Uuid;

/// Folder handler for office slots that are not yet engine-bound.
/// Word stays on `WordFolderOperation` so `{user_data}/word/` does not move.
pub struct OfficeFolderOperation {
  user: Weak<AuthenticateUser>,
  kind: OfficeBlobKind,
}

impl OfficeFolderOperation {
  pub fn excel(user: Weak<AuthenticateUser>) -> Self {
    Self {
      user,
      kind: OfficeBlobKind::Excel,
    }
  }

  pub fn slides(user: Weak<AuthenticateUser>) -> Self {
    Self {
      user,
      kind: OfficeBlobKind::Slides,
    }
  }

  pub fn pdf(user: Weak<AuthenticateUser>) -> Self {
    Self {
      user,
      kind: OfficeBlobKind::Pdf,
    }
  }

  fn store(&self) -> Result<OfficeBlobStore, FlowyError> {
    let user = self.user.upgrade().ok_or_else(FlowyError::ref_drop)?;
    let workspace = user.workspace_id()?;
    Ok(OfficeBlobStore::for_user(
      &user.get_user_data_dir()?,
      &workspace,
      self.kind,
    ))
  }
}

#[async_trait]
impl FolderOperationHandler for OfficeFolderOperation {
  fn name(&self) -> &str {
    match self.kind {
      OfficeBlobKind::Excel => "ExcelFolderOperationHandler",
      OfficeBlobKind::Slides => "SlidesFolderOperationHandler",
      OfficeBlobKind::Pdf => "PdfFolderOperationHandler",
    }
  }

  async fn open_view(&self, _view_id: &Uuid) -> Result<(), FlowyError> {
    Ok(())
  }

  async fn close_view(&self, _view_id: &Uuid) -> Result<(), FlowyError> {
    Ok(())
  }

  async fn delete_view(&self, view_id: &Uuid) -> Result<(), FlowyError> {
    self.store()?.delete(view_id).await
  }

  async fn duplicate_view(&self, view_id: &Uuid) -> Result<Bytes, FlowyError> {
    let (bytes, _) = self.store()?.get(view_id).await?;
    Ok(Bytes::from(bytes))
  }

  async fn create_view_with_view_data(
    &self,
    _user_id: i64,
    params: CreateViewParams,
  ) -> Result<Option<EncodedCollab>, FlowyError> {
    let bytes = match &params.initial_data {
      ViewData::DuplicateData(data) | ViewData::Data(data) if !data.is_empty() => data.to_vec(),
      _ => self.kind.empty_template().to_vec(),
    };
    self.store()?.put(&params.view_id, &bytes).await?;
    Ok(None)
  }

  async fn create_default_view(
    &self,
    _user_id: i64,
    _parent_view_id: &Uuid,
    view_id: &Uuid,
    _name: &str,
    _layout: ViewLayout,
  ) -> Result<(), FlowyError> {
    self
      .store()?
      .put(view_id, self.kind.empty_template())
      .await?;
    Ok(())
  }

  async fn import_from_bytes(
    &self,
    _uid: i64,
    view_id: &Uuid,
    _name: &str,
    _import_type: ImportType,
    bytes: Vec<u8>,
  ) -> Result<Vec<ImportedData>, FlowyError> {
    self.store()?.put(view_id, &bytes).await?;
    Ok(vec![])
  }

  async fn import_from_file_path(
    &self,
    view_id: &str,
    _name: &str,
    path: String,
  ) -> Result<(), FlowyError> {
    let bytes = tokio::fs::read(&path).await.map_err(|e| {
      FlowyError::internal().with_context(format!("read {} import: {e}", self.kind.subdir()))
    })?;
    let view_id = Uuid::parse_str(view_id).map_err(|e| {
      FlowyError::internal().with_context(format!("{} import view id: {e}", self.kind.subdir()))
    })?;
    self.store()?.put(&view_id, &bytes).await?;
    Ok(())
  }
}
