mod shared;
mod sse42;
mod avx2;




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn avx_and_sse_equal() {
        let (a, b, c, d) = sse42::helper(1.0,0.0,-1.0,0.5);        
        println!("{},{},{},{}", a, b, c, d);
        let (a2,b2,c2,d2,e2,f2,g2,h2) = avx2::helper(1.0,0.0,-1.0,0.5,1.0,0.0,-1.0,0.5);
        assert_eq!(a,a2);
        assert_eq!(b,b2);
        assert_eq!(c,c2);
        assert_eq!(d,d2);
    }
}
