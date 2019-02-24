use crate::prelude::*;
use rust_skia::GrSurfaceOrigin;
use rust_skia::GrMipMapped;

pub type MipMapped = EnumHandle<GrMipMapped>;

#[allow(non_upper_case_globals)]
impl EnumHandle<GrMipMapped> {
    pub const No: Self = Self(GrMipMapped::kNo);
    pub const Yes: Self = Self(GrMipMapped::kYes);
}

pub type SurfaceOrigin = EnumHandle<GrSurfaceOrigin>;

#[allow(non_upper_case_globals)]
impl EnumHandle<GrSurfaceOrigin> {
    pub const TopLeft: Self = Self(GrSurfaceOrigin::kTopLeft_GrSurfaceOrigin);
    pub const BottomLeft: Self = Self(GrSurfaceOrigin::kBottomLeft_GrSurfaceOrigin);
}
