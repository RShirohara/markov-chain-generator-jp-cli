use markov::Chain;

pub fn build_chain(source: &Vec<String>, order: &usize) -> Chain<String> {
    let mut chain = Chain::of_order(*order);
    for s in source {
        chain.feed_str(s);
    }
    return chain;
}
