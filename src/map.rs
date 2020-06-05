use core::fmt;
use crate::map::Color::{Gray, Red, Blue, Black};
use rand::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    Gray,
    Red,
    Blue,
    Black
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let char = match self {
            Gray => 'N',
            Red => 'R',
            Blue => 'B',
            Black => 'X'
        };
        write!(f, "{}", char)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Cell<'a> {
    pub color: Color,
    pub word: &'a str,
    pub revealed: bool
}

impl fmt::Display for Cell<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.revealed {
            f.pad(&format!("{}", self.color))
        } else {
            f.pad(self.word)
        }
    }
}

pub struct Map<'a> {
    cells: Vec<Cell<'a>>
}


impl fmt::Debug for Map<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..5 {
            write!(f, "{} {} {} {} {}\n",
                   self.cells[i*5].color, self.cells[i*5+1].color, self.cells[i*5+2].color, self.cells[i*5+3].color, self.cells[i*5+4].color)?;
        }
        let max_len = self.cells.iter().map(|x| x.word.len()).max().unwrap();
        for i in 0..5 {
            write!(f, "{:width$} {:width$} {:width$} {:width$} {:width$}\n",
                   self.cells[i*5].word, self.cells[i*5+1].word, self.cells[i*5+2].word, self.cells[i*5+3].word, self.cells[i*5+4].word, width=max_len)?;
        }
        Ok(())
    }
}

impl fmt::Display for Map<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let max_len = self.cells.iter().map(|x| x.word.len()).max().unwrap();
        for i in 0..5 {
            for j in 0..5 {
                write!(f, "{:width$} ", self.cells[i*5+j], width=max_len)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Map<'_> {
    pub fn new<'a>(words: &'a Vec<&str>) -> Map<'a> {
        let mut colors = vec![Gray; 7];
        colors.append(&mut vec![Red; 9]);
        colors.append(&mut vec![Blue; 8]);
        colors.push(Black);

        let mut rng = thread_rng();
        colors.shuffle(&mut rng);
        let words: Vec<&&str> = words.into_iter().choose_multiple(&mut rng, 25);

        let mut cells = Vec::with_capacity(25);
        for i in 0..25 {
            cells.push(Cell { color: colors[i], word: words[i], revealed: false });
        }
        Map{cells}
    }
}