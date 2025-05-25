// Test module for HMR functionality
// This file can be modified to trigger HMR updates

#[allow(dead_code)] // Used in tests and HMR demonstrations
pub struct TestComponent {
    pub value: i32,
    pub message: String,
}

#[allow(dead_code)] // Used in tests and HMR demonstrations
impl TestComponent {
    pub fn new() -> Self {
        Self {
            value: 42,
            message: "Hello from Orbit HMR!".to_string(),
        }
    }

    pub fn update(&mut self, new_value: i32) {
        self.value = new_value;
        println!("Component updated with value: {}", new_value);
    }

    pub fn render(&self) -> String {
        format!(
            "TestComponent {{ value: {}, message: '{}' }}",
            self.value, self.message
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_creation() {
        let component = TestComponent::new();
        assert_eq!(component.value, 42);
        assert_eq!(component.message, "Hello from Orbit HMR!");
    }

    #[test]
    fn test_component_update() {
        let mut component = TestComponent::new();
        component.update(100);
        assert_eq!(component.value, 100);
    }
}
