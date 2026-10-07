import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';

/// Mobile renderer for the generic plugin-interaction envelope. Plugin IDs are
/// data: this client has no Easel or social-platform implementation.
final class RemotePluginInteraction {
  const RemotePluginInteraction({
    required this.id,
    required this.pluginId,
    required this.title,
    required this.state,
    required this.message,
    required this.mediaHandle,
  });

  final String id;
  final String pluginId;
  final String title;
  final String state;
  final String message;
  final String mediaHandle;

  static RemotePluginInteraction? parse(Object? value) {
    if (value == null) return null;
    if (value is! Map ||
        value['protocol'] != 'openmuse.plugin-interaction/v1' ||
        value['type'] != 'image.challenge' ||
        value['id'] is! String ||
        value['pluginId'] is! String ||
        value['title'] is! String ||
        value['state'] is! String ||
        value['message'] is! String ||
        value['mediaHandle'] is! String) {
      throw const FormatException('Invalid plugin interaction envelope');
    }
    return RemotePluginInteraction(
      id: value['id'] as String,
      pluginId: value['pluginId'] as String,
      title: value['title'] as String,
      state: value['state'] as String,
      message: value['message'] as String,
      mediaHandle: value['mediaHandle'] as String,
    );
  }
}

final class PairedPluginInteractionClient {
  PairedPluginInteractionClient(this.connection);

  final PairedDesktopConnection connection;
  final HttpClient _http = HttpClient();

  Uri get _origin => Uri.parse(connection.session.origin);

  Future<RemotePluginInteraction?> current() async {
    final request = await _http.getUrl(
      _origin.replace(path: '/openmuse/plugin-interaction/v1', query: ''),
    );
    request.cookies.add(Cookie('OpenMuse-Paired', connection.grantRef));
    final response = await request.close();
    final body = await utf8.decodeStream(response);
    if (response.statusCode != HttpStatus.ok) {
      throw HttpException('Interaction HTTP ${response.statusCode}');
    }
    final decoded = jsonDecode(body);
    if (decoded is! Map) {
      throw const FormatException('Invalid interaction reply');
    }
    return RemotePluginInteraction.parse(decoded['interaction']);
  }

  Future<Uint8List> image(String handle) async {
    final request = await _http.getUrl(
      _origin.replace(
        path: '/openmuse/plugin-interaction/media/v1',
        queryParameters: {'handle': handle},
      ),
    );
    request.cookies.add(Cookie('OpenMuse-Paired', connection.grantRef));
    final response = await request.close();
    final bytes = await response.fold<List<int>>(
      <int>[],
      (current, chunk) => current..addAll(chunk),
    );
    if (response.statusCode != HttpStatus.ok ||
        bytes.length > 2 * 1024 * 1024) {
      throw HttpException('Interaction media HTTP ${response.statusCode}');
    }
    return Uint8List.fromList(bytes);
  }

  void close() => _http.close(force: true);
}

class PairedPluginInteractionLayer extends StatefulWidget {
  const PairedPluginInteractionLayer({super.key, required this.connection});

  final PairedDesktopConnection connection;

  @override
  State<PairedPluginInteractionLayer> createState() =>
      _PairedPluginInteractionLayerState();
}

class _PairedPluginInteractionLayerState
    extends State<PairedPluginInteractionLayer> {
  late PairedPluginInteractionClient _client;
  Timer? _timer;
  RemotePluginInteraction? _interaction;
  Uint8List? _image;
  String? _dismissedId;
  bool _busy = false;

  @override
  void initState() {
    super.initState();
    _client = PairedPluginInteractionClient(widget.connection);
    _timer = Timer.periodic(const Duration(seconds: 1), (_) => _poll());
    unawaited(_poll());
  }

  @override
  void didUpdateWidget(covariant PairedPluginInteractionLayer oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.connection.grantRef == widget.connection.grantRef) return;
    _client.close();
    _client = PairedPluginInteractionClient(widget.connection);
    _interaction = null;
    _image = null;
    _dismissedId = null;
    unawaited(_poll());
  }

  Future<void> _poll() async {
    if (_busy) return;
    _busy = true;
    try {
      final next = await _client.current();
      if (!mounted) return;
      if (next == null) {
        if (_interaction != null) setState(() => _interaction = null);
        return;
      }
      if (next.id != _interaction?.id) {
        final image = await _client.image(next.mediaHandle);
        if (!mounted) return;
        setState(() {
          _interaction = next;
          _image = image;
          _dismissedId = null;
        });
      } else if (next.state != _interaction?.state ||
          next.message != _interaction?.message) {
        setState(() => _interaction = next);
      }
    } catch (_) {
      // Keep an already displayed challenge during transient relay loss.
    } finally {
      _busy = false;
    }
  }

  @override
  void dispose() {
    _timer?.cancel();
    _client.close();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final interaction = _interaction;
    if (interaction == null ||
        interaction.id == _dismissedId ||
        interaction.state == 'success') {
      return const SizedBox.shrink();
    }
    return Positioned.fill(
      child: ColoredBox(
        color: Colors.black54,
        child: SafeArea(
          child: Center(
            child: Card(
              margin: const EdgeInsets.all(20),
              child: Padding(
                padding: const EdgeInsets.all(20),
                child: Column(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    Text(
                      interaction.title,
                      style: Theme.of(context).textTheme.titleLarge,
                    ),
                    const SizedBox(height: 16),
                    if (_image case final image?)
                      Image.memory(
                        image,
                        key: const ValueKey('paired-plugin-interaction-image'),
                        width: 260,
                        height: 260,
                        fit: BoxFit.contain,
                      ),
                    const SizedBox(height: 12),
                    Text(
                      interaction.message,
                      key: const ValueKey('paired-plugin-interaction-status'),
                    ),
                    if (interaction.state == 'qr_ready')
                      const Padding(
                        padding: EdgeInsets.only(top: 12),
                        child: Text(
                          '可将二维码截图，在目标 App 的扫一扫中从相册选择。',
                          textAlign: TextAlign.center,
                        ),
                      ),
                    const SizedBox(height: 8),
                    TextButton(
                      onPressed: () =>
                          setState(() => _dismissedId = interaction.id),
                      child: const Text('暂时关闭'),
                    ),
                  ],
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
