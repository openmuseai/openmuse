# Collaboration presence widgets

Shared desktop/mobile presence button and live member sheet. Resource plugins
supply authenticated user/session IDs, a listenable, trusted Host display names
and authorized location labels. The neutral contract groups active positions by
user; one user with several positions counts once. The current identity must be
known before a numeric count is displayed. User colors stay stable across slots.

Multiple positions expand in a scrolling sheet. Connection slots are not claimed
to be physical devices. Content, CRDT merging, transport, identity lookup and the
sharing ACL remain outside this package. The internal-preview API now requires
`CollaborationPerson.uid/sessionId` and `CollaboratorsButton.selfUid/selfSessionId`;
display names alone cannot establish identity or an accurate count.
