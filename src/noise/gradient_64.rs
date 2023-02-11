use simdeez::*;

/// Generates a random integer gradient in ±7 inclusive, and returns its product with `x`
///
/// This differs from Gustavson's well-known implementation in that gradients can be zero, and the
/// maximum gradient is 7 rather than 8.
#[inline(always)]
pub unsafe fn grad1<S: Simd>(seed: i64, hash: S::Vi64, x: S::Vf64) -> S::Vf64 {
    let h = S::and_epi64(S::xor_epi64(S::set1_epi64(seed), hash), S::set1_epi64(15));
    let v = S::cvtepi64_pd(S::and_epi64(h, S::set1_epi64(7)));

    let h_and_8 = S::castepi64_pd(S::cmpeq_epi64(
        S::setzero_epi64(),
        S::and_epi64(h, S::set1_epi64(8)),
    ));
    let grad = S::blendv_pd(S::sub_pd(S::setzero_pd(), v), v, h_and_8);
    S::mul_pd(grad, x)
}

/// Generates a random gradient vector where one component is ±1 and the other is ±2, and computes
/// the dot product with [x, y].
///
/// This differs from Gustavson's gradients by having a constant magnitude, providing results that
/// are more consistent between directions.
#[inline(always)]
pub unsafe fn grad2<S: Simd>(seed: i64, hash: S::Vi64, x: S::Vf64, y: S::Vf64) -> S::Vf64 {
    let h = S::and_epi64(S::xor_epi64(hash, S::set1_epi64(seed)), S::set1_epi64(7));
    let mask = S::castepi64_pd(S::cmpgt_epi64(S::set1_epi64(4), h));
    let u = S::blendv_pd(y, x, mask);
    let v = S::mul_pd(S::set1_pd(2.0), S::blendv_pd(x, y, mask));

    let h_and_1 = S::castepi64_pd(S::cmpeq_epi64(
        S::setzero_epi64(),
        S::and_epi64(h, S::set1_epi64(1)),
    ));
    let h_and_2 = S::castepi64_pd(S::cmpeq_epi64(
        S::setzero_epi64(),
        S::and_epi64(h, S::set1_epi64(2)),
    ));

    S::add_pd(
        S::blendv_pd(S::sub_pd(S::setzero_pd(), u), u, h_and_1),
        S::blendv_pd(S::sub_pd(S::setzero_pd(), v), v, h_and_2),
    )
}

#[inline(always)]
pub unsafe fn grad3d<S: Simd>(
    seed: i64,
    hash: S::Vi64,
    x: S::Vf64,
    y: S::Vf64,
    z: S::Vf64,
) -> S::Vf64 {
    let h = S::and_epi64(S::xor_epi64(S::set1_epi64(seed), hash), S::set1_epi64(15));

    let mut u = S::castepi64_pd(S::cmpgt_epi64(S::set1_epi64(8), h));
    u = S::blendv_pd(y, x, u);

    let mut v = S::castepi64_pd(S::cmpgt_epi64(S::set1_epi64(4), h));
    let mut h12_or_14 = S::castepi64_pd(S::cmpeq_epi64(
        S::setzero_epi64(),
        S::or_epi64(
            S::cmpeq_epi64(h, S::set1_epi64(12)),
            S::cmpeq_epi64(h, S::set1_epi64(14)),
        ),
    ));
    h12_or_14 = S::blendv_pd(x, z, h12_or_14);
    v = S::blendv_pd(h12_or_14, y, v);

    let h_and_1 = S::castepi64_pd(S::cmpeq_epi64(
        S::setzero_epi64(),
        S::and_epi64(h, S::set1_epi64(1)),
    ));
    let h_and_2 = S::castepi64_pd(S::cmpeq_epi64(
        S::setzero_epi64(),
        S::and_epi64(h, S::set1_epi64(2)),
    ));

    S::add_pd(
        S::blendv_pd(S::sub_pd(S::setzero_pd(), u), u, h_and_1),
        S::blendv_pd(S::sub_pd(S::setzero_pd(), v), v, h_and_2),
    )
}

#[inline(always)]
pub unsafe fn grad4<S: Simd>(
    seed: i64,
    hash: S::Vi64,
    x: S::Vf64,
    y: S::Vf64,
    z: S::Vf64,
    t: S::Vf64,
) -> S::Vf64 {
    let h = S::and_epi64(S::xor_epi64(S::set1_epi64(seed), hash), S::set1_epi64(31));
    let mut mask = S::castepi64_pd(S::cmpgt_epi64(S::set1_epi64(24), h));
    let u = S::blendv_pd(y, x, mask);
    mask = S::castepi64_pd(S::cmpgt_epi64(S::set1_epi64(16), h));
    let v = S::blendv_pd(z, y, mask);
    mask = S::castepi64_pd(S::cmpgt_epi64(S::set1_epi64(8), h));
    let w = S::blendv_pd(t, z, mask);

    let h_and_1 = S::castepi64_pd(S::cmpeq_epi64(
        S::setzero_epi64(),
        S::and_epi64(h, S::set1_epi64(1)),
    ));
    let h_and_2 = S::castepi64_pd(S::cmpeq_epi64(
        S::setzero_epi64(),
        S::and_epi64(h, S::set1_epi64(2)),
    ));
    let h_and_4 = S::castepi64_pd(S::cmpeq_epi64(
        S::setzero_epi64(),
        S::and_epi64(h, S::set1_epi64(4)),
    ));

    S::add_pd(
        S::blendv_pd(S::sub_pd(S::setzero_pd(), u), u, h_and_1),
        S::add_pd(
            S::blendv_pd(S::sub_pd(S::setzero_pd(), v), v, h_and_2),
            S::blendv_pd(S::sub_pd(S::setzero_pd(), w), w, h_and_4),
        ),
    )
}
