import 'package:web_socket_channel/web_socket_channel.dart';

WebSocketChannel createAppFlowySocket(Uri uri) => WebSocketChannel.connect(uri);
