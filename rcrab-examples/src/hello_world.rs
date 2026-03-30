// Hello World example - Basic application setup

use rcrab_core::scene::{Camera, Geometry, MeshBuilder, Node};
use rcrab_core::app::AppSettings;
use rcrab_core::math::Vec3;
use std::sync::Arc;

fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    tracing::info!("Starting rCrabEngine Hello World Example");

    // Create application settings
    let settings = AppSettings::new();
    settings.set("Title", "rCrabEngine - Hello World");
    settings.set("Width", 1280);
    settings.set("Height", 720);

    // Create scene
    let root = Node::new("Root");

    // Create camera
    let camera = Arc::new(Camera::new(1280, 720));
    camera.set_location(Vec3::new(0.0, 2.0, 10.0));
    camera.look_at(Vec3::ZERO, Vec3::Y);

    // Create a simple box geometry
    let mesh = MeshBuilder::create_box(2.0, 2.0, 2.0);
    let geometry = Arc::new(Geometry::new_with_mesh("Box", mesh));

    // Set position
    geometry.set_position(Vec3::new(0.0, 0.0, 0.0));

    // Attach to root using Node's attach_child
    root.attach_child(geometry.clone());

    tracing::info!("Scene created successfully");
    tracing::info!("Root has {} children", root.get_quantity());

    println!("rCrabEngine Hello World Example");
    println!("==============================");
    println!("Scene structure created successfully!");
    println!("Root node: {}", root.get_name());
    println!("Camera position: {:?}", camera.get_location());
    println!("Geometry: {}", geometry.get_name());

    println!("\nNote: Full graphics rendering requires window initialization.");
}
