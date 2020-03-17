// Page 51 (Ch. 3: Wading into WebAssembly with Rust)
// -To start writing code to manave the game board,
//  the first thing we want to do is write some code
//  to manage a GamePiece.
// -We will create an enum for piece color
//  and a struct to represent a single piece.

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GamePiece {
    pub color: PieceColor,
    pub crowned: bool,
}
impl GamePiece {
    // Create a new game piece of a given color
    pub fn new(color: PieceColor) -> GamePiece {
        GamePiece {
            color,
            crowned: false,,
        }
    }
    // Create a new piece of a given color 
    // with a crown on top
    pub fn crowned(p: GamePiece) -> GamePiece {
        GamePiece {
            color: p.color,
            crowned: true,
        }
    }
}

// Page 52 (Ch. 3: Wading into WebAssembly with Rust)
// -derive macro is used to auto-generate boilerplate
//  code to deal with common things most data-structures
//  in Rust need
//  (like the ability to be compared, copied, cloned, 
//   and printed out for debug purposes).

// -Implement coordinate abstraction
// -And some preliminary logic to support game rules
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Coordinate(pub usize, pub usize); // Define a tuple struct (strongly typed tuple with public fields)
impl Coordinate {
    pub fn on_board(self) -> bool {
        let Coordinate(x, y) = self;
        x <= 7 && y <= 7
    }

    // Return a list of potential jomp targets from a given coordinate
    pub fn jump_targets_from(&self) -> impl Iterator<Item = Coordinate> {
        let mut jumps = Vec::new();
        let Coordinate(x, y) = *self;
        if y >= 2 {
            jumps.push(Coordinate(x + 2, y - 2));
        }
        jumps.push(Coordinate(x + 2, y + 2));
        if x >= 2 && y >= 2 {
            jumps.push(Coordinate(x - 2, y - 2));
        }
        if x >= 2 {
            jumps.push(Coordinate(x - 2, y + 2));
        }
        jumps.into_iter()
    }

    // Return a list of potential move (adjacent space) targets given the current coordinate location
    pub fn move_targets_from(&self) -> impl Iterator<Item = Coordinate> {
        let mut moves = Vec::new();
        let Coordinate(x, y) = *self;
            if x >= 1 {
            moves.push(Coordinate(x - 1, y + 1));
        }
        moves.push(Coordinate(x + 1, y + 1));
        if y >= 1 {
            moves.push(Coordinate(x + 1, y - 1));
        }
        if x >= 1 && y >= 1 {
            moves.push(Coordinate(x - 1, y - 1));
        }
        moves.into_iter()
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Move {
        pub from: Coordinate,
        pub to: Coordinate,
    }
    impl Move {
    pub fn new(from: (usize, usize), to: (usize, usize)) -> Move {
        Move {
            from: Coordinate(from.0, from.1),
            to: Coordinate(to.0, to.1),
        }
    }
}

// Any assignment that isn't a reference is a move.

// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
// -Keeping track of what parts of code are using what data on the heap, 
//  minimizing the amount of duplicate data on the heap, 
//  and cleaning up unused data on the heap so you don’t run out of 
//  space are all problems that ownership addresses. 
// -Once you understand ownership, you won’t need to think about 
//  the stack and the heap very often, but knowing that managing heap
//  data is why ownership exists can help explain why it works the way it does.