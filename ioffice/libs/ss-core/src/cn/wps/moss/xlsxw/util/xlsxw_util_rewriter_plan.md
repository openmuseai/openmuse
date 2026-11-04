# xlsxw/util 原类续行

2026-10-29，M4 持续实施。Java 源为 `koffice/et/core/cn/wps/moss/xlsxw/util`。

`XlsxwPackage` 已逐方法手工接入 createBookPart、四个 getOutputStream 重载、九个 part 工厂与 getRelation(int)，保留方法顺序、原调用、普通/剪贴板关系策略及异常 cause。对应语句保留 `// java`。R0b-29 真机八组包/264 个 ZIP 条目对照通过，见里程碑 `fixtures/verify-r0b29.py` 与方法索引。

整个目录和整个类尚未完成。`getRelation(PictureData)` 与 `isGif` 必须先接回原 PictureData/EscherBlipRecord/FileDataStorage 和 slim-tool 依赖，不能将文件存储改成 Vec 或省略构造器副作用。后续继续原 ValueHelper、writer/label、XlsxWriter、saveImpl，不在本工具类另写简化序列化器。
