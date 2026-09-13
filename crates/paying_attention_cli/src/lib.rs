//! Technical recovery operations shared by the command line and desktop shell.

use paying_attention_storage::SqliteAttentionStore;

pub struct TechnicalRecoveryService<'store> {
    store: &'store mut SqliteAttentionStore,
}

impl<'store> TechnicalRecoveryService<'store> {
    pub fn new(store: &'store mut SqliteAttentionStore) -> Self {
        Self { store }
    }

    pub fn status(&self) -> rusqlite::Result<String> {
        Ok(self
            .store
            .load_restorable_state()?
            .unwrap_or_else(|| "unlocked".into()))
    }

    pub fn force_unlock(&mut self) -> rusqlite::Result<()> {
        self.store.clear_restorable_state()
    }
}
