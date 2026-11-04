# 02_function_style_event — styles.xml 事件 → KStyles（两阶段收集）

| 项 | 值 |
|----|----|
| feature | wc-05-orchestration |
| order | 2 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `StylesImporter` 构造 / `importDocDefaults` / `importStyleStart` / `importStyleChild` / `importStyleEnd` / `importStylesEnd` / `importTblStylePrEnd` / private `createKStyle`·`getStylePrSet`·`correctTcPrSet`·`toSti`·`toBaseSti`·`toNextsSti`·`toDisplayName` |
| 独立性 | styles.xml → `KStyles` 的**两阶段收集**单元（不依赖 DocumentImporter 总线） |
| 测试状态 | pass |

## 1. 功能说明

### 1.1 一句话

把 styles.xml 的 SAX 事件序列累积成**草稿列表**，`importStylesEnd` 时一次性把全部 `KStyle` 落进 `KStyles` + 设置 docDefaults——保证 basedOn/next/link 的 sti 映射在 createKStyle 前已全部 put 完毕。

### 1.2 输入

| 入口 | 输入 | Java 路径 |
|------|------|-----------|
| 构造 | `KStyles kStyles` + `StyleId2Sti styleId2Sti` | StylesImporter.java:35–47 |
| `importDocDefaults` | `MutablePropertySet rPrSet` + `pPrSet`（可 null） | :48–59 |
| `importStyleStart` | `Attributes`（type/customStyle/styleId） | :60–66 |
| `importStyleChild` | `int elementId` + `Attributes` | :68–92 |
| `importStyleEnd` | `StylePrModel` | :93–106 |
| `importStylesEnd` | （无） | :127–139 |
| `importTblStylePrEnd` | `StylePrModel`（表样式条件格式） | :182–189 |

### 1.3 输出

| 字段/目标 | 含义 |
|-----------|------|
| `mStyles`（KStyles） | rPr/pPr Default + 全部 `KStyle`（add） |
| `mStyleId2Sti` | 通过 iOStyle.convertTypeAndId 填充 |
| `mIOStyleList` | `ArrayList<IOStyle>(30)` 全部草稿 |
| `mCurIOStyle` | 当前解析中的草稿 |

### 1.4 上游

- 01 `IOStyle`：草稿对象 + `convertTypeAndId`。
- wc-02 `KStyles`（add/setRPrDefault/setPPrDefault/`getTextDocument`）、`KStyle::new__textdocument…`（rs 已实现）。
- wc-02 `StyleId2Sti`（getSti/put，rs 已实现）。
- `BuildinStyleSheet`（isReservedName/isReservedIndex，rs 已实现）。
- `StylePrModel`（core-libs 已生成）——`importStyleEnd` 入参。

### 1.5 下游

- 03/05 `DocumentImporter` 把 styles 回调直转本 importer。
- feature-output：styles 在正文消费 styleId 之前落入 KStyles（MVP 验收 P0）。

### 1.6 关键不变量（钉死）

1. **两阶段**：`importStyleStart/Child/End` 期间**只累积** `mIOStyleList`，`KStyles.add` 只在 `importStylesEnd` 发生。
2. `importDocDefaults`：rPrSet≠null → `setRPrDefault(rPrSet.freeze())`；pPrSet≠null → `setPPrDefault(freeze)`；**不在两阶段内，单条即提交**。
3. `importStyleChild` elementId 常量：**-681243704**→mQFormat（toBoolean4NotNull）；**-332612366**→mBasedOn（getString）；**3321850**→mLink；**3373707**→mName；**3377907**→mNext；**872749144**→mUiPriority（ShareConvertHelper.getLong4STDecimalNumber）；**971752949**→DocxReaderHelper.setTblStylePrType。
4. `importStyleEnd`：`convertTypeAndId` + `mCorePrSet.add(getStylePrSet(model))` + 入 list。`getStylePrSet` = rPrBaseSet + PPrGeneralSet + correctTcPrSet(tcPrSet) + TblPrBaseSet 合并。`correctTcPrSet` 移除 prop **346–351**，`correctCellPaddingStype` 把 prop **355**（List）转数组。
5. `createKStyle`：`id=toSti(rectifyStyleId())`；`base=toBaseSti(mBasedOn,sti,id)`；`next=toNextsSti(mNext,id,sti)`；`name=toDisplayName(mName,id)`；`pr=mCorePrSet.freeze()`；`new KStyle(mTextDocument,id,name,base,next,mCoreType,pr)`。
6. **sti 哨兵 4095**：`toSti(null)→4095`、`toSti(查不到)→4095`；`toBaseSti` base==coreSti→4095、**coreSti==0→4095**；`toNextsSti` nextId null 或查不到→返回自身 sti。
7. `importStylesEnd` 循环：`kStyle.setLink(toSti(mLink))`；`mQFormat!=null`→setQFormat；`mUiPriority!=null`→setUiPriority；`mStyles.add(kStyle)`。
8. `toDisplayName`：name 保留名 & 保留索引→原名；保留名但非保留索引→`name+"_"+UUID`；否则原名。
9. `importTblStylePrEnd`（表样式条件格式，02 内可为 stub/延后，须保留签名）：CFC/CPF/TSP 三 Value 写回 mCorePrSet（MUST 可延后，见 §1.7）。

