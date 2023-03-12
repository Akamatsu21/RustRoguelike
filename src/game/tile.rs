#[derive(Clone, Copy, Debug)]
pub struct Tile
{
	pub blocks_sight: bool,
	pub explored: bool,
	pub passable: bool
}

impl Tile
{
	pub fn empty() -> Self
	{
		Tile {blocks_sight: false, explored: false, passable: true}
	}

	pub fn wall() -> Self
	{
		Tile {blocks_sight: true, explored: false, passable: false}
	}
}