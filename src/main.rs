use log::{debug, info};
use sha1::Sha1;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use structopt::StructOpt;
use tera::{Context, Tera};

/// generating nesseccary configs and data to the given files
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Image URL
    #[structopt(short, long)]
    image: String,

    /// String from which we generate the hash
    #[structopt(short, long)]
    hash: String,

    /// pattern used to match against provided configs
    #[structopt(short, long)]
    pattern: String,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

const TMPLS: [(&str, &str); 4] = [
    ("dag_volumes.tmpl", r#""<CICD_DAG_VOLUMES_PLACEHOLDER>""#),
    (
        "dag_volume_mounts.tmpl",
        r#""<CICD_DAG_VOLUME_MOUNTS_PLACEHOLDER>""#,
    ),
    ("k8s_volumes.tmpl", "#<CICD_K8S_VOLUMES_PLACEHOLDER>"),
    (
        "k8s_volume_mounts.tmpl",
        "#<CICD_K8S_VOLUME_MOUNTS_PLACEHOLDER>",
    ),
];

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let opt = Opt::from_args();
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    env_logger::init();

    // image template
    info!("adding image to generated templates");
    let mut generated: HashMap<String, String> = HashMap::new();
    generated.insert(String::from("<CICD_IMAGE_PLACEHOLDER>"), opt.image);
    debug!("generated template so far is:{generated:#?}");

    // hash template
    info!("adding hash to generated templates");
    generated.insert(
        String::from("<CICD_HASH_PLACEHOLDER>"),
        generate_hash(opt.hash),
    );
    debug!("generated template so far is:{generated:#?}");

    // get allowed volumes from `rules`
    info!("calculating matching volumes to mount");
    let volumes = get_volume_names(&opt.pattern);
    debug!("volumes selected via given pattern:{volumes:#?}");

    // generate config template
    info!("generating config templates");
    insert_volume_configs(&mut generated, &volumes);
    debug!("generated template so far is:{generated:#?}");

    // update target files
    info!("updating target files");
    update_targets(generated, opt.files);
}

fn update_targets(generated: HashMap<String, String>, files: Vec<PathBuf>) {
    for path in files {
        let mut src = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("couldn't open file {path:?}"));
        for (k, v) in &generated {
            src = src.replace(&*k, v);
        }
        info!("updating file: [{path:?}]");
        std::fs::write(&path, src).unwrap_or_else(|_| panic!("failed to update file {path:?}"));
    }
}

fn insert_volume_configs(output: &mut HashMap<String, String>, volumes: &[String]) {
    let mut ctx = Context::new();
    ctx.insert("volumes", &volumes);
    ctx.insert("dagger_version", VERSION);
    let tmpl = Tera::new("tmpl/*.tmpl").expect("couldn't open tmpl folder");
    for (tmpl_name, key) in TMPLS {
        let val = tmpl
            .render(tmpl_name, &ctx)
            .unwrap_or_else(|_| panic!("couldn't render {tmpl_name}"));
        output.insert(key.into(), val);
    }
}

fn generate_hash(input: String) -> String {
    let mut hash = Sha1::new();
    hash.update(input.as_bytes());
    hash.digest().to_string()
}

fn get_volume_names(pattern: &str) -> Vec<String> {
    let mut v:Vec<String>=serde_json::from_str::<HashMap<String, String>>(
        &env::var("rules").expect("rules are not provided via env"),
    )
    .expect("invalid rules format")
    .into_iter()
    .filter(|(k, _)| pattern.starts_with(k))
    .map(|(_, v)| v)
    .collect();
    v.sort();
    v
    
}
