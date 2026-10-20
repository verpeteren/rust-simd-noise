use super::{
    Cellular2Settings, CellularSettings, FbmSettings, GradientSettings, NoiseDimensions,
    RidgeSettings, Settings, TurbulenceSettings,
};

pub struct NoiseBuilder {}

impl NoiseBuilder {
    pub fn cellular_2d(width: usize, height: usize) -> CellularSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.extent_x = width;
        dim.extent_y = height;
        CellularSettings::default(dim)
    }

    pub fn cellular_2d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
    ) -> CellularSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.offset_x = x_offset;
        dim.offset_y = y_offset;
        CellularSettings::default(dim)
    }

    pub fn cellular_3d(width: usize, height: usize, depth: usize) -> CellularSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
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
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.offset_x = x_offset;
        dim.offset_y = y_offset;
        dim.offset_z = z_offset;
        CellularSettings::default(dim)
    }

    pub fn cellular_1d(width: usize) -> CellularSettings {
        let mut dim = NoiseDimensions::default(1);
        dim.extent_x = width;
        CellularSettings::default(dim)
    }

    pub fn cellular_1d_offset(x_offset: f32, width: usize) -> CellularSettings {
        let mut dim = NoiseDimensions::default(1);
        dim.extent_x = width;
        dim.offset_x = x_offset;
        CellularSettings::default(dim)
    }

    pub fn cellular_4d(width: usize, height: usize, depth: usize, time: usize) -> CellularSettings {
        let mut dim = NoiseDimensions::default(4);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        CellularSettings::default(dim)
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
        let mut dim = NoiseDimensions::default(4);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        dim.offset_x = x_offset;
        dim.offset_y = y_offset;
        dim.offset_z = z_offset;
        dim.offset_w = w_offset;
        CellularSettings::default(dim)
    }

    /// Experimental: 5D/6D noise. See the crate-level docs on 5D/6D.
    pub fn cellular_5d(
        width: usize,
        height: usize,
        depth: usize,
        time: usize,
        extent5: usize,
    ) -> CellularSettings {
        let mut dim = NoiseDimensions::default(5);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        dim.extent_u = extent5;
        CellularSettings::default(dim)
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
        let mut dim = NoiseDimensions::default(5);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        dim.extent_u = extent5;
        dim.offset_x = x_offset;
        dim.offset_y = y_offset;
        dim.offset_z = z_offset;
        dim.offset_w = w_offset;
        dim.offset_u = u_offset;
        CellularSettings::default(dim)
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
        let mut dim = NoiseDimensions::default(6);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        dim.extent_u = extent5;
        dim.extent_v = extent6;
        CellularSettings::default(dim)
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
        let mut dim = NoiseDimensions::default(6);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        dim.extent_u = extent5;
        dim.extent_v = extent6;
        dim.offset_x = x_offset;
        dim.offset_y = y_offset;
        dim.offset_z = z_offset;
        dim.offset_w = w_offset;
        dim.offset_u = u_offset;
        dim.offset_v = v_offset;
        CellularSettings::default(dim)
    }

    pub fn cellular2_2d(width: usize, height: usize) -> Cellular2Settings {
        let mut dim = NoiseDimensions::default(2);
        dim.extent_x = width;
        dim.extent_y = height;
        Cellular2Settings::default(dim)
    }

    pub fn cellular2_2d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
    ) -> Cellular2Settings {
        let mut dim = NoiseDimensions::default(2);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.offset_x = x_offset;
        dim.offset_y = y_offset;
        Cellular2Settings::default(dim)
    }

    pub fn cellular2_3d(width: usize, height: usize, depth: usize) -> Cellular2Settings {
        let mut dim = NoiseDimensions::default(3);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
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
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.offset_x = x_offset;
        dim.offset_y = y_offset;
        dim.offset_z = z_offset;
        Cellular2Settings::default(dim)
    }

    pub fn cellular2_1d(width: usize) -> Cellular2Settings {
        let mut dim = NoiseDimensions::default(1);
        dim.extent_x = width;
        Cellular2Settings::default(dim)
    }

    pub fn cellular2_1d_offset(x_offset: f32, width: usize) -> Cellular2Settings {
        let mut dim = NoiseDimensions::default(1);
        dim.extent_x = width;
        dim.offset_x = x_offset;
        Cellular2Settings::default(dim)
    }

    pub fn cellular2_4d(
        width: usize,
        height: usize,
        depth: usize,
        time: usize,
    ) -> Cellular2Settings {
        let mut dim = NoiseDimensions::default(4);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        Cellular2Settings::default(dim)
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
        let mut dim = NoiseDimensions::default(4);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        dim.offset_x = x_offset;
        dim.offset_y = y_offset;
        dim.offset_z = z_offset;
        dim.offset_w = w_offset;
        Cellular2Settings::default(dim)
    }

    /// Experimental: 5D/6D noise. See the crate-level docs on 5D/6D.
    pub fn cellular2_5d(
        width: usize,
        height: usize,
        depth: usize,
        time: usize,
        extent5: usize,
    ) -> Cellular2Settings {
        let mut dim = NoiseDimensions::default(5);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        dim.extent_u = extent5;
        Cellular2Settings::default(dim)
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
        let mut dim = NoiseDimensions::default(5);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        dim.extent_u = extent5;
        dim.offset_x = x_offset;
        dim.offset_y = y_offset;
        dim.offset_z = z_offset;
        dim.offset_w = w_offset;
        dim.offset_u = u_offset;
        Cellular2Settings::default(dim)
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
        let mut dim = NoiseDimensions::default(6);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        dim.extent_u = extent5;
        dim.extent_v = extent6;
        Cellular2Settings::default(dim)
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
        let mut dim = NoiseDimensions::default(6);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        dim.extent_u = extent5;
        dim.extent_v = extent6;
        dim.offset_x = x_offset;
        dim.offset_y = y_offset;
        dim.offset_z = z_offset;
        dim.offset_w = w_offset;
        dim.offset_u = u_offset;
        dim.offset_v = v_offset;
        Cellular2Settings::default(dim)
    }

    pub fn fbm_1d(width: usize) -> FbmSettings {
        let mut dim = NoiseDimensions::default(1);
        dim.extent_x = width;
        FbmSettings::default(dim)
    }

    pub fn fbm_1d_offset(x_offset: f32, width: usize) -> FbmSettings {
        let mut dim = NoiseDimensions::default(1);
        dim.extent_x = width;
        dim.offset_x = x_offset;
        FbmSettings::default(dim)
    }

    pub fn fbm_2d(width: usize, height: usize) -> FbmSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.extent_x = width;
        dim.extent_y = height;
        FbmSettings::default(dim)
    }

    pub fn fbm_2d_offset(x_offset: f32, width: usize, y_offset: f32, height: usize) -> FbmSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.offset_x = x_offset;
        dim.offset_y = y_offset;
        FbmSettings::default(dim)
    }

    pub fn fbm_3d(width: usize, height: usize, depth: usize) -> FbmSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
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
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.offset_x = x_offset;
        dim.offset_y = y_offset;
        dim.offset_z = z_offset;
        FbmSettings::default(dim)
    }

    pub fn fbm_4d(width: usize, height: usize, depth: usize, time: usize) -> FbmSettings {
        let mut dim = NoiseDimensions::default(4);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        FbmSettings::default(dim)
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
        let mut dim = NoiseDimensions::default(4);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        dim.offset_x = x_offset;
        dim.offset_y = y_offset;
        dim.offset_z = z_offset;
        dim.offset_w = w_offset;
        FbmSettings::default(dim)
    }

    /// Experimental: 5D/6D noise. See the crate-level docs on 5D/6D.
    pub fn fbm_5d(
        width: usize,
        height: usize,
        depth: usize,
        time: usize,
        extent5: usize,
    ) -> FbmSettings {
        let mut dim = NoiseDimensions::default(5);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        dim.extent_u = extent5;
        FbmSettings::default(dim)
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
        let mut dim = NoiseDimensions::default(5);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        dim.extent_u = extent5;
        dim.offset_x = x_offset;
        dim.offset_y = y_offset;
        dim.offset_z = z_offset;
        dim.offset_w = w_offset;
        dim.offset_u = u_offset;
        FbmSettings::default(dim)
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
        let mut dim = NoiseDimensions::default(6);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        dim.extent_u = extent5;
        dim.extent_v = extent6;
        FbmSettings::default(dim)
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
        let mut dim = NoiseDimensions::default(6);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        dim.extent_u = extent5;
        dim.extent_v = extent6;
        dim.offset_x = x_offset;
        dim.offset_y = y_offset;
        dim.offset_z = z_offset;
        dim.offset_w = w_offset;
        dim.offset_u = u_offset;
        dim.offset_v = v_offset;
        FbmSettings::default(dim)
    }

    pub fn ridge_1d(width: usize) -> RidgeSettings {
        let mut dim = NoiseDimensions::default(1);
        dim.extent_x = width;
        RidgeSettings::default(dim)
    }

    pub fn ridge_1d_offset(x_offset: f32, width: usize) -> RidgeSettings {
        let mut dim = NoiseDimensions::default(1);
        dim.extent_x = width;
        dim.offset_x = x_offset;
        RidgeSettings::default(dim)
    }

    pub fn ridge_2d(width: usize, height: usize) -> RidgeSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.extent_x = width;
        dim.extent_y = height;
        RidgeSettings::default(dim)
    }

    pub fn ridge_2d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
    ) -> RidgeSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.offset_x = x_offset;
        dim.offset_y = y_offset;
        RidgeSettings::default(dim)
    }

    pub fn ridge_3d(width: usize, height: usize, depth: usize) -> RidgeSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
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
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.offset_x = x_offset;
        dim.offset_y = y_offset;
        dim.offset_z = z_offset;
        RidgeSettings::default(dim)
    }

    pub fn ridge_4d(width: usize, height: usize, depth: usize, time: usize) -> RidgeSettings {
        let mut dim = NoiseDimensions::default(4);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        RidgeSettings::default(dim)
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
        let mut dim = NoiseDimensions::default(4);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        dim.offset_x = x_offset;
        dim.offset_y = y_offset;
        dim.offset_z = z_offset;
        dim.offset_w = w_offset;
        RidgeSettings::default(dim)
    }

    /// Experimental: 5D/6D noise. See the crate-level docs on 5D/6D.
    pub fn ridge_5d(
        width: usize,
        height: usize,
        depth: usize,
        time: usize,
        extent5: usize,
    ) -> RidgeSettings {
        let mut dim = NoiseDimensions::default(5);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        dim.extent_u = extent5;
        RidgeSettings::default(dim)
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
        let mut dim = NoiseDimensions::default(5);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        dim.extent_u = extent5;
        dim.offset_x = x_offset;
        dim.offset_y = y_offset;
        dim.offset_z = z_offset;
        dim.offset_w = w_offset;
        dim.offset_u = u_offset;
        RidgeSettings::default(dim)
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
        let mut dim = NoiseDimensions::default(6);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        dim.extent_u = extent5;
        dim.extent_v = extent6;
        RidgeSettings::default(dim)
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
        let mut dim = NoiseDimensions::default(6);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        dim.extent_u = extent5;
        dim.extent_v = extent6;
        dim.offset_x = x_offset;
        dim.offset_y = y_offset;
        dim.offset_z = z_offset;
        dim.offset_w = w_offset;
        dim.offset_u = u_offset;
        dim.offset_v = v_offset;
        RidgeSettings::default(dim)
    }

    // Turbulence Builders
    pub fn turbulence_1d(width: usize) -> TurbulenceSettings {
        let mut dim = NoiseDimensions::default(1);
        dim.extent_x = width;
        TurbulenceSettings::default(dim)
    }

    pub fn turbulence_1d_offset(x_offset: f32, width: usize) -> TurbulenceSettings {
        let mut dim = NoiseDimensions::default(1);
        dim.extent_x = width;
        dim.offset_x = x_offset;
        TurbulenceSettings::default(dim)
    }

    pub fn turbulence_2d(width: usize, height: usize) -> TurbulenceSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.extent_x = width;
        dim.extent_y = height;
        TurbulenceSettings::default(dim)
    }

    pub fn turbulence_2d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
    ) -> TurbulenceSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.offset_x = x_offset;
        dim.offset_y = y_offset;
        TurbulenceSettings::default(dim)
    }

    pub fn turbulence_3d(width: usize, height: usize, depth: usize) -> TurbulenceSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
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
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.offset_x = x_offset;
        dim.offset_y = y_offset;
        dim.offset_z = z_offset;
        TurbulenceSettings::default(dim)
    }

    pub fn turbulence_4d(
        width: usize,
        height: usize,
        depth: usize,
        time: usize,
    ) -> TurbulenceSettings {
        let mut dim = NoiseDimensions::default(4);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        TurbulenceSettings::default(dim)
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
        let mut dim = NoiseDimensions::default(4);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        dim.offset_x = x_offset;
        dim.offset_y = y_offset;
        dim.offset_z = z_offset;
        dim.offset_w = w_offset;
        TurbulenceSettings::default(dim)
    }

    /// Experimental: 5D/6D noise. See the crate-level docs on 5D/6D.
    pub fn turbulence_5d(
        width: usize,
        height: usize,
        depth: usize,
        time: usize,
        extent5: usize,
    ) -> TurbulenceSettings {
        let mut dim = NoiseDimensions::default(5);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        dim.extent_u = extent5;
        TurbulenceSettings::default(dim)
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
        let mut dim = NoiseDimensions::default(5);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        dim.extent_u = extent5;
        dim.offset_x = x_offset;
        dim.offset_y = y_offset;
        dim.offset_z = z_offset;
        dim.offset_w = w_offset;
        dim.offset_u = u_offset;
        TurbulenceSettings::default(dim)
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
        let mut dim = NoiseDimensions::default(6);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        dim.extent_u = extent5;
        dim.extent_v = extent6;
        TurbulenceSettings::default(dim)
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
        let mut dim = NoiseDimensions::default(6);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        dim.extent_u = extent5;
        dim.extent_v = extent6;
        dim.offset_x = x_offset;
        dim.offset_y = y_offset;
        dim.offset_z = z_offset;
        dim.offset_w = w_offset;
        dim.offset_u = u_offset;
        dim.offset_v = v_offset;
        TurbulenceSettings::default(dim)
    }

    // Gradient Builders
    pub fn gradient_1d(width: usize) -> GradientSettings {
        let mut dim = NoiseDimensions::default(1);
        dim.extent_x = width;
        GradientSettings::default(dim)
    }

    pub fn gradient_1d_offset(x_offset: f32, width: usize) -> GradientSettings {
        let mut dim = NoiseDimensions::default(1);
        dim.extent_x = width;
        dim.offset_x = x_offset;
        GradientSettings::default(dim)
    }

    pub fn gradient_2d(width: usize, height: usize) -> GradientSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.extent_x = width;
        dim.extent_y = height;
        GradientSettings::default(dim)
    }

    pub fn gradient_2d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
    ) -> GradientSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.offset_x = x_offset;
        dim.offset_y = y_offset;
        GradientSettings::default(dim)
    }

    pub fn gradient_3d(width: usize, height: usize, depth: usize) -> GradientSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
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
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.offset_x = x_offset;
        dim.offset_y = y_offset;
        dim.offset_z = z_offset;
        GradientSettings::default(dim)
    }

    pub fn gradient_4d(width: usize, height: usize, depth: usize, time: usize) -> GradientSettings {
        let mut dim = NoiseDimensions::default(4);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        GradientSettings::default(dim)
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
        let mut dim = NoiseDimensions::default(4);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        dim.offset_x = x_offset;
        dim.offset_y = y_offset;
        dim.offset_z = z_offset;
        dim.offset_w = w_offset;
        GradientSettings::default(dim)
    }

    /// Experimental: 5D/6D noise. See the crate-level docs on 5D/6D.
    pub fn gradient_5d(
        width: usize,
        height: usize,
        depth: usize,
        time: usize,
        extent5: usize,
    ) -> GradientSettings {
        let mut dim = NoiseDimensions::default(5);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        dim.extent_u = extent5;
        GradientSettings::default(dim)
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
        let mut dim = NoiseDimensions::default(5);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        dim.extent_u = extent5;
        dim.offset_x = x_offset;
        dim.offset_y = y_offset;
        dim.offset_z = z_offset;
        dim.offset_w = w_offset;
        dim.offset_u = u_offset;
        GradientSettings::default(dim)
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
        let mut dim = NoiseDimensions::default(6);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        dim.extent_u = extent5;
        dim.extent_v = extent6;
        GradientSettings::default(dim)
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
        let mut dim = NoiseDimensions::default(6);
        dim.extent_x = width;
        dim.extent_y = height;
        dim.extent_z = depth;
        dim.extent_w = time;
        dim.extent_u = extent5;
        dim.extent_v = extent6;
        dim.offset_x = x_offset;
        dim.offset_y = y_offset;
        dim.offset_z = z_offset;
        dim.offset_w = w_offset;
        dim.offset_u = u_offset;
        dim.offset_v = v_offset;
        GradientSettings::default(dim)
    }
}
