pub struct CliArgs {
    pub number_of_dice: usize,
    pub max: usize,
    pub min: usize,
}

pub fn input() -> CliArgs {
    return CliArgs {
        number_of_dice: 3,
        max: 3,
        min: 0,
    };
}

pub fn output(args: CliArgs, result: usize) {
    let dice = args.number_of_dice.to_string() + "d" + &args.max.to_string();
    let result_as_string = &result.to_string();

    let statement = dice + " -> " + result_as_string;
    println!("{}", statement);
}
