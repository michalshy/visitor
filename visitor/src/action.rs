use crate::state::PickType;

pub enum Action {
    ListDir,
    MoveToParent,
    GetFileDetails { idx: usize },
    Execute { idx: usize },
    Pick { idx: usize, pick_type: PickType },
    ActPicked,
}