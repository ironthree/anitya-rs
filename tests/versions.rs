use std::time::Duration;

use anitya::v2::VersionQuery;
use anitya::ClientBuilder;

#[tokio::test]
async fn query() {
    env_logger::builder().filter_level(log::LevelFilter::Debug).init();

    let client = ClientBuilder::new("https://release-monitoring.org")
        .with_delay(Duration::from_millis(1000))
        .build()
        .unwrap();

    let query = VersionQuery::new(7635);
    let _result = client.request(&query).await.unwrap();
}
