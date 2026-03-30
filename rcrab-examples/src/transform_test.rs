// Transform Test Example

use rcrab_core::scene::{Camera, Geometry, MeshBuilder, Node};
use rcrab_core::math::{Vec3, Quat, Mat4, Transform};
use std::sync::Arc;

fn main() {
    tracing_subscriber::fmt::init();

    println!("rCrabEngine Transform Test");
    println!("========================\n");

    // Test 1: Create transforms
    println!("Test 1: Creating transforms...");

    let transform1 = Transform::identity();
    let transform2 = Transform::from_position(Vec3::new(5.0, 3.0, 2.0));
    let transform3 = Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::PI / 4.0));
    let transform4 = Transform::from_scale(Vec3::new(2.0, 2.0, 2.0));

    println!("  Identity: pos={:?}, rot={:?}, scale={:?}",
        transform1.position, transform1.rotation, transform1.scale);
    println!("  Position only: pos={:?}", transform2.position);
    println!("  Rotation only: rot={:?}", transform3.rotation);
    println!("  Scale only: scale={:?}", transform4.scale);

    // Test 2: Full transform
    println!("\nTest 2: Full transform...");
    let full_transform = Transform::from_translation_rotation_scale(
        Vec3::new(1.0, 2.0, 3.0),
        Quat::from_rotation_x(std::f32::consts::PI / 6.0),
        Vec3::new(1.5, 1.5, 1.5)
    );

    println!("  Position: {:?}", full_transform.position);
    println!("  Rotation: {:?}", full_transform.rotation);
    println!("  Scale: {:?}", full_transform.scale);

    // Test 3: Transform to matrix
    println!("\nTest 3: Transform to matrix...");
    let matrix = full_transform.to_matrix();
    println!("  Matrix:\n{}", matrix);

    // Test 4: Direction helpers
    println!("\nTest 4: Direction helpers...");
    let mut camera_transform = Transform::identity();
    camera_transform.position = Vec3::new(0.0, 5.0, 10.0);
    camera_transform.look_at(Vec3::ZERO, Vec3::Y);

    println!("  Forward: {:?}", camera_transform.forward());
    println!("  Backward: {:?}", camera_transform.backward());
    println!("  Up: {:?}", camera_transform.up());
    println!("  Down: {:?}", camera_transform.down());
    println!("  Right: {:?}", camera_transform.right());
    println!("  Left: {:?}", camera_transform.left());

    // Test 5: Transform points and directions
    println!("\nTest 5: Transform points and directions...");
    let test_point = Vec3::new(1.0, 0.0, 0.0);
    let test_dir = Vec3::new(1.0, 0.0, 0.0);

    let transformed_point = full_transform.transform_point(test_point);
    let transformed_dir = full_transform.transform_direction(test_dir);

    println!("  Original point: {:?}", test_point);
    println!("  Transformed point: {:?}", transformed_point);
    println!("  Original direction: {:?}", test_dir);
    println!("  Transformed direction: {:?}", transformed_dir);

    // Test 6: Combine transforms
    println!("\nTest 6: Combine transforms...");
    let parent = Transform::from_position(Vec3::new(10.0, 0.0, 0.0));
    let child = Transform::from_position(Vec3::new(5.0, 0.0, 0.0));

    let combined = parent.combine(&child);

    println!("  Parent pos: {:?}", parent.position);
    println!("  Child pos (local): {:?}", child.position);
    println!("  Combined pos (world): {:?}", combined.position);

    // Test 7: Inverse transform
    println!("\nTest 7: Inverse transform...");
    let inverse = full_transform.invert();

    println!("  Original: pos={:?}", full_transform.position);
    println!("  Inverse: pos={:?}", inverse.position);

    // Verify inverse works
    let round_trip = full_transform.combine(&inverse);
    println!("  Round trip (should be identity): pos={:?}, scale={:?}",
        round_trip.position, round_trip.scale);

    // Test 8: Apply to geometry
    println!("\nTest 8: Apply transforms to geometry...");
    let box_mesh = MeshBuilder::create_box(1.0, 1.0, 1.0);
    let box_geo = Arc::new(Geometry::new_with_mesh("TransformBox", box_mesh));

    box_geo.set_position(Vec3::new(3.0, 2.0, 1.0));
    box_geo.set_rotation(Quat::from_rotation_z(std::f32::consts::PI / 3.0));
    box_geo.set_scale(Vec3::new(2.0, 1.0, 0.5));

    let local = box_geo.get_local_transform();
    println!("  Position: {:?}", local.position);
    println!("  Rotation: {:?}", local.rotation);
    println!("  Scale: {:?}", local.scale);

    // Test 9: World transform
    println!("\nTest 9: World transform hierarchy...");
    let root = Node::new("Root");
    let parent_node = Node::new("Parent");
    let child_node = Node::new("Child");

    parent_node.set_position(Vec3::new(5.0, 0.0, 0.0));
    child_node.set_position(Vec3::new(3.0, 2.0, 0.0));

    root.attach_child(Arc::new(parent_node.clone()));
    parent_node.attach_child(Arc::new(child_node.clone()));

    // Update world transforms
    root.update_world_transform(Mat4::IDENTITY);

    println!("  Parent world pos: {:?}", parent_node.get_world_position());
    println!("  Child world pos: {:?}", child_node.get_world_position());

    println!("\n========================");
    println!("All transform tests completed!");
}
