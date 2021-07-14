
//===================================================================================================================//

pub struct Config {
    pub input_filename: String,
    pub output_filename: String,
}

//===================================================================================================================//

// get first arg from command line
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        let output_filename;

        if args.len() > 3 {
            return Err("\n not too many arguments \n correct notation: program INPUT OUTPUT \n (by default OUTPUT = output.csv)");
        }
        if args.len() < 2 {
            return Err("\n not enough arguments \n correct notation: program INPUT OUTPUT \n (by default OUTPUT = output.csv)");
        }
        if args.len() == 2 {
            output_filename = String::from("output.csv");
        } else {
            output_filename = args[2].clone();
        }

        let input_filename = args[1].clone();

        Ok(Config {
            input_filename,
            output_filename,
        })
    }
}

//===================================================================================================================//