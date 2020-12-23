// mod_within.rs
mod food {
    pub struct Cake;
    struct Smoothie;
    struct Pizza;
}

// cake is private
use food::Cake;

fn main() {
    let eatbale = Cake;
}
