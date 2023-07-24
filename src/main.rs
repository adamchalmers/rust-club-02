use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let req = client.get("https://api.kittycad.io");
    let resp_future = req.send();
    match resp_future.await {
        Ok(response) => match response.text().await {
            Ok(b) => print_json(&b),
            Err(e) => {
                exit(&format!("Could not curl KittyCAD API: {e}"));
            }
        },
        Err(e) => exit(&format!("Could not curl KittyCAD API: {e}")),
    }

    let serialized = ApiResponse {
        components: Components {
            responses: Responses {
                error: serde_json::Value::String("foo".to_owned()),
                error_code: 3333,
            },
        },
    };
    println!("The API response is");
    println!("{}", serde_json::to_string_pretty(&serialized).unwrap());
}

/// Note the ! type signature.
/// This function never returns (because it terminates the program)
fn exit(s: &str) -> ! {
    eprintln!("{s}");
    std::process::exit(1)
}

#[derive(Deserialize, Serialize, Debug)]
struct ApiResponse {
    components: Components,
}

#[derive(Deserialize, Serialize, Debug)]
struct Components {
    responses: Responses,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Responses {
    error: serde_json::Value,
    #[serde(default)]
    error_code: i32,
}

fn print_json(body: &str) {
    let resp: ApiResponse = match serde_json::from_str(body) {
        Ok(v) => v,
        Err(e) => exit(&format!("KittyCAD API returned invalid JSON: {e}")),
    };
    println!("API error type is {:#?}", resp.components.responses.error);
    println!(
        "API error code is {:#?}",
        resp.components.responses.error_code
    )
}
