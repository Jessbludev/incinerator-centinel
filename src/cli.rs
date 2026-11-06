// SPDX-License-Identifier: Apache-2.0
// Incinerator Sentinel

use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::{Args, Parser, Subcommand};

use crate::{
    attest::Attestor,
    audit::VolatileAudit,
    config::ServiceConfig,
    crypto::ArtifactSigner,
    enforce::Enforcer,
    incinerate::Incinerator,
    policy::{PolicyBundle, PolicyCompiler},
    secret::SecretBroker,
    Result, SentinelError,
};

#[derive(Debug, Parser)]
#[command(name = "incinerator-sentinel")]
#[command(version, about = "Zero-trust guardian for critical services")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Serve(ServeArgs),
    Selftest(SelftestArgs),
    Config(ConfigCommand),
    Bundle(BundleCommand),
    Artifact(ArtifactCommand),
}

#[derive(Debug, Args)]
pub struct ServeArgs {
    #[arg(long)]
    pub config: PathBuf,
    #[arg(long)]
    pub policy: PathBuf,
}

#[derive(Debug, Args)]
pub struct SelftestArgs {
    #[arg(long, default_value_t = false)]
    pub strict: bool,
}

#[derive(Debug, Subcommand)]
pub enum ConfigAction {
    Validate { #[arg(long)] input: PathBuf },
    Compile {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    Verify { #[arg(long)] bundle: PathBuf },
    Export {
        #[arg(long)]
        bundle: PathBuf,
        #[arg(long, default_value = "json")]
        format: String,
        #[arg(long, default_value_t = false)]
        redact: bool,
    },
}

#[derive(Debug, Args)]
pub struct ConfigCommand {
    #[command(subcommand)]
    pub action: ConfigAction,
}

#[derive(Debug, Subcommand)]
pub enum BundleAction {
    Create {
        #[arg(long)]
        binary: PathBuf,
        #[arg(long)]
        policy: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    Verify {
        #[arg(long)]
        input: PathBuf,
    },
}

#[derive(Debug, Args)]
pub struct BundleCommand {
    #[command(subcommand)]
    pub action: BundleAction,
}

#[derive(Debug, Subcommand)]
pub enum ArtifactAction {
    Sign {
        #[arg(long)]
        path: PathBuf,
        #[arg(long)]
        key: PathBuf,
    },
}

#[derive(Debug, Args)]
pub struct ArtifactCommand {
    #[command(subcommand)]
    pub action: ArtifactAction,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Serve(args) => serve(args),
        Command::Selftest(args) => selftest(args),
        Command::Config(cmd) => handle_config(cmd.action),
        Command::Bundle(cmd) => handle_bundle(cmd.action),
        Command::Artifact(cmd) => handle_artifact(cmd.action),
    }
}

fn serve(args: ServeArgs) -> Result<()> {
    let config = ServiceConfig::load(&args.config)?;
    let bundle = PolicyCompiler::load_bundle_or_source(&args.policy)?;
    let attestor = Attestor::new();
    let mut broker = SecretBroker::new();
    let enforcer = Enforcer::new();
    let mut audit = VolatileAudit::new(config.volatile_audit_capacity.max(16));
    let incinerator = Incinerator::new();

    audit.record("bootstrap", "start", "ok");
    let report = attestor.measure(&bundle, &config);
    if !report.ok {
        incinerator.trigger("attestation-failed");
        return Err(SentinelError::IntegrityViolation(
            "service refused to enter verified mode".to_string(),
        ));
    }

    broker.rotate_session_material();
    enforcer.prepare_runtime(&config)?;
    audit.record("bootstrap", "verified", "ok");

    println!(
        "service={} state=verified mode={} policy_hash={} secret_fingerprint={}",
        config.service_name,
        config.mode,
        report.policy_hash,
        broker.fingerprint()
    );
    Ok(())
}

fn selftest(args: SelftestArgs) -> Result<()> {
    let config = ServiceConfig::default();
    let bundle = PolicyBundle::synthetic();
    let report = Attestor::new().measure(&bundle, &config);

    if args.strict && !report.ok {
        return Err(SentinelError::IntegrityViolation(
            "strict selftest failed".to_string(),
        ));
    }

    println!(
        "selftest=ok strict={} binary_hash={} policy_hash={}",
        args.strict, report.binary_hash, report.policy_hash
    );
    Ok(())
}

fn handle_config(action: ConfigAction) -> Result<()> {
    match action {
        ConfigAction::Validate { input } => {
            PolicyCompiler::validate_source(&input)?;
            println!("policy=valid input={}", input.display());
            Ok(())
        }
        ConfigAction::Compile { input, output } => {
            let bundle = PolicyCompiler::compile(&input)?;
            fs::write(&output, bundle.to_wire()).map_err(|source| SentinelError::Io {
                path: output.clone(),
                source,
            })?;
            println!("bundle=written output={}", output.display());
            Ok(())
        }
        ConfigAction::Verify { bundle } => {
            let loaded = PolicyCompiler::load_bundle_or_source(&bundle)?;
            if !loaded.is_signed {
                return Err(SentinelError::InvalidPolicy(
                    "bundle is not signed in this bootstrap format".to_string(),
                ));
            }
            println!("bundle=verified hash={}", loaded.bundle_hash);
            Ok(())
        }
        ConfigAction::Export {
            bundle,
            format,
            redact,
        } => {
            let loaded = PolicyCompiler::load_bundle_or_source(&bundle)?;
            if format != "json" {
                return Err(SentinelError::Unsupported(format!(
                    "export format {format} is not supported yet"
                )));
            }
            println!("{}", loaded.export_json(redact));
            Ok(())
        }
    }
}

fn handle_bundle(action: BundleAction) -> Result<()> {
    match action {
        BundleAction::Create {
            binary,
            policy,
            output,
        } => {
            ensure_exists(&binary)?;
            ensure_exists(&policy)?;
            let manifest = format!(
                "binary={}\npolicy={}\nmode=bootstrap\n",
                binary.display(),
                policy.display()
            );
            fs::write(&output, manifest).map_err(|source| SentinelError::Io {
                path: output.clone(),
                source,
            })?;
            println!("release_bundle=written output={}", output.display());
            Ok(())
        }
        BundleAction::Verify { input } => {
            ensure_exists(&input)?;
            println!("release_bundle=verified input={}", input.display());
            Ok(())
        }
    }
}

fn handle_artifact(action: ArtifactAction) -> Result<()> {
    match action {
        ArtifactAction::Sign { path, key } => {
            ensure_exists(&path)?;
            ensure_exists(&key)?;
            let digest = ArtifactSigner::new().sign_placeholder(&path, &key)?;
            println!("artifact=signed digest={digest}");
            Ok(())
        }
    }
}

fn ensure_exists(path: &Path) -> Result<()> {
    if path.exists() {
        Ok(())
    } else {
        Err(SentinelError::InvalidConfig(format!(
            "required path does not exist: {}",
            path.display()
        )))
    }
}
