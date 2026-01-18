use email_processor::config::Config;
use email_processor::{process_ses_event, AppContext, SesEvent};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .json()
        .init();

    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .load()
        .await;
    let context = AppContext::new(&config);

    let lambda_config =
        Config::from_env().map_err(|e| Error::from(format!("Configuration error: {}", e)))?;

    run(service_fn(|event: LambdaEvent<SesEvent>| async {
        process_ses_event(event.payload, &context, &lambda_config).await
    }))
    .await
}
