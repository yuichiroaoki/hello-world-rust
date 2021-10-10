use anyhow::{Context, Result};


fn main() -> Result<()> {
    let xs = vec![1, 2, 3];
    println!("The list is: {:?}", xs);

    let path = "test.txt";
    let content =
        std::fs::read_to_string(path).with_context(|| format!("could not read file `{}`", path))?;
    println!("file content: {}", content);
    Ok(())
}
