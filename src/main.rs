mod goal;
use chrono::Utc;
mod stats;
use env_logger::Builder;
use log::*;
use stats::Stats;

const YEARLY_TARGET: u32 = 1000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Builder::new().filter_level(LevelFilter::Info).init();
    let daily_goal = goal::get_daily_goal(Utc::now(), YEARLY_TARGET);
    info!("Current day target : {}km", daily_goal);
    let mut stats = Stats::new(".secrets");
    if !stats.is_configured() {
        stats.configure_secrets(None, None)?;
    } else {
        info!("Credentials are already configured.");
        stats.update_secrets()?
    }
    let running_yearly_stat = stats.get_running_yearly_stat()?;
    info!("Running yearly stat : {}km", running_yearly_stat);
    if running_yearly_stat >= daily_goal {
        info!(
            "You are ahead of target by {}km !",
            running_yearly_stat - daily_goal
        );
    } else {
        info!(
            "You are behind of target by {}km...",
            daily_goal - running_yearly_stat
        );
    }
    Ok(())
}
