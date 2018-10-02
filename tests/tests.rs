extern crate machine_uid;

#[test]
fn test_get() {
	let id = machine_uid::get().unwrap();
	for _ in 1..100 {
		let id2 = machine_uid::get().unwrap();
		assert_eq!(id, id2);
	}
}
