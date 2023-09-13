use std::error::Error;
use std::fmt::Display;

struct Pie;

#[derive(Debug)]
struct NotFreshError;

impl Display for NotFreshError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "This pie is not fresh!")
    }
}

impl Error for NotFreshError {}

impl Pie {
    fn eat(&self) -> Result<(), Box<dyn Error>> {
        Err(Box::new(NotFreshError))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let heap_pie = Box::new(Pie);
    heap_pie.eat()?;
    Ok(())
}
