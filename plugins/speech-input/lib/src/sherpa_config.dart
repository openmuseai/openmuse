import 'dart:io';

final class SherpaZipformerConfig {
  const SherpaZipformerConfig({
    required this.modelId,
    required this.encoder,
    required this.decoder,
    required this.joiner,
    required this.tokens,
    this.numThreads = 2,
    this.decodingMethod = 'greedy_search',
  });

  factory SherpaZipformerConfig.zh14m(String directory) =>
      SherpaZipformerConfig(
        modelId: 'sherpa-onnx-streaming-zipformer-zh-14M-2023-02-23',
        encoder: '$directory/encoder-epoch-99-avg-1.int8.onnx',
        decoder: '$directory/decoder-epoch-99-avg-1.onnx',
        joiner: '$directory/joiner-epoch-99-avg-1.int8.onnx',
        tokens: '$directory/tokens.txt',
      );

  final String modelId;
  final String encoder;
  final String decoder;
  final String joiner;
  final String tokens;
  final int numThreads;
  final String decodingMethod;

  Iterable<String> get files => [encoder, decoder, joiner, tokens];

  bool get isInstalled => files.every((path) => File(path).existsSync());

  List<String> get missingFiles => [
    for (final path in files)
      if (!File(path).existsSync()) path,
  ];

  Map<String, Object?> toMessage() => {
    'modelId': modelId,
    'encoder': encoder,
    'decoder': decoder,
    'joiner': joiner,
    'tokens': tokens,
    'numThreads': numThreads,
    'decodingMethod': decodingMethod,
  };
}
