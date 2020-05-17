#[macro_use]
extern crate napi_rs as napi;
#[macro_use]
extern crate napi_rs_derive;
#[macro_use]
extern crate serde_derive;

use canvas::Canvas;
use canvas_trait::Canvas2dMsg;
use euclid::default::Size2D;
use napi::{Buffer, CallContext, Env, Error, Number, Object, Result, Status, Undefined, Value};
use std::convert::TryInto;

register_module!(node_rs_canvas, init);

mod canvas;
mod canvas_trait;

const SEND_MSG_FN_NAME: &'static str = "sendMsg";
const TO_PNG_METHOD: &'static str = "toPNG";

fn init(env: &Env, exports: &mut Value<Object>) -> Result<()> {
  exports.set_named_property(
    "createCanvas",
    env.create_function("createCanvas", create_canvas)?,
  )?;
  Ok(())
}

#[js_function(2)]
fn create_canvas(ctx: CallContext) -> Result<Value<Object>> {
  let width: i64 = ctx.get::<Number>(0)?.try_into()?;
  let height: i64 = ctx.get::<Number>(1)?.try_into()?;
  let mut js_ctx = ctx.env.create_object()?;
  let send_msg_fn = ctx.env.create_function(SEND_MSG_FN_NAME, send_msg)?;
  let to_png_fn = ctx.env.create_function(TO_PNG_METHOD, get_png_data)?;
  js_ctx.set_named_property(SEND_MSG_FN_NAME, send_msg_fn)?;
  js_ctx.set_named_property(TO_PNG_METHOD, to_png_fn)?;
  ctx.env.wrap(
    &mut js_ctx,
    Canvas::new(Size2D::new(width as i32, height as i32)),
  )?;
  Ok(js_ctx)
}

#[js_function(1)]
fn send_msg(ctx: CallContext<Object>) -> Result<Value<Undefined>> {
  let msg = ctx.get::<Buffer>(0)?;
  let msg: Canvas2dMsg = serde_json::from_slice(&msg).map_err(|e| {
    dbg!(
      "Parse message json error {:?}, raw string: {:?}",
      e,
      String::from_utf8((&msg).to_vec()).unwrap_or("None".to_owned())
    );
    Error::from_status(Status::InvalidArg)
  })?;
  let canvas: &mut Canvas = ctx.env.unwrap(&ctx.this)?;
  match msg {
    Canvas2dMsg::Arc {
      center,
      radius,
      start_angle,
      end_angle,
      from_end,
    } => {
      canvas.arc(center, radius, start_angle, end_angle, from_end);
    }
    Canvas2dMsg::ArcTo { ctrl, to, radius } => canvas.arc_to(ctrl, to, radius),
    Canvas2dMsg::BeginPath => canvas.begin_path(),
    Canvas2dMsg::BezierCurveTo { ctrl0, ctrl1, to } => {
      canvas.bezier_curve_to(ctrl0.x, ctrl0.y, ctrl1.x, ctrl1.y, to.x, to.y)
    }
    Canvas2dMsg::ClearRect { rect } => canvas.clear_rect(rect),
    Canvas2dMsg::Clip { fillrule } => canvas.clip(fillrule.to_pf()),
    Canvas2dMsg::ClosePath => canvas.close_path(),
    Canvas2dMsg::Ellipse {
      center,
      radius_x,
      radius_y,
      rotation,
      start_angle,
      end_angle,
      anticlockwise,
    } => canvas.ellipse(
      center,
      radius_x,
      radius_y,
      rotation,
      start_angle,
      end_angle,
      anticlockwise,
    ),
    Canvas2dMsg::Fill { fillrule } => canvas.fill(None, Some(fillrule.to_pf())),
    Canvas2dMsg::FillText {
      text,
      x,
      y,
      max_width,
    } => canvas.fill_text(text, x, y, max_width),
    Canvas2dMsg::MoveTo { to } => canvas.move_to(to.x, to.y),
    Canvas2dMsg::LineTo { to } => canvas.line_to(to.x, to.y),
    Canvas2dMsg::SetLineWidth { width } => canvas.set_line_width(width),
    Canvas2dMsg::StrokeRect { rect } => canvas.stroke_rect(rect),
    Canvas2dMsg::FillRect { rect } => canvas.fill_rect(rect),
    Canvas2dMsg::Stroke => canvas.stroke(),
    Canvas2dMsg::QuadraticCurveTo { ctrl, to } => canvas.quad_to(ctrl.x, ctrl.y, to.x, to.y),
    _ => {
      dbg!("Unsupport canvas method");
      return Err(Error::from_status(Status::InvalidArg));
    }
  }
  ctx.env.get_undefined()
}

#[js_function]
fn get_png_data(ctx: CallContext<Object>) -> Result<Value<Buffer>> {
  let canvas: &mut Canvas = ctx.env.unwrap(&ctx.this)?;
  let pixel_data = canvas
    .read_pixels()
    .map_err(|_| Error::from_status(Status::GenericFailure))?;
  let data_len = pixel_data.len();
  let width = canvas.get_size().x();
  let height = canvas.get_size().y();

  let mut output = Vec::with_capacity(data_len);

  ctx.env.create_buffer_with_data(output)
}
