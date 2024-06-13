//! This is a very simple implementation with no attempt to optimize memory usage

use std::io;

use bittwiddler_core::prelude::*;

#[derive(Default)]
struct StatePiecesHolder(Vec<(String, String)>);
impl HumanSinkForStatePieces for StatePiecesHolder {
    fn add_state_piece(&mut self, arg: &str, val: &str) {
        self.0.push((arg.to_owned(), val.to_owned()));
    }
}

fn format_sublevel_name(
    prefix: &str,
    sublevel_name: &str,
    sublevel_obj: &(impl HumanLevelThatHasState + ?Sized),
    write_dot: bool,
) -> String {
    let mut sublevel_full_name = prefix.to_string();
    sublevel_full_name.push_str(sublevel_name);

    let mut x = StatePiecesHolder::default();
    sublevel_obj._human_dump_my_state(&mut x);

    if x.0.len() > 0 {
        sublevel_full_name.push('[');
        for (i, xi) in x.0.iter().enumerate() {
            if i != 0 {
                sublevel_full_name.push_str(", ");
            }
            sublevel_full_name.push_str(&xi.0);
            sublevel_full_name.push('=');
            sublevel_full_name.push_str(&xi.1);
        }
        if write_dot {
            sublevel_full_name.push_str("].");
        } else {
            sublevel_full_name.push(']');
        }
    }

    sublevel_full_name
}

fn write_recurse<W: io::Write>(
    w: &mut W,
    bitstream: &impl BitArray,
    level: &dyn HumanLevelDynamicAccessor,
    prefix: &str,
) -> io::Result<()> {
    for (sublevel_idx, sublevel_name) in level._human_sublevels().iter().enumerate() {
        for sublevel_obj in level._human_construct_all_sublevels(sublevel_idx) {
            let sublevel_full_name =
                format_sublevel_name(prefix, sublevel_name, &*sublevel_obj, true);
            write_recurse(w, bitstream, &*sublevel_obj, &sublevel_full_name)?;
        }
    }

    for (field_idx, field_name) in level._human_fields().iter().enumerate() {
        for field_obj in level._human_construct_all_fields(field_idx) {
            let is_default = field_obj._human_is_at_default(bitstream);
            if is_default {
                continue;
            }
            let field_full_name = format_sublevel_name(prefix, field_name, &*field_obj, false);
            let value_str = field_obj._human_string_get(bitstream);
            write!(w, "{} = {}\n", field_full_name, value_str)?;
        }
    }

    Ok(())
}

pub fn write<B: BitArray + HumanLevelDynamicAccessor, W: io::Write>(
    mut w: W,
    bitstream: &B,
) -> io::Result<()> {
    write_recurse(&mut w, bitstream, bitstream, "")
}
