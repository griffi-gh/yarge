use winit::{
  window::WindowBuilder,
  event_loop::{ControlFlow, EventLoop},
  dpi::LogicalSize,
  event::{Event, VirtualKeyCode},
  window::Window
};
use winit_input_helper::WinitInputHelper;
use pixels::{PixelsContext, Pixels, SurfaceTexture, Error as PixelsError, wgpu};
use egui::{ClippedMesh, Context as EguiCtx};
use egui_wgpu_backend::{RenderPass, ScreenDescriptor, BackendError};

const PKG_NAME: Option<&str> = option_env!("CARGO_PKG_NAME");

const WIDTH: u32 = 160;
const HEIGHT: u32 = 144;

#[derive(Default)]
struct GuiState {
  //TODO GuiState
}

struct Framework {
  state: GuiState,
  egui_ctx: EguiCtx,
  egui_state: egui_winit::State,
  screen_descriptor: ScreenDescriptor,
  rpass: RenderPass,
  paint_jobs: Vec<ClippedMesh>,
}
impl Framework {
  fn new(width: u32, height: u32, scale_factor: f32, pixels: &pixels::Pixels) -> Self {
    let egui_ctx = EguiCtx::default();
    let egui_state = egui_winit::State::from_pixels_per_point(
      pixels.device().limits().max_texture_dimension_2d as usize,
      scale_factor
    );
    let screen_descriptor = ScreenDescriptor {
      physical_width: width,
      physical_height: height,
      scale_factor
    };
    let rpass = RenderPass::new(
      pixels.device(), 
      pixels.render_texture_format(),
      0
    );
    let state = GuiState::default();
    Self {
      state,
      egui_ctx,
      egui_state,
      screen_descriptor,
      rpass,
      paint_jobs: Vec::<ClippedMesh>::new()
    }
  }

  pub fn ui(&mut self, _egui_ctx: &EguiCtx) {
    //TODO
  }

  pub fn handle_event(&mut self, event: &winit::event::WindowEvent) {
    self.egui_state.on_event(&self.egui_ctx, event);
  }
  pub fn resize(&mut self, width: u32, height: u32) {
    if width > 0 && height > 0 {
      self.screen_descriptor.physical_width = width;
      self.screen_descriptor.physical_height = height;
    }
  }
  pub fn scale_factor(&mut self, scale_factor: f64) {
    self.screen_descriptor.scale_factor = scale_factor as f32;
  }
  pub fn prepare(&mut self, window: &Window) {
    // Run the egui frame and create all paint jobs to prepare for rendering.
    let raw_input = self.egui_state.take_egui_input(window);
    let full_output = self.egui_ctx.run(raw_input, |egui_ctx| {
      //self.ui(egui_ctx);
    });

    self.egui_state.handle_platform_output(
      window, &self.egui_ctx, 
      full_output.platform_output
    );
    self.paint_jobs = self.egui_ctx.tessellate(full_output.shapes);
  }

  pub(crate) fn render(
      &mut self,
      encoder: &mut wgpu::CommandEncoder,
      render_target: &wgpu::TextureView,
      context: &PixelsContext,
  ) -> Result<(), BackendError> {
    // Upload all resources to the GPU.
    /*self.rpass.add_textures(
      &context.device,
      &context.queue, 
      &self.egui_ctx.fonts().font_image_delta().unwrap()
    );*/ //FIXME
    //self.rpass.update_user_textures(&context.device, &context.queue);
    self.rpass.update_buffers(
        &context.device,
        &context.queue,
        &self.paint_jobs,
        &self.screen_descriptor,
    );

    // Record all render passes.
    self.rpass.execute(
        encoder,
        render_target,
        &self.paint_jobs,
        &self.screen_descriptor,
        None,
    )
  }
}

pub fn init() -> Result<(), PixelsError> {
  let event_loop = EventLoop::new();
  let mut input = WinitInputHelper::new();
  let window = {
    let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
    WindowBuilder::new()
        .with_title(PKG_NAME.unwrap_or("open source gameboy emulator"))
        .with_inner_size(size)
        .with_min_inner_size(size)
        .build(&event_loop)
        .unwrap()
  };
  let (mut pixels, mut framework) = {
    let window_size = window.inner_size();
    let scale_factor = window.scale_factor() as f32;
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)?;
    let framework = Framework::new(window_size.width, window_size.height, scale_factor, &pixels);
    (pixels, framework)
  };

  event_loop.run(move |event, _, control_flow| {
    // Handle input events
    if input.update(&event) {
      // Close events
      if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
        *control_flow = ControlFlow::Exit;
        return;
      }

      if let Some(scale_factor) = input.scale_factor() {
        framework.scale_factor(scale_factor);
      }

      // Resize the window
      if let Some(size) = input.window_resized() {
        pixels.resize_surface(size.width, size.height);
      }

      window.request_redraw();
    }
    match event {
      Event::WindowEvent { event, .. } => {
        framework.handle_event(&event);
      }
      // Draw the current frame
      Event::RedrawRequested(_) => {
        // Prepare egui
        framework.prepare(&window);
        let render_result = pixels.render_with(|encoder, render_target, context| {
          context.scaling_renderer.render(encoder, render_target);
          framework.render(encoder, render_target, context)?;
          Ok(())
        });
        if render_result.is_err() {
          *control_flow = ControlFlow::Exit;
        }
        render_result.unwrap();
      }
      _ => (),
    }
  });
}