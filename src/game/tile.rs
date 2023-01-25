#[derive(Clone, Copy, Debug)]
pub struct Tile
{
	pub blocks_sight: bool,
	pub passable: bool
}

impl Tile
{
	pub fn empty() -> Self
	{
		Tile {blocks_sight: false, passable: true}
	}

	pub fn wall() -> Self
	{
		Tile {blocks_sight: true, passable: false}
	}
}