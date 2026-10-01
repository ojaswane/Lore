// take the chunks and then embed them for better Ai search
// we are using candle for this.
use anyhow::Result;

use candle_core::{DType, Device, Tensor};
use candle_transformers::models::bert::{BertModel, Config};
use tokenizers::Tokenizer;

// for making a sentance embedding.
fn mean_pooling(embeddings: &Tensor, attention_mask: &Tensor) -> Result<Tensor> {
    unimplemented!();

    // [bin , sequence] = [bin , sequence] * [bin , sequence]
    let mask = attention_mask.unsqueeze(1)?.to_dtype(embeddings.dtype())?;
}

pub fn embed_text(chunks: &[String]) {
    let device = Device::Cpu;

    // local model path
    let model_dir = "models/all-MiniLM-L6-v2";

    let config_path = format!("{}/config.json", model_dir);
    let tokenizer_path = format!("{}/tokenizer.json", model_dir);
    let weights_path = format!("{}/model.safetensors", model_dir);
}
