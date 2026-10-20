//! Triángulos de prueba del laboratorio, en coordenadas de pantalla (800x600,
//! `y` hacia abajo). Se dibujan con la matriz identidad, igual que el "primer
//! triángulo" de la clase.

use raylib::prelude::*;

use crate::vertex::Vertex;

pub struct TestCase {
    pub name: &'static str,
    pub vertices: Vec<Vertex>,
}

fn tri(points: [(f32, f32); 3], color: Vector3) -> Vec<Vertex> {
    points
        .iter()
        .map(|&(x, y)| Vertex::new_with_color(Vector3::new(x, y, 0.0), color))
        .collect()
}

pub fn test_cases() -> Vec<TestCase> {
    let large = tri([(60.0, 580.0), (260.0, 240.0), (500.0, 560.0)], Vector3::new(0.95, 0.75, 0.2));

    vec![
        TestCase {
            name: "pequeño",
            vertices: tri([(40.3, 40.7), (52.6, 44.1), (44.2, 54.9)], Vector3::new(0.9, 0.3, 0.3)),
        },
        TestCase {
            name: "grande",
            vertices: large.clone(),
        },
        TestCase {
            name: "borde horizontal",
            vertices: tri([(120.0, 60.5), (300.0, 60.5), (220.0, 180.5)], Vector3::new(0.3, 0.8, 0.4)),
        },
        TestCase {
            name: "borde vertical",
            vertices: tri([(340.5, 40.0), (340.5, 200.0), (470.0, 120.0)], Vector3::new(0.3, 0.6, 0.95)),
        },
        // Los mismos tres vértices de "grande" en orden inverso.
        TestCase {
            name: "orden inverso",
            vertices: large.iter().rev().cloned().collect(),
        },
        TestCase {
            name: "parcialmente fuera",
            vertices: tri([(600.0, 420.0), (900.0, 480.0), (700.0, 700.0)], Vector3::new(0.2, 0.85, 0.85)),
        },
        TestCase {
            name: "colineales",
            vertices: tri([(560.0, 300.0), (660.0, 250.0), (760.0, 200.0)], Vector3::new(1.0, 1.0, 1.0)),
        },
    ]
}

/// Todos los casos juntos en una sola secuencia de vértices.
pub fn test_scene() -> Vec<Vertex> {
    test_cases().into_iter().flat_map(|case| case.vertices).collect()
}
