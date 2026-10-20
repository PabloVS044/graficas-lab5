//! `--benchmark-triangles`: mide solo la rasterización, sin ventana.
//!
//! Los vértices se procesan y se ensamblan antes de empezar a medir. Dentro
//! de la medición queda únicamente `rasterize`: la creación y el
//! almacenamiento de los fragmentos.

use std::hint::black_box;
use std::time::{Duration, Instant};

use crate::rasterizer::rasterize;
use crate::scene;
use crate::triangle::TriangleMode;
use crate::{assemble_triangles, identity_uniforms};

/// Ejecuciones de `rasterize` dentro de una medición.
const RUNS: u32 = 100;
/// Mediciones por algoritmo; se reporta la mediana.
const REPETITIONS: usize = 15;

pub fn run() {
    // Los mismos triángulos de prueba para ambos algoritmos.
    let triangles = assemble_triangles(&identity_uniforms(), &scene::test_scene());

    for mode in [TriangleMode::FilledBarycentric, TriangleMode::FilledScanline] {
        let fragment_count = rasterize(&triangles, mode).len();

        let mut times: Vec<Duration> = Vec::with_capacity(REPETITIONS);
        for _ in 0..REPETITIONS {
            let start = Instant::now();

            for _ in 0..RUNS {
                let fragments = rasterize(black_box(&triangles), mode);
                black_box(fragments);
            }

            times.push(start.elapsed());
        }

        times.sort_unstable();
        let median = times[times.len() / 2];

        println!(
            "{}: {} fragmentos, mediana de {} mediciones: {:?} por {} ejecuciones",
            mode.name(),
            fragment_count,
            REPETITIONS,
            median,
            RUNS,
        );
    }
}
