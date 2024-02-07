use std::{
	fs::read_to_string,
	io::{stdin, Read},
	path::PathBuf,
};

use clap::Parser;
use swc_common::{BytePos, Span};
use swc_ecma_parser::{
	lexer::Lexer, token::TokenAndSpan, StringInput, Syntax, TsConfig,
};

#[derive(Parser)]
struct Args {
	src: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
	let args = Args::parse();

	let src = if let Some(src) = args.src {
		read_to_string(src)?
	} else {
		let mut v = vec![];
		stdin().read_to_end(&mut v)?;
		String::from_utf8(v)?
	};

	let lexer = Lexer::new(
		Syntax::Typescript(TsConfig {
			tsx: true,
			decorators: true,
			..Default::default()
		}),
		Default::default(),
		StringInput::new(
			&src,
			BytePos(0),
			BytePos(src.len() as u32),
		),
		None,
	);

	for TokenAndSpan {
		span: Span { lo, hi, .. },
		token,
		..
	} in lexer
	{
		println!("{} {} #{:?}", lo.0, hi.0, token);
	}

	Ok(())
}
