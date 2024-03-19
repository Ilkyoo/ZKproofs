use lambdaworks_math::{
    cyclic_group::IsGroup,
    elliptic_curve::{
        short_weierstrass::curves::bls12_381::curve::BLS12381Curve, traits::IsEllipticCurve},
    unsigned_integer::element::UnsignedInteger
};



fn main() {
    // secret key in hex
    let sec_key_hex = "0x6C616D6264617370"; // lambdasp
    // sec_key_hex to UnsignedInteger
    let sec_key: UnsignedInteger<6>= UnsignedInteger::from_hex(sec_key_hex).unwrap(); // NOTE: unwrap() may panic
    println!("sec_key -- {:#?}\n",sec_key);

    // get generator of BLS12-381 (y^2=x^3+4)
    let gen_pt = BLS12381Curve::generator(); 
    println!("generator -- {:#?}\n", gen_pt);

    // get public key: add generator to itself sec_key times
    let pub_key = gen_pt.operate_with_self(sec_key); 
    println!("pub_key -- {:#?}\n", pub_key);
    
    // public key in hex: get x coordinate of the usual (x,y) representation to transform to hex
    let pub_key_hex = UnsignedInteger::to_hex(pub_key.to_affine().x().value());
    println!("pub_key.to_affine() -- {:#?}",pub_key.to_affine());
    println!("pub_key.to_affine().x() -- {:#?}",pub_key.to_affine().x());
    println!("5hex public key {}\n",pub_key_hex);
    // result is 773B526CAB2D1FEC900E6AD51E75BBE8EF6395D02F2FBFC84A1B09118FA4121A6EA675532E89A7B43AF5B9FFF9FE7F8

}



/*
fn main() {
    println!("Hello, world!");
}
*/