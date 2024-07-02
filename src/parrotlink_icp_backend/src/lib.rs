use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};
use serde_json::json;

use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);
use ic_cdk::api::time;

#[ic_cdk::update]
async fn process_redeem_args(id: String, signature: String, vault_address: String, target_address: String) -> String {
    let host = "host";
    let url = "url";

    let request_headers = vec![
        HttpHeader {
            name: "Host".to_string(),
            value: format!("{host}:8080"),
        },
        HttpHeader {
            name: "User-Agent".to_string(),
            value: "ICP_BACKEND_CANISTER".to_string(),
        },
        HttpHeader {
            name: "Idempotency-Key".to_string(),
            value: generate_idempotency_key(),
        },
        HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        },
    ];

    let json_value = json!({
        "id": id,
        "signature": signature,
        "vault_address": vault_address,
        "target_address": target_address,
    });

    let json_string = json_value.to_string();
    let json_utf8: Vec<u8> = json_string.into_bytes();
    let request_body: Option<Vec<u8>> = Some(json_utf8);

    let request = CanisterHttpRequestArgument {
        url: url.to_string(),
        max_response_bytes: None, 
        method: HttpMethod::POST,
        headers: request_headers,
        body: request_body,
        transform: None,
    };

    match http_request(request, 2_000_000_000).await {

        Ok((response,)) => {
            let str_body = String::from_utf8(response.body)
                .expect("Transformed response is not UTF-8 encoded.");
            ic_cdk::api::print(format!("{:?}", str_body));
            let result: String = format!("{}", str_body);
            result
        }
        Err((r, m)) => {
            let message =
                format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");

            message
        }
    }
}
fn generate_idempotency_key() -> String {
    let timestamp = time();
    let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{}-{}", timestamp, counter)
}
