use std::env;
use itertools::Itertools;


// !0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz
const PLAINTEXT_SPACE: &str = "Rcp36iuE1wWI5QUjtXnFMPxkl7dyJmTLHAoNDOeCbgVs2rv_zfSqG8YKB49!a0hZ";


const MATRIX_KEY: [[isize; 2]; 2] = [
    [ 42, 23 ],
    [ 19, 41 ],
];

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>()[1..].to_vec();

    if args.is_empty() {
        println!("\n\n{}", "-".repeat(40));
        println!("No command line arguments detected displaying help message!");
        println!("Welcome to my home-made encryption program and password manager. ");
        println!("");
        println!("Usage:");
        println!("\t{: <15} {}", "-e   'msg'", "encrypts the message, omit quotes, exclusive");
        println!("\t{: <15} {}", "-d   'enc'", "(WIP) decrypts the message, omit quotes");
        println!("\t{: <15} {}", "-k", "show encrypted passwords.");
        println!("\t{: <15} {}", "-c", "show current working msg characters.");
        println!("\n");

        std::process::exit(0);
    }

    let joined: String = args.join("").replace("-", "").to_lowercase();

    if args[0].to_lowercase().replace("-", "") == "e" && args.len() >= 2 {
        println!("Encrypting messages...");
        for password in &args[1..] {
            let cipher = encrypt(&password).unwrap();
            println!("{} => {}", password, cipher);
        }
    } else {
        if joined.contains("c") {
            println!("Accepted Characters are: '{}'", PLAINTEXT_SPACE.chars().sorted().collect::<String>());
            println!("Length: {}", PLAINTEXT_SPACE.len());
        } 
        if joined.contains("d") {
            println!("WIP");

            if true == true {
                std::process::exit(0);
            }

            for cipher in &args[1..] {
                let plaintext = decrypt(&cipher).unwrap();
                println!("{}", plaintext);
            }
        } 
        if joined.contains("k") {
            
        }
    }
}

fn encrypt(msg: &String) -> Result<String, String> {
    if msg.len() % 2 == 1 {
        print!("Odd sized input is not allowed");
        std::process::exit(0);
    }

    for c in msg.chars() {
        if !PLAINTEXT_SPACE.contains(c) {
            println!("Character '{}' is not allowed! I haven't figured out how to do that just yet. Do -c to see the allowed characters.", c);
            std::process::exit(0);
        }
    }

    let msg_arr: Vec<char> = msg.chars().collect();
    let mut paired_characters = Vec::new();
    for i in 0..msg.len() / 2 {
        paired_characters.push([msg_arr[2*i].to_string(), msg_arr[2*i + 1].to_string()]);
    }

    let mut encipher_matrices: Vec<[isize; 2]> = vec![];

    for matrix in paired_characters {
        encipher_matrices.push(multiply_2x2_by_2x1_matrices(&MATRIX_KEY, &to_representative_matrix(matrix).unwrap()));
    }

    let mut result_string = String::new();
    for pair in encipher_matrices.iter() {
        let alpha = to_alpha_representation(pair).unwrap();
        result_string.push_str(&alpha[0]);
        result_string.push_str(&alpha[1]);
    }

    Ok(result_string)
}

fn to_representative_matrix(mat: [String; 2]) -> Result<[isize; 2], ()> {
    let one: Option<usize> = PLAINTEXT_SPACE.find(&mat[0]);
    let two: Option<usize> = PLAINTEXT_SPACE.find(&mat[1]);

    if one.is_none() && two.is_none() {
        panic!("Chars '{}' and '{}' are outside the plaintext space. You will not find the key here.", mat[0], mat[1]);
    } else if  one.is_none() {
        panic!("Char '{}' is outside the plaintext space. You will not find the key here.", mat[0]);
    } else if two.is_none() {
        panic!("Char '{}' is outside the plaintext space. You will not find the key here.", mat[1]);
    } else {
        return Ok([one.unwrap() as isize, two.unwrap() as isize]);
    }
}

fn to_alpha_representation(mat: &[isize; 2]) -> Result<[String; 2], ()> {
    let space: Vec<char> = PLAINTEXT_SPACE.chars().collect();
    let space_len = space.len();
    
    Ok(
        [ space[(mat[0] as usize % space_len) as usize].to_string(), 
        space[(mat[1] as usize % space_len) as usize].to_string() ]
    )
}

fn multiply_2x2_by_2x2_matrix(a: &[[isize; 2]; 2], b: &[[isize; 2]; 2]) -> [[isize; 2]; 2] {
    [
        [ a[0][0]*b[0][0] + a[0][1]*b[1][0], a[0][0]*b[0][1] + a[0][1]*b[1][1]],
        [ a[1][0]*b[0][0] + a[1][1]*b[1][0], a[1][0]*b[0][1] + a[1][1]*b[1][1]],
    ]
}

fn multiply_2x2_by_2x1_matrices(a: &[[isize; 2]; 2], b: &[isize; 2]) -> [isize; 2] {
    [
        a[0][0]*b[0] + a[0][1]*b[1], a[1][0]*b[0] + a[1][1]*b[1]
    ]
}

fn decrypt(cipher: &String) -> Result<String, String> {
    if cipher.len() % 2 == 1 {
        print!("Odd sized input is not allowed");
        std::process::exit(0);
    }

    let mut encipher_matrices: Vec<[isize; 2]> = vec![];

    let cipher_chars: Vec<char> = cipher.chars().collect();
    for i in 0..cipher.len() / 2 {
        encipher_matrices.push([
            PLAINTEXT_SPACE.find(cipher_chars[2 * i]).unwrap() as isize,
            PLAINTEXT_SPACE.find(cipher_chars[2 * i + 1]).unwrap() as isize,
        ]);
    }

    let det = MATRIX_KEY[0][0] * MATRIX_KEY[1][1] - MATRIX_KEY[0][1] * MATRIX_KEY[1][0];

    // Calculate modular inverse of the determinant
    let det_mod_inverse = mod_inverse(det, PLAINTEXT_SPACE.len() as isize).ok_or("Determinant is zero")?;
    
    // Calculate the scalar for the inverse key matrix
    let inv_key_scalar = [[MATRIX_KEY[1][1] * det_mod_inverse % PLAINTEXT_SPACE.len() as isize, 
                           -MATRIX_KEY[0][1] * det_mod_inverse % PLAINTEXT_SPACE.len() as isize],
                          [-MATRIX_KEY[1][0] * det_mod_inverse % PLAINTEXT_SPACE.len() as isize, 
                           MATRIX_KEY[0][0] * det_mod_inverse % PLAINTEXT_SPACE.len() as isize]];
    

    let mut result_string = String::new();
    for pair in encipher_matrices.iter() {
        let alpha = to_alpha_representation(&multiply_2x2_by_2x1_matrices(&inv_key_scalar, pair)).unwrap();
        result_string.push_str(&alpha[0]);
        result_string.push_str(&alpha[1]);
    }

    Ok(result_string)
}

fn mod_inverse(mut a: isize, mut m: isize) -> Option<isize> {
    let m0 = m;
    let mut x0 = 0;
    let mut x1 = 1;


    if m == 1 {
        return Some(0);
    }

    while a > 1 {
        // q is quotient
        let q = a / m;

        let mut t = m;
        // m is remainder now, process same as Euclid's algo
        m = a % m;
        a = t;

        t = x0;
        x0 = x1 - q * x0;
        x1 = t;
    }

    // Make x1 positive
    if x1 < 0 {
        x1 += m0;
    }

    Some(x1)
}
