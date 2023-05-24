use std::{
	fmt, io
};


fn main() {
	let mut game = GameManager::console_game();
	game.run();

	//game.game.get_moves_cord(CORD_NONE);
	//let game = Game::new();
    //println!("{}", game);
    
    //println!("{}", Cord::new(0,0));
}

use Square::Empty;
#[derive(Copy, Clone)]//maybe remove, maybe make custom implementation using get_const()
enum Square {
	Empty,
	Piece(Piece)
}

impl Square {
	
	fn get_const(&self) -> Square {
		match self {
			Empty => Empty,
			Square::Piece(piece) => if piece.color {
				match piece.piece_type {
					Pawn   => PAWN_WHITE,
					Knight => KNIGHT_WHITE,
					Bishop => BISHOP_WHITE,
					Rook   => ROOK_WHITE,
					Queen  => QUEEN_WHITE,
					King   => KING_WHITE
				}
			} else {
				match piece.piece_type {
					Pawn   => PAWN_BLACK,
					Knight => KNIGHT_BLACK,
					Bishop => BISHOP_BLACK,
					Rook   => ROOK_BLACK,
					Queen  => QUEEN_BLACK,
					King   => KING_BLACK
				}
			}
		}
	}

	fn val(&self) -> i8 {
		match self {
			Empty => 0,
			Square::Piece(piece) => piece.val()
		}
	}
	
	fn to_char(&self) -> char {
		match self {
			Empty => '.',
			Square::Piece(piece) => piece.to_char()
		}
	}
	
	fn to_char_fancy(&self, invert: bool) -> char {
		match self {
			Empty => '.',
			Square::Piece(piece) => {
				if piece.color != invert {
					match piece.piece_type {
						Pawn   => '♙',
						Knight => '♘',
						Bishop => '♗',
						Rook   => '♖',
						Queen  => '♕',
						King   => '♔'
					}
				} else {
					match piece.piece_type {
						Pawn   => '♟',
						Knight => '♞',
						Bishop => '♝',
						Rook   => '♜',
						Queen  => '♛',
						King   => '♚'
					}
				}
			}
		}
	}
	
	//useless function? remove?
	fn compareSquaresColor(s1: &Square, s2: &Square) -> SquareComparison {
		match s1 {
			Empty => SquareComparison::Empty,
			Square::Piece(piece1) => match s2 {
				Empty => SquareComparison::Empty,
				Square::Piece(piece2) => {
					if piece1.color == piece2.color {
						SquareComparison::ColorSame
					} else {
						SquareComparison::ColorOther
					}
				}
			}
		}
	}
	
	fn compareColor(&self, color: bool) -> SquareComparison {
		match self {
			Empty => SquareComparison::Empty,
			Square::Piece(Piece{color:piece_color, ..}) => {
				if color == *piece_color {
					SquareComparison::ColorSame
				} else {
					SquareComparison::ColorOther
				}
			}
		}
	}
}

enum SquareComparison {
	Empty,
	ColorSame,
	ColorOther
}

impl SquareComparison {
	fn empty_or_captureable(&self) -> bool {
		if let SquareComparison::ColorSame = self {
			false
		} else {
			true
		}
	}
	
	fn is_empty(&self) -> bool {
		if let SquareComparison::Empty = self {
			true
		} else {
			false
		}
	}
	
	fn is_piece(&self) -> bool {
		if let SquareComparison::Empty = self {
			false
		} else {
			true
		}
	}
	
	//useless?
	fn empty_or_same_color(&self) -> bool {
		if let SquareComparison::ColorOther = self {
			false
		} else {
			true
		}
	}
}

#[derive(Copy, Clone)]
struct Piece {
	color : bool,
	piece_type : PieceType
}

impl Piece {
	fn val(&self) -> i8 {
		let v = self.piece_type.val();
		if self.color {
			v
		} else {
			-v
		}
	}
	
