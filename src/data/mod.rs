pub fn load_words() -> Vec<[char; 5]> {
    let words = include_str!("words.txt");
    words
        .trim()
        .split('\n')
        .map(|word| {
            std::convert::TryInto::<[char; 5]>::try_into(word.chars().collect::<Vec<_>>())
                .expect("Word is not 5 characters long")
        })
        .collect()
}
