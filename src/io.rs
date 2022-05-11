use clap::{Arg, Command};

pub struct CliArgs {
    pub number_of_dice: usize,
    pub max: usize,
    pub min: usize,
}

pub fn input() -> CliArgs {
    let matches = self::create_command().get_matches();
    let arg1 = matches.value_of("number").unwrap();
    let number_of_dice = arg1.parse::<usize>().unwrap();
    let arg2 = matches.value_of("max").unwrap();
    let max = arg2.parse::<usize>().unwrap();

    return CliArgs {
        number_of_dice: number_of_dice,
        max: max,
        min: 1,
    };
}

pub fn output(args: CliArgs, result: usize) {
    let dice = args.number_of_dice.to_string() + "d" + &args.max.to_string();
    let result_as_string = &result.to_string();

    let statement = dice + " -> " + result_as_string;
    println!("{}", statement);
}

// FIXME: 2d100みたいな感じで受け取ってパースしたい
fn create_command() -> Command<'static> {
    return Command::new("trpgd")
        .version("1.0")
        .author("orifuji")
        .about("Does awesome things")
        .arg(
            Arg::new("number")
                .required(true)
                .help("set your number of dice to roll")
                .index(1),
        )
        .arg(
            Arg::new("max")
                .required(true)
                .help("set max value of your dice")
                .index(2),
        );
}
