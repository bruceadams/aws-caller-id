use anyhow::{Context, Result};
use aws_config::{BehaviorVersion, SdkConfig};
use aws_sdk_sts::Client;
use aws_types::region::Region;
use clap::Parser;
use tracing::debug;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// Calls the AWS GetCallerIdentity API to get the
/// caller identity and prints the result.
///
/// There is little reason to run this tool.
/// My goal here is a simple, complete example for a
/// command line program that makes calls to AWS.
///
/// You can set the environment variable `RUST_LOG` to
/// adjust logging, for example `RUST_LOG=trace aws-caller-id`
#[derive(Clone, Debug, Parser)]
#[command(about, author, version)]
struct MyArgs {
    /// AWS profile to use.
    ///
    /// This overrides the standard (and complex!) AWS profile handling.
    #[arg(long, short)]
    profile: Option<String>,

    /// AWS region to target.
    ///
    /// This override the standard (and complex!) AWS region handling.
    #[arg(long, short)]
    region: Option<String>,
}

async fn aws_sdk_config(args: &MyArgs) -> SdkConfig {
    let base = aws_config::defaults(BehaviorVersion::latest());
    let with_profile = match &args.profile {
        None => base,
        Some(profile_name) => base.profile_name(profile_name),
    };
    let with_overrides = match &args.region {
        None => with_profile,
        Some(region_name) => with_profile.region(Region::new(region_name.clone())),
    };
    with_overrides.load().await
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
    let args = MyArgs::parse();
    let config = aws_sdk_config(&args).await;
    debug!("Config: {:?}", config);
    let client = Client::new(&config);
    let result = client
        .get_caller_identity()
        .send()
        .await
        .context("calling get_caller_identity")?;
    debug!("{:?}", result);
    if let Some(account) = result.account() {
        println!("account: {}", account)
    };
    if let Some(arn) = result.arn() {
        println!("arn: {}", arn)
    };
    if let Some(user_id) = result.user_id() {
        println!("user_id: {}", user_id)
    };
    Ok(())
}
