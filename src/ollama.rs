use ollama_rs::Ollama;

pub fn ollama() -> Ollama {
    Ollama::default()
    //     let res = ollama.list_local_models().await.unwrap();
}

pub async fn get_models() -> dyn Future {
    ollama().list_local_models()
}
