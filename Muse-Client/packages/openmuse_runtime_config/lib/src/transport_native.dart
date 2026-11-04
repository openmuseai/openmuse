import 'package:http/http.dart' as http;
import 'client.dart';

ConfigTransport createConfigTransport() =>
    HttpConfigTransport(client: http.Client());
