use std::path::Path;

mod hiragana;

fn main() {
    hiragana::jp_a1_1_course().build(Path::new("japanese")).unwrap();
}
