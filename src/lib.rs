mod shared;
mod sse2;
mod sse41;
mod avx2;




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn avx_sse2_sse41_equal() {
        let (a, b, c, d) = sse41::helper(1.0,0.0,-1.0,0.5);        
        println!("{},{},{},{}", a, b, c, d);
        let (a2,b2,c2,d2,e2,f2,g2,h2) = avx2::helper(1.0,0.0,-1.0,0.5,1.0,0.0,-1.0,0.5);
        let (a3,b3,c3,d3) = sse2::helper(1.0,0.0,-1.0,0.5);
        assert_eq!(a,a2);
        assert_eq!(b,b2);
        assert_eq!(c,c2);
        assert_eq!(d,d2);
        
        assert_eq!(a,a3);
        assert_eq!(b,b3);
        assert_eq!(c,c3);
        assert_eq!(d,d3);
    }
}
