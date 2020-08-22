use super::{Children, HashedVolumeNode};
use crate::volume::{IsVolume, Volume, VolumeDimensions, VolumeIndex, VolumePosition};

use std::{
    collections::hash_map::DefaultHasher,
    fmt,
    hash::{Hash, Hasher},
};

pub struct HashedVolume {
    hashnodes: Vec<HashedVolumeNode>,
    pub dimensions: VolumeDimensions,
}

impl IsVolume for HashedVolume {
    fn get_dimensions(&self) -> VolumeDimensions {
        self.dimensions
    }
}

impl From<&Volume> for HashedVolume {
    fn from(volume: &Volume) -> Self {
        let old_dimensions = volume.get_dimensions();

        let new_dimensions = (
            old_dimensions.0 / 2,
            old_dimensions.1 / 2,
            old_dimensions.2 / 2,
        );

        let mut hashed_volume = HashedVolume {
            hashnodes: vec![
                HashedVolumeNode::new(0, Children::new());
                new_dimensions.0 * new_dimensions.1 * new_dimensions.2
            ],
            dimensions: new_dimensions,
        };

        for y in 0..new_dimensions.1 {
            for z in 0..new_dimensions.2 {
                for x in 0..new_dimensions.0 {
                    let x_src = x * 2;
                    let y_src = y * 2;
                    let z_src = z * 2;

                    let mut children = Children::new();
                    children.set(0, *volume.get((x_src, y_src, z_src)));
                    children.set(1, *volume.get((x_src, y_src, z_src + 1)));
                    children.set(2, *volume.get((x_src, y_src + 1, z_src)));
                    children.set(3, *volume.get((x_src, y_src + 1, z_src + 1)));
                    children.set(4, *volume.get((x_src + 1, y_src, z_src)));
                    children.set(5, *volume.get((x_src + 1, y_src, z_src + 1)));
                    children.set(6, *volume.get((x_src + 1, y_src + 1, z_src)));
                    children.set(7, *volume.get((x_src + 1, y_src + 1, z_src + 1)));

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
    pub fn from_hashed_volume(source_hashed_volume: &HashedVolume) -> HashedVolume {
        let new_dimensions: VolumeDimensions = (
            source_hashed_volume.dimensions.0 / 2,
            source_hashed_volume.dimensions.1 / 2,
            source_hashed_volume.dimensions.2 / 2,
        );

        let mut new_hashed_volume = HashedVolume {
            hashnodes: vec![
                HashedVolumeNode::new(0, Children::new());
                new_dimensions.0 * new_dimensions.1 * new_dimensions.2
            ],
            dimensions: new_dimensions,
        };
        for y in 0..new_dimensions.2 {
            for z in 0..new_dimensions.1 {
                for x in 0..new_dimensions.0 {
                    let x_src = x * 2;
                    let y_src = y * 2;
                    let z_src = z * 2;

                    let node0 = source_hashed_volume.get((x_src, y_src, z_src));
                    let node1 = source_hashed_volume.get((x_src, y_src, z_src + 1));
                    let node2 = source_hashed_volume.get((x_src, y_src + 1, z_src));
                    let node3 = source_hashed_volume.get((x_src, y_src + 1, z_src + 1));
                    let node4 = source_hashed_volume.get((x_src + 1, y_src, z_src));
                    let node5 = source_hashed_volume.get((x_src + 1, y_src, z_src + 1));
                    let node6 = source_hashed_volume.get((x_src + 1, y_src + 1, z_src));
                    let node7 = source_hashed_volume.get((x_src + 1, y_src + 1, z_src + 1));

                    let mut children = Children::new();
                    children.set(0, node0.children.are_interesting());
                    children.set(1, node1.children.are_interesting());
                    children.set(2, node2.children.are_interesting());
                    children.set(3, node3.children.are_interesting());
                    children.set(4, node4.children.are_interesting());
                    children.set(5, node5.children.are_interesting());
                    children.set(6, node6.children.are_interesting());
                    children.set(7, node7.children.are_interesting());

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

    pub fn get_mut(&mut self, volume_index: impl VolumeIndex) -> &mut HashedVolumeNode {
        let index = volume_index.get_index(self);
        &mut self.hashnodes[index]
    }

    pub fn get(&self, volume_index: impl VolumeIndex) -> &HashedVolumeNode {
        &self.hashnodes[volume_index.get_index(self)]
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

impl fmt::Display for HashedVolume {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut hashnode_map: Vec<u64> = Vec::new();

        for y in 0..self.dimensions.1 {
            for z in 0..self.dimensions.2 {
                for x in 0..self.dimensions.0 {
                    let hashnode = self.get((x, y, z));

                    let find_hash = hashnode_map
                        .iter()
                        .position(|&compared_hashnode| compared_hashnode == hashnode.hash);

                    let density_index = match find_hash {
                        None => {
                            hashnode_map.push(hashnode.hash);
                            hashnode_map.len() - 1
                        }
                        Some(found_hashnode_index) => found_hashnode_index,
                    };

                    write!(f, "{:02} ", density_index).unwrap();
                }
                writeln!(f,).unwrap();
            }
            writeln!(f, "-----").unwrap();
        }
        write!(f, "")
    }
}
