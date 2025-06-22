trait Builder {
    fn make_title(&mut self, title: String) -> &mut Self;
    fn make_string(&mut self, str: String) -> &mut Self;
    fn make_items(&mut self, items: Vec<String>) -> &mut Self;
}

struct TextBuilder {
    buffer: String
}

impl Builder for TextBuilder {
    fn make_title(&mut self, title: String) -> &mut Self {
        self.buffer += "============================\n";
        self.buffer += &format!("[{}]\n", title);
        self.buffer += "============================\n";
        self.buffer += "\n";

        self
    }

    fn make_string(&mut self, str: String) -> &mut Self {
        
        self
    }

    fn make_items(&mut self, items: Vec<String>) -> &mut Self {
        
        self
    }
}

impl TextBuilder {

    fn new() -> Self {
        TextBuilder { buffer: String::new() }
    }
    fn get_result(self) {

        println!("{}", self.buffer);
    }
}
fn main() {
    let mut text = TextBuilder::new();
    text.make_title("タイトル".to_string()).make_string("内容".to_string()).make_items(["test".to_string(), "sample".to_string()].to_vec());
    
    text.get_result();
}
