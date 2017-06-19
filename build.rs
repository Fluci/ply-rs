extern crate peg;
extern crate skeptic;

fn main() {
    peg::cargo_build("src/ply_grammar.rustpeg");
    skeptic::generate_doc_tests(&["README.md"]);
}
