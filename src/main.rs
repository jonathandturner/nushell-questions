#![allow(unused_imports)]
#![allow(unused_variables)]

use clap::{App, Arg};
use nu_cli::run_block;
use nu_cli::run_pipeline;
use nu_cli::Tag;
use nu_cli::{
    create_default_context, parse_and_eval, EvaluationContext, InputStream, OutputStream,
};
use nu_errors::ShellError;
use nu_parser::ParserScope;
use nu_protocol::hir::Block;

use futures::executor;

pub async fn parse_and_eval_line(
    line: &str,
    ctx: &EvaluationContext,
) -> Result<String, ShellError> {
    let block = parse_line(line, &ctx).await?;
    let result = eval_block(block, &ctx).await;
    result
}

pub async fn parse_line(line: &str, ctx: &EvaluationContext) -> Result<Block, ShellError> {
    // FIXME: do we still need this?
    let line = if let Some(s) = line.strip_suffix('\n') {
        s
    } else {
        line
    };

    // TODO ensure the command whose examples we're testing is actually in the pipeline
    ctx.scope.enter_scope();
    let (classified_block, err) = nu_parser::parse(&line, 0, &ctx.scope);
    if let Some(err) = err {
        ctx.scope.exit_scope();
        return Err(err.into());
    }
    Ok(classified_block)
}

pub async fn eval_block(
    classified_block: Block,
    ctx: &EvaluationContext,
) -> Result<String, ShellError> {
    let pipelines = &classified_block.block[0].pipelines;
    dbg!(pipelines);

    let input_stream = InputStream::empty();
    let env = ctx.get_env();
    ctx.scope.add_env(env);

    // let result = run_pipeline(&pipelines[0], ctx, input_stream).await;
    let result = run_block(&classified_block, ctx, input_stream).await;

    ctx.scope.exit_scope();

    dbg!("Collecting");
    let out = result?;
    dbg!("Result OK");
    let out2 = out.collect_string(Tag::unknown()).await.map(|x| x.item);
    dbg!("Finished collecting");

    out2
}

fn main() {
    let matches = App::new("nushell test")
        .arg(Arg::with_name("line").long("line").takes_value(true))
        .get_matches();

    let line = matches.value_of("line").unwrap();
    let context = create_default_context(false).unwrap();

    // let line = "= 1 + 3";
    // let line = "= 1 + 3 | autoview";
    // let line = "ls | sort-by size";
    // let line = "echo hi | cat | autoview";

    let fut = parse_and_eval_line(&line, &context);

    dbg!("Executing");

    let stream = executor::block_on(fut).unwrap();
    dbg!(stream);
}
