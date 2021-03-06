use super::{cli, file};
use commons::MergeOptions;
use failure::Fallible;
use std::collections::HashSet;
use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;
use std::time;
use structopt::StructOpt;

/// Runtime application settings (validated config).
#[derive(Debug, SmartDefault)]
pub struct AppSettings {
    /// Listening address for the main service.
    #[default(IpAddr::V4(Ipv4Addr::LOCALHOST))]
    pub address: IpAddr,

    /// Optional auth secrets for the registry scraper.
    pub credentials_path: Option<PathBuf>,

    /// Required client parameters for the main service.
    pub mandatory_client_parameters: HashSet<String>,

    /// Metadata key where to record the manifest-reference.
    #[default("io.openshift.upgrades.graph.release.manifestref")]
    pub manifestref_key: String,

    /// Endpoints namespace for the main service.
    pub path_prefix: String,

    /// Pause (in seconds) between registry scrapes.
    #[default(time::Duration::from_secs(30))]
    pub pause_secs: time::Duration,

    /// Listening port for the main service.
    #[default(8080)]
    pub port: u16,

    // TODO(lucab): split this in (TLS, hostname+port).
    /// Target host for the registry scraper.
    #[default(crate::plugins::release_scrape_dockerv2::DEFAULT_SCRAPE_REGISTRY.to_string())]
    pub registry: String,

    /// Target image for the registry scraper.
    #[default(crate::plugins::release_scrape_dockerv2::DEFAULT_SCRAPE_REPOSITORY.to_string())]
    pub repository: String,

    /// Listening address for the status service.
    #[default(IpAddr::V4(Ipv4Addr::LOCALHOST))]
    pub status_address: IpAddr,

    /// Listening port for the status service.
    #[default(9080)]
    pub status_port: u16,

    /// Global log level.
    #[default(log::LevelFilter::Warn)]
    pub verbosity: log::LevelFilter,

    // TODO(lucab): drop this when plugins are configurable.
    /// Disable quay-metadata (Satellite compat).
    #[default(false)]
    pub disable_quay_api_metadata: bool,

    /// Concurrency for graph fetching
    #[default(crate::plugins::release_scrape_dockerv2::DEFAULT_FETCH_CONCURRENCY)]
    pub fetch_concurrency: usize,

    /// Metrics which are required to be registered, to be specified without the `METRICS_PREFIX`.
    /// If these are not registered by the time all plugins have been loaded an error will be thrown.
    #[default([
        "graph_upstream_raw_releases",
    ].iter().cloned().map(Into::into).collect())]
    pub metrics_required: HashSet<String>,
}

impl AppSettings {
    /// Lookup all optional configs, merge them with defaults, and
    /// transform into valid runtime settings.
    pub fn assemble() -> Fallible<Self> {
        // Source options.
        let cli_opts = cli::CliOptions::from_args();
        let file_opts = match &cli_opts.config_path {
            Some(ref path) => Some(file::FileOptions::read_filepath(path)?),
            None => None,
        };
        let defaults = Self::default();

        // Combine options into a single config.
        let mut cfg = defaults;
        cfg.try_merge(cli_opts)?;
        cfg.try_merge(file_opts)?;

        // Validate and convert to settings.
        Self::try_validate(cfg)
    }

    /// Validate and build runtime settings.
    fn try_validate(self) -> Fallible<Self> {
        if self.pause_secs.as_secs() == 0 {
            bail!("unexpected 0s pause");
        }

        Ok(self)
    }
}