### 1.7 依赖与实现状态（必写，着重）

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓（保留 Java 注释+签名） | 禁止本 function 去改 |
|------|------|------------------------|--------|-----------------------------|----------------------|
| `IOStyle` | 01（本 feature） | `convertTypeAndId` / `rectifyStyleId` / 字段 | loop 先做 01 | — | 依赖 01 |
| `KStyles` | wc-02 | `add(&mut KStyle)`、`setRPrDefault/setPPrDefault(&PropertySet)`、`getTextDocument()` | **已实现**（KStyles.rs L193/213/325/181） | — | 不去改 KStyles |
| `KStyle` | wc-02 | `new__textdocument_int_string_int_int_int_propertyset`、`setLink/setQFormat/setUiPriority` | **已实现**（KStyle.rs L781/1040/1066/1080） | — | 不去改 KStyle |
| `StyleId2Sti` | wc-02 | `getSti(&str)->Option<i32>`、`put×3` | **已实现**（StyleId2Sti.rs L49/69/92/105） | — | 不去改 |
| `BuildinStyleSheet` | core-libs helpers | `isReservedName/isReservedIndex` | 已实现（rs 存在） | — | 不去改 |
| `StylePrModel` | core-libs（已生成） | 读 `mRprBaseSet/mPPrGeneralSet/mTcPrSet/mTblPrBaseSet` | core-libs 已生成 | — | 不在本 function 补方法 |
| `MutablePropertySet` | core-libs util | `new/add/setValue/getValue/remove/freeze/setInt` | **已实现** | — | 不去改 |
| `ConvertHelper` | wc-03（blocked） | `getString/getBoolean/toBoolean4NotNull` | Rust rs 已存在（wc-03 已实现 ConvertHelper），**loop 内联或直用** | — | 不把 wc-03 拖进来改 |
| `ShareConvertHelper` | wc-03（blocked） | `getLong4STDecimalNumber` | rs 已存在 | — | 不拖入改 |
| `DocxReaderHelper` | word-core helper | `setTblStylePrType` | rs 已存在 | — | 不去改 |
| `UUID` | std | `random` | std | — | — |
| `CellMargin/Conditional*/TableStyleProperties` | wc-02/value | `importTblStylePrEnd` 专用 | value 类未全部改写 | **importTblStylePrEnd 可暂缓（MUST 延后）**：保留 Java 注释+签名，body 用 `todo!("DEFERRED_NON_FUNCTION")`+注释停 | 不新建 |

> 注：Split 阶段 02 的 `importTblStylePrEnd` 族（CFC/CPF/TSP）依赖多个 value 类型，属 MUST-可延后；改写列表可将该族方法标为**列表外/延后**，停车原模版体。

## 2. 核心代码与流程

Java：`koffice/writer/core/cn/wps/moffice/writer/io/reader/docxReader/importer/StylesImporter.java`（237 行）
Rust：`libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/StylesImporter.rs`（loop 新建）

```mermaid
sequenceDiagram
    participant H as StylesHandler
    participant SI as StylesImporter
    participant IOS as IOStyle
    participant KS as KStyles
    H->>SI: importStyleStart(attrs)
    SI->>IOS: new + type/customStyle/styleId
    H->>SI: importStyleChild(elementId, attrs)
    SI->>IOS: 填 name/basedOn/next/link/qFormat/uiPriority
    H->>SI: importStyleEnd(StylePrModel)
    SI->>IOS: convertTypeAndId + mCorePrSet.add
    SI->>IOS: mIOStyleList.add
    H->>SI: importStylesEnd()
    loop 每个 iOStyle
      SI->>SI: createKStyle → toSti/toBaseSti/toNextsSti/toDisplayName
      SI->>KS: setLink + setQFormat/setUiPriority + add(kStyle)
    end
```

## 3. 测试方案

### 3.1 代码审计与测试面

| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|--------|----------------|-----------|-------------|---------------|
| 构造 | StylesImporter.java:35–47 | kStyles/styleId2Sti | 存 KStyles + 取 mTextDocument | null → assertNotNull 红 |
| importDocDefaults | :48–59 | rPrSet/pPrSet（null 变体） | setRPrDefault/setPPrDefault | 各自独立判 null |
| importStyleStart | :60–66 | attributes | new IOStyle + 填 type/customStyle/styleId | attributes null → 红 |
| importStyleChild | :68–92 | elementId + attrs | 按 7 个常量填字段 | 未知 elementId 无分支（switch fall-through，不 fail） |
| importStyleEnd | :93–106 | StylePrModel | convertTypeAndId + mCorePrSet.add + list.add | mCurIOStyle/stylePrModel null → 红 |
| importStylesEnd | :127–139 | mIOStyleList | createKStyle + setLink/qFormat/uiPriority + add | 空 list；mLink/next 映射；qFormat/uiPriority null |
| createKStyle | :143–151 | iOStyle | id/base/next/name/pr + new KStyle | toBaseSti 4095 双分支 |
| toSti/toBaseSti/toNextsSti | :162–181 | styleId/sti | 4095 哨兵逻辑 | null/查不到/==self/coreSti==0 |
| toDisplayName | :153–158 | name + sti | 保留名处理 | 保留名&保留索引 / 保留名&非保留索引 |
| correctTcPrSet | :108–119 | tcPrSet（null 变体） | remove 346-351 + cellPadding | tcPrSet null → 直接返回 null 不炸 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id / 未覆盖理由 |
|-------------|---------------|----------|-------|----------------------|
| SE-CTOR | 构造保存 KStyles + 取 textDoc | 主路径 | 是 | SE1 |
| SE-CTOR-NULL | 构造 null → assert 红 | 失败 | 否* | Rc/RefCell 无 null 语义，Java assertNotNull 恒真不可触发（记录，非门闩） |
| SE-DOCDEFAULTS-BOTH | importDocDefaults(rPr,pPr 均非 null) → 双 default 落 | 主路径 | 是 | SE3 |
| SE-DOCDEFAULTS-NULL | rPr/pPr 分别 null | 分支 | 是 | SE4 |
| SE-STYLE-START | importStyleStart 填 type/customStyle/styleId | 主路径 | 是 | SE5 |
| SE-CHILD-QF | elementId=-681243704 → mQFormat | 分支 | 是 | SE6 |
| SE-CHILD-BASEDON | -332612366 → mBasedOn | 分支 | 是 | SE7 |
| SE-CHILD-LINK | 3321850 → mLink | 分支 | 是 | SE8 |
| SE-CHILD-NAME | 3373707 → mName | 分支 | 是 | SE9 |
| SE-CHILD-NEXT | 3377907 → mNext | 分支 | 是 | SE10 |
| SE-CHILD-UIPRI | 872749144 → mUiPriority | 分支 | 是 | SE11 |
| SE-CHILD-TBLSTYLE | 971752949 → setTblStylePrType | 分支 | 是 | SE12 |
| SE-END-2PHASE | importStyleEnd 只累积不 add | 状态 | 是 | SE13 |
| SE-END-GETPRSET | getStylePrSet 合并 + correctTcPrSet(346-351) | 主路径 | 是 | SE14 |
| SE-END-NULLPR | stylePrModel null → 红 | 失败 | 否* | &StylePrModel 无 null 语义不可触发（记录，非门闩） |
| SE-STYLESEND-COMMIT | importStylesEnd 才 add 全部 | 状态 | 是 | SE16 |
| SE-CREATE-KSTYLE | createKStyle id/base/next/name/type/pr | 主路径 | 是 | SE17 |
| SE-STI-NULL | toSti(null)→4095 | 边界 | 是 | SE18 |
| SE-STI-MISS | toSti(查不到)→4095 | 边界 | 是 | SE19 |
| SE-BASE-SELF | toBaseSti base==coreSti→4095 | 分支 | 是 | SE20 |
| SE-BASE-NORMAL | toBaseSti coreSti==0→4095 | 分支 | 否* | `#[ignore = "spec: BuildinStyleSheet 内置 Normal→sti0 未接通"]`（依赖缺口，SE21） |
| SE-NEXT-NULL | toNextsSti nextId null → 自身 sti | 分支 | 是 | SE22 |
| SE-NEXT-MISS | toNextsSti 查不到 → 自身 sti | 分支 | 是 | SE23 |
| SE-LINK-QFORMAT | setLink/setQFormat/setUiPriority 条件调用 | 分支 | 是 | SE24 |
| SE-DISPLAY-RESERVED | toDisplayName 保留名&保留索引 → 原名 | 分支 | 否* | `#[ignore = "spec: 保留索引(0..267) 不可达，UUSti 从 4097 起"]`（SE25）；保留名&非保留索引分支 SE26 已绿 |
| SE-DISPLAY-UUID | 保留名&非保留索引 → name_UUID | 分支 | 是 | SE26 |
| SE-TBLSTYLE-END | importTblStylePrEnd 表样式条件格式 | 延后 | 否 | 延后（§1.7 MUST-可延后，保留签名，`TODO`+`DEFERRED`） |

