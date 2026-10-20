//! `--compare-triangles`: comprueba que ambos rellenos generan las mismas
//! posiciones, no solo una imagen parecida.

use std::collections::HashSet;

use crate::rasterizer::rasterize;
use crate::scene;
use crate::triangle::TriangleMode;
use crate::vertex::Vertex;
use crate::{assemble_triangles, identity_uniforms};

type Position = (i32, i32);

/// Posiciones generadas por un modo, ordenadas.
fn positions(triangles: &[[Vertex; 3]], mode: TriangleMode) -> Vec<Position> {
    let mut positions: Vec<Position> = rasterize(triangles, mode)
        .iter()
        .map(|fragment| (fragment.position.x as i32, fragment.position.y as i32))
        .collect();
    positions.sort_unstable();
    positions
}

/// Cuántas posiciones aparecen en `a` y no en `b`, y cuántas al revés.
fn difference(a: &[Position], b: &[Position]) -> (usize, usize) {
    let set_a: HashSet<_> = a.iter().copied().collect();
    let set_b: HashSet<_> = b.iter().copied().collect();
    (set_a.difference(&set_b).count(), set_b.difference(&set_a).count())
}

pub fn run() {
    let uniforms = identity_uniforms();

    println!(
        "{:<20} {:>12} {:>12} {:>12} {:>12} {:>10}",
        "caso", "baricéntrico", "scanline", "solo baric.", "solo scan.", "iguales"
    );

    let mut barycentric_by_case = Vec::new();

    for case in scene::test_cases() {
        let triangles = assemble_triangles(&uniforms, &case.vertices);

        let bary = positions(&triangles, TriangleMode::FilledBarycentric);
        let scan = positions(&triangles, TriangleMode::FilledScanline);
        let (only_bary, only_scan) = difference(&bary, &scan);

        println!(
            "{:<20} {:>12} {:>12} {:>12} {:>12} {:>10}",
            case.name,
            bary.len(),
            scan.len(),
            only_bary,
            only_scan,
            if bary == scan { "sí" } else { "NO" },
        );

        barycentric_by_case.push((case.name, bary));
    }

    // "orden inverso" usa los vértices de "grande" al revés: deben salir las
    // mismas posiciones.
    let find = |name: &str| {
        barycentric_by_case
            .iter()
            .find(|(case, _)| *case == name)
            .map(|(_, positions)| positions)
            .unwrap()
    };
    println!(
        "\n\"orden inverso\" genera las mismas posiciones que \"grande\": {}",
        if find("orden inverso") == find("grande") { "sí" } else { "NO" },
    );
}
