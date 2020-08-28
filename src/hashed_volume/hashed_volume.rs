use super::{Children, HashedVolumeNode};
use crate::volume::{CubicVolume, DensityVolume, IsVolume, VolumePosition};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub type HashedVolume = CubicVolume<HashedVolumeNode>;

impl From<&DensityVolume> for HashedVolume {
    fn from(src_density_volume: &DensityVolume) -> Self {
        let old_dimensions = src_density_volume.get_dimensions();
        let new_side_length = old_dimensions.0 / 2;

        println!("new side length: {}", new_side_length);

        let mut hashed_volume = HashedVolume::new(src_density_volume.depth - 1);

        for y in 0..new_side_length {
            for z in 0..new_side_length {
                for x in 0..new_side_length {
                    let x_src = x * 2;
                    let y_src = y * 2;
                    let z_src = z * 2;

                    let mut children = Children::default();
                    children.set(0, *src_density_volume.get((x_src, y_src, z_src)));
                    children.set(1, *src_density_volume.get((x_src, y_src, z_src + 1)));
                    children.set(2, *src_density_volume.get((x_src, y_src + 1, z_src)));
                    children.set(3, *src_density_volume.get((x_src, y_src + 1, z_src + 1)));
                    children.set(4, *src_density_volume.get((x_src + 1, y_src, z_src)));
                    children.set(5, *src_density_volume.get((x_src + 1, y_src, z_src + 1)));
                    children.set(6, *src_density_volume.get((x_src + 1, y_src + 1, z_src)));
                    children.set(
                        7,
                        *src_density_volume.get((x_src + 1, y_src + 1, z_src + 1)),
                    );

                    let mut hasher = DefaultHasher::new();
                    children.hash(&mut hasher);

                    let mut new_hashnode = hashed_volume.get_mut((x, y, z));
                    new_hashnode.children = children;
                    new_hashnode.hash = hasher.finish();
                }
            }
        }

        hashed_volume
    }
}

impl HashedVolume {
    pub fn from_hashed_volume(src_hashed_volume: &HashedVolume) -> HashedVolume {
        let old_dimensions = src_hashed_volume.get_dimensions();
        let new_side_length = old_dimensions.0 / 2;

        println!("new side length: {}", new_side_length);

        let mut new_hashed_volume = HashedVolume::new(src_hashed_volume.depth - 1);

        for y in 0..new_side_length {
            for z in 0..new_side_length {
                for x in 0..new_side_length {
                    let x_src = x * 2;
                    let y_src = y * 2;
                    let z_src = z * 2;

                    let node0 = src_hashed_volume.get((x_src, y_src, z_src));
                    let node1 = src_hashed_volume.get((x_src, y_src, z_src + 1));
                    let node2 = src_hashed_volume.get((x_src, y_src + 1, z_src));
                    let node3 = src_hashed_volume.get((x_src, y_src + 1, z_src + 1));
                    let node4 = src_hashed_volume.get((x_src + 1, y_src, z_src));
                    let node5 = src_hashed_volume.get((x_src + 1, y_src, z_src + 1));
                    let node6 = src_hashed_volume.get((x_src + 1, y_src + 1, z_src));
                    let node7 = src_hashed_volume.get((x_src + 1, y_src + 1, z_src + 1));

                    let mut children = Children::default();
                    children.set(0, node0.children.have_occupied_children());
                    children.set(1, node1.children.have_occupied_children());
                    children.set(2, node2.children.have_occupied_children());
                    children.set(3, node3.children.have_occupied_children());
                    children.set(4, node4.children.have_occupied_children());
                    children.set(5, node5.children.have_occupied_children());
                    children.set(6, node6.children.have_occupied_children());
                    children.set(7, node7.children.have_occupied_children());

                    let mut hasher = DefaultHasher::new();
                    node0.hash.hash(&mut hasher);
                    node1.hash.hash(&mut hasher);
                    node2.hash.hash(&mut hasher);
                    node3.hash.hash(&mut hasher);
                    node4.hash.hash(&mut hasher);
                    node5.hash.hash(&mut hasher);
                    node6.hash.hash(&mut hasher);
                    node7.hash.hash(&mut hasher);

                    let mut new_hashnode = new_hashed_volume.get_mut((x, y, z));
                    new_hashnode.children = children;
                    new_hashnode.hash = hasher.finish();
                }
            }
        }

        new_hashed_volume
    }

    pub fn calculate_children_positions(&self, position: VolumePosition) -> [VolumePosition; 8] {
        [
            (position.0 * 2, position.1 * 2, position.2 * 2),
            (position.0 * 2, position.1 * 2, position.2 * 2 + 1),
            (position.0 * 2, position.1 * 2 + 1, position.2 * 2),
            (position.0 * 2, position.1 * 2 + 1, position.2 * 2 + 1),
            (position.0 * 2 + 1, position.1 * 2, position.2 * 2),
            (position.0 * 2 + 1, position.1 * 2, position.2 * 2 + 1),
            (position.0 * 2 + 1, position.1 * 2 + 1, position.2 * 2),
            (position.0 * 2 + 1, position.1 * 2 + 1, position.2 * 2 + 1),
        ]
    }
}
