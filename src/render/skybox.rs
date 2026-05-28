use raylib::models::{Mesh, RaylibMesh};

use crate::render::mesh_tools::VecMesh;

// We need to render the skybox as a cube in a fixed location relative to the
// player, but NOT in a fixed orientation (like the end portal is in Minecraft,)
// for example. This needs to be far enough away that the skybox won't eclipse
// the actual terrain.
const SK_DIST: f32 = 200.;

pub fn create_skybox_mesh() -> Mesh {
    let mut vmesh = VecMesh::new();
    vmesh.vertices.extend_from_slice(&[
        // East face
         SK_DIST,  SK_DIST,  SK_DIST,
         SK_DIST,  SK_DIST, -SK_DIST,
         SK_DIST, -SK_DIST,  SK_DIST,
         SK_DIST, -SK_DIST, -SK_DIST,
         SK_DIST, -SK_DIST,  SK_DIST,
         SK_DIST,  SK_DIST, -SK_DIST,
        // West face
        -SK_DIST,  SK_DIST,  SK_DIST,
        -SK_DIST, -SK_DIST,  SK_DIST,
        -SK_DIST,  SK_DIST, -SK_DIST,
        -SK_DIST, -SK_DIST, -SK_DIST,
        -SK_DIST,  SK_DIST, -SK_DIST,
        -SK_DIST, -SK_DIST,  SK_DIST,
    ]);
    vmesh.normals.extend_from_slice(&[
        // East face faces westwards
        -1., 0., 0.,
        -1., 0., 0.,
        -1., 0., 0.,
        -1., 0., 0.,
        -1., 0., 0.,
        -1., 0., 0.,
        // West face faces eastwards
         1., 0., 0.,
         1., 0., 0.,
         1., 0., 0.,
         1., 0., 0.,
         1., 0., 0.,
         1., 0., 0.,
    ]);
    let mut mesh = vmesh.to_mesh();
    unsafe { mesh.upload(false); }
    mesh
}
