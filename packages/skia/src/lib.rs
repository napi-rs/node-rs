#[macro_use]
extern crate napi;
#[macro_use]
extern crate napi_derive;
#[macro_use]
extern crate serde_derive;

use canvas::Canvas;
use canvas_trait::Canvas2dMsg;
use napi::{CallContext, Error, JsBuffer, JsNumber, JsObject, JsUndefined, Module, Result, Status};
use std::convert::TryInto;

register_module!(canvas, init);

mod canvas;
mod canvas_trait;
mod paint_state;

const SEND_MSG_FN_NAME: &'static str = "sendMsg";
const TO_PNG_METHOD: &'static str = "toPNG";

fn init(module: &mut Module) -> Result<()> {
  module.create_named_method("createCanvas", create_canvas)?;
  Ok(())
}

#[js_function(2)]
fn create_canvas(ctx: CallContext) -> Result<JsObject> {
  let width: i32 = ctx.get::<JsNumber>(0)?.try_into()?;
  let height: i32 = ctx.get::<JsNumber>(1)?.try_into()?;
  let mut js_ctx = ctx.env.create_object()?;
  let send_msg_fn = ctx.env.create_function(SEND_MSG_FN_NAME, send_msg)?;
  let to_png_fn = ctx.env.create_function(TO_PNG_METHOD, get_png_data)?;
  js_ctx.set_named_property(SEND_MSG_FN_NAME, send_msg_fn)?;
  js_ctx.set_named_property(TO_PNG_METHOD, to_png_fn)?;
  ctx.env.wrap(&mut js_ctx, Canvas::new(width, height))?;
  Ok(js_ctx)
}

#[js_function(1)]
fn send_msg(ctx: CallContext<JsObject>) -> Result<JsUndefined> {
  let msg = ctx.get::<JsBuffer>(0)?;
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
      center_x,
      center_y,
      radius,
      start_angle,
      end_angle,
      from_end,
    } => {
      canvas.arc(center_x, center_y, radius, start_angle, end_angle, from_end);
    }
    Canvas2dMsg::ArcTo {
      ctrl_x,
      ctrl_y,
      to_x,
      to_y,
      radius,
    } => canvas.arc_to(ctrl_x, ctrl_y, to_x, to_y, radius),
    Canvas2dMsg::BeginPath => canvas.begin_path(),
    Canvas2dMsg::BezierCurveTo { ctrl0, ctrl1, to } => {
      canvas.bezier_curve_to(ctrl0.x, ctrl0.y, ctrl1.x, ctrl1.y, to.x, to.y)
    }
    Canvas2dMsg::ClearRect { x, y, w, h } => canvas.clear_rect(x, y, w, h),
    Canvas2dMsg::Clip { fillrule } => canvas.clip(fillrule.to_pf()),
    Canvas2dMsg::ClosePath => canvas.close_path(),
    Canvas2dMsg::Ellipse {
      center_x,
      center_y,
      radius_x,
      radius_y,
      rotation,
      start_angle,
      end_angle,
      anticlockwise,
    } => canvas.ellipse(
      (center_x, center_y),
      (radius_x, radius_y),
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
fn get_png_data(ctx: CallContext<JsObject>) -> Result<JsBuffer> {
  let canvas: &mut Canvas = ctx.env.unwrap(&ctx.this)?;
  let pixel_data = canvas.data().ok_or(Error::new(
    Status::GenericFailure,
    "Encode png data from Canvas failed".to_owned(),
  ))?;
  let data_len = pixel_data.len();
  let width = canvas.width;
  let height = canvas.height;

  let mut output = Vec::with_capacity(data_len);

  ctx.env.create_buffer_with_data(output)
}
