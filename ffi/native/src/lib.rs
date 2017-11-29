#[macro_use]
extern crate neon;
extern crate ratel;
#[macro_use]
extern crate serde;

#[macro_use]
extern crate serde_json;

use neon::vm::{Call, JsResult};
use neon::js::{JsString, JsBoolean};
use neon::js::error::{JsError, Kind};
use ratel::{parser, codegen, error, transformer};
use error::{Error, ParseError};
use ratel::{module, ast};
use ratel::astgen::*;

fn generate_ast(module: &module::Module) -> serde_json::Value {
    json!(Program {
        body: module.body()
    })
}

fn ast(call: Call) -> JsResult<JsString> {
    let scope = call.scope;

    if call.arguments.len() == 0 {
        return JsError::throw(Kind::TypeError, "First argument must be a string")
    }

    let source = call.arguments.require(scope, 0)?.check::<JsString>()?;
    let minify = call.arguments.require(scope, 1)?.check::<JsBoolean>()?;

    let module = match parser::parse(source.value().as_str()) {
        Err(errors) => {
            let str = format_errors(errors, source).join("\n");
            return JsError::throw(Kind::SyntaxError, &str)
        },
        Ok(module) => module,
    };

    let result = generate_ast(&module);
    let result = match minify.value () {
        true => serde_json::to_string_pretty(&result),
        false => serde_json::to_string(&result)
    };

    let out = result.unwrap().to_string();
    Ok(JsString::new(scope, &out).unwrap())
}

fn format_errors(errors: Vec<Error>, source: neon::mem::Handle<JsString>) -> Vec<String> {
    errors
    .into_iter()
    .map(|err| {
        match err {
            Error { start, end, .. } => {
               ParseError::UnexpectedToken { start, end, source: source.value() }
            }
        }
    })
    .map(|err| format!("{}", err))
    .collect()
}

fn transform(call: Call) -> JsResult<JsString> {
    let scope = call.scope;

    if call.arguments.len() == 0 {
        return JsError::throw(Kind::TypeError, "First argument must be a string")
    }

    let source = call.arguments.require(scope, 0)?.check::<JsString>()?;
    let minify = call.arguments.require(scope, 1)?.check::<JsBoolean>()?;

    let mut module = match parser::parse(source.value().as_str()) {
        Err(errors) => {
            let str = format_errors(errors, source).join("\n");
            return JsError::throw(Kind::SyntaxError, &str)
        },
        Ok(module) => module,
    };
    // transformer::transform(&mut module, transformer::Settings::target_es5());
    let out = codegen::codegen(&module, minify.value());

    Ok(JsString::new(scope, &out).unwrap())
}

fn parse(call: Call) -> JsResult<JsString> {
    let scope = call.scope;

    if call.arguments.len() == 0 {
        return JsError::throw(Kind::TypeError, "First argument must be a string")
    }

    let source = call.arguments.require(scope, 0)?.check::<JsString>()?;

    let module = match parser::parse(source.value().as_str()) {
        Err(errors) => {
            let str = format_errors(errors, source).join("\n");
            return JsError::throw(Kind::SyntaxError, &str)
        },
        Ok(module) => module,
    };

    let out = format!("{:?}", module.body());

    Ok(JsString::new(scope, &out).unwrap())
}

register_module!(m, {
    m.export("transform", transform)?;
    m.export("parse", parse)?;
    m.export("ast", ast)?;
    Ok(())
});
