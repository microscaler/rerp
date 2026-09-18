// Implementation crate main entry point
// Generated starting point — safe to customize; not auto-regenerated on stub regen.

mod controllers;
mod impl_registry;

use brrtrouter::server::{RunAppArgs, RunAppBuilder};
use clap::Parser;
use rerp_documents_render_gen::registry as gen_registry;
use std::io;
use std::path::PathBuf;

#[cfg(feature = "jemalloc")]
use tikv_jemallocator::Jemalloc;

#[cfg(feature = "jemalloc")]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "./doc/openapi.yaml")]
    spec: PathBuf,
    #[arg(long)]
    static_dir: Option<PathBuf>,
    #[arg(long, default_value = "./doc")]
    doc_dir: PathBuf,
    #[arg(long, default_value_t = false)]
    hot_reload: bool,
    #[arg(long)]
    test_api_key: Option<String>,
    #[arg(long, default_value = "./config/config.yaml")]
    config: PathBuf,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    RunAppBuilder::new()
        .args(RunAppArgs {
            spec: args.spec,
            config: args.config,
            doc_dir: args.doc_dir,
            static_dir: args.static_dir,
            hot_reload: args.hot_reload,
            test_api_key: args.test_api_key,
            manifest_dir,
            default_port: 8080,
            service_name: "rerp_documents_render".into(),
        })
        .register(|dispatcher, routes| unsafe {
            gen_registry::register_from_spec(dispatcher, routes);
            impl_registry::register_impl(dispatcher, routes);
        })
        .run()
}
