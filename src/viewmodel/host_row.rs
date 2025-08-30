use serde::Serialize;

use crate::model::host_row::HostRowModel;

#[derive(Debug, Serialize)]
pub(crate) struct HostRowView {
    host: String,
    host_url: String,
    loc: String,
    system: String,
    rev: String,
    rev_url: String,
    ref_: String,
    bg_color: String,
    fg_color: String,
}

impl From<HostRowModel> for HostRowView {
    fn from(
        HostRowModel {
            host,
            host_url,
            loc,
            system,
            rev,
            rev_url,
            ref_,
        }: HostRowModel,
    ) -> Self {
        let (bg_color, fg_color) = HostRowView::compute_colors(&rev);
        HostRowView {
            host,
            host_url,
            loc,
            system,
            rev,
            rev_url,
            ref_,
            bg_color,
            fg_color,
        }
    }
}

impl HostRowView {
    fn compute_colors(rev: &str) -> (String, String) {
        let last_six_chars: String = rev.chars().rev().take(6).collect::<String>();
        let num = u32::from_str_radix(&last_six_chars, 16).unwrap_or(0);
		let first_two: u32 = (num >> 16) & 0xff;
		let middle_two: u32 = (num >> 8) & 0xff;
		let last_two: u32 = num & 0xff;

		let avg = (first_two + middle_two + last_two) / 3;

        let fg = if avg > 148 {
            "000000"
        } else {
            "ffffff"
        };
        (last_six_chars, fg.to_string())
    }
}
