#![feature(plugin)]
#![plugin(peg_syntax_ext)]

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Chunk<'input> {
    Mark(Marker),
    Text(&'input str),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Marker {
    V { num: u32 }
}

peg_file! grammar("usfm.rustpeg");

#[test]
fn v_marker() {
    let input = "xyz\\v 17 abc";
    let parsed = grammar::file(&input);
    let expected = vec![
        Chunk::Text("xyz"),
        Chunk::Mark(Marker::V { num: 17 }),
        Chunk::Text("abc")
    ];
    assert_eq!(parsed, Ok(expected));
}
