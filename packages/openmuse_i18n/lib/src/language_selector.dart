import 'package:flutter/material.dart';
import 'controller.dart';

/// The available languages and their self-names come from the registry.
final class OpenMuseLanguageSelector extends StatelessWidget {
  const OpenMuseLanguageSelector({super.key});
  @override
  Widget build(BuildContext context) {
    final locale = OpenMuseLocaleScope.maybeOf(context);
    if (locale == null) return const SizedBox.shrink();
    return ListTile(
      contentPadding: EdgeInsets.zero,
      title: Text(locale.strings.commonLanguage()),
      trailing: DropdownButton<String>(
        key: const ValueKey('runtime-language-selector'),
        value: locale.followsSystem ? 'system' : locale.tag,
        items: [
          DropdownMenuItem(
            value: 'system',
            child: Text(locale.strings.commonFollowSystem()),
          ),
          for (final option in locale.options)
            DropdownMenuItem(value: option.tag, child: Text(option.label)),
        ],
        onChanged: (value) {
          if (value == 'system') {
            locale.followSystem();
          } else if (value != null) {
            locale.selectTag(value);
          }
        },
      ),
    );
  }
}
