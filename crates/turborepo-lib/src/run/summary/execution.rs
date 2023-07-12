use std::collections::HashMap;

use chrono::Utc;
use turbopath::AnchoredSystemPathBuf;

// Represents the status of a target when we log a build result.
enum ExecutionEventName {
    Initialized,
    Building,
    BuildStopped,
    Executed,
    Built,
    Cached,
    BuildFailed,
}

struct TaskExecutionSummary {
    start_at: chrono::DateTime<Utc>,
    status: ExecutionEventName,
    err: Option<String>,
    duration: chrono::Duration,
    exit_code: u32,
}

// The state of the entire `turbo run`. Individual task state in `Tasks` field
struct ExecutionSummary {
    #[serde(skip)]
    tasks: HashMap<String, TaskExecutionSummary>,
    #[serde(skip)]
    profile_filename: String,

    // These get serialized to JSON
    // A synthesized turbo command to produce this invocation
    command: String,
    repo_path: AnchoredSystemPathBuf,
    // number of tasks that exited successfully (does not include cache hits)
    success: int,
    // number of tasks that exited with failure
    failure: int,
    // number of tasks that had a cache hit
    cached: int,
    // number of tasks that started
    attempted: int,
    started_at: chrono::DateTime<Utc>,
    ended_at: chrono::DateTime<Utc>,
    exit_code: u32,
}
