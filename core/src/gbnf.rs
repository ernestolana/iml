use schemars::schema::RootSchema;

pub fn schema_to_gbnf(_schema: &RootSchema) -> String {
    // Phase 1: We provide a robust statically-aligned GBNF grammar
    // matching the exported JSON schema for the Arena.
    // A future implementation could walk the `RootSchema` ast recursively.
    
    let grammar = r#"root ::= arena
arena ::= "{\"nodes\":[" node_list "]}"
node_list ::= (node ("," node)*)?
node ::= "{\"t\":" node_type ",\"c\":[" int_list "],\"r\":" string ",\"o\":" string "}"
node_type ::= "{\"N\":" int "}" | "\"A\"" | "\"M\"" | "\"L\"" | "\"D\"" | "{\"V\":" string "}" | "{\"B\":[[" float_list "]," int "," int "]}" | "{\"Q\":[" float_list "]}"
int_list ::= ( int ( "," int )* )?
float_list ::= ( float ( "," float )* )?
int ::= [0-9]+
float ::= [0-9]+ "." [0-9]+
string ::= "\"" [a-zA-Z0-9_ ]* "\""
"#;
    grammar.to_string()
}
