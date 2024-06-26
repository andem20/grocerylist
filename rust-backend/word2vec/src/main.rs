use std::{
    collections::HashMap,
    io::{Read, Write},
    time::Instant,
    vec,
};

// Number of words (4 bytes), Embedding length (4 bytes)
// word_length (1 byte), word,

pub struct Word2Vec {
    embeddings: HashMap<String, Vec<f32>>,
}

impl Word2Vec {
    pub fn new(file_path: &str) -> Self {
        let embeddings: HashMap<String, Vec<f32>> = load_w2v(file_path);
        Self { embeddings }
    }

    pub fn distance(&self, a: &str, b: &str) -> f32 {
        let dist_a = self.embeddings.get(a).unwrap();
        let dist_b = self.embeddings.get(b).unwrap();

        return distance(dist_a, dist_b);
    }

    pub fn size(&self) -> usize {
        self.embeddings.keys().len()
    }
}

fn distance(vec_a: &Vec<f32>, vec_b: &Vec<f32>) -> f32 {
    return vec_a
        .iter()
        .zip(vec_b.iter())
        .map(|(&a_val, &b_val)| (a_val - b_val) * (a_val - b_val))
        .fold(0.0, std::ops::Add::add);
}

fn main() {
    let start = Instant::now();
    let w2v = Word2Vec::new("./resources/glove-wiki-gigaword-300.w2v");

    println!(
        "Loaded {} words in {:.3}s",
        w2v.size(),
        start.elapsed().as_secs_f32()
    );

    println!("Dist: {}", w2v.distance("cheese", "dairy"));
}

fn load_w2v(file_name: &str) -> HashMap<String, Vec<f32>> {
    let mut file = std::fs::File::open(file_name).unwrap();
    let mut buffer: Vec<u8> = vec![];
    let _ = file.read_to_end(&mut buffer);

    let (num_words, embedding_size, cursor) = read_headers(&buffer);
    let (words, cursor) = read_words(&buffer, num_words, cursor);
    let (embeddings, _) = read_embeddings(&buffer, num_words, embedding_size, cursor);

    return words.into_iter().zip(embeddings).collect();
}

fn read_embeddings(
    buffer: &[u8],
    num_words: usize,
    embedding_size: usize,
    mut cursor: usize,
) -> (Vec<Vec<f32>>, usize) {
    let mut embeddings: Vec<Vec<f32>> = vec![];

    for _ in 0..num_words {
        let size = std::mem::size_of::<f32>();
        let end = cursor + embedding_size as usize * size;
        let embedding: Vec<f32> = buffer[cursor..end]
            .chunks_exact(size)
            .map(|chunk| f32::from_le_bytes(chunk.try_into().unwrap()))
            .collect();

        embeddings.push(embedding);
        cursor = end;
    }

    return (embeddings, cursor);
}

fn read_words(buffer: &[u8], num_words: usize, mut cursor: usize) -> (Vec<String>, usize) {
    let mut words = vec![];
    for _ in 0..num_words {
        let word_length = buffer[cursor];
        let start = cursor + 1;
        let end = start + word_length as usize;
        let word = std::str::from_utf8(&buffer[start..end])
            .map(|s| s.to_string())
            .unwrap();
        words.push(word);
        cursor = end;
    }

    return (words, cursor);
}

fn read_headers(buffer: &[u8]) -> (usize, usize, usize) {
    let num_words = u32::from_le_bytes(buffer[0..4].try_into().unwrap());
    let embedding_size = u32::from_le_bytes(buffer[4..8].try_into().unwrap());
    return (num_words as usize, embedding_size as usize, 8);
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
