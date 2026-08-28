// this is to split the output into smaller chunks for the embeddings. The chunks will be stored in a separate table in the database.

pub fn chunk_output(output: &str) -> Vec<String> {
    output
        .lines()
        .collect::<Vec<_>>()
        .chunks(40)
        .map(|chunk| chunk.join("\n"))
        .filter(|chunk| !chunk.trim().is_empty())
        .collect()
}

// pub fn store_chunks(_chunks: &Vec<String>) {
//     // Tokanize the chunks
// }
