mod execution;
mod global_hash;

use global_hash::GlobalHashSummary;
use svix_ksuid::Ksuid;

use crate::cli::EnvMode;

// NOTE: When changing this, please ensure that the server side is updated to
// handle the new version on vercel.com this is required to ensure safe handling
// of env vars (unknown run summary versions will be ignored on the server)
const RUN_SUMMARY_SCHEMA_VERSION: &str = "1";

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct RunSummary {
    id: Ksuid,
    version: String,
    turbo_version: String,
    monorepo: bool,
    global_hash_summary: GlobalHashSummary,
    packages: Vec<String>,
    env_mode: EnvMode,
    framework_inference: bool,
    execution_summary: ExecutionSummary,
    tasks: Vec<TaskSummary>,
    user: String,
    scm: ScmState,
}
