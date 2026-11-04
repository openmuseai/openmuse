# Word-Core Rust Rewrite 规范文档

## 背景与目的

`koffice/writer/core` 是纯 Java 语言实现的 Android 平台完整 Word 渲染管线，现在要迁移到基于 Flutter + Rust 的新架构中，做到 Web、Desktop、Mobile 三端统一的实现。

因为原始代码量很多（~2855 个 Java 文件），无法一次性 rewrite 并保持逻辑 100%，所以制定了 `libs/word-core/RUST_REWRITE_PLAN.md`，按照依赖关系和优先级，逐步进行 rewrite。

### Java 模块拆分现状


| 模块                                | 文件数 | 功能                      |
| --------------------------------- | --- | ----------------------- |
| `koffice/writer/core/src`         | 863 | 文档模型 + 布局核心 + 视图        |
| `koffice/writer/core-layout/src`  | 470 | 排版引擎                    |
| `koffice/writer/core-io/src`      | 942 | 格式读写（DOCX/DOC/HTML/RTF） |
| `koffice/writer/core-data/src`    | 362 | PLC/属性/字段/表格数据          |
| `koffice/writer/core-cache/src`   | 95  | 排版快照（TypoSnapshot）      |
| `koffice/writer/core-render/src`  | 69  | Canvas 渲染               |
| `koffice/writer/core-service/src` | 54  | 命中测试/定位服务               |


详细的模块架构文档在 `koffice/writer/core/MODULE_ARCHITECTURE.md`。
详细的渲染管线追踪在 `libs/word-core/RENDERING_PIPELINE_TRACE.md`。

---

## 模块 Rewrite 流程

为了更加好地追踪重写的进度与功能/逻辑完整性
`RUST_REWRITE_PLAN.md` 是按功能划分代码库，并制定粗略计划。 现在要求对应以下的代码库：

`koffice/writer/core/src`
`koffice/writer/core-layout/src`
`koffice/writer/core-io/src`
`koffice/writer/core-data/src`
`koffice/writer/core-cache/src`
`koffice/writer/core-render/src`
`koffice/writer/core-service/src`

每个代码库里面的子模块分别列出重写计划（是全量重写，重写一部分（多少代码文件）），重写进度，测试是否通过等），并且把这些信息重新更新到`RUST_REWRITE_PLAN.md`,形成一个更加完整的数据面板。 然后按照以下流程逐步进行重写：

### Phase 1: Before Rewrite（准备阶段）

1.1 检查 `RUST_REWRITE_PLAN.md`，找出下一步需要 Rewrite 的子 功能/子模块。

1.2 详细检查该子功能对应的 `koffice/writer` Java 代码，分析其依赖关系：

- 该子模块依赖的其他子模块是否已完成 Rewrite？
- 如果所有依赖都已 Rewrite，该子模块可以开始。
- 如果有未完成的依赖，使用 trait 抽象或 stub 临时隔离。

1.3 详细分析 Java 源码，梳理以下内容，并输出 `REWRITE_DESIGN.md` 到 `libs/word-core` 对应子目录：

- Java 源文件清单（文件名、行数、核心职责）
- 类/接口继承关系图
- 外部依赖列表（哪些 Java API 需要 Rust 替代）
- 数据流图（输入 → 处理 → 输出）
- Rust 目标文件结构和 Java→Rust 类映射表

1.4 当选定功能模块进行改写后，修改 `RUST_REWRITE_PLAN.md`，在对应模块加入“改写中” 状态。

### Phase 2: Rewrite（重写阶段）

2.1 严格遵守下文 "Rewrite 原则" 进行重写。

2.2 在每完成一个代码文件重写后，与 `koffice` 对应的 Java 功能代码进行一对一对照：

- 逐目录对照：Rust 文件的目录结构 和 java 对应
- 逐类对照：Rust struct/enum ↔ Java class/interface
- 逐方法对照：Rust fn ↔ Java method，确保函数签名，数量，函数控制流一致
- 逐字段对照：Rust field ↔ Java field，确保类型和语义一致
   另外，包括字段/函数的位置等，都需要保持一致，总之要保持“外观尽可能一致”和“逻辑100%一致”
   以上需要使用 Python 脚本来进行验证。
