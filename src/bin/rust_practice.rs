struct FileDirectory;
struct ColorRGB(u8, u8, u8);

struct SizeAndColor {
    size: u8,
    color: ColorRGB,
}

fn main() {
    // let color = ColorRGB(255, 0, 0);
    let size = 10;
    let asian_paint = SizeAndColor {
        size,
        color: ColorRGB(10, 20, 30),
    };
}
