use super::{
    Cellular2Settings, CellularSettings, FbmSettings, GradientSettings, NoiseDimensions,
    RidgeSettings, TurbulenceSettings,
};

pub struct NoiseBuilder {}

impl NoiseBuilder {
    pub fn cellular_2d(width: usize, height: usize) -> CellularSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.width = width;
        dim.height = height;
        CellularSettings::default(dim)
    }

    pub fn cellular_2d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
    ) -> CellularSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.width = width;
        dim.height = height;
        dim.x = x_offset;
        dim.y = y_offset;
        CellularSettings::default(dim)
    }

    pub fn cellular_3d(width: usize, height: usize, depth: usize) -> CellularSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        CellularSettings::default(dim)
    }

    pub fn cellular_3d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
    ) -> CellularSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        dim.x = x_offset;
        dim.y = y_offset;
        dim.z = z_offset;
        CellularSettings::default(dim)
    }

    pub fn cellular2_2d(width: usize, height: usize) -> Cellular2Settings {
        let mut dim = NoiseDimensions::default(2);
        dim.width = width;
        dim.height = height;
        Cellular2Settings::default(dim)
    }

    pub fn cellular2_2d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
    ) -> Cellular2Settings {
        let mut dim = NoiseDimensions::default(2);
        dim.width = width;
        dim.height = height;
        dim.x = x_offset;
        dim.y = y_offset;
        Cellular2Settings::default(dim)
    }

    pub fn cellular2_3d(width: usize, height: usize, depth: usize) -> Cellular2Settings {
        let mut dim = NoiseDimensions::default(3);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        Cellular2Settings::default(dim)
    }

    pub fn cellular2_3d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
    ) -> Cellular2Settings {
        let mut dim = NoiseDimensions::default(3);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        dim.x = x_offset;
        dim.y = y_offset;
        dim.z = z_offset;
        Cellular2Settings::default(dim)
    }

    pub fn fbm_1d(width: usize) -> FbmSettings {
        let mut dim = NoiseDimensions::default(1);
        dim.width = width;
        FbmSettings::default(dim)
    }

    pub fn fbm_1d_offset(x_offset: f32, width: usize) -> FbmSettings {
        let mut dim = NoiseDimensions::default(1);
        dim.width = width;
        dim.x = x_offset;
        FbmSettings::default(dim)
    }

    pub fn fbm_2d(width: usize, height: usize) -> FbmSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.width = width;
        dim.height = height;
        FbmSettings::default(dim)
    }

    pub fn fbm_2d_offset(x_offset: f32, width: usize, y_offset: f32, height: usize) -> FbmSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.width = width;
        dim.height = height;
        dim.x = x_offset;
        dim.y = y_offset;
        FbmSettings::default(dim)
    }

    pub fn fbm_3d(width: usize, height: usize, depth: usize) -> FbmSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        FbmSettings::default(dim)
    }

    pub fn fbm_3d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
    ) -> FbmSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        dim.x = x_offset;
        dim.y = y_offset;
        dim.z = z_offset;
        FbmSettings::default(dim)
    }

    pub fn fbm_4d(width: usize, height: usize, depth: usize, time: usize) -> FbmSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        dim.time = time;
        FbmSettings::default(dim)
    }

    pub fn fbm_4d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
        w_offset: f32,
        time: usize,
    ) -> FbmSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        dim.time = time;
        dim.x = x_offset;
        dim.y = y_offset;
        dim.z = z_offset;
        dim.w = w_offset;
        FbmSettings::default(dim)
    }

    pub fn ridge_1d(width: usize) -> RidgeSettings {
        let mut dim = NoiseDimensions::default(1);
        dim.width = width;
        RidgeSettings::default(dim)
    }

    pub fn ridge_1d_offset(x_offset: f32, width: usize) -> RidgeSettings {
        let mut dim = NoiseDimensions::default(1);
        dim.width = width;
        dim.x = x_offset;
        RidgeSettings::default(dim)
    }

    pub fn ridge_2d(width: usize, height: usize) -> RidgeSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.width = width;
        dim.height = height;
        RidgeSettings::default(dim)
    }

    pub fn ridge_2d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
    ) -> RidgeSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.width = width;
        dim.height = height;
        dim.x = x_offset;
        dim.y = y_offset;
        RidgeSettings::default(dim)
    }

    pub fn ridge_3d(width: usize, height: usize, depth: usize) -> RidgeSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        RidgeSettings::default(dim)
    }

    pub fn ridge_3d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
    ) -> RidgeSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        dim.x = x_offset;
        dim.y = y_offset;
        dim.z = z_offset;
        RidgeSettings::default(dim)
    }

    pub fn ridge_4d(width: usize, height: usize, depth: usize, time: usize) -> RidgeSettings {
        let mut dim = NoiseDimensions::default(4);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        dim.time = time;
        RidgeSettings::default(dim)
    }

    pub fn ridge_4d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
        w_offset: f32,
        time: usize,
    ) -> RidgeSettings {
        let mut dim = NoiseDimensions::default(4);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        dim.time = time;
        dim.x = x_offset;
        dim.y = y_offset;
        dim.z = z_offset;
        dim.w = w_offset;
        RidgeSettings::default(dim)
    }

    // Turbulence Builders
    pub fn turbulence_1d(width: usize) -> TurbulenceSettings {
        let mut dim = NoiseDimensions::default(1);
        dim.width = width;
        TurbulenceSettings::default(dim)
    }

    pub fn turbulence_1d_offset(x_offset: f32, width: usize) -> TurbulenceSettings {
        let mut dim = NoiseDimensions::default(1);
        dim.width = width;
        dim.x = x_offset;
        TurbulenceSettings::default(dim)
    }

    pub fn turbulence_2d(width: usize, height: usize) -> TurbulenceSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.width = width;
        dim.height = height;
        TurbulenceSettings::default(dim)
    }

    pub fn turbulence_2d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
    ) -> TurbulenceSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.width = width;
        dim.height = height;
        dim.x = x_offset;
        dim.y = y_offset;
        TurbulenceSettings::default(dim)
    }

    pub fn turbulence_3d(width: usize, height: usize, depth: usize) -> TurbulenceSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        TurbulenceSettings::default(dim)
    }

    pub fn turbulence_3d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
    ) -> TurbulenceSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        dim.x = x_offset;
        dim.y = y_offset;
        dim.z = z_offset;
        TurbulenceSettings::default(dim)
    }

    pub fn turbulence_4d(
        width: usize,
        height: usize,
        depth: usize,
        time: usize,
    ) -> TurbulenceSettings {
        let mut dim = NoiseDimensions::default(4);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        dim.time = time;
        TurbulenceSettings::default(dim)
    }

    pub fn turbulence_4d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
        w_offset: f32,
        time: usize,
    ) -> TurbulenceSettings {
        let mut dim = NoiseDimensions::default(4);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        dim.time = time;
        dim.x = x_offset;
        dim.y = y_offset;
        dim.z = z_offset;
        dim.w = w_offset;
        TurbulenceSettings::default(dim)
    }

    // Gradient Builders
    pub fn gradient_1d(width: usize) -> GradientSettings {
        let mut dim = NoiseDimensions::default(1);
        dim.width = width;
        GradientSettings::default(dim)
    }

    pub fn gradient_1d_offset(x_offset: f32, width: usize) -> GradientSettings {
        let mut dim = NoiseDimensions::default(1);
        dim.width = width;
        dim.x = x_offset;
        GradientSettings::default(dim)
    }

    pub fn gradient_2d(width: usize, height: usize) -> GradientSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.width = width;
        dim.height = height;
        GradientSettings::default(dim)
    }

    pub fn gradient_2d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
    ) -> GradientSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.width = width;
        dim.height = height;
        dim.x = x_offset;
        dim.y = y_offset;
        GradientSettings::default(dim)
    }

    pub fn gradient_3d(width: usize, height: usize, depth: usize) -> GradientSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        GradientSettings::default(dim)
    }

    pub fn gradient_3d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
    ) -> GradientSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        dim.x = x_offset;
        dim.y = y_offset;
        dim.z = z_offset;
        GradientSettings::default(dim)
    }

    pub fn gradient_4d(width: usize, height: usize, depth: usize, time: usize) -> GradientSettings {
        let mut dim = NoiseDimensions::default(4);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        dim.time = time;
        GradientSettings::default(dim)
    }

    pub fn gradient_4d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
        w_offset: f32,
        time: usize,
    ) -> GradientSettings {
        let mut dim = NoiseDimensions::default(4);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        dim.time = time;
        dim.x = x_offset;
        dim.y = y_offset;
        dim.z = z_offset;
        dim.w = w_offset;
        GradientSettings::default(dim)
    }
}
