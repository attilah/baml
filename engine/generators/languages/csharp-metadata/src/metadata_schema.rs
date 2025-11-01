use serde::{Deserialize, Serialize};
use indexmap::IndexMap;
use internal_baml_core::ir::repr::IntermediateRepr;
use internal_baml_parser_database::RetryPolicyStrategy;
use internal_llm_client::{ClientProvider, OpenAIClientProviderVariant};

/// Root metadata structure matching the C# BamlMetadata class
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CSharpMetadata {
    pub version: String,
    pub namespace: String,
    pub types: Vec<TypeDefinition>,
    pub functions: Vec<FunctionDefinition>,
    #[serde(rename = "typeAliases")]
    pub type_aliases: Vec<TypeAliasDefinition>,
    pub clients: Vec<ClientDefinition>,
    #[serde(rename = "retryPolicies")]
    pub retry_policies: Vec<RetryPolicyDefinition>,
}

/// Type definition (class, record, or enum)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeDefinition {
    pub name: String,
    pub kind: String, // "Record", "Class", or "Enum"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<PropertyDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<EnumValueDefinition>>,
    #[serde(rename = "isDynamic")]
    pub is_dynamic: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Vec<ConstraintDefinition>>,
}

/// Property definition for classes/records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyDefinition {
    pub name: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub nullable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "defaultValue")]
    pub default_value: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Vec<ConstraintDefinition>>,
}

/// Enum value definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumValueDefinition {
    pub name: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
}

/// Function definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub name: String,
    #[serde(rename = "async")]
    pub is_async: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub parameters: Vec<ParameterDefinition>,
    #[serde(rename = "returnType")]
    pub return_type: String,
    #[serde(rename = "returnNullable")]
    pub return_nullable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "returnSummary")]
    pub return_summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "defaultClient")]
    pub default_client: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tests: Option<Vec<TestCaseDefinition>>,
}

/// Parameter definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDefinition {
    pub name: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub nullable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "defaultValue")]
    pub default_value: Option<serde_json::Value>,
}

/// Type alias definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeAliasDefinition {
    pub name: String,
    #[serde(rename = "aliasFor")]
    pub alias_for: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Constraint definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintDefinition {
    #[serde(rename = "type")]
    pub constraint_type: String, // "Assert" or "Check"
    pub expression: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Client definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientDefinition {
    pub name: String,
    pub provider: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "retryPolicy")]
    pub retry_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<IndexMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Retry policy definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicyDefinition {
    pub name: String,
    #[serde(rename = "maxRetries")]
    pub max_retries: u32,
    pub strategy: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<IndexMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Test case definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCaseDefinition {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub arguments: IndexMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "expectedResult")]
    pub expected_result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Vec<ConstraintDefinition>>,
}

