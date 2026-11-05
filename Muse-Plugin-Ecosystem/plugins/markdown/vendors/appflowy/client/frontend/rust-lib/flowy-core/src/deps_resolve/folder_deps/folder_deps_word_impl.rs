use crate::deps_resolve::folder_deps::word_blob::{WordBlobStore, WORD_EMPTY_DOCX};
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

pub struct WordFolderOperation(pub Weak<AuthenticateUser>);

impl WordFolderOperation {
  fn store(&self) -> Result<WordBlobStore, FlowyError> {
    let user = self.0.upgrade().ok_or_else(FlowyError::ref_drop)?;
    let workspace = user.workspace_id()?;
    Ok(WordBlobStore::for_user(&user.get_user_data_dir()?, &workspace))
  }
}

#[async_trait]
impl FolderOperationHandler for WordFolderOperation {
  fn name(&self) -> &str {
    "WordFolderOperationHandler"
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
      _ => WORD_EMPTY_DOCX.to_vec(),
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
    self.store()?.put(view_id, WORD_EMPTY_DOCX).await?;
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
      FlowyError::internal().with_context(format!("read word import: {e}"))
    })?;
    let view_id = Uuid::parse_str(view_id).map_err(|e| {
      FlowyError::internal().with_context(format!("word import view id: {e}"))
    })?;
    self.store()?.put(&view_id, &bytes).await?;
    Ok(())
  }
}
