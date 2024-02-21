/// A trait representing a grabbable object in 3D space
pub trait Grabbable {
	fn uses_fuzzy_grab(&self) -> bool;
}
