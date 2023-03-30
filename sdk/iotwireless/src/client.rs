// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[doc(inline)]
                    pub use aws_smithy_client::Builder;
#[derive(Debug)]
                pub(crate) struct Handle {
                    pub(crate) client: aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>,
                    pub(crate) conf: crate::Config,
                }

                /// Client for AWS IoT Wireless
                    ///
                    /// Client for invoking operations on AWS IoT Wireless. Each operation on AWS IoT Wireless is a method on this
                    /// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
/// ## Constructing a `Client`
/// 
/// A [`Config`] is required to construct a client. For most use cases, the [`aws-config`]
/// crate should be used to automatically resolve this config using
/// [`aws_config::load_from_env()`], since this will resolve an [`SdkConfig`] which can be shared
/// across multiple different AWS SDK clients. This config resolution process can be customized
/// by calling [`aws_config::from_env()`] instead, which returns a [`ConfigLoader`] that uses
/// the [builder pattern] to customize the default config.
/// 
/// In the simplest case, creating a client looks as follows:
/// ```rust,no_run
/// # async fn wrapper() {
/// let config = aws_config::load_from_env().await;
/// let client = aws_sdk_iotwireless::Client::new(&config);
/// # }
/// ```
/// 
/// Occasionally, SDKs may have additional service-specific that can be set on the [`Config`] that
/// is absent from [`SdkConfig`], or slightly different settings for a specific client may be desired.
/// The [`Config`] struct implements `From<&SdkConfig>`, so setting these specific settings can be
/// done as follows:
/// 
/// ```rust,no_run
/// # async fn wrapper() {
/// let sdk_config = aws_config::load_from_env().await;
/// let config = aws_sdk_iotwireless::config::Builder::from(&sdk_config)
/// # /*
///     .some_service_specific_setting("value")
/// # */
///     .build();
/// # }
/// ```
/// 
/// See the [`aws-config` docs] and [`Config`] for more information on customizing configuration.
/// 
/// _Note:_ Client construction is expensive due to connection thread pool initialization, and should
/// be done once at application start-up.
/// 
/// [`Config`]: crate::Config
/// [`ConfigLoader`]: https://docs.rs/aws-config/*/aws_config/struct.ConfigLoader.html
/// [`SdkConfig`]: https://docs.rs/aws-config/*/aws_config/struct.SdkConfig.html
/// [`aws-config` docs]: https://docs.rs/aws-config/*
/// [`aws-config`]: https://crates.io/crates/aws-config
/// [`aws_config::from_env()`]: https://docs.rs/aws-config/*/aws_config/fn.from_env.html
/// [`aws_config::load_from_env()`]: https://docs.rs/aws-config/*/aws_config/fn.load_from_env.html
/// [builder pattern]: https://rust-lang.github.io/api-guidelines/type-safety.html#builders-enable-construction-of-complex-values-c-builder
/// # Using the `Client`
/// 
/// A client has a function for every operation that can be performed by the service.
/// For example, the [`AssociateAwsAccountWithPartnerAccount`](crate::operation) operation has
/// a [`Client::associate_aws_account_with_partner_account`], function which returns a builder for that operation.
/// The fluent builder ultimately has a `call()` function that returns an async future that
/// returns a result, as illustrated below:
/// 
/// ```rust,ignore
/// let result = client.associate_aws_account_with_partner_account()
///     .client_request_token("example")
///     .call()
///     .await;
/// ```
/// 
/// The underlying HTTP requests that get made by this can be modified with the `customize_operation`
/// function on the fluent builder. See the [`customize`](crate::client::customize) module for more
/// information.
                #[derive(std::fmt::Debug)]
                pub struct Client {
                    handle: std::sync::Arc<Handle>
                }

                impl std::clone::Clone for Client {
                    fn clone(&self) -> Self {
                        Self { handle: self.handle.clone() }
                    }
                }

                impl From<aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>> for Client {
                    fn from(client: aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>) -> Self {
                        Self::with_config(client, crate::Config::builder().build())
                    }
                }

                impl Client {
                    /// Creates a client with the given service configuration.
                    pub fn with_config(client: aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>, conf: crate::Config) -> Self {
                        Self {
                            handle: std::sync::Arc::new(Handle {
                                client,
                                conf,
                            })
                        }
                    }

                    /// Returns the client's configuration.
                    pub fn conf(&self) -> &crate::Config {
                        &self.handle.conf
                    }
                }

