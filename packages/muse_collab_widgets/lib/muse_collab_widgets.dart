import 'package:flutter/material.dart';
import 'package:muse_collab_contract/muse_collab_contract.dart';

/// One authenticated live position. Names come from the trusted Host resolver;
/// location labels come from the authorized content domain, never raw drafts.
final class CollaborationPerson {
  const CollaborationPerson({
    required this.uid,
    required this.sessionId,
    required this.name,
    this.location,
  });
  final String uid, sessionId, name;
  final String? location;
}

final class _Member {
  const _Member(this.uid, this.name, this.positions);
  final String uid, name;
  final List<CollaborationPerson> positions;
}

final class _Roster {
  const _Roster(this.members, this.knownSelf, {this.invalid = false});
  final List<_Member> members;
  final bool knownSelf, invalid;
}

/// Shared resource-neutral online-user entry. Presence is not an ACL roster.
final class CollaboratorsButton extends StatelessWidget {
  const CollaboratorsButton({
    super.key,
    required this.listenable,
    required this.people,
    required this.connected,
    required this.selfUid,
    required this.selfSessionId,
    this.buttonKey,
  });
  final Listenable listenable;
  final List<CollaborationPerson> Function() people;
  final bool Function() connected;
  final String? Function() selfUid;
  final String selfSessionId;
  final Key? buttonKey;
  _Roster roster() {
    final uid = selfUid(), known = connected() && uid != null;
    final values = [
      ...people(),
      if (known)
        CollaborationPerson(
          uid: uid,
          sessionId: selfSessionId,
          name: '我',
          location: '当前会话',
        ),
    ];
    try {
      final groups = groupPresenceSlots(
        values.map(
          (p) =>
              CollaborationPresenceSlot(actorId: p.uid, sessionId: p.sessionId),
        ),
      );
      final positions = {for (final value in values) value.sessionId: value};
      final members = [
        for (final group in groups)
          _Member(
            group.actorId,
            known && group.actorId == uid
                ? '我'
                : values.firstWhere((p) => p.uid == group.actorId).name,
            [for (final id in group.sessionIds) positions[id]!],
          ),
      ];
      members.sort(
        (a, b) => a.uid == b.uid
            ? 0
            : known && a.uid == uid
            ? -1
            : known && b.uid == uid
            ? 1
            : a.uid.compareTo(b.uid),
      );
      return _Roster(members, known);
    } on FormatException {
      return const _Roster([], false, invalid: true);
    }
  }

  Widget avatar(_Member member, {double radius = 20}) => CircleAvatar(
    radius: radius,
    backgroundColor: collaboratorSlotColor(member.uid),
    child: Icon(
      Icons.person,
      color: Colors.white,
      size: radius == 13 ? 16 : 24,
    ),
  );
  Widget memberTile(_Member member) {
    final count = member.positions.length;
    if (count == 1)
      return ListTile(
        key: ValueKey('collaboration.member.${member.uid}'),
        leading: avatar(member),
        title: Text(member.name),
        subtitle: Text(member.positions.single.location ?? '在线'),
      );
    return ExpansionTile(
      key: ValueKey('collaboration.member.${member.uid}'),
      leading: avatar(member),
      title: Text(member.name),
      subtitle: Text('$count 处在线'),
      children: [
        for (var i = 0; i < count; i++)
          ListTile(
            key: ValueKey(
              'collaboration.position.${member.positions[i].sessionId}',
            ),
            title: Text(
              member.positions[i].sessionId == selfSessionId
                  ? '当前会话'
                  : '在线位置 ${i + 1}',
            ),
            subtitle: Text(member.positions[i].location ?? '在线'),
          ),
      ],
    );
  }

  @override
  Widget build(BuildContext context) => ListenableBuilder(
    listenable: listenable,
    builder: (context, _) {
      final current = roster();
      return Semantics(
        label: '在线协作',
        child: TextButton(
          key: buttonKey,
          onPressed: () => showModalBottomSheet<void>(
            context: context,
            showDragHandle: true,
            builder: (_) => SafeArea(
              child: ListenableBuilder(
                listenable: listenable,
                builder: (context, _) {
                  final latest = roster();
                  return Padding(
                    padding: const EdgeInsets.fromLTRB(20, 0, 20, 20),
                    child: Column(
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        const ListTile(title: Text('在线协作者')),
                        Flexible(
                          child: ListView(
                            shrinkWrap: true,
                            children: [
                              if (latest.invalid)
                                const ListTile(title: Text('在线名单暂不可用'))
                              else ...[
                                if (!latest.knownSelf && connected())
                                  const ListTile(title: Text('身份尚未确认，在线人数待确认')),
                                for (final member in latest.members)
                                  memberTile(member),
                                if (latest.members.isEmpty)
                                  const ListTile(title: Text('暂无其他在线协作者')),
                              ],
                              if (!connected())
                                const ListTile(title: Text('正在重新连接，本机修改仍保留')),
                            ],
                          ),
                        ),
                      ],
                    ),
                  );
                },
              ),
            ),
          ),
          child: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              for (final member in current.members.take(3))
                Padding(
                  padding: const EdgeInsets.only(right: 3),
                  child: avatar(member, radius: 13),
                ),
              if (current.members.isEmpty)
                const Icon(Icons.group_outlined, size: 20),
              const SizedBox(width: 4),
              Text(
                current.knownSelf && !current.invalid
                    ? '${current.members.length} 人'
                    : '在线协作',
              ),
            ],
          ),
        ),
      );
    },
  );
}

Color collaboratorSlotColor(String actorId) {
  const colors = [
    Color(0xff6c5ce7),
    Color(0xff00897b),
    Color(0xffc75c21),
    Color(0xff1976d2),
    Color(0xffa7358b),
  ];
  return colors[actorId.codeUnits.fold<int>(
        0,
        (a, b) => (a * 31 + b) & 0x7fffffff,
      ) %
      colors.length];
}
