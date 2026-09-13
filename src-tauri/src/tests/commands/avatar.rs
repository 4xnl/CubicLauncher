use super::*;

fn skin_png(width: u32, height: u32) -> Vec<u8> {
    let mut skin = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);
    let scale = width / 64;
    if scale > 0 && height >= 16 * scale {
        for y in 8 * scale..16 * scale {
            for x in 8 * scale..16 * scale {
                skin.put_pixel(x, y, Rgba([10, 20, 30, 255]));
            }
        }
        skin.put_pixel(40 * scale, 8 * scale, Rgba([40, 50, 60, 255]));
    }
    let mut png = Cursor::new(Vec::new());
    skin.write_to(&mut png, image::ImageFormat::Png).unwrap();
    png.into_inner()
}

#[test]
fn renders_legacy_and_modern_hd_skins() {
    for width in [64, 128, MAX_SKIN_DIMENSION] {
        for height in [width / 2, width] {
            let svg = skin_bytes_to_svg(&skin_png(width, height)).unwrap();
            assert!(svg.starts_with("<svg "));
            assert!(svg.contains("viewBox=\"0 0 128 128\""));
            let b64 = svg
                .split("data:image/png;base64,")
                .nth(1)
                .unwrap()
                .split('"')
                .next()
                .unwrap();
            let png = general_purpose::STANDARD.decode(b64).unwrap();
            let avatar = image::load_from_memory(&png).unwrap().to_rgba8();
            assert_eq!(avatar.dimensions(), (128, 128));
            assert_eq!(*avatar.get_pixel(0, 0), Rgba([40, 50, 60, 255]));
            assert_eq!(*avatar.get_pixel(127, 127), Rgba([10, 20, 30, 255]));
        }
    }
}

#[test]
fn rejects_short_and_invalid_skin_dimensions() {
    for (width, height) in [
        (64, 1),
        (64, 15),
        (128, 31),
        (64, 48),
        (64, 128),
        (32, 32),
        (65, 65),
    ] {
        assert_eq!(
            skin_bytes_to_svg(&skin_png(width, height)).unwrap_err(),
            "La imagen de skin tiene dimensiones inesperadas",
            "{width}x{height}"
        );
    }
}

#[test]
fn rejects_oversized_skin_dimensions() {
    for (width, height) in [(1088, 544), (1088, 1088), (64, 1025)] {
        assert!(
            skin_bytes_to_svg(&skin_png(width, height)).is_err(),
            "{width}x{height}"
        );
    }
}
