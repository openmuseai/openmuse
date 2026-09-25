# OpenMuse adaptation

This is the MIT-licensed `dsh-model-capabilities` 0.5.0 source carried from
the separately licensed vendor package. `lib/index.js` is adapted for DSH
0.1.7-rc.1's profile-backed settings reader; the upstream plugin used the
removed `settings.get` method. `openmuse.patch.yml` mounts the local package
by relative source path so no other checkout or profile package installation
is required. The original LICENSE and README remain with the code.

The default desktop package must test the Models card and an actual fenced
settings write; HTTP startup alone is not proof that its browser half works.
