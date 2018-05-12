extern crate uhn;

use uhn::piece;

fn main() {
    println!("   ~~\n");
    for piece_name in uhn::piece::Name::all() {
        let mut piece = piece_name.to_piece();
     
        for _ in 0..piece.len() {
            println!("{}", piece.draw());
            piece.rot(true);
        }

        println!("   ~~\n");
    }
}
