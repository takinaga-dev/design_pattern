mod text_editor;

use text_editor::TextEditor;
use std::{cell::RefCell, ops::Index, rc::Rc};

trait Command {
    fn execute(&self);
    fn undo(&self);
}

trait Receiver {
    // Commandが実行したい操作を実装する
    fn write(&self, c: char, position: usize);
    fn undo(&self, position: usize);  // Receiverがテキストバッファ全体の管理をしているためundoで直前の文字を削除する
}

pub struct MacroCommand {
    commands: Vec<Box<dyn Command>>,
}

// テキストフィールド上の任意の位置に文字を描画
/*struct PrintTextField {
    char: Option<char>
}*/

struct PrintTextFieldBuffer {
    buffer: RefCell<Vec<char>>
}

impl PrintTextFieldBuffer {
    fn new() -> Self {
        PrintTextFieldBuffer { buffer:  RefCell::new(Vec::new()) }
    }

    fn print(&self) {
        let buffer = self.buffer.borrow();
        println!("{}", buffer.iter().collect::<String>());
    }

    fn clear(&self) {
        let mut buffer = self.buffer.borrow_mut();
        buffer.clear();
    }
}

impl Receiver for PrintTextFieldBuffer {
    fn write(&self, c: char, position: usize) {
        let mut buffer = self.buffer.borrow_mut();
        if position < buffer.len() {
            buffer[position] = c;
        } else {
            buffer.insert(position, c);
        }
    }

    fn undo(&self, position: usize) {
        let mut buffer = self.buffer.borrow_mut();
        buffer.remove(position);
    }
}

struct PrintCharCommand {
    receiver: Rc<PrintTextFieldBuffer>, // このレシーバーはCharCommandが個別に持っているというより全体のバッファ
    char: char,
    position: usize,
}

impl PrintCharCommand {
    fn new(receiver: Rc<PrintTextFieldBuffer>, c: char, position: usize) -> Self {
        PrintCharCommand { receiver, char: c, position }
    }
}

impl Command for PrintCharCommand {
    // bufferに文字を書き込む
    fn execute(&self) {
        self.receiver.write(self.char, self.position);
        
        self.receiver.print();
    }
    fn undo(&self) {
        self.receiver.undo(self.position);
    }

}
impl Command for MacroCommand {
    fn execute(&self) {
        for command in self.commands.iter() {
            command.execute();
        }
        println!();
    }

    fn undo(&self) {
        unreachable!()
    }
}

impl MacroCommand {
    fn new() -> Self {
        MacroCommand { commands: Vec::new() }
    }

    // cmdは所有権を渡しているけど問題なさそう？
    fn append(&mut self, cmd: Box<dyn Command>) {
        self.commands.push(cmd);
    }

    fn clear(&mut self) {
        self.commands.clear();
    }

    fn undo_last(&mut self) {
         if let Some(cmd) = self.commands.pop() {
            cmd.undo();
        }
    }
}

fn main() {
    //let buffer = Rc::new(PrintTextFieldBuffer::new());

    let mut editor = TextEditor::new();
    editor.type_char('H', 0 as usize);
    editor.type_char('e', 1 as usize);
    editor.type_char('l', 2 as usize);
    editor.type_char('l', 3 as usize);
    editor.type_char('o', 4 as usize);
    editor.type_char('\n',5 as usize);

    editor.undo();
    editor.undo();
    editor.print();
}
