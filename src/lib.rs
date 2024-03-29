//! Simple badge generator

use std::convert::TryFrom;

use ab_glyph::{point, Font, FontArc, Glyph, PxScale, ScaleFont};
use base64::display::Base64Display;
use once_cell::sync::Lazy;

const FONT_DATA: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/DejaVuSans.ttf"
));
const FONT_SIZE: f32 = 11.0;

pub struct BadgeOptions {
    /// Subject will be displayed on the left side of badge
    pub subject: String,
    /// Status will be displayed on the right side of badge
    pub status: String,
    /// HTML color of badge
    pub color: String,
}

impl Default for BadgeOptions {
    fn default() -> BadgeOptions {
        BadgeOptions {
            subject: "build".to_owned(),
            status: "passing".to_owned(),
            color: "#4c1".to_owned(),
        }
    }
}

pub struct Badge {
    options: BadgeOptions,
    font: FontArc,
    scale: PxScale,
}

impl Badge {
    /// Create a new badge.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the `status` or `subject` is empty.
    pub fn new(options: BadgeOptions) -> Result<Badge, String> {
        static FONT: Lazy<FontArc> =
            Lazy::new(|| FontArc::try_from_slice(FONT_DATA).expect("Failed to parse FONT_DATA"));

        let font = &*FONT;
        let scale = PxScale {
            x: FONT_SIZE,
            y: FONT_SIZE,
        };

        if options.status.is_empty() || options.subject.is_empty() {
            return Err(String::from("status and subject must not be empty"));
        }

        Ok(Badge {
            options,
            // This clone is cheap since Font is an Arc
            font: font.clone(),
            scale,
            // offset,
        })
    }

    #[must_use]
    pub fn to_svg_data_uri(&self) -> String {
        format!(
            "data:image/svg+xml;base64,{}",
            Base64Display::new(
                self.to_svg().as_bytes(),
                &base64::engine::general_purpose::STANDARD
            ),
        )
    }

    #[must_use]
    pub fn to_svg(&self) -> String {
        let left_width = self.calculate_width(&self.options.subject) + 6;
        let right_width = self.calculate_width(&self.options.status) + 6;

        let svg = format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="{badge_width}" height="20">
              <linearGradient id="smooth" x2="0" y2="100%">
                <stop offset="0" stop-color="#bbb" stop-opacity=".1"/>
                <stop offset="1" stop-opacity=".1"/>
              </linearGradient>

              <clipPath id="round">
                <rect width="{badge_width}" height="20" rx="3" fill="#fff"/>
              </clipPath>

              <g clip-path="url(#round)">
                <rect width="{left_width}" height="20" fill="#555"/>
                <rect x="{left_width}" width="{right_width}" height="20" fill="{color}"/>
                <rect width="{badge_width}" height="20" fill="url(#smooth)"/>
              </g>

              <g fill="#fff" text-anchor="middle" font-family="DejaVu Sans,Verdana,Geneva,sans-serif" font-size="11">
                <text x="{subject_x}" y="15" fill="#010101" fill-opacity=".3">{subject}</text>
                <text x="{subject_x}" y="14">{subject}</text>
                <text x="{status_x}" y="15" fill="#010101" fill-opacity=".3">{status}</text>
                <text x="{status_x}" y="14">{status}</text>
              </g>
            </svg>"##,
            badge_width = left_width + right_width,
            left_width = left_width,
            right_width = right_width,
            color = self.options.color,
            subject_x = left_width / 2,
            subject = self.options.subject,
            status_x = left_width + (right_width / 2),
            status = self.options.status
        );

        svg
    }

    /// # Panics
    ///
    /// If `text` is longer than [`u32::MAX`]
    fn calculate_width(&self, text: &str) -> u32 {
        #[allow(clippy::cast_sign_loss, // layout is always positive
                clippy::cast_possible_truncation // TODO: this might break for texts that are too long
                )]
        let width = self.layout(text).ceil() as u32;
        width + ((u32::try_from(text.len()).expect("text to long") - 1) * 2) - 2
    }

    /// Simple single-line glyph layout.
    fn layout(&self, text: &str) -> f32 {
        let font = self.font.as_scaled(self.scale);

        let mut caret = point(0.0, font.ascent());
        let mut last_glyph: Option<Glyph> = None;
        let mut target = Vec::new();
        for c in text.chars() {
            if c.is_control() {
                continue;
            }
            let mut glyph = font.scaled_glyph(c);
            if let Some(previous) = last_glyph.take() {
                caret.x += font.kern(previous.id, glyph.id);
            }
            glyph.position = caret;

            last_glyph = Some(glyph.clone());
            caret.x += font.h_advance(glyph.id);

            target.push(glyph);
        }
        match (target.first(), target.last()) {
            (Some(first), Some(last)) => {
                font.glyph_bounds(last).max.x - font.glyph_bounds(first).min.x
            }
            _ => 0.,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn options() -> BadgeOptions {
        BadgeOptions::default()
    }

    #[test]
    fn test_new() {
        assert!(Badge::new(options()).is_ok());

        let mut bad_options_status = options();
        bad_options_status.status = String::new();
        assert!(Badge::new(bad_options_status).is_err());

        let mut bad_options_subject = options();
        bad_options_subject.subject = String::new();
        assert!(Badge::new(bad_options_subject).is_err());
    }

    #[test]
    fn test_calculate_width() {
        let badge = Badge::new(options()).unwrap();

        assert_eq!(badge.calculate_width("build"), 31);
        assert_eq!(badge.calculate_width("passing"), 48);
    }

    #[test]
    fn test_to_svg() {
        const TEST_BADGE: &str =
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test_badge.svg"));

        let options = BadgeOptions {
            subject: "docs".to_owned(),
            status: "0.5.3".to_owned(),
            color: "#4d76ae".to_owned(),
        };
        let badge = Badge::new(options).unwrap();

        // I don't like this any more than you do, but the alternative is making sure that
        // both svgs match, space for space and newline for newline
        assert_eq!(
            badge.to_svg().split_whitespace().collect::<String>(),
            TEST_BADGE.split_whitespace().collect::<String>()
        );
    }
}
