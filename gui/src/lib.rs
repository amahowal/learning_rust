pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

pub struct Screen {
    // Vector of any type inside a box that implements the draw trait
    pub components: Vec<Box<dyn Draw>>,
}

// impl<T> Screen<T> THIS IMPLEMENTATION would only allow for the first type at runtime and nothing
// else
// this is using generics and trait bounds
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
