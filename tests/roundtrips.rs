extern crate graphql_parser;
#[cfg(test)] #[macro_use] extern crate pretty_assertions;

use std::io::Read;
use std::fs::File;

use graphql_parser::parse_query;

fn roundtrip(filename: &str) {
    let mut buf = String::with_capacity(1024);
    let path = format!("tests/samples/{}.graphql", filename);
    let mut f = File::open(&path).unwrap();
    f.read_to_string(&mut buf).unwrap();
    let ast = parse_query(&buf).unwrap();
    assert_eq!(ast.to_string(), buf);
}

#[test] fn minimal() { roundtrip("minimal"); }
#[test] fn minimal_query() { roundtrip("minimal_query"); }
#[test] fn named_query() { roundtrip("named_query"); }
#[test] fn query_vars() { roundtrip("query_vars"); }
#[test] fn query_var_defaults() { roundtrip("query_var_defaults"); }
#[test] fn query_aliases() { roundtrip("query_aliases"); }
#[test] fn query_arguments() { roundtrip("query_arguments"); }
#[test] fn query_list_arg() { roundtrip("query_list_argument"); }
#[test] fn nested_selection() { roundtrip("nested_selection"); }
#[test] fn inline_fragment() { roundtrip("inline_fragment"); }
#[test] fn inline_fragment_dir() { roundtrip("inline_fragment_dir"); }
#[test] fn fragment_spread() { roundtrip("fragment_spread"); }
#[test] fn minimal_mutation() { roundtrip("minimal_mutation"); }
#[test] fn fragment() { roundtrip("fragment"); }
// this one doesnt' work yet, we tackle all the features one by one
//#[test] fn kitchen_sink() { roundtrip("kitchen-sink"); }