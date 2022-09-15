use glam::{vec3, EulerRot, Mat4,Vec4};
use {egui_miniquad as egui_mq, miniquad as mq};

use std::time::{SystemTime, UNIX_EPOCH};
use once_cell::sync::{OnceCell,Lazy};
use std::{sync::Mutex};
use hrstopwatch::Stopwatch;

fn PIX1() -> &'static Vec<u8> {
    static INSTANCE: OnceCell<Vec<u8>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        vec![0;4*2048*1024]
    })
}
/*
fn ST() -> &'static Box<Stopwatch> {
    static INSTANCE: OnceCell<Box<Stopwatch>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let mut st = Stopwatch::start().unwrap();
        st.pause().unwrap();
        Box::new(st)
    })
}

static PIX1: Lazy<[u8;4*2048*1024]> = Lazy::new(|| {
[0;4*2048*1024]
});*/

struct Stage {
    egui_mq: egui_mq::EguiMq,
    offscreen_pipeline: mq::Pipeline,
    offscreen_bind: mq::Bindings,
    offscreen_pass: mq::RenderPass,
    rx: f32,
    ry: f32,
    set_font:bool,
    test_str:String,
    test_fps:u32,
    stopwatch:Stopwatch,
}
#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
}
#[repr(C)]
struct Vertex {
    pos: Vec2,
    uv: Vec2,
}
impl Stage {
    pub fn new(ctx: &mut mq::Context) -> Stage {
      //  let dpi_scale= ctx.dpi_scale();
        let (width, height) = ctx.screen_size();
      //  let (width, height) = (width*dpi_scale, height*dpi_scale);

        let color_img = mq::Texture::new_render_texture(
            ctx,
            mq::TextureParams {
                width: width as u32,
                height: height as u32,
                format: mq::TextureFormat::RGBA8,
                ..Default::default()
            },
        );
        let depth_img = mq::Texture::new_render_texture(
            ctx,
            mq::TextureParams {
                width: width as u32,
                height: height as u32,
                format: mq::TextureFormat::Depth,
                ..Default::default()
            },
        );

        let offscreen_pass = mq::RenderPass::new(ctx, color_img, depth_img);

        #[rustfmt::skip]
        let vertices: [Vertex; 4] = [
            Vertex { pos : Vec2 { x: -0.5, y: -0.5 }, uv: Vec2 { x: 0., y: 0. } },
            Vertex { pos : Vec2 { x:  0.5, y: -0.5 }, uv: Vec2 { x: 1., y: 0. } },
            Vertex { pos : Vec2 { x:  0.5, y:  0.5 }, uv: Vec2 { x: 1., y: 1. } },
            Vertex { pos : Vec2 { x: -0.5, y:  0.5 }, uv: Vec2 { x: 0., y: 1. } },
        ];
        let vertex_buffer = mq::graphics::Buffer::immutable(ctx, mq::graphics::BufferType::VertexBuffer, &vertices);

        #[rustfmt::skip]
        let indices: &[u16] = &[
            0, 1, 2,  0, 2, 3,
        ];

        let index_buffer = mq::Buffer::immutable(ctx, mq::BufferType::IndexBuffer, indices);


        let offscreen_shader = mq::Shader::new(
            ctx,
            offscreen_shader::VERTEX,
            offscreen_shader::FRAGMENT,
            offscreen_shader::meta(),
        )
        .unwrap();

        let offscreen_pipeline = mq::Pipeline::new(
            ctx,
            &[mq::BufferLayout::default()],
            &[
                mq::VertexAttribute::new("pos", mq::VertexFormat::Float2),
                mq::VertexAttribute::new("uv", mq::VertexFormat::Float2),
            ],
            offscreen_shader,
        );
        let offscreen_bind = mq::Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![],
        };
        Stage {
            egui_mq: egui_mq::EguiMq::new(ctx),
            offscreen_pipeline,
            offscreen_bind,
            offscreen_pass,
            rx: 0.,
            ry: 0.,
            set_font:false,
            test_str:"".to_string(),
            test_fps:0,
            stopwatch:Stopwatch::start().unwrap(),
        }
    }
    fn update_binding(&mut self, ctx: &mut mq::Context) {
        let pixels: [u8; 4 * 4 * 4] = [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00,
            0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        ];
        let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let rnd = since_the_epoch.as_secs() as i64 * 1000i64 + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as i64;
    for it in self.offscreen_bind.images.clone() {
        it.delete();
   }
        if rnd%2 == 0 {
 
        let texture = mq::graphics::Texture::from_rgba8(ctx, 4, 4, &pixels);
        self.offscreen_bind.images = vec![texture];
        } else {
        let tt = vec![0;2048*1024*4];
        let texture = mq::graphics::Texture::from_rgba8(ctx, 2048, 1024, tt.as_slice());
       // let texture = mq::graphics::Texture::from_rgba8(ctx, 4, 4, &pixels);
 
     //  self.offscreen_pass.delete(ctx);
       self.offscreen_bind.images = vec![texture];
        }
    }
}

