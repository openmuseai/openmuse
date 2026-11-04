
我已经添加了一条rule， Never use subagents. Do not launch any sub-agents for any task
参考word-core 重写流程对 et-core 进行重写。

为了更加好地追踪重写的进度与功能/逻辑完整性
`RUST_REWRITE_PLAN.md` 是按功能划分代码库。 现在要求对应以下的代码库：

`koffice/et/core/src`
`koffice/et/et-core-io/src`
`koffice/et/et-core-chart/src`
`koffice/et/et-core-formula/src`

以每个包含具体代码最后一级的代码目录为单元，
把以上Java 代码库进一步拆分为最小代码目录单元（参考libs\word-core\RUST_REWRITE_PLAN.md）， 每个代码目录里面在对应目录分别列出重写计划xxx_rewrite_plan.md（参考 libs\word-core\src\core\async\async_rewriter_plan.md）），并且把这些信息重新更新到`RUST_REWRITE_PLAN.md`,形成一个更加完整的数据面板。 然后按照以下流程逐步进行重写

按照代码目录为单元，按照依赖关系，功能优先级，分阶段一一实现这些参照实现这些方法体。功能优先级如下：
1. 基础数据模型。（前置依赖）
2. 核心渲染管线及其依赖。包括io，解析，layout，RenderCommand 
3. HitTest 等基础功能
4. 其他功能。

制定Plan：
   在libs\et-core\RUST_REWRITE_PLAN.md 给每个代码目录分配这些功能/依赖以及功能优先级，然后以代码目录为单元，分阶段实现这些Rust 方法，每个阶段制定一个plan，这个plan 包含这个阶段的所有的代码库（每个代码目录为一个batchs，逐个batch 实现对应代码目录的所有Rust 方法）。
 
实施Plan:
 这个plan 包含这个阶段的所有的代码库(每个代码目录为一个batchs，逐个batch 实现对应代码目录的所有Rust 方法），每个Rust 文件依次进行处理，不要使用脚本，当完成一个文件后再进行下一个文件的改写，每个文件的方法依次改写（不要使用脚本批次处理），每个Rust 重写的原则如下：
   1. libs\et-core-xxx\src  目录下的模块可以相互依赖，模块间的依赖可以传递，无需考虑模块隔离，可以自由引入依赖的代码，因此Box<dyn std::any::Any> 等其他泛型，需要参考对应Java 方法，引入相关的依赖并且替换为真实的类型。
   2.  Java 语言和 Rust 语言存在差异，在一些语法上不能做到一一对应，因此需要进行转换。 libs\tools\verify_rewrite.py 的 _compare_method_logic 对这些情况进行重点检查与校验。
   3. @RUST_REWRITE_RULE.md (98-157) 因为是Rust 方法逐行一一对照，必现严格按照一条Java 语句作为注释，一条Rust 语句对应（一对多，多对一，缺少等都视为错误，并注释的Java 代码行数量需要和原始方法的Java 行数一致）按照逐行对照的原则重新改写 libs\tools\verify_rewrite.py 的 _compare_method_logic 方法，以保证逻辑100% 一致。

重要！！！当一个代码目录下的所有Rust 方法已经实现完之后，使用 libs\tools\verify_rewrite.py 检查当前代码库的目录的Rust 方法是否已经合格，每次完成一个目录的重写后，必须运行验证脚本并确保 **0 个 ❌ 错误** 后才能标记该目录为"完成"。如果测试通过，重新开始这个流程，寻找下一个代码目录（batch) 进行改写, 直到所有的代码目录都改写完成。




Q2:

重写koffice\et\core-xxx 为 Rust 库 libs\ss-core
已经添加了一条rule， Never use subagents. Do not launch any sub-agents for any task

现在参考word-core 重写流程同样对et-core 进行重写。

为了更加好地追踪重写的进度与功能/逻辑完整性
`RUST_REWRITE_PLAN.md` 是按功能划分代码库。 现在要求对应以下的代码库：

`koffice/et/core/src`
`koffice/et/et-core-io/src`
`koffice/et/et-core-chart/src`
`koffice/et/et-core-formula/src`

以每个包含具体代码最后一级的代码目录为单元，
把以上Java 代码库进一步拆分为最小代码目录单元（参考libs\word-core\RUST_REWRITE_PLAN.md）， 每个代码目录里面在对应目录分别列出重写计划xxx_rewrite_plan.md（参考 libs\word-core\src\core\async\async_rewriter_plan.md）），并且把这些信息重新更新到`RUST_REWRITE_PLAN.md`,形成一个更加完整的数据面板。 然后按照以下流程逐步进行重写

按照代码目录为单元，按照依赖关系，功能优先级，分阶段一一实现这些参照实现这些方法体。功能优先级如下：
1. 基础数据模型。（前置依赖）
2. 核心渲染管线及其依赖。包括io，解析，layout，RenderCommand 
3. HitTest 等基础功能
4. 其他功能。

制定Plan：
   在libs\et-core\RUST_REWRITE_PLAN.md 给每个代码目录分配这些功能/依赖以及功能优先级，然后以代码目录为单元，分阶段实现这些Rust 方法，每个阶段制定一个plan，这个plan 包含这个阶段的所有的代码库（每个代码目录为一个batchs，逐个batch 实现对应代码目录的所有Rust 方法）。
 
实施Plan:
 这个plan 包含这个阶段的所有的代码库(每个代码目录为一个batchs，逐个batch 实现对应代码目录的所有Rust 方法），每个Rust 文件依次进行处理，不要使用脚本，当完成一个文件后再进行下一个文件的改写，每个文件的方法依次改写（不要使用脚本批次处理），每个Rust 重写的原则如下：
   1. 每次对代码目录进行改写之前，先使用 libs\tools\verify_rewrite.py 对这个代码目录进行一次校验，以获得一个概览。
   2. libs\et-core-xxx\src  目录下的模块可以相互依赖，模块间的依赖可以传递，无需考虑模块隔离，可以自由引入依赖的代码，因此不能使用Box<dyn std::any::Any> 等泛型，需要参考对应Java 方法，引入相关的依赖并且替换为Java 对应的实际类型。并且在 libs\tools\verify_rewrite.py 中进行校验。
   3.  Java 语言和 Rust 语言存在差异，在一些语法上不能做到一一对应，因此需要进行转换。 libs\tools\verify_rewrite.py 的 _compare_method_logic 对这些情况进行重点检查与校验。
   4. @RUST_REWRITE_RULE.md (98-157) 因为是Rust 方法逐行一一对照，必现严格按照一条Java 语句作为注释，一条Rust 语句对应（一对多，多对一，缺少等都视为错误，并注释的Java 代码行数量需要和原始方法的Java 行数一致）按照逐行对照的原则重新改写 libs\tools\verify_rewrite.py 的 _compare_method_logic 方法，以保证逻辑100% 一致。

重要！！！当一个代码目录下的所有Rust 方法已经实现完之后，使用 libs\tools\verify_rewrite.py 检查当前代码库的目录的Rust 方法是否已经合格，每次完成一个目录的重写后，必须运行验证脚本并确保 **0 个 ❌ 错误** 后才能标记该目录为"完成"。如果测试通过，更新`RUST_REWRITE_PLAN.md` 和该目录下xxx_rewrite_plan.md 的进度和状态。然后重新开始这个流程，寻找plan中下一个代码目录（batch) 进行改写, 直到所有的代码目录都改写完成。











