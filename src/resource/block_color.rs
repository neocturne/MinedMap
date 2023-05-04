use super::{Biome, BlockType};

pub fn block_color(block: BlockType, _biome: &Biome, depth: f32) -> [u8; 4] {
	let h = 0.5 + 0.005 * depth;
	let c = block
		.color
		.0
		.map(|v| (f32::from(v) * h).clamp(0.0, 255.0) as u8);
	[c[0], c[1], c[2], 255]
}
