use crate::maps::test::create_test_map;

pub mod map;
mod painter;
mod timer;

pub fn test() {
    use painter::Painter;

    Painter::push_backdrop();

    let painter = Painter::new();
    painter.push();

    let map = create_test_map();
}
