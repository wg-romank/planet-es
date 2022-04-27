use glsmrs::{Ctx, texture::{UploadedTexture, TextureSpec, ColorFormat}, UniformData};
use glsmrs::GL;
use image::GenericImageView;

pub fn tex_unis<'b>(name: &'static str, size_name: &'static str, tex: &'b mut Option<UploadedTexture>) -> Vec<(&'static str, UniformData<'b>)> {
  if let Some(t) = tex.as_mut() {
    vec![
      (size_name, UniformData::Vector2(t.sizef32())),
      (name, UniformData::Texture(t)),
    ]
  } else {
    vec![]
  }
}

pub fn to_png_texture(ctx: &Ctx, bytes: &[u8]) -> Result<UploadedTexture, String> {
  let texture = image::load_from_memory(bytes)
    .map_err(|e| format!("{:?}", e))?;

  let (width, height) = texture.dimensions();
  let tex = texture.into_rgb8();

  TextureSpec::new(ColorFormat(GL::RGB), [width, height])
    .upload_u8(&ctx, &tex.as_raw())
}
