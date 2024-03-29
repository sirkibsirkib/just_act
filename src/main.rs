use slick::ast::{GroundAtom, Program};
use slick::{
    parse::{self, nom},
    util,
};

fn extract(author: &GroundAtom, program: &mut Program) {
    let hash = fxhash::hash64(&(author, program));
    println!("hash {hash:?}");
}

fn main() {
    let source = util::stdin_to_string();
    let maybe_program = parse::ended(parse::program)(&source);
    let mut program = match maybe_program {
        Err(nom::Err::Error(e)) => {
            return println!("{}", nom::error::convert_error(source.as_str(), e.clone()));
        }
        Err(e) => return println!("PARSE ERROR {e:#?}"),
        Ok((rest, program)) => {
            println!("UNPARSED SUFFIX: {rest:?}");
            program
        }
    };
    println!("PROGRAM: {:#?}", program);
    program.preprocess();
}
