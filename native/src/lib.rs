#[macro_use]
extern crate neon;
extern crate svgcleaner;
extern crate svgdom;

use neon::prelude::*;
use svgcleaner::{cleaner, ParseOptions, WriteOptions, CleaningOptions, StyleJoinMode};
use svgdom::{
    Indent,
    ListSeparator,
    AttributesOrder,
};


fn compile(mut cx: FunctionContext) -> JsResult<JsString> {
    let path = cx.argument::<JsString>(0)?;

    let parse_opt = ParseOptions::default();
    let write_opt = WriteOptions {
        indent: Indent::None,
        attributes_indent: Indent::None,
        use_single_quote: false,
        trim_hex_colors: true,
        write_hidden_attributes: false,
        remove_leading_zero: true,
        use_compact_path_notation: true,
        join_arc_to_flags: false,
        remove_duplicated_path_commands: true,
        use_implicit_lineto_commands: true,
        simplify_transform_matrices: true,
        list_separator: ListSeparator::Space,
        attributes_order: AttributesOrder::Alphabetical,
    };
    let cleaning_opt = CleaningOptions {
        remove_unused_defs: true,
        convert_shapes: true,
        remove_title: true,
        remove_desc: true,
        remove_metadata: true,
        remove_dupl_linear_gradients: true,
        remove_dupl_radial_gradients: true,
        remove_dupl_fe_gaussian_blur: true,
        ungroup_groups: true,
        ungroup_defs: true,
        group_by_style: true,
        merge_gradients: true,
        regroup_gradient_stops: true,
        remove_invalid_stops: true,
        remove_invisible_elements: true,
        resolve_use: true,

        remove_version: true,
        remove_unreferenced_ids: true,
        trim_ids: true,
        remove_text_attributes: true,
        remove_unused_coordinates: true,
        remove_default_attributes: true,
        remove_xmlns_xlink_attribute: true,
        remove_needless_attributes: true,
        remove_gradient_attributes: false,
        join_style_attributes: StyleJoinMode::Some,
        apply_transform_to_gradients: true,
        apply_transform_to_shapes: true,

        paths_to_relative: true,
        remove_unused_segments: true,
        convert_segments: true,
        apply_transform_to_paths: true,

        coordinates_precision: 6,
        properties_precision: 6,
        paths_coordinates_precision: 8,
        transforms_precision: 8,
    };


    let raw = match cleaner::load_file(&path.value()) {
        Ok(raw) => raw,
        Err(err) => return cx.throw_error(err.to_string()),
    };

    let input_size = raw.len();
    let mut buf = raw.into_bytes();
    let mut prev_size = 0;

    loop {
        // Parse it.
        //
        // 'buf' is either an input data or cleaned data in the multipass mode.
        //
        // We can't reuse cleaned doc, because 'join_style_attributes', if enabled, breaks it.
        let mut doc = match cleaner::parse_data(std::str::from_utf8(&buf).unwrap(), &parse_opt) {
            Ok(d) => d,
            Err(e) => {
                return cx.throw_error(e.to_string());
            }
        };

        // Clean document.
        match cleaner::clean_doc(&mut doc, &cleaning_opt, &write_opt) {
            Ok(_) => {}
            Err(e) => {
                break;
            }
        }
        // Clear buffer.
        //
        // We are reusing the same buffer for input and output data.
        buf.clear();

        // Write buffer.
        cleaner::write_buffer(&doc, &write_opt, &mut buf);

        // If size is unchanged - exit from the loop.
        if prev_size == buf.len() {
            break;
        }

        prev_size = buf.len();
    }

    if buf.len() > input_size {
        return cx.throw_error("Error: cleaned file is bigger than original.");
    }

    Ok(cx.string(std::str::from_utf8(&buf).unwrap()))
}

register_module!(mut cx, {
    cx.export_function("compile", compile)?;
    Ok(())
});
