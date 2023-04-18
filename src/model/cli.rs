#[derive(FromArgs)]
#[argh(description = "A program to convert json to json schema")]
pub struct CliArgs {
    #[argh(option, short = 'i', description = "input source, if omitted stdin will be read")]
    pub input: Option<String>,
    #[argh(option, short = 'o', description = "output destination, if omitted stdout will be written")]
    pub output: Option<String>,
    #[argh(switch, short = 'v', description = "show default parameters")]
    pub show_default: bool,
}
