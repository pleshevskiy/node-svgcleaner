use serde::{Deserialize, Serialize};

pub use svgcleaner::{ParseOptions, WriteOptions, CleaningOptions, StyleJoinMode};
use svgdom::{
    Indent,
    ListSeparator,
    AttributesOrder,
};



#[derive(Deserialize, Serialize)]
pub enum IndentFork {
    None,
    Spaces(u8),
    Tabs,
}

impl From<IndentFork> for Indent {
    fn from(fork: IndentFork) -> Self {
        match fork {
            IndentFork::None => Indent::None,
            IndentFork::Spaces(num) => Indent::Spaces(num),
            IndentFork::Tabs => Indent::Tabs,
        }
    }
}


#[derive(Deserialize, Serialize)]
pub enum ListSeparatorFork {
    Comma,
    Space,
    CommaSpace,
}

impl From<ListSeparatorFork> for ListSeparator {
    fn from(fork: ListSeparatorFork) -> Self {
        match fork {
            ListSeparatorFork::Comma => ListSeparator::Comma,
            ListSeparatorFork::Space => ListSeparator::Space,
            ListSeparatorFork::CommaSpace => ListSeparator::CommaSpace,
        }
    }
}


#[derive(Deserialize, Serialize)]
pub enum AttributesOrderFork {
    AsIs,
    Alphabetical,
    Specification,
}

impl From<AttributesOrderFork> for AttributesOrder {
    fn from(fork: AttributesOrderFork) -> Self {
        match fork {
            AttributesOrderFork::AsIs => AttributesOrder::AsIs,
            AttributesOrderFork::Alphabetical => AttributesOrder::Alphabetical,
            AttributesOrderFork::Specification => AttributesOrder::Specification,
        }
    }
}


#[derive(Deserialize, Serialize)]
pub enum StyleJoinModeFork {
    None,
    Some,
    All,
}

impl From<StyleJoinModeFork> for StyleJoinMode {
    fn from(fork: StyleJoinModeFork) -> Self {
        match fork {
            StyleJoinModeFork::None => StyleJoinMode::None,
            StyleJoinModeFork::Some => StyleJoinMode::Some,
            StyleJoinModeFork::All => StyleJoinMode::All,
        }
    }
}


pub struct NodeSvgCleanerFlags {
    pub multipass: bool,
    pub allow_bigger_file: bool,
    pub append_newline: bool,
}


#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields, default, rename_all = "camelCase")]
pub struct SvgCleanerOptions {
    parse_comments: bool,
    parse_declarations: bool,
    parse_unknown_elements: bool,
    parse_unknown_attributes: bool,
    parse_px_unit: bool,
    skip_unresolved_classes: bool,
    skip_invalid_attributes: bool,
    skip_invalid_css: bool,
    skip_paint_fallback: bool,

    indent: IndentFork,
    attributes_indent: IndentFork,
    use_single_quote: bool,
    trim_hex_colors: bool,
    write_hidden_attributes: bool,
    remove_leading_zero: bool,
    use_compact_path_notation: bool,
    join_arc_to_flags: bool,
    remove_duplicated_path_commands: bool,
    use_implicit_lineto_commands: bool,
    simplify_transform_matrices: bool,
    list_separator: ListSeparatorFork,
    attributes_order: AttributesOrderFork,

    remove_unused_defs: bool,
    convert_shapes: bool,
    remove_title: bool,
    remove_desc: bool,
    remove_metadata: bool,
    remove_dupl_linear_gradients: bool,
    remove_dupl_radial_gradients: bool,
    remove_dupl_fe_gaussian_blur: bool,
    ungroup_groups: bool,
    ungroup_defs: bool,
    group_by_style: bool,
    merge_gradients: bool,
    regroup_gradient_stops: bool,
    remove_invalid_stops: bool,
    remove_invisible_elements: bool,
    resolve_use: bool,
    remove_version: bool,
    remove_unreferenced_ids: bool,
    trim_ids: bool,
    remove_text_attributes: bool,
    remove_unused_coordinates: bool,
    remove_default_attributes: bool,
    remove_xmlns_xlink_attribute: bool,
    remove_needless_attributes: bool,
    remove_gradient_attributes: bool,
    join_style_attributes: StyleJoinModeFork,
    apply_transform_to_gradients: bool,
    apply_transform_to_shapes: bool,
    paths_to_relative: bool,
    remove_unused_segments: bool,
    convert_segments: bool,
    apply_transform_to_paths: bool,
    coordinates_precision: u8,
    properties_precision: u8,
    paths_coordinates_precision: u8,
    transforms_precision: u8,

    multipass: bool,
    allow_bigger_file: bool,
    append_newline: bool,
}

