use crate::{FeatureFlag, PresenceFlag, register_feature_flag};

pub struct NotebookFeatureFlag;

impl FeatureFlag for NotebookFeatureFlag {
    const NAME: &'static str = "notebooks";
    type Value = PresenceFlag;
}
register_feature_flag!(NotebookFeatureFlag);

pub struct PanicFeatureFlag;

impl FeatureFlag for PanicFeatureFlag {
    const NAME: &'static str = "panic";
    type Value = PresenceFlag;
}
register_feature_flag!(PanicFeatureFlag);

pub struct DiffReviewFeatureFlag;

impl FeatureFlag for DiffReviewFeatureFlag {
    const NAME: &'static str = "diff-review";
    type Value = PresenceFlag;

    fn enabled_for_staff() -> bool {
        false
    }
}
register_feature_flag!(DiffReviewFeatureFlag);

pub struct ProjectPanelUndoRedoFeatureFlag;

impl FeatureFlag for ProjectPanelUndoRedoFeatureFlag {
    const NAME: &'static str = "project-panel-undo-redo";
    type Value = PresenceFlag;

    fn enabled_for_staff() -> bool {
        true
    }
}
register_feature_flag!(ProjectPanelUndoRedoFeatureFlag);

pub struct AutoWatchFeatureFlag;

impl FeatureFlag for AutoWatchFeatureFlag {
    const NAME: &'static str = "auto-watch-screens";
    type Value = PresenceFlag;
}
register_feature_flag!(AutoWatchFeatureFlag);
