use std::cmp::Ordering;

mod locomotion;
mod spacial;
#[cfg(test)]
mod tests;
mod ui;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InteractionType {
	HandPhysics = 0,
	Raycast = 1,
	FuzzySelect = 2,
}

impl Ord for InteractionType {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		let self_int = *self as u32;
		let other_int = *other as u32;

		self_int.cmp(&other_int).reverse()
	}

	fn max(self, other: Self) -> Self
	where
		Self: Sized,
	{
		std::cmp::max_by(self, other, Ord::cmp)
	}

	fn min(self, other: Self) -> Self
	where
		Self: Sized,
	{
		std::cmp::min_by(self, other, Ord::cmp)
	}

	fn clamp(self, min: Self, max: Self) -> Self
	where
		Self: Sized,
		Self: PartialOrd,
	{
		assert!(min <= max);
		if self > max {
			max
		} else if self < min {
			min
		} else {
			self
		}
	}
}

impl PartialOrd for InteractionType {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}
