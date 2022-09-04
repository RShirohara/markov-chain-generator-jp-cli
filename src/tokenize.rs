use lindera::formatter::format;
use lindera::formatter::Format;
use lindera::mode::Mode;
use lindera::tokenizer::{DictionaryConfig, DictionaryKind, Tokenizer, TokenizerConfig};
use regex::Regex;

pub fn tokenize(source: &str) -> Vec<String> {
    // Setup tokenizer
    let dictionary = DictionaryConfig {
        kind: DictionaryKind::IPADIC,
        path: None,
    };
    let config = TokenizerConfig {
        dictionary,
        user_dictionary: None,
        mode: Mode::Normal,
    };
    let tokenizer = Tokenizer::with_config(config).unwrap();

    // Format text before tokenize
    let cr = Regex::new("\r").unwrap();
    let duplicate_linebreaks = Regex::new("\n+").unwrap();

    let cr_removed_source = cr.replace_all(source, "\n");
    let dup_linebreaks_removed_source = duplicate_linebreaks.replace_all(&cr_removed_source, "\n");

    let formatted_source = dup_linebreaks_removed_source.to_string();

    // Tokenize
    let tokens: Vec<String> = formatted_source
        .lines()
        .map(|t| tokenizer.tokenize(t).unwrap())
        .map(|t| format(t, Format::Wakati).unwrap())
        .collect();

    return tokens;
}
