use super::receipt;

use std::path::PathBuf;

use clap::Parser;

// Make a top-level parser that either computes the receipt or prints the schema
#[derive(Parser)]
#[clap(version = "1.0", author = "Daniel Winkelman")]
pub enum Args {
    Compute(ReceiptArgs),
    Schema(SchemaArgs),
}

// Create a main method for each subcommand
impl Args {
    pub fn main(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Args::Compute(args) => {
                let input_path = args.get_input_path();
                let output_path = args.get_output_path();
                receipt::process_receipt(input_path, output_path)
            }
            Args::Schema(args) => {
                let output_path = args.get_output_path();
                receipt::print_schema(output_path)
            }
        }
    }
}

#[derive(Parser)]
pub struct ReceiptArgs {
    #[clap(long)]
    input_path: PathBuf,

    #[clap(long)]
    output_path: Option<PathBuf>,
}

impl ReceiptArgs {
    pub fn get_input_path(&self) -> PathBuf {
        self.input_path.clone()
    }

    pub fn get_output_path(&self) -> PathBuf {
        // if there is no output path, append _output to the input path
        // for example, my_receipt.json would become my_receipt_output.json
        self.output_path.clone().unwrap_or_else(|| {
            self.input_path.with_file_name({
                let file_name = self.input_path.file_stem().unwrap().to_os_string();
                Into::<std::ffi::OsString>::into(format!(
                    "{}_output.json",
                    file_name.to_string_lossy()
                ))
            })
        })
    }
}

#[derive(Parser)]
pub struct SchemaArgs {
    #[clap(long)]
    output_path: Option<PathBuf>,
}

impl SchemaArgs {
    pub fn get_output_path(&self) -> PathBuf {
        // if there is no output path, get the path of this source file using file!(), then place the schema in the same directory
        self.output_path.clone().unwrap_or_else(|| {
            let mut path = PathBuf::from(file!());
            path.set_file_name("receipt_schema.json");
            path
        })
    }
}
