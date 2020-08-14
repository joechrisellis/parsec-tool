// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Lists the authenticators supported by the Parsec service.

pub use crate::cli::ParsecToolApp;
use crate::error::ParsecToolError;
use crate::subcommands::ParsecToolSubcommand;
use parsec_client::core::interface::requests::ProviderID;
use parsec_client::core::operation_client::OperationClient;
use parsec_interface::operations::list_authenticators;
use parsec_interface::operations::{NativeOperation, NativeResult};
use std::convert::TryFrom;
use structopt::StructOpt;

/// Lists the authenticators supported by the Parsec service.
#[derive(Debug, StructOpt)]
#[structopt(name = "list_authenticators")]
pub struct ListAuthenticatorsSubcommand {}

impl TryFrom<&ListAuthenticatorsSubcommand> for NativeOperation {
    type Error = ParsecToolError;

    fn try_from(
        _list_authenticators_subcommand: &ListAuthenticatorsSubcommand,
    ) -> Result<Self, Self::Error> {
        // Trivially converted to a `NativeOperation`.
        Ok(NativeOperation::ListAuthenticators(
            list_authenticators::Operation {},
        ))
    }
}

impl ParsecToolSubcommand<'_> for ListAuthenticatorsSubcommand {
    /// Lists the authenticators supported by the Parsec service.
    fn run(&self, matches: &ParsecToolApp) -> Result<(), ParsecToolError> {
        let client = OperationClient::new();
        let native_result = client.process_operation(
            NativeOperation::try_from(self)?,
            ProviderID::Core,
            &matches.authentication_data(),
        )?;

        if let NativeResult::ListAuthenticators(result) = native_result {
            info!("Available authenticators:");
            for authenticator in result.authenticators {
                title!("0x{:02x} ({:?})", authenticator.id as u32, authenticator.id);
                field!("Description", "{}", authenticator.description);
                field!(
                    "Version",
                    "{}.{}.{}",
                    authenticator.version_maj,
                    authenticator.version_min,
                    authenticator.version_rev
                );
                println!();
            }
            Ok(())
        } else {
            Err(ParsecToolError::UnexpectedNativeResult(native_result))
        }
    }
}
