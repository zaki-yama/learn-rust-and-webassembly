use std::{cell::RefCell, rc::Rc};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures::channel::oneshot::channel;
use std::sync::Mutex;
use wasm_bindgen::{convert::IntoWasmAbi, prelude::*};
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::browser::{self, LoopClosure};

pub async fn load_image(source: &str) -> Result<HtmlImageElement> {
    let image = browser::new_image()?;

    let (complete_tx, complete_rx) = channel::<Result<()>>();
    let success_tx = Rc::new(Mutex::new(Some(complete_tx)));
    let error_tx = Rc::clone(&success_tx);

    let success_callback = browser::closure_once(move || {
        if let Some(success_tx) = success_tx.lock().ok().and_then(|mut opt| opt.take()) {
            success_tx.send(Ok(()));
        }
    });
    let error_callback: Closure<dyn FnMut(JsValue)> = browser::closure_once(move |err| {
        if let Some(error_tx) = error_tx.lock().ok().and_then(|mut opt| opt.take()) {
            error_tx.send(Err(anyhow!("Error Loading Image: {:#?}", err)));
        }
    });

    image.set_onload(Some(success_callback.as_ref().unchecked_ref()));
    image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
    image.set_src("rhb.png");

    complete_rx.await??;
    Ok(image)
}

#[async_trait(?Send)]
pub trait Game {
    async fn initialize(&self) -> Result<Box<dyn Game>>;
    fn update(&mut self);
    fn draw(&self, context: &Renderer);
}

const FRAME_SIZE: f32 = 1.0 / 60.0 * 1000.0;
pub struct GameLoop {
    // 直前のフレームがリクエストされた時刻
    last_frame: f64,
    // 最後に描画してから累積した差分時間
    accumlated_delta: f32,
}

type SharedLoopClosure = Rc<RefCell<Option<LoopClosure>>>;

impl GameLoop {
    pub async fn start(game: impl Game + 'static) -> Result<()> {
        let mut game = game.initialize().await?;
        let mut game_loop = GameLoop {
            last_frame: browser::now()?,
            accumlated_delta: 0.0,
        };

        let renderer = Renderer {
            context: browser::context()?,
        };

        let f: SharedLoopClosure = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(browser::create_raf_closure(move |perf: f64| {
            // perf: request_animation_frame がコールバック関数を呼び出した時刻
            game_loop.accumlated_delta += (perf - game_loop.last_frame) as f32;
            while game_loop.accumlated_delta > FRAME_SIZE {
                game.update();
                game_loop.accumlated_delta -= FRAME_SIZE;
            }
            game_loop.last_frame = perf;
            game.draw(&renderer);

            browser::request_animation_frame(f.borrow().as_ref().unwrap());
        }));
        browser::request_animation_frame(
            g.borrow()
                .as_ref()
                .ok_or_else(|| anyhow!("GameLoop: Loop is None?"))?,
        )?;
        Ok(())
    }
}

pub struct Renderer {
    context: CanvasRenderingContext2d,
}

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Renderer {
    pub fn clear(&self, rect: &Rect) {
        self.context.clear_rect(
            rect.x.into(),
            rect.y.into(),
            rect.width.into(),
            rect.height.into(),
        );
    }

    pub fn draw_image(&self, image: &HtmlImageElement, frame: &Rect, destination: &Rect) {
        self.context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &image,
                frame.x.into(),
                frame.y.into(),
                frame.width.into(),
                frame.height.into(),
                destination.x.into(),
                destination.y.into(),
                destination.width.into(),
                destination.height.into(),
            )
            .expect("Drawing is throwing exceptions! Unrecoverable error.");
    }
}
