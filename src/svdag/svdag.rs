use super::SvdagBuilder;
use crate::hashed_volume::Children;
use crate::volume::VolumeDimensions;
use crate::volume::{IsVolume, Volume, VolumePosition};
use std::fmt;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Svdag {
    pub depth: u8,
    pub nodes: Vec<SvdagValue>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SvdagPointer {
    pub value: i16,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SvdagNode {
    pub children: Children,
    pub padding: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union SvdagValue {
    pub pointer: SvdagPointer,
    pub node: SvdagNode,
}

impl fmt::Debug for SvdagValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            f.debug_struct("Value")
                .field("p", &self.pointer)
                .field("n", &self.node)
                .finish()
        }
    }
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
        //Half the filter dimensions through reference for better performance
        filter_dimensions.0 /= 2;
        filter_dimensions.1 /= 2;
        filter_dimensions.2 /= 2;

        //Determine which child index to check
        let mut child_index = 0;
        if target_position.0 >= filter_position.0 + filter_dimensions.0 {
            filter_position.0 += filter_dimensions.0;
            child_index += 4;
        }
        if target_position.1 >= filter_position.1 + filter_dimensions.1 {
            filter_position.1 += filter_dimensions.2;
            child_index += 2;
        }
        if target_position.2 >= filter_position.2 + filter_dimensions.2 {
            filter_position.2 += filter_dimensions.2;
            child_index += 1;
        }

        unsafe {
            let node = &self.nodes.get(node_index).unwrap().node;

            //Check if this node's child area is occupied
            let is_child_occupied = node.children.get(child_index);

            //If it's not occupied there won't be a child node so the space is empty
            if !is_child_occupied {
                return false;
            }

            //Otherwise find the child area's consecutive index and pass it off to the recursion
            let child_pointer_index = node_index + node.children.get_n(child_index) + 1;

            if current_depth + 1 < self.depth {
                let child_pointer = &self.nodes.get(child_pointer_index).unwrap().pointer;

                return self.get_recursive(
                    target_position,
                    (child_pointer_index as isize + child_pointer.value as isize) as usize,
                    current_depth + 1,
                    (filter_position, filter_dimensions),
                );
            } else {
                return is_child_occupied;
            }
        }
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
