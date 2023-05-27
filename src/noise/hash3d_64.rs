use simdeez::Simd;

pub struct Hash3d<S: Simd> {
    // Masks guiding dimension selection
    pub l8: S::Vf64,
    pub l4: S::Vf64,
    pub h12_or_14: S::Vf64,

    // Signs for the selected dimensions
    pub h1: S::Vf64,
    pub h2: S::Vf64,
}

impl<S> Hash3d<S>
where
    S: Simd,
{
    pub fn new(l8: S::Vf64, l4: S::Vf64, h12_or_14: S::Vf64, h1: S::Vf64, h2: S::Vf64) -> Self {
        Self {
            l8,
            l4,
            h12_or_14,
            h1,
            h2,
        }
    }
}
/// Compute hash values used by `grad3d` and `grad3d_dot`

#[inline(always)]
pub unsafe fn hash3d<S: Simd>(seed: i64, i: S::Vi64, j: S::Vi64, k: S::Vi64) -> Hash3d<S> {
    // This 64 bit variant is not implemented.
    // The codeblock below is just the 64 bit SIMD instructions with the 32 bit magic numbers.
    // I don't know what values the Hash3d fields should hold or what magic number are needed for the bit shifts.
    unimplemented!();
    let mut hash = S::xor_epi64(i, S::set1_epi64(seed));
    hash = S::xor_epi64(j, hash);
    hash = S::xor_epi64(k, hash);
    hash = S::mullo_epi64(
        S::mullo_epi64(S::mullo_epi64(hash, hash), S::set1_epi64(60493)),
        hash,
    );
    hash = S::xor_epi64(S::srai_epi64(hash, 13), hash);
    let hasha13 = S::and_epi64(hash, S::set1_epi64(13));
    Hash3d::new(
        S::castepi64_pd(S::cmplt_epi64(hasha13, S::set1_epi64(8))),
        S::castepi64_pd(S::cmplt_epi64(hasha13, S::set1_epi64(2))),
        S::castepi64_pd(S::cmpeq_epi64(S::set1_epi64(12), hasha13)),
        S::castepi64_pd(S::slli_epi64(hash, 31)),
        S::castepi64_pd(S::slli_epi64(S::and_epi64(hash, S::set1_epi64(2)), 30)),
    )
}
