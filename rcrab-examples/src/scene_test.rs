// Scene Graph Test Example

use rcrab_core::scene::{Camera, Geometry, MeshBuilder, Node, Light, LightType};
use rcrab_core::math::{Vec3, Mat4};
use std::sync::Arc;

fn main() {
    tracing_subscriber::fmt::init();

    println!("rCrabEngine Scene Graph Test");
    println!("========================\n");

    // Test 1: Create nodes and hierarchy
    println!("Test 1: Creating scene hierarchy...");

    let root = Node::new("Root");
    let child1 = Node::new("Child1");
    let child2 = Node::new("Child2");
    let grandchild = Node::new("GrandChild");

    root.attach_child(Arc::new(child1.clone()));
    root.attach_child(Arc::new(child2.clone()));
    child1.attach_child(Arc::new(grandchild.clone()));

    println!("  Root children: {}", root.get_quantity());
    println!("  Child1 children: {}", child1.get_quantity());

    // Test 2: Create geometries
    println!("\nTest 2: Creating geometries...");

    let box_mesh = MeshBuilder::create_box(1.0, 1.0, 1.0);
    let sphere_mesh = MeshBuilder::create_sphere(0.5, 16, 16);
    let plane_mesh = MeshBuilder::create_plane(10.0, 10.0, 1, 1);

    let box_geo = Arc::new(Geometry::new_with_mesh("Box", box_mesh));
    let sphere_geo = Arc::new(Geometry::new_with_mesh("Sphere", sphere_mesh));
    let plane_geo = Arc::new(Geometry::new_with_mesh("Plane", plane_mesh));

    child1.attach_child(box_geo.clone());
    child2.attach_child(sphere_geo.clone());
    root.attach_child(plane_geo.clone());

    println!("  Created: Box, Sphere, Plane geometries");

    // Test 3: Transforms
    println!("\nTest 3: Testing transforms...");

    box_geo.set_position(Vec3::new(2.0, 0.0, 0.0));
    sphere_geo.set_position(Vec3::new(-2.0, 0.0, 0.0));
    plane_geo.set_position(Vec3::new(0.0, -1.0, 0.0));

    box_geo.set_rotation(glam::Quat::from_rotation_y(0.5));
    box_geo.set_scale(Vec3::new(1.5, 1.5, 1.5));

    println!("  Box position: {:?}", box_geo.get_local_transform().position);
    println!("  Box rotation: {:?}", box_geo.get_local_transform().rotation);
    println!("  Box scale: {:?}", box_geo.get_local_transform().scale);

    // Test 4: World transforms
    println!("\nTest 4: Testing world transforms...");

    root.update_world_transform(Mat4::IDENTITY);

    println!("  Box world position: {:?}", box_geo.get_world_position());
    println!("  Sphere world position: {:?}", sphere_geo.get_world_position());
    println!("  Plane world position: {:?}", plane_geo.get_world_position());

    // Test 5: Camera
    println!("\nTest 5: Testing camera...");

    let camera = Arc::new(Camera::new(1280, 720));
    camera.set_location(Vec3::new(0.0, 5.0, 10.0));
    camera.look_at(Vec3::ZERO, Vec3::Y);

    println!("  Camera position: {:?}", camera.get_location());
    println!("  Camera forward: {:?}", camera.get_forward());
    println!("  Camera right: {:?}", camera.get_right());

    // Test 6: Lights
    println!("\nTest 6: Testing lights...");

    let dir_light = Arc::new(Light::new("Directional", LightType::Directional));
    dir_light.set_direction(Vec3::new(-1.0, -1.0, -1.0));
    dir_light.set_color_rgb(1.0, 0.9, 0.8);
    dir_light.set_intensity(1.2);

    let point_light = Arc::new(Light::new("Point", LightType::Point));
    point_light.set_position(Vec3::new(3.0, 3.0, 3.0));
    point_light.set_color_rgb(1.0, 1.0, 1.0);
    point_light.set_intensity(1.0);

    println!("  Directional light direction: {:?}", dir_light.get_direction());
    println!("  Point light position: {:?}", point_light.get_position());

    // Test 7: Bounding volumes
    println!("\nTest 7: Testing bounding volumes...");

    if let Some(bounding) = box_geo.get_bounding() {
        println!("  Box has bounding volume: {:?}", bounding.get_type());
    }

    // Test traversal
    println!("\nTest 8: Scene traversal...");

    fn print_tree(node: &Node, depth: usize) {
        let indent = "  ".repeat(depth);
        println!("{}{}", indent, node.get_name());
        for child in node.get_children_slice() {
            if let Some(child_node) = child.as_node() {
                print_tree(&*child_node, depth + 1);
            }
        }
    }

    print_tree(&root, 0);

    println!("\n========================");
    println!("All scene graph tests completed!");
}