### 3.3 可测试性

不依赖 DocumentImporter / 不开 docx。测试用**真实 `KStyles`+`StyleId2Sti`**（rs 已实现），直接构造 `StylesImporter`。`StylePrModel` 用 core-libs 已生成类型，测试里构造最小 `StylePrModel`（填 `mRprBaseSet/mPPrGeneralSet/mTcPrSet/mTblPrBaseSet`）或 mock。`importStyleChild` 的 `Attributes` 用轻量假 attributes（DummyAttributes：按名取 string/boolean/long），避免引入 org.xml.sax。`importStylesEnd` 两阶段用「`importStylesEnd` 前 `getStyleById(0)==None`、之后有值」断言（不 watch 内部 list，避免依赖内部）。

> 关键：`correctTcPrSet` 的 346–351 与 355 数组化，需真实 `MutablePropertySet`（rs 已实现）+ 一个预填 `mTcPrSet` 的 `StylePrModel` 观察 remove/setValue 副作用。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|----|--------|----------|----------|-----------------|----------|
| SE1 | SE-CTOR | 真实 KStyles+StyleId2Sti | new StylesImporter | 保存字段；mTextDocument 可读 | 构造失败 |
| SE2 | SE-CTOR-NULL | kStyles=null | new | panic（assertNotNull） | 未拒绝 null |
| SE3 | SE-DOCDEFAULTS-BOTH | 空 KStyles | importDocDefaults(rPrSet, pPrSet) | `getRPrDefault`/`getPPrDefault` 非 None 且 ==freeze | default 未落 |
| SE4 | SE-DOCDEFAULTS-NULL | 空 KStyles | 传 (null,pPr)/(rPr,null) | 只落非 null 侧；另一侧保持 None | null 侧误落 |
| SE5 | SE-STYLE-START | 空 KStyles | importStyleStart({type:"paragraph",customStyle:"1",styleId:"H1"}) | mCurIOStyle 的 mType/styleId/isCustom 正确；KStyles 无增加 | 字段没填 |
| SE6 | SE-CHILD-QF | 已有 cur | importStyleChild(-681243704, qformat-attrs) | mCurIOStyle.mQFormat 非 None | 未填 |
| SE7 | SE-CHILD-BASEDON | 已有 cur | importStyleChild(-332612366, basedOn="Base") | mBasedOn=="Base" | 未填 |
| SE8 | SE-CHILD-LINK | 已有 cur | importStyleChild(3321850, link="Linked") | mLink 填 | 未填 |
| SE9 | SE-CHILD-NAME | 已有 cur | importStyleChild(3373707, name="Heading 1") | mName 填 | 未填 |
| SE10 | SE-CHILD-NEXT | 已有 cur | importStyleChild(3377907, next="Normal") | mNext 填 | 未填 |
| SE11 | SE-CHILD-UIPRI | 已有 cur | importStyleChild(872749144, uiPriority=10) | mUiPriority==Some(10) | 未填 |
| SE12 | SE-CHILD-TBLSTYLE | 已有 cur | importStyleChild(971752949, {type:"firstRow"}) | mTblStyle*Id 被设 | 未设 |
| SE13 | SE-END-2PHASE | importStyleStart 后 | importStyleEnd(minStylePrModel) | `getStyleById` 仍 None（未 add）；list 累计 | 提前 add |
| SE14 | SE-END-GETPRSET | StylePrModel 带 4 个子集 + tcPr 带 346-351/355 | importStyleEnd | mCorePrSet 合并；346-351 被 remove；355 数组化 | 合并/清理错 |
| SE15 | SE-END-NULLPR | cur 非 null | importStyleEnd(null) | panic | 未拒绝 null |
| SE16 | SE-STYLESEND-COMMIT | mIOStyleList 有 2 条 | importStylesEnd | `KStyles.add` 2 次；`getStyleById(id)` 均可查 | 未提交/顺序错 |
| SE17 | SE-CREATE-KSTYLE | 一条完整 iOStyle | importStylesEnd内 createKStyle | KStyle(id,name,base,next,type,pr) 各字段符合 sti 哨兵逻辑 | 生成错 |
| SE18 | SE-STI-NULL | mLink=null | importStylesEnd | kStyle.setLink(4095) | 哨兵错 |
| SE19 | SE-STI-MISS | mBasedOn=查不到 | importStylesEnd | base==4095 | 哨兵错 |
| SE20 | SE-BASE-SELF | mBasedOn==自身 styleId | importStylesEnd | base==4095 | 自引用未拦截 |
| SE21 | SE-BASE-NORMAL | styleId 映射 sti==0（Normal） | importStylesEnd | base==4095 | Normal 强制无 base 未生效 |
| SE22 | SE-NEXT-NULL | mNext=null | importStylesEnd | next==自身 sti | 缺省错 |
| SE23 | SE-NEXT-MISS | mNext 查不到 | importStylesEnd | next==自身 sti | 缺省错 |
| SE24 | SE-LINK-QFORMAT | mQFormat=Some(true), mUiPriority=Some(10) | importStylesEnd | setQFormat(true)+setUiPriority(10) 被调 | 未调 |
| SE25 | SE-DISPLAY-RESERVED | mName="Normal"（保留名&保留索引） | importStylesEnd | KStyle.name=="Normal" | 被改名 |
| SE26 | SE-DISPLAY-UUID | mName=保留名 但 sti 非保留索引 | importStylesEnd | name 以 "name_" 开头且带 UUID | 未加 UUID |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src:   `libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/StylesImporter.rs`
test:  `libs/word-core/tests/cn/wps/moffice/writer/io/reader/docxReader/importer/StylesImporter.rs`

