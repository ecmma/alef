use alef_parser::{source::MemoryBuffer, lex::scan::Scanner};
use anyhow;
use std::fs;
use std::path::PathBuf;

fn lex_file(path: PathBuf) -> anyhow::Result<()> {
        let mbuf = MemoryBuffer::from_file(path.to_string_lossy().into())?;
        let mut lex = Scanner::new(Box::new(mbuf), None);
        let mut tok = lex.tok();
        
        while !tok.is_end() {
//            println!("{}", tok);
            tok = lex.tok();
        }

        Ok(())
}

#[test]
fn test_lexer() -> anyhow::Result<()> {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.push("tests/p9_srcs");
    println!("{}", dir.display());
    let paths = fs::read_dir(dir)?;

    for path in paths {
        let path = path?.path();
        println!("{}", path.display());
        assert!(lex_file(path.clone()).is_ok(), "error lexing file {}", path.display());
    }

    Ok(())
}