impl Client {
    /// Creates a new client from an [SDK Config](aws_types::sdk_config::SdkConfig).
                    ///
                    /// # Panics
                    ///
                    /// - This method will panic if the `sdk_config` is missing an async sleep implementation. If you experience this panic, set
                    ///     the `sleep_impl` on the Config passed into this function to fix it.
                    /// - This method will panic if the `sdk_config` is missing an HTTP connector. If you experience this panic, set the
                    ///     `http_connector` on the Config passed into this function to fix it.
                    pub fn new(sdk_config: &aws_types::sdk_config::SdkConfig) -> Self {
                        Self::from_conf(sdk_config.into())
                    }
    
                    /// Creates a new client from the service [`Config`](crate::Config).
                    ///
                    /// # Panics
                    ///
                    /// - This method will panic if the `conf` is missing an async sleep implementation. If you experience this panic, set
                    ///     the `sleep_impl` on the Config passed into this function to fix it.
                    /// - This method will panic if the `conf` is missing an HTTP connector. If you experience this panic, set the
                    ///     `http_connector` on the Config passed into this function to fix it.
                    pub fn from_conf(conf: crate::Config) -> Self {
                        let retry_config = conf.retry_config().cloned().unwrap_or_else(aws_smithy_types::retry::RetryConfig::disabled);
                        let timeout_config = conf.timeout_config().cloned().unwrap_or_else(aws_smithy_types::timeout::TimeoutConfig::disabled);
                        let sleep_impl = conf.sleep_impl();
                        if (retry_config.has_retry() || timeout_config.has_timeouts()) && sleep_impl.is_none() {
                            panic!("An async sleep implementation is required for retries or timeouts to work. \
                                    Set the `sleep_impl` on the Config passed into this function to fix this panic.");
                        }
    
                        let connector = conf.http_connector().and_then(|c| {
                            let timeout_config = conf
                                .timeout_config()
                                .cloned()
                                .unwrap_or_else(aws_smithy_types::timeout::TimeoutConfig::disabled);
                            let connector_settings = aws_smithy_client::http_connector::ConnectorSettings::from_timeout_config(
                                &timeout_config,
                            );
                            c.connector(&connector_settings, conf.sleep_impl())
                        });
    
                        let builder = aws_smithy_client::Builder::new();
    
                        let builder = match connector {
                            // Use provided connector
                            Some(c) => builder.connector(c),
                            None =>{
                                #[cfg(any(feature = "rustls", feature = "native-tls"))]
                                {
                                    // Use default connector based on enabled features
                                    builder.dyn_https_connector(aws_smithy_client::http_connector::ConnectorSettings::from_timeout_config(&timeout_config))
                                }
                                #[cfg(not(any(feature = "rustls", feature = "native-tls")))]
                                {
                                    panic!("No HTTP connector was available. Enable the `rustls` or `native-tls` crate feature or set a connector to fix this.");
                                }
                            }
                        };
                        let mut builder = builder
                            .middleware(aws_smithy_client::erase::DynMiddleware::new(crate::middleware::DefaultMiddleware::new()))
                            .retry_config(retry_config.into())
                            .operation_timeout_config(timeout_config.into());
                        builder.set_sleep_impl(sleep_impl);
                        let client = builder.build();
    
                        Self { handle: std::sync::Arc::new(Handle { client, conf }) }
                    }
}

