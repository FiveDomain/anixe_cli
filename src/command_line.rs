use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
pub struct Config {

    /// Name input file with extension csv to convert
    #[structopt()]
    pub input_filename: String,
    /// Name output file with extension csv where this data are save
    #[structopt()]
    pub output_filename: Option<String>,
}
