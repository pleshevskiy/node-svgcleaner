extern crate svgcleaner;
extern crate svgdom;
extern crate neon;
extern crate neon_serde;
extern crate serde;

mod options;

use neon::prelude::*;
use svgcleaner::cleaner;
use options::SvgCleanerOptions;


fn read_file(mut cx: FunctionContext) -> JsResult<JsString> {
    let file_path = cx.argument::<JsString>(0)?.value();

    match cleaner::load_file(&file_path) {
        Ok(raw) => Ok(cx.string(raw)),
        Err(err) => cx.throw_error(err.to_string()),
    }
}


fn clean(mut cx: FunctionContext) -> JsResult<JsObject> {
    let svg_file_body = cx.argument::<JsString>(0)?.value();
    let options: SvgCleanerOptions = match cx.argument_opt(1) {
        Some(arg) => {
            let value = arg.downcast::<JsValue>().or_throw(&mut cx)?;
            neon_serde::from_value(&mut cx, value)?
        },
        None => SvgCleanerOptions::default(),
    };

    let (
        parse_opt,
        write_opt,
        cleaning_opt,
        flags
    ) = options.split();

    let input_size = svg_file_body.len();
    let mut buf = svg_file_body.into_bytes();
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
//                writeln!(stderr(), "{}.", e.full_chain()).unwrap();
//                on_err();
                return cx.throw_error(e.to_string())
            }
        };

        // Clean document.
        match cleaner::clean_doc(&mut doc, &cleaning_opt, &write_opt) {
            Ok(_) => {}
            Err(_) => {
//                writeln!(stderr(), "{}.", e.full_chain()).unwrap();
//                on_err();
                break;
            }
        }


        // Clear buffer.
        //
        // We are reusing the same buffer for input and output data.
        buf.clear();

        // Write buffer.
        cleaner::write_buffer(&doc, &write_opt, &mut buf);

        if !flags.multipass {
            // Do not repeat without '--multipass'.
            break;
        }

//         If size is unchanged - exit from the loop.
        if prev_size == buf.len() {
            break;
        }

        prev_size = buf.len();
    }

    if !flags.allow_bigger_file && buf.len() > input_size {
        return cx.throw_error("Cleaned file is bigger than original.");
    }

    // Optionally add a newline to the end of the file.
    // This is placed after the check for if the file is smaller. It's OK if the
    // file grows when adding a newline, since the user explicitly wanted that.
    if flags.append_newline {
        buf.push(b'\n');
    }

    let obj = JsObject::new(&mut cx);
    let content = std::str::from_utf8(&buf).unwrap();

    let js_content_length = cx.number(content.len() as f64);
    let js_content = cx.string(content);

    obj.set(&mut cx, "length", js_content_length)?;
    obj.set(&mut cx, "content", js_content)?;

    Ok(obj)
}

register_module!(mut cx, {
    cx.export_function("readFile", read_file)?;
    cx.export_function("clean", clean)?;
    Ok(())
});