	fn to_char(&self) -> char {
		let t = self.piece_type.to_char();
		if self.color {
			t.to_ascii_uppercase()
		} else {
			t
		}
	}
}

use PieceType::*;
#[derive(Copy, Clone)]
enum PieceType {
	Pawn,
	Knight,
	Bishop,
	Rook,
	Queen,
	King
}

impl PieceType {
	fn val(&self) -> i8 {
		match self {
			Pawn   => 1,
			Knight => 3,
			Bishop => 3,
			Rook   => 5,
			Queen  => 9,
			King   => 0
		}
	}
	
	fn to_char(&self) -> char {
		match self {
			Pawn   => 'p',
			Knight => 'n',
			Bishop => 'b',
			Rook   => 'r',
			Queen  => 'q',
			King   => 'k'
		}
	}
}

struct CordOffset {
	x: i8,
	y: i8
}

#[derive(Copy, Clone)]
struct Cord {
	x: u8,
	y: u8
}

impl Cord {
	fn new(x:u8, y:u8) -> Cord {
		Cord{ x, y }
	}
	
	fn from_tuple(x: (u8, u8)) -> Cord {
		Cord{ x:x.0, y:x.1 }
	}
	
	fn offset_xy(&self, x:i8, y:i8) -> Cord {
		Cord{
			x:(self.x as i8 + x) as u8,
			y:(self.y as i8 + y) as u8
		}
	}
	
	
	fn offset_x(&self, x:i8) -> Cord {
		Cord{
			x:(self.x as i8 + x) as u8,
			y:self.y
		}
	}
	
	fn offset_y(&self, y:i8) -> Cord {
		Cord{
			x:self.x,
			y:(self.y as i8 + y) as u8
		}
	}
	
	fn offset(&self, o: CordOffset) -> Cord {
		Cord{
			x:(self.x as i8 + o.x) as u8,
			y:(self.y as i8 + o.y) as u8
		}
	}
	
	
	fn sum(&self, cord: &Cord) -> Cord {
		Cord{
			x:self.x + cord.x,
			y:self.y + cord.y
		}
	}
	
	fn is_in_range(&self) -> bool {
		if self.x_is_in_range() {
			return self.y_is_in_range()
		}
		false
	}
	
	fn x_is_in_range(&self) -> bool {
		if self.x >= 0 {
			return self.x < 8
		}
		false
	}
	fn y_is_in_range(&self) -> bool {
		if self.y >= 0 {
			return self.y < 8
		}
		false
	}
}

impl fmt::Display for Cord {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}",
        	(self.x + 97) as char,
        	self.y+1
        )
    }
}

#[derive(Copy, Clone)]
struct Move {
	from: Cord,
	to: Cord
}

impl Move {
	fn new(from: Cord, to: Cord) -> Self {
		Move{from,to}
	}
}

impl fmt::Display for Move {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}",
        	self.from, self.to
        )
    }
}

const CORD_NONE: Cord = Cord{ x: 0, y:0 };
const MOVE_NONE: Move = Move{from: CORD_NONE, to: CORD_NONE};

const EMPTY: Square = Empty;

//White
const PAWN_WHITE  : Square = Square::Piece(Piece{ color:true,  piece_type:Pawn   });
const KNIGHT_WHITE: Square = Square::Piece(Piece{ color:true,  piece_type:Knight });
const BISHOP_WHITE: Square = Square::Piece(Piece{ color:true,  piece_type:Bishop });
const ROOK_WHITE  : Square = Square::Piece(Piece{ color:true,  piece_type:Rook   });
const QUEEN_WHITE : Square = Square::Piece(Piece{ color:true,  piece_type:Queen  });
const KING_WHITE  : Square = Square::Piece(Piece{ color:true,  piece_type:King   });

