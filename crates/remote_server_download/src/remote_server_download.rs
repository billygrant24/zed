use anyhow::{Context as _, Result};
use client::Client;
use futures_lite::StreamExt;
use gpui::AsyncApp;
use http_client::{HttpClient, HttpClientWithUrl};
use release_channel::ReleaseChannel;
use semver::Version;
use serde::{Deserialize, Serialize};
use smol::{fs::File, io::AsyncReadExt};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::Arc,
    time::SystemTime,
};

const REMOTE_SERVER_CACHE_LIMIT: usize = 5;

#[derive(Serialize, Debug)]
struct AssetQuery<'a> {
    asset: &'a str,
    os: &'a str,
    arch: &'a str,
    metrics_id: Option<&'a str>,
    system_id: Option<&'a str>,
    is_staff: Option<bool>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct ReleaseAsset {
    version: String,
    url: String,
}

pub async fn download_remote_server_release(
    release_channel: ReleaseChannel,
    version: Option<Version>,
    os: &str,
    arch: &str,
    set_status: impl Fn(&str, &mut AsyncApp) + Send + 'static,
    cx: &mut AsyncApp,
) -> Result<PathBuf> {
    set_status("Fetching remote server release", cx);
    let release =
        get_release_asset(release_channel, version, "zed-remote-server", os, arch, cx).await?;

    let servers_dir = paths::remote_servers_dir();
    let channel_dir = servers_dir.join(release_channel.dev_name());
    let platform_dir = channel_dir.join(format!("{os}-{arch}"));
    let version_path = platform_dir.join(format!("{}.gz", release.version));
    smol::fs::create_dir_all(&platform_dir).await.ok();

    if smol::fs::metadata(&version_path).await.is_err() {
        log::info!(
            "downloading zed-remote-server {os} {arch} version {}",
            release.version
        );
        set_status("Downloading remote server", cx);
        let client = cx.update(|cx| Client::global(cx).http_client());
        download_remote_server_binary(&version_path, release, client).await?;
    }

    if let Err(error) =
        cleanup_remote_server_cache(&platform_dir, &version_path, REMOTE_SERVER_CACHE_LIMIT).await
    {
        log::warn!(
            "Failed to clean up remote server cache in {:?}: {error:#}",
            platform_dir
        );
    }

    Ok(version_path)
}

pub async fn get_remote_server_release_url(
    release_channel: ReleaseChannel,
    version: Option<Version>,
    os: &str,
    arch: &str,
    cx: &mut AsyncApp,
) -> Result<Option<String>> {
    let release =
        get_release_asset(release_channel, version, "zed-remote-server", os, arch, cx).await?;
    Ok(Some(release.url))
}

async fn get_release_asset(
    release_channel: ReleaseChannel,
    version: Option<Version>,
    asset: &str,
    os: &str,
    arch: &str,
    cx: &mut AsyncApp,
) -> Result<ReleaseAsset> {
    let version = if let Some(mut version) = version {
        version.pre = semver::Prerelease::EMPTY;
        version.build = semver::BuildMetadata::EMPTY;
        version.to_string()
    } else {
        "latest".to_string()
    };
    let http_client = cx.update(|cx| Client::global(cx).http_client());
    let path = format!("/releases/{}/{version}/asset", release_channel.dev_name());
    let url = http_client.build_zed_cloud_url_with_query(
        &path,
        AssetQuery {
            os,
            arch,
            asset,
            metrics_id: None,
            system_id: None,
            is_staff: None,
        },
    )?;

    let mut response = http_client
        .get(url.as_str(), Default::default(), true)
        .await?;
    let mut body = Vec::new();
    response.body_mut().read_to_end(&mut body).await?;

    anyhow::ensure!(
        response.status().is_success(),
        "failed to fetch remote server release: {:?}",
        String::from_utf8_lossy(&body),
    );

    serde_json::from_slice(body.as_slice()).with_context(|| {
        format!(
            "error deserializing remote server release {:?}",
            String::from_utf8_lossy(&body),
        )
    })
}

async fn download_remote_server_binary(
    target_path: &Path,
    release: ReleaseAsset,
    client: Arc<HttpClientWithUrl>,
) -> Result<()> {
    let temp = tempfile::Builder::new().tempfile_in(paths::remote_servers_dir())?;
    let mut temp_file = File::create(&temp).await?;

    let mut response = client.get(&release.url, Default::default(), true).await?;
    anyhow::ensure!(
        response.status().is_success(),
        "failed to download remote server release: {:?}",
        response.status()
    );
    smol::io::copy(response.body_mut(), &mut temp_file).await?;
    smol::fs::rename(&temp, target_path).await?;

    Ok(())
}

async fn cleanup_remote_server_cache(
    platform_dir: &Path,
    keep_path: &Path,
    limit: usize,
) -> Result<()> {
    if limit == 0 {
        return Ok(());
    }

    let mut entries = smol::fs::read_dir(platform_dir).await?;
    let now = SystemTime::now();
    let mut candidates = Vec::new();

    while let Some(entry) = entries.next().await {
        let entry = entry?;
        let path = entry.path();
        if path.extension() != Some(OsStr::new("gz")) {
            continue;
        }

        let modified = if path == keep_path {
            now
        } else {
            smol::fs::metadata(&path)
                .await
                .and_then(|metadata| metadata.modified())
                .unwrap_or(SystemTime::UNIX_EPOCH)
        };
        candidates.push((path, modified));
    }

    if candidates.len() <= limit {
        return Ok(());
    }

    candidates.sort_by(|(path_a, time_a), (path_b, time_b)| {
        time_b.cmp(time_a).then_with(|| path_a.cmp(path_b))
    });

    for (index, (path, _)) in candidates.into_iter().enumerate() {
        if index < limit || path == keep_path {
            continue;
        }
        if let Err(error) = smol::fs::remove_file(&path).await {
            log::warn!(
                "Failed to remove old remote server archive {:?}: {error}",
                path
            );
        }
    }

    Ok(())
}
