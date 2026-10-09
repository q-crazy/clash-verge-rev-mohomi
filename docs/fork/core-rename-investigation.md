# Core executable filename boundaries

The accepted scope is Windows **x86_64/x64**, with `verge-mohomi.exe` and `verge-mohomi-alpha.exe`. The baseline is Clash Verge Rev v2.5.8 (`4337bd85758d54d7b9044b2da3c9ac1176eae1f3`), with service v2.7.6 (`b2a7427d6681c8d3780ab83a73f4dc21da2f22d6`). This document records the implementation boundaries; runtime results belong in [README.md](README.md).

## Mapping

| Boundary | Windows x64 behavior |
| --- | --- |
| Persisted `clash_core` | Keep `verge-mihomo` and `verge-mihomo-alpha`; no configuration migration. |
| Local executable | Map those identifiers to `verge-mohomi` and `verge-mohomi-alpha`, then append `.exe`. |
| Prebuild resource | Use the new stem with `-x86_64-pc-windows-msvc.exe`. |
| Tauri package | The fork configuration declares the two new sidecar stems. |
| Launch and validation | Sidecar execution, configuration validation, executable fingerprinting, service installation/start, and service health inspection share the application filename mapping. |
| Core updates | Map the managed target and rollback filename; retain upstream archive URLs, version checks, staging, restart and rollback. |
| Installer | Stage new filenames with hashes of the exact bundled bytes; detect new and legacy residual processes; clean displaced core files. |
| Occupancy notices | Recognize new Windows process names and existing legacy diagnostics. |

Configuration defaults, enhancement selection, UI channel identifiers, controller endpoints, upstream asset names and attribution retain their existing meanings. Shared code preserves the original filenames outside Windows x64. There is no blanket replacement of `mihomo`.

## Service source and packaged helpers

The separate service enforces core filenames before elevation and again during installation inspection. An application-only rename would be rejected. The local service source therefore changes these narrow allowlists on Windows x64, bundled-core discovery, and Windows residual-process recognition. Legacy core processes remain recognized to prevent a second core from starting during an upgrade.

The application uses the vendored source as a path dependency. Prebuild detects that path and builds the service, installer and uninstaller from the same source before packaging. The source version and upstream commit are recorded in `vendor/clash-verge-service-ipc/FORK.md`.

The following protections remain: staged-copy SHA-256 attestation, verification after publication, trusted installation paths, managed executable inspection, rejection of bookkeeping names, and refusal to start a second residual core. Renaming the approved copy back to an old executable filename or bypassing these checks would not satisfy this request.

Pinned upstream evidence:

- [Management preparation](https://github.com/clash-verge-rev/clash-verge-service-ipc/blob/b2a7427d6681c8d3780ab83a73f4dc21da2f22d6/src/management.rs).
- [Installation inspection](https://github.com/clash-verge-rev/clash-verge-service-ipc/blob/b2a7427d6681c8d3780ab83a73f4dc21da2f22d6/src/core/installation.rs).
- [Windows process exclusion](https://github.com/clash-verge-rev/clash-verge-service-ipc/blob/b2a7427d6681c8d3780ab83a73f4dc21da2f22d6/src/execution_windows.rs).
- [Privileged installer](https://github.com/clash-verge-rev/clash-verge-service-ipc/blob/b2a7427d6681c8d3780ab83a73f4dc21da2f22d6/src/bin/install_service.rs).
- [Approved runtime asset resolution](https://github.com/clash-verge-rev/clash-verge-service-ipc/blob/b2a7427d6681c8d3780ab83a73f4dc21da2f22d6/src/core/runtime_generation/assets.rs).

## Endpoints and updates

The owner/channel scoped application and service controller pipes are unchanged. Application updates from upstream are rejected for this Windows x64 fork; core updates still use original MetaCubeX download assets. See README for the package build command and manual application-update policy.

## Verification scope

The package and installed stable core were verified, including system proxy, scoped controller readiness, service handoff, TUN routing, and DNS interception. Alpha switching and core update/rollback remain unexercised. See [build and runtime evidence](README.md) for the checks and limitations.
