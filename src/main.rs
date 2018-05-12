extern crate uhn;

fn main() {
    
    println!("     ~~\n");
    for piece_name in uhn::piece::Name::all() {
        let mut piece = piece_name.to_piece();
     
        for _ in 0..piece.len() {
            println!("{}", piece.draw());
            piece.rot(true);
        }

        println!("     ~~\n");
    }
    

    for piece_name in uhn::piece::Name::all() {
        let mut piece = piece_name.to_piece();
        let mut well = uhn::well::Well::new();

        let spawn_pos = well.get_spawn_pos(&piece);

        well.imprint(spawn_pos, &piece);
        println!("{}", well.draw());
    }
}
