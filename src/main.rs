mod dice;
mod io;

use crate::io as cli_io;

fn main() {
    let inputs = cli_io::input();
    let result = dice::roll(inputs.number_of_dice, inputs.max, inputs.min);
    cli_io::output(inputs, result);
}
