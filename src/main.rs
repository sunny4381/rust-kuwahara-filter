mod error;
mod filter;
mod iter;

use std::io::{self, Write};

use clap::Clap;
use image::open;

use error::Error;
use filter::kuwahara;

#[derive(Clap)]
#[clap(version = "1.0", author = "Kevin K. <kbknapp@gmail.com>")]
struct Opts {
    input: String,
    #[clap(short, long, default_value = "a.png")]
    output: String,
    #[clap(short, long, default_value = "7")]
    r: u32,
}

impl Opts {
    pub fn run(&self) -> Result<(), Error> {
        let image = open(&self.input)?;
        let mut rgba_image = image.into_rgba();

        // apply filter
        kuwahara(&mut rgba_image, self.r)?;

        // save image
        rgba_image.save(&self.output)?;
        Ok(())
    }
}

fn main() {
    let opts = Opts::parse();

    match opts.run() {
        Ok(_) => (),
        Err(ref e) => abort(e),
    };
}

pub fn abort(e: &Error) {
    writeln!(&mut io::stderr(), "{}", e).unwrap();
    ::std::process::exit(1)
}
