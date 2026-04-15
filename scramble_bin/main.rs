use clap::Parser;
use scramble_lib::Seq;

const NOTATION_HELP: &str = r#"Notation:
Uses the standard WCA notation
Faces:
    R - Right face
    L - Left face
    U - Up/top face
    D - Down/bottom face
    F - Front face
    B - Back face
Modifiers:
    '    - Rotate counter-clockwise
    2    - Rotate twice (180°)
    XFw  - Rotate X layers of F side
    Fw   - Rotate 2 layers of F side
    None - Rotate clockwise once (90°)
"#;

#[derive(Parser)]
#[command(after_help = NOTATION_HELP)]
struct Cli {
    #[arg(short = 'l', long = "length", 
        help = "Generate scramble with <LEN> moves", default_value_t = 20, value_name = "LENGTH")]
    len: usize,

    #[arg(short = 's', long = "size",
        help = "Generate a scramble for X size cube, for example if value is 5 the scramble will be for a 5x5 cube", 
        default_value_t = 3, value_name = "SIZE")]
    size: i32,

    #[arg(short = 'n', long = "num", 
        help = "Generate x number of scrambles", default_value_t = 1, value_name = "NUM")]
    num: i32,
}

fn main() {
    let cli = Cli::parse();
    for _ in 0..cli.num {
        let sq = Seq::generate(cli.len, cli.size);
        println!("{}", sq);
    }
}
