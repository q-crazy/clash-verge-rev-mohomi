# Windows x64 fork build and verification

Based on Clash Verge Rev **v2.5.8**, commit `4337bd85758d54d7b9044b2da3c9ac1176eae1f3`. Publication scope is recorded in [issue #1](https://github.com/q-crazy/clash-verge-rev-mohomi/issues/1).

The Windows x64 implementation uses `verge-mohomi.exe` and `verge-mohomi-alpha.exe`. Existing stable/alpha configuration identifiers, upstream core download names, scoped controller endpoints, subscriptions, rules, proxy and TUN settings retain their upstream behavior. The application and matching service helpers resolve the renamed files. Other platform packages are outside this release.

## Build

The package was built with Node.js `24.21.0`, pnpm `12.10.0`, Rust `1.99.0`, MSVC x64 tools `14.51.36231`, and Windows SDK `10.0.26100.0`. Install the `x86_64-pc-windows-msvc` Rust target and GNU patch, and use a shell with the MSVC/SDK tools on PATH. Retain the dependency lockfiles. See [CONTRIBUTING.md](../../CONTRIBUTING.md) for general prerequisites.

Run from the repository root:

```powershell
pnpm install --frozen-lockfile
pnpm build:windows-x64
```

The fork command prepares x64 resources, builds the pinned local service helpers, and passes `src-tauri/tauri.mohomi.conf.json` to Tauri. It produces the ordinary NSIS installer under `target/x86_64-pc-windows-msvc/release/bundle/nsis`. Use this command for the customized package; the generic upstream build configuration retains its original sidecar declarations. The development wrapper also supplies the fork configuration on Windows x64.

Service source is vendored at `vendor/clash-verge-service-ipc`, based on **v2.7.6**, commit `b2a7427d6681c8d3780ab83a73f4dc21da2f22d6`. The application client and packaged service/install/uninstall tools use this same source. Prebuild compiles these helpers rather than downloading incompatible upstream helpers.

## Update policy

Application updates from upstream are rejected on Windows x64 because they would replace this fork's executable names. This applies to manual checks, background checks, and cached update eligibility. Signed application updater artifacts are not produced; install future fork application packages manually.

Core updates remain enabled: they download original MetaCubeX stable/alpha assets and use the renamed destinations with the existing hash attestation and rollback behavior. See [implementation boundaries](core-rename-investigation.md) for the filename mapping and security constraints.

## Package and build evidence

On 2026-10-09 (Asia/Shanghai), frozen frontend dependency installation, locked Rust dependency fetch, and the full Windows x64 release build completed successfully. TypeScript/Vite compilation, six focused notice-rendering cases, affected frontend lint/formatting, Rust formatting, and ten focused service tests passed. The service tests cover renamed filename inspection, staged-byte attestation, and installer publication/rejection. The original service snapshot rejected the renamed filename with `unsupported core name`; the modified snapshot passed. Application Rust unit tests were not run.

The original build output was `Clash Verge_2.5.8_x64-setup.exe`. The release publishes the same bytes as `Clash-Verge-Rev-Mohomi_2.5.8-mohomi.1_windows-x64-setup.exe`:

- Size: **59,786,375 bytes**.
- SHA-256: `a89d09cd41cdd98dc28bc39494f288a8b5a9ba765c20587e177c1891118e95c0`.

Archive integrity checking and extraction passed. The package contains both renamed cores and locally built matching service helpers; no packaged executable basename contains `mihomo`. The application, both cores and all three service helpers have PE machine `0x8664` (x64). Extracted helper hashes match the built resources.

The stable core is v1.19.32 and the bundled alpha core is alpha-e4dd968. Both reported their expected versions and passed isolated configuration checks. Their packaged hashes match installer attestation:

| Resource | SHA-256 |
| --- | --- |
| `verge-mohomi-x86_64-pc-windows-msvc.exe` | `3c107d54ef1b895e4980b53f517d52f3c1137101fa2637347c9ce343fc50d7b2` |
| `verge-mohomi-alpha-x86_64-pc-windows-msvc.exe` | `69f2a312636b538e9bbdff4cf4403f4e4f70d9ac50ad0674838b35cac7bceb60` |

## Installed runtime evidence

On 2026-10-09, the installed application, both cores and three application-resource service helpers matched the extracted package hashes. The stable renamed core ran successfully in Sidecar mode with a responsive scoped controller and working system-proxy browsing.

After service installation, application logs confirmed a successful Sidecar-to-Service transition. The service hosted the renamed stable core. TUN was enabled with the virtual adapter up, IPv4/IPv6 default routes, and DNS interception. All 15 HTTPS checks without an explicit proxy passed with expected responses; controller records correlated them with TUN ingress. Local DNS responded and an external-address DNS query was intercepted with a fake-IP response. An earlier standalone DNS bind failure was no longer present in Service/TUN mode.

Alpha switching, core update/rollback, service reinstall, and native UI automation were not exercised. Protected service/core copies could not be hashed from the unelevated validation shell; the installed application-resource helpers had already matched the package. These observations describe the tested installation, not a guarantee for every environment.
