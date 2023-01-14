use lindera::mode::Mode;
use lindera::tokenizer::{DictionaryConfig, Tokenizer, TokenizerConfig};
use lindera::DictionaryKind;
use regex::Regex;

pub fn tokenize(source: &str) -> Vec<String> {
    // Setup tokenizer
    let dictionary = DictionaryConfig {
        kind: Some(DictionaryKind::IPADIC),
        path: None,
    };
    let config = TokenizerConfig {
        dictionary,
        user_dictionary: None,
        mode: Mode::Normal,
        with_details: false,
    };
    let tokenizer = Tokenizer::from_config(config).unwrap();

    // Format text before tokenize
    let cr = Regex::new("\r").unwrap();
    let duplicate_linebreaks = Regex::new("\n+").unwrap();

    let cr_removed_source = cr.replace_all(source, "\n");
    let dup_linebreaks_removed_source = duplicate_linebreaks.replace_all(&cr_removed_source, "\n");

    let formatted_source = dup_linebreaks_removed_source.to_string();

    // Tokenize
    let tokens = formatted_source
        .lines()
        .map(|line| tokenizer.tokenize(line).unwrap())
        .map(|tokens| {
            tokens
                .into_iter()
                .map(|token| token.text.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        })
        .collect();

    tokens
}