- 逐逻辑对照：对于复杂逻辑的方法，Rust Line↔ Java Line 需要“逐行”对照，在Rust 代码处添加对应的Java 代码注释

   **Python 验证脚本** (`libs/word-core/tools/verify_rewrite.py`) 规格说明：

   **用法**: `python tools/verify_rewrite.py <java_dir> <rust_dir>`

   **验证内容**:
   a. **文件对照**: 检查每个 Java `.java` 文件是否有对应的 Rust `.rs` 文件（命名规则: `PascalCase.java` → `snake_case.rs`）
   b. **类/接口对照**: 提取 Java `class`/`interface` 和 Rust `struct`/`enum`/`trait`，验证数量和名称映射
   c. **字段对照**: 提取所有字段声明，验证数量、顺序、类型映射是否一致
   d. **方法对照**: 提取所有方法签名，验证数量、顺序、参数数量、返回值类型是否一致
   e. **位置顺序**: 验证字段和方法在文件中的出现顺序是否与 Java 一致

   **输出格式**: 生成对照报告（`verify_report.md`），使用以下标记：

- ✅ 完全匹配
- ❌ 缺失（Java 有但 Rust 缺少）
- ⚠️ 不一致（顺序错误、参数不匹配等）

   **集成**: 每次完成一个目录的重写后，必须运行验证脚本并确保 **0 个 ❌ 错误** 后才能标记该目录为"完成"。
   验证报告需保存在对应 Rust 目录中，供后续审查。
   libs\word-core\src\core\io 中是已经完成重新的目录，请参照这个它作为样例进行重写。

2.3 Rust 方法体重写与验证。
   每个Rust 文件依次进行处理，不要使用脚本，当完成一个文件后再进行下一个文件的改写，每个文件的方法依次改写（不要使用脚本批次处理）。
   根据对应的Java 的方法的方法体， 实现对应 Rust 方法，禁止遗留 todo，Manual_Rewrite ，placeholder等标签，或者遗留空实现，空代码块。
   Rust 的方法体需要100% 与对应的Java 方法体逻辑保持一致，并且实现逐行一一对照（参照以下例子），保持“外观尽可能一致”和“逻辑100%一致”，例如以下例子：
   DocumentPreliminaryLoader 的 dispose() 方法 :
  ```
   public void dispose() {
      if (this.poifsFileSystem != null) {
         this.poifsFileSystem.dispose();
         this.poifsFileSystem = null;
      }

      if (this.mDiskDoc != null) {
         this.mDiskDoc.dispose();
         this.mDiskDoc = null;
      }

      if (this.isEncryption()) {
         this.mOnlineSecurityTool = null;
      }

      if (VersionState.isProVersion() && this.isProEncryption()) {
         this.mDocumentCallback.dispose();
      }

      this.docx_diskDoc = null;
   }
  ```
  需要转换为对应的Rust 方法:
  ```
   pub fn dispose(&mut self) {
        let _ = &self;
        // Java: if (this.poifsFileSystem != null)
        if self.poifs_file_system.is_some() {
            // Java: this.poifsFileSystem.dispose();
            self.poifsFileSystem.dispose();
            // Java: this.poifsFileSystem = null;
            self.poifs_file_system = None;
        }
        // Java: if (this.mDiskDoc != null)
        if self.disk_doc.is_some() {
            // Java: this.mDiskDoc.dispose();
            self.mDiskDoc.dispose();
            // Java: this.mDiskDoc = null;
            self.disk_doc = None;
        }
        // Java: if (this.isEncryption())
        if self.is_encryption() {
            // Java: this.mOnlineSecurityTool = null;
            self.online_security_tool = None;
        }
        // Java: if (VersionState.isProVersion() && this.isProEncryption())
        if VersionState.isProVersion() && self.is_pro_encryption() {
            // Java: this.mDocumentCallback.dispose();
        self.mDocumentCallback.dispose();
        }
        // Java: this.docx_diskDoc = null;
        self.docx_disk_doc = None;
    }
  ```
  因为是Rust 方法逐行一一对照，按照逐行对照的原则重新改写 libs\tools\verify_rewrite.py 的 _compare_method_logic 方法，以保证逻辑100% 一致。

2.4 对照通过后，编写单元测试（`#[test]`），覆盖每个公开接口。

### Phase 3: After Rewrite（验证阶段）

  修改 `RUST_REWRITE_PLAN.md`，在对应模块加入“测试中” 状态。
通过以下 **双管线对比测试** 方案验证逻辑一致性：

**Step 1: Rust 端测试代码**
在 `libs/word-core` 中，为已重写的子模块添加对比测试接口：

```rust
// 示例：model 层对比接口
#[cfg(feature = "uniffi")]
pub fn parse_and_serialize_model(docx_bytes: Vec<u8>) -> String {
    let model = parse::parse_docx(&docx_bytes).unwrap();
    serde_json::to_string_pretty(&model).unwrap()
}
```

通过 UniFFI 生成 Kotlin 绑定，编译为 `.so` 动态库。

**Step 2: Java 端测试代码**
在 `koffice/writer/core-xxx` 中添加对应的 JSON 序列化接口：

```java
// 示例：Java 端输出等价 JSON
public static String serializeModelToJson(String docxPath) {
    TextDocument doc = openDocument(docxPath);
    return new Gson().toJson(buildModelSnapshot(doc));
}
```

