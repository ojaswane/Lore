// take the chunks and then embed them for better Ai search
// we are using candle for this.
use anyhow::Result;

use candle_core::{DType, Device, Tensor};
use candle_transformers::models::bert::{BertModel, Config};
use tokenizers::Tokenizer;

// for making a sentance embedding.
fn mean_pooling(embeddings: &Tensor, attention_mask: &Tensor) -> Result<Tensor> {
    unimplemented!();

    // // [bin , sequence] = [bin , sequence] * [bin , sequence]
    // let mask = attention_mask.unsqueeze(1)?.to_dtype(embeddings.dtype())?;
}

pub fn embed_text(chunks: &[String]) {
    let device = Device::Cpu;

    // local model path
    let model_dir = "models/all-MiniLM-L6-v2";

    let config_path = format!("{}/config.json", model_dir);
    let tokenizer_path = format!("{}/tokenizer.json", model_dir);
    let weights_path = format!("{}/model.safetensors", model_dir);

    // Load tokenizer
    let tokenizer = Tokenizer::from_file(&tokenizer_path)
        .map_err(|e| anyhow::anyhow!("Failed to load tokenizer: {:?}", e))
        .unwrap();

    // Load BERT config
    let config: Config = serde_json::from_reader(std::fs::File::open(config_path)?)?;

    // load the weights from safetensors
    let vb = candle_nn::VarBuilder::from_mmaped_safetensors(&[weights_path], DType::F32, &device)?;

    let mini_llm = BertModel::new(&config, &vb, device)?;

    // text to embeded

    let text = Vec::new();

    for chunk in chunks {
        text = Vec::from([chunk.as_str()]);

        // tokenize the text

        let encoding = tokenizer
            .encode_batch(text.clone(), true)
            .map_err(|e| anyhow::anyhow!(e))?;
    }

    let input_ids: Vec<Vec<u32>> = encoding.iter().map(|e| e.get_ids().to_vec()).collect();

    let attention_masks: Vec<Vec<u32>> = encoding
        .iter()
        .map(|e| e.get_attention_mask().to_vec())
        .collect();

    // convert to tensors

    let seq_len = input_ids[0].len();

    let input_ids_flat: Vec<u32> = input_ids.into_iter().flatten().collect();

    let attention_flat: Vec<u32> = attention_masks.into_iter().flatten().collect();

    let input_ids = Tensor::from_vec(input_ids_flat, (texts.len(), seq_len), &device)?;

    let attention_mask = Tensor::from_vec(attention_flat, (texts.len(), seq_len), &device)?;

    // forward pass through the model
    let token_embeddings = model.forward(&input_ids, &attention_mask)?;
}
