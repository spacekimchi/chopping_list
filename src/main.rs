use chopping_list::telemetry::{get_subscriber, init_subscriber};
use chopping_list::configuration::get_configuration;
use chopping_list::startup::Application;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    /* a way for application to ignore errors from loading .env instead of failing */
    dotenv::dotenv().ok();

    let subscriber = get_subscriber("chopping_list".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let application = Application::build(configuration).await?;

    application.run_until_stopped().await
}
