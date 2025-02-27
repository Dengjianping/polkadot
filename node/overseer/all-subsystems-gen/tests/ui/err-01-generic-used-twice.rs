#![allow(dead_code)]

use polkadot_overseer_all_subsystems_gen::AllSubsystemsGen;
#[derive(Clone, AllSubsystemsGen)]
struct AllSubsystems<X> {
	a: X,
	b: X,
}

fn main() {
	let all = AllSubsystems::<u16> {
		a: 0_u16,
		b: 1_u16,
	};
	let _all = all.replace_a(|_| 77u8);
}
