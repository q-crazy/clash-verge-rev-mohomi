# Clash Verge Rev — mohomi Windows x64 fork

A Windows x64 fork of [Clash Verge Rev v2.5.8](https://github.com/clash-verge-rev/clash-verge-rev), using `verge-mohomi.exe` and `verge-mohomi-alpha.exe` for its stable and alpha cores, with matching service helpers. Existing configuration identifiers, subscriptions, rules, and controller endpoints retain their upstream behavior.

## Download and install

[Download the Windows x64 installer](https://github.com/q-crazy/clash-verge-rev-mohomi/releases/download/v2.5.8-mohomi.1/Clash-Verge-Rev-Mohomi_2.5.8-mohomi.1_windows-x64-setup.exe), then run the setup file. See the [release page](https://github.com/q-crazy/clash-verge-rev-mohomi/releases/tag/v2.5.8-mohomi.1) for the checksum and package details. This fork publishes Windows x64 packages only; the installed application still reports version 2.5.8.

## Updates

Upstream application updates are disabled to preserve the customized core filenames. Install future fork application releases manually from this repository. Core updates remain enabled and use the original MetaCubeX downloads.

## Source and attribution

Based on [Clash Verge Rev](https://github.com/clash-verge-rev/clash-verge-rev) commit `4337bd85758d54d7b9044b2da3c9ac1176eae1f3`, with vendored [clash-verge-service-ipc v2.7.6](https://github.com/clash-verge-rev/clash-verge-service-ipc) and [MetaCubeX/mihomo](https://github.com/MetaCubeX/mihomo) cores. Clash Verge Rev continues [zzzgydi/clash-verge](https://github.com/zzzgydi/clash-verge). This is an independently maintained fork.

The application and vendored service retain their GPL-3.0 licenses: [application license](LICENSE), [service license](vendor/clash-verge-service-ipc/LICENSE). Upstream copyright notices are preserved. See [fork build and verification notes](docs/fork/README.md) and [implementation boundaries](docs/fork/core-rename-investigation.md) for details.
