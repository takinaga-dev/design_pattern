use std::rc::Rc;
use crate::Command;
use crate::MacroCommand;
use crate::PrintCharCommand;
use crate::PrintTextFieldBuffer;

pub struct TextEditor {
  buffer: Rc<PrintTextFieldBuffer>,
  history: MacroCommand,
}

impl TextEditor {
  pub fn new() -> Self {
    TextEditor { buffer: Rc::new(PrintTextFieldBuffer::new()), history: MacroCommand::new() }
  }

  pub fn undo(&mut self) {
    self.history.undo_last();
  }

  pub fn clear_history(&mut self) {
    self.history.clear();
  }

  pub fn type_char(&mut self, c: char, position: usize) {
    let cmd = PrintCharCommand::new(Rc::clone(&self.buffer), c, position);
    cmd.execute();
    self.history.append(Box::new(cmd));
    
  }
  pub fn print(&self) {
    self.buffer.clear();
    self.history.execute();
  }
}