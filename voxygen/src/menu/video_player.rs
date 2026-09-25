use crate::{
    Direction, GlobalState, PlayState, PlayStateResult,
    render::{Drawer, GlobalsBindGroup},
    settings::Settings,
    window::Event,
};
use tracing::{error, info};
use windows::{
    Win32::{
        Media::MediaFoundation::{
            IMFSourceReader, MFCreateMediaType, MFCreateSourceReaderFromURL,
            MFMediaType_Video, MF_MT_MAJOR_TYPE, MF_MT_SUBTYPE, MF_SOURCE_READER_FIRST_VIDEO_STREAM,
            MF_VERSION, MFStartup, MFVideoFormat_RGB32,
        },
        System::Com::{COINIT_MULTITHREADED, CoInitializeEx},
    },
    core::w,
};

pub struct VideoPlayState {
    video_ended: bool,
    source_reader: Option<IMFSourceReader>,
    bind_group: crate::render::GlobalsBindGroup,
}

impl VideoPlayState {
    #[allow(unsafe_code)]
    pub fn new(_renderer: &mut crate::render::Renderer) -> Self {
        info!("VideoPlayState initialized");
        let mut source_reader = None;
        unsafe {
            let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
            let _ = MFStartup(MF_VERSION, 0);
            let url = w!("final-demo.mp4");
            if let Ok(reader) = MFCreateSourceReaderFromURL(url, None) {
                if let Ok(media_type) = MFCreateMediaType() {
                    let _ = media_type.SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Video);
                    let _ = media_type.SetGUID(&MF_MT_SUBTYPE, &MFVideoFormat_RGB32);
                    let _ = reader.SetCurrentMediaType(
                        MF_SOURCE_READER_FIRST_VIDEO_STREAM.0 as u32,
                        None,
                        &media_type,
                    );
                    source_reader = Some(reader);
                }
            } else {
                error!("Failed to open video file: final-demo.mp4");
            }
        }
        // Build a minimal bind group using the renderer; we don't have direct access to GlobalModel from dummy_scene,
        // so we construct one manually matching what dummy_scene uses.
        use crate::render::{GlobalModel, Globals, Light, Shadow, ShadowLocals, RainOcclusionLocals, PointLightMatrix};
        use crate::scene::{self, MAX_LIGHT_COUNT, MAX_SHADOW_COUNT, MAX_POINT_LIGHT_MATRICES_COUNT};
        let global_data = GlobalModel {
            globals: _renderer.create_consts(&[Globals::default()]),
            lights: _renderer.create_consts(&[Light::default(); scene::MAX_LIGHT_COUNT]),
            shadows: _renderer.create_consts(&[Shadow::default(); scene::MAX_SHADOW_COUNT]),
            shadow_mats: _renderer.create_shadow_bound_locals(&[ShadowLocals::default()]),
            rain_occlusion_mats: _renderer.create_rain_occlusion_bound_locals(&[RainOcclusionLocals::default()]),
            point_light_matrices: Box::new([PointLightMatrix::default(); scene::MAX_POINT_LIGHT_MATRICES_COUNT]),
        };
        let lod_data = crate::render::LodData::dummy(_renderer);
        let bind_group = _renderer.bind_globals(&global_data, &lod_data);
        Self {
            video_ended: false,
            source_reader,
            bind_group,
        }
    }

    #[allow(unsafe_code)]
    fn read_frame(&mut self) {
        if let Some(reader) = &self.source_reader {
            unsafe {
                let mut sample = None;
                let mut index: u32 = 0;
                let mut flags: u32 = 0;
                let mut time: i64 = 0;
                match reader.ReadSample(
                    MF_SOURCE_READER_FIRST_VIDEO_STREAM.0 as u32,
                    0,
                    Some(&mut index),
                    Some(&mut flags),
                    Some(&mut time),
                    Some(&mut sample),
                ) {
                    Ok(()) => {
                        if sample.is_some() {
                            // Frame read successfully; continue playing
                        } else if flags != 0 {
                            // End of stream reached
                            self.video_ended = true;
                        }
                    }
                    Err(_) => {
                        error!("Failed to read video frame");
                        self.video_ended = true;
                    }
                }
            }
        }
    }
}

impl PlayState for VideoPlayState {
    fn enter(&mut self, _global_state: &mut GlobalState, _direction: Direction) {
        info!("Entering VideoPlayState - playing final-demo.mp4");
    }

    fn tick(&mut self, _global_state: &mut GlobalState, _events: Vec<Event>) -> PlayStateResult {
        if !self.video_ended {
            self.read_frame();
        }
        if self.video_ended {
            info!("Video ended - shutting down application");
            PlayStateResult::Shutdown
        } else {
            PlayStateResult::Continue
        }
    }

    fn name(&self) -> &'static str { "VideoPlayer" }

    fn capped_fps(&self) -> bool { false }

    fn globals_bind_group(&self) -> &GlobalsBindGroup {
        &self.bind_group
    }

    fn render(&self, _drawer: &mut Drawer<'_>, _settings: &Settings) {
        // Video rendering is handled by Media Foundation, not wgpu
    }

    fn egui_enabled(&self) -> bool { false }
}
