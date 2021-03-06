extern crate piston_window;

use piston_window::*;

// useful when drawing a structure by drawing each of its fields
pub trait Draw {
    fn on_draw(
        self: &mut Self,
        centre: math::Matrix2d,
        time: f64,
        graphics: &mut G2d,
    );
}

pub trait DrawAdvanced {
    fn on_draw(
        self: &mut Self,
        context: Context,
        args: RenderArgs,
        graphics: &mut G2d,
    );
}

impl<G> DrawAdvanced for G where G: Draw {
    fn on_draw(
        self: &mut Self,
        context: Context,
        args: RenderArgs,
        graphics: &mut G2d,
    ) {
        let centre = get_screen_centre(&context, &args);
        Draw::on_draw(self, centre, args.ext_dt, graphics);
    }
}

pub fn get_screen_centre(
    context: &Context,
    ren: &RenderArgs,
) -> math::Matrix2d {
    let corner = context.transform;
    Transformed::trans(corner, ren.width / 2.0, ren.height / 2.0)
}

pub trait App: DrawAdvanced {
    fn on_update(
        self: &mut Self,
        args: UpdateArgs,
    );
    fn on_input(
        self: &mut Self,
        args: ButtonArgs,
    );
    fn on_mouse_move(
        self: &mut Self,
        mouse: [f64; 2],
    );

    fn window_name() -> &'static str;
    fn window_starting_size() -> [u32; 2];
}


// similar to https://github.com/PistonDevelopers/piston-examples/issues/336
fn build_window<A, W>() -> PistonWindow<W>
    where A: App,
          W: Window + BuildFromWindowSettings + OpenGLWindow,
{
    let title = A::window_name();
    let resolution = A::window_starting_size();

    let opengl = OpenGL::V3_2;
    let window_settings = WindowSettings::new(title, resolution)
        .exit_on_esc(true)
        .opengl(opengl);

    let first_attempt = window_settings
        .clone()
        .srgb(true)       // try to init windowbuilder with srgb enabled
        .build();
    let second_attempt = first_attempt
        .or_else(|err1| window_settings
            .srgb(false)  // then without
            .build()
            .map_err(|err2| [err1, err2])
        );
    let window_inner = second_attempt.unwrap_or_else(|errs| {
        panic!("Failed to build PistonWindow: {:?}", errs);
    });

    PistonWindow::new(opengl, 0, window_inner)
}

pub fn run_until_escape<A>(mut app: A)
    where A: App
{
    let mut window: PistonWindow = build_window::<A, _>();

    while let Some(e) = window.next() {
        if let Some(ren) = e.render_args() {
            window.draw_2d(&e, |c, g| app.on_draw(c, ren, g));
        }
        if let Some(upd) = e.update_args() {
            app.on_update(upd);
        }
        if let Some(bin) = e.button_args() {
            app.on_input(bin);
        }
        if let Some(mouse) = e.mouse_cursor_args() {
            app.on_mouse_move(mouse);
        }
    }
}
