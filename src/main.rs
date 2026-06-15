use std::path::PathBuf;
use std::time::Instant;

use clap::Parser;
use anyhow::Result;

mod model;
mod parser;
mod graph;
mod validator;

use parser::SCDParser;
use graph::{GraphBuilder, TopologyAnalyzer, TarjanSCC, TimingAnalyzer};
use validator::IsolationValidator;

#[derive(Parser, Debug)]
#[command(
    name = "scd-topology-checker",
    version = "1.0.0",
    about = "超高性能变电站配置文件(SCD)语义拓扑校验工具",
    long_about = "用于国家电网新一代数字变电站验收环节的配置合规性终端检测工具"
)]
struct Cli {
    /// SCD 配置文件路径
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,

    /// 输出拓扑统计信息
    #[arg(short = 's', long = "stats", default_value_t = true)]
    stats: bool,

    /// 执行安全隔离校验
    #[arg(short = 'c', long = "check", default_value_t = true)]
    check: bool,

    /// 执行时序延迟分析
    #[arg(short = 't', long = "timing", default_value_t = true)]
    timing: bool,

    /// 时序安全阈值（毫秒）
    #[arg(long = "threshold-ms", default_value_t = 8)]
    threshold_ms: u64,

    /// 输出 Graphviz DOT 格式图
    #[arg(short = 'g', long = "graphviz")]
    graphviz: bool,

    /// 构建多层通信图（包含交换机节点）
    #[arg(short = 'm', long = "multilayer")]
    multilayer: bool,

    /// 详细输出模式
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("============================================================");
    println!("  SCD 拓扑校验工具 v1.0.0 - 国家电网新一代数字变电站");
    println!("============================================================");
    println!();

    let start = Instant::now();

    println!("[1/5] 正在解析 SCD 配置文件...");
    println!("  文件路径: {:?}", cli.input);

    let parse_start = Instant::now();
    let scd_model = SCDParser::parse_file(&cli.input)?;
    let parse_duration = parse_start.elapsed();

    println!("  ✓ 解析完成，耗时: {:.2?}", parse_duration);
    println!("    - IED 数量: {}", scd_model.ieds.len());
    println!("    - 子网数量: {}", scd_model.sub_networks.len());

    let total_goose_subs: usize = scd_model
        .ieds
        .iter()
        .map(|ied| {
            ied.access_points
                .iter()
                .map(|ap| ap.goose_subs.len())
                .sum::<usize>()
        })
        .sum();

    let total_sv_subs: usize = scd_model
        .ieds
        .iter()
        .map(|ied| {
            ied.access_points
                .iter()
                .map(|ap| ap.sv_subs.len())
                .sum::<usize>()
        })
        .sum();

    println!("    - GOOSE 订阅: {}", total_goose_subs);
    println!("    - SV 订阅: {}", total_sv_subs);
    println!();

    println!("[2/5] 正在构建有向图拓扑模型...");
    let graph_start = Instant::now();

    let graph = if cli.multilayer {
        GraphBuilder::build_multilayer_graph(&scd_model)
    } else {
        GraphBuilder::build_from_scd(&scd_model)
    };

    let graph_duration = graph_start.elapsed();
    println!("  ✓ 图构建完成，耗时: {:.2?}", graph_duration);
    println!("    - 节点数: {}", graph.node_count());
    println!("    - 边数: {}", graph.edge_count());
    println!();

    println!("[3/5] 正在分析强连通分量...");
    let scc_start = Instant::now();
    let sccs = TarjanSCC::compute(&graph);
    let scc_duration = scc_start.elapsed();
    println!("  ✓ SCC 分析完成，耗时: {:.2?}", scc_duration);
    println!("    - SCC 数量: {}", sccs.len());
    println!("    - 最大 SCC 规模: {}", sccs.iter().map(|s| s.len()).max().unwrap_or(0));
    println!();

    if cli.stats {
        let stats = TopologyAnalyzer::analyze(&scd_model, &graph);
        TopologyAnalyzer::print_stats(&stats);
    }

    if cli.check {
        println!("[4/5] 正在执行安全隔离规范校验...");
        let check_start = Instant::now();

        let mut validator = IsolationValidator::new();
        let violations = validator.validate_all(&scd_model, &graph);
        let check_duration = check_start.elapsed();

        println!("  ✓ 校验完成，耗时: {:.2?}", check_duration);
        println!("    - 违规项数: {}", violations.len());
        println!();

        validator.print_report();
    } else {
        println!("[4/5] 跳过安全隔离校验");
        println!();
    }

    if cli.timing {
        println!("[5/5] 正在执行继电保护时序动力学分析...");
        let timing_start = Instant::now();

        let timing_result = TimingAnalyzer::analyze(&graph, cli.threshold_ms);
        let timing_duration = timing_start.elapsed();

        println!("  ✓ 时序分析完成，耗时: {:.2?}", timing_duration);
        println!("    - 保护触发端: {}", timing_result.protection_triggers.len());
        println!("    - 断路器终端: {}", timing_result.breaker_terminals.len());
        println!("    - 有效路径数: {}", timing_result.all_paths.len());
        println!("    - 时序违规数: {}", timing_result.violations.len());

        TimingAnalyzer::print_report(&timing_result);
    } else {
        println!("[5/5] 跳过时序延迟分析");
        println!();
    }

    if cli.graphviz {
        println!();
        println!("【 Graphviz DOT 输出 】");
        println!();
        TopologyAnalyzer::print_graphviz(&graph);
    }

    let total_duration = start.elapsed();

    println!();
    println!("============================================================");
    println!("  处理完成，总耗时: {:.2?}", total_duration);
    println!("============================================================");

    Ok(())
}
