# @muse/plugin-appflowy-view-reference

Cordis/DSH adapter for the Host-authorized AppFlowy current View reference. The model cannot select
workspace or View IDs; the Plugin binds with the opaque `appflowy.selection=current` hint and exposes
only bounded, redacted structural metadata.

This package does not import AppFlowy, Flutter, CRDT, database, Shell, or filesystem APIs.
