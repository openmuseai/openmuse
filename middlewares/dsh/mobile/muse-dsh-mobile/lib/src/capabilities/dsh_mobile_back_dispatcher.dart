import 'package:flutter/widgets.dart';
import 'package:muse_dsh_mobile/src/capabilities/dsh_native_capability_broker.dart';

/// Asks the DSH page to consume back first; pops the Flutter route if unused.
class DshMobileBackDispatcher {
  const DshMobileBackDispatcher(this.broker);

  final DshNativeCapabilityBroker? broker;

  Future<void> handle(BuildContext context) async {
    final consumed = await broker?.requestBack() ?? false;
    if (!consumed && context.mounted) {
      Navigator.of(context).maybePop();
    }
  }
}
