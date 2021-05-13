use super::Passport;

const VALID_PASSPORT: &str = "
    byr:1963
    iyr:2017
    eyr:2029
    hgt:155cm
    hcl:#18171d
    ecl:blu
    pid:316505921
";

#[test]
fn valid_passport() {
    let passport_data = VALID_PASSPORT;

    assert_eq!(passport_data.parse::<Passport>().unwrap().is_valid(), true);
}

#[test]
fn only_birth_year() {
    let passport_data = "byr:1920";

    assert_eq!(passport_data.parse::<Passport>().unwrap().is_valid(), false);
}

#[test]
fn incorrect_birth_year() {
    let too_low = "byr:1919";
    let passport_data = VALID_PASSPORT.replace("byr:1963", too_low);

    assert_eq!(passport_data.parse::<Passport>().unwrap().is_valid(), false);

    let too_high = "byr:2003";
    let passport_data = VALID_PASSPORT.replace("byr:1963", too_high);

    assert_eq!(passport_data.parse::<Passport>().unwrap().is_valid(), false);
}

#[test]
fn incorrect_issue_year() {
    let too_low = "iyr:2009";
    let passport_data = VALID_PASSPORT.replace("iyr:2017", too_low);

    assert_eq!(passport_data.parse::<Passport>().unwrap().is_valid(), false);

    let too_high = "iyr:2021";
    let passport_data = VALID_PASSPORT.replace("iyr:2017", too_high);

    assert_eq!(passport_data.parse::<Passport>().unwrap().is_valid(), false);
}

#[test]
fn incorrect_expiration_year() {
    let too_low = "eyr:2019";
    let passport_data = VALID_PASSPORT.replace("eyr:2029", too_low);

    assert_eq!(passport_data.parse::<Passport>().unwrap().is_valid(), false);

    let too_high = "eyr:2031";
    let passport_data = VALID_PASSPORT.replace("eyr:2029", too_high);

    assert_eq!(passport_data.parse::<Passport>().unwrap().is_valid(), false);
}

#[test]
fn incorrect_height_in_cm() {
    let too_low = "hgt:149cm";
    let passport_data = VALID_PASSPORT.replace("hgt:155cm", too_low);

    assert_eq!(passport_data.parse::<Passport>().unwrap().is_valid(), false);

    let too_high = "hgt:194cm";
    let passport_data = VALID_PASSPORT.replace("hgt:155cm", too_high);

    assert_eq!(passport_data.parse::<Passport>().unwrap().is_valid(), false);
}

#[test]
fn incorrect_height_in_inches() {
    let too_low = "hgt:58in";
    let passport_data = VALID_PASSPORT.replace("hgt:155cm", too_low);

    assert_eq!(passport_data.parse::<Passport>().unwrap().is_valid(), false);

    let too_high = "hgt:77in";
    let passport_data = VALID_PASSPORT.replace("hgt:155cm", too_high);

    assert_eq!(passport_data.parse::<Passport>().unwrap().is_valid(), false);
}

#[test]
fn incorrect_hair_color() {
    let z_in_hex = "hcl:58in";
    let passport_data = VALID_PASSPORT.replace("hcl:#18171d", z_in_hex);

    assert_eq!(passport_data.parse::<Passport>().unwrap().is_valid(), false);

    let seven_symbols_in_hex = "hcl:#18171dd";
    let passport_data = VALID_PASSPORT.replace("hcl:#18171d", seven_symbols_in_hex);

    assert_eq!(passport_data.parse::<Passport>().unwrap().is_valid(), false);
}

#[test]
fn incorrect_eye_color() {
    let z_eye_color = "ecl:zzz";
    let passport_data = VALID_PASSPORT.replace("ecl:blu", z_eye_color);

    assert_eq!(passport_data.parse::<Passport>().unwrap().is_valid(), false);
}
