use regice::evaluator::*;
use regice::tokenizer;

fn main() {
    Code::eval(Code::new(tokenizer::parse(
        std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap(),
    )));
}
