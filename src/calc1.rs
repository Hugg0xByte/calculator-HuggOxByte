// func para somar, por padrão o rust ja faz o return da ultima linha
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

pub fn sub(a: u32, b: u32) -> u32 {
    if a < b { 0 } else { a - b }
}
