import 'dart:async';
import 'package:openmuse_dataset/openmuse_dataset.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'resource_checkpoint.dart';

typedef DatasetResourceLease =
    ResourceSessionLease<
      DatasetController,
      DatasetResourceCheckpoint,
      DatasetRealtimeSession?
    >;

/// Content composition only; the SDK retains the existing owner/lease logic.
final class DatasetSessionOwners
    extends
        ResourceSessionOwners<
          DatasetController,
          DatasetResourceCheckpoint,
          DatasetRealtimeSession?
        > {
  DatasetSessionOwners({
    required super.open,
    required super.release,
    required super.objectId,
    required super.realtime,
    super.prepareAuthority,
  }) : super(
         supportsPrivateRecovery: true,
         createCheckpoint: DatasetResourceCheckpoint.new,
         controllerObjectId: (c) => c.objectId,
         controllerEditable: (c) => c.editable,
         prepareClose: (c) async {
           final ticket = await c.prepareClose();
           return ResourceOwnerPreparation(
             active: () => ticket.isActive,
             cancel: ticket.cancel,
           );
         },
         authorityEnded: (c, realtime) {
           c.invalidateAuthority();
           if (realtime != null)
             unawaited(realtime.close().catchError((Object _) {}));
         },
       );
}
