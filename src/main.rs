mod db;
mod oss;
mod spider;
fn main() {
    let kind = spider::NovelCategory::from("玄幻");
    println!("{}", kind.to_string());
}
