pub const MAX_CHARS: usize = 140;

pub fn check_text_length(text: &str) -> bool {
    text.chars().count() <= MAX_CHARS
}

pub fn split_text(text: &str) -> Vec<String> {
    if text.chars().count() <= MAX_CHARS {
        return vec![text.to_string()];
    }

    let mut chunks = Vec::new();
    let mut current_chunk = String::new();
    let mut chars_count = 0;

    let sentences = split_into_sentences(text);

    for sentence in sentences {
        let sentence_len = sentence.chars().count();

        if chars_count + sentence_len <= MAX_CHARS {
            current_chunk.push_str(&sentence);
            chars_count += sentence_len;
        } else {
            if !current_chunk.is_empty() {
                chunks.push(current_chunk.trim().to_string());
                current_chunk = String::new();
                chars_count = 0;
            }

            if sentence_len <= MAX_CHARS {
                current_chunk.push_str(&sentence);
                chars_count = sentence_len;
            } else {
                let sub_chunks = split_long_sentence(&sentence);
                for (i, sub_chunk) in sub_chunks.iter().enumerate() {
                    if i == sub_chunks.len() - 1 {
                        current_chunk.push_str(sub_chunk);
                        chars_count = sub_chunk.chars().count();
                    } else {
                        chunks.push(sub_chunk.trim().to_string());
                    }
                }
            }
        }
    }

    if !current_chunk.trim().is_empty() {
        chunks.push(current_chunk.trim().to_string());
    }

    chunks
}

fn split_into_sentences(text: &str) -> Vec<String> {
    let sentence_endings = ['。', '！', '？', '.', '!', '?'];
    let mut sentences = Vec::new();
    let mut current_sentence = String::new();

    for ch in text.chars() {
        current_sentence.push(ch);

        if sentence_endings.contains(&ch) {
            sentences.push(current_sentence.clone());
            current_sentence.clear();
        }
    }

    if !current_sentence.trim().is_empty() {
        sentences.push(current_sentence);
    }

    sentences
}

fn split_long_sentence(sentence: &str) -> Vec<String> {
    let break_points = ['、', '，', ',', ' ', '　'];
    let mut chunks = Vec::new();
    let mut current_chunk = String::new();
    let mut chars_count = 0;

    for ch in sentence.chars() {
        current_chunk.push(ch);
        chars_count += 1;

        if chars_count >= MAX_CHARS {
            if break_points.contains(&ch) {
                chunks.push(current_chunk.clone());
                current_chunk.clear();
                chars_count = 0;
            } else {
                let last_break = find_last_break_point(&current_chunk, &break_points);
                if let Some(char_pos) = last_break {
                    let chars: Vec<char> = current_chunk.chars().collect();
                    let first_part: String = chars[..=char_pos].iter().collect();
                    let second_part: String = chars[char_pos + 1..].iter().collect();
                    chunks.push(first_part);
                    current_chunk = second_part;
                    chars_count = current_chunk.chars().count();
                } else {
                    chunks.push(current_chunk.clone());
                    current_chunk.clear();
                    chars_count = 0;
                }
            }
        }
    }

    if !current_chunk.trim().is_empty() {
        chunks.push(current_chunk);
    }

    chunks
}

fn find_last_break_point(text: &str, break_points: &[char]) -> Option<usize> {
    let chars: Vec<char> = text.chars().collect();
    chars
        .iter()
        .enumerate()
        .rev()
        .find(|(_, ch)| break_points.contains(ch))
        .map(|(i, _)| i)
}
