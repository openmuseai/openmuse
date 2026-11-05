import 'package:flutter/material.dart';

const wordWebUnsupportedMessage =
    'Please edit Word documents in the desktop client.';
const wordMissingBlobMessage =
    'Document data is missing. Delete this page and recreate it.';

class WordStatusPage extends StatelessWidget {
  const WordStatusPage({super.key, required this.message});

  final String message;

  @override
  Widget build(BuildContext context) {
    return ColoredBox(
      color: Theme.of(context).colorScheme.surface,
      child: Center(
        child: Padding(
          padding: const EdgeInsets.all(24),
          child: Text(message, textAlign: TextAlign.center),
        ),
      ),
    );
  }
}
