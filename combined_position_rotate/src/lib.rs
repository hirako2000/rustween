use wasm_bindgen::prelude::{wasm_bindgen, JsValue, Closure, JsCast};
use web_sys::{Element, HtmlElement, Document};
use std::cell::RefCell;
use std::rc::Rc;
use tween::{Tweener, SineInOut, BounceInOut};

const BOX_SIZE: i16 = 20;
const GRID_SIZE: i16 = 100;
const AXIS_LABEL_SIZE: i16 = 20;
const DEFAULT_FONT_STYLE: &str = "font-family: Raleway,HelveticaNeue,Helvetica,Arial,sans-serif;font-size: 1.2em;";

fn window() -> web_sys::Window {
  web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
  window()
      .request_animation_frame(f.as_ref().unchecked_ref())
      .expect("should register `requestAnimationFrame` OK");
}

fn update_square_position_rotation(square: &web_sys::Element, x: f32, r: f32) {
  //let style = format!("background: blue; width: {}px; height: {}px; transform: translateX({}px) rotate({}deg), -{}px);", BOX_SIZE, BOX_SIZE, x, r, BOX_SIZE / 2);
  let style = format!("background: blue; width: {}px; height: {}px; transform: translate({}px, -{}px) rotate({}deg);", BOX_SIZE, BOX_SIZE, x, BOX_SIZE / 2, r);

  square.set_attribute("style", &style).expect("error setting style attribute");
}

fn get_body() -> HtmlElement {
  let body = get_document().body().expect("document should have a body");
  return body;
}

fn get_document() -> Document {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  return document;
}

fn append_demo_code() {
  let title = get_document().create_element("p").expect("could not create element");
  title.set_text_content(Some("Position And Rotate Tween..."));
  title.set_attribute("style", "font-size: 1.3em;").expect("failed to set attribute to title");
  get_body().append_child(&title).expect("failed to add title to body");

  let code = get_document().create_element("pre").expect("could not get dom document");
  code.set_text_content(Some(demo_code()));
  code.set_attribute("style", "background: #11111110; padding: 0.225em;font-size: 0.8em;").expect("Could not set attribute to code element");
  get_body().append_child(&code).expect("cannot add code element to body");
}


fn demo_code() -> &'static str {
  return r#"
  use tween::{Tweener, SineInOut, BounceInOut};

  let mut x: f32 = 0.0;
  let mut r: f32 = 0.0;

  let square = append_square();

  let (p_start, p_end) = (0, 250);
  let (r_start, r_end) = (0, 360);
  let duration = 3.0; // in seconds
  let in_a_second = -1.0;
  
  let mut tweener_position = Tweener::new_at(p_start, p_end, duration, SineInOut, in_a_second);
  let mut tweener_rotation = Tweener::new_at(r_start, r_end, duration, BounceInOut, in_a_second);

  const DT: f32 = 1.0 / 60.0;

  // and then in your main loop...
  loop {

    if tweener_position.is_started() || tweener_rotation.is_started() { 
      status.set_text_content(Some("Tweening Running"));
    }

    if tweener_position.is_finished() && tweener_rotation.is_finished() {
      status.set_text_content(Some("Tweening Finished"));
      break;
    }

    x = tweener_position.move_by(DT) as f32;
    r = tweener_rotation.move_by(DT) as f32;

    update_square_position(&square, x as f32);
    update_square_rotation(&square, r as f32);
  }
"#;
}

fn append_home_link() {
  let link = get_document().create_element("a").expect("failed to get dom document");
  link.set_attribute("href", "/").expect("failed to set href attribute to link");
  link.set_text_content(Some("All Demos"));
  link.set_id("home");
  link.set_attribute("style", DEFAULT_FONT_STYLE).expect("failed to set style attribute to link");
  get_body().append_child(&link).expect("failed to append link to body");
}

fn appened_restart_link() {
  let document = get_document();
  let container =  document.create_element("div").expect("failed to create div element");
  let link = document.create_element("a").expect("failed to create a element");
  link.set_attribute("href", "/combined-position-rotate.html").expect("failed to set href attribute to link");
  link.set_text_content(Some("Restart"));
  link.set_attribute("style", DEFAULT_FONT_STYLE).expect("failed to set style attribute to link");

  container.set_attribute("style", "margin-top: 1em;").expect("Could not set style attribute to container");
  container.append_child(&link).expect("failed to appened link to container");
  get_body().append_child(&container).expect("failed to appened container to body");
}

