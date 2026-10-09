use crate::core::runtime_generation::approved_core_copy;
use crate::core::trusted_core_location::require_trusted_core_location;
use crate::core::{
    desired::{load_active_owner, load_owner_desired_state},
    manager::CORE_MANAGER,
    paths::service_paths,
};
use crate::{CoreAvailability, CoreInspection, CoreRequirement, InstallationStatus, ProtocolInfo};
use std::path::Path;

pub(super) async fn inspect(
    requirements: &[CoreRequirement],
    include_service_digest: bool,
) -> anyhow::Result<InstallationStatus> {
    anyhow::ensure!(requirements.len() <= 2, "at most two cores can be inspected");
    let recovering = match load_active_owner().await? {
        Some(owner) => load_owner_desired_state(&owner.owner_key).await?.core_should_be_running,
        None => false,
    };
    let core_busy = recovering
        || CORE_MANAGER.lock().await.status().await.core_pid.is_some()
        || crate::execution::CoreExecutionGuard::is_held()?;
    let requirements = requirements.to_vec();
    tokio::task::spawn_blocking(move || {
        let directory = service_paths()?.core_dir();
        let cores = requirements
            .iter()
            .map(|required| {
                let availability = inspect_core(&directory, required);
                CoreInspection {
                    name: required.name.clone(),
                    availability,
                }
            })
            .collect();
        Ok(InstallationStatus {
            service_sha256: if include_service_digest {
                crate::management::sha256_file(&std::env::current_exe()?)?
            } else {
                String::new()
            },
            protocol: ProtocolInfo::current(),
            cores,
            core_busy,
        })
    })
    .await?
}

fn inspect_core(directory: &Path, required: &CoreRequirement) -> CoreAvailability {
    #[cfg(all(windows, target_arch = "x86_64"))]
    let allowed = ["verge-mohomi.exe".to_owned(), "verge-mohomi-alpha.exe".to_owned()];
    #[cfg(not(all(windows, target_arch = "x86_64")))]
    let allowed = [
        format!("verge-mihomo{}", std::env::consts::EXE_SUFFIX),
        format!("verge-mihomo-alpha{}", std::env::consts::EXE_SUFFIX),
    ];
    if !allowed.contains(&required.name) {
        return CoreAvailability::Rejected {
            reason: "unsupported core name".into(),
        };
    }
    let requested = directory.join(&required.name);
    if matches!(std::fs::symlink_metadata(&requested), Err(error) if error.kind() == std::io::ErrorKind::NotFound) {
        return CoreAvailability::Missing;
    }
    let result = (|| -> anyhow::Result<CoreAvailability> {
        let approved = approved_core_copy(directory, &requested)?;
        require_trusted_core_location(&approved)?;
        if let Some(expected) = &required.sha256 {
            anyhow::ensure!(
                expected.len() == 64 && expected.bytes().all(|c| c.is_ascii_hexdigit()),
                "invalid SHA-256"
            );
            if crate::management::sha256_file(&approved)? != expected.to_ascii_lowercase() {
                return Ok(CoreAvailability::DigestMismatch);
            }
        }
        Ok(CoreAvailability::Ready)
    })();
    result.unwrap_or_else(|error| CoreAvailability::Rejected {
        reason: error.to_string(),
    })
}

#[cfg(all(test, windows, target_arch = "x86_64"))]
mod tests {
    use super::*;

    #[test]
    fn renamed_windows_cores_are_inspected_without_accepting_other_names() {
        let directory = std::env::temp_dir().join(format!("missing-mohomi-core-{}", std::process::id()));
        for name in ["verge-mohomi.exe", "verge-mohomi-alpha.exe"] {
            assert_eq!(
                inspect_core(
                    &directory,
                    &CoreRequirement {
                        name: name.into(),
                        sha256: None
                    }
                ),
                CoreAvailability::Missing
            );
        }
        assert!(matches!(
            inspect_core(
                &directory,
                &CoreRequirement {
                    name: "other.exe".into(),
                    sha256: None
                }
            ),
            CoreAvailability::Rejected { .. }
        ));
    }
}