**Step 3: 真机对比测试**
将两套测试代码编译进同一个 APK（writer-demo），在真机上执行：

```
adb push test-samples/*.docx /sdcard/Download/test-samples/

adb shell am start -n cn.wps.moffice.writer/.WriterDemoActivity \
    --es testMode "compare" \
    --es sampleDir "/sdcard/Download/test-samples/"
```

测试流程：

```
对于每个 sample.docx:
  1. Java 管线: DOCX → Java parse → Java model → JSON → /sdcard/test-output/java/sample.json
  2. Rust 管线: DOCX → UniFFI → Rust parse → Rust model → JSON → /sdcard/test-output/rust/sample.json
  3. 逐字段比对: diff java/sample.json rust/sample.json
  4. 输出报告: /sdcard/test-output/compare_report.txt
```

**对比测试分三阶段逐步推进：**


| 阶段      | 对比内容                    | 容差          | 通过标准         |
| ------- | ----------------------- | ----------- | ------------ |
| Stage 1 | DocumentModel JSON      | 严格相等        | 所有字段 100% 匹配 |
| Stage 2 | TypoSnapshot JSON（排版坐标） | x/y ±0.01px | 每行坐标精确匹配     |
| Stage 3 | RenderCommand 列表        | 指令序列相等      | 指令数量和参数完全一致  |


**自动化工具：**

- `libs/word-core/tests/compare_outputs.py` — 读取两组 JSON，逐字段比对，输出差异报告
- `libs/word-core/tests/run_device_test.sh` — 自动 push 样本、触发测试、拉取结果
- 测试样本存放在 `libs/word-core/test-samples/`

如果测试通过，

1. 提交代码，然后重复此流程处理下一个子模块。
2. 修改 `RUST_REWRITE_PLAN.md`，在对应模块加入“改写完成” 状态。
3. 重新开始这个流程，寻找下一个功能/模块进行改写。

---

## Rewrite 原则

按照最小粒度原则进行计划，遵循测试驱动，要求 100% 与原有逻辑保持一致。

### 1. 100% 逻辑保持原则

核心渲染管线的逻辑必须 100% 兼容，最终渲染结果与原管线像素级对齐。

- 数值计算使用相同的精度（Java double ↔ Rust f64）
- 整数溢出行为需要显式处理（Java wrapping ↔ Rust wrapping_add 等）
- 浮点比较使用与 Java 相同的 epsilon（不引入额外的近似）

### 2. 功能逻辑与溯源原则

渲染管线逻辑必须严格按照原有逻辑迁移，严禁私自修改核心渲染管线逻辑。

- 每个 Rust 函数必须能追溯到对应的 Java 方法
- 如果发现 Java 端存在 bug，在 Rust 端原样复现（加注释标注），不做修复
- 仅在以下情况允许逻辑差异：Java 侧存在 Android 平台特定代码（如 Canvas API），用 RenderCommand 替代

### 3. 最小粒度原则

针对可独立测试的功能/模块，按最小模块粒度进行改写。

- 每次 Rewrite 的范围不超过 ~500 行 Rust 代码（对应 ~20-30 个 Java 文件）
- 每个 Rewrite 周期产出一个可编译、可测试的增量
- 不允许出现"大爆炸"式的一次性大量代码提交

### 4. 测试驱动原则

每个功能模块的改写必须通过测试后才能提交，并进行下一步。

- 单元测试：每个公开结构体/函数至少一个 `#[test]`
- 序列化测试：核心数据结构的 serde JSON 往返测试
- 对比测试：与 Java 端输出逐字段比对（见 Phase 3）

### 5. 结构一致原则

模块名、包名、文件名与原仓库保持一致。

