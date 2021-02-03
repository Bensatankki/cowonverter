fn main(){
    
}
mod converter {
    
    #[allow(dead_code)]
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

