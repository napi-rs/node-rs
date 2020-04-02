#[macro_use]
extern crate napi_rs as napi;
#[macro_use]
extern crate serde_derive;

use canvas::Canvas;
use canvas_trait::Canvas2dMsg;
use euclid::default::Size2D;
use napi::{Any, Env, Error, Object, Result, Status, Value};
use std::ops::Deref;

register_module!(node_rs_canvas, init);

mod canvas;
mod canvas_trait;

const SEND_MSG_FN_NAME: &'static str = "sendMsg";
const TO_PNG_METHOD: &'static str = "toPNG";

fn init<'env>(
  env: &'env Env,
  exports: &'env mut Value<'env, Object>,
) -> Result<Option<Value<'env, Object>>> {
  exports.set_named_property(
    "createCanvas",
    env.create_function("createCanvas", callback!(create_canvas)),
  )?;
  Ok(None)
}

fn create_canvas<'a>(
  env: &'a Env,
  _this: Value<'a, Any>,
  args: &[Value<'a, Any>],
) -> Result<Option<Value<'a, Any>>> {
  let width: i64 = args
    .get(0)
    .and_then(|v| v.coerce_to_number().ok())
    .map(|v| v.into())
    .ok_or_else(|| Error::new(Status::NumberExpected))?;
  let height: i64 = args
    .get(1)
    .and_then(|v| v.coerce_to_number().ok())
    .map(|v| v.into())
    .ok_or_else(|| Error::new(Status::NumberExpected))?;
  let mut js_ctx = env.create_object();
  let send_msg_fn = env.create_function(SEND_MSG_FN_NAME, callback!(send_msg));
  let to_png_fn = env.create_function(TO_PNG_METHOD, callback!(get_png_data));
  js_ctx.set_named_property(SEND_MSG_FN_NAME, send_msg_fn)?;
  js_ctx.set_named_property(TO_PNG_METHOD, to_png_fn)?;
  env.wrap(
    &mut js_ctx,
    Canvas::new(Size2D::new(width as i32, height as i32)),
  )?;
  js_ctx.try_into().map(|v| Some(v))
}

fn send_msg<'a>(
  env: &'a Env,
  this: Value<'a, Any>,
  args: &[Value<'a, Any>],
) -> Result<Option<Value<'a, Any>>> {
  let msg = args
    .get(0)
    .map(|v| Value::from_value(env, v))
    .ok_or_else(|| Error::new(Status::InvalidArg))?;
  let buf = msg.deref();
  let msg: Canvas2dMsg = serde_json::from_slice(buf).map_err(|e| {
    dbg!(
      "Parse message json error {:?}, raw string: {:?}",
      e,
      String::from_utf8(buf.to_vec()).unwrap_or("None".to_owned())
    );
    Error::new(Status::InvalidArg)
  })?;
  let this_obj: Value<'a, Object> = this.coerce_to_object()?;
  let canvas: &mut Canvas = env.unwrap(&this_obj)?;
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
    Canvas2dMsg::BezierCurveTo { ctrl0, ctrl1, to } => canvas.bezier_curve_to(ctrl0, ctrl1, to),
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
    Canvas2dMsg::MoveTo { to } => canvas.move_to(to),
    Canvas2dMsg::LineTo { to } => canvas.line_to(to),
    Canvas2dMsg::SetLineWidth { width } => canvas.set_line_width(width),
    Canvas2dMsg::StrokeRect { rect } => canvas.stroke_rect(rect),
    Canvas2dMsg::FillRect { rect } => canvas.fill_rect(rect),
    Canvas2dMsg::Stroke => canvas.stroke(),
    Canvas2dMsg::QuadraticCurveTo { ctrl, to } => canvas.quadratic_curve_to(ctrl, to),
    _ => {
      dbg!("Unsupport canvas method");
      return Err(Error::new(Status::InvalidArg));
    }
  }
  env.get_undefined().try_into().map(|v| Some(v))
}

#[inline]
fn get_png_data<'a>(
  env: &'a Env,
  this: Value<'a, Any>,
  _args: &[Value<'a, Any>],
) -> Result<Option<Value<'a, Any>>> {
  let this_obj: Value<'a, Object> = this.coerce_to_object()?;
  let canvas: &mut Canvas = env.unwrap(&this_obj)?;
  let pixel_data = canvas
    .read_pixels()
    .map_err(|_| Error::new(Status::GenericFailure))?;
  let data_len = pixel_data.len();
  let width = canvas.get_size().x();
  let height = canvas.get_size().y();

  let mut output = Vec::with_capacity(data_len);

  let return_value = env
    .create_buffer_with_data(output)
    .try_into()
    .map(|v| Some(v));
  return_value
}