impl mq::EventHandler for Stage {
    fn update(&mut self, _ctx: &mut mq::Context) {}

    fn draw(&mut self, ctx: &mut mq::Context) {

        if self.test_fps == 0 {
            self.stopwatch=Stopwatch::start().unwrap();
        }
        self.test_fps=self.test_fps+1;
        let dpi_scale= ctx.dpi_scale();
        let (width, height) = ctx.screen_size();
        let (width, height) = (width/dpi_scale, height/dpi_scale);
    //  println!("w={} h={}",width,height);

        // the offscreen pass, rendering an rotating, untextured cube into a render target image
        ctx.begin_pass(
            self.offscreen_pass,
            mq::PassAction::clear_color(1.0, 1.0, 1.0, 1.),
        );
        ctx.apply_pipeline(&self.offscreen_pipeline);
        self.update_binding(ctx);
        ctx.apply_bindings(&self.offscreen_bind);
        let t = mq::date::now();
        for i in 0..10 {
            let t = t + i as f64 * 0.3;

            ctx.apply_uniforms(&offscreen_shader::Uniforms {
                offset: (t.cos() as f32 * 0.5, (t * 3.).cos() as f32 * 0.5),
            });
            ctx.draw(0, 6, 1);
        }
        ctx.apply_viewport(0, 0, width as i32, height as i32);
        ctx.end_render_pass();

        // Extract texture from offscreen render pass
        let mq_texture = self.offscreen_pass.texture(ctx);
        // create egui TextureId from Miniquad GL texture Id
        let egui_texture_id = egui::TextureId::User(mq_texture.gl_internal_id() as u64);
        ctx.clear(Some((1., 1., 1., 1.)), None, None);
        ctx.begin_default_pass(mq::PassAction::clear_color(0.0, 0.0, 0.0, 1.0));
        //ctx.apply_viewport(0, 0, width as i32, height as i32);
        ctx.end_render_pass();

        // Run the UI code:
        self.egui_mq.run(ctx, |_mq_ctx, egui_ctx| {
            use egui::epaint::text::{FontDefinitions,FontFamily,FontData};
            if self.set_font == false {
                let mut fonts = FontDefinitions::default();
                //
                // Install my own font (maybe supporting non-latin characters):
                fonts.font_data.insert("my_font".to_owned(),
                FontData::from_static(include_bytes!("../SourceHanSerifCN-Regular.ttf"))); // .ttf and .otf supported
                
                // Put my font first (highest priority):
                fonts.families.get_mut(&FontFamily::Proportional).unwrap()
                .insert(0, "my_font".to_owned());
                
                // Put my font as last fallback for monospace:
            //  fonts.fonts_for_family.get_mut(&FontFamily::Monospace).unwrap()
                //   .push("my_font".to_owned());
                egui_ctx.set_fonts(fonts);
                self.set_font=true;
            }
            egui::CentralPanel::default().frame(egui::Frame::dark_canvas(&egui_ctx.style())).show(egui_ctx, |ui| {
                ui.image(egui_texture_id, egui::Vec2::new(width, height));
            });  
            egui::Window::new("王艺").vscroll(true).show(egui_ctx, |ui| {
        //        ui.image(egui_texture_id, egui::Vec2::new(10.0, 10.0));
                egui::TextEdit::multiline(&mut self.test_str)
            .hint_text("Type something!")
            .show(ui);

                ui.image(egui_texture_id, egui::Vec2::new(140.0, 140.0));
                /*
                ui.image(egui_texture_id, egui::Vec2::new(140.0, 140.0));
                ui.image(egui_texture_id, egui::Vec2::new(10.0, 10.0));
                ui.image(egui_texture_id, egui::Vec2::new(140.0, 140.0));
                ui.image(egui_texture_id, egui::Vec2::new(140.0, 140.0));
                ui.image(egui_texture_id, egui::Vec2::new(10.0, 10.0));
                ui.image(egui_texture_id, egui::Vec2::new(140.0, 140.0));
                ui.image(egui_texture_id, egui::Vec2::new(140.0, 140.0));
                */
                #[cfg(not(target_arch = "wasm32"))]
                {
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                }
            }
            );
        });

        // Draw things behind egui here

        self.egui_mq.draw(ctx);

        // Draw things in front of egui here

        ctx.commit_frame();
        if self.test_fps == 1000 {
            self.stopwatch.stop().unwrap();
            println!("fps={}", 1000/self.stopwatch.elapsed_seconds());
        }
    }

