// main.rs

mod framebuffer;
mod triangle;
mod line;
mod vertex;
mod fragment;
mod shaders;
mod primitive;
mod rasterizer;
mod barycentric;
mod scanline;
mod scene;
mod benchmark;
mod compare;

use framebuffer::Framebuffer;
use vertex::Vertex;
use triangle::TriangleMode;
use shaders::{vertex_shader, fragment_shader};
use primitive::primitive_assembly;
use rasterizer::rasterize;
use raylib::prelude::*;

pub struct Uniforms {
    pub model_matrix: Matrix,
}

pub fn identity_uniforms() -> Uniforms {
    Uniforms {
        model_matrix: Matrix::identity(),
    }
}

/// Etapas 1 y 2 del pipeline. Se separan de `render` para que la comparación
/// y la medición rastericen los mismos triángulos que se dibujan.
pub fn assemble_triangles(uniforms: &Uniforms, vertex_array: &[Vertex]) -> Vec<[Vertex; 3]> {
    // 1. Procesamiento de vértices
    let transformed_vertices: Vec<Vertex> = vertex_array
        .iter()
        .map(|vertex| vertex_shader(vertex, uniforms))
        .collect();

    // 2. Ensamblaje de primitivas
    primitive_assembly(&transformed_vertices)
}

fn render(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
    mode: TriangleMode,
) {
    // 1. Procesamiento de vértices
    // 2. Ensamblaje de primitivas
    let triangles = assemble_triangles(uniforms, vertex_array);

    // 3. Rasterización
    let fragments = rasterize(&triangles, mode);

    // 4. Procesamiento de fragmentos
    for fragment in fragments {
        let color = fragment_shader(&fragment);

        framebuffer.point(
            fragment.position.x as i32,
            fragment.position.y as i32,
            color,
        );
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let has_flag = |flag: &str| args.iter().any(|arg| arg == flag);

    if has_flag("--benchmark-triangles") {
        benchmark::run();
        return;
    }
    if has_flag("--compare-triangles") {
        compare::run();
        return;
    }
    if has_flag("--export-images") {
        export_images();
        return;
    }

    let (mut window, thread) = raylib::init()
        .size(800, 600)
        .title("Filling triangles")
        .build();

    window.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(800, 600);
    framebuffer.set_background_color(Vector3::new(0.0, 0.0, 0.0));
    framebuffer.init_texture(&mut window, &thread);

    let vertex_array = scene::test_scene();
    let uniforms = identity_uniforms();
    let mut mode = TriangleMode::Wireframe;

    while !window.window_should_close() {
        if window.is_key_pressed(KeyboardKey::KEY_ONE) {
            mode = TriangleMode::Wireframe;
        }
        if window.is_key_pressed(KeyboardKey::KEY_TWO) {
            mode = TriangleMode::FilledBarycentric;
        }
        if window.is_key_pressed(KeyboardKey::KEY_THREE) {
            mode = TriangleMode::FilledScanline;
        }

        framebuffer.clear();
        render(&mut framebuffer, &uniforms, &vertex_array, mode);
        framebuffer.swap_buffers(&mut window, &thread);
    }
}

/// Guarda una imagen de cada modo en `docs/`, sin abrir la ventana.
fn export_images() {
    std::fs::create_dir_all("docs").expect("No se pudo crear docs/");

    let vertex_array = scene::test_scene();
    let uniforms = identity_uniforms();

    for mode in TriangleMode::ALL {
        let mut framebuffer = Framebuffer::new(800, 600);
        framebuffer.set_background_color(Vector3::new(0.0, 0.0, 0.0));
        framebuffer.clear();

        render(&mut framebuffer, &uniforms, &vertex_array, mode);
        framebuffer.export(&format!("docs/{}.png", mode.name()));
    }
}
