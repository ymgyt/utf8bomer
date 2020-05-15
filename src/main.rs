use std::{
    env, fs,
    io::{Cursor, Seek, SeekFrom, Write},
    path::Path,
};
use unicode_bom::Bom;

static UTF8_BOM: [u8; 3] = [0xEF, 0xBB, 0xBF];

fn main() -> anyhow::Result<()> {
    let f = env::args().nth(1).expect("file path required");
    match f.parse()? {
        Bom::Null => {
            push_front_utf8_bom(f)?;
            println!("append bom");
        }
        Bom::Utf8 => {
            println!("UTF8 BOM detected");
        }
        etc => {
            println!("Another BOM detected {:?}", etc);
        }
    }
    Ok(())
}

fn push_front_utf8_bom<P: AsRef<Path>>(p: P) -> anyhow::Result<()> {
    let mut f = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(p.as_ref())?;
    let mut buff = Cursor::new(Vec::new());
    buff.write(&UTF8_BOM)?;
    std::io::copy(&mut f, &mut buff)?;

    f.seek(SeekFrom::Start(0))?;
    f.write_all(&buff.get_ref()[..])?;

    Ok(())
}
