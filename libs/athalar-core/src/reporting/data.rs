use super::{ReportLevel, ReportMessage, ReportMessageOrigin};

pub struct GeneratorReportCreator {}

impl GeneratorReportCreator {
    pub fn can_not_create_file(location: &str) -> ReportMessage {
        ReportMessage {
            origin: ReportMessageOrigin::GeneratorBinding,
            code: "CanNotCreateFile",
            level: ReportLevel::Severe,
            message: format!("File at location {:?} can not be created", location),
        }
    }

    pub fn file_already_exists(location: &str) -> ReportMessage {
        ReportMessage {
            origin: ReportMessageOrigin::GeneratorBinding,
            code: "FileAlreadyExists",
            level: ReportLevel::Warning,
            message: format!("The file {:?} already exists", location),
        }
    }

    pub fn file_conflict(b1: &str, b2: &str) -> ReportMessage {
        ReportMessage {
            origin: ReportMessageOrigin::GeneratorBinding,
            code: "FileConflict",
            level: ReportLevel::Warning,
            message: format!(
                "Generator binding {:?} and {:?} have the same output location",
                b1, b2
            ),
        }
    }

    pub fn partial_does_not_exist(partial_name: &str) -> ReportMessage {
        ReportMessage {
            origin: ReportMessageOrigin::GeneratorConfig,
            code: "PartialDoesNotExist",
            level: ReportLevel::Severe,
            message: format!("Named partial {:?} does not exist", partial_name),
        }
    }
}

pub struct PartialReportCreator {}

impl PartialReportCreator {
    pub fn name_conflict(partial_name: &str) -> ReportMessage {
        ReportMessage {
            origin: ReportMessageOrigin::PartialConfig,
            code: "NameConflict",
            level: ReportLevel::Warning,
            message: format!(
                "Partial name {:?} conflicts with another partial of the same name",
                partial_name
            ),
        }
    }
}
