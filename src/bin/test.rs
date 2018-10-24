extern crate sw3d;


use sw3d::render::mesh;


fn main () {
    println!("ez");
    let mut window = sw3d::render::Render::new(800, 800, "Voxel");


    let cube_vertex = sw3d::render::CUBE;
    let cube = mesh::Mesh::new(cube_vertex.to_vec(), window.device.clone(), window.queue.clone(), window.graphics_pipeline.clone(), "tex.png".into());


    window.meshs.push(cube);

    

    window.run();
}