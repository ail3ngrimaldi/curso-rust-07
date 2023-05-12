// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

impl<T> Vec<T> {
    fn push(&mut self, item: T) {
        self.data.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
}#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic() {
        let mut shopping_list: Vec<?> = Vec::new();
        shopping_list.push("milk");
    }

}