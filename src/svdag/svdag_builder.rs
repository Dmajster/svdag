use super::{svdag::SvdagValue, Svdag, SvdagNode, SvdagPointer};

use crate::hashed_volume::HashedVolume;
use crate::volume::Volume;
use crate::volume::{IsVolume, VolumePosition};
use std::collections::HashMap;

pub struct SvdagBuilder {
    hash_volume_layers: Vec<HashedVolume>,
    node_hashes: HashMap<u64, usize>,
    graph: Svdag,
}

impl SvdagBuilder {
    pub fn new() -> SvdagBuilder {
        SvdagBuilder {
            hash_volume_layers: Vec::new(),
            node_hashes: HashMap::new(),
            graph: Svdag::new(),
        }
    }

    pub fn create_layers(&mut self, volume: &Volume) -> &mut Self {
        println!("Volume dimensions: {:?}", volume.get_dimensions());
        self.graph.depth = volume.depth;

        let mut hashed_volume = HashedVolume::from(volume);

        while hashed_volume.dimensions.0 >= 1 {
            let new_hashed_volume = HashedVolume::from_hashed_volume(&hashed_volume);

            self.hash_volume_layers.insert(0, hashed_volume);

            hashed_volume = new_hashed_volume;
        }

        self
    }

    pub fn create_graph(&mut self) -> &mut Self {
        let mut graph = Svdag::new();
        graph.depth = self.graph.depth;

        let mut node_hashes: HashMap<u64, usize> = HashMap::new();

        self.recurse_layers(&mut graph, &mut node_hashes, 0, (0, 0, 0));
        self.graph = graph;
        self.node_hashes = node_hashes;

        self
    }

    fn recurse_layers(
        &self,
        new_graph: &mut Svdag,
        node_hashes: &mut HashMap<u64, usize>,
        layer_index: usize,
        position: VolumePosition,
    ) -> i16 {
        if layer_index >= self.hash_volume_layers.len() {
            return 0;
        }

        let layer = self.hash_volume_layers.get(layer_index).unwrap();
        let node = layer.get(position);

        let duplicate_node: Option<&usize>;

        //Check if this a new node
        duplicate_node = node_hashes.get(&node.hash);

        //If checked node is new
        if duplicate_node.is_none() {
            let children_positions = layer.calculate_children_positions(position);
            let children_count = node.children.count_occupied();

            //Store index of this node to return to parent after recursing trough all the node's children
            let current_node_absolute_index = new_graph.nodes.len();

            //Hash this node and store it's index for fast checking of duplicates
            node_hashes.insert(node.hash, current_node_absolute_index);

            //Preamptively push the node in the array so it maintains the parent index < child index rule
            new_graph.nodes.push(SvdagValue {
                node: SvdagNode {
                    children: node.children,
                    padding: 0,
                },
            });

            //Iterate over all hash layers to build the complete tree, +1 is because we don't need nodes for leaf children
            if layer_index + 1 < self.hash_volume_layers.len() {
                //Reserve space for all children
                for _ in 0..children_count {
                    new_graph.nodes.push(SvdagValue {
                        pointer: SvdagPointer { value: 0 },
                    })
                }

                let mut child_index_offset = 1; //Relative offset where in array to store child pointers

                //Recurse trough all children
                for child_position_index in 0..8 {
                    //Go to next child if this one is not occupied
                    if !node.children.get(child_position_index) {
                        continue;
                    }

                    //Recursively get this childs absolute array index
                    let child_node_absolute_index = self.recurse_layers(
                        new_graph,
                        node_hashes,
                        layer_index + 1,
                        children_positions[child_position_index],
                    );

                    //Get child's index by adding the child offset to the this node's absolute index
                    let child_offset_index =
                        (current_node_absolute_index + child_index_offset) as isize;

                    //Store a relative offset to the child node at the calculated child offset index
                    new_graph.nodes[child_offset_index as usize] = SvdagValue {
                        pointer: SvdagPointer {
                            value: (child_node_absolute_index as isize - child_offset_index) as i16,
                        },
                    };

                    child_index_offset += 1;
                }
            }

            current_node_absolute_index as i16
        }
        //If checked node is a duplicate
        else {
            duplicate_node.unwrap().clone() as i16
        }
    }

    pub fn finish(&self) -> Svdag {
        self.graph.clone()
    }
}
