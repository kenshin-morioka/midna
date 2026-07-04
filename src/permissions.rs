#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    Allow,
    Ask,
    Deny,
}

/// ツールの操作が持つ危険度。権限判定の入力になる。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Risk {
    /// 読むだけで状態を変えない（ファイル読み取り、一覧）
    ReadOnly,
    /// ファイルシステムを書き換える
    Write,
    /// 任意のコマンドを実行する
    Execute,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Policy;

impl Policy {
    pub fn new() -> Self {
        Self
    }

    /// 読み取りは自動許可、状態を変える操作は毎回ユーザーに確認する。
    pub fn check(&self, risk: Risk) -> Permission {
        match risk {
            Risk::ReadOnly => Permission::Allow,
            Risk::Write | Risk::Execute => Permission::Ask,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_only_is_allowed_without_asking() {
        assert_eq!(Policy::new().check(Risk::ReadOnly), Permission::Allow);
    }

    #[test]
    fn state_changing_operations_require_confirmation() {
        let policy = Policy::new();
        assert_eq!(policy.check(Risk::Write), Permission::Ask);
        assert_eq!(policy.check(Risk::Execute), Permission::Ask);
    }
}
