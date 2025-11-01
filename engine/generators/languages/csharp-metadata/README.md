# BAML C# Metadata Generator

This generator produces JSON metadata files that can be consumed by C# source generators to create BAML client implementations.

## Overview

The C# metadata generator converts BAML IR (Intermediate Representation) into a structured JSON file containing:
- Type definitions (enums, classes, records)
- Function signatures with parameters and return types
- LLM client configurations
- Retry policy definitions
- Type aliases

## Output Format

The generator produces a single file: `baml-metadata.json`

### Example Output

```json
{
  "version": "1.0.0",
  "namespace": "BamlClient",
  "types": [...],
  "functions": [...],
  "typeAliases": [],
  "clients": [...],
  "retryPolicies": [...]
}
```

## Usage

### In BAML File

```baml
generator csharp_metadata {
  output_type csharp/metadata
  output_dir "../output"
  version "0.212.0"
}
```

### Generate Metadata

```bash
baml-cli generate
```

## JSON Schema

A complete JSON Schema is provided in `baml-metadata-schema.json` for validation and IDE support.

### Schema Highlights

#### Provider Names (Enum)
Valid client provider values:
- `openai`
- `anthropic`
- `aws-bedrock`
- `google-ai`
- `vertex-ai`
- `ollama`
- `azure-openai`
- `openai-responses`
- `openai-generic`
- `strategy`

#### Retry Policy Strategies (Enum)
Valid strategy types:
- `ConstantDelay` - Fixed delay between retries
  - Required options: `delayMs`
- `ExponentialBackoff` - Exponentially increasing delay
  - Required options: `delayMs`, `multiplier`, `maxDelayMs`

#### Type Kinds (Enum)
Valid type classifications:
- `Enum` - Enumeration type (requires `values` array)
- `Record` - Record type (requires `properties` array)
- `Class` - Class type (requires `properties` array)

#### Constraint Types (Enum)
Valid constraint types:
- `Assert` - Hard constraint that must pass
- `Check` - Soft constraint for validation

## Validation

Use the provided validation script to verify generated metadata:

```bash
./validate-schema.sh path/to/baml-metadata.json
```

The script validates:
- JSON syntax correctness
- Required field presence
- Enum value validity (providers, strategies, type kinds)
- Type-specific requirements (e.g., Enum must have values)
- Retry policy options based on strategy type

## Type Conversion

BAML types are converted to C# types as follows:

### Primitive Types
- `string` → `string`
- `int` → `int`
- `float` → `double`
- `bool` → `bool`
- `null` → `object`

### Media Types
- `image` → `BamlImage`
- `audio` → `BamlAudio`
- `video` → `BamlVideo`
- `pdf` → `BamlPdf`

### Complex Types
- `T[]` → `List<T>`
- `map<K, V>` → `Dictionary<K, V>`
- `T?` → Nullable (indicated by `nullable: true`)

### Custom Types
- Class names → PascalCase
- Enum names → PascalCase
- Type aliases → PascalCase

## Naming Conventions

### Types and Properties
- Type names: **PascalCase** (e.g., `UserProfile`, `Recipe`)
- Property names: **PascalCase** (e.g., `FirstName`, `EmailAddress`)
- Enum value names: **PascalCase** (e.g., `Happy`, `Sad`)

### Functions and Parameters
- Function names: **PascalCase** (e.g., `GetUser`, `AnalyzeSentiment`)
- Parameter names: **camelCase** (e.g., `userId`, `inputText`)

## Client Configuration

Clients include:
- **name**: Client identifier (PascalCase)
- **provider**: LLM provider type (see enum above)
- **retryPolicy**: Optional reference to retry policy name
- **options**: Provider-specific configuration (handled by BAML runtime)

## Retry Policies

Retry policies are handled entirely by the BAML runtime. The C# code only needs to reference the policy name when calling functions.

### Policy Structure
- **name**: Policy identifier
- **maxRetries**: Maximum number of retry attempts
- **strategy**: Strategy type (ConstantDelay or ExponentialBackoff)
- **options**: Strategy-specific configuration
  - `delayMs`: Initial delay in milliseconds
  - `multiplier`: Backoff multiplier (ExponentialBackoff only)
  - `maxDelayMs`: Maximum delay cap (ExponentialBackoff only)

## Implementation Details

### Generator Location
`baml/engine/generators/languages/csharp-metadata/`

### Key Files
- `src/lib.rs` - Main generator implementation
- `src/metadata_schema.rs` - Metadata structure definitions
- `src/ir_to_metadata.rs` - Type conversion logic
- `baml-metadata-schema.json` - JSON Schema for validation
- `validate-schema.sh` - Validation script

### Dependencies
- `internal-baml-core` - IR access
- `internal-baml-parser-database` - Retry policy types
- `internal-llm-client` - Client provider types
- `serde` + `serde_json` - JSON serialization
- `convert_case` - Name case conversion

## Testing

The generator includes comprehensive test coverage through the BAML test harness.

### Running Tests
```bash
cargo test -p generators-csharp-metadata
```

### Test Fixtures
Test cases are located in `baml/engine/generators/data/*/` and are automatically discovered by the test harness.

## For C# Source Generator Developers

To consume this metadata in your C# source generator:

1. Parse the JSON file using `System.Text.Json`
2. Deserialize into your metadata model classes
3. Generate C# client code based on the metadata
4. The BAML runtime handles:
   - LLM communication
   - Retry logic
   - Type conversion
   - Serialization/deserialization

### Example C# Deserialization

```csharp
using System.Text.Json;

var json = File.ReadAllText("baml-metadata.json");
var metadata = JsonSerializer.Deserialize<BamlMetadata>(json);

foreach (var function in metadata.Functions)
{
    // Generate client method for this function
}
```

## Version History

- **1.0.0** - Initial release with full IR conversion support
  - Types, functions, clients, retry policies, type aliases
  - Complete enum support for providers and strategies
  - PascalCase/camelCase naming conventions
  - JSON Schema validation

## License

Part of the BAML project. See main BAML license for details.
