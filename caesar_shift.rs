fn caesar_shift(m:&[u8], d:int)->Vec<u8>{
    m.iter()
        .map(|x| x.to_int().unwrap())
        .map(|x| ((x + d) % 256) as u8)
        .collect::<Vec<u8>>()
}

fn main(){

    // Same as caesar_shift, but closure
    let cz = | m:&[u8], d:int |  
        m.iter()
            .map(|x| x.to_int().unwrap())
            .map(|x| ((x + d) % 256) as u8)
            .collect::<Vec<u8>>();
    
    let input = "abcdef".as_bytes();
    let key =5;
    let enc_str = cz(input, key);
    let dec_str = cz(enc_str.as_slice(), -key);
    println!("{} -> {} -> {}", input , enc_str, dec_str);
    
    println!("-------------------------");
    println!("Char by char test");
    
    for i in range(0i,256) {
        let ip = i as u8;
        let enc = cz([ip], key);
        let dec = cz(enc.as_slice(), -key);
        println!("{} -> {} -> {}", ip , enc, dec);
    } 
}