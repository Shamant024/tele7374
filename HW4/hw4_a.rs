#[derive(Debug)]
enum State {
	A,
	B,
	C,
	D,
}

#[derive(Debug)]
enum Signal {
	Present,
	Absent,
}

fn update(s: (State, Signal, Signal)) -> (State, i32) {
	// Initial State at A
	if matches!(s.0, State::A) {
		if matches!(s.1, Signal::Present) && matches!(s.2, Signal::Absent) {
			// A to B, output 1
			return (State::B, 1);
		} else if matches!(s.1, Signal::Present) && matches!(s.2, Signal::Present) {
			// A to D, output 3
			return (State::D, 3);
		}
	}
	// State B transitions
	else if matches!(s.0, State::B) {
		if matches!(s.1, Signal::Present) && matches!(s.2, Signal::Absent) {
			// B to C, output 2
			return (State::C, 2);
		} else if matches!(s.1, Signal::Absent) && matches!(s.2, Signal::Present) {
			// B to A, output 0
			return (State::A, 0);
		}
	}
	// State C transitions
	else if matches!(s.0, State::C) {
		if matches!(s.1, Signal::Absent) && matches!(s.2, Signal::Present) {
			// C to B, output 1
			return (State::B, 1);
		} else if matches!(s.1, Signal::Present) && matches!(s.2, Signal::Present) {
			// C to D, output 4
			return (State::D, 4);
		}
	}
	// State D transitions
	else if matches!(s.0, State::D) {
		if matches!(s.1, Signal::Absent) && matches!(s.2, Signal::Absent) {
			// D to C, output 3
			return (State::C, 3);
		}
	}
	(s.0, -1)
}

fn main() {
	let state = State::A;
	println!("Initial State: {:?}", state);

	let (state, out) = update((state, Signal::Present, Signal::Absent));
	println!("State: {:?}, Output: {}", state, out);

	let (state, out) = update((state, Signal::Present, Signal::Absent));
	println!("State: {:?}, Output: {}", state, out);

	let (state, out) = update((state, Signal::Present, Signal::Present));
	println!("State: {:?}, Output: {}", state, out);

	let (state, out) = update((state, Signal::Absent, Signal::Absent));
	println!("State: {:?}, Output: {}", state, out);

	let (state, out) = update((state, Signal::Absent, Signal::Present));
	println!("State: {:?}, Output: {}", state, out);

	let (state, out) = update((state, Signal::Absent, Signal::Present));
	println!("State: {:?}, Output: {}", state, out);

	let (state, out) = update((state, Signal::Present, Signal::Present));
	println!("State: {:?}, Output: {}", state, out);

	let (state, out) = update((state, Signal::Absent, Signal::Absent));
	println!("State: {:?}, Output: {}", state, out);
}
