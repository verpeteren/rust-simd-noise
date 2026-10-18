//! Output buffer -> 2D slice -> colormap -> `slint::Image`.
//!
//! The simdnoise buffer is width-contiguous (loop nesting, outer->inner:
//! extent6, extent5, time, depth, height, width), so the (x,y) plane at any fixed
//! (z,w,u,v) is a single contiguous sub-slice. Index:
//!   idx = ((((v*E5 + u)*T + w)*D + z)*H + y)*W + x

use slint::Rgb8Pixel;

use crate::dispatch::Buffers;
use crate::params::*;

/// Height of the strip drawn for 1D noise.
const STRIP_H: usize = 160;

/// Rendered frame. Holds the raw RGB pixels (which are `Send`) rather than a
/// `slint::Image` (which is not), so the worker can post it to the UI thread where
/// the `Image` is actually constructed.
#[derive(Clone)]
pub struct Rendered {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Rgb8Pixel>,
    pub probe: Probe,
}

/// Per-pixel data for the currently displayed frame, kept on the UI thread so a
/// hover/click can report the value and coordinates under the cursor. `values` and
/// `zsel` are indexed by `py * img_w + px`.
#[derive(Clone)]
pub struct Probe {
    pub img_w: usize,
    pub img_h: usize,
    pub dim: usize,
    /// Fixed higher-axis slice positions [w, u, v].
    pub cross: [usize; 3],
    pub values: Vec<f32>,
    /// Colour-encoded z index per pixel for dim >= 3, else -1.
    pub zsel: Vec<i32>,
}

impl Probe {
    /// Build the hover detail string and the click JSON for pixel (px, py), or
    /// `None` when outside the image.
    pub fn at(&self, px: i32, py: i32) -> Option<(String, String)> {
        if px < 0 || py < 0 || px as usize >= self.img_w || py as usize >= self.img_h {
            return None;
        }
        let (xu, yu) = (px as usize, py as usize);
        let i = yu * self.img_w + xu;
        let value = self.values[i];
        let z = self.zsel[i];
        // Grid y only exists for dim >= 2 (1D is drawn as a strip).
        let y_is_real = self.dim >= 2;

        let mut text = format!("x={xu}");
        if y_is_real {
            text.push_str(&format!(", y={yu}"));
        }
        if self.dim >= 3 && z >= 0 {
            text.push_str(&format!(", z={z}"));
        }
        if self.dim >= 4 {
            text.push_str(&format!(", w={}", self.cross[0]));
        }
        if self.dim >= 5 {
            text.push_str(&format!(", u={}", self.cross[1]));
        }
        if self.dim >= 6 {
            text.push_str(&format!(", v={}", self.cross[2]));
        }
        text.push_str(&format!("   value={value:.5}"));

        // Click JSON: just this position's coordinates and value.
        let mut json = String::from("{");
        json.push_str(&format!("\"x\": {xu}"));
        if y_is_real {
            json.push_str(&format!(", \"y\": {yu}"));
        }
        if self.dim >= 3 && z >= 0 {
            json.push_str(&format!(", \"z\": {z}"));
        }
        if self.dim >= 4 {
            json.push_str(&format!(", \"w\": {}", self.cross[0]));
        }
        if self.dim >= 5 {
            json.push_str(&format!(", \"u\": {}", self.cross[1]));
        }
        if self.dim >= 6 {
            json.push_str(&format!(", \"v\": {}", self.cross[2]));
        }
        json.push_str(&format!(", \"value\": {value}"));
        json.push('}');

        Some((text, json))
    }
}

