/*
 * TODO use BufReader??
 * TODO nice error handling
 * TODO get a nice pair of programming socks to boost programming skill up to 4x
 */

fn main() {
    pub fn convert(text: &str ) -> String {

        let result: String = text.chars()
        .map(|x| match x {
            'l' => 'w',
            'L' => 'W',
            'r' => 'w',
            'R' => 'W',
            _  => x
        }).collect();
        result
    }
}
