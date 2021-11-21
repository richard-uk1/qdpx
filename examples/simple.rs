use qdpx::{ProjectVisitor, Qdpx, Traversal};
use qu::ick_use::*;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    filename: String,
}

#[qu::ick]
fn main(opt: Opt) -> Result {
    let qdpx = Qdpx::load_into_memory(&opt.filename)?;
    //println!("{}", serde_json::to_string_pretty(qdpx.project()).unwrap());
    qdpx.project().visit_code(|code| {
        println!("{}", code.name);
        Traversal::Continue
    });
    Ok(())
}
