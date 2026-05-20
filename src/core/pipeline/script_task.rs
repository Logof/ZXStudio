// src/core/pipeline/script_task.rs
use super::{BuildContext, PipelineError, TaskStatus};
use crate::models::ProjectData;

pub fn export_scripts(
    _project: &ProjectData,
    _ctx: &BuildContext,
) -> Result<TaskStatus, PipelineError> {
    Ok(TaskStatus::Success(
        "игровые скрипты скомпилированы (заглушка)".to_string(),
    ))
}
