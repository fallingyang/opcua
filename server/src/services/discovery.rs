use std::result::Result;

use opcua_core;
use opcua_core::types::*;
use opcua_core::services::*;
use opcua_core::comms::*;

use types::*;
use server::ServerState;

pub struct DiscoveryService {}

impl DiscoveryService {
    pub fn new() -> DiscoveryService {
        DiscoveryService {}
    }

    pub fn get_endpoints(&self, server_state: &mut ServerState, _: &mut SessionState, request: &GetEndpointsRequest) -> Result<SupportedMessage, &'static StatusCode> {
        debug!("get_endpoints");

        let server_certificate = server_state.server_certificate.clone();

        let mut endpoints: Vec<EndpointDescription> = Vec::with_capacity(server_state.endpoints.len());
        for e in &server_state.endpoints {
            let mut user_identity_tokens = Vec::new();
            if e.anonymous {
                user_identity_tokens.push(UserTokenPolicy::new_anonymous());
            }
            // TODO username / pass
            // user_identity_tokens.push(UserTokenPolicy::new_user_pass());

            endpoints.push(EndpointDescription {
                endpoint_url: UAString::from_str(&e.endpoint_url),
                server: ApplicationDescription {
                    application_uri: server_state.application_uri.clone(),
                    product_uri: server_state.product_uri.clone(),
                    application_name: server_state.application_name.clone(),
                    application_type: ApplicationType::Server,
                    gateway_server_uri: UAString::null(),
                    discovery_profile_uri: UAString::null(),
                    discovery_urls: None,
                },
                server_certificate: server_certificate.clone(),
                security_mode: e.security_mode,
                security_policy_uri: e.security_policy_uri.clone(),
                user_identity_tokens: Some(user_identity_tokens),
                transport_profile_uri: UAString::from_str(opcua_core::profiles::TRANSPORT_BINARY),
                security_level: 1,
            });
        }

        let response = GetEndpointsResponse {
            response_header: ResponseHeader::new(&DateTime::now(), &request.request_header),
            endpoints: Some(endpoints),
        };
        Ok(SupportedMessage::GetEndpointsResponse(response))
    }
}