//Black
const PAWN_BLACK  : Square = Square::Piece(Piece{ color:false, piece_type:Pawn   });
const KNIGHT_BLACK: Square = Square::Piece(Piece{ color:false, piece_type:Knight });
const BISHOP_BLACK: Square = Square::Piece(Piece{ color:false, piece_type:Bishop });
const ROOK_BLACK  : Square = Square::Piece(Piece{ color:false, piece_type:Rook   });
const QUEEN_BLACK : Square = Square::Piece(Piece{ color:false, piece_type:Queen  });
const KING_BLACK  : Square = Square::Piece(Piece{ color:false, piece_type:King   });

#[derive(Copy, Clone)]
struct Board ( [[Square; 8]; 8] );

impl Board {
	fn new() -> Board {
		Board(
			[
				[ROOK_WHITE, KNIGHT_WHITE, BISHOP_WHITE,
				 QUEEN_WHITE, KING_WHITE,
				 BISHOP_WHITE, KNIGHT_WHITE, ROOK_WHITE],
				[PAWN_WHITE;8],
				[EMPTY;8],
				[EMPTY;8],
				[EMPTY;8],
				[EMPTY;8],
				[PAWN_BLACK;8],
				[ROOK_BLACK, KNIGHT_BLACK, BISHOP_BLACK,
				 QUEEN_BLACK, KING_BLACK,
				 BISHOP_BLACK, KNIGHT_BLACK, ROOK_BLACK],
			]
		)
	}
	
	fn empty() -> Board {
		Board(
			[
				[EMPTY;8],
				[EMPTY;8],
				[EMPTY;8],
				[EMPTY;8],
				[EMPTY;8],
				[EMPTY;8],
				[EMPTY;8],
				[EMPTY;8]
			]
		)
	}
	
	fn row_str(&self, row: usize) -> String {
		self.0[row].iter().map(
			//|x| x.to_char()
			|x| x.to_char_fancy(true)
		).collect()
	}
	
	fn val(&self) -> i8 {
		self.0.iter().map(
			|x|x.iter().map(
				|y|y.val()
			).sum::<i8>()
		).sum()
	}
	
	fn make_move(&mut self, m: &Move) {
		self.0[m.to.x as usize][m.to.y as usize] =
		 self.0[m.from.x as usize][m.from.y as usize].get_const();
		
		self.0[m.from.x as usize][m.from.y as usize] = EMPTY;
	}
	
}

impl fmt::Display for Board {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\
+------------+
|  abcdefgh  |
| +--------+ |
|8|{}|8|
|7|{}|7|
|6|{}|6|
|5|{}|5|
|4|{}|4|
|3|{}|3|
|2|{}|2|
|1|{}|1|
| +--------+ |
|  abcdefgh  |
+------------+
|eval: {}
+------------+",
        	self.row_str(7),
        	self.row_str(6),
        	self.row_str(5),
        	self.row_str(4),
        	self.row_str(3),
        	self.row_str(2),
        	self.row_str(1),
        	self.row_str(0),
        	self.val()
        )
    }
}

#[derive(Copy, Clone)]
struct Game {
	board: Board,
	turn: bool,
}

impl Game {
	
	fn new() -> Game {
		Game {
			board: Board::new(),
			turn: true
		}
	}
	
	fn turn_str(&self) -> &str {
		if self.turn {
			"white"
		} else {
			"black"
		}
	}
	
	fn make_move(&mut self, m: &Move) {
		self.board.make_move(m);
		self.turn = !self.turn;
	}
	
	/*
	fn get_moves(&self) -> Vec<Move> {
		self.board.0.iter()
			.filter(|x| if let Square::Peice(piece) = x {true})
			.map(|x| if let Square::Peice(piece) = x { x.get_moves()} )
			.collect()
	}
	*/
	
	fn square_at(&self, cord: Cord) -> Square {
		self.board.0[cord.x as usize][cord.y as usize]
	}
	
