extern crate image;

use image::GenericImageView;

fn main() {
    let file = "/tmp/rgbavgclip.png" ;
    let img = image::open(file).unwrap();
    let size = img.dimensions();
    let w = size.0;
    let h = size.1;
    let mut red:   u32 = 0;
    let mut green: u32 = 0;
    let mut blue:  u32 = 0;

    for i in 0..w {
        for j in 0..h {
            let pixel = &img.get_pixel(i,j);
            red = red + pixel[0] as u32;
            green = green + pixel[1] as u32;
            blue  = blue + pixel[2] as u32;
        }

    }
    let n = h*w;
    red   = red/n;
    green = green/n;
    blue  = blue/n;
    let r: f64 = red   as f64/255.0;
    let g: f64 = green as f64/255.0;
    let b: f64 = blue  as f64/255.0;
    let v = f64::max(r,f64::max(g,b));
    let c = v - f64::min(r,f64::min(g,b));
    let s: f64;
    if v == 0.0 {
        s = 0.0;
    } else {
        s = c/v;
    };
    let h: f64;
    if c == 0.0 {
        h = std::f64::NAN;
    } else if v == r {
        h = 60.0*((g - b)/c).rem_euclid(6.0);
    } else if v == g {
        h = 60.0*((b - r)/c + 2.0);
    } else {
        h = 60.0*((r - g)/c + 4.0);
    }
    let x: f64;
    let y: f64;
    let h1: f64;
    let s1: f64;
    let l: f64;
    let h2: f64;
    let s2: f64;
    let h3: f64;
    let s3: f64;
    x = r - (g + b)/2.0;
    y = (3.0_f64).sqrt()*(g - b)/2.0;
    if c == 0.0 {
        h1 = std::f64::NAN;
    } else if y < 0.0 {
        h1 = 180.0*y.atan2(x)/std::f64::consts::PI + 360.0;
    } else {
        h1 = 180.0*y.atan2(x)/std::f64::consts::PI;
    }
    s1 = (x*x+y*y).sqrt();
    l = (r + g + b)/3.0;
    let yl: f64;
    let u1: f64;
    let v1: f64;
    yl = 0.299*r + 0.587*g + 0.114*b;
    //u1 = -0.14713*r - 0.28886*g + 0.436*b;
    //v1 = 0.615*r -0.51499*g -0.10001*b;
    u1 = -0.168736*r - 0.331264*g + 0.5*b;
    v1 = 0.5*r -0.418688*g -0.081312*b;
    let y2: f64;
    let u2: f64;
    let v2: f64;
    y2 = 0.2126*r + 0.7152*g + 0.0722*b;
    u2 = -0.09991*r - 0.33609*g + 0.436*b;
    v2 = 0.615*r -0.55861*g -0.05639*b;
    if c == 0.0 {
        h2 = std::f64::NAN;
    } else if v1 < 0.0 {
        h2 = 180.0*v1.atan2(u1)/std::f64::consts::PI + 360.0;
    } else {
        h2 = 180.0*v1.atan2(u1)/std::f64::consts::PI;
    }
    s2 = (u1*u1+v1*v1).sqrt();
    if c == 0.0 {
        h3 = std::f64::NAN;
    } else if v2 < 0.0 {
        h3 = 180.0*v2.atan2(u2)/std::f64::consts::PI + 360.0;
    } else {
        h3 = 180.0*v2.atan2(u2)/std::f64::consts::PI;
    }
    s3 = (u2*u2+v2*v2).sqrt();
    if h.is_nan() {
        println!("▯▮▯RGB:  [{}, {}, {}] | ⬡ HSV: [{:.0}, {:.0}%, {:.0}%] | ◯ HCL: [{:.0}, {:.0}%, {:.0}%] | BT.601: [{:.0},{:.0},{:.0}] | 601's HS: [{:.0},{:.0}%] | rec709: [{:.0},{:.0},{:.0}] | 709's HS: [{:.0},{:.0}%]",
            red,green,blue,h,100.0*s,100.0*v,h1,100.0*s1,100.0*l,255.0*yl,255.0*u1,255.0*v1,h2,100.0*s2,255.0*y2,255.0*u2,255.0*v2,h3,100.0*s3);
    } else {
        println!("▯▮▯RGB:  [{}, {}, {}] | ⬡ HSV: [{:.0}°, {:.0}%, {:.0}%] | ◯  HCL:[{:.0}°, {:.0}%, {:.0}%] | BT.601: [{:.0},{:.0},{:.0}] | 601's HS: [{:.0}°,{:.0}%] | rec709: [{:.0},{:.0},{:.0}] | 709's HS: [{:.0}°,{:.0}%]",
            red,green,blue,h,100.0*s,100.0*v,h1,100.0*s1,100.0*l,255.0*yl,255.0*u1,255.0*v1,h2,100.0*s2,255.0*y2,255.0*u2,255.0*v2,h3,100.0*s3);
    }
}
