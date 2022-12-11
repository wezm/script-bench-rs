use std::cmp::Ordering;

use boa_engine::{
    class::{Class, ClassBuilder},
    Context, JsResult, JsValue,
};
use boa_engine::syntax::Parser;
use boa_gc::{Finalize, Trace};
use criterion::{criterion_group, criterion_main, Criterion};
use gc::Gc;

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Debug, Trace, Finalize)]
struct RustData(String);

impl RustData {
    pub fn new(s: String) -> Self {
        RustData(s)
    }

    /// Says hello if `this` is a `Person`
    fn js_cmp(this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let other = match args
            .get(0)
            .and_then(|arg| arg.as_object())
            .and_then(|obj| obj.downcast_ref::<RustData>())
        {
            Some(data) => data,
            None => context.throw_type_error("'other' is not a RustData object")?,
        };

        if let Some(object) = this.as_object() {
            if let Some(data) = object.downcast_ref::<RustData>() {
                let res = match data.cmp(&other) {
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                };

                return Ok(JsValue::from(res));
            }
        }
        context.throw_type_error("'this' is not a RustData object")
    }
}

const PROG: &'static str = include_str!("sort_userdata.js");

impl Class for RustData {
    const NAME: &'static str = "RustData";
    // We set the length to `1` since we accept 1 arguments in the constructor.
    const LENGTH: usize = 1;

    // This is what is called when we construct a `RustData` with the expression `new RustData()`.
    fn constructor(_this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<Self> {
        // We get the first argument. If it is unavailable we default to `undefined`,
        // and then we call `to_string()`.
        //
        // This is equivalent to `String(arg)`.
        let s = args
            .get(0)
            .cloned()
            .unwrap_or_default()
            .to_string(context)?;

        // We construct a new native struct `RustData`
        Ok(RustData::new(s.to_string()))
    }

    /// Here is where the class is initialized.
    fn init(class: &mut ClassBuilder) -> JsResult<()> {
        class.method("cmp", 1, Self::js_cmp);

        Ok(())
    }
}

fn benchmark(c: &mut Criterion) {
    let mut context = Context::default();
    context.register_global_class::<RustData>().unwrap();

    let statement_list = Parser::new(PROG.as_bytes())
        .parse_all(&mut context)
        .unwrap();

    let code_block = context.compile(&statement_list).unwrap();

    c.bench_function("Sort userdata", |b| {
        b.iter(|| {
            // This clone is cheap
            let code_block = Gc::clone(&code_block);
            context.execute(code_block).unwrap();
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = benchmark,
}

criterion_main!(benches);
