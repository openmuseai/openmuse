// Java: import cn.wps.io.xwpf.util.ZipEntryFutureTask
// Java: import cn.wps.io.xwpf.util.ZipInputStreamCallable
// Java: import cn.wps.moffice.kfs.zip.ZipInputStream
// Java: import java.util.concurrent.ExecutionException
// Java: import java.util.concurrent.ExecutorService
// Java: import java.util.concurrent.Executors
// Java: import java.util.concurrent.FutureTask
// Java: import java.util.concurrent.TimeUnit
// Java: import java.util.concurrent.TimeoutException

use core_libs_kfs::kfs::zip::ZipInputStream::ZipInputStream;
use crate::cn::wps::io::xwpf::util::ZipEntryFutureTask::ZipEntryFutureTask;
use std::time::{Duration, Instant};

// Java: public class Helper
pub struct Helper {
}

// Java: public class Helper
impl Helper {
    // Java: Helper()
    pub fn new() -> Self {
      // Java: super();
      Helper {}
    }

    // Java: static void close(ZipInputStream inp)
    pub fn close(inp: &mut ZipInputStream) {
      // Java: ExecutorService executor = Executors.newSingleThreadExecutor();
      // Java: FutureTask<Void> futureTask = new ZipEntryFutureTask(new ZipInputStreamCallable(inp));
      // Rust: Java 的「任务」= `inp.close()`（ZipInputStreamCallable.call() 的全部内容）；
      //       这里保留同一份预算常量作为 futureTask 的对应物。
      let futureTask_timeout_ms = ZipEntryFutureTask::TIMEOUT as u64;
      let started_at = Instant::now();
      // Java: executor.execute(futureTask);
      // Java: try
      // Java: futureTask.get(5000L, TimeUnit.MILLISECONDS);
      // Rust: kfs `ZipInputStream` 不是 Send，无法照搬「搬到另一个线程再 interrupt」
      //       （`thread::scope` 退出时会 join，等于把挂死的读变成永久阻塞），故直接调用。
      inp.close();
      // Java: catch (InterruptedException var9)
      // Java: catch (ExecutionException var10)
      // Java: catch (TimeoutException var11)
      // Rust: Java 超时会 cancel(true) 中断；Rust 无法中断已返回的 close()，
      //       因此把「是否超出同一预算」变成可观测的断言（超时语义的偏差见 §1.7 登记）。
      debug_assert!(
          started_at.elapsed() <= Duration::from_millis(futureTask_timeout_ms),
          "Helper.close(ZipInputStream) exceeded the Java 5000ms budget"
      );
      // Java: finally
      // Java: executor.shutdown();
    }

}
