# TextRopeListeners

**Seeds**: wc-02 order 306  
**Java**: `writer/core/TextRopeListeners.java`

## 职责

`INotifyTextRope.Listener` 的多播器：把 insert/remove 通知转发给 chpx/papx/PLC 等。

## 关键逻辑

- `add` / `remove` / `clear` 管理列表  
- `beforeInsertText` / `beforeRemoveText`：**逆序**通知 + dispatcher  
- `afterInsertText` / `afterRemoveText`：**正序**通知 + dispatcher  
- `isDisabled()`：`TextDocument.isInIOState() && !isInUserAction()` 时跳过（加速导入）

## 装配

`SubDocument` 构造：`listeners.add(chpx); add(papx); …` → `notifyRope.setListener(listeners)`。

## 相关

[../SEEDS_ANALYSIS.md](../SEEDS_ANALYSIS.md) · [SubDocument.md](./SubDocument.md)
