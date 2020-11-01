use skia_safe::{
  path::FillType, BlendMode, Color, Data, EncodedImageFormat, Matrix, Paint, PaintStyle, Path,
  PathDirection, Point, Rect, Surface,
};
use std::f32::consts::PI;
use std::mem;

pub struct Canvas {
  pub(crate) width: i32,
  pub(crate) height: i32,
  path: Path,
  surface: Surface,
  paint: Paint,
}

impl Canvas {
  pub fn new(width: i32, height: i32) -> Canvas {
    let mut surface = Surface::new_raster_n32_premul((width, height)).expect("no surface!");
    let path = Path::new();
    let mut paint = Paint::default();
    paint.set_color(Color::BLACK);
    paint.set_anti_alias(true);
    paint.set_stroke_width(1.0);
    surface.canvas().clear(Color::WHITE);
    Canvas {
      width,
      height,
      surface,
      path,
      paint,
    }
  }

  #[inline]
  pub fn arc(
    &mut self,
    center_x: f32,
    center_y: f32,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    from_end: bool,
  ) {
    self.ellipse(
      (center_x, center_y),
      (radius, radius),
      0.0,
      start_angle,
      end_angle,
      from_end,
    )
  }

  #[inline]
  pub fn arc_to(&mut self, ctrl_x: f32, ctrl_y: f32, to_x: f32, to_y: f32, radius: f32) {
    self.scoot(ctrl_x, ctrl_y);
    self
      .path
      .arc_to_tangent((ctrl_x, ctrl_y), (to_x, to_y), radius);
  }

  #[inline]
  pub fn rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
    self
      .path
      .add_rect(Rect::from_xywh(x, y, w, h), Some((PathDirection::CW, 0)));
  }

  #[inline]
  pub fn clear_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
    let mut paint = Paint::default();
    paint.set_style(PaintStyle::Fill);
    paint.set_blend_mode(BlendMode::Clear);
    self
      .canvas()
      .draw_rect(&Rect::from_xywh(x, y, w, h), &paint);
  }

  #[inline]
  pub fn save(&mut self) {
    self.canvas().save();
  }

  #[inline]
  pub fn translate(&mut self, dx: f32, dy: f32) {
    self.canvas().translate((dx, dy));
  }

  #[inline]
  pub fn scale(&mut self, sx: f32, sy: f32) {
    self.canvas().scale((sx, sy));
  }

  #[inline]
  pub fn move_to(&mut self, x: f32, y: f32) {
    self.begin_path();
    self.path.move_to((x, y));
  }

  #[inline]
  pub fn line_to(&mut self, x: f32, y: f32) {
    self.scoot(x, y);
    self.path.line_to((x, y));
  }

  #[inline]
  pub fn quad_to(&mut self, cpx: f32, cpy: f32, x: f32, y: f32) {
    self.scoot(cpx, cpy);
    self.path.quad_to((cpx, cpy), (x, y));
  }

  #[inline]
  pub fn bezier_curve_to(&mut self, cp1x: f32, cp1y: f32, cp2x: f32, cp2y: f32, x: f32, y: f32) {
    self.scoot(cp1x, cp1y);
    self.path.cubic_to((cp1x, cp1y), (cp2x, cp2y), (x, y));
  }

  #[inline]
  pub fn close_path(&mut self) {
    self.path.close();
  }

  #[inline]
  pub fn begin_path(&mut self) {
    let new_path = Path::new();
    self.surface.canvas().draw_path(&self.path, &self.paint);
    let _ = mem::replace(&mut self.path, new_path);
  }

  #[inline]
  pub fn stroke(&mut self) {
    self.paint.set_style(PaintStyle::Stroke);
    self.surface.canvas().draw_path(&self.path, &self.paint);
  }

  #[inline]
  pub fn fill(&mut self, fill_type: FillType, path: Option<Path>) {
    self.paint.set_style(PaintStyle::Fill);
    self.surface.canvas().draw_path(&self.path, &self.paint);
  }

  #[inline]
  pub fn set_line_width(&mut self, width: f32) {
    self.paint.set_stroke_width(width);
  }

  #[inline]
  pub(crate) fn data(&mut self) -> Option<Data> {
    let image = self.surface.image_snapshot();
    image.encode_to_data(EncodedImageFormat::PNG)
  }

  #[inline]
  fn canvas(&mut self) -> &mut skia_safe::Canvas {
    self.surface.canvas()
  }

  #[inline]
  pub fn ellipse(
    &mut self,
    origin: impl Into<Point>,
    radii: impl Into<Point>,
    rotation: f32,
    start_angle: f32,
    end_angle: f32,
    ccw: bool,
  ) {
    let Point { x, y } = origin.into();
    let Point {
      x: x_radius,
      y: y_radius,
    } = radii.into();

    // based off of CanonicalizeAngle in Chrome
    let tau = 2.0 * PI;
    let mut new_start_angle = start_angle % tau;
    if new_start_angle < 0.0 {
      new_start_angle += tau;
    }
    let delta = new_start_angle - start_angle;
    let start_angle = new_start_angle;
    let mut end_angle = end_angle + delta;

    // Based off of AdjustEndAngle in Chrome.
    if !ccw && (end_angle - start_angle) >= tau {
      end_angle = start_angle + tau; // Draw complete ellipse
    } else if ccw && (start_angle - end_angle) >= tau {
      end_angle = start_angle - tau; // Draw complete ellipse
    } else if !ccw && start_angle > end_angle {
      end_angle = start_angle + (tau - (start_angle - end_angle) % tau);
    } else if ccw && start_angle < end_angle {
      end_angle = start_angle - (tau - (end_angle - start_angle) % tau);
    }

    // Based off of Chrome's implementation in
    // https://cs.chromium.org/chromium/src/third_party/blink/renderer/platform/graphics/path.cc
    // of note, can't use addArc or addOval because they close the arc, which
    // the spec says not to do (unless the user explicitly calls closePath).
    // This throws off points being in/out of the arc.
    let oval = Rect::new(x - x_radius, y - y_radius, x + x_radius, y + y_radius);
    let mut rotated = Matrix::new_identity();
    rotated
      .pre_translate((x, y))
      .pre_rotate(radians_to_degrees(rotation), None)
      .pre_translate((-x, -y));
    let unrotated = rotated.invert().unwrap();

    self.path.transform(&unrotated);

    // draw in 2 180 degree segments because trying to draw all 360 degrees at once
    // draws nothing.
    let sweep_deg = radians_to_degrees(end_angle - start_angle);
    let start_deg = radians_to_degrees(start_angle);
    if almost_equal(sweep_deg.abs(), 360.0) {
      let half_sweep = sweep_deg / 2.0;
      self.path.arc_to(oval, start_deg, half_sweep, false);
      self
        .path
        .arc_to(oval, start_deg + half_sweep, half_sweep, false);
    } else {
      self.path.arc_to(oval, start_deg, sweep_deg, false);
    }

    self.path.transform(&rotated);
  }

  #[inline]
  fn scoot(&mut self, x: f32, y: f32) {
    if self.path.is_empty() {
      self.path.move_to((x, y));
    }
  }
}

#[inline]
fn radians_to_degrees(rad: f32) -> f32 {
  (rad / PI) * 180.0
}

#[inline]
fn degrees_to_radians(deg: f32) -> f32 {
  (deg / 180.0) * PI
}

#[inline]
fn almost_equal(floata: f32, floatb: f32) -> bool {
  (floata - floatb).abs() < 0.00001
}
