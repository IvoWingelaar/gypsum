extern crate gypsum;

use gypsum::App;

fn main() {
    let app = App::new(4 * 400, 4 * 220);

    app.run();
}
