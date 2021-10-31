use qdpx::Qdpx;
use qu::ick_use::*;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    filename: String,
}

#[qu::ick]
fn main(opt: Opt) -> Result {
    let file = Qdpx::open(&opt.filename)?;
    Ok(())
}
