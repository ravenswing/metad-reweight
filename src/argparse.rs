use clap::Parser;

#[derive(Parser, Default, Debug)]
#[clap(author = "Rhys Evans", version, about)]
/// MetaD-Reweight
struct Arguments {
    #[clap(short, long, required = true)]
    /// FES filenames prefix as generated with plumed sum_hills --stride. Expects FPREF%%d.dat
    fes_prefix: String,
    #[clap(short, long, required = true)]
    /// Filenames for the output, reweighted Free Energy Surface.
    out_fes: String,
    #[clap(short = 'y', long)]
    /// Biasfactor used in the well-tempered metadynamics.
    /// WARNING: if omitted assumes a non-well-tempered metadynamics.
    bias_factor: f64,
    #[clap(short, long)]
    ///  Filename containing original CVs, reweighting CVs and metadynamics bias.
    colvar: String,

    #[clap(default_value_t = 2.49, long)]
    /// kT in the energy units of the FES files
    kt: f64,

    // Legacy Options:
    #[clap(long)]
    /// Columns in the FES file with the Free Energy.
    fes_fe_col: i32,
    #[clap(long, value_delimiter = ' ', num_args = 1..)]
    /// Columns in the COLVAR file with the CVs to reweight to.
    colvar_rew_col: Vec<i32>,
    #[clap(long, value_delimiter = ' ', num_args = 1..)]
    /// Columns in the COLVAR file with the bias.
    colvar_bias_col: Vec<i32>,
    #[clap(long, group = "exp_beta_ct")]
    /// Use precalculated ebetac list, if omitted use FES files
    exp_bct_file: String,
    #[clap(long, group = "exp_beta_ct")]
    /// Use precalculated ebetac list, if omitted use FES files
    exp_bct_out: String,
    // MISSING ARGS: -min -max -bin
}

fn main() {
    let args = Arguments::parse();
    println!("{:?}", args);
}