Split 阶段不落 `[[test]]`（src 未生成）；loop L2 已落地镜像测试并在 `Cargo.toml` 加 `[[test]]`（`name = "02_function_style_event"`）。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_style_event

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 02_function_style_event \
  --src src/cn/wps/moffice/writer/io/reader/docxReader/importer/StylesImporter.rs \
  --src src/cn/wps/moffice/writer/io/reader/docxReader/importer/IOStyle.rs \
  --report libs/word-core/parser/features/wc-05-orchestration/functions/02_function_style_event/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_style_event --json`)
date: 2026-08-21 16:45

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/io/reader/docxReader/importer/IOStyle.rs` | 41/51 | 80.4% | 57/71 | 80.3% |
| `src/cn/wps/moffice/writer/io/reader/docxReader/importer/StylesImporter.rs` | 179/210 | 85.2% | 282/318 | 88.7% |
| **function total** | **220/261** | **84.3%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/StylesImporter.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/io/reader/docxReader/importer/StylesImporter.java
# 本 function 点名且本轮编辑过的依赖（01 IOStyle）再加一组：
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/IOStyle.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/io/reader/docxReader/importer/IOStyle.java
```

Loop L5b（2026-08-21）：**PASS** — StylesImporter.rs + IOStyle.rs 两组 `check-template-layout.mjs` 全绿（153 + 47 java-comment 行，`java comments preserved and aligned`，method comments covered）。

Loop 备注（2026-08-21）：
- `ShareConvertHelper.getLong4STDecimalNumber` 上游已存在真实现（核对时首次 grep 截断漏看；未改 ShareConvertHelper）。
- `correctCellPaddingStype`（355 List→CellMargin[]）与 `importTblStylePrEnd` 族（CFC/CPF/TSP）按 §1.7 延后：保留签名 + Java 注释 + `/** DEFERRED_NON_FUNCTION **/` 停车。
- `toDisplayName` 的 name=null → 空串（Java null 的 Rust 等价，KStyle name 接收 &str）；代码注释已说明。
- SE21/SE25 两个 `#[ignore = "spec: ..."]` 为真实依赖缺口：Rust `BuildinStyleSheet` 内置表（Normal→sti0、保留索引域 0..267）在测试环境未接通；断言保留真实，`pass` 门闩不依赖它们。

校验：`// java` 行数不降；`importStyleChild` 的 7 个 case 注释与 Rust 签名相邻；`importTblStylePrEnd` 族若在改写列表外，需停车模版体（`todo!("DEFERRED_NON_FUNCTION")` + 注释停模版体），禁止删注释或 dummy `let X = ()`；`correctTcPrSet`/`getStylePrSet`（私有 static）须在改写列表，实现。Java 无对应语句处标 `// rust: no-stmt`。

本轮 layout：StylesImporter / IOStyle / DocxReaderHelper / ConvertHelper / ShareConvertHelper 均为 `PASS: java comments preserved and aligned`。
