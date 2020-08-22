use super::SvdagBuilder;
use crate::hashed_volume::Children;
use crate::volume::VolumeDimensions;
use crate::volume::{IsVolume, Volume, VolumePosition};

#[derive(Clone, Debug)]
pub struct Svdag {
    pub depth: u8,
    pub nodes: Vec<SvdagNode>,
}

#[derive(Clone, Copy, Debug)]
pub enum SvdagNode {
    Pointer(i16),
    Node(Children, u8),
}

impl Svdag {
    pub fn new() -> Svdag {
        Svdag {
            depth: 0,
            nodes: Vec::new(),
        }
    }

    pub fn get(&self, target_position: VolumePosition) -> bool {
        self.get_recursive(
            &target_position.clone(),
            0,
            0,
            (&mut (0, 0, 0), &mut self.get_dimensions()),
        )
    }

    fn get_recursive(
        &self,
        target_position: &VolumePosition,
        node_index: usize,
        current_depth: u8,
        (filter_position, filter_dimensions): (&mut VolumePosition, &mut VolumeDimensions),
    ) -> bool {
        // println!(
        //     "position: {:?}, dimensions: {:?}",
        //     filter_position, filter_dimensions
        // );

        //Half the filter dimensions through reference for better performance
        filter_dimensions.0 /= 2;
        filter_dimensions.1 /= 2;
        filter_dimensions.2 /= 2;

        //Determine which child index to check
        let mut child_index = 0;
        child_index += if target_position.0 >= filter_position.0 + filter_dimensions.0 {
            filter_position.0 += filter_dimensions.0;
            4
        } else {
            0
        };
        child_index += if target_position.1 >= filter_position.1 + filter_dimensions.1 {
            filter_position.1 += filter_dimensions.2;
            2
        } else {
            0
        };
        child_index += if target_position.2 >= filter_position.2 + filter_dimensions.2 {
            filter_position.2 += filter_dimensions.2;
            1
        } else {
            0
        };

        let mut node = self.nodes.get(node_index).unwrap();

        // println!(
        //     "depth: {}, node index: {} child index: {}",
        //     current_depth, node_index, child_index
        // );

        while let SvdagNode::Pointer(relative_offset) = node {
            node = self
                .nodes
                .get((node_index as isize + *relative_offset as isize) as usize)
                .unwrap();
        }

        if let SvdagNode::Node(children, _) = node {
            //Check if this node's child area is occupied
            let is_child_occupied = children.get(child_index);

            //If it's not occupied there won't be a child node so the space is empty
            if !is_child_occupied {
                //println!("occupied = false! children: {:08b}", children.child_bits);
                return false //TODO fix early termination on completly full nodes
            }

            //Otherwise find the child area's consecutive index and pass it off to the recursion
            let child_pointer_index = node_index + children.get_n(child_index) + 1;

            if current_depth + 1 < self.depth {
                let child_pointer = self.nodes.get(child_pointer_index).unwrap();

                if let SvdagNode::Pointer(relative_offset) = child_pointer {
                    return self.get_recursive(
                        target_position,
                        (child_pointer_index as isize + *relative_offset as isize) as usize,
                        current_depth + 1,
                        (filter_position, filter_dimensions),
                    );
                }
            } else {
                return is_child_occupied;
            }
        }

        panic!()
    }
}

impl From<&Volume> for Svdag {
    fn from(volume: &Volume) -> Self {
        SvdagBuilder::new()
            .create_layers(volume)
            .create_graph()
            .finish()
    }
}

impl IsVolume for Svdag {
    fn get_dimensions(&self) -> VolumeDimensions {
        let side_size = 2usize.pow(self.depth as u32);

        (side_size, side_size, side_size)
    }
}
