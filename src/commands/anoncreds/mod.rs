pub mod issuer;
pub mod prover;
pub mod verifier;

use commands::anoncreds::issuer::{IssuerCommand, IssuerCommandExecutor};
use commands::anoncreds::prover::{ProverCommand, ProverCommandExecutor};
use commands::anoncreds::verifier::{VerifierCommand, VerifierCommandExecutor};

use services::crypto::CryptoService;
use services::pool::PoolService;
use services::wallet::WalletService;

use std::rc::Rc;

pub enum AnoncredsCommand {
    Issuer(IssuerCommand),
    Prover(ProverCommand),
    Verifier(VerifierCommand),
}

pub struct AnoncredsCommandExecutor {
    issuer_command_cxecutor: IssuerCommandExecutor,
    prover_command_cxecutor: ProverCommandExecutor,
    verifier_command_cxecutor: VerifierCommandExecutor
}

impl AnoncredsCommandExecutor {
    pub fn new(crypto_service: Rc<CryptoService>,
               pool_service: Rc<PoolService>,
               wallet_service: Rc<WalletService>) -> AnoncredsCommandExecutor {
        AnoncredsCommandExecutor {
            issuer_command_cxecutor: IssuerCommandExecutor::new(
                crypto_service.clone(), pool_service.clone(), wallet_service.clone()),
            prover_command_cxecutor: ProverCommandExecutor::new(
                crypto_service.clone(), pool_service.clone(), wallet_service.clone()),
            verifier_command_cxecutor: VerifierCommandExecutor::new(
                crypto_service.clone(), pool_service.clone(), wallet_service.clone()),
        }
    }

    pub fn execute(&self, command: AnoncredsCommand) {
        match command {
            AnoncredsCommand::Issuer(cmd) => {
                info!(target: "anoncreds_command_executor", "Issuer command received");
                self.issuer_command_cxecutor.execute(cmd);
            },
            AnoncredsCommand::Prover(cmd) => {
                info!(target: "anoncreds_command_executor", "Prover command received");
                self.prover_command_cxecutor.execute(cmd);
            },
            AnoncredsCommand::Verifier(cmd) => {
                info!(target: "anoncreds_command_executor", "Verifier command received");
                self.verifier_command_cxecutor.execute(cmd);
            }
        };
    }
}