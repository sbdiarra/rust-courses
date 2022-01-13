use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

fn main() {
	let simulated_user_specified_value = 60;
	let simulated_random_number_value = 7;

	generated_workout(simulated_user_specified_value, simulated_random_number_value);
}
struct Cacher<T, U, V>
where
	U: Hash + Eq + Clone + Copy,
	T: Fn(U) -> V,
{
	calculation: T,
	value: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
where
	U: Hash + Eq + Clone + Copy,
	T: Fn(U) -> V,
{
	fn new(calculation: T) -> Cacher<T, U, V> {
		Cacher { calculation, value: HashMap::new() }
	}

	fn value(&mut self, arg: U) -> &V {
		if !self.value.contains_key(&arg) {
			let v = (self.calculation)(arg);
			self.value.insert(arg.clone(), v);
		}
		self.value.get(&arg).unwrap()
	}
}

fn generated_workout(intensity: u32, random_number: u32) {
	let mut expensive_result = Cacher::new(|num: u32| -> u32 {
		println!("calculating slowly...");
		thread::sleep(Duration::from_secs(2));
		num
	});

	if intensity < 25 {
		println!("Today, do {} pushups!", expensive_result.value(intensity));
		println!("Next, do {} situps!", expensive_result.value(intensity));
	} else if random_number == 3 {
		println!("Take a break today! Remember to say hydrated!");
	} else {
		println!("Today, run for {} minutes", expensive_result.value(intensity));
	}
}

#[test]
fn call_with_different_values() {
	let mut c = Cacher::new(|a: i32| a);

	let v1 = c.value(1);
	let v2 = c.value(2);

	assert_eq!(*v2, 2);
}

mod shoeExample {

	#[derive(PartialEq, Debug)]
	struct Shoe {
		size: u32,
		stylw: String,
	}

	fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
		shoes.into_iter().filter(|s| s.size == shoe_size).collect()
	}
}

mod customIterator {
	struct Counter {
		count: u32,
	}

	impl Counter {
		fn new() -> Counter {
			Counter { count: 0 }
		}
	}

	impl Iterator for Counter {
		type Item = u32;

		fn next(&mut self) -> Option<Self::Item> {
			if self.count < 5 {
				self.count += 1;
				Some(self.count)
			} else {
				None
			}
		}
	}
}
