use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Encode and decode messages into a png file, when passing the CHUKNTYPE argument 
/// it must be a string with 4 characters (no numbers) and the 3rd letter MUST be capitalized
pub enum Args {
    /// Encode a message into a .png file
    Encode {
        #[structopt(parse(from_os_str), name = "FILE", help = ".png file")]
        file: PathBuf,

        #[structopt(name = "CHUNKTYPE", help = "4 character sequence, 3rd char MUST be uppercase e.g. ruSt")]
        ctype: String,

        #[structopt(name = "MESSAGE", help = "A UTF-8 string to encode into a png chun")]
        message: String,

        #[structopt(parse(from_os_str), name = "OUTPUT", help = "An option output file, otherwise the original will be overwritten")]
        output: Option<PathBuf>
    },

    /// Decode a message into a .png file
    Decode {
        #[structopt(parse(from_os_str), name = "FILE", help = ".png file")]
        file: PathBuf,

        #[structopt(name = "CHUNKTYPE", help = "4 character sequence, 3rd char MUST be uppercase e.g. ruSt")]
        ctype: String,

        #[structopt(short, long, help = "print the messages of all chunks with CHUNKTYPE")]
        all: bool,
    },

    /// Remove a message from a .png file through its chunk-type
    Remove {
        #[structopt(parse(from_os_str), name = "FILE", help = ".png file")]
        file: PathBuf,

        #[structopt(name = "CHUNKTYPE", help = "4 character sequence, 3rd char MUST be uppercase e.g. ruSt")]
        ctype: String,

        #[structopt(short, long, help = "print the messages of all chunks with CHUNKTYPE")]
        all: bool,
    },

    /// Print all valid UTF-8 strings contained in chunk data
    Print {
        #[structopt(parse(from_os_str), name = "FILE", help = ".png file")]
        file: PathBuf,
    },
}
