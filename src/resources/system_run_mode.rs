pub enum SystemRunMode {
    Prediction,
    Reconciliation,
    // (render_timestamp, blending_factor)
    Interpolation(u64, f32),
}

pub struct CurrentSystemRunMode(pub SystemRunMode);
