fn main() {
    let json = r#"
    {
      "nodes": [
        { "t": { "Q": [ 0.707, 0, 0, 0.707 ] }, "c": [], "r": "Bell state" },
        { "t": { "B": [ [ 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1 ], 4, 4 ] }, "c": [], "r": "Identity transform" },
        { "t": { "M": null }, "c": [ 0, 1 ], "r": "Apply transformation" },
        { "t": { "D": null }, "c": [ 2 ], "r": "Deallocate state" }
      ]
    }
    "#;
    let arena: Result<core::Arena, _> = serde_json::from_str(json);
    println!("{:?}", arena);
}
