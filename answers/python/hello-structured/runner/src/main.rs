use {
    anyhow::{anyhow, Result},
    async_trait::async_trait,
    clap::Parser,
    example::demo::greeter_candidates::{self, Candidate, Person},
    tokio::fs,
    wasmtime::{
        component::{Component, Linker},
        Config, Engine, Store,
    },
    wasmtime_wasi::{
        preview2::{command, DirPerms, FilePerms, Table, WasiCtx, WasiCtxBuilder, WasiView},
        Dir,
    },
};

fn parse_mapdir(s: &str) -> Result<(String, String)> {
    if let Some((guest_dir, host_dir)) = s.split_once("::") {
        Ok((guest_dir.into(), host_dir.into()))
    } else {
        Err(anyhow!(
            "expected string of form GUEST_DIR::HOST_DIR; got {s}"
        ))
    }
}

fn parse_env(s: &str) -> Result<(String, String)> {
    if let Some((name, value)) = s.split_once('=') {
        Ok((name.into(), value.into()))
    } else {
        Err(anyhow!("expected string of form NAME=VALUE; got {s}"))
    }
}

#[derive(Parser)]
pub struct Options {
    component: String,

    #[clap(long, value_name = "GUEST_DIR::HOST_DIR", value_parser = parse_mapdir)]
    mapdir: Vec<(String, String)>,

    #[clap(long, value_name = "NAME=VALUE", value_parser = parse_env)]
    env: Vec<(String, String)>,
}

wasmtime::component::bindgen!({
    path: "../wit",
    world: "demo",
    async: true
});

struct Host;

#[async_trait]
impl greeter_candidates::Host for Host {
    async fn get(&mut self) -> Result<Vec<Candidate>> {
        Ok(vec![
            Candidate::Hermit(Person {
                name: "Tom".into(),
                lego_count: Some(42),
            }),
            Candidate::Excited(Person {
                name: "Sally".into(),
                lego_count: None,
            }),
            Candidate::Excited(Person {
                name: "Reggie".into(),
                lego_count: Some(0),
            }),
            Candidate::Excited(Person {
                name: "Jules".into(),
                lego_count: Some(592),
            }),
        ])
    }
}

struct Ctx {
    wasi: WasiCtx,
    table: Table,
    host: Host,
}

impl WasiView for Ctx {
    fn ctx(&self) -> &WasiCtx {
        &self.wasi
    }
    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
    fn table(&self) -> &Table {
        &self.table
    }
    fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let options = Options::parse();

    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let engine = Engine::new(&config)?;

    let mut linker = Linker::<Ctx>::new(&engine);
    command::add_to_linker(&mut linker)?;
    Demo::add_to_linker(&mut linker, |ctx| &mut ctx.host)?;

    let mut table = Table::new();
    let mut wasi = WasiCtxBuilder::new();
    wasi.inherit_stdio();

    for (guest_dir, host_dir) in options.mapdir {
        wasi.preopened_dir(
            Dir::from_std_file(std::fs::File::open(host_dir)?),
            DirPerms::all(),
            FilePerms::all(),
            guest_dir,
        );
    }

    for (name, value) in options.env {
        wasi.env(name, value);
    }

    let wasi = wasi.build(&mut table)?;
    let mut store = Store::new(
        &engine,
        Ctx {
            wasi,
            table,
            host: Host,
        },
    );

    let instance = linker
        .instantiate_async(
            &mut store,
            &Component::new(&engine, &fs::read(&options.component).await?)?,
        )
        .await?;

    println!(
        "result: {:?}",
        Demo::new(&mut store, &instance)?
            .example_demo_greeter()
            .call_greet(&mut store)
            .await?
            .map_err(|s| anyhow!("greet function returned an error: {s}"))?
    );

    Ok(())
}
