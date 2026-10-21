use super::{
    Cellular2Settings, CellularSettings, FbmSettings, GaborSettings, GradientSettings,
    NoiseDimensions, RidgeSettings, Settings, TurbulenceSettings,
};

pub struct NoiseBuilder {}

impl NoiseBuilder {
    pub fn cellular_2d(width: usize, height: usize) -> CellularSettings {
        let mut dimension = NoiseDimensions::default(2);
        dimension.extent_x = width;
        dimension.extent_y = height;
        CellularSettings::default(dimension)
    }

    pub fn cellular_2d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
    ) -> CellularSettings {
        let mut dimension = NoiseDimensions::default(2);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        CellularSettings::default(dimension)
    }

    pub fn cellular_3d(width: usize, height: usize, depth: usize) -> CellularSettings {
        let mut dimension = NoiseDimensions::default(3);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        CellularSettings::default(dimension)
    }

    pub fn cellular_3d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
    ) -> CellularSettings {
        let mut dimension = NoiseDimensions::default(3);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        dimension.offset_z = z_offset;
        CellularSettings::default(dimension)
    }

    pub fn cellular_1d(width: usize) -> CellularSettings {
        let mut dimension = NoiseDimensions::default(1);
        dimension.extent_x = width;
        CellularSettings::default(dimension)
    }

    pub fn cellular_1d_offset(x_offset: f32, width: usize) -> CellularSettings {
        let mut dimension = NoiseDimensions::default(1);
        dimension.extent_x = width;
        dimension.offset_x = x_offset;
        CellularSettings::default(dimension)
    }

    pub fn cellular_4d(width: usize, height: usize, depth: usize, time: usize) -> CellularSettings {
        let mut dimension = NoiseDimensions::default(4);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        CellularSettings::default(dimension)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn cellular_4d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
        w_offset: f32,
        time: usize,
    ) -> CellularSettings {
        let mut dimension = NoiseDimensions::default(4);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        dimension.offset_z = z_offset;
        dimension.offset_w = w_offset;
        CellularSettings::default(dimension)
    }

    /// Experimental: 5D/6D noise. See the crate-level docs on 5D/6D.
    pub fn cellular_5d(
        width: usize,
        height: usize,
        depth: usize,
        time: usize,
        extent5: usize,
    ) -> CellularSettings {
        let mut dimension = NoiseDimensions::default(5);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        dimension.extent_u = extent5;
        CellularSettings::default(dimension)
    }

    #[allow(clippy::too_many_arguments)]
    /// Experimental: 5D/6D noise. See the crate-level docs on 5D/6D.
    pub fn cellular_5d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
        w_offset: f32,
        time: usize,
        u_offset: f32,
        extent5: usize,
    ) -> CellularSettings {
        let mut dimension = NoiseDimensions::default(5);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        dimension.extent_u = extent5;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        dimension.offset_z = z_offset;
        dimension.offset_w = w_offset;
        dimension.offset_u = u_offset;
        CellularSettings::default(dimension)
    }

    #[allow(clippy::too_many_arguments)]
    /// Experimental: 5D/6D noise. See the crate-level docs on 5D/6D.
    pub fn cellular_6d(
        width: usize,
        height: usize,
        depth: usize,
        time: usize,
        extent5: usize,
        extent6: usize,
    ) -> CellularSettings {
        let mut dimension = NoiseDimensions::default(6);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        dimension.extent_u = extent5;
        dimension.extent_v = extent6;
        CellularSettings::default(dimension)
    }

    #[allow(clippy::too_many_arguments)]
    /// Experimental: 5D/6D noise. See the crate-level docs on 5D/6D.
    pub fn cellular_6d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
        w_offset: f32,
        time: usize,
        u_offset: f32,
        extent5: usize,
        v_offset: f32,
        extent6: usize,
    ) -> CellularSettings {
        let mut dimension = NoiseDimensions::default(6);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        dimension.extent_u = extent5;
        dimension.extent_v = extent6;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        dimension.offset_z = z_offset;
        dimension.offset_w = w_offset;
        dimension.offset_u = u_offset;
        dimension.offset_v = v_offset;
        CellularSettings::default(dimension)
    }

    pub fn cellular2_2d(width: usize, height: usize) -> Cellular2Settings {
        let mut dimension = NoiseDimensions::default(2);
        dimension.extent_x = width;
        dimension.extent_y = height;
        Cellular2Settings::default(dimension)
    }

    pub fn cellular2_2d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
    ) -> Cellular2Settings {
        let mut dimension = NoiseDimensions::default(2);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        Cellular2Settings::default(dimension)
    }

    pub fn cellular2_3d(width: usize, height: usize, depth: usize) -> Cellular2Settings {
        let mut dimension = NoiseDimensions::default(3);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        Cellular2Settings::default(dimension)
    }

    pub fn cellular2_3d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
    ) -> Cellular2Settings {
        let mut dimension = NoiseDimensions::default(3);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        dimension.offset_z = z_offset;
        Cellular2Settings::default(dimension)
    }

    pub fn cellular2_1d(width: usize) -> Cellular2Settings {
        let mut dimension = NoiseDimensions::default(1);
        dimension.extent_x = width;
        Cellular2Settings::default(dimension)
    }

    pub fn cellular2_1d_offset(x_offset: f32, width: usize) -> Cellular2Settings {
        let mut dimension = NoiseDimensions::default(1);
        dimension.extent_x = width;
        dimension.offset_x = x_offset;
        Cellular2Settings::default(dimension)
    }

    pub fn cellular2_4d(
        width: usize,
        height: usize,
        depth: usize,
        time: usize,
    ) -> Cellular2Settings {
        let mut dimension = NoiseDimensions::default(4);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        Cellular2Settings::default(dimension)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn cellular2_4d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
        w_offset: f32,
        time: usize,
    ) -> Cellular2Settings {
        let mut dimension = NoiseDimensions::default(4);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        dimension.offset_z = z_offset;
        dimension.offset_w = w_offset;
        Cellular2Settings::default(dimension)
    }

    /// Experimental: 5D/6D noise. See the crate-level docs on 5D/6D.
    pub fn cellular2_5d(
        width: usize,
        height: usize,
        depth: usize,
        time: usize,
        extent5: usize,
    ) -> Cellular2Settings {
        let mut dimension = NoiseDimensions::default(5);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        dimension.extent_u = extent5;
        Cellular2Settings::default(dimension)
    }

    #[allow(clippy::too_many_arguments)]
    /// Experimental: 5D/6D noise. See the crate-level docs on 5D/6D.
    pub fn cellular2_5d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
        w_offset: f32,
        time: usize,
        u_offset: f32,
        extent5: usize,
    ) -> Cellular2Settings {
        let mut dimension = NoiseDimensions::default(5);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        dimension.extent_u = extent5;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        dimension.offset_z = z_offset;
        dimension.offset_w = w_offset;
        dimension.offset_u = u_offset;
        Cellular2Settings::default(dimension)
    }

    #[allow(clippy::too_many_arguments)]
    /// Experimental: 5D/6D noise. See the crate-level docs on 5D/6D.
    pub fn cellular2_6d(
        width: usize,
        height: usize,
        depth: usize,
        time: usize,
        extent5: usize,
        extent6: usize,
    ) -> Cellular2Settings {
        let mut dimension = NoiseDimensions::default(6);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        dimension.extent_u = extent5;
        dimension.extent_v = extent6;
        Cellular2Settings::default(dimension)
    }

    #[allow(clippy::too_many_arguments)]
    /// Experimental: 5D/6D noise. See the crate-level docs on 5D/6D.
    pub fn cellular2_6d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
        w_offset: f32,
        time: usize,
        u_offset: f32,
        extent5: usize,
        v_offset: f32,
        extent6: usize,
    ) -> Cellular2Settings {
        let mut dimension = NoiseDimensions::default(6);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        dimension.extent_u = extent5;
        dimension.extent_v = extent6;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        dimension.offset_z = z_offset;
        dimension.offset_w = w_offset;
        dimension.offset_u = u_offset;
        dimension.offset_v = v_offset;
        Cellular2Settings::default(dimension)
    }

    pub fn fbm_1d(width: usize) -> FbmSettings {
        let mut dimension = NoiseDimensions::default(1);
        dimension.extent_x = width;
        FbmSettings::default(dimension)
    }

    pub fn fbm_1d_offset(x_offset: f32, width: usize) -> FbmSettings {
        let mut dimension = NoiseDimensions::default(1);
        dimension.extent_x = width;
        dimension.offset_x = x_offset;
        FbmSettings::default(dimension)
    }

    pub fn fbm_2d(width: usize, height: usize) -> FbmSettings {
        let mut dimension = NoiseDimensions::default(2);
        dimension.extent_x = width;
        dimension.extent_y = height;
        FbmSettings::default(dimension)
    }

    pub fn fbm_2d_offset(x_offset: f32, width: usize, y_offset: f32, height: usize) -> FbmSettings {
        let mut dimension = NoiseDimensions::default(2);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        FbmSettings::default(dimension)
    }

    pub fn fbm_3d(width: usize, height: usize, depth: usize) -> FbmSettings {
        let mut dimension = NoiseDimensions::default(3);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        FbmSettings::default(dimension)
    }

    pub fn fbm_3d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
    ) -> FbmSettings {
        let mut dimension = NoiseDimensions::default(3);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        dimension.offset_z = z_offset;
        FbmSettings::default(dimension)
    }

    pub fn fbm_4d(width: usize, height: usize, depth: usize, time: usize) -> FbmSettings {
        let mut dimension = NoiseDimensions::default(4);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        FbmSettings::default(dimension)
    }

    #[allow(clippy::too_many_arguments)]
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
        let mut dimension = NoiseDimensions::default(4);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        dimension.offset_z = z_offset;
        dimension.offset_w = w_offset;
        FbmSettings::default(dimension)
    }

    /// Experimental: 5D/6D noise. See the crate-level docs on 5D/6D.
    pub fn fbm_5d(
        width: usize,
        height: usize,
        depth: usize,
        time: usize,
        extent5: usize,
    ) -> FbmSettings {
        let mut dimension = NoiseDimensions::default(5);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        dimension.extent_u = extent5;
        FbmSettings::default(dimension)
    }

    #[allow(clippy::too_many_arguments)]
    /// Experimental: 5D/6D noise. See the crate-level docs on 5D/6D.
    pub fn fbm_5d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
        w_offset: f32,
        time: usize,
        u_offset: f32,
        extent5: usize,
    ) -> FbmSettings {
        let mut dimension = NoiseDimensions::default(5);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        dimension.extent_u = extent5;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        dimension.offset_z = z_offset;
        dimension.offset_w = w_offset;
        dimension.offset_u = u_offset;
        FbmSettings::default(dimension)
    }

    #[allow(clippy::too_many_arguments)]
    /// Experimental: 5D/6D noise. See the crate-level docs on 5D/6D.
    pub fn fbm_6d(
        width: usize,
        height: usize,
        depth: usize,
        time: usize,
        extent5: usize,
        extent6: usize,
    ) -> FbmSettings {
        let mut dimension = NoiseDimensions::default(6);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        dimension.extent_u = extent5;
        dimension.extent_v = extent6;
        FbmSettings::default(dimension)
    }

    #[allow(clippy::too_many_arguments)]
    /// Experimental: 5D/6D noise. See the crate-level docs on 5D/6D.
    pub fn fbm_6d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
        w_offset: f32,
        time: usize,
        u_offset: f32,
        extent5: usize,
        v_offset: f32,
        extent6: usize,
    ) -> FbmSettings {
        let mut dimension = NoiseDimensions::default(6);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        dimension.extent_u = extent5;
        dimension.extent_v = extent6;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        dimension.offset_z = z_offset;
        dimension.offset_w = w_offset;
        dimension.offset_u = u_offset;
        dimension.offset_v = v_offset;
        FbmSettings::default(dimension)
    }

    pub fn ridge_1d(width: usize) -> RidgeSettings {
        let mut dimension = NoiseDimensions::default(1);
        dimension.extent_x = width;
        RidgeSettings::default(dimension)
    }

    pub fn ridge_1d_offset(x_offset: f32, width: usize) -> RidgeSettings {
        let mut dimension = NoiseDimensions::default(1);
        dimension.extent_x = width;
        dimension.offset_x = x_offset;
        RidgeSettings::default(dimension)
    }

    pub fn ridge_2d(width: usize, height: usize) -> RidgeSettings {
        let mut dimension = NoiseDimensions::default(2);
        dimension.extent_x = width;
        dimension.extent_y = height;
        RidgeSettings::default(dimension)
    }

    pub fn ridge_2d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
    ) -> RidgeSettings {
        let mut dimension = NoiseDimensions::default(2);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        RidgeSettings::default(dimension)
    }

    pub fn ridge_3d(width: usize, height: usize, depth: usize) -> RidgeSettings {
        let mut dimension = NoiseDimensions::default(3);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        RidgeSettings::default(dimension)
    }

    pub fn ridge_3d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
    ) -> RidgeSettings {
        let mut dimension = NoiseDimensions::default(3);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        dimension.offset_z = z_offset;
        RidgeSettings::default(dimension)
    }

    pub fn ridge_4d(width: usize, height: usize, depth: usize, time: usize) -> RidgeSettings {
        let mut dimension = NoiseDimensions::default(4);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        RidgeSettings::default(dimension)
    }

    #[allow(clippy::too_many_arguments)]
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
        let mut dimension = NoiseDimensions::default(4);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        dimension.offset_z = z_offset;
        dimension.offset_w = w_offset;
        RidgeSettings::default(dimension)
    }

    /// Experimental: 5D/6D noise. See the crate-level docs on 5D/6D.
    pub fn ridge_5d(
        width: usize,
        height: usize,
        depth: usize,
        time: usize,
        extent5: usize,
    ) -> RidgeSettings {
        let mut dimension = NoiseDimensions::default(5);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        dimension.extent_u = extent5;
        RidgeSettings::default(dimension)
    }

    #[allow(clippy::too_many_arguments)]
    /// Experimental: 5D/6D noise. See the crate-level docs on 5D/6D.
    pub fn ridge_5d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
        w_offset: f32,
        time: usize,
        u_offset: f32,
        extent5: usize,
    ) -> RidgeSettings {
        let mut dimension = NoiseDimensions::default(5);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        dimension.extent_u = extent5;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        dimension.offset_z = z_offset;
        dimension.offset_w = w_offset;
        dimension.offset_u = u_offset;
        RidgeSettings::default(dimension)
    }

    #[allow(clippy::too_many_arguments)]
    /// Experimental: 5D/6D noise. See the crate-level docs on 5D/6D.
    pub fn ridge_6d(
        width: usize,
        height: usize,
        depth: usize,
        time: usize,
        extent5: usize,
        extent6: usize,
    ) -> RidgeSettings {
        let mut dimension = NoiseDimensions::default(6);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        dimension.extent_u = extent5;
        dimension.extent_v = extent6;
        RidgeSettings::default(dimension)
    }

    #[allow(clippy::too_many_arguments)]
    /// Experimental: 5D/6D noise. See the crate-level docs on 5D/6D.
    pub fn ridge_6d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
        w_offset: f32,
        time: usize,
        u_offset: f32,
        extent5: usize,
        v_offset: f32,
        extent6: usize,
    ) -> RidgeSettings {
        let mut dimension = NoiseDimensions::default(6);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        dimension.extent_u = extent5;
        dimension.extent_v = extent6;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        dimension.offset_z = z_offset;
        dimension.offset_w = w_offset;
        dimension.offset_u = u_offset;
        dimension.offset_v = v_offset;
        RidgeSettings::default(dimension)
    }

    // Turbulence Builders
    pub fn turbulence_1d(width: usize) -> TurbulenceSettings {
        let mut dimension = NoiseDimensions::default(1);
        dimension.extent_x = width;
        TurbulenceSettings::default(dimension)
    }

    pub fn turbulence_1d_offset(x_offset: f32, width: usize) -> TurbulenceSettings {
        let mut dimension = NoiseDimensions::default(1);
        dimension.extent_x = width;
        dimension.offset_x = x_offset;
        TurbulenceSettings::default(dimension)
    }

    pub fn turbulence_2d(width: usize, height: usize) -> TurbulenceSettings {
        let mut dimension = NoiseDimensions::default(2);
        dimension.extent_x = width;
        dimension.extent_y = height;
        TurbulenceSettings::default(dimension)
    }

    pub fn turbulence_2d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
    ) -> TurbulenceSettings {
        let mut dimension = NoiseDimensions::default(2);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        TurbulenceSettings::default(dimension)
    }

    pub fn turbulence_3d(width: usize, height: usize, depth: usize) -> TurbulenceSettings {
        let mut dimension = NoiseDimensions::default(3);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        TurbulenceSettings::default(dimension)
    }

    pub fn turbulence_3d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
    ) -> TurbulenceSettings {
        let mut dimension = NoiseDimensions::default(3);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        dimension.offset_z = z_offset;
        TurbulenceSettings::default(dimension)
    }

    pub fn turbulence_4d(
        width: usize,
        height: usize,
        depth: usize,
        time: usize,
    ) -> TurbulenceSettings {
        let mut dimension = NoiseDimensions::default(4);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        TurbulenceSettings::default(dimension)
    }

    #[allow(clippy::too_many_arguments)]
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
        let mut dimension = NoiseDimensions::default(4);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        dimension.offset_z = z_offset;
        dimension.offset_w = w_offset;
        TurbulenceSettings::default(dimension)
    }

    /// Experimental: 5D/6D noise. See the crate-level docs on 5D/6D.
    pub fn turbulence_5d(
        width: usize,
        height: usize,
        depth: usize,
        time: usize,
        extent5: usize,
    ) -> TurbulenceSettings {
        let mut dimension = NoiseDimensions::default(5);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        dimension.extent_u = extent5;
        TurbulenceSettings::default(dimension)
    }

    #[allow(clippy::too_many_arguments)]
    /// Experimental: 5D/6D noise. See the crate-level docs on 5D/6D.
    pub fn turbulence_5d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
        w_offset: f32,
        time: usize,
        u_offset: f32,
        extent5: usize,
    ) -> TurbulenceSettings {
        let mut dimension = NoiseDimensions::default(5);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        dimension.extent_u = extent5;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        dimension.offset_z = z_offset;
        dimension.offset_w = w_offset;
        dimension.offset_u = u_offset;
        TurbulenceSettings::default(dimension)
    }

    #[allow(clippy::too_many_arguments)]
    /// Experimental: 5D/6D noise. See the crate-level docs on 5D/6D.
    pub fn turbulence_6d(
        width: usize,
        height: usize,
        depth: usize,
        time: usize,
        extent5: usize,
        extent6: usize,
    ) -> TurbulenceSettings {
        let mut dimension = NoiseDimensions::default(6);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        dimension.extent_u = extent5;
        dimension.extent_v = extent6;
        TurbulenceSettings::default(dimension)
    }

    #[allow(clippy::too_many_arguments)]
    /// Experimental: 5D/6D noise. See the crate-level docs on 5D/6D.
    pub fn turbulence_6d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
        w_offset: f32,
        time: usize,
        u_offset: f32,
        extent5: usize,
        v_offset: f32,
        extent6: usize,
    ) -> TurbulenceSettings {
        let mut dimension = NoiseDimensions::default(6);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        dimension.extent_u = extent5;
        dimension.extent_v = extent6;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        dimension.offset_z = z_offset;
        dimension.offset_w = w_offset;
        dimension.offset_u = u_offset;
        dimension.offset_v = v_offset;
        TurbulenceSettings::default(dimension)
    }

    // Gabor Builders
    pub fn gabor_2d(width: usize, height: usize) -> GaborSettings {
        let mut dimension = NoiseDimensions::default(2);
        dimension.extent_x = width;
        dimension.extent_y = height;
        GaborSettings::default(dimension)
    }

    pub fn gabor_2d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
    ) -> GaborSettings {
        let mut dimension = NoiseDimensions::default(2);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        GaborSettings::default(dimension)
    }

    pub fn gabor_3d(width: usize, height: usize, depth: usize) -> GaborSettings {
        let mut dimension = NoiseDimensions::default(3);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        GaborSettings::default(dimension)
    }

    pub fn gabor_3d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
    ) -> GaborSettings {
        let mut dimension = NoiseDimensions::default(3);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        dimension.offset_z = z_offset;
        GaborSettings::default(dimension)
    }

    pub fn gabor_4d(width: usize, height: usize, depth: usize, time: usize) -> GaborSettings {
        let mut dimension = NoiseDimensions::default(4);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        GaborSettings::default(dimension)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn gabor_4d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
        w_offset: f32,
        time: usize,
    ) -> GaborSettings {
        let mut dimension = NoiseDimensions::default(4);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        dimension.offset_z = z_offset;
        dimension.offset_w = w_offset;
        GaborSettings::default(dimension)
    }

    // Gradient Builders
    pub fn gradient_1d(width: usize) -> GradientSettings {
        let mut dimension = NoiseDimensions::default(1);
        dimension.extent_x = width;
        GradientSettings::default(dimension)
    }

    pub fn gradient_1d_offset(x_offset: f32, width: usize) -> GradientSettings {
        let mut dimension = NoiseDimensions::default(1);
        dimension.extent_x = width;
        dimension.offset_x = x_offset;
        GradientSettings::default(dimension)
    }

    pub fn gradient_2d(width: usize, height: usize) -> GradientSettings {
        let mut dimension = NoiseDimensions::default(2);
        dimension.extent_x = width;
        dimension.extent_y = height;
        GradientSettings::default(dimension)
    }

    pub fn gradient_2d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
    ) -> GradientSettings {
        let mut dimension = NoiseDimensions::default(2);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        GradientSettings::default(dimension)
    }

    pub fn gradient_3d(width: usize, height: usize, depth: usize) -> GradientSettings {
        let mut dimension = NoiseDimensions::default(3);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        GradientSettings::default(dimension)
    }

    pub fn gradient_3d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
    ) -> GradientSettings {
        let mut dimension = NoiseDimensions::default(3);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        dimension.offset_z = z_offset;
        GradientSettings::default(dimension)
    }

    pub fn gradient_4d(width: usize, height: usize, depth: usize, time: usize) -> GradientSettings {
        let mut dimension = NoiseDimensions::default(4);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        GradientSettings::default(dimension)
    }

    #[allow(clippy::too_many_arguments)]
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
        let mut dimension = NoiseDimensions::default(4);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        dimension.offset_z = z_offset;
        dimension.offset_w = w_offset;
        GradientSettings::default(dimension)
    }

    /// Experimental: 5D/6D noise. See the crate-level docs on 5D/6D.
    pub fn gradient_5d(
        width: usize,
        height: usize,
        depth: usize,
        time: usize,
        extent5: usize,
    ) -> GradientSettings {
        let mut dimension = NoiseDimensions::default(5);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        dimension.extent_u = extent5;
        GradientSettings::default(dimension)
    }

    #[allow(clippy::too_many_arguments)]
    /// Experimental: 5D/6D noise. See the crate-level docs on 5D/6D.
    pub fn gradient_5d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
        w_offset: f32,
        time: usize,
        u_offset: f32,
        extent5: usize,
    ) -> GradientSettings {
        let mut dimension = NoiseDimensions::default(5);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        dimension.extent_u = extent5;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        dimension.offset_z = z_offset;
        dimension.offset_w = w_offset;
        dimension.offset_u = u_offset;
        GradientSettings::default(dimension)
    }

    #[allow(clippy::too_many_arguments)]
    /// Experimental: 5D/6D noise. See the crate-level docs on 5D/6D.
    pub fn gradient_6d(
        width: usize,
        height: usize,
        depth: usize,
        time: usize,
        extent5: usize,
        extent6: usize,
    ) -> GradientSettings {
        let mut dimension = NoiseDimensions::default(6);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        dimension.extent_u = extent5;
        dimension.extent_v = extent6;
        GradientSettings::default(dimension)
    }

    #[allow(clippy::too_many_arguments)]
    /// Experimental: 5D/6D noise. See the crate-level docs on 5D/6D.
    pub fn gradient_6d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
        w_offset: f32,
        time: usize,
        u_offset: f32,
        extent5: usize,
        v_offset: f32,
        extent6: usize,
    ) -> GradientSettings {
        let mut dimension = NoiseDimensions::default(6);
        dimension.extent_x = width;
        dimension.extent_y = height;
        dimension.extent_z = depth;
        dimension.extent_w = time;
        dimension.extent_u = extent5;
        dimension.extent_v = extent6;
        dimension.offset_x = x_offset;
        dimension.offset_y = y_offset;
        dimension.offset_z = z_offset;
        dimension.offset_w = w_offset;
        dimension.offset_u = u_offset;
        dimension.offset_v = v_offset;
        GradientSettings::default(dimension)
    }
}
