// Collision Test Example

use rcrab_core::scene::{Camera, Geometry, MeshBuilder, Node};
use rcrab_core::math::{Vec3, BoundingBox, BoundingSphere, BoundingVolume};
use std::sync::Arc;

fn main() {
    tracing_subscriber::fmt::init();

    println!("rCrabEngine Collision Test");
    println!("======================\n");

    // Test 1: Bounding Sphere
    println!("Test 1: Bounding Sphere...");
    let sphere1 = BoundingSphere::new(Vec3::new(0.0, 0.0, 0.0), 2.0);
    let sphere2 = BoundingSphere::new(Vec3::new(2.5, 0.0, 0.0), 1.5);
    let sphere3 = BoundingSphere::new(Vec3::new(10.0, 0.0, 0.0), 2.0);

    println!("  Sphere1: center={:?}, radius={}", sphere1.center, sphere1.radius);
    println!("  Sphere2: center={:?}, radius={}", sphere2.center, sphere2.radius);
    println!("  Sphere3: center={:?}, radius={}", sphere3.center, sphere3.radius);

    // Test collision
    let sphere1_intersects_2 = sphere1.intersects(&sphere2);
    let sphere1_intersects_3 = sphere1.intersects(&sphere3);

    println!("  Sphere1 intersects Sphere2: {}", sphere1_intersects_2);
    println!("  Sphere1 intersects Sphere3: {}", sphere1_intersects_3);

    // Test 2: Bounding Box
    println!("\nTest 2: Bounding Box...");
    let box1 = BoundingBox::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(2.0, 2.0, 2.0));
    let box2 = BoundingBox::new(Vec3::new(3.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
    let box3 = BoundingBox::new(Vec3::new(10.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));

    println!("  Box1: min={:?}, max={:?}", box1.min(), box1.max());
    println!("  Box2: min={:?}, max={:?}", box2.min(), box2.max());

    let box1_intersects_2 = box1.intersects(&box2);
    let box1_intersects_3 = box1.intersects(&box3);

    println!("  Box1 intersects Box2: {}", box1_intersects_2);
    println!("  Box1 intersects Box3: {}", box1_intersects_3);

    // Test 3: Sphere-Box intersection
    println!("\nTest 3: Sphere-Box intersection...");
    let sphere_in_box = BoundingSphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0);
    let sphere_outside_box = BoundingSphere::new(Vec3::new(5.0, 0.0, 0.0), 1.0);

    println!("  Sphere in box intersects: {}", sphere_in_box.intersects_box(&box1));
    println!("  Sphere outside intersects: {}", sphere_outside_box.intersects_box(&box1));

    // Test 4: Contains point
    println!("\nTest 4: Contains point...");
    let point_in = Vec3::new(0.0, 0.0, 0.0);
    let point_out = Vec3::new(5.0, 5.0, 5.0);

    println!("  Sphere contains point (0,0,0): {}", sphere1.contains_point(point_in));
    println!("  Sphere contains point (5,5,5): {}", sphere1.contains_point(point_out));
    println!("  Box contains point (0,0,0): {}", box1.contains_point(point_in));
    println!("  Box contains point (5,5,5): {}", box1.contains_point(point_out));

    // Test 5: Bounding Volume enum
    println!("\nTest 5: BoundingVolume enum...");
    let vol_sphere = BoundingVolume::Sphere(sphere1);
    let vol_box = BoundingVolume::Box(box1);

    println!("  Volume 1 type: {:?}", vol_sphere.get_type());
    println!("  Volume 2 type: {:?}", vol_box.get_type());

    // Test 6: From min/max
    println!("\nTest 6: Create box from min/max...");
    let box_from_minmax = BoundingBox::from_min_max(
        Vec3::new(-1.0, -1.0, -1.0),
        Vec3::new(1.0, 1.0, 1.0)
    );

    println!("  Box from min/max: center={:?}, extents={:?}",
        box_from_minmax.center, box_from_minmax.extents);

    // Test 7: Scene geometry with bounds
    println!("\nTest 7: Geometry bounding volumes...");
    let box_mesh = MeshBuilder::create_box(2.0, 2.0, 2.0);
    let box_geo = Arc::new(Geometry::new_with_mesh("TestBox", box_mesh));

    if let Some(bounding) = box_geo.get_bounding() {
        println!("  Geometry bounding type: {:?}", bounding.get_type());
    } else {
        println!("  Note: Bounding not set on mesh (would be generated during render)");
    }

    println!("\n========================");
    println!("All collision tests completed!");
}
