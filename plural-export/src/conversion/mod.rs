mod pluralkit;
mod tupperbox;


use crate::models::intermediary::IntermediaryExport;


pub(crate) trait IntoIntermediary {
    fn into_intermediary(self, log: &mut Vec<String>) -> IntermediaryExport;
}

pub(crate) trait LoggedFrom<T>: Sized {
    fn logged_from(value: T, log: &mut Vec<String>) -> Self;
}

impl<Source: IntoIntermediary, Target: LoggedFrom<IntermediaryExport>>
    LoggedFrom<Source> for Target
{
    default fn logged_from(value: Source, log: &mut Vec<String>) -> Target {
        Target::logged_from(value.into_intermediary(log), log)
    }
}
