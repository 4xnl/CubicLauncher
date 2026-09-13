use super::*;

#[test]
fn nonce_increments() {
    let a = next_nonce().parse::<u64>().unwrap();
    let b = next_nonce().parse::<u64>().unwrap();
    assert_eq!(b, a + 1);
}

#[test]
fn buttons_include_url_in_activity() {
    let activity = Activity::builder()
        .button("GitHub", "https://github.com")
        .button("Docs", "https://docs.rs")
        .build();

    let (val, meta) = build_activity_payload(&activity);

    // Buttons include label AND url in the activity object
    let labels = val["buttons"].as_array().unwrap();
    assert_eq!(labels[0]["label"], "GitHub");
    assert_eq!(labels[0]["url"], "https://github.com");
    assert_eq!(labels[1]["label"], "Docs");
    assert_eq!(labels[1]["url"], "https://docs.rs");

    // No separate metadata field
    assert!(meta.is_none());
}

#[tokio::test]
async fn client_stores_id() {
    let client = DiscordRpcClient::new("123456789");
    assert_eq!(client.client_id, "123456789");
}
