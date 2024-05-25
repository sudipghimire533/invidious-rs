use crate::endpoints_impl::impl_reqwest::*;
use crate::endpoints_impl::impl_template::*;

use crate::endpoints::InstanceUrl;

fn get_instance() -> InstanceUrl {
    InstanceUrl::new("https://vid.puffyan.us".parse().unwrap(), "/api/v1")
}

#[tokio::test]
async fn test_sample_endpoint() {
    let res = REQWEST_INVIDIOUS.get_instance_stats(&get_instance()).await;
    assert!(res.is_ok());
}
