# F0.2 开发文档

## 1. 代码工作包

| ID | 实施内容 | 新增/修改路径 | 状态 |
|---|---|---|---|
| HR-W01 | Provider Registry + generation + binding | `resource-host/src/provider-registry.ts` | Done |
| HR-W02 | Authority/Provider/Data Plane ports | `resource-host/src/types.ts` | Done |
| HR-W03 | Materialization TTL/audience/owner/disposer | `resource-host/src/materialization-manager.ts` | Done |
| HR-W04 | Commit/idempotency/digest | `resource-host/src/commit-coordinator.ts` | Done |
| HR-W05 | Event cursor/retention | `resource-host/src/event-log.ts` | Done |
| HR-W06 | Facade/lifecycle/leak snapshot | `resource-host/src/resource-host.ts` | Done |
| HR-W07 | Fake Provider/fault injection | `resource-host/src/testing/` | Done |
| HR-W08 | Packaging/build/link maps | `middlewares/scripts/lib/`、core docs | Done |

## 2. 包结构

```text
middlewares/dsh/core/resource-host/
├─ src/
│  ├─ errors.ts
│  ├─ types.ts
│  ├─ provider-registry.ts
│  ├─ materialization-manager.ts
│  ├─ commit-coordinator.ts
│  ├─ event-log.ts
│  ├─ resource-host.ts
│  └─ testing/fake-provider.ts
├─ tests/resource-host.test.ts
├─ package.json
├─ README.md
└─ TECH.zh-CN.md
```

依赖方向固定：`resource-host → contract-resource → host-bridge`。合同包不反向 import runtime；runtime 不 import Presentation、EngineSession、vendor 或 AppFlowy 类型。

## 3. 核心实现顺序

1. Registry 先建立 provider generation 和显式 locator binding。
2. `describe()` 建立 authority 与 Provider 输出 schema 校验。
3. Materialization 采用“Provider 先创建、Host 验证、成功后转移 ownership”的 exception-safe 流程。
4. resolve/write 在每次使用时重新校验全 audience 与 TTL，不把签发时授权当永久授权。
5. Commit 使用 Host Bridge canonical input digest 建立 scoped idempotency ledger。
6. Provider commit 后再产生 receipt/event；只有 committed 追加 event。
7. shutdown 按 materialization → provider → transient receipt 逆序清理。
8. Build/link/copy 列表加入四个 F0 包，保证干净环境按依赖顺序生成 dist。

## 4. Review 要点

- 任何新增 `MaterializedData` consumer 都必须证明运行在 Host-local 信任边界。
- 不允许把 `resolveMaterialization()` 直接注册成 Agent tool 或公开 HTTP JSON API。
- Provider 必须返回与请求完全相同的 materialization kind；转换应在 Provider 内明确完成。
- 新 writable kind 必须同时修改 Resource schema、安全审查和 commit TCK。
- commit Provider 必须实现存储侧原子 compare-and-store；Host 内存检查不能替代。
- event log 是事实记录，不承载正文或 Ontology 业务对象。

## 5. 构建与测试

```bash
cd Muse-Clients/middlewares/dsh/core/resource-host
pnpm install --offline --frozen-lockfile=false
pnpm check
```

整仓依赖顺序验证：

```bash
Muse-Clients/middlewares/scripts/build-muse-packages.sh --skip-tests
```

`pnpm check` 必须包含 strict typecheck、Vitest 和 declaration build。修改 F0.1 schema 后先重新 build 合同包，再验证 resource-host。

## 6. Migration

本阶段没有用户数据 migration。首次生产接入时：

1. 现有 path/url 先由 Host adapter 建立 resourceRef → locator binding；
2. shadow describe 比较旧链 metadata，不 materialize；
3. internal cohort 使用新 handle；
4. 禁止在迁移数据库中把临时 materialization handle 当资源主键；
5. 关闭 flag 后 locator 和 Resource 真源不删除。

## 7. 后续接口

F0.3 使用 `ResourceHostApi` 构建 Host Bridge Service Definition/Provider；F0.4 使用 Host-local `resolve/write` 连接 fake Adapter 和 Flutter Surface。生产 Provider、持久 event store 和 orphan reconciliation 分别由 Host/Storage、F0.6 owner 负责。

## 8. Owner 与阻塞

| 责任 | Owner | 状态 |
|---|---|---|
| Core runtime | Protocol/Host | 完成 |
| Native Provider adapter | Host/Storage | F0.3/F0.4 前置设计待建 |
| DSH capability seam | DSH | 下一阶段 F0.3 |
| Surface/data-plane broker | Client Platform | F0.4/F0.5 |
| Soak/orphan reconciliation | QA + Host | F0.6 |

Ontology Runtime 不在依赖或阻塞列表中。
