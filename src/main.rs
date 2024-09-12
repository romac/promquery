use core::{f32, fmt};

use clap::Parser;
use owo_colors::OwoColorize;
use pluralizer::pluralize;
use prometheus_parse::{Sample, Scrape, Value};
use reqwest::Url;
use textplots::Plot;

#[derive(Debug, Parser)]
pub struct Args {
    pub url: Url,
    pub metric: String,
}

type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), BoxError> {
    let Args { mut url, metric } = Args::parse();

    if url.path() != "/metrics" {
        url.set_path("/metrics");
    }

    let text = reqwest::get(url).await?.text().await?;

    let lines = text.lines().map(|l| Ok(l.to_string()));
    let mut scrape = prometheus_parse::Scrape::parse(lines)?;

    let doc = scrape
        .docs
        .get(&metric)
        .ok_or_else(|| format!("Metric not found: {metric}"))?;

    println!("{}", metric.bold().blue().reversed());
    println!("{}", doc.italic());

    scrape.samples.retain(|s| s.metric.starts_with(&metric));
    let exact = scrape
        .samples
        .iter()
        .filter(|s| s.metric == metric)
        .collect::<Vec<_>>();

    let count = exact.len();
    println!("{}\n", pluralize("sample", count as isize, true));

    for (i, sample) in exact.into_iter().enumerate() {
        show_sample(i + 1, sample, &scrape)?;
    }

    Ok(())
}

fn show_sample(i: usize, sample: &Sample, scrape: &Scrape) -> Result<(), BoxError> {
    println!("{i}. {}", sample.timestamp.to_string().bright_white());
    if !sample.labels.is_empty() {
        println!("{}: {}", "Labels".bold(), sample.labels);
    }

    match Type::of(&sample.value) {
        Type::Histogram => {
            let Value::Histogram(data) = &sample.value else {
                panic!("Wrong type");
            };

            let count = scrape
                .samples
                .iter()
                .find(|s| s.metric == format!("{}_count", sample.metric))
                .ok_or_else(|| format!("No count for {}", sample.metric))?;

            let sum = scrape
                .samples
                .iter()
                .find(|s| s.metric == format!("{}_sum", sample.metric))
                .ok_or_else(|| format!("No sum for {}", sample.metric))?;

            let count = value_to_num(&count.value);
            let sum = value_to_num(&sum.value);
            let mean = sum / count;

            println!(
                "{}: count={}, sum={}, mean={}, buckets={}",
                "Histogram".bold(),
                format!("{count}").green(),
                format!("{sum:.2}").green(),
                format!("{mean:.2}").green(),
                format!("{}", data.len()).green()
            );

            println!();

            let points = data
                .iter()
                .map(|c| (c.less_than as f32, c.count as f32))
                .filter(|c| c.0 != f32::INFINITY)
                .collect::<Vec<_>>();

            let min = points
                .iter()
                .map(|(x, _)| *x)
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();

            let max = points
                .iter()
                .map(|(x, _)| *x)
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();

            textplots::Chart::new(180, 60, min, max)
                .lineplot(&textplots::Shape::Bars(&points))
                .nice();
        }
    }

    Ok(())
}

fn value_to_num(value: &Value) -> f64 {
    match value {
        Value::Counter(v) => *v,
        Value::Gauge(v) => *v,
        Value::Untyped(v) => *v,
        Value::Histogram(_) => unreachable!(),
        Value::Summary(_) => unreachable!(),
    }
}

#[derive(Copy, Clone)]
pub enum Type {
    Histogram,
}

impl Type {
    pub fn of(value: &Value) -> Self {
        match value {
            Value::Histogram(_) => Type::Histogram,
            _ => unimplemented!(),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Histogram => write!(f, "Histogram"),
        }
    }
}
