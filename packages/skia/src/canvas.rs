use euclid::default::{Point2D, Size2D};
use skia_safe::{Color, Data, EncodedImageFormat, Paint, PaintStyle, Path, Point, Rect, Surface};
use std::f32::consts::PI;
use std::mem;

pub struct Canvas {
  size: Size2D<i32>,
  path: Path,
  surface: Surface,
  paint: Paint,
}

impl Canvas {
  pub fn new(size: Size2D<i32>) -> Option<Self> {
    Surface::new_raster_n32_premul((size.width, size.height)).map(|surface| {
      let mut paint = Paint::default();
      paint.set_color(Color::WHITE);
      paint.set_anti_alias(true);
      paint.set_stroke_width(1.0);
      surface.canvas().clear(Color::WHITE);
      Canvas {
        size,
        surface,
        path: Path::new(),
        paint,
      }
    })
  }

  #[inline]
  pub fn get_size(&self) -> &Size2D<i32> {
    &self.size
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
    self.path.line_to((x, y));
  }

  #[inline]
  pub fn quad_to(&mut self, cpx: f32, cpy: f32, x: f32, y: f32) {
    self.path.quad_to((cpx, cpy), (x, y));
  }

  #[allow(dead_code)]
  #[inline]
  pub fn bezier_curve_to(&mut self, cp1x: f32, cp1y: f32, cp2x: f32, cp2y: f32, x: f32, y: f32) {
    self.path.cubic_to((cp1x, cp1y), (cp2x, cp2y), (x, y));
  }

  #[allow(dead_code)]
  #[inline]
  pub fn close_path(&mut self) {
    self.path.close();
  }

  #[inline]
  pub fn begin_path(&mut self) {
    let new_path = Path::new();
    self.surface.canvas().draw_path(&self.path, &self.paint);
    mem::replace(&mut self.path, new_path);
  }

  #[inline]
  pub fn stroke(&mut self) {
    self.paint.set_style(PaintStyle::Stroke);
    self.surface.canvas().draw_path(&self.path, &self.paint);
  }

  #[inline]
  pub fn fill(&mut self) {
    self.paint.set_style(PaintStyle::Fill);
    self.surface.canvas().draw_path(&self.path, &self.paint);
  }

  #[inline]
  pub fn set_line_width(&mut self, width: f32) {
    self.paint.set_stroke_width(width);
  }

  #[inline]
  pub fn data(&mut self) -> Data {
    let image = self.surface.image_snapshot();
    image.encode_to_data(EncodedImageFormat::PNG).unwrap()
  }

  #[inline]
  fn canvas(&mut self) -> &mut skia_safe::Canvas {
    self.surface.canvas()
  }

  #[inline]
  pub fn arc(
    &mut self,
    center: Point2D<f32>,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    from_end: bool,
  ) {
  }

  #[inline]
  pub fn arc_to(&mut self, ctrl: Point2D<f32>, to: Point2D<f32>, radius: f32) {
    self
      .path
      .arc_to_tangent(ctrl.into_skpoint(), to.into_skpoint(), radius);
  }

  #[inline]
  pub fn clear_rect(&mut self, rect: RectF) {
    self.surface.canvas().clear_rect(rect);
  }

  #[inline]
  pub fn clip(&mut self, rule: FillRule) {
    let path = mem::replace(&mut self.path, Path2D::new());
    self.ctx.clip_path(path, rule);
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
    ccw: bool,
  ) {
    let tao = 2.0 * PI;
    let new_start_angle = start_angle % tao;
    if new_start_angle < 0.0 {
      new_start_angle += tao;
    }
    let delta = new_start_angle - start_angle;
    start_angle = new_start_angle;
    end_angle += delta;

    // Based off of AdjustEndAngle in Chrome.
    if !ccw && (end_angle - start_angle) >= tao {
      // Draw complete ellipse
      end_angle = start_angle + tao;
    } else if ccw && (start_angle - end_angle) >= tao {
      // Draw complete ellipse
      end_angle = start_angle - tao;
    } else if !ccw && start_angle > end_angle {
      end_angle = start_angle + (tao - (start_angle - end_angle) % tao);
    } else if ccw && start_angle < end_angle {
      end_angle = start_angle - (tao - (end_angle - start_angle) % tao);
    }

    // Based off of Chrome's implementation in
    // https://cs.chromium.org/chromium/src/third_party/blink/renderer/platform/graphics/path.cc
    // of note, can't use addArc or addOval because they close the arc, which
    // the spec says not to do (unless the user explicitly calls closePath).
    // This throws off points being in/out of the arc.
    if rotation == 0.0 {
      self.ellipse_helper(center.x(), y, radiusX, radiusY, start_angle, end_angle);
    }
    let rotated = CanvasKit.SkMatrix.rotated(rotation, x, y);
    let rotatedInvert = CanvasKit.SkMatrix.rotated(-rotation, x, y);
    self.path.transform(rotatedInvert);
    self.ellipse_helper(x, y, radiusX, radiusY, start_angle, end_angle);
    self.path.transform(rotated);
  }

  #[inline]
  fn ellipse_helper(
    &self,
    x: f32,
    y: f32,
    radius_x: f32,
    radius_y: f32,
    start_angle: f32,
    end_angle: f32,
  ) {
    let sweep_degrees = radians_to_degrees(end_angle - start_angle);
    let start_degrees = radians_to_degrees(start_angle);

    let oval = Rect(x - radius_x, y - radius_y, x + radius_x, y + radius_y);

    // draw in 2 180 degree segments because trying to draw all 360 degrees at once
    // draws nothing.
    if almost_equal(sweep_degrees.abs(), 360.0) {
      let halfSweep = sweep_degrees / 2.0;
      self.path.arc_to(oval, start_degrees, halfSweep, false);
      self
        .path
        .arc_to(oval, start_degrees + halfSweep, halfSweep, false);
      return;
    }
    self.path.arc_to(oval, start_degrees, sweep_degrees, false);
  }

  #[inline]
  pub fn fill_text(&mut self, text: String, x: f32, y: f32, max_width: Option<f32>) {
    self.ctx.fill_text(text.as_str(), Size2D<i32>::new(x, y));
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
  pub fn read_pixels(&mut self) -> Result<Vec<u8>, ()> {
    unimplemented!();
  }
}

trait IntoSkPoint {
  fn into_skpoint(self) -> Point;
}

impl IntoSkPoint for Point2D<f32> {
  fn into_skpoint(self) -> Point {
    Point {
      x: self.x,
      y: self.y,
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
