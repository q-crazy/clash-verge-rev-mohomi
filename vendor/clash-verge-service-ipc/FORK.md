# Windows x64 core filename fork

This source snapshot is from [clash-verge-service-ipc](https://github.com/clash-verge-rev/clash-verge-service-ipc), commit `b2a7427d6681c8d3780ab83a73f4dc21da2f22d6` (v2.7.6). The upstream GPL-3.0 license is retained in [LICENSE](LICENSE).

The local Windows x64 fork accepts `verge-mohomi.exe` and `verge-mohomi-alpha.exe` when preparing, inspecting, and discovering bundled cores. Residual-process protection also detects the original Windows core names during upgrades. Hash attestation, protected source validation, approved-copy checks, service identifiers, and controller endpoints retain their upstream behavior. Other architectures and platforms retain their original executable names.

Build the matching Windows x64 service, installer, and uninstaller from this same source; upstream precompiled helpers enforce the original core names and cannot be substituted. The application uses the local client dependency and packages these locally built helpers. This fork preserves upstream version `2.7.6`; its source commit and local filename changes identify the customized artifacts.