mod associate_aws_account_with_partner_account;

mod associate_multicast_group_with_fuota_task;

mod associate_wireless_device_with_fuota_task;

mod associate_wireless_device_with_multicast_group;

mod associate_wireless_device_with_thing;

mod associate_wireless_gateway_with_certificate;

mod associate_wireless_gateway_with_thing;

mod cancel_multicast_group_session;

mod create_destination;

mod create_device_profile;

mod create_fuota_task;

mod create_multicast_group;

mod create_network_analyzer_configuration;

mod create_service_profile;

mod create_wireless_device;

mod create_wireless_gateway;

mod create_wireless_gateway_task;

mod create_wireless_gateway_task_definition;

mod delete_destination;

mod delete_device_profile;

mod delete_fuota_task;

mod delete_multicast_group;

mod delete_network_analyzer_configuration;

mod delete_queued_messages;

mod delete_service_profile;

mod delete_wireless_device;

mod delete_wireless_gateway;

mod delete_wireless_gateway_task;

mod delete_wireless_gateway_task_definition;

mod disassociate_aws_account_from_partner_account;

mod disassociate_multicast_group_from_fuota_task;

mod disassociate_wireless_device_from_fuota_task;

mod disassociate_wireless_device_from_multicast_group;

mod disassociate_wireless_device_from_thing;

mod disassociate_wireless_gateway_from_certificate;

mod disassociate_wireless_gateway_from_thing;

/// Utilities to ergonomically construct a request to the service.
/// 
/// Fluent builders are created through the [`Client`](crate::client::Client) by calling
/// one if its operation methods. After parameters are set using the builder methods,
/// the `send` method can be called to initiate the request.
pub mod fluent_builders;

mod get_destination;

mod get_device_profile;

mod get_event_configuration_by_resource_types;

mod get_fuota_task;

mod get_log_levels_by_resource_types;

mod get_multicast_group;

mod get_multicast_group_session;

mod get_network_analyzer_configuration;

mod get_partner_account;

mod get_position;

mod get_position_configuration;

mod get_position_estimate;

mod get_resource_event_configuration;

mod get_resource_log_level;

mod get_resource_position;

mod get_service_endpoint;

mod get_service_profile;

mod get_wireless_device;

mod get_wireless_device_statistics;

mod get_wireless_gateway;

mod get_wireless_gateway_certificate;

mod get_wireless_gateway_firmware_information;

mod get_wireless_gateway_statistics;

mod get_wireless_gateway_task;

mod get_wireless_gateway_task_definition;

mod list_destinations;

mod list_device_profiles;

mod list_event_configurations;

mod list_fuota_tasks;

mod list_multicast_groups;

mod list_multicast_groups_by_fuota_task;

mod list_network_analyzer_configurations;

mod list_partner_accounts;

mod list_position_configurations;

mod list_queued_messages;

mod list_service_profiles;

mod list_tags_for_resource;

mod list_wireless_devices;

mod list_wireless_gateway_task_definitions;

mod list_wireless_gateways;

mod put_position_configuration;

mod put_resource_log_level;

mod reset_all_resource_log_levels;

mod reset_resource_log_level;

mod send_data_to_multicast_group;

mod send_data_to_wireless_device;

mod start_bulk_associate_wireless_device_with_multicast_group;

mod start_bulk_disassociate_wireless_device_from_multicast_group;

mod start_fuota_task;

mod start_multicast_group_session;

mod tag_resource;

mod test_wireless_device;

mod untag_resource;

mod update_destination;

mod update_event_configuration_by_resource_types;

mod update_fuota_task;

mod update_log_levels_by_resource_types;

mod update_multicast_group;

mod update_network_analyzer_configuration;

mod update_partner_account;

mod update_position;

mod update_resource_event_configuration;

mod update_resource_position;

mod update_wireless_device;

mod update_wireless_gateway;