- Java `cn\wps\moffice\writer\layout\base\MainLayouter.java` → Rust `word_core::layout::base::main_layout.rs`
- Java `cn\wps\moffice\writer\layout\base\TextLayouter.java` → Rust `word_core::layout::base::text_layouter.rs`
- Java `cn/wps/moffice/writer/core/KParagraph.java` → Rust
`word_core::core::base:kparagraph.rs`
- 并且类的中方法的名称,参数，方法体的逻辑都保持一致。比如:
TextLayouter 的 flow(LayoutState layoutState) :
  ```
     public int flowToPage(int pageIndex) {
      if (!this.mLocksHelper.isLockDocument()) {
         this.mLocksHelper.lockDocument();
      }

      if (!this.mEvents.isEmpty()) {
         this.flowViewRect((Rect)null);
      }

      if (null == this.mMainState.snapshot) {
         this.mMainState.snapshot = this.mTypoDocument.newSnapshot();
      }

      int cp = this.mMainLayouter.flowToPage(pageIndex);
      if (this.mViewEnv.getViewMode() == 0) {
         int oldViewTop = this.mViewEnv.getViewTop();
         this.mViewEnv.onScroll(oldViewTop);
         this.mLayoutServiceCache.updateTableInfoCache(this.mViewEnv, this.mTypoDocument);
      }

      this.commitSnapshot();
      if (this.mLocksHelper.isLockDocument()) {
         this.mLocksHelper.unlockDocument();
      }

      return cp;
   }
  ```
  需要转换为对应的Rust 方法:
  ```
  pub fn flow_to_page(&mut self, page_index: i32) -> i32 {
    // 1. 加锁文档：如果未加锁则执行加锁（与Java逻辑完全一致）
    if !self.m_locks_helper.is_lock_document() {
        self.m_locks_helper.lock_document();
    }

    // 2. 如果事件列表非空，执行 flowViewRect(null)
    if !self.m_events.is_empty() {
        self.flow_view_rect(None);
    }

    // 3. 如果快照为空，创建新快照
    if self.m_main_state.snapshot.is_none() {
        self.m_main_state.snapshot = Some(self.m_typo_document.new_snapshot());
    }

    // 4. 核心布局逻辑，获取返回值
    let cp = self.m_main_layouter.flow_to_page(page_index);

    // 5. 视图模式为0时执行滚动与缓存更新
    if self.m_view_env.get_view_mode() == 0 {
        let old_view_top = self.m_view_env.get_view_top();
        self.m_view_env.on_scroll(old_view_top);
        self.m_layout_service_cache.update_table_info_cache(&self.m_view_env, &self.m_typo_document);
    }

    // 6. 提交快照
    self.commit_snapshot();

    // 7. 解锁文档：如果已加锁则解锁
    if self.m_locks_helper.is_lock_document() {
        self.m_locks_helper.unlock_document();
    }

    // 8. 返回结果
    cp
  ```

}

---

## 子模块 Rewrite 工作流模板

每个子模块的重写遵循以下 6 步循环：

### Step 1: Analyze（分析）

- 阅读 Java 源码，理解功能和数据流
- 列出所有依赖（已完成 / 未完成）
- 在目标 Rust 目录创建 `REWRITE_DESIGN.md`

### Step 2: Scaffold（脚手架）

- 创建 Rust 文件，定义 struct/enum/trait 签名
- 每个类型添加 doc-comment 溯源注解
- 确保 `cargo check` 通过（方法体用 `todo!()` 占位）

### Step 3: Implement（实现）

- 逐字段、逐方法翻译 Java 逻辑到 Rust
- 保持控制流一致（循环、条件、异常处理）
- 对 Android 平台 API 使用 trait 抽象或 RenderCommand 替代

### Step 4: Self-Test（自测）

- 为每个公开函数编写 `#[test]`
- 使用硬编码测试数据验证基本正确性
- 运行 `cargo test` 确认全部通过

### Step 5: Cross-Compare（交叉对比）

- 通过 UniFFI 桥接在真机上运行对比测试
- 使用 `test-samples/` 中的真实 DOCX 文件
- 逐字段比对 Java 和 Rust 的 JSON 输出

### Step 6: Commit（提交）

- `cargo fmt && cargo clippy && cargo test`
- Git commit，消息格式：`[word-core] rewrite <module>/<sub-module>: <description>`
- 更新 `RUST_REWRITE_PLAN.md` 中对应模块的完成状态

---

## 附录：Rust 替代 Java/Android API 参照表


| Java API                  | Rust 替代                                     | 说明             |
| ------------------------- | ------------------------------------------- | -------------- |
| `ArrayList<T>`            | `Vec<T>`                                    |                |
| `HashMap<K,V>`            | `HashMap<K,V>` / `IndexMap<K,V>`            | 需保序时用 IndexMap |
| `String`                  | `String`                                    |                |
| `int/long/float/double`   | `i32/i64/f32/f64`                           |                |
| `null`                    | `Option<T>`                                 |                |
| `Exception`               | `Result<T, WordCoreError>`                  |                |
| `PropertySet`             | `PropertySet` (HashMap<u16, PropertyValue>) |                |
| `ITextPool` / `ITextRope` | `ropey::Rope`                               | TextRope 包装    |
| `Paint.measureText()`     | `rustybuzz` + `fontdb`                      | 字体度量           |
| `Canvas.drawXxx()`        | `RenderCommand` 枚举                          | 平台无关指令         |
| `ZipPackage` (POI/OPC)    | `zip` crate                                 |                |
| `dom4j` / SAX XML         | `quick-xml`                                 |                |
| `Gson`                    | `serde_json`                                |                |
| `synchronized`            | `Mutex<T>` / `RwLock<T>`                    | 仅在需要时          |


