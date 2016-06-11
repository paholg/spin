extern crate sdl2;

mod phi;
mod views;

fn main() {
    ::phi::spawn("Spinny", |_| {
        Box::new(::views::ShipView)
    });
}
