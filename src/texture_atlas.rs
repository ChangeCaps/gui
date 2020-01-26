use super::*;
use math::*;
use glium::Rect;

pub fn crate_atlas<D>(
    display: &glium::Display, 
    images: &mut Vec<glium::texture::RawImage2d<D>>, 
    dimensions: &Vec<Vec2<u32>>
) -> (glium::texture::Texture2d, Vec<Vec2<f32>>, Vec2<f32>)
    where D: Clone + glium::texture::PixelValue
{
    const STARTING_DIMENSIONS: Vec2<u32> = Vec2 { x: 256, y: 256 };
    let mut atlas_dimensions = STARTING_DIMENSIONS;

    // make the dimensions vec hold the indecies of their respective images
    let mut dimensions: Vec<(Vec2<u32>, usize)> = dimensions.iter().enumerate().map(|(i, x)| (*x, i)).collect();

    dimensions.sort_by(|a, b| b.0.x.min(b.0.y).partial_cmp(&a.0.x.min(a.0.y)).unwrap());

    // first calulation
    let mut image_rects = calculate_rects(atlas_dimensions, &dimensions);

    // if the first calculation failed, try again with a larger atlas size
    while image_rects.is_none() {
        atlas_dimensions *= 2;

        image_rects = calculate_rects(atlas_dimensions, &dimensions);
    }

    // we know that out atlas calculation went well, so unwrap is fine
    let mut image_rects = image_rects.expect("wut");

    image_rects.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    // create texture
    let texture = glium::texture::Texture2d::empty(display, atlas_dimensions.x, atlas_dimensions.y).unwrap();

    let atlas_dimensions = Vec2::new(atlas_dimensions.x as f32, atlas_dimensions.y as f32);
    let image_positions = image_rects.iter().map(|(rect, _)| {
        Vec2::new(
            rect.left as f32,
            rect.bottom as f32
        ) / atlas_dimensions
    }).collect();

    // draw the images onto the atlas
    for _ in 0..image_rects.len() {
        let (rect, _) = image_rects.pop().unwrap();

        texture.write(rect, images.pop().unwrap());
    }

    (texture, image_positions, atlas_dimensions)
}

fn calculate_rects(
    atlas_dimensions: Vec2<u32>,
    dimensions: &Vec<(Vec2<u32>, usize)>,
) -> Option<Vec<(Rect, usize)>> {
    let mut empty_spaces = vec![Rect { bottom: 0, left: 0, height: atlas_dimensions.y, width: atlas_dimensions.x}];

    let mut image_rects = Vec::new();

    for (dimensions, image_index) in dimensions {
        let space = empty_spaces.iter().enumerate().rev().find(|(index, space)| {
            dimensions.x <= space.width && dimensions.y <= space.height
        });

        if space == None {
            return None;
        }

        let (index, space) = space.unwrap();

        image_rects.push(
            (
                Rect {
                    left: space.left,
                    bottom: space.bottom,
                    width: dimensions.x,
                    height: dimensions.y,
                },
                *image_index
            )
        );

        let dw = space.width - dimensions.x;
        let dh = space.height - dimensions.y;

        if dw == 0 && dh == 0 {
            empty_spaces.swap_remove(index);
            
            continue;
        }

        if dw > 0 && dh == 0 {
            let mut r = space.clone();

            r.left += dimensions.x;
            r.width -= dimensions.x;

            empty_spaces.swap_remove(index);
            
            empty_spaces.push(r);

            continue;
        }

        if dw == 0 && dh > 0 {
            let mut r = space.clone();

            r.bottom += dimensions.y;
            r.height -= dimensions.y;

            empty_spaces.swap_remove(index);
            
            empty_spaces.push(r);

            continue;
        }

        let (bigger, smaller) = if dw > dh {
            (
                Rect {
                    left: space.left + dimensions.x,
                    bottom: space.bottom,
                    width: dw,
                    height: space.height,
                },
                Rect {
                    left: space.left,
                    bottom: space.bottom + dimensions.y,
                    width: dimensions.x,
                    height: dh,
                }
            )
        } else {
            (
                Rect {
                    left: space.left,
                    bottom: space.bottom + dimensions.y,
                    width: space.width,
                    height: dh,
                },
                Rect {
                    left: space.left + dimensions.x,
                    bottom: space.bottom,
                    width: dw,
                    height: dimensions.y,
                }
            )
        };

        empty_spaces.swap_remove(index);

        empty_spaces.push(bigger);
        empty_spaces.push(smaller);
    }

    return Some(image_rects);
}