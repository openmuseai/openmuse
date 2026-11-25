# Resource surface contribution

`ResourceEmbedTarget` reuses the public ResourceReference v1 identity/version
contract. A surface ID and opaque, bounded selector identify an installed
contribution (database view, future Office page, video, app). It contains no
transport or permission information. This package owns no CRDT or content model.

Hosts bind `ResourceEmbedHost` to an authenticated resource session. Picking a
target does not grant access. Each contribution checks the child resource anew;
the parent policy is an additional upper bound. Historical parents must disable
live resolution. Child persistence/history/lifetime belong to the child plugin.
Removing a locator never deletes its referenced resource.
