fn main() -> Result<(), &'static str> {
    let s = vec!["apple", "mango", "banana"];
    let _fourth = s.get(4).ok_or("I got only 3 fruits")?;
    Ok(())
}