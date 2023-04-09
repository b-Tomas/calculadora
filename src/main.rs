mod math;
mod structs;
mod exp_interpreter;
mod app;

use app::App;

fn main() {
    let mut app = App::new();

    let exit = app.start();
    if let Err(_) = exit {
        print!("Ha ocurrido un error");
    };
}
