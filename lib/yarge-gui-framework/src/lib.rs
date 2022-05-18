pub use egui;
pub use pixels;
pub use winit;

use winit::{
  window::WindowBuilder,
  event_loop::{ControlFlow, EventLoop},
  dpi::LogicalSize,
  event::Event,
  window::Window,
};
pub use winit::{
  event::VirtualKeyCode,
  window::Icon
};
#[cfg(target_os = "windows")]
use winit::platform::windows::WindowExtWindows;

pub use winit_input_helper::WinitInputHelper;

use pixels::{
  PixelsContext,
  Pixels, 
  SurfaceTexture,
};
pub use pixels::wgpu;

use egui::{
  ClippedPrimitive,
  Context as EguiCtx,
  TexturesDelta
};

use egui_wgpu::{
  renderer::{RenderPass, ScreenDescriptor}
};

pub const PKG_NAME: Option<&str> = option_env!("CARGO_PKG_NAME");

pub type Dimensions<T> = (T, T);

#[allow(unused_variables)]
pub trait Gui {
  fn prepare(&mut self);
  fn render(&mut self, frame: &mut [u8]);
  fn gui(&mut self, ctx: &EguiCtx, size: Dimensions<f32>) -> bool;
  fn handle_input(&mut self, input: &WinitInputHelper) {}
}

struct Framework<T: Gui> {
  pub(crate) state: T,
  egui_ctx: EguiCtx,
  egui_state: egui_winit::State,
  screen_descriptor: ScreenDescriptor,
  rpass: RenderPass,
  paint_jobs: Vec<ClippedPrimitive>,
  texture_delta: Option<TexturesDelta>,
}
impl<T: Gui> Framework<T> {
  fn new(
    width: u32, height: u32, 
    scale_factor: f32, pixels: &pixels::Pixels,
    gui_state: T
  ) -> Self {
    let egui_ctx = EguiCtx::default();
    let egui_state = egui_winit::State::from_pixels_per_point(
      pixels.device().limits().max_texture_dimension_2d as usize,
      scale_factor
    );
    let screen_descriptor = ScreenDescriptor {
      size_in_pixels: [width, height],
      pixels_per_point: scale_factor
    };
    let rpass = RenderPass::new(
      pixels.device(), 
      pixels.render_texture_format(),
      1
    );
    Self {
      state: gui_state,
      egui_ctx,
      egui_state,
      screen_descriptor,
      rpass,
      paint_jobs: Vec::<ClippedPrimitive>::new(),
      texture_delta: None
    }
  }

  pub fn handle_event(&mut self, event: &winit::event::WindowEvent) {
    self.egui_state.on_event(&self.egui_ctx, event);
  }
  pub fn resize(&mut self, width: u32, height: u32) {
    if width > 0 && height > 0 {
      self.screen_descriptor.size_in_pixels = [width, height];
    }
  }
  pub fn scale_factor(&mut self, scale_factor: f64) {
    self.screen_descriptor.pixels_per_point = scale_factor as f32;
  }
  pub fn prepare(&mut self, window: &Window) -> bool {
    // Run the egui frame and create all paint jobs to prepare for rendering.
    let size: Dimensions<f32> = {
      let size = window.inner_size();
      (size.width as f32, size.height as f32)
    };
    let raw_input = self.egui_state.take_egui_input(window);
    let mut do_exit = false;
    let full_output = self.egui_ctx.run(raw_input, |egui_ctx| {
      do_exit = self.state.gui(egui_ctx, size);
    });
    self.egui_state.handle_platform_output(
      window, &self.egui_ctx, 
      full_output.platform_output
    );
    self.texture_delta = Some(full_output.textures_delta);
    self.paint_jobs = self.egui_ctx.tessellate(full_output.shapes);
    do_exit
  }

  pub(crate) fn render(
    &mut self,
    encoder: &mut wgpu::CommandEncoder,
    render_target: &wgpu::TextureView,
    context: &PixelsContext,
  ) {
    // Upload all resources to the GPU.
    let delta = self.texture_delta.take().unwrap();
    for (texture_id, ref image_delta) in delta.set {
      self.rpass.update_texture(
        &context.device,
        &context.queue,
        texture_id, 
        image_delta
      );
    }
    self.rpass.update_buffers(
      &context.device,
      &context.queue,
      &self.paint_jobs,
      &self.screen_descriptor,
    );
    self.rpass.execute(
      encoder,
      render_target,
      &self.paint_jobs,
      &self.screen_descriptor,
      None,
    );
    for ref id in delta.free {
      self.rpass.free_texture(id)
    }
  }

  pub fn handle_input(&mut self, input: &WinitInputHelper) {
    self.state.handle_input(input);
  }
}

#[allow(unused)]
pub struct InitProperties<'a> {
  pub size: (u32, u32),
  pub min_size: (u32, u32),
  pub pixels_resoltion: (u32, u32),
  pub title: &'a str,
  pub window_icon: Option<Icon>,
  pub taskbar_icon: Option<Icon>
}

pub fn init<T: 'static + Gui>(state: T, prop: InitProperties) {
  let event_loop = EventLoop::new();
  let mut input = WinitInputHelper::new();
  let window = {
    let size = LogicalSize::new(
      prop.size.0 as f64, 
      prop.size.1 as f64
    );
    let min_size = LogicalSize::new(
      prop.min_size.0 as f64,
      prop.min_size.1 as f64
    );
    WindowBuilder::new()
      .with_title(prop.title)
      .with_inner_size(size)
      .with_min_inner_size(min_size)
      .build(&event_loop)
      .unwrap()
  };
  window.set_window_icon(prop.window_icon);
  #[cfg(target_os = "windows")] {
    window.set_taskbar_icon(prop.taskbar_icon);
  }
  let (mut pixels, mut framework) = {
    let window_size = window.inner_size();
    let scale_factor = window.scale_factor() as f32;
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let pixels = Pixels::new(
      prop.pixels_resoltion.0, 
      prop.pixels_resoltion.1,
      surface_texture
    ).unwrap();
    let framework = Framework::new(
      window_size.width, window_size.height, 
      scale_factor, &pixels, state
    );
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
        framework.resize(size.width, size.height);
        pixels.resize_surface(size.width, size.height);
      }

      framework.handle_input(&input);
      window.request_redraw();
    }
    match event {
      Event::WindowEvent { event, .. } => {
        framework.handle_event(&event);
      }
      // Draw the current frame
      Event::RedrawRequested(_) => {
        framework.state.prepare();
        // Render
        framework.state.render(pixels.get_frame());
        // Prepare egui
        let exit_requested = framework.prepare(&window);
        if exit_requested {
          *control_flow = ControlFlow::Exit; 
        }
        let render_result = pixels.render_with(|encoder, render_target, context| {
          context.scaling_renderer.render(encoder, render_target);
          framework.render(encoder, render_target, context);
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
