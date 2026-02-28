use regex::Regex;
pub fn to_slug(input: &str) -> String {
    let re_a = Regex::new(r"á|à|ả|ạ|ã|ă|ắ|ằ|ẳ|ẵ|ặ|â|ấ|ầ|ẩ|ẫ|ậ").unwrap();
    let re_e = Regex::new(r"é|è|ẻ|ẽ|ẹ|ê|ế|ề|ể|ễ|ệ").unwrap();
    let re_i = Regex::new(r"i|í|ì|ỉ|ĩ|ị").unwrap();
    let re_o = Regex::new(r"ó|ò|ỏ|õ|ọ|ô|ố|ồ|ổ|ỗ|ộ|ơ|ớ|ờ|ở|ỡ|ợ").unwrap();
    let re_u = Regex::new(r"ú|ù|ủ|ũ|ụ|ư|ứ|ừ|ử|ữ|ự").unwrap();
    let re_y = Regex::new(r"ý|ỳ|ỷ|ỹ|ỵ").unwrap();
    let re_d = Regex::new(r"đ").unwrap();
    let re_space = Regex::new(r"\s+").unwrap();
    let mut source = input.trim().to_lowercase().to_string();
    source = re_a.replace_all(&source, "a").to_string();
    source = re_e.replace_all(&source, "e").to_string();
    source = re_i.replace_all(&source, "i").to_string();
    source = re_o.replace_all(&source, "o").to_string();
    source = re_u.replace_all(&source, "u").to_string();
    source = re_y.replace_all(&source, "y").to_string();
    source = re_d.replace_all(&source, "d").to_string();
    source = re_space.replace_all(&source, "-").to_string();
    // Xóa các ký tự đặc biệt
    let mut sb = String::new();
    for c in source.chars().flat_map(char::to_lowercase) {
        if (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9') || c == '-' {
            sb.push(c);
        }
    }
    sb
}
