use async_openai::{types::CreateCompletionRequestArgs, Client};

async fn _sample_request() {
    dotenv::dotenv().ok();
    let client = Client::new();
    let request = CreateCompletionRequestArgs::default()
        .model("text-davinci-003")
        .prompt("hello")
        .build()
        .unwrap();

    let response = client.completions().create(request).await.unwrap();
    println!("{}", response.choices.get(0).unwrap().text);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sample_request() {
        _sample_request().await;
    }
}
