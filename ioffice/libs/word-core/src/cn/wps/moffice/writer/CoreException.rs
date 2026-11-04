// Java: import cn.wps.moffice.writer.status.IStatus
use crate::cn::wps::moffice::writer::status::IStatus::*;

// java private static final long serialVersionUID = -8572540021897727666L;
const serialVersionUID: i64 = -8572540021897728000;

// java public class CoreException extends Exception
#[derive(Default)]
pub struct CoreException {
    // java Exception super_
    pub(crate) super_: Exception,
    // java private IStatus mStatus;
    mStatus: Option<Box<dyn IStatus>>,
}

// java public class CoreException extends Exception
impl CoreException {
    // java public  CoreException(IStatus status)
    pub fn CoreException(status: &dyn IStatus) -> Self {
      // java super(status.getMessage(), status.getException());
      // java this.mStatus = status;
      Self { super_: Exception::new(status.getMessage(), status.getException()), mStatus: status }
    }

    // java public final IStatus getStatus()
    pub fn getStatus(&self) -> Box<dyn IStatus> {
      // java return this.mStatus;
      return self.mStatus;
    }

}

impl std::ops::Deref for CoreException {
    type Target = Exception;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for CoreException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