	fn get_moves_cord(&self, cord: Cord) -> Vec<Move> {
		match self.square_at(cord) {
			Empty => Vec::<Move>::new(),
			Square::Piece(piece) => {
				let color = piece.color;
				match piece.piece_type {
					Pawn   => self.get_pawn_moves(cord, color),
					Knight => self.get_knight_moves(cord, color),
					Bishop => self.get_bishop_moves(cord, color),
					Rook   => self.get_rook_moves(cord, color),
					Queen  => self.get_queen_moves(cord, color),
					King   => self.get_king_moves(cord, color)
				}
			}
		}
	}
	
	fn get_pawn_moves(&self, cord: Cord, color: bool) -> Vec<Move> {
		let mut v = Vec::<Move>::new();
		if color {//white
			let to = cord.offset_y(1);
			if let Empty = self.square_at(to) {
				v.push(Move::new(cord,to));
				
				if cord.y == 1 {
					let to = cord.offset_y(2);
					if let Empty = self.square_at(to) {
						v.push(Move::new(cord,to));
					}
				}
			}
			
			{
				let to = to.offset_x(1);
				if let SquareComparison::ColorOther = self.square_at(to).compareColor(color) {
					v.push(Move::new(cord,to));
				}
			}
			
			{
				let to = to.offset_x(-1);
				if let SquareComparison::ColorOther = self.square_at(to).compareColor(color) {
					v.push(Move::new(cord,to));
				}
			}
			
			
		} else {//black
			let to = cord.offset_y(-1);
			if let Empty = self.square_at(to) {
				v.push(Move::new(cord,to));
				
				if cord.y == 6 {
					let to = cord.offset_y(-2);
					if let Empty = self.square_at(to) {
						v.push(Move::new(cord,to));
					}
				}
			}
			
			{
				let to = to.offset_x(1);
				if let SquareComparison::ColorOther = self.square_at(to).compareColor(color) {
					v.push(Move::new(cord,to));
				}
			}
			
			{
				let to = to.offset_x(-1);
				if let SquareComparison::ColorOther = self.square_at(to).compareColor(color) {
					v.push(Move::new(cord,to));
				}
			}
		}
		v
	}
	
	//maybe optimize with out of bounds check
	fn get_knight_moves(&self, cord: Cord, color: bool) -> Vec<Move> {
		let mut v = Vec::<Move>::new();
		
		let to = cord.offset_xy(1,2);
		self.add_if_unobstructed_cords(&mut v, &cord, &to, color);
		
		let to = cord.offset_xy(2,1);
		self.add_if_unobstructed_cords(&mut v, &cord, &to, color);
		
		let to = cord.offset_xy(-1,2);
		self.add_if_unobstructed_cords(&mut v, &cord, &to, color);
		
		let to = cord.offset_xy(-2,1);
		self.add_if_unobstructed_cords(&mut v, &cord, &to, color);
		
		let to = cord.offset_xy(1,-2);
		self.add_if_unobstructed_cords(&mut v, &cord, &to, color);
		
		let to = cord.offset_xy(2,-1);
		self.add_if_unobstructed_cords(&mut v, &cord, &to, color);
		
		let to = cord.offset_xy(-1,-2);
		self.add_if_unobstructed_cords(&mut v, &cord, &to, color);
		
		let to = cord.offset_xy(-2,-1);
		self.add_if_unobstructed_cords(&mut v, &cord, &to, color);
		
		v
	}
	
	fn get_bishop_moves(&self, cord: Cord, color: bool) -> Vec<Move> {
		let mut v = Vec::<Move>::new();
		v
	}
	
	fn get_rook_moves(&self, cord: Cord, color: bool) -> Vec<Move> {
		let mut v = Vec::<Move>::new();
		v
	}
	
	fn get_queen_moves(&self, cord: Cord, color: bool) -> Vec<Move> {
		let mut v = Vec::<Move>::new();
		v
	}
	
	fn get_king_moves(&self, cord: Cord, color: bool) -> Vec<Move> {
		let mut v = Vec::<Move>::new();
		v
	}
	
