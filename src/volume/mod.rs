pub type VolumeDimensions = (usize, usize, usize);
pub type VolumePosition = (usize, usize, usize);

pub trait IsVolume {
    fn get_dimensions(&self) -> VolumeDimensions;
}

pub trait VolumeIndex {
    fn get_index(&self, volume: &impl IsVolume) -> usize;
}

impl VolumeIndex for usize {
    fn get_index(&self, _: &impl IsVolume) -> usize {
        *self
    }
}

impl VolumeIndex for VolumePosition {
    fn get_index(&self, volume: &impl IsVolume) -> usize {
        let dimensions = volume.get_dimensions();
        self.2 * (dimensions.0 * dimensions.1) + self.1 * dimensions.0 + self.0
    }
}

pub struct Volume {
    densities: Vec<bool>,
    pub depth: u8,
}

impl IsVolume for Volume {
    fn get_dimensions(&self) -> VolumeDimensions {
        let side_size = 2usize.pow(self.depth as u32);

        (side_size, side_size, side_size)
    }
}

impl Volume {
    pub fn new(depth: u8) -> Volume {
        Volume {
            densities: vec![false; 8usize.pow(depth as u32)],
            depth,
        }
    }

    pub fn get(&self, volume_index: impl VolumeIndex) -> &bool {
        &self.densities[volume_index.get_index(self)]
    }

    pub fn set(&mut self, volume_index: impl VolumeIndex, value: bool) {
        let index = volume_index.get_index(self);
        self.densities[index] = value;
    }
}
