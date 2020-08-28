pub type VolumeDimensions = (usize, usize, usize);
pub type VolumePosition = (usize, usize, usize);
pub type VolumeIndex = usize;
pub type DensityVolume = CubicVolume<bool>;

pub trait IsVolume {
    fn get_dimensions(&self) -> VolumeDimensions;

    fn get_element_count(&self) -> usize {
        let dimensions = &self.get_dimensions();

        dimensions.0 * dimensions.1 * dimensions.2
    }
}

pub trait IsVolumeIndex {
    fn get_index(&self, volume: &impl IsVolume) -> VolumeIndex;
}

impl IsVolumeIndex for usize {
    fn get_index(&self, _: &impl IsVolume) -> VolumeIndex {
        *self
    }
}

impl IsVolumeIndex for VolumePosition {
    fn get_index(&self, volume: &impl IsVolume) -> VolumeIndex {
        let dimensions = volume.get_dimensions();
        self.2 * (dimensions.0 * dimensions.1) + self.1 * dimensions.0 + self.0
    }
}

pub struct CubicVolume<T>
where
    T: Default + Clone,
{
    values: Vec<T>,
    pub depth: u8,
}

impl<T> IsVolume for CubicVolume<T>
where
    T: Default + Clone,
{
    fn get_dimensions(&self) -> VolumeDimensions {
        let side_size = CubicVolume::<T>::get_side_element_count(self.depth);

        (side_size, side_size, side_size)
    }
}

impl<T> CubicVolume<T>
where
    T: Default + Clone,
{
    pub fn new(depth: u8) -> CubicVolume<T> {
        CubicVolume {
            values: vec![T::default(); CubicVolume::<T>::get_volume_element_count(depth)],
            depth,
        }
    }

    pub fn get(&self, volume_index: impl IsVolumeIndex) -> &T {
        &self.values[volume_index.get_index(self)]
    }

    pub fn get_mut(&mut self, volume_index: impl IsVolumeIndex) -> &mut T {
        let index = volume_index.get_index(self);
        &mut self.values[index]
    }

    pub fn get_side_element_count(depth: u8) -> usize {
        2usize.pow(depth as u32)
    }

    pub fn get_area_element_count(depth: u8) -> usize {
        4usize.pow(depth as u32)
    }

    pub fn get_volume_element_count(depth: u8) -> usize {
        8usize.pow(depth as u32)
    }
}
