// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
use anyhow::{bail, Context};
use clap::{Parser, Subcommand};
use std::{
    ffi::OsString,
    fs, io,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

#[derive(Debug, Parser)]
struct Cli {
    /// Operation
    #[command(subcommand)]
    operation: Operation,
}

#[derive(Debug, Subcommand)]
enum Operation {
    Encode {
        /// SMT-LIB file to encode.
        #[arg(long)]
        instance: PathBuf,
        /// Output file for the CNF encoding.
        #[arg(long)]
        cnf_outfile: PathBuf,
    },
    Solve {
        /// SMT-LIB file to solve.
        #[arg(long)]
        instance: PathBuf,
        /// SAT solver to use.
        #[arg(long, default_value = "kissat")]
        solver: OsString,
    },
}

fn main() -> anyhow::Result<()> {
    let config = Cli::parse();

    let reader = |path: &Path| -> anyhow::Result<_> {
        let file = fs::File::open(path)
            .with_context(|| format!("Unable to open file: {}", path.display()))?;
        Ok(io::BufReader::new(file))
    };

    match config.operation {
        Operation::Encode {
            instance,
            cnf_outfile,
        } => {
            println!("Encoding {}", instance.display());
            let problem = reader(instance.as_ref())?;
            // Spawn minisat to simplify the generated CNF. The simplified formula it generates
            // cannot be mapped back to the original variables (as far as I can tell), so models
            // for the simplified version cannot be translated back. If we need models, cadical
            // supports writing simplified output and seems to preserve the original variables.
            // It can be run as `cadical -o <OUTFILE>`.
            let mut minisat = Command::new("minisat")
                .arg("-verb=0")
                .arg(format!("-dimacs={}", cnf_outfile.to_string_lossy()))
                .stdin(Stdio::piped())
                .spawn()
                .context("Unable to spawn `minisat` for CNF simplification")?;
            amzn_smt_eager_arithmetic::encode(problem, minisat.stdin.as_mut().unwrap())?;
            let exit = minisat.wait()?;
            if exit.success() {
                Ok(())
            } else {
                bail!("minisat exited with non-success status {}", exit)
            }
        }
        Operation::Solve { instance, solver } => {
            println!("Solving {}", instance.display());
            let problem = reader(instance.as_ref())?;
            match amzn_smt_eager_arithmetic::solve(problem, solver)? {
                Some(model) => println!("sat\n{}", model),
                None => println!("unsat"),
            }
            Ok(())
        }
    }
}
