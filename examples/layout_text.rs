extern crate glyphydog;
extern crate cgmath_geometry;
extern crate cgmath;
extern crate png;

use glyphydog::{FTLib, Face, Shaper, FontSize, DPI, ShapedBuffer, RenderMode, GlyphMetricsPx};
use std::fs::File;
use std::io::Read;

use std::io::BufWriter;
use png::HasParameters;

use cgmath::{Vector2, EuclideanSpace};
use cgmath_geometry::{Rectangle, OffsetRect, DimsRect};

fn main() {
    let mut font_buf = vec![];
    File::open("./DejaVuSans.ttf").unwrap().read_to_end(&mut font_buf).unwrap();

    let lib = FTLib::new();
    let mut face = Face::new(font_buf, 0, &lib).unwrap();
    let mut shaper = Shaper::new();

    let mut output_image = vec![0; 128 * 128];

    let font_size = FontSize {
        width: 16*64,
        height: 16*64
    };
    let dpi = DPI {
        hori: 72,
        vert: 72
    };
    let mut buffer = ShapedBuffer::new();
    shaper.shape_text("Γειά σου Κόσμε!", &mut face, font_size, dpi, &mut buffer).unwrap();
    let mut cursor_x = 0;
    for i in 0..buffer.segments_len() {
        let segment = buffer.get_segment(i).unwrap();
        for glyph in segment.shaped_glyphs {
            let mut slot = face.load_glyph(glyph.glyph_index, font_size, dpi).unwrap();
            let bitmap = slot.render_glyph(RenderMode::Normal).unwrap();
            let metrics = GlyphMetricsPx::from(slot.metrics());
            println!("{:?}", metrics);

            blit(
                bitmap.buffer, bitmap.dims, bitmap.dims.into(),
                &mut output_image, DimsRect::new(128, 128),
                    (Vector2::new(cursor_x + metrics.hori_bearing.x, 32 - metrics.hori_bearing.y) + glyph.pos.to_vec()).cast().unwrap()
            );
        }
        cursor_x += segment.advance;
        println!("{:#?}", buffer.get_segment(i).unwrap());
    }

    let file = File::create("./layout_text.png").unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, 128, 128);
    encoder.set(png::ColorType::Grayscale).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&output_image).unwrap();
}

fn blit<P: Copy>(
    src: &[P], src_dims: DimsRect<u32>, src_copy_from: OffsetRect<u32>,
    dst: &mut [P], dst_dims: DimsRect<u32>, dst_offset: Vector2<u32>
) {
    for row_num in 0..src_copy_from.height() as usize {
        let dst_row_num = row_num + dst_offset.y as usize;
        let dst_slice_offset = dst_row_num * dst_dims.width() as usize;
        let dst_row = &mut dst[dst_slice_offset..dst_slice_offset + dst_dims.width() as usize];

        let src_row_num = row_num + src_copy_from.min().y as usize;
        let src_slice_offset = src_row_num * src_dims.width() as usize;
        let src_row = &src[src_slice_offset..src_slice_offset + src_dims.width() as usize];

        let src_copy_slice = &src_row[src_copy_from.min().x as usize..src_copy_from.max().x as usize];
        let dst_copy_to_slice = &mut dst_row[dst_offset.x as usize..(dst_offset.x + src_copy_from.width()) as usize];
        dst_copy_to_slice.copy_from_slice(src_copy_slice);
    }
}
