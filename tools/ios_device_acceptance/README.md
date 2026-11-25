# Physical channel acceptance

This is a stateful XCTest driver for an already-installed Office Harness channel
App. It does not build, replace, or re-sign the production App. Run a selected
method with `-only-testing`; do not execute the entire class unattended. Each
method documents its required screen. Only bundled QA documents may be edited.
Observation-only cases are not functional acceptance passes. Historical failed
assertions remain recorded alongside later attempts.

`generate_project.rb /tmp/<driver>` creates the separate signed runner. The
optional `private-auth` argument suppresses automatic system attachments. For
existing-account login, forward `MUSE_ACCEPTANCE_EMAIL` and
`MUSE_ACCEPTANCE_PASSWORD` using Xcode's `TEST_RUNNER_` environment variables;
read them in memory from the user-authorized source, redact subprocess output,
and never put values in shell arguments, source files, schemes, or receipts.
Authentication pages must not be captured. Enter system passcodes on the device.

`pptx_host_diagnostic.dart` is a separate development fixture. It loads the exact
bundled MCPB and an explicitly supplied synthetic `QA_PPTX_BASE64` document,
tracing button event kinds and standard download-message byte counts. Its
acknowledging sink does **not** verify production resource persistence. Build
and install it under `com.openmuseai.qa.pptxdiagnostic`, never the production
bundle identifier. It must not be uploaded to a store or counted as a release
candidate. The current fixture references the 0.9.7 MCPB used for the 112503
failure investigation.
