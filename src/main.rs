
fn ack(x: i128, y: i128) -> i128{

    if x == 0{
        y + 1
    }
    else if y == 0 {
        return ack(x-1, 1);
    }
    else {
        ack(x - 1, ack(x, y - 1))
    }
}
fn main() {
    let caca = ack(2, 2);
    print!("{}", caca);
}