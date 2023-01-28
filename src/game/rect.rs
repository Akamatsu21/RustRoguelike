#[derive(Clone, Copy, Debug)]
pub struct Rect
{
	pub x1: usize,
	pub y1: usize,
	pub x2: usize,
	pub y2: usize
}

impl Rect
{
	pub fn new(x: usize, y: usize, w: usize, h: usize) -> Self
	{
		Rect {x1: x, y1: y, x2: x + w, y2: y + h}
	}

	pub fn center(&self) -> (usize, usize)
	{
		((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
	}

	pub fn collides(&self, rect: &Rect) -> bool
	{
		(self.x1 <= rect.x2) && (self.x2 >= rect.x1)
			&& (self.y1  <= rect.y2) && (self.y2 >= rect.y1)
	}
}