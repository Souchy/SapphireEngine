use std::{fs, path::Path};

use glium::{uniform, Surface};



pub fn gli() {
    // 1. The **winit::EventLoop** for handling events.
    let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
    // 2. Create a glutin context and glium Display
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    let image = image::load(
        std::io::Cursor::new(&include_bytes!("../assets/blue.png")),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    let texture = glium::texture::Texture2d::new(&display, image).unwrap();


    // 4. Define the vertex buffer (a rectangle to map the texture onto)
    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        tex_coords: [f32; 2],
    }
    glium::implement_vertex!(Vertex, position, tex_coords);
    let shape = vec![
        Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] },
        Vertex { position: [ 0.5, -0.5], tex_coords: [1.0, 0.0] },
        Vertex { position: [ 0.5,  0.5], tex_coords: [1.0, 1.0] },

        Vertex { position: [ 0.5,  0.5], tex_coords: [1.0, 1.0] },
        Vertex { position: [-0.5,  0.5], tex_coords: [0.0, 1.0] },
        Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] },
    ];
    
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    let vertex_path = Path::new("src/shaders/vertex.glsl");
    let fragment_path = Path::new("src/shaders/fragment.glsl");
    let vertex_shader_src = fs::read_to_string(vertex_path).unwrap();
    let fragment_shader_src = fs::read_to_string(fragment_path).unwrap();
    let program = glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap();

    
    let mut t:f32 = 0.0;
    
    #[allow(deprecated)]
    event_loop.run(move |ev, window_target| {
        match ev {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => {
                    window_target.exit();
                },
                // We now need to render everyting in response to a RedrawRequested event due to the animation
                glium::winit::event::WindowEvent::RedrawRequested => {
                    // we update `t`
                    t += 0.02;
                    let x = t.sin() * 0.5;

                    let mut target = display.draw();
                    target.clear_color(0.0, 0.0, 0.0, 1.0);

                    let uniforms = uniform! {
                        matrix: [
                            [1.0, 0.0, 0.0, 0.0],
                            [0.0, 1.0, 0.0, 0.0],
                            [0.0, 0.0, 1.0, 0.0],
                            [ x , 0.0, 0.0, 1.0f32],
                        ],
                        tex: &texture,
                    };

                    target.draw(&vertex_buffer, &indices, &program, &uniforms,
                                &Default::default()).unwrap();
                    target.finish().unwrap();
                },
                // Because glium doesn't know about windows we need to resize the display
                // when the window's size has changed.
                glium::winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                },
                _ => (),
            },
            // By requesting a redraw in response to a AboutToWait event we get continuous rendering.
            // For applications that only change due to user input you could remove this handler.
            glium::winit::event::Event::AboutToWait => {
                window.request_redraw();
            },
            _ => (),
        }
    })
    .unwrap();
}
