use euclid::default::Size2D;
use skia_safe::Canvas as SkCanvas;
use std::mem;

pub struct Canvas {
  size: Size2D<i32>,
  sk_canvas: SkCanvas,
}

impl Canvas {
  pub fn new(size: Size2D<i32>) -> Self {
    let sk_canvas = SkCanvas;
    Canvas { size, sk_canvas }
  }

  #[inline]
  pub fn get_size(&self) -> &Size2D<i32> {
    &self.size
  }

  #[inline]
  pub fn arc(
    &mut self,
    center: Size2D<f32>,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    from_end: bool,
  ) {
    self.path.arc(
      center,
      radius,
      start_angle,
      end_angle,
      if from_end {
        ArcDirection::CCW
      } else {
        ArcDirection::CW
      },
    );
  }

  #[inline]
  pub fn begin_path(&mut self) {
    self.path = Path2D::new();
  }

  #[inline]
  pub fn arc_to(&mut self, ctrl: Size2D<i32>, to: Size2D<i32>, radius: f32) {
    self.path.arc_to(ctrl, to, radius);
  }

  #[inline]
  pub fn move_to(&mut self, to: Size2D<i32>) {
    self.path.move_to(to);
  }

  #[inline]
  pub fn line_to(&mut self, to: Size2D<i32>) {
    self.path.line_to(to);
  }

  #[inline]
  pub fn bezier_curve_to(&mut self, ctrl0: Size2D<i32>, ctrl1: Size2D<i32>, to: Size2D<i32>) {
    self.path.bezier_curve_to(ctrl0, ctrl1, to);
  }

  #[inline]
  pub fn clear_rect(&mut self, rect: RectF) {
    self.ctx.clear_rect(rect);
  }

  #[inline]
  pub fn clip(&mut self, rule: FillRule) {
    let path = mem::replace(&mut self.path, Path2D::new());
    self.ctx.clip_path(path, rule);
  }

  #[inline]
  pub fn close_path(&mut self) {
    self.path.close_path();
  }

  #[inline]
  pub fn ellipse(
    &mut self,
    center: Size2D<i32>,
    radius_x: f32,
    radius_y: f32,
    rotation: f32,
    start_angle: f32,
    end_angle: f32,
    anticlockwise: bool,
  ) {
    self.path.ellipse(
      center,
      Size2D<i32>::new(radius_x, radius_y),
      rotation,
      if anticlockwise {
        end_angle
      } else {
        start_angle
      },
      if anticlockwise {
        start_angle
      } else {
        end_angle
      },
    );
  }

  #[inline]
  pub fn fill(&mut self, path: Option<Path2D>, rule: Option<FillRule>) {
    let self_path = mem::replace(&mut self.path, Path2D::new());
    self
      .ctx
      .fill_path(path.unwrap_or(self_path), rule.unwrap_or(FillRule::Winding));
  }

  #[inline]
  pub fn fill_text(&mut self, text: String, x: f32, y: f32, max_width: Option<f32>) {
    self.ctx.fill_text(text.as_str(), Size2D<i32>::new(x, y));
  }

  #[inline]
  pub fn set_line_width(&mut self, width: f32) {
    self.ctx.set_line_width(width);
  }

  #[inline]
  pub fn stroke_rect(&mut self, rect: RectF) {
    self.ctx.stroke_rect(rect);
  }

  #[inline]
  pub fn fill_rect(&mut self, rect: RectF) {
    self.ctx.fill_rect(rect);
  }

  #[inline]
  pub fn stroke(&mut self) {
    let path = mem::replace(&mut self.path, Path2D::new());
    self.ctx.stroke_path(path);
  }

  #[inline]
  pub fn quadratic_curve_to(&mut self, ctrl: Size2D<i32>, to: Size2D<i32>) {
    self.path.quadratic_curve_to(ctrl, to);
  }

  #[inline]
  pub fn read_pixels(&mut self) -> Result<Vec<u8>, ()> {
    let empty_ctx =
      CanvasRenderingContext2D::new(CanvasFontContext::new(SYSTEM_SOURCE.clone()), self.size);
    let ctx = mem::replace(&mut self.ctx, empty_ctx);
    let scene = SceneProxy::from_scene(ctx.into_scene(), RayonExecutor);
    scene.build_and_render(&mut self.renderer, BuildOptions::default());
    let viewport = RectI::new(Vector2I::default(), self.size.to_i32());
    let texture_data_receiver = self
      .renderer
      .device
      .read_pixels(&RenderTarget::Default, viewport);

    let pixels = match self
      .renderer
      .device
      .recv_texture_data(&texture_data_receiver)
    {
      TextureData::U8(pixels) => Ok(pixels),
      _ => {
        dbg!("Unexpected pixel format for default framebuffer!");
        Err(())
      }
    };

    self
      .device
      .destroy_context(&mut self.gl_context)
      .map_err(|e| {
        dbg!("Destroy gl context failed: {:?}", e);
      })?;

    pixels
  }
}
