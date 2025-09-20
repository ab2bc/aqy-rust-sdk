// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// ===========================================================================
// Aqyns Queries
// ===========================================================================

use crate::query_types::schema;
use crate::query_types::Address as SdkAddress;

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "rpc",
    graphql_type = "Query",
    variables = "ResolveAqynsQueryArgs"
)]
pub struct ResolveAqynsQuery {
    #[arguments(domain: $name)]
    pub resolve_suins_address: Option<DomainAddress>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct ResolveAqynsQueryArgs<'a> {
    pub name: &'a str,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Address")]
pub struct DomainAddress {
    pub address: SdkAddress,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "rpc",
    graphql_type = "Query",
    variables = "DefaultAqynsNameQueryArgs"
)]
pub struct DefaultAqynsNameQuery {
    #[arguments(address: $address)]
    pub address: Option<AddressDefaultAqyns>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct DefaultAqynsNameQueryArgs {
    pub address: SdkAddress,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Address")]
pub struct AddressDefaultAqyns {
    pub default_suins_name: Option<String>,
}
