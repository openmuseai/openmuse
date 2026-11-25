# Muse asset repository

Existing object-scoped Cloud, cached and local repository implementations moved from the smart-document Flutter adapter. The neutral asset contract stays in `muse_asset_contract`; content-domain state and file/IndexedDB store adapters remain with their owners. No storage route, authorization model or asset reference JSON format is changed by this extraction.
