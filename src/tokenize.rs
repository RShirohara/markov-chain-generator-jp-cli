use lindera_core::mode::Mode;
use lindera_dictionary::{DictionaryConfig, DictionaryKind};
use lindera_tokenizer::tokenizer::{Tokenizer, TokenizerConfig};
use regex::Regex;

pub fn tokenize(source: &str) -> Vec<String> {
    // Setup tokenizer
    let dictionary = DictionaryConfig {
        kind: Some(DictionaryKind::IPADICNEologd),
        path: None,
    };
    let config = TokenizerConfig {
        dictionary,
        user_dictionary: None,
        mode: Mode::Normal,
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
