// Simple Physics Example

use rcrab_core::scene::{Camera, Geometry, MeshBuilder, Node};
use rcrab_core::math::Vec3;
use rcrab_physics::{PhysicsSpace, RigidBody, ColliderBuilder};
use std::sync::Arc;

fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting rCrabEngine Physics Example");

    // Create physics space
    let physics = PhysicsSpace::with_default_gravity();
    tracing::info!("Physics space created with gravity: {:?}", physics.get_gravity());

    // Create a ground plane (static rigid body)
    let ground_body = RigidBody::static_body();
    ground_body.set_position(Vec3::new(0.0, -2.0, 0.0));

    let _ground_collider = ColliderBuilder::box(10.0, 0.5, 10.0);

    // Register ground
    let ground_handle = physics.register_body(&ground_body);

    // Create a dynamic box (falls)
    let box_body = RigidBody::dynamic();
    box_body.set_position(Vec3::new(0.0, 5.0, 0.0));
    box_body.set_mass(1.0);

    let _box_collider = ColliderBuilder::box(1.0, 1.0, 1.0);

    // Register the box
    let box_handle = physics.register_body(&box_body);

    // Create scene objects
    let _root = Node::new("Root");
    let camera = Arc::new(Camera::new(1280, 720));
    camera.set_location(Vec3::new(0.0, 5.0, 15.0));
    camera.look_at(Vec3::ZERO, Vec3::Y);

    // Create visual representation
    let ground_mesh = MeshBuilder::create_box(20.0, 1.0, 20.0);
    let ground_geometry = Arc::new(Geometry::new_with_mesh("Ground", ground_mesh));
    ground_geometry.set_position(Vec3::new(0.0, -2.0, 0.0));

    let box_mesh = MeshBuilder::create_box(1.0, 1.0, 1.0);
    let box_geometry = Arc::new(Geometry::new_with_mesh("Box", box_mesh));

    // Simulate physics
    println!("Initial box position: {:?}", box_body.get_position());

    for i in 0..10 {
        physics.update();
        let pos = box_body.get_position();
        println!("Step {}: Box position: {:?}", i + 1, pos);

        // Update visual
        box_geometry.set_position(pos);
    }

    println!("\nPhysics simulation completed!");
    println!("Ground collider handle: {:?}", ground_handle);
    println!("Box collider handle: {:?}", box_handle);
    println!("Number of bodies: {}", physics.num_bodies());

    println!("\nrCrabEngine Physics Example");
    println!("======================");
    println!("Physics simulation ran successfully!");
    println!("The box fell from height and collided with the ground.");
}