	fn add_if_unobstructed( &self, moves: &mut Vec<Move>, m: Move, color: bool) {
		if m.to.is_in_range() {
			let square = self.square_at(m.to);
			match square {
				Empty => moves.push(m),
				Square::Piece(piece) => if piece.color != color {
					moves.push(m);
				}
			}
		}
	}
	
	fn add_if_unobstructed_cords(&self, moves: &mut Vec<Move>, from: &Cord, to: &Cord, color: bool) {
		if to.is_in_range() {
			let square = self.square_at(*to);
			match square {
				Empty => moves.push(Move::new(*from,*to)),
				Square::Piece(piece) => if piece.color != color {
					moves.push(Move::new(*from,*to));
				}
			}
		}
	}
	
	fn is_move_legal(&self, m: &Move) -> bool {
		false
	}
	
	fn is_in_check(&self) -> bool {
		false
	}
	
	fn game_state(&self) -> GameState {
		//todo: add logic to determine win, loss, stalemate
		GameState::Ongoing
	}
	
}

impl fmt::Display for Game {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\
{}
|turn: {} |
+------------+",
        	self.board,
        	self.turn_str()
        )
    }
}

#[derive(Copy, Clone)]
enum GameState {
	Ongoing,
	Stalemate,
	Win(bool)//color
}

impl fmt::Display for GameState {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
        	match self {
        		GameState::Ongoing => "Ongoing",
        		GameState::Stalemate => "Stalemate",
        		GameState::Win(player_white) => if *player_white {
        			"White Won!"
        		} else {
        			"Black Won!"
        		}
        	}
        )
    }
}

enum Player{
	Console,
	ComputerV1
}

impl Player {
	
	//mutable self to allow persistent reusable nodes for some engines
	fn get_move(&mut self, game: &Game) -> Move {
		match self {
			Player::Console => self.console_move(game),
			Player::ComputerV1 => MOVE_NONE//placeholder value, replace
		}
	}
	
	fn console_move(&mut self, game: &Game) -> Move {
		println!("Enter move:");
    	let mut guess = String::new();
 
    	io::stdin().read_line(&mut guess).expect("failed to readline");
    	
    	let mut chars = guess.chars();
    	
    	let char1 = chars.next().expect("1st character not found") as u8 -97;
    	let char2 = chars.next().expect("2nd character not found") as u8 -49;
    	let char3 = chars.next().expect("3rd character not found") as u8 -97;
    	let char4 = chars.next().expect("4th character not found") as u8 -49;
    	
    	Move{
    		from: Cord{ x: char2, y: char1 },
    		to:   Cord{ x: char4, y: char3 }
    	}
	}
	
	fn is_human(&self) -> bool {
		match self {
			Player::Console => true,// | other
			_ => false
		}
	}
	
}

enum DisplayType {
	Console,
	GUI
}

impl DisplayType {
	fn show(&self, game: &Game){
		match self{
			DisplayType::Console => println!("{}", game),
			DisplayType::GUI => println!("Not Implemented Yet!")
		}
	}
}

struct GameManager {
	game: Game,
	player_white: Player,
	player_black: Player,
	display: DisplayType
}

impl GameManager {
	fn new() -> GameManager {
		GameManager{
			game: Game::new(),
			player_white: Player::ComputerV1,
			player_black: Player::ComputerV1,
			display: DisplayType::Console
		}
	}
	
	fn console_game() -> GameManager {
		GameManager{
			game: Game::new(),
			player_white: Player::Console,
			player_black: Player::Console,
			display: DisplayType::Console
		}
	}
	
	fn run(&mut self) -> GameState {
		while let GameState::Ongoing = self.game.game_state() {
			self.display.show(&self.game);
			self.game.make_move(
				&if self.game.turn 	{ &mut self.player_white }
				else 				{ &mut self.player_black }
					.get_move(&self.game)
			);
		}
		self.display.show(&self.game);
		self.game.game_state()
	}
}
