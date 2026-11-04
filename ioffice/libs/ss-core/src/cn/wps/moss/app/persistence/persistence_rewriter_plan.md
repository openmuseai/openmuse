# persistence 原管线

2026-10-29，R0b-34 改写中。R0b-33 的字符串迭代真机对照后，向 25,000 条持久化边界推进时发现原门面仍含固定失败值。先接回 `KmoFileHelper → Platform.createFile → RandomAccessFile/FileChannel`，再逐方法实现原 DoubleArrayPersistence/ByteArrayPersistence，包括大端映射、1024 double 分块、累计长度表、多个块、关闭后异常及原 byteBuffer8 池。

文件通道使用真实临时文件和 OS 映射；不把持久化存储替换成内存 Vec。Vec 只对应原临时 ByteBuffer/array。OS mapping 叶仅接受当前 host；wasm 尚未具备对应存储适配，不扩称跨平台完成。

SharedObjects 仅 byteBuffer8 字段闭包；ConcurrentArrayQueue、ObjectPool、PoolableObjectFactory 保留原方法、原子 CAS、try 与循环区别及引用语义。完整 SharedObjects、任意 Java ByteBuffer 门面、并发调度等价尚未验收。整条 ET 保存/完整 raw 不因此置为支持。

R0b-34 的 4 个 byte 块、6 个 double 块、正常/null/关闭通道、94,620 文件字节及 25,009 项原字符串访问已与真机一致。空列表异常的原求值顺序及溢写后的原富文本本地索引行为保留。具体方法/字段与未接受范围见 M4 verify-r0b34.py 和机器报告；本目录不标为整体完成。
