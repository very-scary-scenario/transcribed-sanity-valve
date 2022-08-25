mod read_albums;

fn main() {
    let albums = read_albums::read_albums();
    dbg!(albums);
}
