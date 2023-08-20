//! The generic [RegionGroup] data structure

use std::{future::Future, iter};

use anyhow::Result;
use futures_util::future::OptionFuture;

/// A generic array of 3x3 elements
///
/// A RegionGroup is used to store information about a 3x3 neighborhood of
/// regions.
///
/// The center element is always populated, while the 8 adjacent elements may be None.
#[derive(Debug, Clone, Copy)]
pub struct RegionGroup<T> {
	/// The element corresponding to the center of the 9x9 neighborhood
	center: T,
	/// The remaining elements, stored in row-first order
	///
	/// The center element is always None.
	neighs: [Option<T>; 9],
}

impl<T> RegionGroup<T> {
	/// Constructs a new RegionGroup from a closure called for each element
	///
	/// The X and Z coordinates relative to the center (in the range -1..1)
	/// are passed to the closure.
	///
	/// Panics of the closure returns None for the center element.
	pub fn new<F>(f: F) -> Self
	where
		F: Fn(i8, i8) -> Option<T>,
	{
		RegionGroup {
			center: f(0, 0).expect("Center element of RegionGroup must not be None"),
			neighs: [
				f(-1, -1),
				f(-1, 0),
				f(-1, 1),
				f(0, -1),
				None,
				f(0, 1),
				f(1, -1),
				f(1, 0),
				f(1, 1),
			],
		}
	}

	/// Returns a reference to the center element
	pub fn center(&self) -> &T {
		&self.center
	}

	/// Returns a reference to an element of the RegionGroup, if populated
	///
	/// Always returns None for X and Z coordinates outside of the -1..1 range.
	pub fn get(&self, x: i8, z: i8) -> Option<&T> {
		if (x, z) == (0, 0) {
			return Some(&self.center);
		}
		if !(-1..=1).contains(&x) || !(-1..=1).contains(&z) {
			return None;
		}
		self.neighs.get((3 * x + z + 4) as usize)?.as_ref()
	}

	/// Runs a closure on each element to construct a new RegionGroup
	pub fn map<U, F>(self, mut f: F) -> RegionGroup<U>
	where
		F: FnMut(T) -> U,
	{
		RegionGroup {
			center: f(self.center),
			neighs: self.neighs.map(|entry| entry.map(&mut f)),
		}
	}

	/// Runs a fallible closure on each element to construct a new RegionGroup
	///
	/// [Err] return values for the center element are passed up. Outer elements
	/// become unpopulated when the closure fails.
	pub fn try_map<U, F>(self, mut f: F) -> Result<RegionGroup<U>>
	where
		F: FnMut(T) -> Result<U>,
	{
		let RegionGroup { center, neighs } = self;
		let center = f(center)?;
		let neighs = neighs.map(|entry| entry.and_then(|value| f(value).ok()));
		Ok(RegionGroup { center, neighs })
	}

	/// Runs an asynchronous closure on each element to construct a new RegionGroup
	#[allow(dead_code)]
	pub async fn async_map<U, F, Fut>(self, mut f: F) -> RegionGroup<U>
	where
		Fut: Future<Output = U>,
		F: FnMut(T) -> Fut,
	{
		let center = f(self.center);
		let neighs = futures_util::future::join_all(
			self.neighs
				.map(|entry| OptionFuture::from(entry.map(&mut f))),
		);
		let (center, neighs) = futures_util::join!(center, neighs);
		RegionGroup {
			center,
			neighs: <[Option<_>; 9]>::try_from(neighs).ok().unwrap(),
		}
	}

	/// Runs a fallible asynchronous closure on each element to construct a new RegionGroup
	///
	/// [Err] return values for the center element are passed up. Outer elements
	/// become unpopulated when the closure fails.
	pub async fn async_try_map<U, F, Fut>(self, mut f: F) -> Result<RegionGroup<U>>
	where
		Fut: Future<Output = Result<U>>,
		F: FnMut(T) -> Fut,
	{
		let center = f(self.center);
		let neighs = futures_util::future::join_all(
			self.neighs
				.map(|entry| OptionFuture::from(entry.map(&mut f))),
		);
		let (center, neighs) = futures_util::join!(center, neighs);
		let center = center?;

		let neighs: Vec<_> = neighs
			.into_iter()
			.map(|entry| entry.and_then(Result::ok))
			.collect();
		let neighs = <[Option<_>; 9]>::try_from(neighs).ok().unwrap();

		Ok(RegionGroup { center, neighs })
	}

	/// Returns an [Iterator] over all populated elements
	pub fn iter(&self) -> impl Iterator<Item = &T> {
		iter::once(&self.center).chain(self.neighs.iter().filter_map(Option::as_ref))
	}
}
