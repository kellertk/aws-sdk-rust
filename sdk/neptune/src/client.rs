// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[doc(inline)]
                    pub use aws_smithy_client::Builder;
#[derive(Debug)]
                pub(crate) struct Handle {
                    pub(crate) client: aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>,
                    pub(crate) conf: crate::Config,
                }

                /// Client for Amazon Neptune
                    ///
                    /// Client for invoking operations on Amazon Neptune. Each operation on Amazon Neptune is a method on this
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
/// let client = aws_sdk_neptune::Client::new(&config);
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
/// let config = aws_sdk_neptune::config::Builder::from(&sdk_config)
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
/// For example, the [`AddRoleToDBCluster`](crate::operation) operation has
/// a [`Client::add_role_to_db_cluster`], function which returns a builder for that operation.
/// The fluent builder ultimately has a `call()` function that returns an async future that
/// returns a result, as illustrated below:
/// 
/// ```rust,ignore
/// let result = client.add_role_to_db_cluster()
///     .db_cluster_identifier("example")
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

mod add_role_to_db_cluster;

mod add_source_identifier_to_subscription;

mod add_tags_to_resource;

mod apply_pending_maintenance_action;

mod copy_db_cluster_parameter_group;

mod copy_db_cluster_snapshot;

mod copy_db_parameter_group;

mod create_db_cluster;

mod create_db_cluster_endpoint;

mod create_db_cluster_parameter_group;

mod create_db_cluster_snapshot;

mod create_db_instance;

mod create_db_parameter_group;

mod create_db_subnet_group;

mod create_event_subscription;

mod create_global_cluster;

mod delete_db_cluster;

mod delete_db_cluster_endpoint;

mod delete_db_cluster_parameter_group;

mod delete_db_cluster_snapshot;

mod delete_db_instance;

mod delete_db_parameter_group;

mod delete_db_subnet_group;

mod delete_event_subscription;

mod delete_global_cluster;

mod describe_db_cluster_endpoints;

mod describe_db_cluster_parameter_groups;

mod describe_db_cluster_parameters;

mod describe_db_cluster_snapshot_attributes;

mod describe_db_cluster_snapshots;

mod describe_db_clusters;

mod describe_db_engine_versions;

mod describe_db_instances;

mod describe_db_parameter_groups;

mod describe_db_parameters;

mod describe_db_subnet_groups;

mod describe_engine_default_cluster_parameters;

mod describe_engine_default_parameters;

mod describe_event_categories;

mod describe_event_subscriptions;

mod describe_events;

mod describe_global_clusters;

mod describe_orderable_db_instance_options;

mod describe_pending_maintenance_actions;

mod describe_valid_db_instance_modifications;

mod failover_db_cluster;

mod failover_global_cluster;

/// Utilities to ergonomically construct a request to the service.
/// 
/// Fluent builders are created through the [`Client`](crate::client::Client) by calling
/// one if its operation methods. After parameters are set using the builder methods,
/// the `send` method can be called to initiate the request.
pub mod fluent_builders;

mod list_tags_for_resource;

mod modify_db_cluster;

mod modify_db_cluster_endpoint;

mod modify_db_cluster_parameter_group;

mod modify_db_cluster_snapshot_attribute;

mod modify_db_instance;

mod modify_db_parameter_group;

mod modify_db_subnet_group;

mod modify_event_subscription;

mod modify_global_cluster;

mod promote_read_replica_db_cluster;

mod reboot_db_instance;

mod remove_from_global_cluster;

mod remove_role_from_db_cluster;

mod remove_source_identifier_from_subscription;

mod remove_tags_from_resource;

mod reset_db_cluster_parameter_group;

mod reset_db_parameter_group;

mod restore_db_cluster_from_snapshot;

mod restore_db_cluster_to_point_in_time;

mod start_db_cluster;

mod stop_db_cluster;

