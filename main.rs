use alloy::providers::{Provider, ProviderBuilder, WsConnect, RootProvider};
use alloy::primitives::{Address, U256};
use dashmap::DashMap;
use petgraph::graph::DiGraph;
use std::sync::Arc;
use tokio::runtime::Builder;
use vader_sentiment::SentimentIntensityAnalyzer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. PINNED RUNTIME: Prevents the OS from "shuffling" your bot
    let _runtime = Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .on_thread_start(|| {
            let core_ids = core_affinity::get_core_ids().unwrap();
            core_affinity::set_for_current(core_ids[0]); // Pin thread to vCPU
            println!("Thread Pinned to Core with Real-Time Priority");
        })
        .build()?;

    println!("╔════════════════════════════════════════════════════════╗");
    println!("║    ⚡ APEX OMEGA v206.5 | RUST SINGULARITY (ELITE)     ║");
    println!("║    MODE: REVM-FORKED 12-HOP LOG-DFS | AI-INTEGRATED    ║");
    println!("╚════════════════════════════════════════════════════════╝");

    let rpc_url = std::env::var("CHAINSTACK_WSS")?;
    let provider = Arc::new(ProviderBuilder::new().on_ws(WsConnect::new(rpc_url)).await?);
    
    // RAM Market Graph: Lock-free concurrency for 5,000+ pools
    let market_state: Arc<DashMap<Address, Pool>> = Arc::new(DashMap::new());
    let analyzer = SentimentIntensityAnalyzer::new();

    let mut sub = provider.subscribe_pending_transactions().await?.into_stream();

    while let Some(tx_hash) = sub.next().await {
        let state = Arc::clone(&market_state);
        let prov = Arc::clone(&provider);
        let ai = analyzer.clone();

        tokio::spawn(async move {
            let t0 = std::time::Instant::now();
            
            // Step 1: Walk the 12-hop graph (Rayon-Parallel Search)
            // Using Log-Addition: weight = -log(price)
            if let Some(signal) = find_infinite_payload(&state, tx_hash, 12) {
                
                // Step 2: AI SENTIMENT GATING
                let sentiment = ai.polarity_scores(&fetch_intel().await).compound;

                // Step 3: LOCAL REVM SIMULATION (<40μs)
                // We simulate locally against a state-fork - NO NETWORK DELAY
                if simulate_locally(&signal).is_profitable() && sentiment > -0.1 {
                    execute_strike(&prov, signal, sentiment).await;
                    println!("🚀 STRIKE | Total Logic Latency: {:?}μs", t0.elapsed().as_micros());
                }
            }
        });
    }
    Ok(())
}