impl CSharpMetadata {
    pub fn from_ir(ir: &IntermediateRepr) -> Self {
        use crate::ir_to_metadata::{type_to_csharp_string, to_pascal_case, to_camel_case};

        let mut types = Vec::new();

        // Convert enums
        for enum_walker in ir.walk_enums() {
            let enum_item = &enum_walker.item.elem;
            let name = to_pascal_case(&enum_item.name);

            let values = enum_item.values.iter().map(|(value_node, _docstring)| {
                EnumValueDefinition {
                    name: to_pascal_case(value_node.elem.0.as_str()),
                    value: value_node.elem.0.clone(),
                    summary: None, // BAML doesn't currently have summary for enum values
                    description: None, // Could extract from comments if available
                    alias: None, // Could extract from attributes if available
                }
            }).collect();

            types.push(TypeDefinition {
                name,
                kind: "Enum".to_string(),
                summary: None,
                description: None,
                alias: None,
                properties: None,
                values: Some(values),
                is_dynamic: false,
                constraints: None,
            });
        }

        // Convert classes
        for class_walker in ir.walk_classes() {
            let class = &class_walker.item.elem;
            let name = to_pascal_case(&class.name);

            let properties = class.static_fields.iter().map(|field| {
                let (type_name, nullable) = type_to_csharp_string(
                    &field.elem.r#type.elem.to_non_streaming_type(ir)
                );

                PropertyDefinition {
                    name: to_pascal_case(&field.elem.name),
                    type_name,
                    nullable,
                    summary: None,
                    description: None,
                    alias: None,
                    default_value: None,
                    constraints: None,
                }
            }).collect();

            types.push(TypeDefinition {
                name,
                kind: "Record".to_string(), // Use Record as default for data classes
                summary: None,
                description: None,
                alias: None,
                properties: Some(properties),
                values: None,
                is_dynamic: false,
                constraints: None,
            });
        }

        // Convert type aliases
        let type_aliases = ir.type_aliases.iter().map(|alias_node| {
            let alias = &alias_node.elem;
            let name = to_pascal_case(&alias.name);
            let (alias_for, _) = type_to_csharp_string(
                &alias.r#type.elem.to_non_streaming_type(ir)
            );

            TypeAliasDefinition {
                name,
                alias_for,
                summary: None,
                description: None,
            }
        }).collect();

        // Convert functions
        let functions = ir.walk_functions().map(|func_walker| {
            let func = func_walker.elem();
            let name = to_pascal_case(func.name());

            let parameters = func.inputs().iter().map(|(param_name, param_type)| {
                let (type_name, nullable) = type_to_csharp_string(
                    &param_type.to_non_streaming_type(ir)
                );
                ParameterDefinition {
                    name: to_camel_case(param_name),
                    type_name,
                    nullable,
                    summary: None,
                    description: None,
                    default_value: None,
                }
            }).collect();

            let (return_type, return_nullable) = type_to_csharp_string(
                &func.output().to_non_streaming_type(ir)
            );

            FunctionDefinition {
                name,
                is_async: true, // BAML functions are async by default
                summary: None,
                description: None,
                parameters,
                return_type,
                return_nullable,
                return_summary: None,
                default_client: None, // Could extract from func if available
                tests: None,
            }
        }).collect();

        // Convert clients
        let clients = ir.clients.iter().map(|client_node| {
            let client = &client_node.elem;
            let name = to_pascal_case(&client.name);
            let provider_str = client_provider_to_string(&client.provider);

            ClientDefinition {
                name,
                provider: provider_str,
                retry_policy: client.retry_policy_id.clone(),
                options: None, // Options are opaque and handled by BAML runtime
                summary: None,
                description: None,
            }
        }).collect();

        // Convert retry policies
        let retry_policies = ir.retry_policies.iter().map(|retry_node| {
            let retry = &retry_node.elem;
            let name = retry.name.0.clone();
            let (strategy_name, strategy_options) = retry_strategy_to_metadata(&retry.strategy);

            RetryPolicyDefinition {
                name,
                max_retries: retry.max_retries,
                strategy: strategy_name,
                options: Some(strategy_options),
                summary: None,
                description: None,
            }
        }).collect();

        CSharpMetadata {
            version: "1.0.0".to_string(),
            namespace: "BamlClient".to_string(),
            types,
            functions,
            type_aliases,
            clients,
            retry_policies,
        }
    }
}

/// Convert ClientProvider enum to string representation
fn client_provider_to_string(provider: &ClientProvider) -> String {

    match provider {
        ClientProvider::OpenAI(variant) => match variant {
            OpenAIClientProviderVariant::Base => "openai".to_string(),
            OpenAIClientProviderVariant::Ollama => "ollama".to_string(),
            OpenAIClientProviderVariant::Azure => "azure-openai".to_string(),
            OpenAIClientProviderVariant::Responses => "openai-responses".to_string(),
            OpenAIClientProviderVariant::Generic => "openai-generic".to_string(),
        },
        ClientProvider::Anthropic => "anthropic".to_string(),
        ClientProvider::AwsBedrock => "aws-bedrock".to_string(),
        ClientProvider::GoogleAi => "google-ai".to_string(),
        ClientProvider::Vertex => "vertex-ai".to_string(),
        ClientProvider::Strategy(_) => "strategy".to_string(),
    }
}

/// Convert RetryPolicyStrategy to metadata format
fn retry_strategy_to_metadata(
    strategy: &RetryPolicyStrategy,
) -> (String, IndexMap<String, serde_json::Value>) {

    match strategy {
        RetryPolicyStrategy::ConstantDelay(constant) => {
            let mut options = IndexMap::new();
            options.insert(
                "delayMs".to_string(),
                serde_json::Value::Number(constant.delay_ms.into()),
            );
            ("ConstantDelay".to_string(), options)
        }
        RetryPolicyStrategy::ExponentialBackoff(exponential) => {
            let mut options = IndexMap::new();
            options.insert(
                "delayMs".to_string(),
                serde_json::Value::Number(exponential.delay_ms.into()),
            );
            options.insert(
                "multiplier".to_string(),
                serde_json::Number::from_f64(exponential.multiplier as f64)
                    .map(serde_json::Value::Number)
                    .unwrap_or(serde_json::Value::Null),
            );
            options.insert(
                "maxDelayMs".to_string(),
                serde_json::Value::Number(exponential.max_delay_ms.into()),
            );
            ("ExponentialBackoff".to_string(), options)
        }
    }
}
