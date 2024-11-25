mod paint;

use crate::paint::Paint;

fn main() {
    let paint: Paint = Paint::builder()
        .stroke('#')
        .fill('|')
        .background('-')
        .build();
    println!("{}", paint.to_string());
}
