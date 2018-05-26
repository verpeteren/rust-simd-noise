mod shared;
mod sse42;
mod avx2;




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first_test() {
        let (a, b, c, d) = sse42::helper();        
        println!("{},{},{},{}", a, b, c, d);
    }
}
