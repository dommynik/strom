#[macro_use]
extern crate strom;

#[derive(Debug)]
enum FnType {
    Function,
    Sub,
}

function!(ws_space -> &str,
          tag!(" "));
function!(ws_tab -> &str,
          tag!("\t"));
function!(ws_nl -> &str,
          alt!(tag!("_\n") | tag!("_\r") | tag!("_\r\n")));
function!(single_ws -> &str,
          alt!(ws_space | ws_tab | ws_nl));

function!(ws -> (),
          value!(many1!(call!(single_ws)) => ()));

function!(k_function -> FnType,
          value!(tag!(cl "Function") => FnType::Function));
function!(k_sub -> FnType,
          value!(tag!(cl "Sub") => FnType::Sub));
function!(fn_type -> FnType,
          followed_by!(alt!(k_function | k_sub) + call!(ws)));

function!(identifier -> String,
          to_string!(tag!(cl "Sub")));

fn main() {
    let input = strom::Input::new(0, "Sub HelloWorld");

    println!("{:?}", fn_type(input));
    println!("{:?}", identifier(input));
}
