use skia_safe;

#[derive(Debug, Clone)]
pub struct PaintState<'a> {
  pub alpha: f32,
  pub composition_op: CompositionOp,
  pub fill_style: Pattern,
  pub stroke_style: Pattern,
  pub stroke_opts: StrokeOptions<'a>,
  pub font: Font,
  pub transform: Transform2D<f32>,
  pub shadow_offset_x: f64,
  pub shadow_offset_y: f64,
  pub shadow_blur: f64,
  pub shadow_color: Color,
}