impl Default for SvgCleanerOptions {
    fn default() -> Self {
        SvgCleanerOptions {
            parse_comments: true,
            parse_declarations: true,
            parse_unknown_elements: true,
            parse_unknown_attributes: true,
            parse_px_unit: true,
            skip_unresolved_classes: true,
            skip_invalid_attributes: false,
            skip_invalid_css: false,
            skip_paint_fallback: false,

            indent: IndentFork::None,
            attributes_indent: IndentFork::None,
            use_single_quote: false,
            trim_hex_colors: true,
            write_hidden_attributes: true,
            remove_leading_zero: true,
            use_compact_path_notation: true,
            join_arc_to_flags: false,
            remove_duplicated_path_commands: true,
            use_implicit_lineto_commands: true,
            simplify_transform_matrices: true,
            list_separator: ListSeparatorFork::Space,
            attributes_order: AttributesOrderFork::AsIs,

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
            join_style_attributes: StyleJoinModeFork::None,
            apply_transform_to_gradients: true,
            apply_transform_to_shapes: true,
            paths_to_relative: true,
            remove_unused_segments: true,
            convert_segments: true,
            apply_transform_to_paths: false,
            coordinates_precision: 6,
            properties_precision: 6,
            paths_coordinates_precision: 8,
            transforms_precision: 8,

            multipass: false,
            allow_bigger_file: false,
            append_newline: false,
        }
    }
}

impl SvgCleanerOptions {
    pub fn split(self) -> (ParseOptions, WriteOptions, CleaningOptions, NodeSvgCleanerFlags) {
        (
            ParseOptions {
                parse_comments: self.parse_comments,
                parse_declarations: self.parse_declarations,
                parse_unknown_elements: self.parse_unknown_elements,
                parse_unknown_attributes: self.parse_unknown_attributes,
                parse_px_unit: self.parse_px_unit,
                skip_unresolved_classes: self.skip_unresolved_classes,
                skip_invalid_attributes: self.skip_invalid_attributes,
                skip_invalid_css: self.skip_invalid_css,
                skip_paint_fallback: self.skip_paint_fallback,
            },
            WriteOptions {
                indent: self.indent.into(),
                attributes_indent: self.attributes_indent.into(),
                use_single_quote: self.use_single_quote,
                trim_hex_colors: self.trim_hex_colors,
                write_hidden_attributes: self.write_hidden_attributes,
                remove_leading_zero: self.remove_leading_zero,
                use_compact_path_notation: self.use_compact_path_notation,
                join_arc_to_flags: self.join_arc_to_flags,
                remove_duplicated_path_commands: self.remove_duplicated_path_commands,
                use_implicit_lineto_commands: self.use_implicit_lineto_commands,
                simplify_transform_matrices: self.simplify_transform_matrices,
                list_separator: self.list_separator.into(),
                attributes_order: self.attributes_order.into(),
            },
            CleaningOptions {
                remove_unused_defs: self.remove_unused_defs,
                convert_shapes: self.convert_shapes,
                remove_title: self.remove_title,
                remove_desc: self.remove_desc,
                remove_metadata: self.remove_metadata,
                remove_dupl_linear_gradients: self.remove_dupl_linear_gradients,
                remove_dupl_radial_gradients: self.remove_dupl_radial_gradients,
                remove_dupl_fe_gaussian_blur: self.remove_dupl_fe_gaussian_blur,
                ungroup_groups: self.ungroup_groups,
                ungroup_defs: self.ungroup_defs,
                group_by_style: self.group_by_style,
                merge_gradients: self.merge_gradients,
                regroup_gradient_stops: self.regroup_gradient_stops,
                remove_invalid_stops: self.remove_invalid_stops,
                remove_invisible_elements: self.remove_invisible_elements,
                resolve_use: self.resolve_use,
                remove_version: self.remove_version,
                remove_unreferenced_ids: self.remove_unreferenced_ids,
                trim_ids: self.trim_ids,
                remove_text_attributes: self.remove_text_attributes,
                remove_unused_coordinates: self.remove_unused_coordinates,
                remove_default_attributes: self.remove_default_attributes,
                remove_xmlns_xlink_attribute: self.remove_xmlns_xlink_attribute,
                remove_needless_attributes: self.remove_needless_attributes,
                remove_gradient_attributes: self.remove_gradient_attributes,
                join_style_attributes: self.join_style_attributes.into(),
                apply_transform_to_gradients: self.apply_transform_to_gradients,
                apply_transform_to_shapes: self.apply_transform_to_shapes,
                paths_to_relative: self.paths_to_relative,
                remove_unused_segments: self.remove_unused_segments,
                convert_segments: self.convert_segments,
                apply_transform_to_paths: self.apply_transform_to_paths,
                coordinates_precision: self.coordinates_precision,
                properties_precision: self.properties_precision,
                paths_coordinates_precision: self.paths_coordinates_precision,
                transforms_precision: self.transforms_precision,
            },
            NodeSvgCleanerFlags {
                multipass: self.multipass,
                allow_bigger_file: self.allow_bigger_file,
                append_newline: self.append_newline,
            }
        )
    }
}

