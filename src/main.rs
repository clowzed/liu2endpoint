use liu2endpoint::configuration::{Configuration, ConfigurationReader, EnvConfigurationReader};
use noem::{
    adapter::{
        chain::builder::ChainBuilder, unified::Unified, unimail::multipart::UnimailToMultipart,
    },
    fetcher::strategies::liu::{
        parameters::{
            ImapConnectionParameters, ImapCredentials, StrategyParameters as LIUStrategyParameters,
        },
        Strategy as LIUStrategy,
    },
    manager::Manager,
    resender::strategies::endpoint::{
        parameters::StrategyParameters as EndpointStrategyParameters, Strategy as EndpointStrategy,
    },
};
use std::{path::PathBuf, time::Duration};
use tracing_subscriber::layer::SubscriberExt;

#[tokio::main]
async fn main() {
    let stdout_log = tracing_subscriber::fmt::layer().pretty();
    let subscriber = tracing_subscriber::Registry::default()
        .with(stdout_log)
        .with(tracing_subscriber::EnvFilter::from_default_env());

    tracing::subscriber::set_global_default(subscriber).ok();

    let configuration = match EnvConfigurationReader::read::<Configuration, PathBuf>(Option::None) {
        Ok(config) => config,
        Err(cause) => {
            tracing::error!(%cause, "Failed to read configuration from environment variables!");
            return;
        }
    };

    let host = configuration.imap_host().to_owned();
    let port = configuration.imap_port();

    let liu_strategy_connection_parameters = ImapConnectionParameters::new(host, port);

    let login = configuration.imap_login().to_owned();
    let password = configuration.imap_password().to_owned();

    let liu_strategy_credentials = ImapCredentials::new(login, password);

    let folder = configuration.imap_folder().to_owned();

    let liu_strategy_parameters = LIUStrategyParameters::new(
        liu_strategy_connection_parameters,
        liu_strategy_credentials,
        folder,
    );
    let liu_strategy = LIUStrategy::new(liu_strategy_parameters);

    let url = configuration.resend_to().to_owned();

    let resender_parameters = EndpointStrategyParameters::new(url);

    let adapter = ChainBuilder::new(Unified::default())
        .chain(UnimailToMultipart)
        .build();

    let resender = EndpointStrategy::new(resender_parameters);

    let manager = Manager::new(liu_strategy, resender, adapter);

    let interval_duration = Duration::from_secs(configuration.recheck_interval());

    let mut interval = tokio::time::interval(interval_duration);

    loop {
        interval.tick().await;

        if let Err(cause) = manager.run().await {
            tracing::error!(%cause, "Failed to manage!");
        }
    }
}
