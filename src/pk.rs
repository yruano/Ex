/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut hisc = 0;
    let mut hinum = 0;
    
    let mut shapes: Vec<&str> = Vec::new();
    let mut nums: Vec<&str> = Vec::new();
    let mut pks: Vec<&str> = Vec::new();
    
    for i in 0..hands.len() {
        let mut continuous_bits: i16 = 0;
        let mut shape_same = true;
        let mut ck1 = 0;
        let mut ck2 = 0;
        
        for s in hands[i].split(" ") {
            let shape = &s[s.len()-1..];
            let num = &s[..s.len()-1];
            
            shapes.push(shape);
            nums.push(num);

            let i = nums.len();
            let mut prev_num = "";
            let mut prev_shape = "";
            if i as i32 - 1 > 0 {
                prev_num = nums[i - 2];
                prev_shape = shapes[i - 2];
            }
            
            if prev_shape != "" && prev_shape != shape {
                shape_same = false;
            }
            
            let prev_num = cardnum_to_num(prev_num);
            let num = cardnum_to_num(num);
            
            continuous_bits ^= i16::pow(2, (num - 1) as u32);
            if num == 1 {
                continuous_bits ^= i16::pow(2, 13);
            }
        }
        
        nums.sort();
        for (i, num) in nums.iter().enumerate() {
            let mut prev_num = "";
            if i as i32 - 1 > 0 {
                prev_num = nums[i - 2];
            }
            
            if prev_num != "" && prev_num == *num {
                ck1 += 1;
            }else if prev_num != *num {
                if ck1 == 3{
                    ck2 += 3;
                }
                ck2 += 1;
            }
        }
        
        // 스트레이트    
        if continuous_bits & 1 == 1 {
            if continuous_bits == 15873 {
                if shape_same {
                    pks.clear();
                    hisc = 1000000;
                    pks.push(hands[i]);
                }
            } else if (continuous_bits - 8223) % 510 == 0 {
                if hisc < 5 {
                    pks.clear();
                    hisc = 5;
                    pks.push(hands[i]);
                }
            }
        } else if continuous_bits % 62 == 0 {
            if hisc < 5 {
                pks.clear();
                hisc = 5;
                pks.push(hands[i]);
            }
        } else if shape_same {
                if hisc < 6 {
                    pks.clear();
                    hisc = 6;
                    pks.push(hands[i]);
                }
            }else if ck1 == 4 {
                if hisc < 8 {
                    pks.clear();
                    hisc = 8;
                    pks.push(hands[i]);
                }
            }else if ck1 == 4 && ck2 == 1 {
                if hisc < 7 {
                    pks.clear();
                    hisc = 7;
                    pks.push(hands[i]);
                }
            }else if ck1 == 3 && ck2 == 6 {
                if hisc < 4 {
                    pks.clear();
                    hisc = 4;
                    pks.push(hands[i]);
                }
            }else if ck1 == 3 && ck2 == 2 {
                if hisc < 3 {
                    pks.clear();
                    hisc = 3;
                    pks.push(hands[i]);
                }
            }else if ck1 == 2 && ck2 == 3 {
                if hisc < 2 {
                    pks.clear();
                    hisc = 2;
                    pks.push(hands[i]);
                }
            }else if ck1 == 1 && ck2 == 4 {
                if hinum >= 0 {
                    nums.sort();
                    if hinum == nums[5].parse::<i32>().unwrap() {
                        pks.push(hands[i]);
                    }
                    hinum = nums[5].parse::<i32>().unwrap();
                    pks.clear();
                    pks.push(hands[i]);
                }
            }
            else if hisc == hisc {
                pks.push(hands[i]);
            }
        }
    pks
}

fn cardnum_to_num(num: &str) -> i16 {
    if num == "" {
        -1
    } else if "JKQA".contains(num)  {
        if num == "J" { 
            11
        }
        else if num == "Q" {
            12
        }
        else if num == "K" {
            13
        }
        else {
            1
        }
    } else {
        num.parse::<i16>().unwrap()
    }
}