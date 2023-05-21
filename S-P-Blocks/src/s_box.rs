use hex;

pub fn get_s_box() -> Vec<u8> {
    // https://en.wikipedia.org/wiki/Rijndael_S-box  - AES S-box
    let s_block_string = "
63 	7c 	77 	7b 	f2 	6b 	6f 	c5 	30 	01 	67 	2b 	fe 	d7 	ab 	76
ca 	82 	c9 	7d 	fa 	59 	47 	f0 	ad 	d4 	a2 	af 	9c 	a4 	72 	c0
b7 	fd 	93 	26 	36 	3f 	f7 	cc 	34 	a5 	e5 	f1 	71 	d8 	31 	15
04 	c7 	23 	c3 	18 	96 	05 	9a 	07 	12 	80 	e2 	eb 	27 	b2 	75
09 	83 	2c 	1a 	1b 	6e 	5a 	a0 	52 	3b 	d6 	b3 	29 	e3 	2f 	84
53 	d1 	00 	ed 	20 	fc 	b1 	5b 	6a 	cb 	be 	39 	4a 	4c 	58 	cf
d0 	ef 	aa 	fb 	43 	4d 	33 	85 	45 	f9 	02 	7f 	50 	3c 	9f 	a8
51 	a3 	40 	8f 	92 	9d 	38 	f5 	bc 	b6 	da 	21 	10 	ff 	f3 	d2
cd 	0c 	13 	ec 	5f 	97 	44 	17 	c4 	a7 	7e 	3d 	64 	5d 	19 	73
60 	81 	4f 	dc 	22 	2a 	90 	88 	46 	ee 	b8 	14 	de 	5e 	0b 	db
e0 	32 	3a 	0a 	49 	06 	24 	5c 	c2 	d3 	ac 	62 	91 	95 	e4 	79
e7 	c8 	37 	6d 	8d 	d5 	4e 	a9 	6c 	56 	f4 	ea 	65 	7a 	ae 	08
ba 	78 	25 	2e 	1c 	a6 	b4 	c6 	e8 	dd 	74 	1f 	4b 	bd 	8b 	8a
70 	3e 	b5 	66 	48 	03 	f6 	0e 	61 	35 	57 	b9 	86 	c1 	1d 	9e
e1 	f8 	98 	11 	69 	d9 	8e 	94 	9b 	1e 	87 	e9 	ce 	55 	28 	df
8c 	a1 	89 	0d 	bf 	e6 	42 	68 	41 	99 	2d 	0f 	b0 	54 	bb 	16
";

    // no need for 2D array - can use 1D one for better speed (if we want to go even faster can convert to [u8; 256])
    let mut s_box: Vec<u8> = Vec::new();
    for line in s_block_string.lines() {
        // split the string of bytes by whitespace
        let  string_bytes: Vec<&str> = line.split(" 	").collect();
        for val in string_bytes {
            // actually create a table of bytes
            s_box.append(&mut hex::decode(val).unwrap());
        }
    }
    s_box
}

pub fn get_inverse_s_box() -> Vec<u8> {
    let s_block = get_s_box();
    let mut inv_s_box: Vec<u8> = vec![0; 256];


    for (i, bytes) in s_block.into_iter().enumerate() {
        let row = (bytes >> 4) & 0x0f; // first 4 bits of table value
        let col = bytes & 0x0f; // last 4 bits of table value
        // row * 16 + col = 1D index for 2D array
        inv_s_box[usize::from(row) * 16 + usize::from(col)] = i as u8; // use index as a value
    }

    // for i in 0..16 {
    //     for j in 0..16 {
    //         print!("{} ", hex::encode([inv_s_box[i * 16 + j]]));
    //     }
    //     println!();
    // }

    inv_s_box
}

pub fn transform(data: &Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let s_box: Vec<u8> = get_s_box();
    for byte in data { // for each byte(8 bits) chunk of data...
        let row = (byte >> 4) & 0x0f;  // first 4 bits of data chunk
        let col = byte & 0x0f; // last 4 bits of data chunk
        let new_byte: u8 = s_box[usize::from(row) * 16 + usize::from(col)]; // find corresponding entry
        result.push(new_byte); // push the result to the the result vector
    }
    result
}

pub fn inverse_transform(data: &Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let inv_s_box: Vec<u8> = get_inverse_s_box();
    for byte in data { // for each byte(8 bits) chunk of data...
        let row = (byte >> 4) & 0x0f; // first 4 bits of data chunk
        let col = byte & 0x0f; // last 4 bits of data chunk
        let new_byte: u8 = inv_s_box[usize::from(row) * 16 + usize::from(col)]; // find corresponding entry
        result.push(new_byte); // push the result to the the result vector
    }
    result
}