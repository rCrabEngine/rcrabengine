// Lighting Test Example

use rcrab_core::scene::{Camera, Geometry, MeshBuilder, Node, Light, LightType};
use rcrab_core::math::Vec3;
use std::sync::Arc;

fn main() {
    tracing_subscriber::fmt::init();

    println!("rCrabEngine Lighting Test");
    println!("=======================\n");

    // Create scene
    let root = Node::new("Root");

    // Create camera
    let camera = Arc::new(Camera::new(1280, 720));
    camera.set_location(Vec3::new(0.0, 5.0, 15.0));
    camera.look_at(Vec3::ZERO, Vec3::Y);

    // Test 1: Directional Light
    println!("Test 1: Directional Light...");
    let dir_light = Arc::new(Light::new("Sun", LightType::Directional));
    dir_light.set_direction(Vec3::new(-0.5, -1.0, -0.5));
    dir_light.set_color_rgb(1.0, 0.95, 0.8);
    dir_light.set_intensity(1.5);
    dir_light.set_casts_shadows(true);

    println!("  Direction: {:?}", dir_light.get_direction());
    println!("  Color: {:?}", dir_light.get_color());
    println!("  Intensity: {}", dir_light.get_intensity());
    println!("  Casts shadows: {}", dir_light.casts_shadows());

    // Test 2: Point Light
    println!("\nTest 2: Point Light...");
    let point_light = Arc::new(Light::new("PointLight", LightType::Point));
    point_light.set_position(Vec3::new(3.0, 4.0, 2.0));
    point_light.set_color_rgb(1.0, 0.5, 0.2);
    point_light.set_intensity(2.0);
    point_light.set_point_light_attenuation(20.0);

    println!("  Position: {:?}", point_light.get_position());
    println!("  Color: {:?}", point_light.get_color());
    println!("  Intensity: {}", point_light.get_intensity());

    // Test 3: Spot Light
    println!("\nTest 3: Spot Light...");
    let spot_light = Arc::new(Light::new("SpotLight", LightType::Spot));
    spot_light.set_position(Vec3::new(0.0, 8.0, 5.0));
    spot_light.set_direction(Vec3::new(0.0, -1.0, -0.3));
    spot_light.set_color_rgb(0.8, 0.9, 1.0);
    spot_light.set_intensity(3.0);
    spot_light.set_spot_inner_angle(std::f32::consts::PI / 8.0);
    spot_light.set_spot_outer_angle(std::f32::consts::PI / 4.0);

    println!("  Position: {:?}", spot_light.get_position());
    println!("  Direction: {:?}", spot_light.get_direction());
    println!("  Inner cone angle: {} rad", spot_light.get_spot_inner_angle());
    println!("  Outer cone angle: {} rad", spot_light.get_spot_outer_angle());

    // Test 4: Enable/Disable lights
    println!("\nTest 4: Enable/Disable lights...");
    dir_light.set_enabled(true);
    point_light.set_enabled(true);
    spot_light.set_enabled(false);

    println!("  Directional enabled: {}", dir_light.is_enabled());
    println!("  Point enabled: {}", point_light.is_enabled());
    println!("  Spot enabled: {}", spot_light.is_enabled());

    // Test 5: Light with shadows
    println!("\nTest 5: Shadow configuration...");
    let shadow_light = Arc::new(Light::new("ShadowLight", LightType::Directional));
    shadow_light.set_casts_shadows(true);
    shadow_light.set_shadow_distance(50.0);
    shadow_light.set_shadow_intensity(0.5);

    println!("  Shadow distance: {}", shadow_light.get_shadow_distance());
    println!("  Shadow intensity: {}", shadow_light.get_shadow_intensity());

    // Test 6: Multiple lights scene
    println!("\nTest 6: Multiple lights scene...");
    let light_node = Node::new("Lights");
    light_node.attach_child(dir_light.clone());
    light_node.attach_child(point_light.clone());
    light_node.attach_child(spot_light.clone());

    println!("  Light node has {} children", light_node.get_quantity());

    // Create geometries to light
    println!("\nTest 7: Lit geometries...");
    let sphere_mesh = MeshBuilder::create_sphere(1.0, 32, 32);
    let sphere = Arc::new(Geometry::new_with_mesh("LitSphere", sphere_mesh));
    sphere.set_position(Vec3::new(0.0, 1.0, 0.0));

    let box_mesh = MeshBuilder::create_box(1.5, 1.5, 1.5);
    let box = Arc::new(Geometry::new_with_mesh("LitBox", box_mesh));
    box.set_position(Vec3::new(3.0, 0.75, 0.0));

    root.attach_child(sphere.clone());
    root.attach_child(box.clone());

    println!("  Sphere position: {:?}", sphere.get_local_transform().position);
    println!("  Box position: {:?}", box.get_local_transform().position);

    println!("\n========================");
    println!("All lighting tests completed!");
}
