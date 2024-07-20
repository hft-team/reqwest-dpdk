use std::env;

// This is using the `tokio` runtime. You'll need the following dependency:
//
// `tokio = { version = "1", features = ["full"] }`
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let args: Vec<String> = env::args().collect();
    tokio::fstack_init(args.len(), args);

    // Some simple CLI args requirements...
    let url = String::from("https://ifconfig.me/ip");
    eprintln!("Fetching {url:?}...");

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let local_ip = String::from("192.168.8.107:0");
    let local_ip: std::net::SocketAddr = local_ip.parse().expect("Unable to parse socket address");

    rt.block_on(async {
        let local = tokio::task::LocalSet::new();
        local.run_until(async move {
            let client = reqwest::ClientBuilder::new()
                .local_address(local_ip.ip())   // bind client to local ip
                .build()
                .expect("client builder failed");

            let res = client.get(&url).send().await.expect("request failed");

            eprintln!("Response: {:?} {}", res.version(), res.status());
            eprintln!("Headers: {:#?}\n", res.headers());

            let body = res.text().await.expect("response body failed");
            println!("body: {body}");
        }).await;
    });
}

// The [cfg(not(target_arch = "wasm32"))] above prevent building the tokio::main function
// for wasm32 target, because tokio isn't compatible with wasm32.
// If you aren't building for wasm32, you don't need that line.
// The two lines below avoid the "'main' function not found" error when building for wasm32 target.
#[cfg(target_arch = "wasm32")]
fn main() {}
