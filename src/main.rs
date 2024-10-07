use core::f32;

use clap::Parser;
use color_eyre::eyre::{eyre, Result};
use derive_more::Display;
use ordered_float::OrderedFloat;
use owo_colors::OwoColorize;
use pluralizer::pluralize;
use prometheus_parse::{HistogramCount, Sample, Scrape, Value};
use reqwest::Url;
use textplots::Plot;

#[derive(Debug, Parser)]
pub struct Args {
    pub url: Url,
    pub metric: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();
    let url = ensure_metrics_path(args.url);
    let text = fetch_metrics(url).await?;
    let mut scrape = parse_metrics(&text)?;

    process_metric(&args.metric, &mut scrape)?;

    Ok(())
}

fn ensure_metrics_path(mut url: Url) -> Url {
    if url.path() != "/metrics" {
        url.set_path("/metrics");
    }
    url
}

async fn fetch_metrics(url: Url) -> Result<String> {
    Ok(reqwest::get(url).await?.text().await?)
}

fn parse_metrics(text: &str) -> Result<Scrape> {
    let lines = text.lines().map(|l| Ok(l.to_string()));
    prometheus_parse::Scrape::parse(lines).map_err(Into::into)
}

fn process_metric(metric: &str, scrape: &mut Scrape) -> Result<()> {
    let doc = get_metric_doc(metric, scrape)?;
    print_metric_info(metric, doc);

    filter_samples(metric, scrape);

    if scrape.samples.is_empty() {
        println!("No samples found");
        return Ok(());
    }

    let mut exact_samples = get_exact_samples(metric, scrape);

    // If the metric is a counter, its name is postfixed with `_total`
    if exact_samples.is_empty() {
        exact_samples = get_exact_samples(&format!("{metric}_total"), scrape);
    }

    print_sample_count(&exact_samples);

    for (i, sample) in exact_samples.iter().enumerate() {
        show_sample(i + 1, sample, scrape)?;
    }

    Ok(())
}

fn get_metric_doc<'a>(metric: &str, scrape: &'a Scrape) -> Result<&'a str> {
    scrape
        .docs
        .get(metric)
        .map(|doc| doc.as_str())
        .ok_or_else(|| eyre!("Metric not found: {metric}"))
}

fn print_metric_info(metric: &str, doc: &str) {
    println!("{}", metric.bold().blue().reversed());
    println!("{}", doc.italic());
}

fn filter_samples(metric: &str, scrape: &mut Scrape) {
    scrape.samples.retain(|s| s.metric.starts_with(metric));
}

fn get_exact_samples<'a>(metric: &str, scrape: &'a Scrape) -> Vec<&'a Sample> {
    scrape
        .samples
        .iter()
        .filter(|s| s.metric == metric)
        .collect()
}

fn print_sample_count(samples: &[&Sample]) {
    println!("{}\n", pluralize("sample", samples.len() as isize, true));
}

fn show_sample(i: usize, sample: &Sample, scrape: &Scrape) -> Result<()> {
    println!("{i}. {}", sample.timestamp.to_string().bright_white());
    print_labels(sample);

    match Type::of(&sample.value) {
        Type::Gauge => show_gauge(sample, scrape)?,
        Type::Counter => show_counter(sample, scrape)?,
        Type::Histogram => show_histogram(sample, scrape)?,
    }

    Ok(())
}

fn print_labels(sample: &Sample) {
    if !sample.labels.is_empty() {
        println!("  {}: {}", "Labels".bold(), sample.labels);
    }
}

fn show_gauge(sample: &Sample, _scrape: &Scrape) -> Result<()> {
    let value = value_to_num(&sample.value);
    println!("  {}: {}\n", "Gauge".bold(), value.to_string().green());
    Ok(())
}

fn show_counter(sample: &Sample, _scrape: &Scrape) -> Result<()> {
    let value = value_to_num(&sample.value);
    println!("  {}: {}\n", "Counter".bold(), value.to_string().green());
    Ok(())
}

fn show_histogram(sample: &Sample, scrape: &Scrape) -> Result<()> {
    let Value::Histogram(data) = &sample.value else {
        panic!("Wrong type");
    };

    let (count, sum) = get_histogram_stats(sample, scrape)?;

    print_histogram_stats(count, sum, data.len());

    if !data.is_empty() {
        plot_histogram(data);
    }

    Ok(())
}

fn get_histogram_stats(sample: &Sample, scrape: &Scrape) -> Result<(f64, f64)> {
    let count = find_metric_value(scrape, &format!("{}_count", sample.metric))?;
    let sum = find_metric_value(scrape, &format!("{}_sum", sample.metric))?;
    Ok((count, sum))
}

fn find_metric_value(scrape: &Scrape, metric: &str) -> Result<f64> {
    let sample = scrape
        .samples
        .iter()
        .find(|s| s.metric == metric)
        .ok_or_else(|| eyre!("Cannot find metric `{metric}`"))?;

    Ok(value_to_num(&sample.value))
}

fn print_histogram_stats(count: f64, sum: f64, bucket_count: usize) {
    let mean = sum / count;

    println!(
        "  {}: count={}, sum={}, mean={}, buckets={}\n",
        "Histogram".bold(),
        format!("{count}").green(),
        format!("{sum:.2}").green(),
        format!("{mean:.3}").green(),
        format!("{}", bucket_count).green()
    );
}

fn plot_histogram(data: &[HistogramCount]) {
    let points = data
        .iter()
        .map(|c| (c.less_than as f32, c.count as f32))
        .filter(|c| c.0 != f32::INFINITY)
        .collect::<Vec<_>>();

    let (min, max) = get_min_max(&points);

    textplots::Chart::new(180, 60, min, max)
        .lineplot(&textplots::Shape::Bars(&points))
        .nice();
}

fn get_min_max(points: &[(f32, f32)]) -> (f32, f32) {
    let min = points.iter().map(|(x, _)| OrderedFloat(*x)).min().unwrap();
    let max = points.iter().map(|(x, _)| OrderedFloat(*x)).max().unwrap();

    (min.into_inner(), max.into_inner())
}

fn value_to_num(value: &Value) -> f64 {
    match value {
        Value::Counter(v) | Value::Gauge(v) | Value::Untyped(v) => *v,
        Value::Histogram(_) | Value::Summary(_) => unreachable!(),
    }
}

#[derive(Copy, Clone, Display)]
pub enum Type {
    Gauge,
    Counter,
    Histogram,
}

impl Type {
    pub fn of(value: &Value) -> Self {
        match value {
            Value::Gauge(_) => Type::Gauge,
            Value::Counter(_) => Type::Counter,
            Value::Histogram(_) => Type::Histogram,
            Value::Untyped(_) => Type::Counter,
            _ => unimplemented!(),
        }
    }
}
