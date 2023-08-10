use gloo_console::log;

pub struct YewColor {
    hue: i32,
    saturation: i32,
    value: i32,
    alpha: i32,
    enable_alpha: bool,
    format: String
}

impl YewColor {
    pub fn new() ->Self {
        Self {
            hue: 0,
            saturation: 100,
            value: 100,
            alpha: 100,
            enable_alpha: false,
            format: "hex".to_string()
        }
    }

    pub fn get_hue(&self)->i32 {
        self.hue
    }
    pub fn set_hue(&mut self, hue:i32) {
        self.hue = hue;
    }

    pub fn hsv2rgb(h:f64, s:f64, v:f64)->(u8, u8, u8) {
        let i = js_sys::Math::floor(h);
        let f = h - i;
        let mut vv = js_sys::Math::floor(v);
        let mut ss = js_sys::Math::floor(s);
        vv = vv / 100.0;
        ss = ss / 100.0;
        hsv::hsv_to_rgb(h, ss, vv)
    }

    pub fn rgb2hex(r: u8, g: u8, b: u8)->String {
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }
 }

 #[test]
 fn it_works() {
     let rgb = YewColor::hsv2rgb(100.0, 100.0, 20.0);
     println!("fffffff");
 }
// #[cfg(test)]
// mod tests {

// }