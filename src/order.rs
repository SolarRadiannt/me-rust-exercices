struct Order {
	price: u8,
	quantity: u8,
} impl Order {
	fn is_available(&self) -> bool {
		self.quantity > 0
	}
}

fn struct_exercise() {
	fn test_order_is_available() {
		let order = Order {
			price: 100,
			quantity: 10,
		};
		assert!(order.is_available())
	}

	fn test_order_is_not_available() {
		let order = Order {
			price: 100,
			quantity: 0,
		};
		assert!(order.is_available())
	}

	test_order_is_available();
	test_order_is_not_available();
}
