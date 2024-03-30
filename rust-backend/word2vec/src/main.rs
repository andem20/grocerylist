use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    str::Utf8Error,
    time::Instant,
};

// Number of words (4 bytes), Embedding length (4 bytes)
// word_length (1 byte), word,

fn main() {
    let start = Instant::now();
    let embeddings_map: HashMap<String, Vec<f32>> =
        load_w2v("./resources/glove-wiki-gigaword-300.w2v");
    println!(
        "Loaded {} words in {:.3}s",
        &embeddings_map.keys().len(),
        start.elapsed().as_secs_f32()
    );

    let dist_a = embeddings_map.get("cheese").unwrap();
    let dist_b = embeddings_map.get("dairy").unwrap();

    let dist = distance(dist_a, dist_b);
    println!("Dist: {}", dist);
}

fn distance(vec_a: &Vec<f32>, vec_b: &Vec<f32>) -> f32 {
    return vec_a
        .iter()
        .zip(vec_b.iter())
        .map(|(&a_val, &b_val)| (a_val - b_val) * (a_val - b_val))
        .fold(0.0, std::ops::Add::add);
}

fn load_w2v(file_name: &str) -> HashMap<String, Vec<f32>> {
    let file_read = std::fs::File::open(file_name).unwrap();

    let (num_words, embedding_size) = read_headers(&file_read).unwrap();

    let mut words = vec![];

    for _i in 0..num_words {
        let word = read_word(&file_read).unwrap();
        words.push(word);
    }

    let mut embeddings: Vec<Vec<f32>> = vec![];

    for _i in 0..num_words {
        let embedding = read_embedding(&file_read, embedding_size as usize);
        embeddings.push(embedding.try_into().unwrap());
    }

    return words.into_iter().zip(embeddings).collect();
}

fn read_headers(mut file: &File) -> Result<(u32, u32), std::io::Error> {
    let mut buffer = [0; 8];

    let _ = file.read_exact(&mut buffer);

    let num_words = u32::from_le_bytes(buffer[0..4].try_into().unwrap());
    let embedding_size = u32::from_le_bytes(buffer[4..].try_into().unwrap());

    Ok((num_words, embedding_size))
}

fn read_word(mut file: &File) -> Result<String, Utf8Error> {
    let mut word_length = [0; 1];
    let _ = file.read_exact(&mut word_length);

    let mut buffer = vec![0; word_length[0] as usize].into_boxed_slice();
    let _ = file.read_exact(&mut buffer);

    return std::str::from_utf8(&buffer).map(|s| s.to_string());
}

fn read_embedding(mut file: &File, embedding_length: usize) -> Vec<f32> {
    let mut buffer = vec![0; embedding_length * 4].into_boxed_slice();
    let _ = file.read_exact(&mut buffer);

    buffer
        .chunks(4)
        .map(|chunk| f32::from_le_bytes(chunk.try_into().unwrap()))
        .collect()
}

#[allow(dead_code)]
fn save_data_as_binary<const T: usize>(words: Vec<String>, embeddings: Vec<[f32; T]>) {
    let embedding_len: u32 = T as u32;
    let embeddings: Vec<f32> = embeddings.into_iter().flatten().collect();

    let file_name = "./resources/glove-wiki-gigaword-300.w2v";

    unsafe {
        let embedding_bytes =
            std::slice::from_raw_parts(embeddings.as_ptr() as *const u8, embeddings.len() * 4);

        let mut file_write = std::fs::File::create(file_name).unwrap();
        // Headers
        let _ = file_write.write(&(words.len() as u32).to_le_bytes());
        let _ = file_write.write(&embedding_len.to_le_bytes());

        words.iter().for_each(|word| {
            let _ = file_write.write(&(word.len() as u8).to_le_bytes()); // Word length
            let _ = file_write.write(&word.as_bytes()); // Word
        });

        let _ = file_write.write(&embedding_bytes); // Embedding
    }
}

#[allow(dead_code)]
fn load_glove_file(file_path: &str) -> (Vec<String>, Vec<Vec<f32>>) {
    let mut embeddings: Vec<Vec<f32>> = Vec::new();
    let mut keys: Vec<String> = Vec::new();

    std::fs::read_to_string(file_path)
        .expect("Failed loading file")
        .lines()
        .skip(1)
        .for_each(|str| {
            let strings = str.split(" ").map(String::from).collect::<Vec<String>>();
            if let Some(key) = strings.first() {
                let array = strings
                    .iter()
                    .skip(1)
                    .map(|s| {
                        s.parse::<f32>()
                            .expect(format!("Cannot parse string: {s} to float 32").as_str())
                    })
                    .collect();

                keys.push(key.to_owned());
                embeddings.push(array);
            }
        });

    return (keys, embeddings);
}
