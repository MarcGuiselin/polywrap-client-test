#[derive(serde::Serialize)]
pub struct PingArgs {
    pub message: String,
}

#[derive(serde::Deserialize)]
pub struct PongResult {
    pub response: String,
}

fn ping_test_wrap() {
    use polywrap::*;

    println!("Sending ping");

    let mut config = ClientConfig::new();
    config.add(SystemClientConfig::default().into());
    let client = Client::new(config.build());

    let add_file_resp = client
        .invoke::<PongResult>(
            &Uri::try_from("wrap://bogus/bogus").unwrap(),
            "ping",
            Some(
                &to_vec(&PingArgs {
                    message: "Hello from Rust!".to_string(),
                })
                .unwrap(),
            ),
            None,
            None,
        )
        .unwrap();
    println!("Response to ping: '{}'", add_file_resp.response);
}

fn main() {
    ping_test_wrap();
}
