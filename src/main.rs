mod svdag;
use svdag::Svdag;

mod hashed_volume;

mod volume;
use volume::{IsVolume, Volume};

fn main() {
    let mut volume = Volume::new(6);
    let volume_dimensions = volume.get_dimensions();

    let sphere_position = (32, 32, 32);
    let sphere_radius = 16.0;
    for x in 0..volume_dimensions.0 {
        for y in 0..volume_dimensions.1 {
            for z in 0..volume_dimensions.2 {
                let position = (x, y, z);

                let distance = ((position.0 as f32 - sphere_position.0 as f32).powf(2.0)
                    + (position.1 as f32 - sphere_position.1 as f32).powf(2.0)
                    + (position.2 as f32 - sphere_position.2 as f32).powf(2.0))
                .sqrt();

                if distance < sphere_radius {
                    volume.set(position, true);
                }
            }
        }
    }

    let svdag = Svdag::from(&volume);

    for (index, node) in svdag.nodes.iter().enumerate() {
        println!("index: {}, node: {:?}", index, *node);
    }

    for x in 0..volume_dimensions.0 {
        for y in 0..volume_dimensions.1 {
            for z in 0..volume_dimensions.2 {
                let position = (x, y, z);

                let svdag_value = svdag.get(position);

                if *volume.get(position) != svdag_value {
                    println!(
                        "\tposition: {:?}, volume value: {}, svdag value: {}",
                        position,
                        volume.get(position),
                        svdag_value,
                    );
                }
            }
        }
    }

    let array_size = volume_dimensions.0 * volume_dimensions.1 * volume_dimensions.2 / 8;
    let svdag_size = svdag.nodes.len() * 2;

    println!(
        "array size: {}, svdag size: {}, compression ratio of {}%",
        array_size,
        svdag_size,
        array_size as f32 / svdag_size as f32 * 100.0
    )
}
