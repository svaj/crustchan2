use std::sync::Arc;

use axum_gate::{authz::AccessPolicy, codecs::jwt::JsonWebToken, gate::{Gate, bearer::{BearerGate, JwtConfig}}, groups::Group, prelude::{Account, JwtClaims}, roles::Role};


use crate::tuono_main_state::jwt::get_jwt;

#[tuono_lib::middleware]
pub fn admin_middleware() -> BearerGate<JsonWebToken<JwtClaims<Account<Role, Group>>>, Role, Group, JwtConfig<Role, Group>> {
    Gate::bearer("Crustchan", Arc::clone(&get_jwt()))
            .with_policy(AccessPolicy::<Role, Group>::require_role(Role::Admin))
}