/// A read view over whichever buffer the current (engine, float, mode) wrote.
enum Src<'a> {
    F32(&'a [f32]),
    F64(&'a [f64]),
    U8(&'a [u8]),
    U16(&'a [u16]),
    U32(&'a [u32]),
    I16(&'a [i16]),
    I32(&'a [i32]),
}

impl Src<'_> {
    #[inline]
    fn get(&self, i: usize) -> f32 {
        match self {
            Src::F32(s) => s[i],
            Src::F64(s) => s[i] as f32,
            Src::U8(s) => s[i] as f32,
            Src::U16(s) => s[i] as f32,
            Src::U32(s) => s[i] as f32,
            Src::I16(s) => s[i] as f32,
            Src::I32(s) => s[i] as f32,
        }
    }

    fn len(&self) -> usize {
        match self {
            Src::F32(s) => s.len(),
            Src::F64(s) => s.len(),
            Src::U8(s) => s.len(),
            Src::U16(s) => s.len(),
            Src::U32(s) => s.len(),
            Src::I16(s) => s.len(),
            Src::I32(s) => s.len(),
        }
    }
}

/// Pick the active buffer and the (lo, hi) range to normalize against.
fn source<'a>(buf: &'a Buffers, p: &Params) -> (Src<'a>, f32, f32) {
    if p.engine != Engine::Auto {
        return match p.float {
            Float::F32 => (Src::F32(&buf.v_f32), buf.stats.0, buf.stats.1),
            Float::F64 => (Src::F64(&buf.v_f64), buf.stats.0, buf.stats.1),
        };
    }
    match (p.float, &p.out) {
        (Float::F32, OutMode::Raw) => (Src::F32(&buf.v_f32), buf.stats.0, buf.stats.1),
        (Float::F32, OutMode::Scaled { min, max }) => (Src::F32(&buf.v_f32), *min, *max),
        (Float::F64, OutMode::Raw) => (Src::F64(&buf.v_f64), buf.stats.0, buf.stats.1),
        (Float::F64, OutMode::Scaled { min, max }) => (Src::F64(&buf.v_f64), *min, *max),
        (Float::F32, OutMode::Quant { ty, .. }) => match ty {
            IntType::U8 => (Src::U8(&buf.v_u8), 0.0, u8::MAX as f32),
            IntType::U16 => (Src::U16(&buf.v_u16), 0.0, u16::MAX as f32),
            IntType::U32 => (Src::U32(&buf.v_u32), 0.0, u32::MAX as f32),
            IntType::I16 => (Src::I16(&buf.v_i16), i16::MIN as f32, i16::MAX as f32),
            IntType::I32 => (Src::I32(&buf.v_i32), i32::MIN as f32, i32::MAX as f32),
        },
        (Float::F64, OutMode::Quant { .. }) => (Src::F64(&buf.v_f64), buf.stats.0, buf.stats.1),
    }
}

pub fn to_image(buf: &Buffers, p: &Params) -> Rendered {
    let (src, lo, hi) = source(buf, p);
    let w = p.extent[0].max(1);

    let norm = |v: f32| -> f32 {
        if hi > lo {
            ((v - lo) / (hi - lo)).clamp(0.0, 1.0)
        } else {
            0.5
        }
    };

    let (img_w, img_h) = if p.dim == 1 {
        (w, STRIP_H)
    } else {
        (w, p.extent[1].max(1))
    };
    let black = Rgb8Pixel { r: 0, g: 0, b: 0 };
    let mut px = vec![black; img_w * img_h];
    let mut values = vec![0.0f32; img_w * img_h];
    let mut zsel = vec![-1i32; img_w * img_h];
    let n = src.len();
    let fetch = |i: usize| -> f32 {
        if i < n {
            src.get(i)
        } else {
            lo
        }
    };

    if p.dim == 1 {
        for x in 0..img_w {
            let raw = fetch(x);
            let c = colormap(p.colormap, norm(raw));
            for y in 0..img_h {
                px[y * img_w + x] = c;
                values[y * img_w + x] = raw;
            }
        }
    } else if p.dim == 2 {
        // Gradient render modes draw the 2D field as a heightfield (issue #24).
        // `build_params` only enables them for dim==2 derivative-capable configs,
        // so the gradient buffer is present with stride 2 here.
        let grad_mode = p.render != RenderMode::Value
            && buf.grad_stride >= 2
            && buf.grad.len() >= img_w * img_h * buf.grad_stride;
        // Steepest in-plane gradient over the image, to normalize the relief so
        // the shading/normal/magnitude is visible regardless of frequency scale.
        let gmax = if grad_mode {
            let st = buf.grad_stride;
            let mut m = 0.0f32;
            for i in 0..img_w * img_h {
                let (gx, gy) = (buf.grad[i * st], buf.grad[i * st + 1]);
                let mag = (gx * gx + gy * gy).sqrt();
                if mag > m {
                    m = mag;
                }
            }
            m
        } else {
            0.0
        };
        for y in 0..img_h {
            for x in 0..img_w {
                let i = y * w + x;
                let raw = fetch(i);
                values[y * img_w + x] = raw;
                px[y * img_w + x] = if grad_mode {
                    grad_color(p, &buf.grad, i, buf.grad_stride, gmax)
                } else {
                    colormap(p.colormap, norm(raw))
                };
            }
        }
    } else {
        // dim >= 3: colour-encode the z axis (hue = z fraction, brightness = value).
        // Higher axes w/u/v are fixed at the cross-section slider positions.
        let h = img_h;
        let d = p.extent[2].max(1);
        let t = p.extent[3].max(1);
        let e5 = p.extent[4].max(1);
        let e6 = p.extent[5].max(1);
        let ww = p.cross[0].min(t - 1);
        let uu = p.cross[1].min(e5 - 1);
        let vv = p.cross[2].min(e6 - 1);
        let base = (((vv * e5 + uu) * t + ww) * d) * h * w;
        for y in 0..h {
            for x in 0..w {
                let mut best_z = 0usize;
                let mut best_mag = -1.0f32;
                let mut best_t = 0.0f32;
                let mut best_raw = lo;
                for z in 0..d {
                    let idx = base + (z * h + y) * w + x;
                    let raw = fetch(idx);
                    let tval = norm(raw);
                    let mag = (tval - 0.5).abs();
                    if mag > best_mag {
                        best_mag = mag;
                        best_z = z;
                        best_t = tval;
                        best_raw = raw;
                    }
                }
                let hue = if d > 1 {
                    best_z as f32 / (d - 1) as f32
                } else {
                    0.0
                };
                px[y * w + x] = hsv(hue * 300.0, 1.0, best_t);
                values[y * w + x] = best_raw;
                zsel[y * w + x] = best_z as i32;
            }
        }
    }

    let probe = Probe {
        img_w,
        img_h,
        dim: p.dim,
        cross: p.cross,
        values,
        zsel,
    };
    Rendered {
        width: img_w as u32,
        height: img_h as u32,
        pixels: px,
        probe,
    }
}

/// Colour for one pixel of a gradient render mode. `grad` holds `stride`
/// components per sample; `gmax` is the image's steepest in-plane gradient,
/// used to normalize the relief. The field is treated as a heightfield
/// `z = value`, whose surface normal is `normalize((-gx, -gy, 1))` after the
/// gradient is scaled to unit relief.
fn grad_color(p: &Params, grad: &[f32], i: usize, stride: usize, gmax: f32) -> Rgb8Pixel {
    let gx = grad[i * stride];
    let gy = grad[i * stride + 1];
    match p.render {
        RenderMode::GradMag => {
            let mag = (gx * gx + gy * gy).sqrt();
            let t = if gmax > 0.0 { mag / gmax } else { 0.0 };
            colormap(p.colormap, t.clamp(0.0, 1.0))
        }
        RenderMode::SlopeShade => {
            let (nx, ny, nz) = heightfield_normal(gx, gy, gmax);
            // Light from the upper-left, slightly in front: normalize((-1,-1,1)).
            const L: [f32; 3] = [-0.577_35, -0.577_35, 0.577_35];
            let d = (nx * L[0] + ny * L[1] + nz * L[2]).max(0.0);
            let v = (d * 255.0) as u8;
            Rgb8Pixel { r: v, g: v, b: v }
        }
        RenderMode::NormalMap => {
            let (nx, ny, nz) = heightfield_normal(gx, gy, gmax);
            Rgb8Pixel {
                r: ((nx * 0.5 + 0.5) * 255.0) as u8,
                g: ((ny * 0.5 + 0.5) * 255.0) as u8,
                b: ((nz * 0.5 + 0.5) * 255.0) as u8,
            }
        }
        // `grad_mode` in `to_image` is false for `Value`, so this is unreachable;
        // return the flat normal/black rather than panicking if that ever changes.
        RenderMode::Value => Rgb8Pixel { r: 0, g: 0, b: 0 },
    }
}

/// Unit surface normal of the heightfield `z = value`, with the gradient scaled
/// by `1/gmax` so the steepest pixel tilts ~45° (visible relief at any scale).
fn heightfield_normal(gx: f32, gy: f32, gmax: f32) -> (f32, f32, f32) {
    let s = if gmax > 0.0 { 1.0 / gmax } else { 0.0 };
    let (ex, ey) = (-gx * s, -gy * s);
    let inv = 1.0 / (ex * ex + ey * ey + 1.0).sqrt();
    (ex * inv, ey * inv, inv)
}

fn colormap(kind: Colormap, t: f32) -> Rgb8Pixel {
    match kind {
        Colormap::Grayscale => {
            let v = (t * 255.0) as u8;
            Rgb8Pixel { r: v, g: v, b: v }
        }
        Colormap::Heatmap => {
            // Classic "jet"-style ramp blue -> cyan -> yellow -> red.
            let r = (1.5 - (4.0 * t - 3.0).abs()).clamp(0.0, 1.0);
            let g = (1.5 - (4.0 * t - 2.0).abs()).clamp(0.0, 1.0);
            let b = (1.5 - (4.0 * t - 1.0).abs()).clamp(0.0, 1.0);
            Rgb8Pixel {
                r: (r * 255.0) as u8,
                g: (g * 255.0) as u8,
                b: (b * 255.0) as u8,
            }
        }
    }
}

/// HSV -> RGB. `h` in degrees `[0, 360)`, `s`/`v` in `[0, 1]`.
fn hsv(h: f32, s: f32, v: f32) -> Rgb8Pixel {
    let c = v * s;
    let hp = (h / 60.0).rem_euclid(6.0);
    let x = c * (1.0 - (hp % 2.0 - 1.0).abs());
    let (r1, g1, b1) = match hp as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = v - c;
    Rgb8Pixel {
        r: ((r1 + m) * 255.0) as u8,
        g: ((g1 + m) * 255.0) as u8,
        b: ((b1 + m) * 255.0) as u8,
    }
}
