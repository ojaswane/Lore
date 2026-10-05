// take the chunks and then embed them for better Ai search
// we are using candle for this.
use anyhow::Result;

use candle_core::{DType, Device, Tensor};
use candle_transformers::models::bert::{BertModel, Config};
use tokenizers::Tokenizer;

// for making a sentance embedding.
fn mean_pooling(embeddings: &Tensor, attention_mask: &Tensor) -> Result<Tensor> {
    let mask = attention_mask.unsqueeze(2)?.to_dtype(embeddings.dtype())?;
    let summed_embeddings = embeddings.broadcast_mul(&mask)?.sum(1)?;
    let token_counts = mask.sum(1)?;

    Ok(summed_embeddings.broadcast_div(&token_counts)?)
}

fn normalization(embeddings: &Tensor) -> Result<Tensor> {
    let norms = embeddings
        .sqr()?
        .sum(1)?
        .sqrt()?
        .unsqueeze(1)?
        .clamp(1e-12f64, f64::MAX)?;

    Ok(embeddings.broadcast_div(&norms)?)
}

pub fn embed_text(chunks: &[String]) {
    let device = Device::Cpu;

    // local model path
    let model_dir = "core/models/all-MiniLM-L6-v2";
    let config_path = format!("{}/config.json", model_dir);
    let tokenizer_path = format!("{}/tokenizer.json", model_dir);
    let weights_path = format!("{}/model.safetensors", model_dir);

    // Load tokenizer
    let tokenizer = Tokenizer::from_file(&tokenizer_path)
        .map_err(|e| anyhow::anyhow!("Failed to load tokenizer: {:?}", e))
        .unwrap();

    // Load model config
    let config: Config = serde_json::from_reader(std::fs::File::open(config_path)?)?;

    // load the weights from safetensors
    let vb = candle_nn::VarBuilder::from_mmaped_safetensors(&[weights_path], DType::F32, &device)?;

    let mini_llm = mini_llm::new(&config, &vb, device)?;

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

    // To check the output
    println!("Transformer output shape: {:?}", token_embeddings.dims());

    //mean pooling to get sentence embeddings
    let sentence_embeddings = mean_pooling(&token_embeddings, &attention_mask)?;
    println!(
        "Sentence embeddings shape: {:?}",
        sentence_embeddings.dims()
    );

    // normalize the embeddings
    let normalized_sentence_embeddings = normalization(&sentence_embeddings)?;
    println!(
        "Normalized sentence embeddings shape: {:?}",
        normalized_sentence_embeddings.dims()
    );

    Ok(());
}
