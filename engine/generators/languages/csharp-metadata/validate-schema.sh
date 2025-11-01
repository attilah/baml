#!/bin/bash
# Validate BAML C# metadata JSON against the schema
# Usage: ./validate-schema.sh <metadata-file.json>

set -e

SCHEMA_FILE="$(dirname "$0")/baml-metadata-schema.json"
METADATA_FILE="$1"

if [ -z "$METADATA_FILE" ]; then
    echo "Usage: $0 <metadata-file.json>"
    exit 1
fi

if [ ! -f "$METADATA_FILE" ]; then
    echo "Error: File not found: $METADATA_FILE"
    exit 1
fi

echo "Validating: $METADATA_FILE"
echo "Against schema: $SCHEMA_FILE"
echo ""

# Check if jq is available
if ! command -v jq &> /dev/null; then
    echo "Error: jq is required but not installed"
    echo "Install with: brew install jq (macOS) or apt-get install jq (Linux)"
    exit 1
fi

# Extract JSON (skip comment lines starting with //)
JSON_CONTENT=$(grep -v '^//' "$METADATA_FILE" | grep -v '^$')

# Validate JSON syntax
echo "$JSON_CONTENT" | jq empty 2>/dev/null
if [ $? -eq 0 ]; then
    echo "✅ JSON syntax is valid"
else
    echo "❌ JSON syntax is invalid"
    exit 1
fi

# Check required fields
ERRORS=0

# Check version field
if ! echo "$JSON_CONTENT" | jq -e '.version' > /dev/null 2>&1; then
    echo "❌ Missing required field: version"
    ERRORS=$((ERRORS + 1))
fi

# Check namespace field
if ! echo "$JSON_CONTENT" | jq -e '.namespace' > /dev/null 2>&1; then
    echo "❌ Missing required field: namespace"
    ERRORS=$((ERRORS + 1))
fi

# Check array fields exist
for field in types functions typeAliases clients retryPolicies; do
    if ! echo "$JSON_CONTENT" | jq -e ".$field" > /dev/null 2>&1; then
        echo "❌ Missing required field: $field"
        ERRORS=$((ERRORS + 1))
    elif ! echo "$JSON_CONTENT" | jq -e ".$field | type == \"array\"" > /dev/null 2>&1; then
        echo "❌ Field $field must be an array"
        ERRORS=$((ERRORS + 1))
    fi
done

# Validate client providers are valid enums
VALID_PROVIDERS=("openai" "anthropic" "aws-bedrock" "google-ai" "vertex-ai" "ollama" "azure-openai" "openai-responses" "openai-generic" "strategy")
CLIENT_COUNT=$(echo "$JSON_CONTENT" | jq '.clients | length')
for ((i=0; i<$CLIENT_COUNT; i++)); do
    PROVIDER=$(echo "$JSON_CONTENT" | jq -r ".clients[$i].provider")
    if [[ ! " ${VALID_PROVIDERS[@]} " =~ " ${PROVIDER} " ]]; then
        echo "❌ Invalid provider at clients[$i]: $PROVIDER"
        echo "   Valid providers: ${VALID_PROVIDERS[*]}"
        ERRORS=$((ERRORS + 1))
    fi
done

# Validate retry policy strategies are valid enums
VALID_STRATEGIES=("ConstantDelay" "ExponentialBackoff")
POLICY_COUNT=$(echo "$JSON_CONTENT" | jq '.retryPolicies | length')
for ((i=0; i<$POLICY_COUNT; i++)); do
    STRATEGY=$(echo "$JSON_CONTENT" | jq -r ".retryPolicies[$i].strategy")
    if [[ ! " ${VALID_STRATEGIES[@]} " =~ " ${STRATEGY} " ]]; then
        echo "❌ Invalid strategy at retryPolicies[$i]: $STRATEGY"
        echo "   Valid strategies: ${VALID_STRATEGIES[*]}"
        ERRORS=$((ERRORS + 1))
    fi

    # Validate strategy options
    if [ "$STRATEGY" == "ConstantDelay" ]; then
        if ! echo "$JSON_CONTENT" | jq -e ".retryPolicies[$i].options.delayMs" > /dev/null 2>&1; then
            echo "❌ ConstantDelay strategy at retryPolicies[$i] must have delayMs option"
            ERRORS=$((ERRORS + 1))
        fi
    elif [ "$STRATEGY" == "ExponentialBackoff" ]; then
        if ! echo "$JSON_CONTENT" | jq -e ".retryPolicies[$i].options.delayMs" > /dev/null 2>&1; then
            echo "❌ ExponentialBackoff strategy at retryPolicies[$i] must have delayMs option"
            ERRORS=$((ERRORS + 1))
        fi
        if ! echo "$JSON_CONTENT" | jq -e ".retryPolicies[$i].options.multiplier" > /dev/null 2>&1; then
            echo "❌ ExponentialBackoff strategy at retryPolicies[$i] must have multiplier option"
            ERRORS=$((ERRORS + 1))
        fi
        if ! echo "$JSON_CONTENT" | jq -e ".retryPolicies[$i].options.maxDelayMs" > /dev/null 2>&1; then
            echo "❌ ExponentialBackoff strategy at retryPolicies[$i] must have maxDelayMs option"
            ERRORS=$((ERRORS + 1))
        fi
    fi
done

# Validate type kinds are valid enums
VALID_KINDS=("Enum" "Record" "Class")
TYPE_COUNT=$(echo "$JSON_CONTENT" | jq '.types | length')
for ((i=0; i<$TYPE_COUNT; i++)); do
    KIND=$(echo "$JSON_CONTENT" | jq -r ".types[$i].kind")
    if [[ ! " ${VALID_KINDS[@]} " =~ " ${KIND} " ]]; then
        echo "❌ Invalid kind at types[$i]: $KIND"
        echo "   Valid kinds: ${VALID_KINDS[*]}"
        ERRORS=$((ERRORS + 1))
    fi

    # Validate Enum types have values
    if [ "$KIND" == "Enum" ]; then
        if ! echo "$JSON_CONTENT" | jq -e ".types[$i].values" > /dev/null 2>&1; then
            echo "❌ Enum type at types[$i] must have values array"
            ERRORS=$((ERRORS + 1))
        fi
    fi

    # Validate Record/Class types have properties
    if [ "$KIND" == "Record" ] || [ "$KIND" == "Class" ]; then
        if ! echo "$JSON_CONTENT" | jq -e ".types[$i].properties" > /dev/null 2>&1; then
            echo "❌ $KIND type at types[$i] must have properties array"
            ERRORS=$((ERRORS + 1))
        fi
    fi
done

echo ""
if [ $ERRORS -eq 0 ]; then
    echo "✅ All validations passed!"
    exit 0
else
    echo "❌ Validation failed with $ERRORS error(s)"
    exit 1
fi
