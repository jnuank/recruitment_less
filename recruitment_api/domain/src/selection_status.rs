use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialOrd, PartialEq)]
pub enum SelectionProcessStatus{
    スカウト,
    エントリー,
    カジュアル面談,
    一次面接,
    二次面接,
    最終面接,
    オファー,
}

#[derive(Debug, Clone, Serialize, PartialOrd, PartialEq)]
pub enum SelectionStatus {
    選考中,
    辞退,
    お見送り,
}