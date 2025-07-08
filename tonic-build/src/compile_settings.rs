#[derive(Debug, Clone)]
pub(crate) struct CompileSettings {
    pub(crate) codec_path: String,
}

impl CompileSettings {
    const CODEC_PATH: &str = if cfg!(feature = "rkyv") {
        "tonic::codec::RkyvCodec"
    } else {
        "tonic::codec::ProstCodec"
    };
}

impl Default for CompileSettings {
    fn default() -> Self {
        Self {
            codec_path: Self::CODEC_PATH.to_string(),
        }
    }
}
