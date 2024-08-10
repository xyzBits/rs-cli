use rand::seq::SliceRandom;

// 常量，需要标注类型，否则编译器无法识别
// b 放在字符串前面，会将字符串转换为字节数组

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<String> {
    // 生成随机数, rng.gen_range(0..10) 生成 0 到 10 之间的随机数，每调用一次，都生成不一样的随机数
    let mut rng = rand::thread_rng();

    // 先将各种字符向 password 中 push 一个，保证所有的 字符至少有一个在  password 中
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if upper {
        chars.extend_from_slice(UPPER);
        // Choose 返回 slice 中的一个随机元素，随机数生成器 rng 是传入后调用 gen_range 生成的
        // 如果 UPPER 是空的，会 panic 并且将 expect 中的信息带给  panic
        password.push(*UPPER.choose(&mut rng).expect("UPPER won't be empty"));
    }

    if lower {
        chars.extend_from_slice(LOWER);
        password.push(*UPPER.choose(&mut rng).expect("LOWER won't be empty"));
    }

    if number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("NUMBER won't be empty"));
    }

    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("SYMBOL won't be empty"));
    }

    for _ in 0..(length - password.len() as u8) {
        let c = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context");
        password.push(*c);
    }

    // 打乱 password 中的元素顺序
    password.shuffle(&mut rng);

    let password = String::from_utf8(password)?;
    println!("{}", password);

    let estimate = zxcvbn::zxcvbn(&password, &[])?;
    eprintln!("Password strength: {}", estimate.score());

    Ok(password)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_gen_pass() {
        let result = process_genpass(10, true, true, true, true);
        assert!(result.is_ok());

        let mut rng = rand::thread_rng();

        for _ in 0..10 {
            let result = rng.gen_range(0..10);
            println!("{}", result);
        }

        let message = b"";
        let result = message.choose(&mut rng).expect("message is empty");
    }
}
