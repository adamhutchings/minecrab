use crate::mesh_tools::VecMesh;
use crate::world::generation::World;

pub fn build_geometry_voxel(
    world: &mut World, vmesh: &mut VecMesh, x: i64, y: i64, z: i64
) {
    if !world.get_block_data(x, y, z).non_void { return }
    for (dx, dy, dz) in [
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ] {
        if world.get_block_data(x + dx, y + dy, z + dz).non_void {
            continue;
        }

        // if we've hit an air block, then we have a visible block face
        // add the vertices accordingly
        let (x, y, z) = (x as f32, y as f32, z as f32);
        if dx == 1 {
            // FIXME: how do I stop this from getting formatted
            // #[rustfmt::skip]
            vmesh.vertices.extend_from_slice(&[
                x + 1., y, z,
                x + 1., y + 1., z,
                x + 1., y + 1., z + 1.,
                x + 1., y, z + 1.,
            ]);
            vmesh.texcoords.extend_from_slice(&[
                0.1, 0.0,
                0.1, 0.1,
                0.0, 0.1,
                0.0, 0.0,
            ]);
        } else if dx == -1 {
            vmesh.vertices.extend_from_slice(&[
                x, y, z,
                x, y, z + 1.,
                x, y + 1., z + 1.,
                x, y + 1., z,
            ]);
            vmesh.texcoords.extend_from_slice(&[
                0.0, 0.0,
                0.1, 0.0,
                0.1, 0.1,
                0.0, 0.1,
            ]);
        } else if dy == 1 {
            vmesh.vertices.extend_from_slice(&[
                x, y + 1., z,
                x, y + 1., z + 1.,
                x + 1., y + 1., z + 1.,
                x + 1., y + 1., z,
            ]);
            vmesh.texcoords.extend_from_slice(&[
                0.0, 0.1,
                0.0, 0.0,
                0.1, 0.0,
                0.1, 0.1,
            ]);
        } else if dy == -1 {
            vmesh.vertices.extend_from_slice(&[
                x, y, z,
                x + 1., y, z,
                x + 1., y, z + 1.,
                x, y, z + 1.,
            ]);
            vmesh.texcoords.extend_from_slice(&[
                0.0, 0.0,
                0.1, 0.0,
                0.1, 0.1,
                0.0, 0.1,
            ]);
        } else if dz == 1 {
            vmesh.vertices.extend_from_slice(&[
                x, y, z + 1.0,
                x + 1., y, z + 1.0,
                x + 1., y + 1., z + 1.0,
                x, y + 1., z + 1.0,
            ]);
            vmesh.texcoords.extend_from_slice(&[
                0.0, 0.0,
                0.1, 0.0,
                0.1, 0.1,
                0.0, 0.1,
            ]);
        } else if dz == -1 {
            vmesh.vertices.extend_from_slice(&[
                x, y, z,
                x, y + 1., z,
                x + 1., y + 1., z,
                x + 1., y, z,
            ]);
            vmesh.texcoords.extend_from_slice(&[
                0.1, 0.0,
                0.1, 0.1,
                0.0, 0.1,
                0.0, 0.0,
            ]);
        }

        // dx, dy, dz give us the normals for this face
        let (dx, dy, dz) = (dx as f32, dy as f32, dz as f32);
        vmesh.normals.extend_from_slice(&[
            dx, dy, dz,
            dx, dy, dz,
            dx, dy, dz,
            dx, dy, dz,
        ]);
    }
}