    fn mouse_motion_event(&mut self, _: &mut mq::Context, x: f32, y: f32) {
        self.egui_mq.mouse_motion_event(x, y);
    }

    fn mouse_wheel_event(&mut self, _: &mut mq::Context, dx: f32, dy: f32) {
        self.egui_mq.mouse_wheel_event(dx, dy);
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut mq::Context,
        mb: mq::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.egui_mq.mouse_button_down_event(ctx, mb, x, y);
        println!("mb={:?}",mb); println!("x={:?} {:?}",x, y);
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut mq::Context,
        mb: mq::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.egui_mq.mouse_button_up_event(ctx, mb, x, y);
    }

    fn char_event(
        &mut self,
        _ctx: &mut mq::Context,
        character: char,
        _keymods: mq::KeyMods,
        _repeat: bool,
    ) {
        self.egui_mq.char_event(character);
        println!("keymods={:?}",_keymods); println!("char={:?}",character);
    }

    fn key_down_event(
        &mut self,
        ctx: &mut mq::Context,
        keycode: mq::KeyCode,
        keymods: mq::KeyMods,
        _repeat: bool,
    ) {
        self.egui_mq.key_down_event(ctx, keycode, keymods);
    }

    fn key_up_event(&mut self, _ctx: &mut mq::Context, keycode: mq::KeyCode, keymods: mq::KeyMods) {
        self.egui_mq.key_up_event(keycode, keymods);
    }
}

fn main() {
    let conf = mq::conf::Conf {
        high_dpi: true,
       // window_width: 1024,
       // window_height: 768,
        fullscreen: true,
        ..Default::default()
    };
    mq::start(conf, |mut ctx|{ 
        let dpi_scale= ctx.dpi_scale();
        let (width, height) = ctx.screen_size();
        let (width, height) = (width*dpi_scale, height*dpi_scale);
        ctx.set_window_size(width as u32,height as u32);
        ctx.show_mouse(false);
        Box::new(Stage::new(&mut ctx))});
}

mod offscreen_shader {
    use glam::Vec4;
    use miniquad as mq;

//    uniform mat4 mvp;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 pos;
    attribute vec2 uv;
    uniform vec2 offset;
    varying lowp vec2 texcoord;

    
    void main() {
        gl_Position =  vec4(pos+offset,0,1);
        texcoord = uv;
    }
    "#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec2 texcoord;
    uniform sampler2D tex;
    void main() {
        gl_FragColor = texture2D(tex, texcoord);;
    }
    "#;

    pub fn meta() -> mq::ShaderMeta {
        mq::ShaderMeta {
            images: vec!["tex".to_string()],
            uniforms: mq::UniformBlockLayout {
                uniforms: vec![mq::UniformDesc::new("offset", mq::UniformType::Float2)],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
      //  pub mvp: glam::Mat4,
      pub offset: (f32,f32),
    }
}
