# 原 label 保存链

2026-10-29，改写中。依赖顺序：原 ColorToolKit 两个静态颜色判断和 ValueHelper.toHexColor(int)，再 FontsLabel、MemoryMixSQLStringList 原匿名迭代器、SstLabel，随后 style/workbook/sheet label 与 XlsxWriter/saveImpl。

逐类手工实现、原字段/方法顺序与 `// java`，保持颜色判断、short/byte 符号、字体索引去重、空字体、UTF-16 富文本边界、异常和输出顺序。每个已验证范围独立记录，不能将整个 Java 类/目录或保存主链标记完成。原 UTF-16 半代理字边界不得用字符序号或 lossy replacement 静默替代。

R0b-33～35 已接回 FontsLabel、原匿名字符串 Iterator、真实文件持久化闭包与 SstLabel 八个方法；384 rich 项、4,224 字体查询和 Memory code units/Buffer UTF-8 真机对照通过。下一个保存依赖为剩余 style/workbook/sheet label、媒体原依赖、XlsxWriter/saveImpl。完整目录和 M4 尚未完成，原方法/对象门面边界见里程碑 08 与各段机器报告。
R0b-37 已对照：XlsxwSharedStrings 两方法与 NumFmtsLabel、PatternTypeMap、FillsLabel。PKM110 原 Java 真机 1,322 fill、65,536 short 映射和 null、12 对象别名/去重历史、16 数字格式历史、3 原 sharedStrings 包通过。R0b-38 继续 BordersLabel/ColorsLabel，然后原 CellXfs/CellStyleXfs/DifferentialFormatting/TableStyles/CellStyles → StyleSheetLabel → XlsxwStyles。

R0b-38 原 BordersLabel/ColorsLabel 与六个 BorderFormatting 选项查询已对照通过：802 边框、65,536 short 映射加 null、64 掩码、15 个去重/别名历史、14 个调色板历史，69,272 字段零差。下一依赖仍为原 CellXfs/CellStyleXfs/DifferentialFormatting/TableStyles/CellStyles → StyleSheetLabel → XlsxwStyles；不将部分样式部件当完整 style 或 XLSX 保存验收。
R0b-39 测试中：原 DifferentialFormatting 四个模型方法及 DifferentialFormattingLabel 的现代 DataFormatting/df/pattern/四参数追加、write/writeDxf/writeNumFmt/writeFont。ColorFilter 与旧 BIFF CFRuleRecord 重载的原类依赖尚未恢复，未添加假类型或空函数，整类门面保持未完成。先以原字体选项、格式回退和别名历史真机差分，再补原依赖并接 StyleSheetLabel。
2026-10-30，R0b-40 改写中：原 CellXfs/CellStyleXfs/CellStyles/TableStyles/StyleSheetLabel 与 XlsxwStyles，先手工恢复其 WriterUtils/XmlStringUtils/StyleRecord 叶子，再按原双阶段收集和 pushWriter 顺序接实际样式包并真机比较。Arc/Weak/逐字段锁只承载原父子实例引用，不进入 ET 决策。
R0b-41 改写中：R40 真机已采集（320 XF、两个完整 short 映射、3 styles ZIP），主机到原 formatNumFmtCode2Xml 时遇到共享 KFC NumFmtAnalyzer 旧 todo。继续原 VariantToken/VariantPieceDic/AnalyzerFactory/各 Analyzer/Holder.compile，不旁路原调用；R40 尚未验收。

2026-10-30 R39～41 已测调用闭环通过：原 StyleSheetLabel/XlsxwStyles 实际包与 KFC compile 调用逐项真机零差。详见 M4-10 与机器报告。整类门面/整本保存仍未验收，继续原 raw/事务和 workbook/worksheet 保存链。