fn append_square(x: f32) -> Element {

  let document = get_document();
  let body = get_body();

  let square = document.create_element("div").expect("failed to get dom document");
  square.set_attribute("style", &format!("transformX({}px)", x)).expect("failed to set style attribute to square");
  square.set_id("square");

  let container = document.create_element("div").expect("failed to create dom container");
  container.set_attribute("style", format!("background-image: linear-gradient(rgba(1, 1, 1, 0.1) .1em, transparent .1em), linear-gradient(90deg, rgba(1, 1, 1, 0.1) .1em, transparent .1em);
  background-size: {}px {}px; height: {}px; margin: {}px;", GRID_SIZE, GRID_SIZE, BOX_SIZE, BOX_SIZE).as_str()).expect("cannot set attribute to container");
  container.append_child(&square).expect("Failed to append square to container");
  body.append_child(&container).expect("Failed to append container to body");

  return square;
}

fn append_status() -> Element {
  let document = get_document();
  let body = get_body();

  let status = document.create_element("p").expect("could not create status element");
  status.set_attribute("style", DEFAULT_FONT_STYLE).expect("failed to set style attribute to status");
  status.set_text_content(Some("Tweening about to start"));
  body.append_child(&status).expect("could not append status to body");

  return status;
}

fn append_axis_labels() {
  append_axis_label(0);
  append_axis_label(1);
  append_axis_label(2);
  append_axis_label(3);
}

fn append_axis_label(number: u8) {
  let body = get_body();
  let document = get_document();

  let axis_label = document.create_element("span").expect("failed to create axis element");

  let margin_left: i32;
  let mut display = "0".to_string();

  if number == 0 {
      margin_left = (BOX_SIZE - 3) as i32;
  } else {
      margin_left = (GRID_SIZE - AXIS_LABEL_SIZE) as i32;
      display = format!("{}00", number);
  }

  axis_label.set_attribute("style", &axis_legend_gutted_style(margin_left)).expect("could not set attribute to axis label");
  axis_label.set_text_content(Some(&display));

  body.append_child(&axis_label).expect("failed to append axis label 000 to body");
}

fn axis_legend_gutted_style(margin: i32) -> String {
  return format!("margin-left: {}px; opacity: 0.7; font-family: monospace", margin);
}

fn cubic_out() -> Result<(), JsValue> {
    get_body().set_attribute("style", "margin: 1em").expect("failed to set style attribute to body");
    append_home_link();
    append_demo_code();

    let status = append_status();

    let mut x: f32 = 0.0;
    let mut r: f32 = 0.0;
    let square = append_square(x);

    append_axis_labels();

    let (p_start, p_end) = (0, 250);
    let (r_start, r_end) = (10, 370);
    let duration = 3.0; // in seconds
    let in_a_second = -1.0;

    let mut tweener_position = Tweener::new_at(p_start - (BOX_SIZE / 2) , p_end - (BOX_SIZE / 2), duration, SineInOut, in_a_second);
    let mut tweener_rotation = Tweener::new_at(r_start - (BOX_SIZE / 2) , r_end - (BOX_SIZE / 2), duration, BounceInOut, in_a_second);

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    const DT: f32 = 1.0 / 60.0 as f32;

    appened_restart_link();
    *g.borrow_mut() = Some(Closure::new(move || {

      if tweener_position.is_started() || tweener_rotation.is_started()  { 
        status.set_text_content(Some("Tweening Running"));
      }

      x = tweener_position.move_by(DT) as f32;
      r = tweener_rotation.move_by(DT) as f32;

      match tweener_position.is_finished() && tweener_rotation.is_finished() {
          true => {
            let _ = f.borrow_mut().take();
            status.set_text_content(Some("Tweening Finished"));
            return;
          }
          false => (),
      }

      update_square_position_rotation(&square, x as f32, r as f32);

      // Schedule another requestAnimationFrame callback.
      request_animation_frame(f.borrow().as_ref().unwrap());
    }));

  request_animation_frame(g.borrow().as_ref().unwrap());
  Ok(())
}

// Called by the JS entry point
#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
  return cubic_out();
}

