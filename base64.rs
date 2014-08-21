#[allow(non_snake_case_functions)]
#[allow(unnecessary_parens)]

// Author - Vikram
// Contact - @TheVikO_o
// License - MIT
 
mod Base64 {

    // Debug Module
    pub mod Debug {
        use std::str;
        
        // Print bytes as UTF-8 string
        pub fn PrintBytes(data: Vec<u8>) {
            println!("{}", str::from_utf8(data.as_slice()));
        }
    }
    
    // Encode array of u8 bytes
    pub fn EncodeBytes(source: &[u8])->Vec<u8> {
        let bytes = source;
        let mut i =0;
        let mut output:Vec<u8> = vec![];
        let maxLen = bytes.len();
        
        while i < maxLen {
            let b1 = (bytes[i] & 0b11111100) >> 2;
       
            output.push(BaseIndex(b1));
            
            if (i + 1) < maxLen {
                let b2 = (((bytes[i] & 0b11) << 4) | ((bytes[i+1] & 0b11110000) >> 4));
                output.push(BaseIndex(b2));
                
                if (i + 2) < maxLen {
                    let b3 = (((bytes[i+1] & 0b1111) << 2) | ((bytes[i+2] & 0b11000000) >> 6));
                    output.push(BaseIndex(b3));
                    
                    let b4 = bytes[i+2] & 0b111111;
                    output.push(BaseIndex(b4));
                } else {
                    let b3 = ((bytes[i+1] & 0b1111) << 2);
                    output.push(BaseIndex(b3));
                    output.push(b'=');
                }
                
            } else {
                output.push(BaseIndex(((bytes[i] & 0b11) << 4)));
                output.push(b'=');
                output.push(b'=');
            }
            
            i += 3;
        }
        
        output
    }
    
    // Encode str
    pub fn EncodeStr(source: &str) -> Vec<u8> {
        EncodeBytes(source.as_bytes())
    }
    
    // Decode array of u8 bytes
    pub fn DecodeBytes(source:&[u8]) -> Vec<u8> {
        
        let bytes = source;
        let mut i =0;
        let mut output:Vec<u8> = vec![];
        let maxLen = bytes.len();
        
        if source.len() % 4 != 0 {
            return output;
        }
        
        while i < maxLen {
            let value1:u8 = BaseIndexToByte(bytes[i]);
            let value2:u8 = BaseIndexToByte(bytes[i+1]);
            let value3:u8 = BaseIndexToByte(bytes[i+2]);
            let value4:u8 = BaseIndexToByte(bytes[i+3]);
            
            let b1: u8 = value1 << 2 | (value2 & 0b00110000) >> 4;
            let b2: u8 = (value2 & 0b1111) << 4 | (value3 & 0b111100) >> 2;
            let b3: u8 = (value3 & 0b11) << 6 | value4;
            
            output.push(b1);
            output.push(b2);
            output.push(b3);
            i += 4;
        }
        
        output
    }
    
    // Decode str
    pub fn DecodeStr(source:&str)->Vec<u8>{
        DecodeBytes(source.as_bytes())
    }
    
    // Convert byte to base64 rep
    fn BaseIndex(index:u8) -> u8 {
        match index {
            62 => {b'+'}
            63 => {b'/'}
            0..25 => { b'A' + index }
            26..51 => { b'a' + index - 26 }
            _ => { b'0' + index  - 52 }
        }
    }
    
    // Convert base64 rep to byte
    fn BaseIndexToByte(source:u8)->u8{
        match source {
            b'+' => { 62 }
            b'/' => { 63 }
            b'A'..b'Z' => { source - b'A'}
            b'a'..b'z' => { 26 + source - b'a'}
            b'0'..b'9' => { 52 + source - b'0'}
            _ => { 0 }
        }
    }
}

// Some tests
fn main() {   
    
    let mut encoded = ::Base64::EncodeStr("Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut");
    ::Base64::Debug::PrintBytes(encoded);
    let mut decoded = ::Base64::DecodeStr("TG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQsIGNvbnNlY3RldHVyIGFkaXBpc2ljaW5nIGVsaXQsIHNlZCBkbyBlaXVzbW9kIHRlbXBvciBpbmNpZGlkdW50IHV0");
    ::Base64::Debug::PrintBytes(decoded);
    
    encoded = ::Base64::EncodeStr("ab");
    ::Base64::Debug::PrintBytes(encoded);
    decoded = ::Base64::DecodeStr("YWI=");
    ::Base64::Debug::PrintBytes(decoded); 
    
    encoded = ::Base64::EncodeStr("easure.");
    ::Base64::Debug::PrintBytes(encoded);
    decoded = ::Base64::DecodeStr("ZWFzdXJlLg==");
    ::Base64::Debug::PrintBytes(decoded); 
    
    encoded = ::Base64::EncodeStr("C:\\Users\\AppData\\Local\\Viky Notes\\VikyNotes.exe");
    ::Base64::Debug::PrintBytes(encoded);
    decoded = ::Base64::DecodeStr("QzpcVXNlcnNcQXBwRGF0YVxMb2NhbFxWaWt5IE5vdGVzXFZpa3lOb3Rlcy5leGU=");
    ::Base64::Debug::PrintBytes(decoded);
  
    
    decoded = ::Base64::DecodeStr("TW");
    ::Base64::Debug::PrintBytes(decoded);
}