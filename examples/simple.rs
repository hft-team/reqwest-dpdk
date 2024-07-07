use std::env;

// This is using the `tokio` runtime. You'll need the following dependency:
//
// `tokio = { version = "1", features = ["full"] }`
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let args: Vec<String> = env::args().collect();
    tokio::fstack_init(args.len(), args);

    // Some simple CLI args requirements...
    let url = String::from("https://hyper.rs");
    eprintln!("Fetching {url:?}...");

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let local = tokio::task::LocalSet::new();
        local.run_until(async move {
            let res = reqwest::get(url).await.expect("request failed");
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
