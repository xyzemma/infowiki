use pest::Parser;
use pest_derive;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct wtParse;