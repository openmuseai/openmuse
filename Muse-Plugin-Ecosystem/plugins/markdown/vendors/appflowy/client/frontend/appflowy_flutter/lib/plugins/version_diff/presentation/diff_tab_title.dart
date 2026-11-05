import 'package:appflowy/plugins/version_diff/domain/version_diff_contract.dart';
import 'package:intl/intl.dart';

String museVersionTabName(MuseVersion version) {
  if (version.kind == MuseVersionKind.working ||
      version.message == 'Working tree') {
    return '当前';
  }
  final message = version.message?.trim();
  if (message != null &&
      message.isNotEmpty &&
      message != 'Manual snapshot' &&
      message != 'Empty') {
    return message;
  }
  return DateFormat('MMM d, HH:mm').format(version.createdAt.toLocal());
}

String museVersionHash(MuseVersion version) {
  final raw = version.contentDigest.trim().isNotEmpty
      ? version.contentDigest.trim()
      : version.ref.id;
  return raw.length <= 8 ? raw : raw.substring(0, 8);
}

String museDiffViewTabTitle(String fileName, MuseVersion version) =>
    '$fileName-${museVersionHash(version)}';

String museDiffCompareTabTitle(
  String fileName,
  MuseVersion left,
  MuseVersion right,
) =>
    fileName;
