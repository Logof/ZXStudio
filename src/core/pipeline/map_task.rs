// src/core/pipeline/map_task.rs
use super::{BuildContext, PipelineError, TaskStatus};
use crate::models::ProjectData;

pub fn export_map_data(
    _project: &ProjectData,
    _ctx: &BuildContext,
) -> Result<TaskStatus, PipelineError> {
    // В следующих итерациях мы пропишем сюда честную упаковку матриц
    // и генерацию Си-массивов под La Churrera
    Ok(TaskStatus::Success(
        "массивы карты подготовлены (заглушка)".to_string(),
    ))
}
